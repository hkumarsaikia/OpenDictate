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

            // Ensure ALSA_CONFIG_DIR and ALSA_CONFIG_PATH include alsa.conf + 50-pulseaudio.conf + asound.conf
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
                && (env_key == "SNAP" || !std::path::Path::new("/usr/share/alsa/alsa.conf").exists())
            {
                unsafe {
                    std::env::set_var("ALSA_CONFIG_DIR", dir);
                }
                let mut config_files = vec![format!("{}/alsa.conf", dir)];
                for extra in [
                    format!("{}/alsa.conf.d/50-pulseaudio.conf", dir),
                    format!("{}/usr/share/alsa/alsa.conf.d/50-pulseaudio.conf", root),
                    format!("{}/etc/asound.conf", root),
                ] {
                    if std::path::Path::new(&extra).exists() && !config_files.contains(&extra) {
                        config_files.push(extra);
                    }
                }
                unsafe {
                    std::env::set_var("ALSA_CONFIG_PATH", config_files.join(":"));
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

fn resolve_pactl_binary() -> String {
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
            } else if t.contains("device.bus = \"bluetooth\"")
                || t.contains("device.api = \"bluez")
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
        }
        None
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

        // Check Bluetooth cards that are connected and have an HFP/HSP/duplex input profile available
        for card_block in cards_output.split("Card #").skip(1) {
            if resolve_bluetooth_input_profile_switch(card_block).is_some() {
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
        silence_alsa_logging();
        if self.is_recording() {
            return Ok(());
        }

        // Always close any lingering stream first so multiple devices never capture simultaneously
        self.stream.take();

        let target_profile = self.device_name.as_deref().unwrap_or("System Default");

        // Route PulseAudio/PipeWire exclusively to the selected hardware profile's source
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

        let mut pulse_source_was_set = false;
        if let Some(exclusive_source) =
            Self::resolve_exclusive_source_from_pactl(target_profile, &sources_out, &cards_out)
        {
            if let Some(port_id) =
                resolve_available_port_for_source(&sources_out, &exclusive_source, target_profile)
            {
                run_pactl_status(&["set-source-port", &exclusive_source, &port_id]);
            }
            unsafe {
                std::env::set_var("PULSE_SOURCE", &exclusive_source);
            }
            pulse_source_was_set = true;
        } else if target_profile == "System Default" || target_profile == "default" {
            unsafe {
                std::env::remove_var("PULSE_SOURCE");
            }
        }

        if let Ok( mut buf) = self.buffer.lock() {
            buf.clear();
        }
        self.is_paused.store(false, Ordering::SeqCst);

        let collect_candidate_devices = |host: &cpal::Host| -> Vec<cpal::Device> {
            let mut candidates: Vec<cpal::Device> = Vec::new();
            let mut push_unique = |dev: cpal::Device| {
                let name = dev.name().unwrap_or_default();
                let lower = name.trim().to_lowercase();
                if lower == "null"
                    || lower.ends_with(".monitor")
                    || lower.starts_with("iec958:")
                    || lower.starts_with("hdmi:")
                    || lower.starts_with("surround")
                {
                    return;
                }
                if !candidates
                    .iter()
                    .any(|c| c.name().ok().as_deref() == Some(name.as_str()))
                {
                    candidates.push(dev);
                }
            };

            if !Self::should_use_default_pulse_device(self.device_name.as_deref())
                && let Some(ref req_name) = self.device_name
            {
                let req_lower = req_name.to_lowercase();
                if let Ok(devices) = host.input_devices() {
                    for dev in devices {
                        if let Ok(n) = dev.name()
                            && n.to_lowercase().contains(&req_lower)
                        {
                            push_unique(dev);
                        }
                    }
                }
            }

            if let Some(def_dev) = host.default_input_device() {
                push_unique(def_dev);
            }

            if let Ok(devices) = host.input_devices() {
                let all_devs: Vec<cpal::Device> = devices.collect();
                for preferred in ["pulse", "pipewire", "default", "sysdefault"] {
                    for dev in &all_devs {
                        if let Ok(n) = dev.name()
                            && n.trim().eq_ignore_ascii_case(preferred)
                        {
                            push_unique(dev.clone());
                        }
                    }
                }
                for dev in all_devs {
                    push_unique(dev);
                }
            }
            candidates
        };

        let try_open_on_device = |device: &cpal::Device| -> Result<(cpal::Stream, u32), AudioError> {
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
            let stream_start_time = Instant::now();
            let level_tx = level_tx.clone();

            let err_fn = |err: cpal::StreamError| {
                log::error!("Audio stream error: {}", err);
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
                        let f32_data: Vec<f32> = data
                            .iter()
                            .map(|&s| (s as f32 - 128.0) / 128.0)
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

        let host = cpal::default_host();
        let candidates = collect_candidate_devices(&host);
        if candidates.is_empty() {
            return Err(AudioError::DefaultDeviceNotFound);
        }

        let mut last_err = AudioError::DefaultDeviceNotFound;
        let mut opened: Option<(cpal::Stream, u32)> = None;

        for dev in &candidates {
            match try_open_on_device(dev) {
                Ok(res) => {
                    opened = Some(res);
                    break;
                }
                Err(e) => {
                    last_err = e;
                }
            }
        }

        // If a specific PULSE_SOURCE failed to open (e.g. unplugged or rejected stream),
        // automatically clear PULSE_SOURCE so PulseAudio/PipeWire uses @DEFAULT_SOURCE@ and retry.
        if opened.is_none() && pulse_source_was_set {
            unsafe {
                std::env::remove_var("PULSE_SOURCE");
            }
            for dev in &candidates {
                if let Ok(res) = try_open_on_device(dev) {
                    opened = Some(res);
                    break;
                }
            }
        }

        let (stream, native_rate) = match opened {
            Some(res) => res,
            None => return Err(last_err),
        };

        self.sample_rate.store(native_rate, Ordering::SeqCst);
        self.stream = Some(stream);
        self.is_recording.store(true, Ordering::SeqCst);
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
