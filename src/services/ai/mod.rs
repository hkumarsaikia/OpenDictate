//! Async AI Service & Providers for speech transcription and text enhancement.

use async_trait::async_trait;
use std::sync::Arc;

use crate::config::Config;

pub mod cerebras;
pub mod claude;
pub mod cloudflare;
pub mod cohere;
pub mod gemini;
pub mod groq;
pub mod huggingface;
pub mod kilocode;
pub mod local_ai;
pub mod mistral;
pub mod nvidia;
pub mod ollama;
pub mod openai;
pub mod opencode;
pub mod openrouter;
pub mod utils;

pub use utils::{
    apply_smart_local_formatting, format_enhancement_prompt, is_transient_ai_error, redact_secrets,
};

/// Standardized user-facing warning when a selected AI model does not support audio input.
pub const AUDIO_UNSUPPORTED_WARNING: &str =
    "Selected model does not support audio. Please select an audio-capable model.";

/// Standardized user-facing warning when a selected AI model requires a paid API plan.
pub const PAID_MODEL_UNPAID_PLAN_WARNING: &str =
    "Selected model requires a paid API plan. Please upgrade your provider plan to use this model.";

/// Removes redundant trailing (free), [free], :free, /free, - free, or bare free tags from model names.
pub fn clean_free_suffix(name: &str) -> String {
    let mut s = name.trim().to_string();
    loop {
        let lower = s.to_lowercase();
        let strip_len = if lower.ends_with("(free)")
            || lower.ends_with("[free]")
            || lower.ends_with("- free")
        {
            6
        } else if lower.ends_with("– free") {
            "– free".len()
        } else if lower.ends_with(":free") || lower.ends_with("/free") || lower.ends_with(" free") {
            5
        } else {
            break;
        };
        s = s[..s.len() - strip_len].trim().to_string();
    }
    s
}

/// Errors that may occur in AI provider operations.
#[derive(Debug)]
pub enum AiError {
    MissingApiKey,
    NetworkError(reqwest::Error),
    ApiError(String),
    AudioEncodingError(String),
    UnsupportedOperation(String),
    JsonError(serde_json::Error),
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiError::MissingApiKey => write!(f, "Missing API key"),
            AiError::NetworkError(err) => {
                let redacted = utils::redact_secrets(&err.to_string(), "");
                write!(f, "Network error: {}", redacted)
            }
            AiError::ApiError(msg) => {
                let redacted = utils::redact_secrets(msg, "");
                write!(f, "API error: {}", redacted)
            }
            AiError::AudioEncodingError(msg) => write!(f, "Audio encoding error: {}", msg),
            AiError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
            AiError::JsonError(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for AiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AiError::NetworkError(err) => Some(err),
            AiError::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for AiError {
    fn from(err: reqwest::Error) -> Self {
        AiError::NetworkError(err)
    }
}

impl From<serde_json::Error> for AiError {
    fn from(err: serde_json::Error) -> Self {
        AiError::JsonError(err)
    }
}

/// Metadata describing an AI model, matching OpenCode style convention.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub inputs: Vec<String>, // e.g. ["text"], ["text", "audio"], ["text", "image", "audio"]
    pub reasoning: bool,     // Allows reasoning vs No reasoning
    pub context_limit: u64,  // Token context length
    pub is_free: bool,
}

impl ModelInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>, is_free: bool) -> Self {
        let id_str = id.into();
        let raw_name = name.into();
        let name_str = clean_free_suffix(&raw_name);
        let inputs = detect_inputs(&id_str);
        let reasoning = detect_reasoning(&id_str);
        let context_limit = detect_context_limit(&id_str);
        Self {
            id: id_str,
            name: name_str,
            provider: String::new(),
            inputs,
            reasoning,
            context_limit,
            is_free,
        }
    }

    pub fn with_details(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
        inputs: Vec<String>,
        reasoning: bool,
        context_limit: u64,
        is_free: bool,
    ) -> Self {
        let raw_name = name.into();
        let name_str = clean_free_suffix(&raw_name);
        Self {
            id: id.into(),
            name: name_str,
            provider: provider.into(),
            inputs,
            reasoning,
            context_limit,
            is_free,
        }
    }

    /// Returns true if this model supports audio input.
    pub fn supports_audio(&self) -> bool {
        self.inputs.iter().any(|i| i.eq_ignore_ascii_case("audio"))
    }

    /// Formatted display label (e.g. "llama-3.3-70b [Free]" or "gpt-4o")
    pub fn display_label(&self) -> String {
        if self.is_free {
            format!("{} [Free]", self.name)
        } else {
            self.name.clone()
        }
    }

    /// Formatted label for hover card title (e.g. "Big Pickle (Free)" or "GPT-5.4 nano")
    pub fn card_model_title(&self) -> String {
        if self.is_free {
            format!("{} (Free)", self.name)
        } else {
            self.name.clone()
        }
    }

    /// Human-readable inputs string (e.g. "text, audio" or "text, image")
    pub fn inputs_label(&self) -> String {
        if self.inputs.is_empty() {
            "text".to_string()
        } else {
            self.inputs.join(", ")
        }
    }

    /// Reasoning label ("Allows reasoning" or "No reasoning")
    pub fn reasoning_label(&self) -> &'static str {
        if self.reasoning {
            "Allows reasoning"
        } else {
            "No reasoning"
        }
    }

    /// Formatted context limit with comma thousand separators (e.g. "200,000" or "1,000,000")
    pub fn formatted_context(&self) -> String {
        let s = self.context_limit.to_string();
        let mut result = String::new();
        let len = s.len();
        for (i, ch) in s.chars().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(ch);
        }
        result
    }
}

pub fn detect_inputs(id: &str) -> Vec<String> {
    let lower = id.to_lowercase();
    if lower.contains("whisper")
        || lower.contains("parakeet")
        || lower.contains("canary")
        || lower.contains("asr")
    {
        vec!["audio".to_string(), "text".to_string()]
    } else if lower.contains("omni") || lower.contains("gemini") {
        vec![
            "text".to_string(),
            "image".to_string(),
            "audio".to_string(),
            "video".to_string(),
        ]
    } else if lower.contains("vision")
        || lower.contains("vl")
        || lower.contains("4o")
        || lower.contains("pixtral")
    {
        vec!["text".to_string(), "image".to_string()]
    } else {
        vec!["text".to_string()]
    }
}

pub fn detect_reasoning(id: &str) -> bool {
    let lower = id.to_lowercase();
    lower.contains("r1")
        || lower.contains("reason")
        || lower.contains("thinking")
        || lower.contains("pickle")
        || lower.contains("o1")
        || lower.contains("o3")
}

pub fn detect_context_limit(id: &str) -> u64 {
    let lower = id.to_lowercase();
    if lower.contains("1m") || lower.contains("gemini") {
        1_000_000
    } else if lower.contains("pickle") {
        200_000
    } else if lower.contains("128k")
        || lower.contains("llama-3")
        || lower.contains("qwen")
        || lower.contains("gpt-4")
    {
        128_000
    } else if lower.contains("32k") || lower.contains("mixtral") {
        32_768
    } else if lower.contains("whisper") {
        25_000
    } else {
        128_000
    }
}

pub fn format_model_name(id: &str) -> String {
    let clean = if let Some(pos) = id.rfind('/') {
        &id[pos + 1..]
    } else {
        id
    };
    let clean = clean.strip_suffix(":free").unwrap_or(clean);

    let parts: Vec<&str> = clean.split('-').collect();
    if parts.len() <= 1 {
        return clean.to_string();
    }
    parts
        .iter()
        .map(|p| {
            if p.eq_ignore_ascii_case("gpt")
                || p.eq_ignore_ascii_case("glm")
                || p.eq_ignore_ascii_case("it")
            {
                p.to_uppercase()
            } else {
                let mut c = p.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Returns standard known models for a given AI provider with free status flags.
pub fn get_provider_models(provider: &str) -> Vec<ModelInfo> {
    match provider.trim().to_lowercase().as_str() {
        "groq" => vec![
            ModelInfo::with_details(
                "whisper-large-v3-turbo",
                "Whisper Large V3 Turbo (Audio)",
                "Groq",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "whisper-large-v3",
                "Whisper Large V3 (Audio)",
                "Groq",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "llama-3.3-70b-versatile",
                "Llama 3.3 70B Versatile",
                "Groq",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "llama-3.1-8b-instant",
                "Llama 3.1 8B Instant",
                "Groq",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "mixtral-8x7b-32768",
                "Mixtral 8x7B 32k",
                "Groq",
                vec!["text".to_string()],
                false,
                32_768,
                false,
            ),
            ModelInfo::with_details(
                "gemma2-9b-it",
                "Gemma 2 9B IT",
                "Groq",
                vec!["text".to_string()],
                false,
                8_192,
                false,
            ),
        ],
        "cloudflare" => vec![
            ModelInfo::with_details(
                "@cf/openai/whisper",
                "Whisper (Audio)",
                "Cloudflare",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "@cf/openai/whisper-tiny-en",
                "Whisper Tiny EN (Audio)",
                "Cloudflare",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "@cf/meta/llama-3.3-70b-instruct",
                "Llama 3.3 70B Instruct",
                "Cloudflare",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "@cf/meta/llama-3.1-8b-instruct",
                "Llama 3.1 8B Instruct",
                "Cloudflare",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "@cf/mistral/mistral-7b-instruct-v0.1",
                "Mistral 7B Instruct",
                "Cloudflare",
                vec!["text".to_string()],
                false,
                32_768,
                false,
            ),
        ],
        "openrouter" => vec![
            ModelInfo::with_details(
                "meta-llama/llama-3.3-70b-instruct:free",
                "Llama 3.3 70B Instruct",
                "OpenRouter",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "google/gemini-2.0-flash-exp:free",
                "Gemini 2.0 Flash Exp",
                "OpenRouter",
                vec![
                    "text".to_string(),
                    "image".to_string(),
                    "audio".to_string(),
                    "video".to_string(),
                ],
                false,
                1_048_576,
                true,
            ),
            ModelInfo::with_details(
                "deepseek/deepseek-r1:free",
                "DeepSeek R1",
                "OpenRouter",
                vec!["text".to_string()],
                true,
                163_840,
                true,
            ),
            ModelInfo::with_details(
                "anthropic/claude-3.5-sonnet",
                "Claude 3.5 Sonnet",
                "OpenRouter",
                vec!["text".to_string(), "image".to_string()],
                false,
                200_000,
                false,
            ),
            ModelInfo::with_details(
                "openai/gpt-4o",
                "GPT-4o",
                "OpenRouter",
                vec!["text".to_string(), "image".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "openai/whisper-large-v3",
                "Whisper Large V3 (Audio)",
                "OpenRouter",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                false,
            ),
        ],
        "nvidia" => vec![
            ModelInfo::with_details(
                "meta/llama-3.3-70b-instruct",
                "Llama 3.3 70B Instruct",
                "NVIDIA NIM",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "deepseek-ai/deepseek-r1",
                "DeepSeek R1",
                "NVIDIA NIM",
                vec!["text".to_string()],
                true,
                163_840,
                false,
            ),
            ModelInfo::with_details(
                "nvidia/nemotron-4-340b-instruct",
                "Nemotron 4 340B Instruct",
                "NVIDIA NIM",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "mistralai/mistral-large-2-instruct",
                "Mistral Large 2 Instruct",
                "NVIDIA NIM",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "google/gemma-2-27b-it",
                "Gemma 2 27B IT",
                "NVIDIA NIM",
                vec!["text".to_string()],
                false,
                8_192,
                false,
            ),
        ],
        "cohere" => vec![
            ModelInfo::with_details(
                "command-light",
                "Command Light",
                "Cohere",
                vec!["text".to_string()],
                false,
                32_768,
                true,
            ),
            ModelInfo::with_details(
                "command-r7b-12-2024",
                "Command R7B",
                "Cohere",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "command-r-08-2024",
                "Command R (08-2024)",
                "Cohere",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "command-r-plus-08-2024",
                "Command R+ (08-2024)",
                "Cohere",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
        ],
        "kilocode" => vec![
            ModelInfo::with_details(
                "kilo-auto/free",
                "Kilo Auto Free (Smart Routing)",
                "Kilo Code",
                vec!["text".to_string(), "image".to_string()],
                false,
                1_000_000,
                true,
            ),
            ModelInfo::with_details(
                "meta-llama/llama-3.3-70b-instruct:free",
                "Llama 3.3 70B Instruct",
                "Kilo Code",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "deepseek/deepseek-r1:free",
                "DeepSeek R1",
                "Kilo Code",
                vec!["text".to_string()],
                true,
                163_840,
                true,
            ),
            ModelInfo::with_details(
                "google/gemma-2-9b-it:free",
                "Gemma 2 9B IT",
                "Kilo Code",
                vec!["text".to_string()],
                false,
                8_192,
                true,
            ),
            ModelInfo::with_details(
                "kilo-auto/frontier",
                "Kilo Auto Frontier",
                "Kilo Code",
                vec!["text".to_string(), "image".to_string()],
                true,
                1_000_000,
                false,
            ),
        ],
        "cerebras" => vec![
            ModelInfo::with_details(
                "llama-3.3-70b",
                "Llama 3.3 70B (Ultra Fast)",
                "Cerebras",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "llama3.1-8b",
                "Llama 3.1 8B (Ultra Fast)",
                "Cerebras",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "gpt-oss-120b",
                "GPT-OSS 120B (High Speed)",
                "Cerebras",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "qwen-3.8-27b",
                "Qwen 3.8 27B",
                "Cerebras",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
        ],
        "opencode" => vec![
            ModelInfo::with_details(
                "big-pickle",
                "Big Pickle",
                "OpenCode Zen",
                vec!["text".to_string()],
                true,
                200_000,
                true,
            ),
            ModelInfo::with_details(
                "mimo-v2.6-flash-free",
                "MiMo-V2.6-Flash Free",
                "OpenCode Zen",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "muse-spark-1.3-contributor-free",
                "Muse Spark 1.3 Free",
                "OpenCode Zen",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "nemotron-3.5-lightning-free",
                "Nemotron 3.5 Lightning Free",
                "OpenCode Zen",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "kimi-k2.6",
                "Kimi K2.6",
                "OpenCode Zen",
                vec!["text".to_string()],
                true,
                200_000,
                false,
            ),
            ModelInfo::with_details(
                "glm-5.1",
                "GLM 5.1",
                "OpenCode Zen",
                vec!["text".to_string()],
                true,
                128_000,
                false,
            ),
        ],
        "gemini" => vec![
            ModelInfo::with_details(
                "gemini-2.0-flash",
                "Gemini 2.0 Flash",
                "Google Gemini",
                vec![
                    "text".to_string(),
                    "image".to_string(),
                    "audio".to_string(),
                    "video".to_string(),
                ],
                false,
                1_048_576,
                true,
            ),
            ModelInfo::with_details(
                "gemini-1.5-flash",
                "Gemini 1.5 Flash",
                "Google Gemini",
                vec![
                    "text".to_string(),
                    "image".to_string(),
                    "audio".to_string(),
                    "video".to_string(),
                ],
                false,
                1_048_576,
                true,
            ),
            ModelInfo::with_details(
                "gemini-1.5-pro",
                "Gemini 1.5 Pro",
                "Google Gemini",
                vec![
                    "text".to_string(),
                    "image".to_string(),
                    "audio".to_string(),
                    "video".to_string(),
                ],
                false,
                2_097_152,
                false,
            ),
        ],
        "openai" => vec![
            ModelInfo::with_details(
                "gpt-4o-mini",
                "GPT-4o Mini",
                "OpenAI",
                vec!["text".to_string(), "image".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "gpt-4o",
                "GPT-4o",
                "OpenAI",
                vec!["text".to_string(), "image".to_string(), "audio".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "whisper-1",
                "Whisper 1 (Audio)",
                "OpenAI",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                false,
            ),
        ],
        "claude" => vec![
            ModelInfo::with_details(
                "claude-3-5-haiku-20241022",
                "Claude 3.5 Haiku",
                "Anthropic Claude",
                vec!["text".to_string(), "image".to_string()],
                false,
                200_000,
                false,
            ),
            ModelInfo::with_details(
                "claude-3-5-sonnet-20241022",
                "Claude 3.5 Sonnet",
                "Anthropic Claude",
                vec!["text".to_string(), "image".to_string()],
                true,
                200_000,
                false,
            ),
        ],
        "mistral" => vec![
            ModelInfo::with_details(
                "mistral-small-latest",
                "Mistral Small",
                "Mistral AI",
                vec!["text".to_string()],
                false,
                32_768,
                true,
            ),
            ModelInfo::with_details(
                "codestral-latest",
                "Codestral",
                "Mistral AI",
                vec!["text".to_string()],
                false,
                32_768,
                false,
            ),
            ModelInfo::with_details(
                "mistral-large-latest",
                "Mistral Large",
                "Mistral AI",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
        ],
        "ollama" => vec![
            ModelInfo::with_details(
                "llama3.2:latest",
                "Llama 3.2",
                "Ollama",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            ModelInfo::with_details(
                "mistral:latest",
                "Mistral",
                "Ollama",
                vec!["text".to_string()],
                false,
                32_768,
                true,
            ),
            ModelInfo::with_details(
                "qwen2.5:latest",
                "Qwen 2.5",
                "Ollama",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
        ],
        "local_ai" => vec![
            ModelInfo::with_details(
                "base",
                "Local Speech Model (Base)",
                "Local AI",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "small",
                "Local Speech Model (Small)",
                "Local AI",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "medium",
                "Local Speech Model (Medium)",
                "Local AI",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "large-v3",
                "Local Speech Model (Large)",
                "Local AI",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
        ],
        "huggingface" => vec![
            ModelInfo::with_details(
                "openai/whisper-large-v3-turbo",
                "Whisper Large V3 Turbo (Audio)",
                "Hugging Face",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "openai/whisper-large-v3",
                "Whisper Large V3 (Audio)",
                "Hugging Face",
                vec!["audio".to_string(), "text".to_string()],
                false,
                25_000,
                true,
            ),
            ModelInfo::with_details(
                "meta-llama/Llama-3.3-70B-Instruct",
                "Llama 3.3 70B Instruct",
                "Hugging Face",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "Qwen/Qwen2.5-72B-Instruct",
                "Qwen 2.5 72B Instruct",
                "Hugging Face",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
            ModelInfo::with_details(
                "deepseek-ai/DeepSeek-R1",
                "DeepSeek R1",
                "Hugging Face",
                vec!["text".to_string()],
                true,
                163_840,
                false,
            ),
            ModelInfo::with_details(
                "google/gemma-3-27b-it",
                "Gemma 3 27B IT",
                "Hugging Face",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
        ],
        _ => vec![],
    }
}

/// Helper to determine whether an AI model belongs to a free tier.
pub fn is_model_free(provider: &str, model_id: &str) -> bool {
    let lower_id = model_id.to_lowercase();
    if lower_id.contains(":free")
        || lower_id.contains("-free")
        || lower_id.contains("/free")
        || lower_id.ends_with("free")
        || lower_id.contains("contributor")
        || lower_id.contains("pickle")
    {
        return true;
    }
    match provider.trim().to_lowercase().as_str() {
        "ollama" | "local_ai" => true, // Local self-hosted models are 100% free
        "groq" => lower_id.contains("whisper"),
        "cloudflare" => lower_id.contains("whisper") || lower_id.contains("tiny"),
        "huggingface" => lower_id.contains("whisper"),
        "gemini" => lower_id.contains("flash") || lower_id.contains("exp"),
        "cohere" => lower_id.contains("light") || lower_id.contains("r7b"),
        _ => false,
    }
}

/// Asynchronously queries the respective AI Provider with the given API key,
/// returning the live list of models provided by that provider with rich capabilities metadata.
pub async fn fetch_models_for_provider(
    provider: &str,
    api_key: &str,
) -> Result<Vec<ModelInfo>, AiError> {
    let normalized = provider.trim().to_lowercase();
    let key = api_key.trim();
    if key.is_empty() && normalized != "ollama" {
        return Err(AiError::MissingApiKey);
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("opencode/1.18.33")
        .build()
        .unwrap_or_default();

    let mut model_infos: Vec<ModelInfo> = match normalized.as_str() {
        "openrouter" => {
            let res = client
                .get("https://openrouter.ai/api/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "OpenRouter API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&id)
                        .to_string();
                    let context_limit = item
                        .get("context_length")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(128_000);

                    let is_free = id.ends_with(":free")
                        || (item
                            .get("pricing")
                            .and_then(|p| p.get("prompt"))
                            .and_then(|p| p.as_str())
                            == Some("0")
                            && item
                                .get("pricing")
                                .and_then(|p| p.get("completion"))
                                .and_then(|p| p.as_str())
                                == Some("0"));

                    let inputs = if let Some(mods) = item
                        .get("architecture")
                        .and_then(|a| a.get("input_modalities"))
                        .and_then(|m| m.as_array())
                    {
                        mods.iter()
                            .filter_map(|m| m.as_str().map(|s| s.to_string()))
                            .collect()
                    } else {
                        detect_inputs(&id)
                    };

                    let reasoning = item.get("reasoning").is_some()
                        || item
                            .get("supported_parameters")
                            .and_then(|p| p.as_array())
                            .map(|arr| arr.iter().any(|p| p.as_str() == Some("reasoning")))
                            .unwrap_or(false)
                        || detect_reasoning(&id);

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "OpenRouter",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "kilocode" => {
            let res = client
                .get("https://api.kilo.ai/api/gateway/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "Kilo Code API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&id)
                        .to_string();
                    let context_limit = item
                        .get("context_length")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(128_000);
                    let is_free = item
                        .get("isFree")
                        .and_then(|v| v.as_bool())
                        .unwrap_or_else(|| id.ends_with(":free"));

                    let inputs = if let Some(mods) = item
                        .get("architecture")
                        .and_then(|a| a.get("input_modalities"))
                        .and_then(|m| m.as_array())
                    {
                        mods.iter()
                            .filter_map(|m| m.as_str().map(|s| s.to_string()))
                            .collect()
                    } else {
                        detect_inputs(&id)
                    };

                    let reasoning = item
                        .get("supported_parameters")
                        .and_then(|p| p.as_array())
                        .map(|arr| arr.iter().any(|p| p.as_str() == Some("reasoning")))
                        .unwrap_or(false)
                        || detect_reasoning(&id);

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "Kilo Code",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "cohere" => {
            let res = client
                .get("https://api.cohere.com/v2/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "Cohere API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("models").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = id.clone();
                    let context_limit = item
                        .get("context_length")
                        .and_then(|v| v.as_u64())
                        .filter(|&v| v > 0)
                        .unwrap_or_else(|| detect_context_limit(&id));
                    let is_free = is_model_free("cohere", &id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "Cohere",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "groq" => {
            let res = client
                .get("https://api.groq.com/openai/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "Groq API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = format_model_name(&id);
                    let context_limit = item
                        .get("context_window")
                        .and_then(|v| v.as_u64())
                        .unwrap_or_else(|| detect_context_limit(&id));
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = if let Some(pricing) = item.get("pricing") {
                        let prompt_str = pricing
                            .get("prompt")
                            .and_then(|p| p.as_str())
                            .unwrap_or("0");
                        let comp_str = pricing
                            .get("completion")
                            .and_then(|p| p.as_str())
                            .unwrap_or("0");
                        let prompt_num: f64 = prompt_str.parse().unwrap_or(0.0);
                        let comp_num: f64 = comp_str.parse().unwrap_or(0.0);
                        (prompt_num == 0.0 && comp_num == 0.0) || id.contains("whisper")
                    } else {
                        id.contains("whisper") || id.ends_with(":free") || id.contains("-free")
                    };

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "Groq",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "nvidia" => {
            let res = client
                .get("https://integrate.api.nvidia.com/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "NVIDIA NIM API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = format_model_name(&id);
                    let context_limit = detect_context_limit(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = item
                        .get("is_free")
                        .and_then(|v| v.as_bool())
                        .or_else(|| {
                            item.get("pricing")
                                .and_then(|p| p.get("prompt"))
                                .and_then(|p| p.as_str())
                                .map(|s| s == "0")
                        })
                        .unwrap_or_else(|| {
                            id.ends_with(":free") || id.contains("-free") || id.contains("/free")
                        });

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "NVIDIA NIM",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "opencode" => {
            let res = client
                .get("https://opencode.ai/zen/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "OpenCode Zen API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = format_model_name(&id);
                    let context_limit = detect_context_limit(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = is_model_free("opencode", &id);

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "OpenCode Zen",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "cerebras" => {
            let res = client
                .get("https://api.cerebras.ai/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await?;

            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "Cerebras API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
                for item in arr {
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let name = format_model_name(&id);
                    let context_limit = 128_000;
                    let inputs = vec!["text".to_string()];
                    let reasoning = false;
                    let is_free = item
                        .get("is_free")
                        .and_then(|v| v.as_bool())
                        .unwrap_or_else(|| id.ends_with(":free") || id.contains("-free"));

                    list.push(ModelInfo::with_details(
                        id,
                        name,
                        "Cerebras",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "gemini" => {
            let url = "https://generativelanguage.googleapis.com/v1beta/models";
            let res = client.get(url).header("x-goog-api-key", key).send().await?;
            if !res.status().is_success() {
                return Err(AiError::ApiError(format!(
                    "Gemini API returned HTTP {}",
                    res.status()
                )));
            }

            let json: serde_json::Value = res.json().await?;
            let mut list = Vec::new();
            if let Some(arr) = json.get("models").and_then(|v| v.as_array()) {
                for item in arr {
                    let raw_name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let id = raw_name
                        .strip_prefix("models/")
                        .unwrap_or(raw_name)
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    let display_name = item
                        .get("displayName")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&id)
                        .to_string();
                    let context_limit = item
                        .get("inputTokenLimit")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1_048_576);
                    let inputs = vec![
                        "text".to_string(),
                        "image".to_string(),
                        "audio".to_string(),
                        "video".to_string(),
                    ];
                    let reasoning = id.contains("thinking");
                    let is_free = is_model_free("gemini", &id);

                    list.push(ModelInfo::with_details(
                        id,
                        display_name,
                        "Google Gemini",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    ));
                }
            }
            list
        }
        "cloudflare" => {
            let account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();
            let p = cloudflare::CloudflareProvider::with_client(
                key.to_string(),
                account_id,
                None,
                client,
            );
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = is_model_free("cloudflare", &id);
                    ModelInfo::with_details(
                        id,
                        name,
                        "Cloudflare",
                        inputs,
                        reasoning,
                        128_000,
                        is_free,
                    )
                })
                .collect()
        }
        "ollama" => {
            let host = if key.starts_with("http") {
                Some(key.to_string())
            } else {
                std::env::var("OLLAMA_HOST").ok()
            };
            let p = ollama::OllamaProvider::with_client(host, None, client);
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    ModelInfo::with_details(id, name, "Ollama", inputs, reasoning, 128_000, true)
                })
                .collect()
        }
        "openai" => {
            let p = openai::OpenAiProvider::with_client(key.to_string(), None, client);
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = false;
                    ModelInfo::with_details(id, name, "OpenAI", inputs, reasoning, 128_000, is_free)
                })
                .collect()
        }
        "claude" => {
            let p = claude::ClaudeProvider::with_client(key.to_string(), None, client);
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = false;
                    ModelInfo::with_details(
                        id,
                        name,
                        "Anthropic Claude",
                        inputs,
                        reasoning,
                        200_000,
                        is_free,
                    )
                })
                .collect()
        }
        "mistral" => {
            let p = mistral::MistralProvider::with_client(key.to_string(), None, client);
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = is_model_free("mistral", &id);
                    ModelInfo::with_details(
                        id,
                        name,
                        "Mistral AI",
                        inputs,
                        reasoning,
                        32_768,
                        is_free,
                    )
                })
                .collect()
        }
        "huggingface" => {
            let p = huggingface::HuggingFaceProvider::with_client(key.to_string(), None, client);
            let raw = p.list_models().await?;
            raw.into_iter()
                .map(|id| {
                    let name = format_model_name(&id);
                    let inputs = detect_inputs(&id);
                    let reasoning = detect_reasoning(&id);
                    let is_free = is_model_free("huggingface", &id);
                    let context_limit = detect_context_limit(&id);
                    ModelInfo::with_details(
                        id,
                        name,
                        "Hugging Face",
                        inputs,
                        reasoning,
                        context_limit,
                        is_free,
                    )
                })
                .collect()
        }
        _ => {
            return Err(AiError::UnsupportedOperation(format!(
                "Unsupported provider: {}",
                provider
            )));
        }
    };

    // Prioritize free models first, then sort alphabetically by ID
    model_infos.sort_by(|a, b| b.is_free.cmp(&a.is_free).then_with(|| a.id.cmp(&b.id)));

    Ok(model_infos)
}

/// Usage and rate limit details for a specific Cloud AI provider, API key, and model.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModelUsageLimitInfo {
    pub provider: String,
    pub model_id: String,
    pub key_status: String,
    pub requests_per_minute: String,
    pub requests_per_day: String,
    pub tokens_per_minute: String,
    pub context_or_audio_limit: String,
    pub remaining_quota: String,
}

/// Returns baseline documented rate and usage limits for a given provider and model ID.
pub fn default_model_usage_limits(
    provider: &str,
    model_id: &str,
    has_api_key: bool,
) -> ModelUsageLimitInfo {
    let norm_prov = provider.trim().to_lowercase();
    let lower_model = model_id.trim().to_lowercase();
    let key_status = if norm_prov == "ollama" {
        "Local Ollama Server".to_string()
    } else if has_api_key {
        "API Key Configured".to_string()
    } else {
        "Standard Tier".to_string()
    };

    match norm_prov.as_str() {
        "groq" => {
            if lower_model.contains("whisper") {
                ModelUsageLimitInfo {
                    provider: "Groq".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "20 RPM".to_string(),
                    requests_per_day: "2,000 RPD".to_string(),
                    tokens_per_minute: "7,200s / hr".to_string(),
                    context_or_audio_limit: "25 MB max".to_string(),
                    remaining_quota: "2,000 RPD".to_string(),
                }
            } else if lower_model.contains("70b") {
                ModelUsageLimitInfo {
                    provider: "Groq".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "30 RPM".to_string(),
                    requests_per_day: "1,000 RPD".to_string(),
                    tokens_per_minute: "12,000 TPM".to_string(),
                    context_or_audio_limit: "128,000 tokens".to_string(),
                    remaining_quota: "1,000 RPD".to_string(),
                }
            } else {
                ModelUsageLimitInfo {
                    provider: "Groq".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "30 RPM".to_string(),
                    requests_per_day: "14,400 RPD".to_string(),
                    tokens_per_minute: "6,000 TPM".to_string(),
                    context_or_audio_limit: "128,000 tokens".to_string(),
                    remaining_quota: "14,400 RPD".to_string(),
                }
            }
        }
        "gemini" => {
            if lower_model.contains("pro") {
                ModelUsageLimitInfo {
                    provider: "Google Gemini".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "2 RPM".to_string(),
                    requests_per_day: "50 RPD".to_string(),
                    tokens_per_minute: "32,000 TPM".to_string(),
                    context_or_audio_limit: "2,097,152 tokens".to_string(),
                    remaining_quota: "50 RPD".to_string(),
                }
            } else {
                ModelUsageLimitInfo {
                    provider: "Google Gemini".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "15 RPM".to_string(),
                    requests_per_day: "1,500 RPD".to_string(),
                    tokens_per_minute: "1,000,000 TPM".to_string(),
                    context_or_audio_limit: "1,048,576 tokens".to_string(),
                    remaining_quota: "1,500 RPD".to_string(),
                }
            }
        }
        "openrouter" => {
            let is_free = lower_model.ends_with(":free") || is_model_free("openrouter", model_id);
            ModelUsageLimitInfo {
                provider: "OpenRouter".to_string(),
                model_id: model_id.to_string(),
                key_status,
                requests_per_minute: if is_free {
                    "20 RPM".to_string()
                } else {
                    "200 RPM".to_string()
                },
                requests_per_day: if is_free {
                    "1,000 RPD".to_string()
                } else {
                    "Unlimited".to_string()
                },
                tokens_per_minute: "Dynamic".to_string(),
                context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
                remaining_quota: "Credit balance".to_string(),
            }
        }
        "openai" => {
            if lower_model.contains("whisper") {
                ModelUsageLimitInfo {
                    provider: "OpenAI".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "50 RPM".to_string(),
                    requests_per_day: "10,000 RPD".to_string(),
                    tokens_per_minute: "25 MB / req".to_string(),
                    context_or_audio_limit: "25 MB max".to_string(),
                    remaining_quota: "10,000 RPD".to_string(),
                }
            } else {
                ModelUsageLimitInfo {
                    provider: "OpenAI".to_string(),
                    model_id: model_id.to_string(),
                    key_status,
                    requests_per_minute: "500 RPM".to_string(),
                    requests_per_day: "10,000 RPD".to_string(),
                    tokens_per_minute: "200,000 TPM".to_string(),
                    context_or_audio_limit: "128,000 tokens".to_string(),
                    remaining_quota: "10,000 RPD".to_string(),
                }
            }
        }
        "claude" => ModelUsageLimitInfo {
            provider: "Anthropic Claude".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "50 RPM".to_string(),
            requests_per_day: "1,000 RPD".to_string(),
            tokens_per_minute: "50,000 TPM".to_string(),
            context_or_audio_limit: "200,000 tokens".to_string(),
            remaining_quota: "1,000 RPD".to_string(),
        },
        "mistral" => ModelUsageLimitInfo {
            provider: "Mistral AI".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "60 RPM".to_string(),
            requests_per_day: "2,000 RPD".to_string(),
            tokens_per_minute: "500,000 TPM".to_string(),
            context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
            remaining_quota: "2,000 RPD".to_string(),
        },
        "cerebras" => ModelUsageLimitInfo {
            provider: "Cerebras".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "30 RPM".to_string(),
            requests_per_day: "14,400 RPD".to_string(),
            tokens_per_minute: "60,000 TPM".to_string(),
            context_or_audio_limit: "128,000 tokens".to_string(),
            remaining_quota: "14,400 RPD".to_string(),
        },
        "cohere" => ModelUsageLimitInfo {
            provider: "Cohere".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "20 RPM".to_string(),
            requests_per_day: "1,000 RPD".to_string(),
            tokens_per_minute: "100,000 TPM".to_string(),
            context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
            remaining_quota: "1,000 RPD".to_string(),
        },
        "cloudflare" => ModelUsageLimitInfo {
            provider: "Cloudflare Workers AI".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: if lower_model.contains("whisper") {
                "300 RPM".to_string()
            } else {
                "720 RPM".to_string()
            },
            requests_per_day: "10,000 Neurons".to_string(),
            tokens_per_minute: "100,000 TPM".to_string(),
            context_or_audio_limit: "128,000 tokens".to_string(),
            remaining_quota: "10,000 Neurons".to_string(),
        },
        "huggingface" => ModelUsageLimitInfo {
            provider: "Hugging Face".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "30 RPM".to_string(),
            requests_per_day: "1,000 RPD".to_string(),
            tokens_per_minute: "100,000 TPM".to_string(),
            context_or_audio_limit: "128,000 tokens".to_string(),
            remaining_quota: "1,000 RPD".to_string(),
        },
        "nvidia" => ModelUsageLimitInfo {
            provider: "NVIDIA NIM".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "40 RPM".to_string(),
            requests_per_day: "1,000 Credits".to_string(),
            tokens_per_minute: "100,000 TPM".to_string(),
            context_or_audio_limit: "128,000 tokens".to_string(),
            remaining_quota: "1,000 Credits".to_string(),
        },
        "kilocode" => ModelUsageLimitInfo {
            provider: "Kilo Code".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "20 RPM".to_string(),
            requests_per_day: "1,000 RPD".to_string(),
            tokens_per_minute: "200,000 TPM".to_string(),
            context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
            remaining_quota: "1,000 RPD".to_string(),
        },
        "opencode" => ModelUsageLimitInfo {
            provider: "OpenCode Zen".to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "30 RPM".to_string(),
            requests_per_day: "1,500 RPD".to_string(),
            tokens_per_minute: "150,000 TPM".to_string(),
            context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
            remaining_quota: "1,500 RPD".to_string(),
        },
        "ollama" => ModelUsageLimitInfo {
            provider: "Ollama (Local)".to_string(),
            model_id: model_id.to_string(),
            key_status: "Local Server".to_string(),
            requests_per_minute: "Unlimited".to_string(),
            requests_per_day: "Unlimited".to_string(),
            tokens_per_minute: "Local Hardware".to_string(),
            context_or_audio_limit: "128,000 tokens".to_string(),
            remaining_quota: "Unlimited".to_string(),
        },
        _ => ModelUsageLimitInfo {
            provider: provider.to_string(),
            model_id: model_id.to_string(),
            key_status,
            requests_per_minute: "30 RPM".to_string(),
            requests_per_day: "1,000 RPD".to_string(),
            tokens_per_minute: "100,000 TPM".to_string(),
            context_or_audio_limit: format!("{} tokens", detect_context_limit(model_id)),
            remaining_quota: "1,000 RPD".to_string(),
        },
    }
}

/// Helper to extract an HTTP header string from a `reqwest::header::HeaderMap`.
fn header_val(headers: &reqwest::header::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Fetches live usage limits, rate limits, and remaining quota for the specified provider,
/// API key, and model ID by querying the provider's API endpoints and HTTP rate-limit headers.
pub async fn fetch_model_usage_limits(
    provider: &str,
    api_key: &str,
    model_id: &str,
) -> Result<ModelUsageLimitInfo, AiError> {
    let normalized = provider.trim().to_lowercase();
    let key = api_key.trim();
    let effective_model = if model_id.trim().is_empty() {
        get_provider_models(&normalized)
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_else(|| "default".to_string())
    } else {
        model_id.trim().to_string()
    };

    if key.is_empty() && normalized != "ollama" {
        return Ok(default_model_usage_limits(
            &normalized,
            &effective_model,
            false,
        ));
    }

    let mut info = default_model_usage_limits(&normalized, &effective_model, true);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .user_agent("OpenDictate/2.0.0")
        .build()
        .unwrap_or_default();

    match normalized.as_str() {
        "openrouter" => {
            let res = client
                .get("https://openrouter.ai/api/v1/key")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;

            if !res.status().is_success() {
                info.key_status = format!("Invalid or Unauthorized Key (HTTP {})", res.status());
                return Ok(info);
            }

            let json: serde_json::Value = res.json().await?;
            if let Some(data) = json.get("data") {
                let is_free_tier = data
                    .get("is_free_tier")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let usage = data.get("usage").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let limit = data.get("limit").and_then(|v| v.as_f64());
                let limit_rem = data.get("limit_remaining").and_then(|v| v.as_f64());

                info.key_status = if is_free_tier {
                    "Verified (Free Tier Key)".to_string()
                } else {
                    "Verified (Paid Credits Key)".to_string()
                };

                if let Some(rl) = data.get("rate_limit") {
                    let reqs = rl.get("requests").and_then(|v| v.as_u64()).unwrap_or(20);
                    let interval = rl.get("interval").and_then(|v| v.as_str()).unwrap_or("10s");
                    info.requests_per_minute = format!("{} req / {}", reqs, interval);
                }

                info.requests_per_day = if effective_model.ends_with(":free") {
                    if is_free_tier {
                        "50 RPD (Free model tier)".to_string()
                    } else {
                        "1,000 RPD (Paid account on :free model)".to_string()
                    }
                } else {
                    "Unlimited (Credit balance)".to_string()
                };

                info.remaining_quota = match (limit_rem, limit) {
                    (Some(rem), Some(lim)) => {
                        format!("${:.2} remaining of ${:.2} (Used: ${:.2})", rem, lim, usage)
                    }
                    _ => format!("Used: ${:.4} (No hard cap set)", usage),
                };
            }
            Ok(info)
        }
        "gemini" => {
            let clean_model = effective_model
                .strip_prefix("models/")
                .unwrap_or(&effective_model);
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}",
                clean_model
            );
            let res = client
                .get(&url)
                .header("x-goog-api-key", key)
                .send()
                .await?;
            if !res.status().is_success() {
                info.key_status = format!("API Error (HTTP {})", res.status());
                return Ok(info);
            }
            let json: serde_json::Value = res.json().await?;
            let in_limit = json
                .get("inputTokenLimit")
                .and_then(|v| v.as_u64())
                .unwrap_or(1_048_576);
            let out_limit = json
                .get("outputTokenLimit")
                .and_then(|v| v.as_u64())
                .unwrap_or(8_192);
            info.key_status = "Verified (Active Gemini API Key)".to_string();
            info.context_or_audio_limit =
                format!("{} input / {} output tokens", in_limit, out_limit);
            info.remaining_quota = "Active (Quota managed by Google AI Studio)".to_string();
            Ok(info)
        }
        "groq" => {
            let res = client
                .get("https://api.groq.com/openai/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;
            if !res.status().is_success() {
                info.key_status = format!("Invalid or Unauthorized Key (HTTP {})", res.status());
                return Ok(info);
            }
            let headers = res.headers().clone();
            if let Some(lim_req) = header_val(&headers, "x-ratelimit-limit-requests") {
                info.requests_per_day = format!("{} RPD", lim_req);
            }
            if let Some(lim_tok) = header_val(&headers, "x-ratelimit-limit-tokens") {
                info.tokens_per_minute = format!("{} TPM", lim_tok);
            }
            let rem_req = header_val(&headers, "x-ratelimit-remaining-requests");
            let rem_tok = header_val(&headers, "x-ratelimit-remaining-tokens");
            let json: serde_json::Value = res.json().await?;
            if let Some(arr) = json.get("data").and_then(|v| v.as_array())
                && let Some(m) = arr.iter().find(|item| {
                    item.get("id")
                        .and_then(|v| v.as_str())
                        .map(|id| id.eq_ignore_ascii_case(&effective_model))
                        .unwrap_or(false)
                })
                && let Some(cw) = m.get("context_window").and_then(|v| v.as_u64())
                && !effective_model.to_lowercase().contains("whisper")
            {
                info.context_or_audio_limit = format!("{} context window", cw);
            }
            info.key_status = "Verified (Active Groq API Key)".to_string();
            info.remaining_quota = match (rem_req, rem_tok) {
                (Some(r), Some(t)) => format!("{} req / {} tokens remaining", r, t),
                (Some(r), None) => format!("{} requests remaining", r),
                _ => "Active (Full Groq tier allowance available)".to_string(),
            };
            Ok(info)
        }
        "huggingface" => {
            let res = client
                .get("https://huggingface.co/api/whoami-v2")
                .header("Authorization", format!("Bearer {}", key))
                .send()
                .await?;
            if !res.status().is_success() {
                info.key_status = format!("Invalid HF Token (HTTP {})", res.status());
                return Ok(info);
            }
            let headers = res.headers().clone();
            let ratelimit_rem = header_val(&headers, "ratelimit")
                .or_else(|| header_val(&headers, "x-ratelimit-remaining"));
            let json: serde_json::Value = res.json().await?;
            let user = json.get("name").and_then(|v| v.as_str()).unwrap_or("User");
            let is_pro = json.get("isPro").and_then(|v| v.as_bool()).unwrap_or(false);
            info.key_status = if is_pro {
                format!("Verified (@{} • HF PRO)", user)
            } else {
                format!("Verified (@{} • Free Tier)", user)
            };
            info.requests_per_minute = if is_pro {
                "300 RPM (HF PRO)".to_string()
            } else {
                "30 RPM (Free Tier)".to_string()
            };
            info.requests_per_day = if is_pro {
                "20,000 RPD (HF PRO)".to_string()
            } else {
                "1,000 RPD (Free Tier)".to_string()
            };
            info.remaining_quota =
                ratelimit_rem.unwrap_or_else(|| "Active Hugging Face Serverless quota".to_string());
            Ok(info)
        }
        "ollama" => {
            let host = if key.starts_with("http") {
                key.to_string()
            } else {
                std::env::var("OLLAMA_HOST")
                    .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string())
            };
            let url = format!("{}/api/tags", host.trim_end_matches('/'));
            match client.get(&url).send().await {
                Ok(res) if res.status().is_success() => {
                    info.key_status = format!("Connected ({})", host);
                    info.remaining_quota = "Unlimited local execution".to_string();
                }
                _ => {
                    info.key_status = format!("Offline ({})", host);
                }
            }
            Ok(info)
        }
        _ => {
            // Verify API key and inspect rate-limit headers via provider models endpoint
            let endpoint = match normalized.as_str() {
                "openai" => Some("https://api.openai.com/v1/models"),
                "claude" => Some("https://api.anthropic.com/v1/models"),
                "mistral" => Some("https://api.mistral.ai/v1/models"),
                "cerebras" => Some("https://api.cerebras.ai/v1/models"),
                "nvidia" => Some("https://integrate.api.nvidia.com/v1/models"),
                "cohere" => Some("https://api.cohere.com/v2/models"),
                "kilocode" => Some("https://api.kilo.ai/api/gateway/models"),
                "opencode" => Some("https://opencode.ai/zen/v1/models"),
                _ => None,
            };

            if let Some(url) = endpoint {
                let mut req = client.get(url);
                if normalized == "claude" {
                    req = req
                        .header("x-api-key", key)
                        .header("anthropic-version", "2023-06-01");
                } else {
                    req = req.header("Authorization", format!("Bearer {}", key));
                }

                if let Ok(res) = req.send().await {
                    if res.status().is_success() {
                        info.key_status = "Verified (Active API Key)".to_string();
                        let headers = res.headers();
                        if let Some(rpm) = header_val(headers, "x-ratelimit-limit-requests")
                            .or_else(|| header_val(headers, "anthropic-ratelimit-requests-limit"))
                        {
                            info.requests_per_minute = format!("{} req limit", rpm);
                        }
                        if let Some(tpm) = header_val(headers, "x-ratelimit-limit-tokens")
                            .or_else(|| header_val(headers, "anthropic-ratelimit-tokens-limit"))
                        {
                            info.tokens_per_minute = format!("{} token limit", tpm);
                        }
                        if let Some(rem) = header_val(headers, "x-ratelimit-remaining-requests")
                            .or_else(|| {
                                header_val(headers, "anthropic-ratelimit-requests-remaining")
                            })
                        {
                            info.remaining_quota = format!("{} requests remaining", rem);
                        } else {
                            info.remaining_quota = "Active (Verified API Key)".to_string();
                        }
                    } else {
                        info.key_status =
                            format!("Invalid or Unauthorized Key (HTTP {})", res.status());
                    }
                }
            }
            Ok(info)
        }
    }
}

/// Evaluates whether selecting the given model should trigger a user-facing warning.
pub fn evaluate_model_selection_warning(
    model: &ModelInfo,
    user_has_paid_plan: bool,
) -> Option<String> {
    if !model.is_free && !user_has_paid_plan {
        Some(PAID_MODEL_UNPAID_PLAN_WARNING.to_string())
    } else if !model.supports_audio() {
        Some(AUDIO_UNSUPPORTED_WARNING.to_string())
    } else {
        None
    }
}

/// Result of checking a model's audio capability and user account paid plan eligibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSelectionValidation {
    pub supports_audio: bool,
    pub is_paid_model: bool,
    pub user_has_paid_plan: bool,
    pub warning_message: Option<String>,
}

/// Asynchronously checks live capabilities and plan eligibility using the user's API key.
pub async fn verify_model_selection_live(
    provider: &str,
    api_key: &str,
    model_id: &str,
    cached_model: Option<&ModelInfo>,
) -> ModelSelectionValidation {
    let normalized = provider.trim().to_lowercase();
    let key = api_key.trim();

    // 1. Fetch live model metadata using the API key (fallback to cached_model if offline)
    let live_fetched_model = if !key.is_empty() && normalized != "local_ai" {
        fetch_models_for_provider(&normalized, key)
            .await
            .ok()
            .and_then(|list| list.into_iter().find(|m| m.id == model_id))
    } else {
        None
    };

    let effective_model = live_fetched_model.as_ref().or(cached_model);

    // 2. Determine if model is paid
    let is_paid_model = effective_model
        .map(|m| !m.is_free)
        .unwrap_or_else(|| !is_model_free(&normalized, model_id));

    // 3. Determine audio support from live model capabilities
    let supports_audio = effective_model
        .map(|m| m.supports_audio())
        .unwrap_or_else(|| {
            detect_inputs(model_id)
                .iter()
                .any(|i| i.eq_ignore_ascii_case("audio"))
        });

    // 4. Live check user's paid plan status if it's a paid model
    let mut user_has_paid_plan = false;
    if !is_paid_model || normalized == "ollama" || normalized == "local_ai" {
        user_has_paid_plan = true;
    } else if !key.is_empty() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(8))
            .user_agent("OpenDictate/2.0.0")
            .build()
            .unwrap_or_default();

        match normalized.as_str() {
            "openrouter" => {
                if let Ok(res) = client
                    .get("https://openrouter.ai/api/v1/key")
                    .header("Authorization", format!("Bearer {}", key))
                    .send()
                    .await
                    && res.status().is_success()
                    && let Ok(json) = res.json::<serde_json::Value>().await
                    && let Some(data) = json.get("data")
                {
                    let is_free_tier = data
                        .get("is_free_tier")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                    let limit_rem = data
                        .get("limit_remaining")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    if !is_free_tier || limit_rem > 0.0 {
                        user_has_paid_plan = true;
                    }
                }
            }
            "huggingface" => {
                if let Ok(res) = client
                    .get("https://huggingface.co/api/whoami-v2")
                    .header("Authorization", format!("Bearer {}", key))
                    .send()
                    .await
                    && res.status().is_success()
                    && let Ok(json) = res.json::<serde_json::Value>().await
                {
                    user_has_paid_plan =
                        json.get("isPro").and_then(|v| v.as_bool()).unwrap_or(false);
                }
            }
            _ => {
                if let Ok(limits) = fetch_model_usage_limits(&normalized, key, model_id).await {
                    let k = limits.key_status.to_lowercase();
                    if k.contains("paid") || k.contains("pro") {
                        user_has_paid_plan = true;
                    }
                }
            }
        }
    }

    let warning_message = if is_paid_model && !user_has_paid_plan {
        Some(PAID_MODEL_UNPAID_PLAN_WARNING.to_string())
    } else if !supports_audio {
        Some(AUDIO_UNSUPPORTED_WARNING.to_string())
    } else {
        None
    };

    ModelSelectionValidation {
        supports_audio,
        is_paid_model,
        user_has_paid_plan,
        warning_message,
    }
}

/// Unified asynchronous AI Provider trait.
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Transcribes WAV audio bytes into text with an optional system prompt.
    async fn transcribe(
        &self,
        audio_wav: Vec<u8>,
        system_prompt: Option<&str>,
    ) -> Result<String, AiError>;

    /// Enhances transcription text according to a tone transformation prompt.
    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError>;

    /// Lists supported or available models from the provider.
    async fn list_models(&self) -> Result<Vec<String>, AiError>;
}

/// AI Manager responsible for instantiating and configuring AI providers.
#[derive(Debug, Clone)]
pub struct AiManager {
    config: Option<Config>,
    client: reqwest::Client,
}

impl Default for AiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AiManager {
    /// Creates a new `AiManager` with default configuration and shared HTTP client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .user_agent("OpenDictate/2.0.0 (Linux; x86_64)")
            .build()
            .unwrap_or_default();
        Self {
            config: None,
            client,
        }
    }

    /// Creates a new `AiManager` populated with user configuration.
    pub fn with_config(config: &Config) -> Self {
        let mut mgr = Self::new();
        mgr.config = Some(config.clone());
        mgr
    }

    /// Creates a new `AiManager` from configuration reference.
    pub fn from_config(config: &Config) -> Self {
        Self::with_config(config)
    }

    /// Returns the active AI mode ("cloud" or "local").
    pub fn active_mode(&self) -> &str {
        self.config
            .as_ref()
            .map(|c| c.ai_mode.as_str())
            .unwrap_or("cloud")
    }

    fn redact_error(&self, err: AiError) -> AiError {
        let configured_key = self
            .config
            .as_ref()
            .map(|c| c.ai_api_key.as_str())
            .unwrap_or("");
        match err {
            AiError::ApiError(msg) => {
                AiError::ApiError(utils::redact_secrets(&msg, configured_key))
            }
            AiError::NetworkError(net_err) => {
                let raw = net_err.to_string();
                let redacted = utils::redact_secrets(&raw, configured_key);
                if redacted != raw {
                    AiError::ApiError(format!("Network error: {}", redacted))
                } else {
                    AiError::NetworkError(net_err)
                }
            }
            other => other,
        }
    }

    /// Transcribes audio bytes using the currently configured provider.
    pub async fn transcribe(&self, audio_wav: &[u8]) -> Result<String, AiError> {
        if self.active_mode() == "local" {
            let (model_id, custom_path, threads) = if let Some(ref cfg) = self.config {
                (
                    cfg.local_model_id.clone(),
                    cfg.local_custom_path.clone(),
                    cfg.local_threads,
                )
            } else {
                ("tiny.en".to_string(), None, 4)
            };
            let provider = local_ai::LocalAiProvider::with_config(model_id, custom_path, threads);
            return provider.transcribe(audio_wav.to_vec(), None).await;
        }

        let provider_name = self
            .config
            .as_ref()
            .map(|c| c.ai_provider.as_str())
            .unwrap_or("groq");
        let provider = self.get_provider(provider_name)?;
        let mut first_attempt = provider.transcribe(audio_wav.to_vec(), None).await;
        if let Err(ref e) = first_attempt
            && utils::is_transient_ai_error(&e.to_string())
        {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            first_attempt = provider.transcribe(audio_wav.to_vec(), None).await;
        }

        match first_attempt {
            Ok(text) => Ok(text),
            Err(AiError::UnsupportedOperation(msg)) => {
                // If primary provider is enhancement-only, fall back to available audio STT providers
                for fallback_name in ["groq", "cloudflare", "gemini", "openai", "huggingface"] {
                    if let Ok(stt_provider) = self.get_provider(fallback_name)
                        && let Ok(text) = stt_provider.transcribe(audio_wav.to_vec(), None).await
                    {
                        return Ok(text);
                    }
                }
                Err(AiError::UnsupportedOperation(msg))
            }
            Err(e) => Err(self.redact_error(e)),
        }
    }

    /// Enhances text with tone transformation using the currently configured provider.
    pub async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        if self.active_mode() == "local" {
            return Ok(utils::apply_smart_local_formatting(text, tone));
        }

        if text.trim().is_empty() {
            return Ok(String::new());
        }
        if tone.trim().eq_ignore_ascii_case("raw") {
            return Ok(text.to_string());
        }

        let provider_name = self
            .config
            .as_ref()
            .map(|c| c.ai_provider.as_str())
            .unwrap_or("groq");
        let provider = self.get_provider(provider_name)?;
        let mut res = provider.enhance(text, tone).await;
        if let Err(ref e) = res
            && utils::is_transient_ai_error(&e.to_string())
        {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            res = provider.enhance(text, tone).await;
        }

        match res {
            Ok(enhanced) => {
                if enhanced.trim().is_empty() {
                    Ok(text.to_string())
                } else {
                    Ok(enhanced)
                }
            }
            Err(e) => Err(self.redact_error(e)),
        }
    }

    /// Returns list of all supported AI provider identifiers.
    pub fn supported_providers(&self) -> Vec<String> {
        vec![
            "groq".to_string(),
            "openai".to_string(),
            "claude".to_string(),
            "gemini".to_string(),
            "mistral".to_string(),
            "openrouter".to_string(),
            "cloudflare".to_string(),
            "nvidia".to_string(),
            "cohere".to_string(),
            "kilocode".to_string(),
            "cerebras".to_string(),
            "opencode".to_string(),
            "huggingface".to_string(),
            "ollama".to_string(),
        ]
    }

    /// Formats standardized instructions for text cleanup and tone transformation.
    pub fn format_enhancement_prompt(text: &str, tone: &str) -> String {
        utils::format_enhancement_prompt(text, tone)
    }

    /// Resolves and returns an instantiated AI Provider by identifier.
    pub fn get_provider(&self, name: &str) -> Result<Arc<dyn AiProvider>, AiError> {
        let normalized = name.trim().to_lowercase();

        let (api_key, model) = if let Some(ref cfg) = self.config {
            if cfg.ai_provider.trim().eq_ignore_ascii_case(&normalized) {
                let key = if cfg.ai_api_key.trim().is_empty() {
                    Self::env_api_key(&normalized)
                } else {
                    cfg.ai_api_key.clone()
                };
                let model = if cfg.ai_model.trim().is_empty() {
                    None
                } else {
                    Some(cfg.ai_model.clone())
                };
                (key, model)
            } else {
                (Self::env_api_key(&normalized), None)
            }
        } else {
            (Self::env_api_key(&normalized), None)
        };

        match normalized.as_str() {
            "gemini" => Ok(Arc::new(gemini::GeminiProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "groq" => Ok(Arc::new(groq::GroqProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "openai" => Ok(Arc::new(openai::OpenAiProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "claude" => Ok(Arc::new(claude::ClaudeProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "ollama" => {
                let host = std::env::var("OLLAMA_HOST").ok();
                Ok(Arc::new(ollama::OllamaProvider::with_client(
                    host,
                    model,
                    self.client.clone(),
                )))
            }
            "cloudflare" => {
                let account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();
                Ok(Arc::new(cloudflare::CloudflareProvider::with_client(
                    api_key,
                    account_id,
                    model,
                    self.client.clone(),
                )))
            }
            "mistral" => Ok(Arc::new(mistral::MistralProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "openrouter" => Ok(Arc::new(openrouter::OpenRouterProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "nvidia" => Ok(Arc::new(nvidia::NvidiaProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "cohere" => Ok(Arc::new(cohere::CohereProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "kilocode" => Ok(Arc::new(kilocode::KiloCodeProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "cerebras" => Ok(Arc::new(cerebras::CerebrasProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "opencode" => Ok(Arc::new(opencode::OpenCodeProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "huggingface" => Ok(Arc::new(huggingface::HuggingFaceProvider::with_client(
                api_key,
                model,
                self.client.clone(),
            ))),
            "local_ai" => {
                let provider = if let Some(ref cfg) = self.config {
                    local_ai::LocalAiProvider::with_config(
                        cfg.local_model_id.clone(),
                        cfg.local_custom_path.clone(),
                        cfg.local_threads,
                    )
                } else {
                    local_ai::LocalAiProvider::new(model)
                };
                Ok(Arc::new(provider))
            }
            _ => Err(AiError::UnsupportedOperation(format!(
                "Unsupported AI provider: {}",
                name
            ))),
        }
    }

    fn env_api_key(provider: &str) -> String {
        match provider {
            "gemini" => std::env::var("GEMINI_API_KEY").unwrap_or_default(),
            "groq" => std::env::var("GROQ_API_KEY").unwrap_or_default(),
            "openai" => std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            "claude" => std::env::var("ANTHROPIC_API_KEY")
                .or_else(|_| std::env::var("CLAUDE_API_KEY"))
                .unwrap_or_default(),
            "cloudflare" => std::env::var("CLOUDFLARE_API_KEY")
                .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
                .unwrap_or_default(),
            "mistral" => std::env::var("MISTRAL_API_KEY").unwrap_or_default(),
            "openrouter" => std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            "nvidia" => std::env::var("NVIDIA_API_KEY").unwrap_or_default(),
            "cohere" => std::env::var("COHERE_API_KEY").unwrap_or_default(),
            "kilocode" => std::env::var("KILOCODE_API_KEY").unwrap_or_default(),
            "cerebras" => std::env::var("CEREBRAS_API_KEY").unwrap_or_default(),
            "opencode" => std::env::var("OPENCODE_API_KEY").unwrap_or_default(),
            "huggingface" => std::env::var("HUGGINGFACE_API_KEY")
                .or_else(|_| std::env::var("HF_TOKEN"))
                .unwrap_or_default(),
            _ => String::new(),
        }
    }
}
