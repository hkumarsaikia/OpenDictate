//! OpenRouter AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{utils::format_enhancement_prompt, AiError, AiProvider};

const DEFAULT_OPENROUTER_MODEL: &str = "qwen/qwen3.8-27b:free";

pub struct OpenRouterProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChatResponse {
    choices: Option<Vec<OpenRouterChoice>>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: Option<OpenRouterMessage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterModelsResponse {
    data: Option<Vec<OpenRouterModelData>>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterModelData {
    id: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: impl Into<String>, model: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .user_agent("OpenDictate/2.0.0 (Linux; x86_64)")
            .build()
            .unwrap_or_default();
        Self::with_client(api_key, model, client)
    }

    pub fn with_client(
        api_key: impl Into<String>,
        model: Option<String>,
        client: reqwest::Client,
    ) -> Self {
        let key = api_key.into();
        let chosen_model = match model {
            Some(m) if !m.trim().is_empty() => m,
            _ => DEFAULT_OPENROUTER_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for OpenRouterProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "OpenRouter audio transcription is not supported".to_string(),
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
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.2
        });

        let url = "https://openrouter.ai/api/v1/chat/completions";
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "OpenRouter API error ({}): {}",
                status, body
            )));
        }

        let chat_resp: OpenRouterChatResponse = response.json().await?;
        let enhanced = chat_resp
            .choices
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|choice| choice.message.as_ref())
            .and_then(|msg| msg.content.as_deref())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| text.to_string());

        Ok(enhanced)
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }

        let url = "https://openrouter.ai/api/v1/models";
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "OpenRouter API error ({}): {}",
                status, body
            )));
        }

        let models_resp: OpenRouterModelsResponse = response.json().await?;
        let models = models_resp
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
