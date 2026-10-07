//! Hugging Face Inference API & Router Provider implementation.
//!
//! Provides speech-to-text audio transcription using serverless ASR endpoints
//! (e.g. `openai/whisper-large-v3-turbo`) and text enhancement using OpenAI-compatible
//! chat completions (e.g. `meta-llama/Llama-3.3-70B-Instruct`) via router.huggingface.co.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{utils::format_enhancement_prompt, AiError, AiProvider};

pub const DEFAULT_HUGGINGFACE_LLM_MODEL: &str = "meta-llama/Llama-3.3-70B-Instruct";
pub const DEFAULT_HUGGINGFACE_STT_MODEL: &str = "openai/whisper-large-v3-turbo";

pub struct HuggingFaceProvider {
    api_key: String,
    model: String,
    transcription_model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceChatResponse {
    choices: Option<Vec<HuggingFaceChoice>>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceChoice {
    message: Option<HuggingFaceMessage>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceAsrResponse {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceModelEntry {
    id: String,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceModelsResponse {
    data: Option<Vec<HuggingFaceModelEntry>>,
}

impl HuggingFaceProvider {
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
            _ => DEFAULT_HUGGINGFACE_LLM_MODEL.to_string(),
        };
        Self {
            api_key: key,
            model: chosen_model,
            transcription_model: DEFAULT_HUGGINGFACE_STT_MODEL.to_string(),
            client,
        }
    }

    pub fn with_transcription_model(mut self, model: impl Into<String>) -> Self {
        let m = model.into();
        if !m.trim().is_empty() {
            self.transcription_model = m;
        }
        self
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn transcription_model(&self) -> &str {
        &self.transcription_model
    }
}

#[async_trait]
impl AiProvider for HuggingFaceProvider {
    async fn transcribe(
        &self,
        audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        if audio_wav.is_empty() {
            return Ok(String::new());
        }

        let model_id = if self.model.contains("whisper") {
            &self.model
        } else {
            &self.transcription_model
        };

        let url = format!(
            "https://router.huggingface.co/hf-inference/models/{}",
            model_id
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .header("Content-Type", "audio/wav")
            .header("x-wait-for-model", "true")
            .body(audio_wav)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let err_text = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Hugging Face ASR error (HTTP {}): {}",
                status, err_text
            )));
        }

        let body_text = response.text().await?;
        if let Ok(parsed) = serde_json::from_str::<HuggingFaceAsrResponse>(&body_text) {
            if let Some(text) = parsed.text {
                return Ok(text);
            }
        }

        // Try parsing as array of results
        if let Ok(arr) = serde_json::from_str::<Vec<HuggingFaceAsrResponse>>(&body_text) {
            if let Some(first) = arr.into_iter().next() {
                if let Some(text) = first.text {
                    return Ok(text);
                }
            }
        }

        Ok(body_text.trim().to_string())
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

        let url = "https://router.huggingface.co/v1/chat/completions";
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
            let err_text = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Hugging Face chat error (HTTP {}): {}",
                status, err_text
            )));
        }

        let chat_res: HuggingFaceChatResponse = response.json().await?;
        let output = chat_res
            .choices
            .and_then(|mut c| c.pop())
            .and_then(|c| c.message)
            .and_then(|m| m.content)
            .unwrap_or_default();

        Ok(output.trim().to_string())
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        let mut models = vec![
            "openai/whisper-large-v3-turbo".to_string(),
            "openai/whisper-large-v3".to_string(),
        ];

        let url = "https://router.huggingface.co/v1/models";
        let mut req = self.client.get(url);
        if !self.api_key.trim().is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key.trim()));
        }

        if let Ok(res) = req.send().await {
            if res.status().is_success() {
                if let Ok(parsed) = res.json::<HuggingFaceModelsResponse>().await {
                    if let Some(entries) = parsed.data {
                        for entry in entries {
                            if !models.contains(&entry.id) {
                                models.push(entry.id);
                            }
                        }
                    }
                }
            }
        }

        Ok(models)
    }
}
