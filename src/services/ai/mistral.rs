//! Mistral AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::default_http_client, utils::format_enhancement_prompt};

const DEFAULT_MISTRAL_MODEL: &str = "mistral-small-latest";

pub struct MistralProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct MistralChatResponse {
    choices: Option<Vec<MistralChoice>>,
}

#[derive(Debug, Deserialize)]
struct MistralChoice {
    message: Option<MistralMessage>,
}

#[derive(Debug, Deserialize)]
struct MistralMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralModelsResponse {
    data: Option<Vec<MistralModelData>>,
}

#[derive(Debug, Deserialize)]
struct MistralModelData {
    id: String,
}

impl MistralProvider {
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
            _ => DEFAULT_MISTRAL_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for MistralProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "Mistral does not support audio transcription directly".to_string(),
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

        let url = "https://api.mistral.ai/v1/chat/completions";
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
                "Mistral API error ({}): {}",
                status, body
            )));
        }

        let chat_resp: MistralChatResponse = response.json().await?;
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

        let url = "https://api.mistral.ai/v1/models";
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
                "Mistral API error ({}): {}",
                status, body
            )));
        }

        let models_resp: MistralModelsResponse = response.json().await?;
        let models = models_resp
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
