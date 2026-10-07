//! Google Gemini AI Provider implementation.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{
    AiError, AiProvider,
    utils::{base64_encode, default_http_client, format_enhancement_prompt},
};

const DEFAULT_GEMINI_MODEL: &str = "gemini-2.0-flash";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiPart {
    InlineData {
        #[serde(rename = "inlineData", alias = "inline_data")]
        inline_data: GeminiInlineData,
    },
    Text {
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiInlineData {
    #[serde(rename = "mimeType", alias = "mime_type")]
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: Option<GeminiResponseContent>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseContent {
    pub parts: Option<Vec<GeminiResponsePart>>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponsePart {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiModelsResponse {
    pub models: Option<Vec<GeminiModelItem>>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiModelItem {
    pub name: String,
}

pub struct GeminiProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiProvider {
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
            _ => DEFAULT_GEMINI_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
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

        let base64_audio = base64_encode(&audio_wav);
        let prompt_text = system_prompt.unwrap_or(
            "Accurately transcribe the spoken audio into text. Output only the transcription.",
        );

        let payload = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![
                    GeminiPart::InlineData {
                        inline_data: GeminiInlineData {
                            mime_type: "audio/wav".to_string(),
                            data: base64_audio,
                        },
                    },
                    GeminiPart::Text {
                        text: prompt_text.to_string(),
                    },
                ],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        );

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Gemini API error ({}): {}",
                status, body
            )));
        }

        let gemini_resp: GeminiResponse = response.json().await?;
        let text = gemini_resp
            .candidates
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|cand| cand.content.as_ref())
            .and_then(|content| content.parts.as_ref())
            .and_then(|parts| {
                let combined: String = parts
                    .iter()
                    .filter_map(|p| p.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                if combined.is_empty() {
                    None
                } else {
                    Some(combined.trim().to_string())
                }
            })
            .unwrap_or_default();

        Ok(text)
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let prompt = format_enhancement_prompt(text, tone);
        let payload = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart::Text { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        );

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Gemini API error ({}): {}",
                status, body
            )));
        }

        let gemini_resp: GeminiResponse = response.json().await?;
        let enhanced = gemini_resp
            .candidates
            .as_ref()
            .and_then(|c| c.first())
            .and_then(|cand| cand.content.as_ref())
            .and_then(|content| content.parts.as_ref())
            .and_then(|parts| {
                let combined: String = parts
                    .iter()
                    .filter_map(|p| p.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("");
                if combined.is_empty() {
                    None
                } else {
                    Some(combined.trim().to_string())
                }
            })
            .unwrap_or_else(|| text.to_string());

        Ok(enhanced)
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }

        let url = "https://generativelanguage.googleapis.com/v1beta/models";
        let response = self
            .client
            .get(url)
            .header("x-goog-api-key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Gemini API error ({}): {}",
                status, body
            )));
        }

        let models_resp: GeminiModelsResponse = response.json().await?;
        let models = models_resp
            .models
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                m.name
                    .strip_prefix("models/")
                    .unwrap_or(&m.name)
                    .to_string()
            })
            .collect();

        Ok(models)
    }
}
