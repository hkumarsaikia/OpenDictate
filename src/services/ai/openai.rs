//! OpenAI AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::default_http_client, utils::format_enhancement_prompt};

const DEFAULT_OPENAI_LLM_MODEL: &str = "gpt-4o-mini";
const DEFAULT_OPENAI_STT_MODEL: &str = "whisper-1";

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    transcription_model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct OpenAiTranscriptionResponse {
    text: String,
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

impl OpenAiProvider {
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
            _ => DEFAULT_OPENAI_LLM_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            transcription_model: DEFAULT_OPENAI_STT_MODEL.to_string(),
            client,
        }
    }
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    async fn transcribe(
        &self,
        audio_wav: Vec<u8>,
        system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        if audio_wav.is_empty() {
            return Ok(String::new());
        }

        let file_part = reqwest::multipart::Part::bytes(audio_wav)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| AiError::AudioEncodingError(e.to_string()))?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("model", self.transcription_model.clone())
            .text("response_format", "json");

        if let Some(prompt) = system_prompt {
            form = form.text("prompt", prompt.to_string());
        }

        let url = "https://api.openai.com/v1/audio/transcriptions";
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "OpenAI API error ({}): {}",
                status, body
            )));
        }

        let resp: OpenAiTranscriptionResponse = response.json().await?;
        Ok(resp.text.trim().to_string())
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

        let url = "https://api.openai.com/v1/chat/completions";
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
                "OpenAI API error ({}): {}",
                status, body
            )));
        }

        let chat_resp: OpenAiChatResponse = response.json().await?;
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

        let url = "https://api.openai.com/v1/models";
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
                "OpenAI API error ({}): {}",
                status, body
            )));
        }

        let models_resp: OpenAiModelsResponse = response.json().await?;
        let models = models_resp
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
