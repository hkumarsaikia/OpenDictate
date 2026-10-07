//! Kilo Code AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::default_http_client, utils::format_enhancement_prompt};

pub const DEFAULT_KILOCODE_MODEL: &str = "kilo-auto/free";

pub struct KiloCodeProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatResponse {
    choices: Option<Vec<OpenAiChoice>>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: Option<OpenAiMessage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelsResponse {
    data: Option<Vec<OpenAiModelData>>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelData {
    id: String,
}

impl KiloCodeProvider {
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
            _ => DEFAULT_KILOCODE_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for KiloCodeProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "Kilo Code does not support direct audio transcription".to_string(),
        ))
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
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

        let url = "https://api.kilo.ai/api/gateway/chat/completions";
        let mut req = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&payload);

        if !self.api_key.trim().is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key.trim()));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Kilo Code API error {}: {}",
                status, err_body
            )));
        }

        let chat_res: OpenAiChatResponse = response.json().await?;
        let choice = chat_res
            .choices
            .as_ref()
            .and_then(|c| c.first())
            .ok_or_else(|| AiError::ApiError("No choice returned from Kilo Code".to_string()))?;

        let content = choice
            .message
            .as_ref()
            .and_then(|m| m.content.clone())
            .unwrap_or_default();

        Ok(content.trim().to_string())
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        let url = "https://api.kilo.ai/api/gateway/models";
        let mut req = self.client.get(url);
        if !self.api_key.trim().is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key.trim()));
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Kilo Code models error {}: {}",
                status, err_body
            )));
        }

        let models_res: OpenAiModelsResponse = response.json().await?;
        let models = models_res
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
