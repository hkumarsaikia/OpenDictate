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

/// Microphone audio recorder managing hardware capture streams and real-time level metering.
pub struct AudioRecorder {
    device_name: Option<String>,
    is_recording: Arc<AtomicBool>,
    is_paused: Arc<AtomicBool>,
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: Arc<AtomicU32>,
}

/// Suppresses ALSA's default stderr error logging during device probing on Linux
/// and ensures ALSA configuration paths are valid inside Snap sandboxes.
#[cfg(target_os = "linux")]
pub fn silence_alsa_logging() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static SILENCED: AtomicBool = AtomicBool::new(false);
    if SILENCED.swap(true, Ordering::SeqCst) {
        return;
    }

    if let Ok(snap_root) = std::env::var("SNAP") {
        if std::env::var_os("ALSA_CONFIG_DIR").is_none()
            && !std::path::Path::new("/usr/share/alsa/alsa.conf").exists()
        {
            let gnome_alsa = format!("{}/gnome-platform/usr/share/alsa", snap_root);
            let snap_alsa = format!("{}/usr/share/alsa", snap_root);
            if std::path::Path::new(&format!("{}/alsa.conf", gnome_alsa)).exists() {
                unsafe {
                    std::env::set_var("ALSA_CONFIG_DIR", &gnome_alsa);
                }
            } else if std::path::Path::new(&format!("{}/alsa.conf", snap_alsa)).exists() {
                unsafe {
                    std::env::set_var("ALSA_CONFIG_DIR", &snap_alsa);
                }
            }
        }
        if std::env::var_os("ALSA_CONFIG_PATH").is_none() {
            let asound_conf = format!("{}/etc/asound.conf", snap_root);
            if std::path::Path::new(&asound_conf).exists() {
                unsafe {
                    std::env::set_var("ALSA_CONFIG_PATH", &asound_conf);
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

impl AudioRecorder {
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
                    || t.starts_with("analog-input-headphone-mic:"))
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

            if port_lower == "analog-input-headset-mic"
                || port_lower == "analog-input-headphone-mic"
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

        // Check Bluetooth cards that are connected and have headset-head-unit available
        for card_block in cards_output.split("Card #").skip(1) {
            let mut is_bt_card = false;
            let mut hfp_available = false;
            for line in card_block.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Name:") {
                    if rest.trim().starts_with("bluez_card.") {
                        is_bt_card = true;
                    }
                } else if (t.starts_with("headset-head-unit")
                    || t.starts_with("hfp_hf")
                    || t.starts_with("hsp_hs"))
                    && t.contains("available: yes")
                {
                    hfp_available = true;
                }
            }
            if is_bt_card && hfp_available {
                return "Handsfree";
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
                    || t.starts_with("analog-input-headphone-mic:"))
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

            let is_hp_source = port_lower == "analog-input-headset-mic"
                || port_lower == "analog-input-headphone-mic"
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
        let sources_out = std::process::Command::new("pactl")
            .args(["list", "sources"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

        let cards_out = std::process::Command::new("pactl")
            .args(["list", "cards"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

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
    pub fn list_friendly_input_devices() -> Vec<String> {
        let raw = Self::list_input_devices().unwrap_or_default();
        Self::filter_and_standardize_devices(&raw)
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
        if self.is_recording() {
            return Ok(());
        }

        // Always close any lingering stream first so multiple devices never capture simultaneously
        self.stream.take();

        let target_profile = self.device_name.as_deref().unwrap_or("System Default");

        // Route PulseAudio/PipeWire exclusively to the selected hardware profile's source
        let sources_out = std::process::Command::new("pactl")
            .args(["list", "sources"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();
        let cards_out = std::process::Command::new("pactl")
            .args(["list", "cards"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default();

        if target_profile == "Handsfree" {
            // If a Bluetooth card is connected in A2DP mode with headset-head-unit available, switch it
            for card_block in cards_out.split("Card #").skip(1) {
                let mut card_name = "";
                let mut hfp_available = false;
                let mut active_is_a2dp = false;
                for line in card_block.lines() {
                    let t = line.trim();
                    if let Some(rest) = t.strip_prefix("Name:") {
                        card_name = rest.trim();
                    } else if t.starts_with("headset-head-unit:") && t.contains("available: yes") {
                        hfp_available = true;
                    } else if let Some(rest) = t.strip_prefix("Active Profile:")
                        && rest.trim().starts_with("a2dp")
                    {
                        active_is_a2dp = true;
                    }
                }
                if card_name.starts_with("bluez_card.") && hfp_available && active_is_a2dp {
                    let _ = std::process::Command::new("pactl")
                        .args(["set-card-profile", card_name, "headset-head-unit"])
                        .status();
                }
            }
        }

        if let Some(exclusive_source) =
            Self::resolve_exclusive_source_from_pactl(target_profile, &sources_out, &cards_out)
        {
            if target_profile == "Headphones" && exclusive_source.starts_with("alsa_input.pci-") {
                let _ = std::process::Command::new("pactl")
                    .args([
                        "set-source-port",
                        &exclusive_source,
                        "analog-input-headset-mic",
                    ])
                    .status();
            } else if target_profile == "System Default"
                && exclusive_source.starts_with("alsa_input.pci-")
            {
                let _ = std::process::Command::new("pactl")
                    .args([
                        "set-source-port",
                        &exclusive_source,
                        "analog-input-internal-mic",
                    ])
                    .status();
            }
            unsafe {
                std::env::set_var("PULSE_SOURCE", &exclusive_source);
            }
        } else if target_profile == "System Default" || target_profile == "default" {
            unsafe {
                std::env::remove_var("PULSE_SOURCE");
            }
        }

        let host = cpal::default_host();
        let default_or_first_device = || -> Result<cpal::Device, AudioError> {
            if let Some(dev) = host.default_input_device() {
                return Ok(dev);
            }
            if let Ok(devices) = host.input_devices() {
                for dev in devices {
                    if let Ok(dev_name) = dev.name() {
                        let d_lower = dev_name.to_lowercase();
                        if !d_lower.ends_with(".monitor") && !d_lower.starts_with("iec958:") {
                            return Ok(dev);
                        }
                    }
                }
            }
            Err(AudioError::DefaultDeviceNotFound)
        };

        let device = if Self::should_use_default_pulse_device(self.device_name.as_deref()) {
            default_or_first_device()?
        } else if let Some(ref name) = self.device_name {
            let mut found = None;
            let target_lower = name.to_lowercase();
            if let Ok(devices) = host.input_devices() {
                for dev in devices {
                    if let Ok(dev_name) = dev.name() {
                        let d_lower = dev_name.to_lowercase();
                        if d_lower.ends_with(".monitor") || d_lower.starts_with("iec958:") {
                            continue;
                        }
                        if d_lower.contains(&target_lower) {
                            found = Some(dev);
                            break;
                        }
                    }
                }
            }
            match found {
                Some(dev) => dev,
                None => default_or_first_device()?,
            }
        } else {
            default_or_first_device()?
        };

        let supported_config = device.default_input_config()?;
        let sample_format = supported_config.sample_format();
        let stream_config: cpal::StreamConfig = supported_config.into();
        let native_rate = stream_config.sample_rate.0;
        let channels = stream_config.channels as usize;

        self.sample_rate.store(native_rate, Ordering::SeqCst);
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
        self.is_paused.store(false, Ordering::SeqCst);

        let buffer = Arc::clone(&self.buffer);
        let is_recording = Arc::clone(&self.is_recording);
        let is_paused = Arc::clone(&self.is_paused);
        let last_level_time = Arc::new(Mutex::new(Instant::now()));
        let stream_start_time = Instant::now();

        let stream = match sample_format {
            cpal::SampleFormat::F32 => {
                let buffer = Arc::clone(&buffer);
                let is_paused = Arc::clone(&is_paused);
                let last_level_time = Arc::clone(&last_level_time);
                let level_tx = level_tx.clone();

                device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        handle_audio_input(
                            data,
                            channels,
                            &buffer,
                            &is_paused,
                            &last_level_time,
                            stream_start_time,
                            &level_tx,
                        );
                    },
                    move |err| {
                        log::error!("Audio stream error: {}", err);
                        crate::services::crash_reporter::CrashReporter::record_event(
                            "AudioStream",
                            &format!("Hardware audio stream error: {}", err),
                        );
                    },
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                let buffer = Arc::clone(&buffer);
                let is_paused = Arc::clone(&is_paused);
                let last_level_time = Arc::clone(&last_level_time);
                let level_tx = level_tx.clone();

                device.build_input_stream(
                    &stream_config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let f32_data: Vec<f32> = data.iter().map(|&s| s as f32 / 32768.0).collect();
                        handle_audio_input(
                            &f32_data,
                            channels,
                            &buffer,
                            &is_paused,
                            &last_level_time,
                            stream_start_time,
                            &level_tx,
                        );
                    },
                    move |err| {
                        log::error!("Audio stream error: {}", err);
                        crate::services::crash_reporter::CrashReporter::record_event(
                            "AudioStream",
                            &format!("Hardware audio stream error: {}", err),
                        );
                    },
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                let buffer = Arc::clone(&buffer);
                let is_paused = Arc::clone(&is_paused);
                let last_level_time = Arc::clone(&last_level_time);
                let level_tx = level_tx.clone();

                device.build_input_stream(
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
                            stream_start_time,
                            &level_tx,
                        );
                    },
                    move |err| {
                        log::error!("Audio stream error: {}", err);
                        crate::services::crash_reporter::CrashReporter::record_event(
                            "AudioStream",
                            &format!("Hardware audio stream error: {}", err),
                        );
                    },
                    None,
                )?
            }
            cpal::SampleFormat::I32 => {
                let buffer = Arc::clone(&buffer);
                let is_paused = Arc::clone(&is_paused);
                let last_level_time = Arc::clone(&last_level_time);
                let level_tx = level_tx.clone();

                device.build_input_stream(
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
                            stream_start_time,
                            &level_tx,
                        );
                    },
                    move |err| {
                        log::error!("Audio stream error: {}", err);
                        crate::services::crash_reporter::CrashReporter::record_event(
                            "AudioStream",
                            &format!("Hardware audio stream error: {}", err),
                        );
                    },
                    None,
                )?
            }
            _ => {
                return Err(AudioError::StreamBuildError(
                    "Unsupported audio sample format".to_string(),
                ));
            }
        };

        stream.play()?;
        self.stream = Some(stream);
        is_recording.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Pause audio recording without destroying the stream. Discards samples and emits zeros.
    pub fn pause(&mut self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    /// Resume audio recording after being paused.
    pub fn resume(&mut self) {
        self.is_paused.store(false, Ordering::SeqCst);
    }

    /// Stop recording, close stream, resample to 16 kHz, and encode to 16-bit PCM WAV bytes.
    pub fn stop_recording(&mut self) -> Result<Vec<u8>, AudioError> {
        self.is_recording.store(false, Ordering::SeqCst);
        self.is_paused.store(false, Ordering::SeqCst);
        self.stream.take();

        let samples = {
            let mut buf = self.buffer.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *buf)
        };

        let native_rate = self.sample_rate.load(Ordering::SeqCst);
        let resampled = resample_to_16k(&samples, native_rate)?;
        encode_pcm_wav(&resampled, 16000)
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
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            sample_rate: Arc::new(AtomicU32::new(16000)),
        })
    }
}

fn handle_audio_input(
    data: &[f32],
    channels: usize,
    buffer: &Arc<Mutex<Vec<f32>>>,
    is_paused: &Arc<AtomicBool>,
    last_level_time: &Arc<Mutex<Instant>>,
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

    // Downmix to mono if multi-channel
    let mono_samples: Vec<f32> = if channels > 1 {
        let frame_count = data.len() / channels;
        let mut mono = Vec::with_capacity(frame_count);
        for frame in 0..frame_count {
            let mut sum = 0.0f32;
            for ch in 0..channels {
                sum += data[frame * channels + ch];
            }
            mono.push(sum / channels as f32);
        }
        mono
    } else {
        data.to_vec()
    };

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
