//! Cohere AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{utils::default_http_client, utils::format_enhancement_prompt, AiError, AiProvider};

pub const DEFAULT_COHERE_MODEL: &str = "command-r-08-2024";

pub struct CohereProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct CohereChatResponse {
    message: Option<CohereMessage>,
}

#[derive(Debug, Deserialize)]
struct CohereMessage {
    content: Option<Vec<CohereContentPart>>,
}

#[derive(Debug, Deserialize)]
struct CohereContentPart {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CohereModelsResponse {
    models: Option<Vec<CohereModelData>>,
}

#[derive(Debug, Deserialize)]
struct CohereModelData {
    name: String,
}

impl CohereProvider {
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
            _ => DEFAULT_COHERE_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for CohereProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "Cohere does not support audio transcription".to_string(),
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
            ]
        });

        let url = "https://api.cohere.com/v2/chat";
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Cohere API error {}: {}",
                status, err_body
            )));
        }

        let chat_res: CohereChatResponse = response.json().await?;
        let message = chat_res
            .message
            .ok_or_else(|| AiError::ApiError("No message returned from Cohere".to_string()))?;

        let mut output = String::new();
        if let Some(parts) = message.content {
            for part in parts {
                if let Some(t) = part.text {
                    output.push_str(&t);
                }
            }
        }

        Ok(output.trim().to_string())
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }

        let url = "https://api.cohere.com/v1/models";
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Cohere models error {}: {}",
                status, err_body
            )));
        }

        let models_res: CohereModelsResponse = response.json().await?;
        let models = models_res
            .models
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.name)
            .collect();

        Ok(models)
    }
}
