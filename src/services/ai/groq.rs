//! Groq AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::format_enhancement_prompt};

const DEFAULT_GROQ_LLM_MODEL: &str = "openai/gpt-oss-120b";
const DEFAULT_GROQ_STT_MODEL: &str = "whisper-large-v3-turbo";

pub struct GroqProvider {
    api_key: String,
    model: String,
    transcription_model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct GroqTranscriptionResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GroqChatResponse {
    choices: Option<Vec<GroqChoice>>,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: Option<GroqMessage>,
}

#[derive(Debug, Deserialize)]
struct GroqMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GroqModelsResponse {
    data: Option<Vec<GroqModelData>>,
}

#[derive(Debug, Deserialize)]
struct GroqModelData {
    id: String,
}

impl GroqProvider {
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
            _ => DEFAULT_GROQ_LLM_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            transcription_model: DEFAULT_GROQ_STT_MODEL.to_string(),
            client,
        }
    }
}

#[async_trait]
impl AiProvider for GroqProvider {
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
            let trimmed = prompt.trim();
            if (trimmed.len() == 2 || trimmed.len() == 3)
                && trimmed.chars().all(|c| c.is_ascii_alphabetic())
            {
                form = form.text("language", trimmed.to_lowercase());
            } else {
                form = form.text("prompt", trimmed.to_string());
            }
        }

        let url = "https://api.groq.com/openai/v1/audio/transcriptions";
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
                "Groq API error ({}): {}",
                status, body
            )));
        }

        let resp: GroqTranscriptionResponse = response.json().await?;
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

        let url = "https://api.groq.com/openai/v1/chat/completions";
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
                "Groq API error ({}): {}",
                status, body
            )));
        }

        let chat_resp: GroqChatResponse = response.json().await?;
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

        let url = "https://api.groq.com/openai/v1/models";
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
                "Groq API error ({}): {}",
                status, body
            )));
        }

        let models_resp: GroqModelsResponse = response.json().await?;
        let models = models_resp
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(models)
    }
}
