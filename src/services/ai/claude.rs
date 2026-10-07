//! Anthropic Claude AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::default_http_client, utils::format_enhancement_prompt};

const DEFAULT_CLAUDE_MODEL: &str = "claude-3-5-haiku-20241022";

pub struct ClaudeProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ClaudeMessageResponse {
    content: Option<Vec<ClaudeContentBlock>>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContentBlock {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaudeModelsResponse {
    data: Option<Vec<ClaudeModelData>>,
}

#[derive(Debug, Deserialize)]
struct ClaudeModelData {
    id: String,
}

impl ClaudeProvider {
    pub fn new(api_key: impl Into<String>, model: Option<String>) -> Self {
        Self::with_client(api_key, model, default_http_client())
    }

    pub fn with_client(
        api_key: impl Into<String>,
        model: Option<String>,
        client: reqwest::Client,
    ) -> Self {
        let key = api_key.into();
        let chosen_model = match model {
            Some(m) if !m.trim().is_empty() => m,
            _ => DEFAULT_CLAUDE_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for ClaudeProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "Claude does not support audio transcription directly".to_string(),
        ))
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let prompt = format_enhancement_prompt(text, tone);
        let payload = json!({
            "model": self.model,
            "max_tokens": 4096,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let url = "https://api.anthropic.com/v1/messages";
        let response = self
            .client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Claude API error ({}): {}",
                status, body
            )));
        }

        let claude_resp: ClaudeMessageResponse = response.json().await?;
        let enhanced = claude_resp
            .content
            .as_ref()
            .and_then(|blocks| blocks.first())
            .and_then(|b| b.text.as_deref())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| text.to_string());

        Ok(enhanced)
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }

        let url = "https://api.anthropic.com/v1/models";
        let response = self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .send()
            .await;

        if let Ok(resp) = response
            && resp.status().is_success()
            && let Ok(models_resp) = resp.json::<ClaudeModelsResponse>().await
            && let Some(data) = models_resp.data
        {
            return Ok(data.into_iter().map(|m| m.id).collect());
        }

        // Return curated list of Claude models if endpoint is unavailable
        Ok(vec![
            "claude-3-5-haiku-20241022".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-opus-20240229".to_string(),
        ])
    }
}
