//! Cloudflare Workers AI Provider implementation.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::{AiError, AiProvider, utils::format_enhancement_prompt};

const DEFAULT_CLOUDFLARE_LLM_MODEL: &str = "@cf/meta/llama-3.1-8b-instruct";
const DEFAULT_CLOUDFLARE_STT_MODEL: &str = "@cf/openai/whisper";

pub struct CloudflareProvider {
    api_key: String,
    account_id: String,
    model: String,
    transcription_model: String,
    client: reqwest::Client,
    cached_account_id: tokio::sync::OnceCell<String>,
}

#[derive(Debug, Deserialize)]
struct CloudflareTranscriptionResponse {
    result: Option<CloudflareTranscriptionResult>,
}

#[derive(Debug, Deserialize)]
struct CloudflareTranscriptionResult {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CloudflareRunResponse {
    result: Option<CloudflareRunResult>,
}

#[derive(Debug, Deserialize)]
struct CloudflareRunResult {
    response: Option<String>,
}

impl CloudflareProvider {
    pub fn new(
        api_key: impl Into<String>,
        account_id: impl Into<String>,
        model: Option<String>,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .user_agent("OpenDictate/2.0.0 (Linux; x86_64)")
            .build()
            .unwrap_or_default();
        Self::with_client(api_key, account_id, model, client)
    }

    pub fn with_client(
        api_key: impl Into<String>,
        account_id: impl Into<String>,
        model: Option<String>,
        client: reqwest::Client,
    ) -> Self {
        let key = api_key.into();
        let acc_id = account_id.into();
        let chosen_model = match model {
            Some(m) if !m.trim().is_empty() => m,
            _ => DEFAULT_CLOUDFLARE_LLM_MODEL.to_string(),
        };
        Self {
            api_key: key,
            account_id: acc_id,
            model: chosen_model,
            transcription_model: DEFAULT_CLOUDFLARE_STT_MODEL.to_string(),
            client,
            cached_account_id: tokio::sync::OnceCell::new(),
        }
    }

    /// Resolves the Cloudflare account ID, auto-discovering it from the API if not provided.
    pub async fn resolve_account_id(&self) -> Result<String, AiError> {
        if !self.account_id.trim().is_empty() {
            return Ok(self.account_id.trim().to_string());
        }

        if let Some(id) = self.cached_account_id.get() {
            return Ok(id.clone());
        }

        // Auto-discover account_id using the Cloudflare API token with retry on transient network errors
        let url = "https://api.cloudflare.com/client/v4/accounts";
        let mut last_err = None;

        for attempt in 0..3 {
            if attempt > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(300 * attempt)).await;
            }

            let send_res = self
                .client
                .get(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await;

            match send_res {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        return Err(AiError::ApiError(format!(
                            "Failed to auto-resolve Cloudflare account ID (HTTP {}): please set CLOUDFLARE_ACCOUNT_ID",
                            resp.status()
                        )));
                    }

                    #[derive(Deserialize)]
                    struct AccountItem {
                        id: String,
                    }
                    #[derive(Deserialize)]
                    struct AccountsResp {
                        result: Option<Vec<AccountItem>>,
                    }

                    let acc_resp: AccountsResp = resp.json().await?;
                    if let Some(list) = acc_resp.result
                        && let Some(first) = list.first()
                    {
                        let resolved = first.id.clone();
                        let _ = self.cached_account_id.set(resolved.clone());
                        return Ok(resolved);
                    }

                    return Err(AiError::ApiError(
                        "No Cloudflare account found for this API token: please set CLOUDFLARE_ACCOUNT_ID".to_string(),
                    ));
                }
                Err(err) => {
                    last_err = Some(err);
                }
            }
        }

        Err(AiError::NetworkError(
            last_err.expect("retry loop must run at least once"),
        ))
    }
}

#[async_trait]
impl AiProvider for CloudflareProvider {
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

        let account_id = self.resolve_account_id().await?;
        let clean_stt = self.transcription_model.trim_start_matches('/');
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            account_id, clean_stt
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/octet-stream")
            .body(audio_wav)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Cloudflare AI error ({}): {}",
                status, body
            )));
        }

        let resp: CloudflareTranscriptionResponse = response.json().await?;
        let text = resp
            .result
            .and_then(|r| r.text)
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(text)
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let account_id = self.resolve_account_id().await?;
        let prompt = format_enhancement_prompt(text, tone);
        let payload = json!({
            "messages": [
                {
                    "role": "system",
                    "content": "You are an AI dictation post-processor. Output ONLY the polished speech text without any code, explanation, notes, or commentary."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 512
        });

        let clean_model = self.model.trim_start_matches('/');
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
            account_id, clean_model
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::ApiError(format!(
                "Cloudflare AI error ({}): {}",
                status, body
            )));
        }

        let resp: CloudflareRunResponse = response.json().await?;
        let enhanced = resp
            .result
            .and_then(|r| r.response)
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| text.to_string());

        Ok(enhanced)
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        if self.api_key.trim().is_empty() {
            return Err(AiError::MissingApiKey);
        }
        Ok(vec![
            "@cf/meta/llama-3.1-8b-instruct".to_string(),
            "@cf/openai/whisper".to_string(),
        ])
    }
}
