//! Local AI Provider implementation (Speech-to-Text).
//!
//! Provides in-process, on-device local speech recognition via embedded engine
//! and model management (catalog, path resolution, atomic downloads).

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

use super::utils::apply_smart_local_formatting;
use super::{AiError, AiProvider};

/// Information describing a local speech recognition model.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LocalModelInfo {
    pub id: String,
    pub label: String,
    pub filename: String,
    pub url: String,
    pub size_mb: u64,
    pub parameters: String,
    pub ram_mb: u64,
    pub languages: String,
    pub speed: String,
    pub license: String,
    pub source: String,
}

/// Returns the official catalog of supported local speech models.
pub fn get_local_model_catalog() -> Vec<LocalModelInfo> {
    const BASE_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";
    vec![
        LocalModelInfo {
            id: "tiny.en".to_string(),
            label: "Tiny (English only)".to_string(),
            filename: "ggml-tiny.en.bin".to_string(),
            url: format!("{}/ggml-tiny.en.bin", BASE_URL),
            size_mb: 78,
            parameters: "39M".to_string(),
            ram_mb: 273,
            languages: "English only".to_string(),
            speed: "~32x real-time (Fastest)".to_string(),
            license: "MIT / OpenAI".to_string(),
            source: "ggerganov/whisper.cpp".to_string(),
        },
        LocalModelInfo {
            id: "tiny".to_string(),
            label: "Tiny (Multilingual)".to_string(),
            filename: "ggml-tiny.bin".to_string(),
            url: format!("{}/ggml-tiny.bin", BASE_URL),
            size_mb: 78,
            parameters: "39M".to_string(),
            ram_mb: 273,
            languages: "99 Multilingual languages".to_string(),
            speed: "~32x real-time (Fastest)".to_string(),
            license: "MIT / OpenAI".to_string(),
            source: "ggerganov/whisper.cpp".to_string(),
        },
        LocalModelInfo {
            id: "base.en".to_string(),
            label: "Base (English only)".to_string(),
            filename: "ggml-base.en.bin".to_string(),
            url: format!("{}/ggml-base.en.bin", BASE_URL),
            size_mb: 148,
            parameters: "74M".to_string(),
            ram_mb: 388,
            languages: "English only".to_string(),
            speed: "~16x real-time (Fast)".to_string(),
            license: "MIT / OpenAI".to_string(),
            source: "ggerganov/whisper.cpp".to_string(),
        },
        LocalModelInfo {
            id: "base".to_string(),
            label: "Base (Multilingual)".to_string(),
            filename: "ggml-base.bin".to_string(),
            url: format!("{}/ggml-base.bin", BASE_URL),
            size_mb: 148,
            parameters: "74M".to_string(),
            ram_mb: 388,
            languages: "99 Multilingual languages".to_string(),
            speed: "~16x real-time (Fast)".to_string(),
            license: "MIT / OpenAI".to_string(),
            source: "ggerganov/whisper.cpp".to_string(),
        },
        LocalModelInfo {
            id: "small".to_string(),
            label: "Small (Multilingual)".to_string(),
            filename: "ggml-small.bin".to_string(),
            url: format!("{}/ggml-small.bin", BASE_URL),
            size_mb: 488,
            parameters: "244M".to_string(),
            ram_mb: 852,
            languages: "99 Multilingual languages".to_string(),
            speed: "~6x real-time (Standard)".to_string(),
            license: "MIT / OpenAI".to_string(),
            source: "ggerganov/whisper.cpp".to_string(),
        },
    ]
}

/// Returns the directory where local speech models are stored (`~/.local/share/opendictate/models/`).
/// Creates the directory if it does not exist.
pub fn models_dir() -> PathBuf {
    let dir = if let Some(base_dirs) = directories::BaseDirs::new() {
        base_dirs
            .data_local_dir()
            .join("opendictate")
            .join("models")
    } else {
        std::env::var("HOME")
            .map(|h| PathBuf::from(h).join(".local/share/opendictate/models"))
            .unwrap_or_else(|_| PathBuf::from("/tmp/opendictate/models"))
    };
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Resolves the filesystem path for a given model ID or custom file path.
pub fn resolve_model_path(model_id: &str, custom_path: Option<&str>) -> PathBuf {
    if let Some(cp) = custom_path
        && !cp.trim().is_empty()
        && (model_id == "custom" || model_id.is_empty())
    {
        return PathBuf::from(cp);
    }

    let catalog = get_local_model_catalog();
    if let Some(info) = catalog.iter().find(|m| m.id == model_id) {
        return models_dir().join(&info.filename);
    }

    if let Some(cp) = custom_path
        && !cp.trim().is_empty()
    {
        return PathBuf::from(cp);
    }

    if model_id.ends_with(".bin") {
        models_dir().join(model_id)
    } else {
        models_dir().join(format!("ggml-{}.bin", model_id))
    }
}

/// Checks whether the model file for the given model ID exists on disk.
pub fn is_model_installed(model_id: &str, custom_path: Option<&str>) -> bool {
    let path = resolve_model_path(model_id, custom_path);
    path.is_file()
        && std::fs::metadata(&path)
            .map(|m| m.len() > 0)
            .unwrap_or(false)
}

/// Decodes WAV bytes (16-bit PCM integer or 32-bit float) into normalized `Vec<f32>` in `[-1.0, 1.0]`.
/// Downmixes multi-channel audio to mono and resamples non-16kHz audio to 16kHz.
pub fn decode_wav_to_f32(wav_bytes: &[u8]) -> Result<Vec<f32>, AiError> {
    if wav_bytes.is_empty() {
        return Ok(Vec::new());
    }

    let cursor = std::io::Cursor::new(wav_bytes);
    let mut reader = hound::WavReader::new(cursor)
        .map_err(|e| AiError::AudioEncodingError(format!("Invalid WAV header: {}", e)))?;
    let spec = reader.spec();

    let raw_samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => match spec.bits_per_sample {
            16 => {
                let mut samples = Vec::new();
                for s in reader.samples::<i16>() {
                    let s = s.map_err(|e| AiError::AudioEncodingError(e.to_string()))?;
                    samples.push(s as f32 / 32768.0);
                }
                samples
            }
            24 => {
                let mut samples = Vec::new();
                for s in reader.samples::<i32>() {
                    let s = s.map_err(|e| AiError::AudioEncodingError(e.to_string()))?;
                    samples.push((s as f32 / 8388608.0).clamp(-1.0, 1.0));
                }
                samples
            }
            32 => {
                let mut samples = Vec::new();
                for s in reader.samples::<i32>() {
                    let s = s.map_err(|e| AiError::AudioEncodingError(e.to_string()))?;
                    samples.push(s as f32 / 2147483648.0);
                }
                samples
            }
            8 => {
                let mut samples = Vec::new();
                for s in reader.samples::<i8>() {
                    let s = s.map_err(|e| AiError::AudioEncodingError(e.to_string()))?;
                    let u = s as u8;
                    samples.push((u as f32 - 128.0) / 128.0);
                }
                samples
            }
            bits => {
                return Err(AiError::AudioEncodingError(format!(
                    "Unsupported PCM integer bit depth: {}",
                    bits
                )));
            }
        },
        hound::SampleFormat::Float => {
            let mut samples = Vec::new();
            for s in reader.samples::<f32>() {
                let s = s.map_err(|e| AiError::AudioEncodingError(e.to_string()))?;
                samples.push(s.clamp(-1.0, 1.0));
            }
            samples
        }
    };

    // If multi-channel, downmix to mono by averaging channels
    let channels = spec.channels as usize;
    let mono_samples = if channels > 1 {
        let num_frames = raw_samples.len() / channels;
        let mut mono = Vec::with_capacity(num_frames);
        for frame in 0..num_frames {
            let mut sum = 0.0f32;
            for ch in 0..channels {
                sum += raw_samples[frame * channels + ch];
            }
            mono.push(sum / channels as f32);
        }
        mono
    } else {
        raw_samples
    };

    // If sample rate is not 16000 Hz, resample to 16 kHz
    if spec.sample_rate != 16000 {
        crate::audio::recorder::resample_to_16k(&mono_samples, spec.sample_rate)
            .map_err(|e| AiError::AudioEncodingError(format!("Audio resampling failed: {}", e)))
    } else {
        Ok(mono_samples)
    }
}

/// Asynchronously streams and downloads a local model from the catalog to `models_dir()`.
///
/// Writes to `{filename}.part` during download, reporting `(downloaded_bytes, total_bytes)`
/// through `progress_cb`. Atomically renames to `{filename}` upon completion.
pub async fn download_local_model(
    model_id: &str,
    progress_cb: impl Fn(u64, u64) + Send + 'static,
) -> Result<PathBuf, AiError> {
    let catalog = get_local_model_catalog();
    let info = catalog
        .iter()
        .find(|m| m.id == model_id)
        .ok_or_else(|| AiError::ApiError(format!("Model '{}' not found in catalog", model_id)))?;

    let dir = models_dir();
    let target_path = dir.join(&info.filename);
    let part_path = dir.join(format!("{}.part", info.filename));

    let client = reqwest::Client::builder()
        .user_agent("OpenDictate/2.0.0 (Linux; x86_64)")
        .build()
        .unwrap_or_default();

    let mut res = client
        .get(&info.url)
        .send()
        .await
        .map_err(AiError::NetworkError)?;

    if !res.status().is_success() {
        return Err(AiError::ApiError(format!(
            "Failed to download model from {}: HTTP {}",
            info.url,
            res.status()
        )));
    }

    let total_bytes = res.content_length().unwrap_or(0);
    let mut downloaded_bytes = 0u64;

    let mut file = tokio::fs::File::create(&part_path)
        .await
        .map_err(|e| AiError::ApiError(format!("Failed to create partial model file: {}", e)))?;

    while let Some(chunk) = res.chunk().await.map_err(AiError::NetworkError)? {
        if let Err(e) = file.write_all(&chunk).await {
            let _ = tokio::fs::remove_file(&part_path).await;
            return Err(AiError::ApiError(format!(
                "Failed writing model data: {}",
                e
            )));
        }
        downloaded_bytes += chunk.len() as u64;
        progress_cb(downloaded_bytes, total_bytes);
    }

    file.flush()
        .await
        .map_err(|e| AiError::ApiError(format!("Failed to flush model file: {}", e)))?;
    drop(file);

    tokio::fs::rename(&part_path, &target_path)
        .await
        .map_err(|e| AiError::ApiError(format!("Failed to finalize model file: {}", e)))?;

    Ok(target_path)
}

/// Filters out Whisper non-speech markers (`[BLANK_AUDIO]`, `(blank audio)`, `[ Silence ]`,
/// `[ Music ]`, `[INAUDIBLE]`, etc.) so ambient silence never prints fake tokens into the UI.
pub fn clean_whisper_segment(seg: &str) -> String {
    let trimmed = seg.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('*') && trimmed.ends_with('*'))
    {
        return String::new();
    }
    let mut cleaned = trimmed.to_string();
    for marker in [
        "[BLANK_AUDIO]",
        "[blank_audio]",
        "(blank audio)",
        "(BLANK AUDIO)",
        "[Silence]",
        "[silence]",
        "[ Silence ]",
        "[ silence ]",
        "(silence)",
        "(Silence)",
        "[Music]",
        "[music]",
        "[ Music ]",
        "[ music ]",
        "(music)",
        "[INAUDIBLE]",
        "[inaudible]",
        "(inaudible)",
        "[NOISE]",
        "[noise]",
        "(noise)",
        "[APPLAUSE]",
        "[applause]",
        "(applause)",
    ] {
        if cleaned.contains(marker) {
            cleaned = cleaned.replace(marker, " ");
        }
    }
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Local AI Provider implementing `AiProvider` via embedded speech engine.
pub struct LocalAiProvider {
    pub model_id: String,
    pub custom_path: Option<String>,
    pub threads: u32,
}

impl LocalAiProvider {
    pub fn new(model: Option<String>) -> Self {
        Self {
            model_id: model
                .filter(|m| !m.trim().is_empty())
                .unwrap_or_else(|| "tiny.en".to_string()),
            custom_path: None,
            threads: 4,
        }
    }

    pub fn with_config(model_id: String, custom_path: Option<String>, threads: u32) -> Self {
        Self {
            model_id: if model_id.trim().is_empty() {
                "tiny.en".to_string()
            } else {
                model_id
            },
            custom_path,
            threads,
        }
    }

    pub fn model(&self) -> &str {
        &self.model_id
    }

    /// Performs in-process speech recognition on float32 16kHz mono audio samples with optional language and prompt.
    pub fn transcribe_wav_samples_with_options(
        samples: &[f32],
        model_path: &Path,
        threads: u32,
        language: Option<&str>,
        initial_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        if samples.is_empty() {
            return Ok(String::new());
        }

        let path_str = model_path
            .to_str()
            .ok_or_else(|| AiError::ApiError(format!("Invalid model path: {:?}", model_path)))?;

        type CachedWhisperCtx = (
            String,
            std::sync::Arc<local_speech_engine::WhisperContext>,
        );
        static CACHED_CTX: std::sync::OnceLock<std::sync::Mutex<Option<CachedWhisperCtx>>> =
            std::sync::OnceLock::new();

        let ctx = {
            let cache_mutex = CACHED_CTX.get_or_init(|| std::sync::Mutex::new(None));
            let mut guard = cache_mutex
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some((ref cached_path, ref cached_arc)) = *guard
                && cached_path == path_str
            {
                std::sync::Arc::clone(cached_arc)
            } else {
                let loaded = std::sync::Arc::new(
                    local_speech_engine::WhisperContext::new_with_params(
                        path_str,
                        Default::default(),
                    )
                    .map_err(|e| {
                        AiError::ApiError(format!("Local speech engine context error: {}", e))
                    })?,
                );
                *guard = Some((path_str.to_string(), std::sync::Arc::clone(&loaded)));
                loaded
            }
        };

        let mut state = ctx
            .create_state()
            .map_err(|e| AiError::ApiError(format!("Local speech engine state error: {}", e)))?;

        let mut params =
            local_speech_engine::FullParams::new(local_speech_engine::SamplingStrategy::Greedy {
                best_of: 1,
            });
        let effective_threads = if threads == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get() as i32)
                .unwrap_or(4)
                .clamp(1, 8)
        } else {
            threads.max(1) as i32
        };
        params.set_n_threads(effective_threads);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let filename = model_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let is_english_model = filename.contains(".en.") || filename.ends_with(".en.bin");

        if let Some(lang) = language {
            let trimmed = lang.trim();
            if !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("auto") {
                params.set_language(Some(trimmed));
            } else if is_english_model {
                params.set_language(Some("en"));
            } else {
                params.set_language(None);
            }
        } else if is_english_model {
            params.set_language(Some("en"));
        } else {
            params.set_language(None);
        }

        if let Some(prompt) = initial_prompt {
            let trimmed = prompt.trim();
            if !trimmed.is_empty() {
                params.set_initial_prompt(trimmed);
            }
        }

        state.full(params, samples).map_err(|e| {
            AiError::ApiError(format!("Local speech engine inference error: {}", e))
        })?;

        let mut result = String::new();
        for segment in state.as_iter() {
            let seg_text = if let Ok(text) = segment.to_str() {
                text.to_string()
            } else {
                segment.to_string()
            };
            let cleaned = clean_whisper_segment(&seg_text);
            if !cleaned.is_empty() {
                if !result.is_empty() && !result.ends_with(' ') && !cleaned.starts_with(' ') {
                    result.push(' ');
                }
                result.push_str(&cleaned);
            }
        }

        Ok(result.trim().to_string())
    }

    /// Performs in-process speech recognition on float32 16kHz mono audio samples.
    pub fn transcribe_wav_samples(
        samples: &[f32],
        model_path: &Path,
        threads: u32,
    ) -> Result<String, AiError> {
        Self::transcribe_wav_samples_with_options(samples, model_path, threads, None, None)
    }
}

impl Default for LocalAiProvider {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl AiProvider for LocalAiProvider {
    async fn transcribe(
        &self,
        audio_wav: Vec<u8>,
        system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        if audio_wav.is_empty() {
            return Ok(String::new());
        }

        let model_path = resolve_model_path(&self.model_id, self.custom_path.as_deref());
        if !is_model_installed(&self.model_id, self.custom_path.as_deref()) {
            return Err(AiError::ApiError(format!(
                "Local model '{}' not installed. Please download it in Settings.",
                self.model_id
            )));
        }

        let samples = decode_wav_to_f32(&audio_wav)?;
        if samples.is_empty() {
            return Ok(String::new());
        }

        let threads = self.threads;
        let prompt_owned = system_prompt.map(|s| s.to_string());
        tokio::task::spawn_blocking(move || {
            let (lang, prompt) = if let Some(ref p) = prompt_owned {
                let trimmed = p.trim();
                if (trimmed.len() == 2 || trimmed.len() == 3)
                    && trimmed.chars().all(|c| c.is_ascii_alphabetic())
                {
                    (Some(trimmed), None)
                } else {
                    (None, Some(trimmed))
                }
            } else {
                (None, None)
            };
            Self::transcribe_wav_samples_with_options(&samples, &model_path, threads, lang, prompt)
        })
        .await
        .map_err(|e| AiError::ApiError(format!("Local speech engine thread join error: {}", e)))?
    }

    async fn enhance(&self, text: &str, tone: &str) -> Result<String, AiError> {
        Ok(apply_smart_local_formatting(text, tone))
    }

    async fn list_models(&self) -> Result<Vec<String>, AiError> {
        Ok(get_local_model_catalog()
            .into_iter()
            .map(|m| m.id)
            .collect())
    }
}
