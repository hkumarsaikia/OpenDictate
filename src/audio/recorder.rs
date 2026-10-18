//! Low-latency microphone audio capture with cpal, rubato resampling,
//! and in-memory 16-bit linear PCM WAV encoding with hound.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Error conditions that can occur during audio recording, resampling, or encoding.
#[derive(Debug)]
pub enum AudioError {
    DeviceNotFound(String),
    DefaultDeviceNotFound,
    DeviceNotAvailable(String),
    StreamBuildError(String),
    StreamPlayError(String),
    WavEncodeError(String),
    ResampleError(String),
    Other(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceNotFound(name) => write!(f, "Audio input device not found: {}", name),
            Self::DefaultDeviceNotFound => write!(f, "No default audio input device found"),
            Self::DeviceNotAvailable(msg) => write!(f, "Audio device not available: {}", msg),
            Self::StreamBuildError(msg) => write!(f, "Failed to build audio stream: {}", msg),
            Self::StreamPlayError(msg) => write!(f, "Failed to start audio stream: {}", msg),
            Self::WavEncodeError(msg) => write!(f, "Failed to encode WAV audio: {}", msg),
            Self::ResampleError(msg) => write!(f, "Failed to resample audio: {}", msg),
            Self::Other(msg) => write!(f, "Audio error: {}", msg),
        }
    }
}

impl std::error::Error for AudioError {}

impl From<hound::Error> for AudioError {
    fn from(err: hound::Error) -> Self {
        AudioError::WavEncodeError(err.to_string())
    }
}

impl From<cpal::DevicesError> for AudioError {
    fn from(err: cpal::DevicesError) -> Self {
        AudioError::DeviceNotAvailable(err.to_string())
    }
}

impl From<cpal::DefaultStreamConfigError> for AudioError {
    fn from(err: cpal::DefaultStreamConfigError) -> Self {
        AudioError::StreamBuildError(err.to_string())
    }
}

impl From<cpal::BuildStreamError> for AudioError {
    fn from(err: cpal::BuildStreamError) -> Self {
        AudioError::StreamBuildError(err.to_string())
    }
}

impl From<cpal::PlayStreamError> for AudioError {
    fn from(err: cpal::PlayStreamError) -> Self {
        AudioError::StreamPlayError(err.to_string())
    }
}

/// Encode raw mono `f32` samples into standard 16-bit linear PCM WAV bytes in memory.
pub fn encode_pcm_wav(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, AudioError> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
        for &sample in samples {
            let scaled = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(scaled)?;
        }
        writer.finalize()?;
    }
    Ok(cursor.into_inner())
}

/// Resample arbitrary mono `f32` samples to 16 kHz using rubato with linear interpolation fallback.
pub fn resample_to_16k(samples: &[f32], orig_rate: u32) -> Result<Vec<f32>, AudioError> {
    if orig_rate == 0 {
        return Err(AudioError::ResampleError(
            "Invalid origin sample rate (0)".into(),
        ));
    }
    if orig_rate == 16000 || samples.is_empty() {
        return Ok(samples.to_vec());
    }

    if samples.len() < 1024 {
        return Ok(linear_resample(samples, orig_rate, 16000));
    }

    use rubato::{FastFixedIn, PolynomialDegree, Resampler};
    let chunk_size = 1024;
    let ratio = 16000.0 / orig_rate as f64;
    let mut resampler =
        match FastFixedIn::<f32>::new(ratio, 1.1, PolynomialDegree::Linear, chunk_size, 1) {
            Ok(r) => r,
            Err(e) => {
                log::warn!(
                    "Rubato FastFixedIn init failed ({}), falling back to linear resampler",
                    e
                );
                return Ok(linear_resample(samples, orig_rate, 16000));
            }
        };

    let mut output = Vec::with_capacity((samples.len() as f64 * ratio) as usize + 1024);
    let mut offset = 0;
    while offset + chunk_size <= samples.len() {
        let chunk = &samples[offset..offset + chunk_size];
        let in_buf = [chunk];
        let out_buf = resampler
            .process(&in_buf, None)
            .map_err(|e| AudioError::ResampleError(e.to_string()))?;
        output.extend_from_slice(&out_buf[0]);
        offset += chunk_size;
    }

    if offset < samples.len() {
        let remainder = &samples[offset..];
        let rem_out = linear_resample(remainder, orig_rate, 16000);
        output.extend_from_slice(&rem_out);
    }

    Ok(output)
}

fn linear_resample(samples: &[f32], orig_rate: u32, target_rate: u32) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }
    let duration = samples.len() as f64 / orig_rate as f64;
    let target_len = (duration * target_rate as f64).round() as usize;
    let mut out = Vec::with_capacity(target_len);
    let ratio = orig_rate as f64 / target_rate as f64;
    for i in 0..target_len {
        let orig_pos = i as f64 * ratio;
        let idx = orig_pos.floor() as usize;
        let frac = (orig_pos - idx as f64) as f32;
        let s0 = samples.get(idx).copied().unwrap_or(0.0);
        let s1 = samples.get(idx + 1).copied().unwrap_or(s0);
        out.push(s0 + frac * (s1 - s0));
    }
    out
}

/// Removes residual DC offset and normalizes speech dynamic range into `[-0.80, 0.80]`
/// when voice energy is present (`peak >= 0.004`), capping boost at `12.0x` so quiet
/// hardware microphones (Bluetooth HFP, distant laptop/USB mics) transcribe cleanly.
pub fn normalize_speech_samples(samples: &[f32]) -> Vec<f32> {
    if samples.is_empty() {
        return Vec::new();
    }
    // Skip initial 80ms (1280 samples at 16kHz) when measuring DC mean, peak, and RMS
    // on recordings longer than 150ms (2400 samples) so hardware ADC wake-up pops
    // never skew DC centering or prevent speech dynamic range normalization.
    let measure_slice = if samples.len() > 2400 {
        &samples[1280..]
    } else {
        samples
    };
    let mean = measure_slice.iter().copied().sum::<f32>() / measure_slice.len() as f32;
    let mut peak = 0.0f32;
    let mut sum_sq = 0.0f32;
    for &s in measure_slice {
        let c = (s - mean).clamp(-1.0, 1.0);
        let a = c.abs();
        if a > peak {
            peak = a;
        }
        sum_sq += c * c;
    }
    let rms = (sum_sq / measure_slice.len() as f32).sqrt();
    let gain = if peak >= 0.004 && rms >= 0.0015 && peak < 0.75 {
        (0.80 / peak).min(12.0)
    } else {
        1.0
    };

    let fade_len = if samples.len() > 2400 {
        640usize
    } else {
        0usize
    };
    let mut centered = Vec::with_capacity(samples.len());
    for (i, &s) in samples.iter().enumerate() {
        let fade = if i < fade_len {
            i as f32 / fade_len as f32
        } else {
            1.0
        };
        let c = ((s - mean) * fade * gain).clamp(-1.0, 1.0);
        centered.push(c);
    }
    centered
}

fn frame_has_voice_energy(frame: &[f32]) -> bool {
    if frame.is_empty() {
        return false;
    }
    let mean = frame.iter().copied().sum::<f32>() / frame.len() as f32;
    let mut sum_sq = 0.0f32;
    let mut peak = 0.0f32;
    for &x in frame {
        let d = x - mean;
        let a = d.abs();
        if a > peak {
            peak = a;
        }
        sum_sq += d * d;
    }
    let rms = (sum_sq / frame.len() as f32).sqrt();
    rms >= 0.0008 && peak >= 0.002
}

/// Returns true if any 20ms short-time window (with 10ms hop) at `sample_rate` contains
/// above-threshold voice energy (`rms >= 0.0008` and `peak >= 0.002`, matching
/// `normalize_speech_samples`), preventing surrounding silence from diluting short speech
/// onsets or brief words spoken after long pauses.
pub fn has_voice_activity_at_rate(samples: &[f32], sample_rate: u32) -> bool {
    if samples.is_empty() {
        return false;
    }
    let frame_len = ((sample_rate.max(8_000) as usize) * 20 / 1_000).max(160);
    if samples.len() <= frame_len {
        return frame_has_voice_energy(samples);
    }
    let step = (frame_len / 2).max(1);
    let mut offset = 0usize;
    while offset + frame_len <= samples.len() {
        if frame_has_voice_energy(&samples[offset..offset + frame_len]) {
            return true;
        }
        offset += step;
    }
    if offset < samples.len() {
        let tail_start = samples.len() - frame_len;
        if frame_has_voice_energy(&samples[tail_start..]) {
            return true;
        }
    }
    false
}

/// Returns true if any 20ms short-time window (at 16 kHz default reference rate) contains
/// above-threshold voice energy (`rms >= 0.0008` and `peak >= 0.002`).
pub fn has_voice_activity(samples: &[f32]) -> bool {
    has_voice_activity_at_rate(samples, 16_000)
}

/// Snapshot of normalized 16kHz `f32` audio samples for zero-WAV-copy local streaming inference.
pub struct StreamingSampleSnapshot {
    pub samples_16k: Vec<f32>,
    pub total_native_samples: usize,
    pub delta_has_voice: bool,
    pub tail_is_quiet: bool,
}

/// Non-destructive live snapshot handle for streaming transcription while recording is active.
#[derive(Clone)]
pub struct AudioBufferSnapshot {
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: Arc<AtomicU32>,
    is_paused: Arc<AtomicBool>,
    stream_error: Arc<AtomicBool>,
}

impl AudioBufferSnapshot {
    /// Atomically checks and clears any hardware stream disconnection/error flag.
    pub fn take_stream_error(&self) -> bool {
        self.stream_error.swap(false, Ordering::SeqCst)
    }

    /// Captures a non-destructive 16kHz 16-bit PCM WAV snapshot of up to the most recent
    /// `max_seconds` of recorded audio without stopping or clearing the active stream.
    /// Returns `Ok(Some((wav_bytes, total_sample_count)))` when at least 0.40s of voice audio is available.
    pub fn snapshot_wav(&self, max_seconds: f32) -> Result<Option<(Vec<u8>, usize)>, AudioError> {
        if self.is_paused.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let native_rate = self.sample_rate.load(Ordering::SeqCst).max(8000);
        let min_samples = ((native_rate as f32) * 0.40) as usize;
        let max_samples = ((native_rate as f32) * max_seconds.max(2.0)) as usize;

        let (window, total_len) = {
            let Ok(buf) = self.buffer.lock() else {
                return Ok(None);
            };
            if buf.len() < min_samples {
                return Ok(None);
            };
            let total = buf.len();
            let start = total.saturating_sub(max_samples);
            (buf[start..].to_vec(), total)
        };

        if !has_voice_activity_at_rate(&window, native_rate) {
            return Ok(None);
        }

        let resampled = resample_to_16k(&window, native_rate)?;
        let normalized = normalize_speech_samples(&resampled);
        let wav = encode_pcm_wav(&normalized, 16000)?;
        Ok(Some((wav, total_len)))
    }

    /// Captures a non-destructive 16kHz `f32` snapshot for local Whisper streaming inference
    /// without WAV encoding/decoding overhead, and returns `Ok(None)` when new samples since
    /// `prev_total_samples` are silent and a post-speech finalization pass (`prev_finalized == true`)
    /// has already completed, ensuring `prev_total_samples` is never advanced past untranscribed audio.
    pub fn snapshot_samples_16k(
        &self,
        max_seconds: f32,
        prev_total_samples: usize,
        prev_finalized: bool,
    ) -> Result<Option<StreamingSampleSnapshot>, AudioError> {
        if self.is_paused.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let native_rate = self.sample_rate.load(Ordering::SeqCst).max(8000);
        let min_samples = ((native_rate as f32) * 0.40) as usize;
        let max_samples = ((native_rate as f32) * max_seconds.max(2.0)) as usize;
        let quiet_tail_len = ((native_rate as f32) * 0.20) as usize;

        let (window, total_len, delta_has_voice, tail_is_quiet) = {
            let Ok(buf) = self.buffer.lock() else {
                return Ok(None);
            };
            let total = buf.len();
            if total < min_samples || total <= prev_total_samples {
                return Ok(None);
            }
            let delta = &buf[prev_total_samples.min(total)..];
            let delta_voice = has_voice_activity_at_rate(delta, native_rate);
            if !delta_voice && prev_finalized {
                return Ok(None);
            }
            let tail_start = total.saturating_sub(quiet_tail_len);
            let tail_quiet = !has_voice_activity_at_rate(&buf[tail_start..], native_rate);
            let start = total.saturating_sub(max_samples);
            (buf[start..].to_vec(), total, delta_voice, tail_quiet)
        };

        if !has_voice_activity_at_rate(&window, native_rate) {
            return Ok(None);
        }

        let resampled = resample_to_16k(&window, native_rate)?;
        let normalized = normalize_speech_samples(&resampled);
        Ok(Some(StreamingSampleSnapshot {
            samples_16k: normalized,
            total_native_samples: total_len,
            delta_has_voice,
            tail_is_quiet,
        }))
    }
}

/// Microphone audio recorder managing hardware capture streams and real-time level metering.
pub struct AudioRecorder {
    device_name: Option<String>,
    is_recording: Arc<AtomicBool>,
    is_paused: Arc<AtomicBool>,
    stream_error: Arc<AtomicBool>,
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: Arc<AtomicU32>,
}

/// Suppresses ALSA's default stderr error logging during device probing on Linux
/// and ensures ALSA plugin/config paths and bundled `pactl` PATH are valid inside Snap/AppImage sandboxes.
#[cfg(target_os = "linux")]
pub fn silence_alsa_logging() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static SILENCED: AtomicBool = AtomicBool::new(false);
    if SILENCED.swap(true, Ordering::SeqCst) {
        return;
    }

    for env_key in ["SNAP", "APPDIR"] {
        if let Ok(root) = std::env::var(env_key) {
            let root = root.trim_end_matches('/');
            if root.is_empty() {
                continue;
            }

            // Ensure bundled usr/bin (e.g. pactl) is in PATH
            let bin_dir = format!("{}/usr/bin", root);
            if std::path::Path::new(&bin_dir).is_dir() {
                let current_path = std::env::var("PATH").unwrap_or_default();
                if !current_path.split(':').any(|p| p == bin_dir) {
                    let new_path = if current_path.is_empty() {
                        bin_dir
                    } else {
                        format!("{}:{}", bin_dir, current_path)
                    };
                    unsafe {
                        std::env::set_var("PATH", new_path);
                    }
                }
            }

            // Ensure pulseaudio private shared libraries (libpulsecommon-*.so) are in LD_LIBRARY_PATH
            // without overriding top-level libc.so.6
            let mut pulse_lib_dirs = Vec::new();
            for candidate in [
                format!("{}/usr/lib/x86_64-linux-gnu/pulseaudio", root),
                format!("{}/usr/lib/aarch64-linux-gnu/pulseaudio", root),
                format!(
                    "{}/gnome-platform/usr/lib/x86_64-linux-gnu/pulseaudio",
                    root
                ),
                format!(
                    "{}/gnome-platform/usr/lib/aarch64-linux-gnu/pulseaudio",
                    root
                ),
                "/snap/gnome-46-2404/current/usr/lib/x86_64-linux-gnu/pulseaudio".to_string(),
                "/snap/gnome-46-2404/current/usr/lib/aarch64-linux-gnu/pulseaudio".to_string(),
            ] {
                if std::path::Path::new(&candidate).is_dir() && !pulse_lib_dirs.contains(&candidate)
                {
                    pulse_lib_dirs.push(candidate);
                }
            }
            if !pulse_lib_dirs.is_empty() {
                let current_ld = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
                let mut parts: Vec<String> = if current_ld.is_empty() {
                    Vec::new()
                } else {
                    current_ld.split(':').map(|s| s.to_string()).collect()
                };
                for p in pulse_lib_dirs {
                    if !parts.iter().any(|existing| existing == &p) {
                        parts.push(p);
                    }
                }
                unsafe {
                    std::env::set_var("LD_LIBRARY_PATH", parts.join(":"));
                }
            }

            // Ensure ALSA_PLUGIN_DIR points to bundled alsa-lib plugins across any CPU architecture or distro layout
            for plugin_candidate in [
                format!("{}/usr/lib/x86_64-linux-gnu/alsa-lib", root),
                format!("{}/usr/lib/aarch64-linux-gnu/alsa-lib", root),
                format!("{}/usr/lib64/alsa-lib", root),
                format!("{}/usr/lib/alsa-lib", root),
                format!("{}/gnome-platform/usr/lib/x86_64-linux-gnu/alsa-lib", root),
                format!("{}/gnome-platform/usr/lib/aarch64-linux-gnu/alsa-lib", root),
            ] {
                if std::path::Path::new(&plugin_candidate).is_dir() {
                    unsafe {
                        std::env::set_var("ALSA_PLUGIN_DIR", &plugin_candidate);
                    }
                    break;
                }
            }

            // Ensure ALSA_CONFIG_DIR and ALSA_CONFIG_PATH point to alsa.conf, and write
            // $XDG_CONFIG_HOME/alsa/asoundrc so alsa.conf's @hooks routes "default" and "pulse"
            // to PulseAudio/PipeWire instead of card 0 (which is often HDMI output-only).
            let snap_alsa = format!("{}/usr/share/alsa", root);
            let gnome_alsa = format!("{}/gnome-platform/usr/share/alsa", root);
            let alsa_dir = if std::path::Path::new(&format!("{}/alsa.conf", snap_alsa)).exists() {
                Some(snap_alsa)
            } else if std::path::Path::new(&format!("{}/alsa.conf", gnome_alsa)).exists() {
                Some(gnome_alsa)
            } else {
                None
            };

            if let Some(ref dir) = alsa_dir
                && (env_key == "SNAP"
                    || !std::path::Path::new("/usr/share/alsa/alsa.conf").exists())
            {
                unsafe {
                    std::env::set_var("ALSA_CONFIG_DIR", dir);
                    std::env::set_var("ALSA_CONFIG_PATH", format!("{}/alsa.conf", dir));
                }

                let asoundrc_content = "\
pcm.!default {
    type pulse
    fallback \"sysdefault\"
    hint {
        show on
        description \"Default ALSA Output (currently PulseAudio/PipeWire Sound Server)\"
    }
}
ctl.!default {
    type pulse
    fallback \"sysdefault\"
}
pcm.pulse {
    type pulse
    hint {
        show on
        description \"PulseAudio/PipeWire Sound Server\"
    }
}
ctl.pulse {
    type pulse
}
";
                let xdg_cfg = std::env::var("XDG_CONFIG_HOME").ok().or_else(|| {
                    std::env::var("HOME")
                        .ok()
                        .map(|h| format!("{}/.config", h.trim_end_matches('/')))
                });
                if let Some(cfg_root) = xdg_cfg {
                    let alsa_cfg_dir = std::path::PathBuf::from(&cfg_root).join("alsa");
                    if std::fs::create_dir_all(&alsa_cfg_dir).is_ok() {
                        let _ = std::fs::write(alsa_cfg_dir.join("asoundrc"), asoundrc_content);
                    }
                }
            }
        }
    }

    type AlsaErrorHandler = unsafe extern "C" fn(
        *const std::os::raw::c_char,
        std::os::raw::c_int,
        *const std::os::raw::c_char,
        std::os::raw::c_int,
        *const std::os::raw::c_char,
    );
    unsafe extern "C" {
        fn snd_lib_error_set_handler(handler: Option<AlsaErrorHandler>) -> std::os::raw::c_int;
    }
    unsafe extern "C" fn noop_handler(
        _file: *const std::os::raw::c_char,
        _line: std::os::raw::c_int,
        _function: *const std::os::raw::c_char,
        _err: std::os::raw::c_int,
        _fmt: *const std::os::raw::c_char,
    ) {
    }
    unsafe {
        let _ = snd_lib_error_set_handler(Some(noop_handler));
    }
}

#[cfg(not(target_os = "linux"))]
pub fn silence_alsa_logging() {}

/// Detailed information for a PipeWire/WirePlumber audio source node discovered via `wpctl status` and `wpctl inspect`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WpctlSource {
    pub id: u32,
    pub is_default: bool,
    pub description: String,
    pub volume: Option<f32>,
    pub is_muted: bool,
    pub node_name: String,
    pub device_api: String,
    pub device_bus: String,
    pub media_class: String,
}

fn resolve_wpctl_binary() -> Option<String> {
    silence_alsa_logging();
    for env_key in ["SNAP", "APPDIR"] {
        if let Ok(root) = std::env::var(env_key) {
            let candidate = format!("{}/usr/bin/wpctl", root.trim_end_matches('/'));
            if std::path::Path::new(&candidate).exists() {
                return Some(candidate);
            }
        }
    }
    if let Ok(output) = std::process::Command::new("wpctl")
        .arg("--version")
        .output()
        && output.status.success()
    {
        return Some("wpctl".to_string());
    }
    for p in ["/usr/bin/wpctl", "/bin/wpctl", "/usr/local/bin/wpctl"] {
        if std::path::Path::new(p).exists() {
            return Some(p.to_string());
        }
    }
    None
}

fn run_wpctl_stdout(args: &[&str]) -> String {
    let Some(bin) = resolve_wpctl_binary() else {
        return String::new();
    };
    std::process::Command::new(bin)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
}

fn run_wpctl_status(args: &[&str]) -> bool {
    let Some(bin) = resolve_wpctl_binary() else {
        return false;
    };
    std::process::Command::new(bin)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn resolve_pactl_binary() -> String {
    silence_alsa_logging();
    for env_key in ["SNAP", "APPDIR"] {
        if let Ok(root) = std::env::var(env_key) {
            let candidate = format!("{}/usr/bin/pactl", root.trim_end_matches('/'));
            if std::path::Path::new(&candidate).exists() {
                return candidate;
            }
        }
    }
    "pactl".to_string()
}

fn run_pactl_stdout(args: &[&str]) -> String {
    let bin = resolve_pactl_binary();
    std::process::Command::new(bin)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
}

fn run_pactl_status(args: &[&str]) {
    let bin = resolve_pactl_binary();
    let _ = std::process::Command::new(bin).args(args).status();
}

/// Inspects a `Card #` block from `pactl list cards` and, if it is a Bluetooth card currently in an output-only
/// profile (like `a2dp-sink`), returns `(card_name, best_input_profile)` for any available HFP/HSP/duplex profile.
fn resolve_bluetooth_input_profile_switch(card_block: &str) -> Option<(String, String)> {
    AudioRecorder::resolve_bluetooth_input_profile_switch(card_block)
}

fn resolve_available_port_for_source(
    sources_output: &str,
    target_source: &str,
    target_profile: &str,
) -> Option<String> {
    AudioRecorder::resolve_available_port_for_source(sources_output, target_source, target_profile)
}

impl AudioRecorder {
    /// Parses audio source nodes from `wpctl status` and inspect properties via `inspect_fn`,
    /// excluding monitor sinks and video nodes so capture only connects to genuine microphone inputs.
    pub fn parse_wpctl_sources(
        status_output: &str,
        inspect_fn: impl Fn(u32) -> String,
    ) -> Vec<WpctlSource> {
        let mut sources = Vec::new();
        let mut in_audio = false;
        let mut in_sources = false;

        for line in status_output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed == "Audio" || trimmed.starts_with("Audio ") {
                in_audio = true;
                in_sources = false;
                continue;
            }

            if trimmed == "Video"
                || trimmed.starts_with("Video ")
                || trimmed == "Settings"
                || trimmed.starts_with("Settings ")
            {
                in_audio = false;
                in_sources = false;
                continue;
            }

            if in_audio {
                if trimmed.contains("Sources:") {
                    in_sources = true;
                    continue;
                }
                if in_sources
                    && (trimmed.contains("Filters:")
                        || trimmed.contains("Streams:")
                        || trimmed.contains("Sinks:")
                        || trimmed.contains("Devices:"))
                {
                    in_sources = false;
                    continue;
                }
            }

            if !in_sources {
                continue;
            }

            // A source line typically looks like:
            //   " │      58. Ryzen HD Audio Controller Analog Stereo [vol: 0.55]"
            //   " │  *   69. Stone 250                           [vol: 1.00]"
            let is_default = line.contains('*');
            let is_muted = line.contains("MUTED");

            let mut node_id = None;
            let mut id_token = "";
            for word in line.split_whitespace() {
                if let Some(num_candidate) = word.strip_suffix('.')
                    && let Ok(val) = num_candidate.parse::<u32>()
                {
                    node_id = Some(val);
                    id_token = word;
                    break;
                }
            }

            let Some(id) = node_id else {
                continue;
            };

            // Extract description and volume
            let mut description = String::new();
            let mut volume = None;

            if let Some(pos) = line.find(id_token) {
                let rest = line[pos + id_token.len()..].trim();
                let (raw_desc, bracket_part) = if let Some(bracket_idx) = rest.find('[') {
                    (&rest[..bracket_idx], Some(&rest[bracket_idx..]))
                } else {
                    (rest, None)
                };

                description = raw_desc.trim().to_string();

                if let Some(bracket) = bracket_part
                    && let Some(vol_idx) = bracket.find("vol:")
                {
                    let after_vol = &bracket[vol_idx + 4..];
                    if let Some(token) = after_vol.split_whitespace().next() {
                        let clean_token = token.trim_end_matches(']').trim_end_matches(',');
                        if let Some(pct_str) = clean_token.strip_suffix('%') {
                            if let Ok(pct) = pct_str.parse::<f32>() {
                                volume = Some(pct / 100.0);
                            }
                        } else if let Ok(val) = clean_token.parse::<f32>() {
                            volume = Some(val);
                        }
                    }
                }
            }

            // Inspect the node
            let inspect_output = inspect_fn(id);
            let mut node_name = String::new();
            let mut device_api = String::new();
            let mut device_bus = String::new();
            let mut media_class = String::new();
            let mut node_desc = String::new();

            for insp_line in inspect_output.lines() {
                let t = insp_line.trim().trim_start_matches('*').trim();
                if let Some((k, v)) = t.split_once('=') {
                    let key = k.trim();
                    let val = v.trim().trim_matches('"').trim();
                    match key {
                        "node.name" => node_name = val.to_string(),
                        "device.api" => device_api = val.to_string(),
                        "device.bus" => device_bus = val.to_string(),
                        "media.class" => media_class = val.to_string(),
                        "node.description" => node_desc = val.to_string(),
                        _ => {}
                    }
                }
            }

            if description.is_empty() && !node_desc.is_empty() {
                description = node_desc;
            }

            // If media.class was inspected, only accept Audio/Source (skip Audio/Sink or Video/Source)
            if !media_class.is_empty() && media_class != "Audio/Source" {
                continue;
            }

            // Also skip obvious monitor names if node_name ends with .monitor
            if node_name.ends_with(".monitor") {
                continue;
            }

            sources.push(WpctlSource {
                id,
                is_default,
                description,
                volume,
                is_muted,
                node_name,
                device_api,
                device_bus,
                media_class,
            });
        }

        sources
    }

    /// Classifies the currently connected microphone hardware profile from a slice of `WpctlSource`s
    /// (`"Handsfree"`, `"Headphones"`, or `"System Default"`).
    pub fn classify_hardware_profile_from_wpctl_sources(sources: &[WpctlSource]) -> &'static str {
        let is_bluetooth = |s: &WpctlSource| -> bool {
            s.device_api == "bluez5"
                || s.device_bus == "bluetooth"
                || s.node_name.to_lowercase().starts_with("bluez_")
                || s.description.to_lowercase().contains("bluetooth")
        };

        let is_headset = |s: &WpctlSource| -> bool {
            s.device_bus == "usb"
                || s.node_name.to_lowercase().starts_with("alsa_input.usb-")
                || s.description.to_lowercase().contains("headset")
                || s.description.to_lowercase().contains("headphone")
        };

        if sources.iter().any(is_bluetooth) {
            return "Handsfree";
        }

        if sources.iter().any(is_headset) {
            return "Headphones";
        }

        "System Default"
    }

    /// Resolves the single exclusive PipeWire/WirePlumber `WpctlSource` corresponding to `target_profile`,
    /// guaranteeing built-in microphones are routed for `"System Default"` even when Bluetooth speakers
    /// with silent dummy profiles are attached.
    pub fn resolve_exclusive_source_from_wpctl(
        target_profile: &str,
        sources: &[WpctlSource],
    ) -> Option<WpctlSource> {
        if sources.is_empty() {
            return None;
        }

        let is_bluetooth = |s: &WpctlSource| -> bool {
            s.device_api == "bluez5"
                || s.device_bus == "bluetooth"
                || s.node_name.to_lowercase().starts_with("bluez_")
                || s.description.to_lowercase().contains("bluetooth")
        };

        let is_headset = |s: &WpctlSource| -> bool {
            s.device_bus == "usb"
                || s.node_name.to_lowercase().starts_with("alsa_input.usb-")
                || s.description.to_lowercase().contains("headset")
                || s.description.to_lowercase().contains("headphone")
        };

        match target_profile {
            "Handsfree" => {
                if let Some(bt) = sources.iter().find(|s| is_bluetooth(s)) {
                    return Some(bt.clone());
                }
            }
            "Headphones" => {
                if let Some(hp) = sources.iter().find(|s| is_headset(s)) {
                    return Some(hp.clone());
                }
            }
            "System Default" | "default" => {
                // Priority 1: Built-in PCI / analog / internal microphone
                if let Some(internal) = sources.iter().find(|s| {
                    !is_bluetooth(s)
                        && !is_headset(s)
                        && (s.device_bus == "pci"
                            || s.node_name.contains("pci")
                            || s.description.to_lowercase().contains("analog")
                            || s.description.to_lowercase().contains("internal")
                            || s.description.to_lowercase().contains("built-in"))
                }) {
                    return Some(internal.clone());
                }

                // Priority 2: Any non-Bluetooth, non-headset microphone
                if let Some(non_bt) = sources.iter().find(|s| !is_bluetooth(s) && !is_headset(s)) {
                    return Some(non_bt.clone());
                }

                // Priority 3: Default source if not Bluetooth
                if let Some(def) = sources.iter().find(|s| s.is_default && !is_bluetooth(s)) {
                    return Some(def.clone());
                }

                // Priority 4: First non-Bluetooth source
                if let Some(first_non_bt) = sources.iter().find(|s| !is_bluetooth(s)) {
                    return Some(first_non_bt.clone());
                }

                // Fallback: Default source or first source
                if let Some(def) = sources.iter().find(|s| s.is_default) {
                    return Some(def.clone());
                }
                return sources.first().cloned();
            }
            custom => {
                let custom_lower = custom.to_lowercase();
                if let Ok(target_id) = custom.parse::<u32>()
                    && let Some(matched) = sources.iter().find(|s| s.id == target_id)
                {
                    return Some(matched.clone());
                }
                if let Some(matched) = sources.iter().find(|s| {
                    s.node_name.to_lowercase() == custom_lower
                        || s.description.to_lowercase() == custom_lower
                        || s.description.to_lowercase().contains(&custom_lower)
                }) {
                    return Some(matched.clone());
                }
            }
        }

        None
    }

    /// Inspects a `Card #` block from `pactl list cards` and, if it is a Bluetooth card currently
    /// on a playback-only (`a2dp-sink`, `bap-sink`) or `off` profile, returns `(card_name, target_input_profile)`
    /// for the highest-priority available duplex/headset profile that provides at least 1 input source
    /// (`headset-head-unit`, `bap-duplex`, `headset-head-unit-msbc`, `headset-head-unit-cvsd`, `hfp_hf`, `hsp_hs`, etc.).
    pub fn resolve_bluetooth_input_profile_switch(card_block: &str) -> Option<(String, String)> {
        let mut card_name = "";
        let mut is_bt = false;
        let mut active_profile = "";
        let mut in_profiles = false;
        let mut candidates: Vec<(i32, String)> = Vec::new();

        for line in card_block.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("Name:") {
                card_name = rest.trim();
                if card_name.starts_with("bluez_card.") {
                    is_bt = true;
                }
            } else if t.contains("device.bus = \"bluetooth\"") || t.contains("device.api = \"bluez")
            {
                is_bt = true;
            } else if t == "Profiles:" {
                in_profiles = true;
            } else if let Some(rest) = t.strip_prefix("Active Profile:") {
                in_profiles = false;
                active_profile = rest.trim();
            } else if in_profiles
                && !t.starts_with("off:")
                && !t.contains("available: no")
                && let Some((prof_name, desc)) = t.split_once(':')
            {
                let prof = prof_name.trim();
                let desc_lower = desc.to_lowercase();
                let has_source = !desc_lower.contains("sources: 0")
                    && (desc_lower.contains("sources: 1")
                        || desc_lower.contains("sources: 2")
                        || prof.starts_with("headset-head-unit")
                        || prof.starts_with("hfp")
                        || prof.starts_with("hsp")
                        || prof.starts_with("bap-duplex"));
                if has_source {
                    let score = if prof == "headset-head-unit" || prof.contains("msbc") {
                        100
                    } else if prof.starts_with("bap") {
                        90
                    } else if prof.starts_with("headset-head-unit") {
                        80
                    } else {
                        50
                    };
                    candidates.push((score, prof.to_string()));
                }
            }
        }

        if !is_bt || card_name.is_empty() {
            return None;
        }
        let active_needs_switch = active_profile.is_empty()
            || active_profile == "off"
            || active_profile.starts_with("a2dp-sink")
            || active_profile.starts_with("bap-sink")
            || active_profile.ends_with("-sink");
        if !active_needs_switch {
            return None;
        }

        candidates.sort_by_key(|a| std::cmp::Reverse(a.0));
        candidates
            .into_iter()
            .next()
            .map(|(_, prof)| (card_name.to_string(), prof))
    }

    /// Inspects the `Ports:` section of the matched `Source #` block in `pactl list sources`
    /// across any hardware driver (PCI HDA, Intel SOF `HiFi__...`, Apple Silicon `platform-sound`, ARM64, USB)
    /// and returns the exact port identifier to activate for `target_profile` if available.
    pub fn resolve_available_port_for_source(
        sources_output: &str,
        target_source: &str,
        target_profile: &str,
    ) -> Option<String> {
        for block in sources_output.split("Source #").skip(1) {
            let mut is_target = false;
            let mut in_ports = false;
            let mut available_ports: Vec<(String, String)> = Vec::new();

            for line in block.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Name:") {
                    if rest.trim() == target_source {
                        is_target = true;
                    }
                } else if t == "Ports:" {
                    in_ports = true;
                } else if t.starts_with("Active Port:") {
                    in_ports = false;
                } else if in_ports {
                    if t.starts_with("Properties:")
                        || t.starts_with("Part of profile")
                        || t.contains('=')
                    {
                        continue;
                    }
                    if !t.contains("not available)")
                        && let Some((port_id, rest)) = t.split_once(':')
                    {
                        let pid = port_id.trim();
                        if !pid.is_empty() {
                            available_ports.push((pid.to_string(), rest.trim().to_lowercase()));
                        }
                    }
                }
            }

            if !is_target {
                continue;
            }

            for (port_id, desc_lower) in &available_ports {
                let pid_lower = port_id.to_lowercase();
                let matches_profile = match target_profile {
                    "Headphones" => {
                        pid_lower.contains("headset-mic")
                            || pid_lower.contains("headphone-mic")
                            || pid_lower.contains("mic2")
                            || pid_lower == "analog-input-mic"
                            || desc_lower.contains("headset")
                            || desc_lower.contains("headphones")
                    }
                    "System Default" => {
                        pid_lower.contains("internal-mic")
                            || pid_lower.contains("mic1")
                            || pid_lower.contains("dmic")
                            || desc_lower.contains("internal")
                            || desc_lower.contains("built-in")
                            || desc_lower.contains("digital microphone")
                    }
                    _ => false,
                };
                if matches_profile {
                    return Some(port_id.clone());
                }
            }

            // If "Headphones" is selected with 3-pole TRS headphones plugged in (so external mic port
            // is marked "not available"), fall back cleanly to the internal microphone port.
            if target_profile == "Headphones" {
                for (port_id, desc_lower) in &available_ports {
                    let pid_lower = port_id.to_lowercase();
                    if pid_lower.contains("internal-mic")
                        || pid_lower.contains("mic1")
                        || pid_lower.contains("dmic")
                        || desc_lower.contains("internal")
                        || desc_lower.contains("built-in")
                        || desc_lower.contains("digital microphone")
                    {
                        return Some(port_id.clone());
                    }
                }
            }
        }
        None
    }

    /// Ensures the selected PulseAudio/PipeWire source is neither muted/near-zero (`< 15%`)
    /// nor over-driven into +60 dB hardware Internal Mic Boost saturation (`> 85%` when
    /// `Base Volume <= -55.0 dB` on an `internal-mic` port). External/headset microphone
    /// user volume sliders are never lowered.
    fn ensure_healthy_source_gain(
        sources_output: &str,
        target_source: &str,
        selected_port: Option<&str>,
    ) {
        for block in sources_output.split("Source #").skip(1) {
            let mut is_target = false;
            let mut vol_pct: Option<u32> = None;
            let mut base_db: Option<f32> = None;
            let mut active_port = String::new();

            for line in block.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Name:") {
                    if rest.trim() == target_source {
                        is_target = true;
                    }
                } else if is_target && t.starts_with("Volume:") && vol_pct.is_none() {
                    for part in t.split('/') {
                        let p = part.trim();
                        if let Some(num_str) = p.strip_suffix('%')
                            && let Ok(pct) = num_str.trim().parse::<u32>()
                        {
                            vol_pct = Some(pct);
                            break;
                        }
                    }
                } else if is_target && t.starts_with("Base Volume:") && base_db.is_none() {
                    for part in t.split('/') {
                        let p = part.trim();
                        if let Some(db_str) = p.strip_suffix("dB")
                            && let Ok(db) = db_str.trim().parse::<f32>()
                        {
                            base_db = Some(db);
                            break;
                        }
                    }
                } else if is_target && let Some(rest) = t.strip_prefix("Active Port:") {
                    active_port = rest.trim().to_lowercase();
                }
            }

            if !is_target {
                continue;
            }

            let effective_port = selected_port
                .map(|p| p.to_lowercase())
                .unwrap_or(active_port);
            let is_internal_mic = effective_port.contains("internal-mic");

            if let Some(pct) = vol_pct {
                if pct < 15 {
                    run_pactl_status(&["set-source-volume", target_source, "50%"]);
                } else if is_internal_mic
                    && let Some(bdb) = base_db
                    && bdb <= -55.0
                    && pct > 85
                {
                    run_pactl_status(&["set-source-volume", target_source, "55%"]);
                }
            }
            break;
        }
    }

    /// Returns true if the requested profile name (`None`, `"System Default"`, `"default"`,
    /// `"Headphones"`, or `"Handsfree"`) should open the default PulseAudio/PipeWire-backed
    /// ALSA device (`PULSE_SOURCE`) rather than scanning raw hardware `hw:` / `iec958:` ALSA subdevices.
    pub fn should_use_default_pulse_device(device_name: Option<&str>) -> bool {
        matches!(
            device_name.map(|s| s.trim()),
            None | Some("")
                | Some("System Default")
                | Some("default")
                | Some("Headphones")
                | Some("Handsfree")
        )
    }

    /// Initialize audio recorder for optional named device, or system default device if `None`.
    pub fn new(device_name: Option<String>) -> Result<Self, AudioError> {
        silence_alsa_logging();
        let host = cpal::default_host();
        if let Some(ref name) = device_name
            && !Self::should_use_default_pulse_device(Some(name))
            && let Ok(devices) = host.input_devices()
        {
            let mut found = false;
            let mut any_device = false;
            for dev in devices {
                any_device = true;
                if let Ok(dev_name) = dev.name()
                    && dev_name.contains(name)
                {
                    found = true;
                    break;
                }
            }
            if any_device && !found {
                return Err(AudioError::DeviceNotFound(name.clone()));
            }
        }

        Ok(Self {
            device_name,
            is_recording: Arc::new(AtomicBool::new(false)),
            is_paused: Arc::new(AtomicBool::new(false)),
            stream_error: Arc::new(AtomicBool::new(false)),
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: Arc::new(AtomicU32::new(16000)),
        })
    }

    /// Standardize microphone options to the three core hardware profiles:
    /// `"System Default"`, `"Headphones"`, and `"Handsfree"`.
    pub fn filter_and_standardize_devices(_raw_devices: &[String]) -> Vec<String> {
        vec![
            "System Default".to_string(),
            "Headphones".to_string(),
            "Handsfree".to_string(),
        ]
    }

    /// Classify the active hardware microphone profile (`"Handsfree"`, `"Headphones"`, or `"System Default"`)
    /// from `pactl list sources` output, `pactl list cards` output, and fallback raw device names.
    /// Loopback `.monitor` sources are strictly excluded so speaker monitors never trigger false positives.
    pub fn classify_hardware_profile_from_sources_and_cards(
        sources_output: &str,
        cards_output: &str,
        raw_devices: &[String],
    ) -> &'static str {
        let mut found_headphones = false;

        for block in sources_output.split("Source #").skip(1) {
            let mut source_name = "";
            let mut description = "";
            let mut is_monitor = false;
            let mut is_bluetooth = false;
            let mut is_usb = false;
            let mut is_headset_form = false;
            let mut active_port = "";
            let mut headset_port_available = false;

            for line in block.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Name:") {
                    source_name = rest.trim();
                    if source_name.ends_with(".monitor") {
                        is_monitor = true;
                    }
                } else if let Some(rest) = t.strip_prefix("Description:") {
                    description = rest.trim();
                } else if t.contains("device.class = \"monitor\"") {
                    is_monitor = true;
                } else if t.contains("device.api = \"bluez")
                    || t.contains("device.bus = \"bluetooth\"")
                {
                    is_bluetooth = true;
                } else if t.contains("device.bus = \"usb\"") {
                    is_usb = true;
                } else if t.contains("device.form_factor = \"headset\"")
                    || t.contains("device.form_factor = \"headphone\"")
                    || t.contains("device.form_factor = \"hands-free\"")
                {
                    is_headset_form = true;
                } else if let Some(rest) = t.strip_prefix("Active Port:") {
                    active_port = rest.trim();
                } else if (t.starts_with("analog-input-headset-mic:")
                    || t.starts_with("analog-input-headphone-mic:")
                    || t.starts_with("[In] Mic2:"))
                    && t.contains("available)")
                    && !t.contains("not available)")
                {
                    headset_port_available = true;
                }
            }

            if is_monitor {
                continue;
            }

            let name_lower = source_name.to_lowercase();
            let desc_lower = description.to_lowercase();
            let port_lower = active_port.to_lowercase();

            if is_bluetooth
                || name_lower.starts_with("bluez_input.")
                || name_lower.starts_with("bluez_source.")
                || port_lower.contains("headset-head-unit")
                || port_lower.contains("bluetooth-input")
                || port_lower.contains("a2dp-source")
            {
                return "Handsfree";
            }

            if port_lower.contains("headset-mic")
                || port_lower.contains("headphone-mic")
                || port_lower.contains("mic2")
                || headset_port_available
                || ((is_usb || name_lower.starts_with("alsa_input.usb-"))
                    && (is_headset_form
                        || name_lower.contains("headset")
                        || name_lower.contains("headphone")
                        || desc_lower.contains("headset")
                        || desc_lower.contains("headphone")))
            {
                found_headphones = true;
            }
        }

        // Check Bluetooth cards that are connected and have an HFP/HSP/duplex input profile available,
        // and check analog cards for plugged-in 3.5mm headphones (`analog-output-headphones: ... available)`).
        for card_block in cards_output.split("Card #").skip(1) {
            if resolve_bluetooth_input_profile_switch(card_block).is_some() {
                return "Handsfree";
            }
            for line in card_block.lines() {
                let t = line.trim();
                if (t.starts_with("analog-output-headphones:")
                    || t.starts_with("analog-input-headset-mic:")
                    || t.starts_with("analog-input-headphone-mic:"))
                    && t.contains("available)")
                    && !t.contains("not available)")
                {
                    found_headphones = true;
                }
            }
        }

        if found_headphones {
            return "Headphones";
        }

        // Fallback check on non-monitor raw device names
        for dev in raw_devices {
            let lower = dev.to_lowercase();
            if lower.ends_with(".monitor") {
                continue;
            }
            if lower.contains("handsfree")
                || lower.contains("handset")
                || lower.contains("bluetooth")
                || lower.contains("bluez")
            {
                return "Handsfree";
            }
            if lower.contains("headphone") || lower.contains("headset") {
                found_headphones = true;
            }
        }

        if found_headphones {
            "Headphones"
        } else {
            "System Default"
        }
    }

    /// Resolve the single exclusive PulseAudio/PipeWire source name for the given profile
    /// (`"Handsfree"`, `"Headphones"`, or `"System Default"`), never returning a `.monitor` sink.
    pub fn resolve_exclusive_source_from_pactl(
        profile: &str,
        sources_output: &str,
        _cards_output: &str,
    ) -> Option<String> {
        let mut fallback_non_monitor = None;

        for block in sources_output.split("Source #").skip(1) {
            let mut source_name = "";
            let mut description = "";
            let mut is_monitor = false;
            let mut is_bluetooth = false;
            let mut is_usb = false;
            let mut is_headset_form = false;
            let mut active_port = "";
            let mut headset_port_available = false;

            for line in block.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Name:") {
                    source_name = rest.trim();
                    if source_name.ends_with(".monitor") {
                        is_monitor = true;
                    }
                } else if let Some(rest) = t.strip_prefix("Description:") {
                    description = rest.trim();
                } else if t.contains("device.class = \"monitor\"") {
                    is_monitor = true;
                } else if t.contains("device.api = \"bluez")
                    || t.contains("device.bus = \"bluetooth\"")
                {
                    is_bluetooth = true;
                } else if t.contains("device.bus = \"usb\"") {
                    is_usb = true;
                } else if t.contains("device.form_factor = \"headset\"")
                    || t.contains("device.form_factor = \"headphone\"")
                {
                    is_headset_form = true;
                } else if let Some(rest) = t.strip_prefix("Active Port:") {
                    active_port = rest.trim();
                } else if (t.starts_with("analog-input-headset-mic:")
                    || t.starts_with("analog-input-headphone-mic:")
                    || t.starts_with("[In] Mic2:"))
                    && t.contains("available)")
                    && !t.contains("not available)")
                {
                    headset_port_available = true;
                }
            }

            if is_monitor || source_name.is_empty() {
                continue;
            }

            let name_lower = source_name.to_lowercase();
            let desc_lower = description.to_lowercase();
            let port_lower = active_port.to_lowercase();

            let is_bt_source = is_bluetooth
                || name_lower.starts_with("bluez_input.")
                || name_lower.starts_with("bluez_source.")
                || port_lower.contains("headset-head-unit")
                || port_lower.contains("bluetooth-input");

            let is_hp_source = port_lower.contains("headset-mic")
                || port_lower.contains("headphone-mic")
                || port_lower.contains("mic2")
                || headset_port_available
                || is_usb
                || name_lower.starts_with("alsa_input.usb-")
                || is_headset_form
                || name_lower.contains("headset")
                || name_lower.contains("headphone")
                || desc_lower.contains("headset")
                || desc_lower.contains("headphone");

            match profile {
                "Handsfree" if is_bt_source => return Some(source_name.to_string()),
                "Headphones" if is_hp_source => return Some(source_name.to_string()),
                "System Default" if !is_bt_source && !is_hp_source => {
                    return Some(source_name.to_string());
                }
                _ => {
                    if fallback_non_monitor.is_none() {
                        fallback_non_monitor = Some(source_name.to_string());
                    }
                }
            }
        }

        fallback_non_monitor
    }

    /// Queries the live Linux audio stack (`pactl list sources` and `pactl list cards`)
    /// to detect the currently connected microphone hardware profile.
    pub fn detect_connected_audio_profile() -> &'static str {
        silence_alsa_logging();

        // 1. Prefer native WirePlumber (wpctl) if available
        if resolve_wpctl_binary().is_some() {
            let status_out = run_wpctl_stdout(&["status"]);
            if !status_out.is_empty() {
                let wp_sources = Self::parse_wpctl_sources(&status_out, |id| {
                    run_wpctl_stdout(&["inspect", &id.to_string()])
                });
                if !wp_sources.is_empty() {
                    return Self::classify_hardware_profile_from_wpctl_sources(&wp_sources);
                }
            }
        }

        // 2. Fall back to PulseAudio (pactl)
        let sources_out = run_pactl_stdout(&["list", "sources"]);
        let cards_out = run_pactl_stdout(&["list", "cards"]);

        let raw_devices = if sources_out.is_empty() && cards_out.is_empty() {
            Self::list_input_devices().unwrap_or_default()
        } else {
            Vec::new()
        };

        Self::classify_hardware_profile_from_sources_and_cards(
            &sources_out,
            &cards_out,
            &raw_devices,
        )
    }

    /// Query input devices and return only friendly human-readable microphone names.
    /// Does not enumerate raw hardware ALSA `hw:` devices so PipeWire nodes are never locked.
    pub fn list_friendly_input_devices() -> Vec<String> {
        Self::filter_and_standardize_devices(&[])
    }

    /// Query and list available input devices on the default host.
    pub fn list_input_devices() -> Result<Vec<String>, AudioError> {
        silence_alsa_logging();
        let host = cpal::default_host();
        let devices = host.input_devices()?;
        let mut names = Vec::new();
        for dev in devices {
            if let Ok(name) = dev.name() {
                names.push(name);
            }
        }
        names.sort();
        names.dedup();
        Ok(names)
    }

    /// Start recording audio from a single exclusive input device and emit 60 Hz 5-bar RMS updates to `level_tx`.
    pub fn start_recording(
        &mut self,
        level_tx: tokio::sync::mpsc::Sender<[f32; 5]>,
    ) -> Result<(), AudioError> {
        silence_alsa_logging();
        if self.is_recording() {
            return Ok(());
        }

        // Always close any lingering stream first so multiple devices never capture simultaneously
        self.stream.take();

        let target_profile = self.device_name.as_deref().unwrap_or("System Default");

        // Route PipeWire/PulseAudio exclusively to the selected hardware profile's source
        let mut routed_exclusive = false;

        // 1. Prefer native WirePlumber (wpctl) if available
        if resolve_wpctl_binary().is_some() {
            let status_out = run_wpctl_stdout(&["status"]);
            if !status_out.is_empty() {
                let wp_sources = Self::parse_wpctl_sources(&status_out, |id| {
                    run_wpctl_stdout(&["inspect", &id.to_string()])
                });
                if let Some(target_source) =
                    Self::resolve_exclusive_source_from_wpctl(target_profile, &wp_sources)
                {
                    let id_str = target_source.id.to_string();
                    let _ = run_wpctl_status(&["set-mute", &id_str, "0"]);

                    if let Some(vol) = target_source.volume {
                        if vol < 0.20 {
                            let _ = run_wpctl_status(&["set-volume", &id_str, "0.55"]);
                        } else if vol > 0.90
                            && (target_source.device_bus == "pci"
                                || target_source.node_name.contains("pci"))
                        {
                            let _ = run_wpctl_status(&["set-volume", &id_str, "0.65"]);
                        }
                    } else if target_source.is_muted {
                        let _ = run_wpctl_status(&["set-volume", &id_str, "0.55"]);
                    }

                    let _ = run_wpctl_status(&["set-default", &id_str]);

                    unsafe {
                        std::env::set_var("PIPEWIRE_NODE", &target_source.node_name);
                        std::env::set_var("PULSE_SOURCE", &target_source.node_name);
                    }
                    routed_exclusive = true;
                }
            }
        }

        // 2. Fall back to PulseAudio (pactl) if wpctl routing was not performed
        if !routed_exclusive {
            let mut sources_out = run_pactl_stdout(&["list", "sources"]);
            let cards_out = run_pactl_stdout(&["list", "cards"]);

            if target_profile == "Handsfree" {
                let mut switched_bt_profile = false;
                for card_block in cards_out.split("Card #").skip(1) {
                    if let Some((card_name, best_profile)) =
                        resolve_bluetooth_input_profile_switch(card_block)
                    {
                        run_pactl_status(&["set-card-profile", &card_name, &best_profile]);
                        switched_bt_profile = true;
                    }
                }
                if switched_bt_profile {
                    std::thread::sleep(Duration::from_millis(150));
                    sources_out = run_pactl_stdout(&["list", "sources"]);
                }
            }

            if let Some(exclusive_source) =
                Self::resolve_exclusive_source_from_pactl(target_profile, &sources_out, &cards_out)
            {
                let selected_port = resolve_available_port_for_source(
                    &sources_out,
                    &exclusive_source,
                    target_profile,
                );
                if let Some(ref port_id) = selected_port {
                    run_pactl_status(&["set-source-port", &exclusive_source, port_id]);
                }
                run_pactl_status(&["set-source-mute", &exclusive_source, "0"]);
                Self::ensure_healthy_source_gain(
                    &sources_out,
                    &exclusive_source,
                    selected_port.as_deref(),
                );
                run_pactl_status(&["set-default-source", &exclusive_source]);
                unsafe {
                    std::env::set_var("PULSE_SOURCE", &exclusive_source);
                    std::env::set_var("PIPEWIRE_NODE", &exclusive_source);
                }
                routed_exclusive = true;
            } else if target_profile == "System Default" || target_profile == "default" {
                unsafe {
                    std::env::remove_var("PULSE_SOURCE");
                    std::env::remove_var("PIPEWIRE_NODE");
                }
            }
        }

        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
        self.is_paused.store(false, Ordering::SeqCst);
        self.stream_error.store(false, Ordering::SeqCst);

        // CRITICAL: Never hold multiple `cpal::Device` instances open simultaneously!
        // In cpal's ALSA backend, each `cpal::Device` keeps an open `snd_pcm_t` capture handle
        // for its entire lifetime. Holding `front:CARD=...` or `hw:CARD=...` in a Vec locks
        // `/dev/snd/pcmC*D*c` with EBUSY and prevents PipeWire/PulseAudio from waking the mic!
        let find_single_device =
            |host: &cpal::Host, match_fn: &dyn Fn(&str) -> bool| -> Option<cpal::Device> {
                let mut matched = None;
                if let Ok(devices) = host.input_devices() {
                    for dev in devices {
                        if matched.is_none()
                            && let Ok(name) = dev.name()
                            && match_fn(name.trim())
                        {
                            matched = Some(dev);
                        }
                        // All non-matching `dev` instances are dropped immediately at the end of each iteration,
                        // releasing their ALSA `snd_pcm_t` handles before we attempt to open the stream.
                    }
                }
                matched
            };

        let is_blacklisted_alsa_device = |name: &str| -> bool {
            let lower = name.trim().to_lowercase();
            lower == "null"
                || lower.ends_with(".monitor")
                || lower.starts_with("iec958:")
                || lower.starts_with("hdmi:")
                || lower.starts_with("surround")
                || lower.starts_with("dsnoop:")
                || lower.starts_with("dmix:")
        };

        let try_open_on_device =
            |device: &cpal::Device| -> Result<(cpal::Stream, u32), AudioError> {
                let supported_config = match device.default_input_config() {
                    Ok(cfg) => cfg,
                    Err(first_err) => {
                        let mut fallback_cfg = None;
                        if let Ok(mut ranges) = device.supported_input_configs()
                            && let Some(range) = ranges.next()
                        {
                            for preferred_rate in [48000, 44100, 16000, 32000, 24000, 8000] {
                                let sr = cpal::SampleRate(preferred_rate);
                                if sr >= range.min_sample_rate() && sr <= range.max_sample_rate() {
                                    fallback_cfg = Some(range.with_sample_rate(sr));
                                    break;
                                }
                            }
                            if fallback_cfg.is_none() {
                                fallback_cfg = Some(range.with_max_sample_rate());
                            }
                        }
                        match fallback_cfg {
                            Some(cfg) => cfg,
                            None => return Err(AudioError::from(first_err)),
                        }
                    }
                };

                let sample_format = supported_config.sample_format();
                let stream_config: cpal::StreamConfig = supported_config.into();
                let native_rate = stream_config.sample_rate.0;
                let channels = stream_config.channels as usize;

                let buffer = Arc::clone(&self.buffer);
                let is_paused = Arc::clone(&self.is_paused);
                let last_level_time = Arc::new(Mutex::new(Instant::now()));
                let dc_state: Arc<Mutex<Option<(f32, f32)>>> = Arc::new(Mutex::new(None));
                let stream_start_time = Instant::now();
                let level_tx = level_tx.clone();
                let stream_error_flag = Arc::clone(&self.stream_error);
                let is_recording_flag = Arc::clone(&self.is_recording);

                let err_fn = move |err: cpal::StreamError| {
                    log::error!("Audio stream error: {}", err);
                    stream_error_flag.store(true, Ordering::SeqCst);
                    is_recording_flag.store(false, Ordering::SeqCst);
                    crate::services::crash_reporter::CrashReporter::record_event(
                        "AudioStream",
                        &format!("Hardware audio stream error: {}", err),
                    );
                };

                let stream = match sample_format {
                    cpal::SampleFormat::F32 => device.build_input_stream(
                        &stream_config,
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            handle_audio_input(
                                data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::I16 => device.build_input_stream(
                        &stream_config,
                        move |data: &[i16], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> =
                                data.iter().map(|&s| s as f32 / 32768.0).collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::U16 => device.build_input_stream(
                        &stream_config,
                        move |data: &[u16], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> = data
                                .iter()
                                .map(|&s| (s as f32 - 32768.0) / 32768.0)
                                .collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::I32 => device.build_input_stream(
                        &stream_config,
                        move |data: &[i32], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> =
                                data.iter().map(|&s| s as f32 / 2147483648.0).collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::I8 => device.build_input_stream(
                        &stream_config,
                        move |data: &[i8], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> =
                                data.iter().map(|&s| s as f32 / 128.0).collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::U8 => device.build_input_stream(
                        &stream_config,
                        move |data: &[u8], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> =
                                data.iter().map(|&s| (s as f32 - 128.0) / 128.0).collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    cpal::SampleFormat::F64 => device.build_input_stream(
                        &stream_config,
                        move |data: &[f64], _: &cpal::InputCallbackInfo| {
                            let f32_data: Vec<f32> = data.iter().map(|&s| s as f32).collect();
                            handle_audio_input(
                                &f32_data,
                                channels,
                                &buffer,
                                &is_paused,
                                &last_level_time,
                                &dc_state,
                                stream_start_time,
                                &level_tx,
                            );
                        },
                        err_fn,
                        None,
                    )?,
                    _ => {
                        return Err(AudioError::StreamBuildError(
                            "Unsupported audio sample format".to_string(),
                        ));
                    }
                };

                stream.play()?;
                Ok((stream, native_rate))
            };

        let attempt_open_sequence = |host: &cpal::Host| -> Result<(cpal::Stream, u32), AudioError> {
            let mut last_err = AudioError::DefaultDeviceNotFound;

            // 1. If a custom non-standard ALSA device name was requested, try it first
            if !Self::should_use_default_pulse_device(self.device_name.as_deref())
                && let Some(ref req_name) = self.device_name
            {
                let req_lower = req_name.to_lowercase();
                if let Some(dev) = find_single_device(host, &|n| {
                    !is_blacklisted_alsa_device(n) && n.to_lowercase().contains(&req_lower)
                }) {
                    match try_open_on_device(&dev) {
                        Ok(res) => return Ok(res),
                        Err(e) => last_err = e,
                    }
                }
            }

            // 2. Try PulseAudio / PipeWire / Default sound-server devices one at a time
            // (never holding raw hw:/front: devices open while opening PulseAudio/PipeWire!)
            for preferred in ["pulse", "pipewire", "default"] {
                if let Some(dev) = find_single_device(host, &|n| n.eq_ignore_ascii_case(preferred))
                {
                    // Small settle delay after dropping enumerated hardware handles so PipeWire
                    // can claim the underlying ALSA PCM node without EBUSY
                    std::thread::sleep(Duration::from_millis(30));
                    match try_open_on_device(&dev) {
                        Ok(res) => return Ok(res),
                        Err(e) => last_err = e,
                    }
                }
            }

            if let Some(def_dev) = host.default_input_device() {
                match try_open_on_device(&def_dev) {
                    Ok(res) => return Ok(res),
                    Err(e) => last_err = e,
                }
            }

            // 3. Fallback: try sysdefault or remaining non-blacklisted devices one by one
            let mut fallback_names = Vec::new();
            if let Ok(devices) = host.input_devices() {
                for dev in devices {
                    if let Ok(n) = dev.name()
                        && !is_blacklisted_alsa_device(&n)
                    {
                        fallback_names.push(n);
                    }
                }
            }
            for name in fallback_names {
                if let Some(dev) = find_single_device(host, &|n| n == name) {
                    match try_open_on_device(&dev) {
                        Ok(res) => return Ok(res),
                        Err(e) => last_err = e,
                    }
                }
            }

            Err(last_err)
        };

        let host = cpal::default_host();
        let mut opened = attempt_open_sequence(&host);

        // If a specific PULSE_SOURCE / PIPEWIRE_NODE failed to open (e.g. unplugged or rejected stream),
        // automatically clear PULSE_SOURCE / PIPEWIRE_NODE so sound server falls back cleanly and retry.
        if opened.is_err() && routed_exclusive {
            unsafe {
                std::env::remove_var("PULSE_SOURCE");
                std::env::remove_var("PIPEWIRE_NODE");
            }
            opened = attempt_open_sequence(&host);
        }

        let (stream, native_rate) = opened?;

        self.sample_rate.store(native_rate, Ordering::SeqCst);
        self.stream = Some(stream);
        self.is_recording.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Returns a thread-safe live snapshot handle for non-destructive streaming transcription.
    pub fn snapshot_handle(&self) -> AudioBufferSnapshot {
        AudioBufferSnapshot {
            buffer: Arc::clone(&self.buffer),
            sample_rate: Arc::clone(&self.sample_rate),
            is_paused: Arc::clone(&self.is_paused),
            stream_error: Arc::clone(&self.stream_error),
        }
    }

    /// Atomically checks and clears any hardware stream disconnection/error flag.
    pub fn take_stream_error(&self) -> bool {
        self.stream_error.swap(false, Ordering::SeqCst)
    }

    /// Simulates a hardware stream disconnection/error for testing recovery behavior.
    pub fn simulate_stream_error(&self) {
        self.stream_error.store(true, Ordering::SeqCst);
        self.is_recording.store(false, Ordering::SeqCst);
    }

    /// Pause audio recording without destroying the stream. Discards samples and emits zeros.
    pub fn pause(&mut self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    /// Resume audio recording after being paused.
    pub fn resume(&mut self) {
        self.is_paused.store(false, Ordering::SeqCst);
    }

    /// Stop recording, close stream, resample to 16 kHz, normalize speech dynamic range,
    /// and encode to 16-bit PCM WAV bytes.
    pub fn stop_recording(&mut self) -> Result<Vec<u8>, AudioError> {
        let (normalized, _) = self.stop_recording_samples_16k(None)?;
        encode_pcm_wav(&normalized, 16000)
    }

    /// Stop recording, close stream, and return `(normalized_16k_samples, reused_finalized)`.
    /// When `finalized_native_samples` is `Some(prev_len)` from a completed streaming pass with
    /// a quiet tail, and no new voice activity was captured in `samples[prev_len..]` (for recordings
    /// `<= 60s`), skips resampling/normalization and returns `(Vec::new(), true)`.
    pub fn stop_recording_samples_16k(
        &mut self,
        finalized_native_samples: Option<usize>,
    ) -> Result<(Vec<f32>, bool), AudioError> {
        self.is_recording.store(false, Ordering::SeqCst);
        self.is_paused.store(false, Ordering::SeqCst);
        self.stream.take();

        let samples = {
            let mut buf = self.buffer.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *buf)
        };

        let native_rate = self.sample_rate.load(Ordering::SeqCst).max(8000);
        if samples.is_empty() {
            return Ok((Vec::new(), false));
        }

        let max_window_samples = (native_rate as usize) * 60;
        if let Some(prev_len) = finalized_native_samples
            && prev_len > 0
            && prev_len <= samples.len()
            && samples.len() <= max_window_samples
        {
            let delta = &samples[prev_len..];
            if delta.is_empty() || !has_voice_activity_at_rate(delta, native_rate) {
                return Ok((Vec::new(), true));
            }
        }

        let resampled = resample_to_16k(&samples, native_rate)?;
        let normalized = normalize_speech_samples(&resampled);
        Ok((normalized, false))
    }

    /// Appends synthetic samples to the internal capture buffer at `sample_rate` for deterministic benchmarks/tests.
    pub fn push_test_samples(&self, samples: &[f32], sample_rate: u32) {
        self.sample_rate
            .store(sample_rate.max(8000), Ordering::SeqCst);
        if let Ok(mut buf) = self.buffer.lock() {
            buf.extend_from_slice(samples);
        }
    }

    /// Starts recording audio without external level sender.
    pub fn start(&mut self) -> Result<(), AudioError> {
        let (tx, _rx) = tokio::sync::mpsc::channel(32);
        self.start_recording(tx)
    }

    /// Stops recording and returns encoded WAV bytes.
    pub fn stop(&mut self) -> Result<Vec<u8>, AudioError> {
        self.stop_recording()
    }

    /// Returns true if currently recording audio.
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    /// Returns true if recording is currently paused.
    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::SeqCst)
    }
}

// cpal::Stream is marked !Send on some platforms due to an internal phantom raw pointer.
// Since AudioRecorder exclusively accesses and drops the stream via `&mut self`,
// ownership of AudioRecorder can safely be transferred across threads.
unsafe impl Send for AudioRecorder {}

impl std::fmt::Debug for AudioRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioRecorder")
            .field("device_name", &self.device_name)
            .field("is_recording", &self.is_recording())
            .field("is_paused", &self.is_paused())
            .finish()
    }
}

impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new(None).unwrap_or_else(|_| Self {
            device_name: None,
            is_recording: Arc::new(AtomicBool::new(false)),
            is_paused: Arc::new(AtomicBool::new(false)),
            stream_error: Arc::new(AtomicBool::new(false)),
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: Arc::new(AtomicU32::new(16000)),
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_audio_input(
    data: &[f32],
    channels: usize,
    buffer: &Arc<Mutex<Vec<f32>>>,
    is_paused: &Arc<AtomicBool>,
    last_level_time: &Arc<Mutex<Instant>>,
    dc_state: &Arc<Mutex<Option<(f32, f32)>>>,
    start_time: Instant,
    level_tx: &tokio::sync::mpsc::Sender<[f32; 5]>,
) {
    if is_paused.load(Ordering::Relaxed) {
        let should_emit = {
            let mut last = last_level_time.lock().unwrap();
            if last.elapsed() >= Duration::from_millis(16) {
                *last = Instant::now();
                true
            } else {
                false
            }
        };
        if should_emit {
            let _ = level_tx.try_send([0.0; 5]);
        }
        return;
    }

    if data.is_empty() {
        return;
    }

    // Downmix to mono if multi-channel and apply 1st-order DC-blocking high-pass filter
    // (`y[n] = x[n] - x[n-1] + 0.995 * y[n-1]`) initialized to the first sample `x[0]`
    // so static hardware ADC DC offsets never inject a step spike at sample 0.
    let mut state_opt = dc_state.lock().ok().and_then(|g| *g);
    let mono_samples: Vec<f32> = if channels > 1 {
        let frame_count = data.len() / channels;
        let mut mono = Vec::with_capacity(frame_count);
        for frame in 0..frame_count {
            let mut sum = 0.0f32;
            for ch in 0..channels {
                sum += data[frame * channels + ch];
            }
            let x = sum / channels as f32;
            let (prev_x, prev_y) = state_opt.unwrap_or((x, 0.0));
            let y = (x - prev_x + 0.995 * prev_y).clamp(-1.0, 1.0);
            state_opt = Some((x, y));
            mono.push(y);
        }
        mono
    } else {
        let mut mono = Vec::with_capacity(data.len());
        for &x in data {
            let (prev_x, prev_y) = state_opt.unwrap_or((x, 0.0));
            let y = (x - prev_x + 0.995 * prev_y).clamp(-1.0, 1.0);
            state_opt = Some((x, y));
            mono.push(y);
        }
        mono
    };
    if let Ok(mut g) = dc_state.lock() {
        *g = state_opt;
    }

    if let Ok(mut buf) = buffer.lock() {
        buf.extend_from_slice(&mono_samples);
    }

    let should_emit = {
        let mut last = last_level_time.lock().unwrap();
        if last.elapsed() >= Duration::from_millis(16) {
            *last = Instant::now();
            true
        } else {
            false
        }
    };

    if should_emit {
        // Suppress audio device power-on transient during the first 150ms
        if start_time.elapsed() < Duration::from_millis(150) {
            let _ = level_tx.try_send([0.0; 5]);
            return;
        }

        let window_samples = if let Ok(buf) = buffer.lock() {
            if buf.len() < 512 {
                Vec::new()
            } else {
                let win_size = 2048.min(buf.len());
                buf[buf.len() - win_size..].to_vec()
            }
        } else {
            mono_samples
        };
        let levels = if window_samples.is_empty() {
            [0.0; 5]
        } else {
            crate::audio::level::calculate_5_bar_levels(&window_samples)
        };
        let _ = level_tx.try_send(levels);
    }
}
