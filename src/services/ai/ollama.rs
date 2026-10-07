//! Ollama local AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{utils::default_http_client, utils::format_enhancement_prompt, AiError, AiProvider};

const DEFAULT_OLLAMA_MODEL: &str = "llama3.2";
const DEFAULT_OLLAMA_HOST: &str = "http://localhost:11434";

pub struct OllamaProvider {
    host: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaModelItem>>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelItem {
    name: String,
}

impl OllamaProvider {
    pub fn new(host: Option<String>, model: Option<String>) -> Self {
        Self::with_client(host, model, default_http_client())
    }

    pub fn with_client(
        host: Option<String>,
        model: Option<String>,
        client: reqwest::Client,
    ) -> Self {
        let chosen_host = match host {
            Some(h) if !h.trim().is_empty() => h.trim_end_matches('/').to_string(),
            _ => DEFAULT_OLLAMA_HOST.to_string(),
        };
        let chosen_model = match model {
            Some(m) if !m.trim().is_empty() => m,
            _ => DEFAULT_OLLAMA_MODEL.to_string(),
        };
        Self {
            host: chosen_host,
            model: chosen_model,
            client,
        }
    }
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn transcribe(
        &self,
        _audio_wav: Vec<u8>,
        _system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        Err(AiError::UnsupportedOperation(
            "Ollama does not provide native audio transcription".to_string(),
        ))
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let prompt = format_enhancement_prompt(text, tone);
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });

        let url = format!("{}/api/generate", self.host);
        let response = self.client.post(&url).json(&payload).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Ollama API error ({}): {}",
                status, body
            )));
        }

        let gen_resp: OllamaGenerateResponse = response.json().await?;
        Ok(gen_resp
            .response
            .unwrap_or_else(|| text.to_string())
            .trim()
            .to_string())
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        let url = format!("{}/api/tags", self.host);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Ollama API error ({}): {}",
                status, body
            )));
        }

        let tags_resp: OllamaTagsResponse = response.json().await?;
        let models = tags_resp
            .models
            .unwrap_or_default()
            .into_iter()
            .map(|m| m.name)
            .collect();

        Ok(models)
    }
}
