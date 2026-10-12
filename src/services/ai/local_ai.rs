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
        .user_agent(concat!("OpenDictate/", env!("CARGO_PKG_VERSION"), " (Linux; x86_64)"))
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

/// Collapses pathological Whisper decoder loops where the exact same word is repeated
/// 4 or more times consecutively (e.g. "the the the the the" -> "the"), while preserving
/// natural 2x and 3x spoken emphasis ("very very", "no no no").
pub fn collapse_excessive_repetitions(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() < 4 {
        return words.join(" ");
    }

    let mut out: Vec<&str> = Vec::with_capacity(words.len());
    let mut i = 0;
    while i < words.len() {
        let w = words[i];
        let norm = w.trim_matches(|c: char| !c.is_alphanumeric());
        let mut run_len = 1;
        while i + run_len < words.len() {
            let next_norm = words[i + run_len].trim_matches(|c: char| !c.is_alphanumeric());
            if !norm.is_empty() && norm.eq_ignore_ascii_case(next_norm) {
                run_len += 1;
            } else {
                break;
            }
        }
        if run_len >= 4 {
            out.push(words[i + run_len - 1]);
            i += run_len;
        } else {
            for item in &words[i..i + run_len] {
                out.push(item);
            }
            i += run_len;
        }
    }
    out.join(" ")
}

/// Filters out Whisper non-speech markers (`[BLANK_AUDIO]`, `(blank audio)`, `[ Silence ]`,
/// `[ Music ]`, `[INAUDIBLE]`, etc.) so ambient silence never prints fake tokens into the UI,
/// and collapses pathological >= 4x decoder repetition loops without deleting legitimate speech.
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
    if !cleaned.chars().any(|c| c.is_alphanumeric()) {
        return String::new();
    }
    collapse_excessive_repetitions(&cleaned)
}

/// Compile-time or runtime x86_64 SIMD features required/supported by `ggml-cpu`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompiledX86SimdFeatures {
    pub sse3: bool,
    pub ssse3: bool,
    pub sse41: bool,
    pub sse42: bool,
    pub avx: bool,
    pub avx_vnni: bool,
    pub avx2: bool,
    pub fma: bool,
    pub f16c: bool,
    pub bmi2: bool,
    pub avx512: bool,
    pub avx512_vbmi: bool,
    pub avx512_vnni: bool,
    pub avx512_bf16: bool,
}

impl CompiledX86SimdFeatures {
    /// Returns the baseline portable `x86-64-v1` (`SSE2`) feature profile where no optional
    /// x86_64 extensions beyond ABI-guaranteed `SSE2` are required.
    pub const fn portable_x86_64_v1() -> Self {
        Self {
            sse3: false,
            ssse3: false,
            sse41: false,
            sse42: false,
            avx: false,
            avx_vnni: false,
            avx2: false,
            fma: false,
            f16c: false,
            bmi2: false,
            avx512: false,
            avx512_vbmi: false,
            avx512_vnni: false,
            avx512_bf16: false,
        }
    }

    /// Returns the required feature set for the optimized `opendictate-avx2` binary tier
    /// (`SSE3`, `SSSE3`, `SSE4.1`, `SSE4.2`, `AVX`, `AVX2`, `FMA`, `F16C`, and `BMI2`).
    pub const fn avx2_optimized_tier() -> Self {
        Self {
            sse3: true,
            ssse3: true,
            sse41: true,
            sse42: true,
            avx: true,
            avx_vnni: false,
            avx2: true,
            fma: true,
            f16c: true,
            bmi2: true,
            avx512: false,
            avx512_vbmi: false,
            avx512_vnni: false,
            avx512_bf16: false,
        }
    }
}

/// Queries the compile-time SIMD flags baked into `libggml-cpu.a` via the lightweight
/// `ggml_cpu_has_*()` C helpers (which return `#if defined(__AVX2__) 1 #else 0` without
/// executing any SIMD instructions) plus compile-time `GGML_SSE42` / `AVX` requirements.
pub fn compiled_x86_simd_features() -> CompiledX86SimdFeatures {
    unsafe extern "C" {
        fn ggml_cpu_has_sse3() -> std::os::raw::c_int;
        fn ggml_cpu_has_ssse3() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx_vnni() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx2() -> std::os::raw::c_int;
        fn ggml_cpu_has_fma() -> std::os::raw::c_int;
        fn ggml_cpu_has_f16c() -> std::os::raw::c_int;
        fn ggml_cpu_has_bmi2() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx512() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx512_vbmi() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx512_vnni() -> std::os::raw::c_int;
        fn ggml_cpu_has_avx512_bf16() -> std::os::raw::c_int;
    }
    let sse42_env = matches!(
        option_env!("GGML_SSE42"),
        Some("ON" | "on" | "1" | "TRUE" | "true")
    );
    unsafe {
        let avx2 = ggml_cpu_has_avx2() != 0;
        let avx = (ggml_cpu_has_avx() != 0) || avx2;
        // Architectural dependency chain in x86-64 and GCC/Clang (`-mavx` -> `-msse4.2` -> `-msse4.1`):
        let sse42 = avx || sse42_env || cfg!(target_feature = "sse4.2");
        let sse41 = sse42 || cfg!(target_feature = "sse4.1");
        CompiledX86SimdFeatures {
            sse3: ggml_cpu_has_sse3() != 0,
            ssse3: ggml_cpu_has_ssse3() != 0,
            sse41,
            sse42,
            avx,
            avx_vnni: ggml_cpu_has_avx_vnni() != 0,
            avx2,
            fma: ggml_cpu_has_fma() != 0,
            f16c: ggml_cpu_has_f16c() != 0,
            bmi2: ggml_cpu_has_bmi2() != 0,
            avx512: ggml_cpu_has_avx512() != 0,
            avx512_vbmi: ggml_cpu_has_avx512_vbmi() != 0,
            avx512_vnni: ggml_cpu_has_avx512_vnni() != 0,
            avx512_bf16: ggml_cpu_has_avx512_bf16() != 0,
        }
    }
}

/// Detects the x86_64 SIMD features supported at runtime by both the host CPU (`CPUID`)
/// and the operating system (`XGETBV` OSXSAVE state for YMM/ZMM registers).
pub fn detected_x86_simd_features() -> CompiledX86SimdFeatures {
    #[cfg(target_arch = "x86_64")]
    {
        let has_avx512_base = std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512cd")
            && std::arch::is_x86_feature_detected!("avx512vl")
            && std::arch::is_x86_feature_detected!("avx512dq")
            && std::arch::is_x86_feature_detected!("avx512bw");
        CompiledX86SimdFeatures {
            sse3: std::arch::is_x86_feature_detected!("sse3"),
            ssse3: std::arch::is_x86_feature_detected!("ssse3"),
            sse41: std::arch::is_x86_feature_detected!("sse4.1"),
            sse42: std::arch::is_x86_feature_detected!("sse4.2"),
            avx: std::arch::is_x86_feature_detected!("avx"),
            avx_vnni: std::arch::is_x86_feature_detected!("avxvnni"),
            avx2: std::arch::is_x86_feature_detected!("avx2"),
            fma: std::arch::is_x86_feature_detected!("fma"),
            f16c: std::arch::is_x86_feature_detected!("f16c"),
            bmi2: std::arch::is_x86_feature_detected!("bmi2"),
            avx512: has_avx512_base,
            avx512_vbmi: std::arch::is_x86_feature_detected!("avx512vbmi"),
            avx512_vnni: std::arch::is_x86_feature_detected!("avx512vnni"),
            avx512_bf16: std::arch::is_x86_feature_detected!("avx512bf16"),
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        CompiledX86SimdFeatures::portable_x86_64_v1()
    }
}

/// Compares the SIMD instruction sets required by the compiled `ggml-cpu` library against
/// the host CPU's runtime capabilities, returning a descriptive `AiError::ApiError` instead
/// of allowing an unrecoverable `SIGILL` trap if any required feature is missing.
pub fn check_x86_simd_requirements(
    required: CompiledX86SimdFeatures,
    detected: CompiledX86SimdFeatures,
) -> Result<(), AiError> {
    let mut missing = Vec::new();
    if required.sse3 && !detected.sse3 {
        missing.push("SSE3");
    }
    if required.ssse3 && !detected.ssse3 {
        missing.push("SSSE3");
    }
    if required.sse41 && !detected.sse41 {
        missing.push("SSE4.1");
    }
    if required.sse42 && !detected.sse42 {
        missing.push("SSE4.2");
    }
    if required.avx && !detected.avx {
        missing.push("AVX");
    }
    if required.avx_vnni && !detected.avx_vnni {
        missing.push("AVX_VNNI");
    }
    if required.avx2 && !detected.avx2 {
        missing.push("AVX2");
    }
    if required.fma && !detected.fma {
        missing.push("FMA");
    }
    if required.f16c && !detected.f16c {
        missing.push("F16C");
    }
    if required.bmi2 && !detected.bmi2 {
        missing.push("BMI2");
    }
    if required.avx512 && !detected.avx512 {
        missing.push("AVX512F");
    }
    if required.avx512_vbmi && !detected.avx512_vbmi {
        missing.push("AVX512_VBMI");
    }
    if required.avx512_vnni && !detected.avx512_vnni {
        missing.push("AVX512_VNNI");
    }
    if required.avx512_bf16 && !detected.avx512_bf16 {
        missing.push("AVX512_BF16");
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(AiError::ApiError(format!(
            "Local Whisper engine requires CPU instruction set(s) [{}] not supported by this processor. Switch to a Cloud AI provider or rebuild with GGML_AVX2=OFF.",
            missing.join(", ")
        )))
    }
}

/// Verifies at runtime that the host CPU supports all SIMD instruction sets compiled into
/// `libggml-cpu.a` before invoking `WhisperContext::new_with_params` (preventing `SIGILL`
/// on pre-2013 non-AVX2 x86_64 CPUs or restricted VMs).
pub fn verify_cpu_simd_compatibility() -> Result<(), AiError> {
    #[cfg(target_arch = "x86_64")]
    {
        let required = compiled_x86_simd_features();
        let detected = detected_x86_simd_features();
        check_x86_simd_requirements(required, detected)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        Ok(())
    }
}

/// Pure decision helper determining whether the portable launcher binary should dispatch
/// via `execv` to its sibling `opendictate-avx2` binary.
pub fn should_dispatch_to_avx2_binary(
    compiled_self: CompiledX86SimdFeatures,
    detected_cpu: CompiledX86SimdFeatures,
    already_dispatched: bool,
    disable_dispatch: bool,
    candidate_exists_and_executable: bool,
) -> bool {
    if compiled_self.avx2 || already_dispatched || disable_dispatch || !candidate_exists_and_executable {
        return false;
    }
    check_x86_simd_requirements(CompiledX86SimdFeatures::avx2_optimized_tier(), detected_cpu)
        .is_ok()
}

/// Resolves the expected sibling path for `opendictate-avx2` next to `current_exe`,
/// returning `None` if `current_exe` is already named `opendictate-avx2`.
pub fn resolve_avx2_sibling_path(current_exe: &Path) -> Option<PathBuf> {
    let file_name = current_exe.file_name()?.to_str()?;
    if file_name == "opendictate-avx2" {
        return None;
    }
    Some(current_exe.parent()?.join("opendictate-avx2"))
}

/// Returns `true` if `path` exists as a regular file and has at least one execute permission bit set.
pub fn is_executable_file(path: &Path) -> bool {
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };
    if !meta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        (meta.permissions().mode() & 0o111) != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

/// If running inside the portable (`x86-64-v1` / `SSE2`) launcher binary on an x86_64 CPU
/// that supports the full `opendictate-avx2` instruction tier (`SSE4.1`, `SSE4.2`, `AVX`,
/// `AVX2`, `FMA`, `F16C`, `BMI2`), replaces the current process image in-place with the
/// sibling `opendictate-avx2` executable while preserving all command-line arguments and environment.
///
/// Falls back cleanly by returning to the caller if the CPU lacks any required instruction,
/// `OPENDICTATE_DISABLE_AVX2_DISPATCH=1` is set, `opendictate-avx2` is missing/non-executable,
/// or `execv` fails.
pub fn try_exec_avx2_sibling() {
    #[cfg(all(target_arch = "x86_64", unix))]
    {
        let already_dispatched = matches!(
            std::env::var("OPENDICTATE_DISPATCHED_AVX2").as_deref(),
            Ok("1" | "true" | "TRUE")
        );
        let disable_dispatch = matches!(
            std::env::var("OPENDICTATE_DISABLE_AVX2_DISPATCH").as_deref(),
            Ok("1" | "true" | "TRUE")
        );
        let current_exe = std::env::current_exe().ok();
        let candidate = current_exe
            .as_deref()
            .and_then(resolve_avx2_sibling_path);
        let candidate_executable = candidate.as_deref().is_some_and(is_executable_file);
        let compiled_self = compiled_x86_simd_features();
        let detected_cpu = detected_x86_simd_features();

        if should_dispatch_to_avx2_binary(
            compiled_self,
            detected_cpu,
            already_dispatched,
            disable_dispatch,
            candidate_executable,
        ) && let Some(avx2_path) = candidate
        {
            use std::os::unix::process::CommandExt;
            let err = std::process::Command::new(&avx2_path)
                .args(std::env::args_os().skip(1))
                .env("OPENDICTATE_DISPATCHED_AVX2", "1")
                .exec();
            eprintln!(
                "OpenDictate notice: failed to dispatch to {:?} ({}); continuing with portable baseline binary.",
                avx2_path, err
            );
        }
    }
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
            threads: 0,
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
        verify_cpu_simd_compatibility()?;

        let path_str = model_path
            .to_str()
            .ok_or_else(|| AiError::ApiError(format!("Invalid model path: {:?}", model_path)))?;

        struct CachedWhisperEngine {
            path: String,
            _ctx: std::sync::Arc<local_speech_engine::WhisperContext>,
            state: std::sync::Arc<std::sync::Mutex<local_speech_engine::WhisperState>>,
        }
        static CACHED_ENGINE: std::sync::OnceLock<std::sync::Mutex<Option<CachedWhisperEngine>>> =
            std::sync::OnceLock::new();

        let state_arc = {
            let cache_mutex = CACHED_ENGINE.get_or_init(|| std::sync::Mutex::new(None));
            let mut guard = cache_mutex
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(ref cached) = *guard
                && cached.path == path_str
            {
                std::sync::Arc::clone(&cached.state)
            } else {
                let mut ctx_params = local_speech_engine::WhisperContextParameters::default();
                if let Ok(use_gpu_str) = std::env::var("OPENDICTATE_USE_GPU") {
                    ctx_params.use_gpu(use_gpu_str != "0" && !use_gpu_str.eq_ignore_ascii_case("false"));
                }
                if let Ok(dev_str) = std::env::var("OPENDICTATE_GPU_DEVICE")
                    && let Ok(dev_id) = dev_str.parse::<i32>()
                {
                    ctx_params.gpu_device(dev_id);
                }
                let loaded_ctx = std::sync::Arc::new(
                    local_speech_engine::WhisperContext::new_with_params(path_str, ctx_params)
                        .map_err(|e| {
                            AiError::ApiError(format!("Local speech engine context error: {}", e))
                        })?,
                );
                let loaded_state = std::sync::Arc::new(std::sync::Mutex::new(
                    loaded_ctx.create_state().map_err(|e| {
                        AiError::ApiError(format!("Local speech engine state error: {}", e))
                    })?,
                ));
                *guard = Some(CachedWhisperEngine {
                    path: path_str.to_string(),
                    _ctx: loaded_ctx,
                    state: std::sync::Arc::clone(&loaded_state),
                });
                loaded_state
            }
        };

        let mut state = state_arc
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut params =
            local_speech_engine::FullParams::new(local_speech_engine::SamplingStrategy::Greedy {
                best_of: 1,
            });
        let effective_threads = if threads == 0 {
            std::thread::available_parallelism()
                .map(|n| {
                    let logical = n.get();
                    if logical >= 12 {
                        6
                    } else if logical >= 8 {
                        (logical / 2).clamp(4, 6)
                    } else {
                        logical.clamp(1, 6)
                    }
                })
                .unwrap_or(4) as i32
        } else {
            threads.max(1) as i32
        };
        params.set_n_threads(effective_threads);
        params.set_no_context(true);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_suppress_nst(true);
        params.set_no_speech_thold(0.6);
        params.set_entropy_thold(2.4);
        params.set_logprob_thold(-1.0);
        let temp_inc = std::env::var("OPENDICTATE_TEMP_INC")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);
        params.set_temperature_inc(temp_inc);

        let no_timestamps = match std::env::var("OPENDICTATE_NO_TIMESTAMPS").as_deref() {
            Ok("1") => true,
            Ok("0") => false,
            _ => samples.len() > 80_000 && samples.len() <= 480_000,
        };
        if no_timestamps {
            params.set_no_timestamps(true);
        }

        let use_dynamic_audio_ctx = std::env::var("OPENDICTATE_AUDIO_CTX")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("full"))
            .unwrap_or(true);
        if use_dynamic_audio_ctx && samples.len() < 400_000 {
            let raw_ctx = ((samples.len() as f32 / 480_000.0) * 1500.0).ceil() as i32 + 128;
            let aligned_ctx = ((raw_ctx + 63) / 64 * 64).clamp(512, 1500);
            params.set_audio_ctx(aligned_ctx);
        }

        let filename = model_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let is_english_model = filename.contains(".en.")
            || filename.contains(".en-")
            || filename.ends_with(".en.bin");

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

    /// Asynchronously transcribes pre-normalized 16kHz `f32` audio samples directly
    /// without WAV encoding/decoding overhead.
    pub async fn transcribe_samples(
        &self,
        samples: Vec<f32>,
        system_prompt: Option<&str>,
    ) -> Result<String, AiError> {
        if samples.is_empty() {
            return Ok(String::new());
        }

        let model_path = resolve_model_path(&self.model_id, self.custom_path.as_deref());
        if !is_model_installed(&self.model_id, self.custom_path.as_deref()) {
            return Err(AiError::ApiError(format!(
                "Local model '{}' not installed. Please download it in Settings.",
                self.model_id
            )));
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

        self.transcribe_samples(samples, system_prompt).await
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
