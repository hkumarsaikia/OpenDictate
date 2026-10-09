use opendictate::audio::level::{LevelSmoother, calculate_5_bar_levels, calculate_rms};
use opendictate::audio::recorder::{AudioRecorder, encode_pcm_wav};

#[test]
fn test_calculate_rms_silence_and_signals() {
    // Empty slice returns 0.0
    assert_eq!(calculate_rms(&[]), 0.0);

    // Silence (zeros) returns 0.0
    let silence = vec![0.0f32; 1000];
    assert_eq!(calculate_rms(&silence), 0.0);

    // Constant DC bias signal of amplitude 0.5 has zero AC acoustic energy (evaluates to 0.0)
    let dc = vec![0.5f32; 1000];
    let rms_dc = calculate_rms(&dc);
    assert!(
        rms_dc < 1e-5,
        "Constant DC bias must yield 0.0 acoustic RMS"
    );

    // Sine wave: amplitude 1.0, RMS should be approx 1 / sqrt(2) ≈ 0.7071
    let sample_rate = 16000.0f32;
    let freq = 440.0f32;
    let sine: Vec<f32> = (0..16000)
        .map(|i| (2.0 * std::f32::consts::PI * freq * (i as f32) / sample_rate).sin())
        .collect();
    let rms_sine = calculate_rms(&sine);
    let expected = 1.0f32 / std::f32::consts::SQRT_2;
    assert!((rms_sine - expected).abs() < 0.01);
}

#[test]
fn test_audio_level_calculation_bounds_and_variation() {
    // Silence gives 0.0 across all bars
    let silence = vec![0.0f32; 1600];
    let zero_levels = calculate_5_bar_levels(&silence);
    assert_eq!(zero_levels.len(), 5);
    for &lvl in &zero_levels {
        assert_eq!(lvl, 0.0);
    }

    // Active oscillating audio (sine wave 440 Hz at amplitude 0.20) gives values within [0.0, 1.0]
    let samples: Vec<f32> = (0..1600)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / 16000.0).sin() * 0.20)
        .collect();
    let levels = calculate_5_bar_levels(&samples);
    assert_eq!(levels.len(), 5);
    for &lvl in &levels {
        assert!((0.0..=1.0).contains(&lvl), "Level out of bounds: {}", lvl);
    }

    // Verify non-zero levels exhibit organic variation (not all 5 values identical)
    let min_lvl = levels.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_lvl = levels.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    assert!(max_lvl > min_lvl, "Levels should vary across 5 bars");
}

#[test]
fn test_audio_noise_gate_filters_ambient_noise() {
    // Ambient quiet room noise (~ -50 dB, AC amplitude 0.003)
    let ambient: Vec<f32> = (0..1600)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / 16000.0).sin() * 0.003)
        .collect();
    let levels = calculate_5_bar_levels(&ambient);
    for &lvl in &levels {
        assert_eq!(lvl, 0.0, "Ambient noise below noise gate should return 0.0");
    }

    // Mic line hiss / fan background noise (~ -34 dB, AC amplitude 0.020)
    let hiss: Vec<f32> = (0..1600)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / 16000.0).sin() * 0.020)
        .collect();
    let hiss_levels = calculate_5_bar_levels(&hiss);
    for &lvl in &hiss_levels {
        assert_eq!(lvl, 0.0, "Mic hiss below 0.030 threshold should return 0.0");
    }

    // Real voice (~ -16 dB, AC amplitude 0.22)
    let voice: Vec<f32> = (0..1600)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / 16000.0).sin() * 0.22)
        .collect();
    let voice_levels = calculate_5_bar_levels(&voice);
    let max_voice = voice_levels.iter().cloned().fold(0.0f32, f32::max);
    assert!(
        max_voice > 0.15,
        "Real voice must pass noise gate and produce active levels"
    );
}

#[test]
fn test_level_smoother_transitions() {
    let mut smoother = LevelSmoother::new(0.7);
    assert_eq!(smoother.current(), [0.0; 5]);

    // Step towards target 1.0
    let target = [1.0; 5];
    let step1 = smoother.update(target);
    for &val in &step1 {
        // With decay 0.7: val should be around 0.3 (target*(1-0.7) + prev*0.7)
        assert!(
            val > 0.0 && val < 1.0,
            "Step 1 should smoothly transition: {}",
            val
        );
    }

    // Second step should be higher
    let step2 = smoother.update(target);
    for i in 0..5 {
        assert!(step2[i] > step1[i], "Step 2 should be higher than Step 1");
        assert!(step2[i] <= 1.0);
    }

    // Smoothing towards 0.0 should decay smoothly
    let zero_target = [0.0; 5];
    let decay_step = smoother.update(zero_target);
    for i in 0..5 {
        assert!(decay_step[i] < step2[i], "Decay step should decrease");
        assert!(decay_step[i] >= 0.0);
    }

    // Reset sets back to zero
    smoother.reset();
    assert_eq!(smoother.current(), [0.0; 5]);
}

#[test]
fn test_wav_in_memory_encoding() {
    let samples = vec![0.0f32; 8000]; // 0.5s silence at 16kHz
    let wav_bytes = encode_pcm_wav(&samples, 16000).expect("failed to encode PCM WAV");

    assert!(wav_bytes.starts_with(b"RIFF"));
    assert_eq!(&wav_bytes[8..12], b"WAVE");

    // Read back with hound to verify standard 16-bit mono PCM structure
    let cursor = std::io::Cursor::new(wav_bytes);
    let reader = hound::WavReader::new(cursor).expect("hound failed to parse encoded WAV");
    let spec = reader.spec();
    assert_eq!(spec.channels, 1);
    assert_eq!(spec.sample_rate, 16000);
    assert_eq!(spec.bits_per_sample, 16);
    assert_eq!(spec.sample_format, hound::SampleFormat::Int);
    assert_eq!(reader.duration(), 8000);
}

#[test]
fn test_wav_in_memory_encoding_sample_values() {
    // Test with non-zero samples and verify amplitude conversion
    let samples = vec![0.5f32, -0.5f32, 1.0f32, -1.0f32];
    let wav_bytes = encode_pcm_wav(&samples, 16000).expect("failed to encode");
    let cursor = std::io::Cursor::new(wav_bytes);
    let mut reader = hound::WavReader::new(cursor).expect("failed to parse");
    let decoded_samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    assert_eq!(decoded_samples.len(), 4);
    // 0.5 * 32767 ≈ 16383
    assert!((decoded_samples[0] - 16383).abs() <= 2);
    // -0.5 * 32767 ≈ -16383
    assert!((decoded_samples[1] - (-16383)).abs() <= 2);
    // 1.0 clamped/scaled to 32767
    assert_eq!(decoded_samples[2], 32767);
    // -1.0 clamped/scaled to -32768 or -32767
    assert!(decoded_samples[3] <= -32767);
}

#[test]
fn test_audio_recorder_list_input_devices() {
    // Should execute on Linux host without panicking or erroring
    let devices_result = AudioRecorder::list_input_devices();
    assert!(
        devices_result.is_ok(),
        "list_input_devices failed: {:?}",
        devices_result.err()
    );
    let devices = devices_result.unwrap();
    println!("Available audio input devices: {:?}", devices);
}

#[test]
fn test_audio_recorder_lifecycle_states() {
    let mut recorder = AudioRecorder::new(None).expect("failed to create AudioRecorder");
    assert!(!recorder.is_recording());
    assert!(!recorder.is_paused());

    recorder.pause();
    assert!(recorder.is_paused());

    recorder.resume();
    assert!(!recorder.is_paused());

    // Stopping when not recording returns empty/valid WAV bytes
    let stopped_wav = recorder
        .stop_recording()
        .expect("stop_recording should succeed");
    assert!(stopped_wav.starts_with(b"RIFF"));
    assert!(!recorder.is_recording());
}

#[test]
fn test_resample_to_16k_from_48k() {
    use opendictate::audio::recorder::resample_to_16k;

    // 0.5s at 48000 Hz = 24000 samples
    let freq = 440.0f32;
    let samples_48k: Vec<f32> = (0..24000)
        .map(|i| (2.0 * std::f32::consts::PI * freq * (i as f32) / 48000.0).sin())
        .collect();

    let resampled = resample_to_16k(&samples_48k, 48000).expect("resampling failed");
    // Expected output duration is 0.5s at 16000 Hz = 8000 samples (± a few samples for filter delay/chunking)
    assert!(
        (resampled.len() as i32 - 8000).abs() < 50,
        "Expected ~8000 samples, got {}",
        resampled.len()
    );

    // Verify signal is non-silent
    let rms = opendictate::audio::level::calculate_rms(&resampled);
    assert!((rms - (1.0 / std::f32::consts::SQRT_2)).abs() < 0.1);
}

#[test]
fn test_resample_to_16k_identity_and_empty() {
    use opendictate::audio::recorder::resample_to_16k;

    // Empty
    let empty = resample_to_16k(&[], 44100).expect("empty resample failed");
    assert!(empty.is_empty());

    // Identity (already 16k)
    let samples = vec![0.1f32, 0.2, 0.3, 0.4];
    let same = resample_to_16k(&samples, 16000).expect("identity failed");
    assert_eq!(same, samples);

    // Short slice (<1024)
    let short_44k = vec![0.5f32; 441]; // 10ms at 44.1kHz -> ~160 samples at 16kHz
    let short_16k = resample_to_16k(&short_44k, 44100).expect("short resample failed");
    assert_eq!(short_16k.len(), 160);
}

#[tokio::test]
async fn test_real_device_recording_lifecycle() {
    let mut recorder = AudioRecorder::new(None).expect("failed to create recorder");
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    match recorder.start_recording(tx) {
        Ok(()) => {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            let mut got_level = false;
            while let Ok(_level) = rx.try_recv() {
                got_level = true;
            }
            let _ = got_level;
            let wav = recorder
                .stop_recording()
                .expect("stop_recording should succeed");
            assert!(!wav.is_empty());
        }
        Err(e) => {
            println!(
                "Skipping real recording test (no input device available): {}",
                e
            );
        }
    }
}

#[test]
fn test_friendly_audio_devices_filtering() {
    let raw = vec![
        "default".to_string(),
        "dsnoop:CARD=Generic_1,DEV=0".to_string(),
        "front:CARD=Generic_1,DEV=0".to_string(),
        "hw:CARD=Generic_1,DEV=0".to_string(),
        "pipewire".to_string(),
        "plughw:CARD=Generic_1,DEV=0".to_string(),
        "surround40:CARD=Generic_1,DEV=0".to_string(),
        "surround51:CARD=Generic_1,DEV=0".to_string(),
        "surround71:CARD=Generic_1,DEV=0".to_string(),
        "sysdefault:CARD=Generic_1".to_string(),
        "Some Random Extra USB Device".to_string(),
    ];

    let filtered = AudioRecorder::filter_and_standardize_devices(&raw);
    assert_eq!(
        filtered,
        vec![
            "System Default".to_string(),
            "Headphones".to_string(),
            "Handsfree".to_string(),
        ],
        "Microphone options must strictly be System Default, Headphones, and Handsfree"
    );
}

#[test]
fn test_audio_profile_detection_from_hardware_snapshot() {
    // 1. Bluetooth earphone/headset active source -> "Handsfree"
    let bt_sources = r#"
Source #54
	State: SUSPENDED
	Name: alsa_output.pci-0000_00_1f.3.analog-stereo.monitor
	Description: Monitor of Built-in Audio Analog Stereo
Source #88
	State: RUNNING
	Name: bluez_input.AA_BB_CC_DD_EE_FF.0
	Description: Galaxy Buds2 Pro
	Properties:
		device.api = "bluez5"
		device.bus = "bluetooth"
	Ports:
		headset-head-unit: Handsfree (type: Headset, priority: 0, available)
	Active Port: headset-head-unit
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(bt_sources, "", &[]),
        "Handsfree"
    );

    // 2. Bluetooth card connected with headset-head-unit available -> "Handsfree"
    let bt_cards = r#"
Card #42
	Name: bluez_card.11_22_33_44_55_66
	Driver: module-bluez5-device.c
	Profiles:
		a2dp-sink: High Fidelity Playback (A2DP Sink) (sinks: 1, sources: 0, priority: 40, available: yes)
		headset-head-unit: Headset Head Unit (HSP/HFP) (sinks: 1, sources: 1, priority: 30, available: yes)
	Active Profile: a2dp-sink
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards("", bt_cards, &[]),
        "Handsfree"
    );

    // 3. Wired 3.5mm TRRS headset plugged into combo jack -> "Headphones"
    let wired_sources = r#"
Source #45
	State: RUNNING
	Name: alsa_input.pci-0000_00_1f.3.analog-stereo
	Description: Built-in Audio Analog Stereo
	Ports:
		analog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, available)
		analog-input-headset-mic: Headset Microphone (type: Headset, priority: 8700, available)
	Active Port: analog-input-headset-mic
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(wired_sources, "", &[]),
        "Headphones"
    );

    // 4. USB Headset plugged in -> "Headphones"
    let usb_headset_sources = r#"
Source #61
	State: SUSPENDED
	Name: alsa_input.usb-Logitech_Logitech_USB_Headset-00.mono-fallback
	Description: Logitech USB Headset Mono
	Properties:
		device.bus = "usb"
		device.form_factor = "headset"
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(
            usb_headset_sources,
            "",
            &[]
        ),
        "Headphones"
    );

    // 5. Internal microphone only (headset port is "not available") + Bluetooth monitor sink ignored -> "System Default"
    let internal_only_sources = r#"
Source #44
	State: SUSPENDED
	Name: bluez_output.AA_BB_CC_DD_EE_FF.1.monitor
	Description: Monitor of Bluetooth Speaker
Source #45
	State: SUSPENDED
	Name: alsa_input.pci-0000_00_1f.3.analog-stereo
	Description: Built-in Audio Analog Stereo
	Ports:
		analog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, available)
		analog-input-headset-mic: Headset Microphone (type: Headset, priority: 8700, not available)
	Active Port: analog-input-internal-mic
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(
            internal_only_sources,
            "",
            &[]
        ),
        "System Default"
    );
}

#[test]
fn test_exclusive_single_device_routing_and_sample_rates() {
    use opendictate::audio::recorder::resample_to_16k;

    let mixed_sources = r#"
Source #44
	Name: alsa_output.pci-0000_00_1f.3.analog-stereo.monitor
Source #45
	Name: alsa_input.pci-0000_00_1f.3.analog-stereo
	Ports:
		analog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, available)
		analog-input-headset-mic: Headset Microphone (type: Headset, priority: 8700, available)
	Active Port: analog-input-headset-mic
Source #88
	Name: bluez_input.AA_BB_CC_DD_EE_FF.0
	Ports:
		headset-head-unit: Handsfree (type: Headset, priority: 0, available)
	Active Port: headset-head-unit
"#;

    // Handsfree resolves exclusively to the single Bluetooth input source (never monitor, never multiple)
    let hf_target =
        AudioRecorder::resolve_exclusive_source_from_pactl("Handsfree", mixed_sources, "");
    assert_eq!(
        hf_target.as_deref(),
        Some("bluez_input.AA_BB_CC_DD_EE_FF.0")
    );

    // Headphones resolves exclusively to the wired headset source
    let hp_target =
        AudioRecorder::resolve_exclusive_source_from_pactl("Headphones", mixed_sources, "");
    assert_eq!(
        hp_target.as_deref(),
        Some("alsa_input.pci-0000_00_1f.3.analog-stereo")
    );

    // Verify resampling across all hardware sample rates: 8kHz (Bluetooth HFP), 16kHz (mSBC), 44.1kHz, 48kHz, 96kHz
    for &rate in &[8_000u32, 16_000, 44_100, 48_000, 96_000] {
        let half_sec_samples: Vec<f32> = (0..(rate / 2))
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / (rate as f32)).sin() * 0.4)
            .collect();
        let resampled = resample_to_16k(&half_sec_samples, rate)
            .unwrap_or_else(|e| panic!("Failed to resample from {} Hz: {}", rate, e));
        assert!(
            (resampled.len() as i32 - 8000).abs() < 80,
            "Resampled length from {} Hz should be ~8000 samples (0.5s at 16kHz), got {}",
            rate,
            resampled.len()
        );
    }
}

#[test]
fn test_should_use_default_pulse_device_for_standard_profiles() {
    // Standard hardware profiles ("System Default", "Headphones", "Handsfree") are routed via
    // PULSE_SOURCE in PipeWire/PulseAudio and must open the default ALSA device (`default`)
    // rather than grabbing a raw `hw:` or `iec958:` ALSA subdevice locked by PipeWire.
    assert!(AudioRecorder::should_use_default_pulse_device(None));
    assert!(AudioRecorder::should_use_default_pulse_device(Some(
        "System Default"
    )));
    assert!(AudioRecorder::should_use_default_pulse_device(Some(
        "default"
    )));
    assert!(AudioRecorder::should_use_default_pulse_device(Some(
        "Headphones"
    )));
    assert!(AudioRecorder::should_use_default_pulse_device(Some(
        "Handsfree"
    )));
    assert!(!AudioRecorder::should_use_default_pulse_device(Some(
        "plughw:CARD=Mic,DEV=0"
    )));
}

#[test]
fn test_multi_hardware_architecture_port_and_profile_resolution() {
    // 1. Intel Sound Open Firmware (sof-hda-dsp) with UCM2 Mic1 (internal DMIC) & Mic2 (headset jack)
    let intel_sof_sources = r#"
Source #51
	State: SUSPENDED
	Name: alsa_input.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp_6__source
	Description: Sof-hda-dsp Digital Microphone
	Ports:
		[Out] Mic1: Digital Microphone (type: Mic, priority: 100, available)
Source #52
	State: RUNNING
	Name: alsa_input.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp__source
	Description: Sof-hda-dsp Headphones Stereo Microphone
	Ports:
		[Out] Mic2: Headphones Stereo Microphone (type: Headset, priority: 200, available)
	Active Port: [Out] Mic2
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(intel_sof_sources, "", &[]),
        "Headphones"
    );
    assert_eq!(
        AudioRecorder::resolve_exclusive_source_from_pactl("Headphones", intel_sof_sources, "")
            .as_deref(),
        Some("alsa_input.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp__source")
    );
    assert_eq!(
        AudioRecorder::resolve_available_port_for_source(
            intel_sof_sources,
            "alsa_input.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp__source",
            "Headphones"
        )
        .as_deref(),
        Some("[Out] Mic2")
    );
    assert_eq!(
        AudioRecorder::resolve_available_port_for_source(
            intel_sof_sources,
            "alsa_input.pci-0000_00_1f.3-platform-skl_hda_dsp_generic.HiFi__hw_sofhdadsp_6__source",
            "System Default"
        )
        .as_deref(),
        Some("[Out] Mic1")
    );

    // 2. Apple Silicon (Asahi Linux) platform-sound with internal mic & 3.5mm combo jack
    let asahi_sources = r#"
Source #33
	State: SUSPENDED
	Name: alsa_input.platform-sound.HiFi__Mic__source
	Description: MacBook Pro J314 Built-in Microphone
	Ports:
		[In] Mic: Internal Microphone (type: Mic, priority: 100, available)
		[In] Jack: Headset Microphone (type: Headset, priority: 200, not available)
	Active Port: [In] Mic
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(asahi_sources, "", &[]),
        "System Default"
    );
    assert_eq!(
        AudioRecorder::resolve_exclusive_source_from_pactl(
            "System Default",
            asahi_sources,
            "alsa_input.platform-sound.HiFi__Mic__source"
        )
        .as_deref(),
        Some("alsa_input.platform-sound.HiFi__Mic__source")
    );
    assert_eq!(
        AudioRecorder::resolve_available_port_for_source(
            asahi_sources,
            "alsa_input.platform-sound.HiFi__Mic__source",
            "System Default"
        )
        .as_deref(),
        Some("[In] Mic")
    );

    // 3. Bluetooth LE Audio (bap-duplex) & mSBC/CVSD dynamic profile discovery
    let bt_le_cards = r#"
Card #90
	Name: bluez_card.AA_BB_CC_11_22_33
	Driver: module-bluez5-device.c
	Profiles:
		bap-sink: High Fidelity Playback (LE Audio Sink) (sinks: 1, sources: 0, priority: 40, available: yes)
		bap-duplex: High Fidelity Duplex (LE Audio BAP) (sinks: 1, sources: 1, priority: 35, available: yes)
		off: Off (sinks: 0, sources: 0, priority: 0, available: yes)
	Active Profile: bap-sink
"#;
    assert_eq!(
        AudioRecorder::resolve_bluetooth_input_profile_switch(bt_le_cards),
        Some((
            "bluez_card.AA_BB_CC_11_22_33".to_string(),
            "bap-duplex".to_string()
        ))
    );

    let bt_cvsd_only_cards = r#"
Card #91
	Name: bluez_card.44_55_66_77_88_99
	Driver: module-bluez5-device.c
	Profiles:
		a2dp-sink: High Fidelity Playback (A2DP Sink) (sinks: 1, sources: 0, priority: 40, available: yes)
		headset-head-unit: Headset Head Unit (HSP/HFP, codec mSBC) (sinks: 1, sources: 1, priority: 30, available: no)
		headset-head-unit-cvsd: Headset Head Unit (HSP/HFP, codec CVSD) (sinks: 1, sources: 1, priority: 20, available: yes)
	Active Profile: a2dp-sink
"#;
    assert_eq!(
        AudioRecorder::resolve_bluetooth_input_profile_switch(bt_cvsd_only_cards),
        Some((
            "bluez_card.44_55_66_77_88_99".to_string(),
            "headset-head-unit-cvsd".to_string()
        ))
    );
}

#[test]
fn test_normalize_speech_samples_and_clean_whisper_segment() {
    use opendictate::audio::recorder::normalize_speech_samples;
    use opendictate::services::ai::local_ai::clean_whisper_segment;

    // 1. Quiet speech signal with DC offset is centered and boosted toward 0.80 peak
    let quiet_with_dc: Vec<f32> = (0..1600)
        .map(|i| 0.45 + 0.05 * ((i as f32) * 0.1).sin())
        .collect();
    let normalized = normalize_speech_samples(&quiet_with_dc);
    let mean = normalized.iter().copied().sum::<f32>() / normalized.len() as f32;
    let peak = normalized.iter().fold(0.0_f32, |acc, &x| acc.max(x.abs()));
    assert!(mean.abs() < 0.01, "Mean should be near zero, got {}", mean);
    assert!(
        peak > 0.50 && peak <= 0.85,
        "Peak should be boosted into optimal range, got {}",
        peak
    );

    // 2. Whisper non-speech tokens are stripped while real speech is preserved
    assert_eq!(clean_whisper_segment("[BLANK_AUDIO]"), "");
    assert_eq!(clean_whisper_segment(" (blank audio) "), "");
    assert_eq!(clean_whisper_segment("[ Silence ]"), "");
    assert_eq!(
        clean_whisper_segment("Hello world [BLANK_AUDIO] testing live dictation"),
        "Hello world testing live dictation"
    );
}

#[test]
fn test_snap_audio_capture_and_combo_jack_regression() {
    use opendictate::audio::recorder::normalize_speech_samples;

    // 1. Realtek ALC257 / Lenovo combo jack: when 3.5mm headphones are plugged in
    // (`analog-output-headphones: ... available)`), profile is classified as "Headphones".
    // When `analog-input-mic` is "not available" (TRS headphones), `resolve_available_port_for_source`
    // falls back to `analog-input-internal-mic`; when `analog-input-mic` is available (TRRS headset),
    // it selects `analog-input-mic`.
    let alc257_trs_sources = r#"
Source #59
	State: SUSPENDED
	Name: alsa_input.pci-0000_05_00.6.analog-stereo
	Description: Ryzen HD Audio Controller Analog Stereo
	Ports:
		analog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, availability group: Legacy 1, availability unknown)
		analog-input-mic: Microphone (type: Mic, priority: 8700, availability group: Legacy 2, not available)
	Active Port: analog-input-internal-mic
"#;
    let alc257_cards_with_hp = r#"
Card #51
	Name: alsa_card.pci-0000_05_00.6
	Ports:
		analog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, availability unknown)
		analog-input-mic: Microphone (type: Mic, priority: 8700, not available)
		analog-output-speaker: Speakers (type: Speaker, priority: 10000, not available)
		analog-output-headphones: Headphones (type: Headphones, priority: 9900, available)
"#;
    assert_eq!(
        AudioRecorder::classify_hardware_profile_from_sources_and_cards(
            alc257_trs_sources,
            alc257_cards_with_hp,
            &[]
        ),
        "Headphones"
    );
    assert_eq!(
        AudioRecorder::resolve_available_port_for_source(
            alc257_trs_sources,
            "alsa_input.pci-0000_05_00.6.analog-stereo",
            "Headphones"
        )
        .as_deref(),
        Some("analog-input-internal-mic")
    );

    let alc257_trrs_sources = alc257_trs_sources.replace("not available", "available");
    assert_eq!(
        AudioRecorder::resolve_available_port_for_source(
            &alc257_trrs_sources,
            "alsa_input.pci-0000_05_00.6.analog-stereo",
            "Headphones"
        )
        .as_deref(),
        Some("analog-input-mic")
    );

    // 2. Wake-up pop during the first 40ms (640 samples at 16kHz) must not prevent
    // speech normalization on the rest of a 0.5s (8000-sample) recording.
    let mut pop_then_quiet_speech = vec![0.0f32; 8000];
    for s in &mut pop_then_quiet_speech[..320] {
        *s = 0.99; // Simulated hardware wake-up rail pop in first 20ms
    }
    for (i, s) in pop_then_quiet_speech[1280..].iter_mut().enumerate() {
        *s = 0.04 * ((i as f32) * 0.15).sin(); // Quiet voice after 80ms
    }
    let norm = normalize_speech_samples(&pop_then_quiet_speech);
    let tail_peak = norm[1280..].iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
    assert!(
        tail_peak > 0.35,
        "Speech after hardware wake-up pop must still be normalized, got tail_peak={}",
        tail_peak
    );

    // 3. Verify live AudioRecorder start/stop works cleanly without locking PipeWire (EBUSY)
    for profile in ["System Default", "Headphones"] {
        let mut rec = AudioRecorder::new(Some(profile.to_string())).unwrap();
        let (tx, _rx) = tokio::sync::mpsc::channel(32);
        if rec.start_recording(tx).is_ok() {
            std::thread::sleep(std::time::Duration::from_millis(220));
            let wav = rec.stop_recording().expect("stop_recording should succeed");
            assert!(wav.len() > 44, "Recorded WAV must contain audio frames");
        }
    }
}
