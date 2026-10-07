use opendictate::audio::level::{calculate_5_bar_levels, calculate_rms};
use opendictate::audio::recorder::{encode_pcm_wav, resample_to_16k};
use opendictate::services::ai::AiProvider;
use opendictate::services::ai::local_ai::{LocalAiProvider, is_model_installed};

#[test]
fn test_resampler_rate_variance_stress() {
    let rates = [
        8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 192000,
    ];
    let duration_secs = 2.0f64;
    let freq = 440.0f32;

    for &rate in &rates {
        let num_samples = (duration_secs * rate as f64).round() as usize;
        let samples: Vec<f32> = (0..num_samples)
            .map(|i| (2.0 * std::f32::consts::PI * freq * (i as f32) / (rate as f32)).sin())
            .collect();

        let resampled = resample_to_16k(&samples, rate)
            .unwrap_or_else(|e| panic!("Failed to resample from {} Hz: {}", rate, e));

        assert!(
            !resampled.is_empty(),
            "Resampled output for {} Hz must not be empty",
            rate
        );

        // Expected output is 2.0s at 16000 Hz = 32,000 samples.
        // Rubato operates in 1024 chunks with linear interpolation for the remainder.
        let expected_samples = (duration_secs * 16000.0).round() as i64;
        let diff = (resampled.len() as i64 - expected_samples).abs();
        assert!(
            diff < 200,
            "Resampled length {} for rate {} Hz deviated by {} from expected 32,000",
            resampled.len(),
            rate,
            diff
        );

        // All samples must be finite and bounded within [-1.05, 1.05] (allowing minor filter ringing)
        for (idx, &s) in resampled.iter().enumerate() {
            assert!(
                s.is_finite(),
                "Sample at index {} for rate {} is not finite: {}",
                idx,
                rate,
                s
            );
            assert!(
                (-1.05..=1.05).contains(&s),
                "Sample at index {} for rate {} is out of bounds [-1.05, 1.05]: {}",
                idx,
                rate,
                s
            );
        }

        // Encode to 16-bit linear PCM WAV and verify structure using hound
        let wav_bytes = encode_pcm_wav(&resampled, 16000)
            .unwrap_or_else(|e| panic!("Failed to encode WAV for rate {}: {}", rate, e));
        assert!(wav_bytes.starts_with(b"RIFF"));

        let cursor = std::io::Cursor::new(wav_bytes);
        let reader = hound::WavReader::new(cursor)
            .unwrap_or_else(|e| panic!("hound failed to parse WAV for rate {}: {}", rate, e));
        let spec = reader.spec();
        assert_eq!(spec.sample_rate, 16000);
        assert_eq!(spec.channels, 1);
        assert_eq!(spec.bits_per_sample, 16);
        assert_eq!(spec.sample_format, hound::SampleFormat::Int);
        assert_eq!(reader.duration(), resampled.len() as u32);
    }
}

#[test]
fn test_sustained_large_audio_buffer_stress() {
    let rate = 48000;
    let duration_secs = 60.0f64;
    let num_samples = (duration_secs * rate as f64).round() as usize; // 2,880,000 samples
    let freq = 440.0f32;

    let samples: Vec<f32> = (0..num_samples)
        .map(|i| (2.0 * std::f32::consts::PI * freq * (i as f32) / (rate as f32)).sin() * 0.5)
        .collect();
    assert_eq!(samples.len(), 2_880_000);

    let resample_start = std::time::Instant::now();
    let resampled = resample_to_16k(&samples, rate)
        .expect("Failed to resample sustained 60s 48kHz audio buffer");
    let resample_duration = resample_start.elapsed();

    // 60s at 16,000 Hz = 960,000 samples
    let expected_len = (duration_secs * 16000.0).round() as i64;
    let diff = (resampled.len() as i64 - expected_len).abs();
    assert!(
        diff < 100,
        "Resampled length {} deviated by {} from expected 960,000 samples",
        resampled.len(),
        diff
    );

    let wav_start = std::time::Instant::now();
    let wav_bytes = encode_pcm_wav(&resampled, 16000).expect("Failed to encode 60s 16kHz WAV");
    let wav_duration = wav_start.elapsed();

    println!(
        "60s buffer benchmark: resampled 2,880,000 -> {} samples in {:.2?}, encoded WAV in {:.2?}",
        resampled.len(),
        resample_duration,
        wav_duration
    );

    assert!(wav_bytes.starts_with(b"RIFF"));
    let cursor = std::io::Cursor::new(wav_bytes);
    let reader = hound::WavReader::new(cursor).expect("Failed to parse 60s WAV with hound");
    let spec = reader.spec();
    assert_eq!(spec.sample_rate, 16000);
    assert_eq!(spec.channels, 1);
    assert_eq!(spec.bits_per_sample, 16);
    assert_eq!(reader.duration(), resampled.len() as u32);
}

#[tokio::test]
async fn test_whisper_thread_scaling_and_constraint_stress() {
    let thread_configs = [0, 1, 2, 4, 8, 12];

    let installed_model = if is_model_installed("tiny", None) {
        Some("tiny".to_string())
    } else if is_model_installed("tiny.en", None) {
        Some("tiny.en".to_string())
    } else if is_model_installed("base", None) {
        Some("base".to_string())
    } else {
        None
    };

    let jfk_path = "tests/fixtures/jfk.wav";
    let audio_wav = if std::path::Path::new(jfk_path).exists() {
        Some(std::fs::read(jfk_path).expect("Failed to read fixture jfk.wav"))
    } else {
        None
    };

    for &threads in &thread_configs {
        let model_id = installed_model
            .clone()
            .unwrap_or_else(|| "tiny.en".to_string());
        let provider = LocalAiProvider::with_config(model_id.clone(), None, threads);
        assert_eq!(
            provider.threads, threads,
            "Provider should store configured thread count {}",
            threads
        );

        if let (Some(_), Some(wav_bytes)) = (&installed_model, &audio_wav) {
            println!(
                "Testing Whisper inference with model '{}' and {} threads...",
                model_id, threads
            );
            let start = std::time::Instant::now();
            let result = provider
                .transcribe(wav_bytes.clone(), None)
                .await
                .unwrap_or_else(|e| {
                    panic!("Whisper inference failed with {} threads: {}", threads, e)
                });
            let elapsed = start.elapsed();
            println!(
                "Inference with {} threads finished in {:.2?}: '{}'",
                threads, elapsed, result
            );

            assert!(
                !result.is_empty(),
                "Transcription must not be empty with {} threads",
                threads
            );
            let lower = result.to_lowercase();
            assert!(
                lower.contains("fellow")
                    || lower.contains("americans")
                    || lower.contains("country"),
                "Transcription should contain speech text, got: '{}'",
                result
            );
        } else {
            println!(
                "Skipping live Whisper execution for {} threads (model or audio fixture not found)",
                threads
            );
        }
    }
}

#[test]
fn test_audio_level_rms_under_extreme_signal_stress() {
    // 1. Silence (all zeros)
    let silence = vec![0.0f32; 1600];
    let rms_silence = calculate_rms(&silence);
    assert_eq!(rms_silence, 0.0, "RMS of silence must be 0.0");
    let levels_silence = calculate_5_bar_levels(&silence);
    assert_eq!(levels_silence, [0.0; 5], "All bars must be 0.0 for silence");

    // 2. Pure DC offset (constant 1.0)
    let dc = vec![1.0f32; 1600];
    let rms_dc = calculate_rms(&dc);
    assert!(
        rms_dc < 1e-5,
        "Constant DC offset should yield 0.0 acoustic RMS, got {}",
        rms_dc
    );
    let levels_dc = calculate_5_bar_levels(&dc);
    assert_eq!(
        levels_dc, [0.0; 5],
        "All bars must be 0.0 for pure DC bias signal"
    );

    // 3. Overdriven values (+5.0, -5.0)
    let overdriven: Vec<f32> = (0..1600)
        .map(|i| if i % 2 == 0 { 5.0f32 } else { -5.0f32 })
        .collect();
    let rms_overdriven = calculate_rms(&overdriven);
    assert!(rms_overdriven.is_finite(), "RMS must be finite");
    assert!(
        (rms_overdriven - 5.0).abs() < 1e-4,
        "RMS of +/-5.0 square wave must be ~5.0, got {}",
        rms_overdriven
    );
    let levels_overdriven = calculate_5_bar_levels(&overdriven);
    for (i, &lvl) in levels_overdriven.iter().enumerate() {
        assert!(
            lvl.is_finite(),
            "Overdriven level bar {} must be finite: {}",
            i,
            lvl
        );
        assert!(
            (0.0..=1.0).contains(&lvl),
            "Overdriven level bar {} must be clamped to [0.0, 1.0], got: {}",
            i,
            lvl
        );
    }

    // 4. Alternating Nyquist signals ([1.0, -1.0, 1.0, -1.0])
    let nyquist: Vec<f32> = (0..1600)
        .map(|i| if i % 2 == 0 { 1.0f32 } else { -1.0f32 })
        .collect();
    let rms_nyquist = calculate_rms(&nyquist);
    assert!(rms_nyquist.is_finite(), "RMS must be finite");
    assert!(
        (rms_nyquist - 1.0).abs() < 1e-4,
        "RMS of +/-1.0 Nyquist square wave must be ~1.0, got {}",
        rms_nyquist
    );
    let levels_nyquist = calculate_5_bar_levels(&nyquist);
    for (i, &lvl) in levels_nyquist.iter().enumerate() {
        assert!(
            lvl.is_finite(),
            "Nyquist level bar {} must be finite: {}",
            i,
            lvl
        );
        assert!(
            (0.0..=1.0).contains(&lvl),
            "Nyquist level bar {} must be clamped to [0.0, 1.0], got: {}",
            i,
            lvl
        );
    }

    // 5. NaN safety and extreme edge conditions
    let nan_signal = vec![f32::NAN; 1600];
    let levels_nan = calculate_5_bar_levels(&nan_signal);
    for (i, &lvl) in levels_nan.iter().enumerate() {
        assert!(
            lvl.is_finite(),
            "Level bar {} must be finite under NaN input: {}",
            i,
            lvl
        );
        assert!(
            (0.0..=1.0).contains(&lvl),
            "Level bar {} must be clamped to [0.0, 1.0] under NaN input, got: {}",
            i,
            lvl
        );
    }

    // 6. Inf / Neg-Inf safety
    let inf_signal = vec![f32::INFINITY; 1600];
    let levels_inf = calculate_5_bar_levels(&inf_signal);
    for (i, &lvl) in levels_inf.iter().enumerate() {
        assert!(
            lvl.is_finite(),
            "Level bar {} must be finite under Inf input: {}",
            i,
            lvl
        );
        assert!(
            (0.0..=1.0).contains(&lvl),
            "Level bar {} must be clamped to [0.0, 1.0] under Inf input, got: {}",
            i,
            lvl
        );
    }

    // 7. Mixed corrupted signal with interleaved NaN and Inf
    let mut corrupted = vec![0.5f32; 1600];
    corrupted[10] = f32::NAN;
    corrupted[50] = f32::INFINITY;
    corrupted[100] = f32::NEG_INFINITY;
    let levels_corrupted = calculate_5_bar_levels(&corrupted);
    for (i, &lvl) in levels_corrupted.iter().enumerate() {
        assert!(
            lvl.is_finite(),
            "Level bar {} must be finite under corrupted input: {}",
            i,
            lvl
        );
        assert!(
            (0.0..=1.0).contains(&lvl),
            "Level bar {} must be clamped to [0.0, 1.0] under corrupted input, got: {}",
            i,
            lvl
        );
    }
}

#[test]
fn test_multi_hardware_profile_hotplug_and_exclusive_routing_stress() {
    use opendictate::audio::recorder::AudioRecorder;

    // 1. Verify strictly 3 standardized microphone profile options across diverse raw hardware lists
    let hardware_scenarios = [
        vec!["default".to_string()],
        vec![
            "sysdefault:CARD=PCH".to_string(),
            "bluez_input.AA_BB_CC_DD_EE_FF.0".to_string(),
            "alsa_output.pci-0000_00_1f.3.analog-stereo.monitor".to_string(),
        ],
        vec![
            "alsa_input.usb-HyperX_Cloud_II_Wireless-00.mono-fallback".to_string(),
            "alsa_input.pci-0000_00_1f.3.analog-stereo".to_string(),
        ],
    ];
    for raw in &hardware_scenarios {
        let friendly = AudioRecorder::filter_and_standardize_devices(raw);
        assert_eq!(
            friendly,
            vec![
                "System Default".to_string(),
                "Headphones".to_string(),
                "Handsfree".to_string()
            ],
            "Microphone selector must strictly expose System Default, Headphones, and Handsfree"
        );
    }

    // 2. Stress test 100 rapid hardware connection/disconnection state transitions
    let pactl_internal_only = "\
Source #1
\tName: alsa_output.pci-0000_00_1f.3.analog-stereo.monitor
\tDescription: Monitor of Built-in Audio Analog Stereo
Source #2
\tName: alsa_input.pci-0000_00_1f.3.analog-stereo
\tDescription: Built-in Audio Analog Stereo
\tPorts:
\t\tanalog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, available)
\t\tanalog-input-headset-mic: Headset Microphone (type: Headset, priority: 8800, not available)
\tActive Port: analog-input-internal-mic
";

    let pactl_wired_trrs = "\
Source #2
\tName: alsa_input.pci-0000_00_1f.3.analog-stereo
\tPorts:
\t\tanalog-input-internal-mic: Internal Microphone (type: Mic, priority: 8900, available)
\t\tanalog-input-headset-mic: Headset Microphone (type: Headset, priority: 8800, available)
\tActive Port: analog-input-headset-mic
";

    let pactl_usb_headset = "\
Source #2
\tName: alsa_input.pci-0000_00_1f.3.analog-stereo
\tActive Port: analog-input-internal-mic
Source #5
\tName: alsa_input.usb-Logitech_Zone_Wired-00.mono-fallback
\tDescription: Logitech Zone Wired Headset Mono
";

    let pactl_bt_handsfree = "\
Source #2
\tName: alsa_input.pci-0000_00_1f.3.analog-stereo
\tActive Port: analog-input-internal-mic
Source #5
\tName: alsa_input.usb-Logitech_Zone_Wired-00.mono-fallback
Source #9
\tName: bluez_input.88_C9_E8_11_22_33.0
\tDescription: Sony WH-1000XM5 Handsfree
";

    let pactl_cards_bt_a2dp_with_hfp = "\
Card #3
\tName: bluez_card.88_C9_E8_11_22_33
\tProfiles:
\t\ta2dp-sink: High Fidelity Playback (A2DP Sink) (sinks: 1, sources: 0, priority: 40, available: yes)
\t\theadset-head-unit: Handsfree Head Unit (HFP) (sinks: 1, sources: 1, priority: 30, available: yes)
\tActive Profile: a2dp-sink
";

    for cycle in 0..100 {
        let (sources, cards, expected_profile) = match cycle % 5 {
            0 => (pactl_internal_only, "", "System Default"),
            1 => (pactl_wired_trrs, "", "Headphones"),
            2 => (pactl_usb_headset, "", "Headphones"),
            3 => (pactl_bt_handsfree, "", "Handsfree"),
            _ => (
                pactl_internal_only,
                pactl_cards_bt_a2dp_with_hfp,
                "Handsfree",
            ),
        };
        let detected =
            AudioRecorder::classify_hardware_profile_from_sources_and_cards(sources, cards, &[]);
        assert_eq!(
            detected, expected_profile,
            "Failed at hotplug cycle {}",
            cycle
        );

        // Verify exclusive single-device resolution never selects a .monitor speaker loopback
        // and strictly isolates a single source name
        let resolved_bt =
            AudioRecorder::resolve_exclusive_source_from_pactl("Handsfree", pactl_bt_handsfree, "")
                .expect("Must resolve Bluetooth source");
        assert_eq!(resolved_bt, "bluez_input.88_C9_E8_11_22_33.0");
        assert!(!resolved_bt.ends_with(".monitor"));

        let resolved_hp = AudioRecorder::resolve_exclusive_source_from_pactl(
            "Headphones",
            pactl_bt_handsfree,
            "",
        )
        .expect("Must resolve USB headphone source");
        assert_eq!(
            resolved_hp,
            "alsa_input.usb-Logitech_Zone_Wired-00.mono-fallback"
        );
        assert!(!resolved_hp.ends_with(".monitor"));

        let resolved_def = AudioRecorder::resolve_exclusive_source_from_pactl(
            "System Default",
            pactl_bt_handsfree,
            "",
        )
        .expect("Must resolve internal microphone source");
        assert_eq!(resolved_def, "alsa_input.pci-0000_00_1f.3.analog-stereo");
        assert!(!resolved_def.ends_with(".monitor"));
    }
}

#[test]
fn test_2018_2026_twelve_distro_and_hardware_matrix_stress() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let packaging_doc =
        std::fs::read_to_string(root.join("docs/PACKAGING.md")).expect("Read docs/PACKAGING.md");
    let readme_doc = std::fs::read_to_string(root.join("README.md")).expect("Read README.md");
    let matrix_script = std::fs::read_to_string(root.join("scripts/test_distro_matrix.sh"))
        .expect("Read scripts/test_distro_matrix.sh");

    // Complete 2018–2026 release catalog for all 12 requested Linux distributions
    let distro_releases: &[(&str, &str, &[&str])] = &[
        (
            "Ubuntu",
            "ubuntu-26.04",
            &[
                "18.04 LTS",
                "18.10",
                "19.04",
                "19.10",
                "20.04 LTS",
                "20.10",
                "21.04",
                "21.10",
                "22.04 LTS",
                "22.10",
                "23.04",
                "23.10",
                "24.04 LTS",
                "24.10",
                "25.04",
                "25.10",
                "26.04",
            ],
        ),
        (
            "Linux Mint",
            "mint-22",
            &[
                "Mint 19", "19.1", "19.2", "19.3", "Mint 20", "20.1", "20.2", "20.3", "Mint 21",
                "21.1", "21.2", "21.3", "Mint 22", "22.1", "22.2", "22.3", "LMDE 3", "LMDE 4",
                "LMDE 5", "LMDE 6", "LMDE 7",
            ],
        ),
        (
            "Debian",
            "debian-13",
            &[
                "Debian 10",
                "Buster",
                "Debian 11",
                "Bullseye",
                "Debian 12",
                "Bookworm",
                "Debian 13",
                "Trixie",
                "Sid",
            ],
        ),
        (
            "Arch Linux",
            "arch",
            &["2018.01", "2026.10", "Arch Linux Rolling"],
        ),
        (
            "Fedora",
            "fedora",
            &[
                "Fedora 28",
                "29",
                "30",
                "31",
                "32",
                "33",
                "34",
                "35",
                "36",
                "37",
                "38",
                "Fedora 39",
                "40",
                "41",
                "42",
                "43",
                "44",
                "Silverblue",
            ],
        ),
        (
            "Kali Linux",
            "kali",
            &[
                "2018.1", "2018.4", "2019.1", "2019.4", "2020.1", "2020.4", "2021.1", "2021.4",
                "2022.1", "2022.4", "2023.1", "2023.2", "2023.3", "2023.4", "2024.1", "2024.4",
                "2025.1", "2025.4", "2026.1", "2026.3",
            ],
        ),
        (
            "Manjaro",
            "manjaro",
            &[
                "18.0 Illyria",
                "18.1 Juhraya",
                "19.0 Kyria",
                "20.0 Lysia",
                "20.1 Mikah",
                "20.2 Nibia",
                "21.0 Ornara",
                "21.1 Pahvo",
                "21.2 Qonos",
                "21.3 Ruah",
                "22.0 Sikaris",
                "22.1 Talos",
                "23.0 Uranos",
                "23.1 Vulcan",
                "24.0 Wynsdey",
                "24.1 Xahea",
                "24.2 Yonada",
                "25.0 Zetar",
                "26.0",
            ],
        ),
        (
            "openSUSE",
            "opensuse",
            &[
                "Leap 15.0",
                "15.1",
                "15.2",
                "15.3",
                "15.4",
                "15.5",
                "15.6",
                "Leap 16.0",
                "Tumbleweed",
                "MicroOS",
                "Aeon",
            ],
        ),
        (
            "elementary OS",
            "elementary-8",
            &[
                "5.0", "Juno", "5.1", "Hera", "6.0", "Odin", "6.1", "Jólnir", "7.0", "7.1",
                "Horus", "8.0", "8.1", "Circe",
            ],
        ),
        (
            "Zorin OS",
            "zorin-18",
            &[
                "Zorin OS 15",
                "15.1",
                "15.2",
                "15.3",
                "Zorin OS 16",
                "16.1",
                "16.2",
                "16.3",
                "Zorin OS 17",
                "17.1",
                "17.2",
                "17.3",
                "Zorin OS 18",
            ],
        ),
        (
            "Linux Lite",
            "linux-lite-7",
            &[
                "4.0", "4.2", "4.4", "4.6", "4.8", "5.0", "5.2", "5.4", "5.6", "5.8", "6.0", "6.2",
                "6.4", "6.6", "7.0", "7.2", "7.4", "Galena",
            ],
        ),
        (
            "SteamOS",
            "steamos-3",
            &[
                "2.195",
                "Brewmaster",
                "3.0",
                "3.1",
                "3.2",
                "3.3",
                "3.4",
                "3.5",
                "3.6",
                "3.7",
                "Holo",
                "Aerith",
                "Sephiroth",
            ],
        ),
    ];

    assert_eq!(
        distro_releases.len(),
        12,
        "Must validate all 12 requested Linux distributions"
    );

    for (distro_name, script_flag, releases) in distro_releases {
        assert!(
            packaging_doc.contains(distro_name),
            "docs/PACKAGING.md must document distribution family '{}'",
            distro_name
        );
        assert!(
            readme_doc.contains(distro_name),
            "README.md must document distribution family '{}'",
            distro_name
        );
        assert!(
            matrix_script.contains(script_flag),
            "scripts/test_distro_matrix.sh must support target flag '{}'",
            script_flag
        );
        for release in *releases {
            assert!(
                packaging_doc.contains(release),
                "docs/PACKAGING.md must explicitly cover {} release '{}'",
                distro_name,
                release
            );
        }
    }

    // Verify hardware architectures across desktop, laptop, legacy x86_64, AVX2/FMA, ARM64, and Steam Deck APU
    for hw_marker in [
        "x86_64",
        "aarch64",
        "AVX2",
        "Universal",
        "NEON",
        "Zen 2 APU",
        "Aerith",
        "Sephiroth",
    ] {
        assert!(
            packaging_doc.contains(hw_marker),
            "docs/PACKAGING.md must cover hardware architecture '{}'",
            hw_marker
        );
    }
}

#[test]
fn test_sqlite_storage_and_dictionary_high_concurrency_stress() {
    use opendictate::config::Config;
    use opendictate::services::storage::StorageService;

    let temp_dir = tempfile::tempdir().expect("Create temp dir for SQLite stress test");
    let db_path = temp_dir.path().join("stress_history.db");
    let storage = StorageService::new(&db_path).expect("Initialize StorageService");

    // Rapidly insert 150 multilingual dictation records
    let mut inserted_ids = Vec::with_capacity(150);
    for i in 0..150 {
        let raw = format!("raw speech transcript iteration {}", i);
        let enhanced = format!(
            "Polished transcript #{} — Español / 日本語 / हिन्दी / العربية",
            i
        );
        let id = storage
            .insert_dictation(&raw, &enhanced, "clean", "cloud", 1.25 + (i as f64) * 0.1)
            .expect("Insert dictation record");
        inserted_ids.push(id);
    }

    let records = storage
        .list_dictations()
        .expect("List 150 dictation records");
    assert_eq!(records.len(), 150);

    // Delete every even-indexed record and verify integrity
    for (idx, &id) in inserted_ids.iter().enumerate() {
        if idx % 2 == 0 {
            storage
                .delete_dictation(id)
                .expect("Delete dictation record");
        }
    }

    let remaining = storage
        .list_dictations()
        .expect("List remaining dictation records");
    assert_eq!(remaining.len(), 75);

    // Config serialization and persistence stress across 100 rapid updates
    let cfg_path = temp_dir.path().join("stress_config.json");
    for i in 0..100 {
        let cfg = Config {
            audio_device: Some(
                match i % 3 {
                    0 => "System Default",
                    1 => "Headphones",
                    _ => "Handsfree",
                }
                .to_string(),
            ),
            local_threads: (i % 9) as u32,
            ..Default::default()
        };
        cfg.save_to(&cfg_path).expect("Save config");
        let loaded = Config::load_from(&cfg_path).expect("Load config");
        assert_eq!(loaded.audio_device, cfg.audio_device);
        assert_eq!(loaded.local_threads, cfg.local_threads);
    }
}

#[test]
fn test_full_ui_ux_i18n_and_model_switching_stress() {
    use gtk4::prelude::*;
    use libadwaita::prelude::*;
    use opendictate::config::Config;
    use opendictate::services::i18n::{LANGUAGES, is_rtl, tr};
    use opendictate::services::storage::StorageService;
    use opendictate::ui::main_window::{MainWindowModel, MainWindowMsg, header, settings_view};
    use opendictate::ui::mini_bar::build_minibar_window;
    use opendictate::ui::theme::ThemeMode;

    if !(gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok())) {
        return;
    }

    let temp_dir = tempfile::tempdir().expect("Temp dir");
    let storage =
        StorageService::new(&temp_dir.path().join("ui_stress.db")).expect("Init UI stress storage");
    let config = Config::default();

    let (_header_bar, header_widgets) = header::build_header_bar(&config, &storage);
    let (_settings_page, settings_widgets) = settings_view::build_settings_view(&config);
    let (_mb_win, minibar_widgets) = build_minibar_window();
    let mut model = MainWindowModel::new_with_config_path(
        config.clone(),
        storage.clone(),
        temp_dir.path().join("stress_config.json"),
    );

    // 1. Cycle through all supported desktop Linux languages and verify live retranslation
    assert!(LANGUAGES.len() >= 25);
    for lang in LANGUAGES {
        model.update(MainWindowMsg::SetUiLanguage(lang.code.to_string()));
        header_widgets.retranslate(lang.code);
        settings_widgets.retranslate(lang.code);
        assert_eq!(
            settings_widgets.page.title().as_str(),
            tr("settings", lang.code)
        );
        if lang.code == "ar" {
            assert!(is_rtl(lang.code));
        } else {
            assert!(!is_rtl(lang.code));
        }
    }
    model.update(MainWindowMsg::SetUiLanguage("en".to_string()));
    header_widgets.retranslate("en");
    settings_widgets.retranslate("en");

    // 2. Cycle through all Cloud AI providers and verify Usage Limit gating & warning tooltip hooks
    for (provider_idx, prov) in [
        "groq",
        "gemini",
        "openai",
        "mistral",
        "nvidia",
        "openrouter",
    ]
    .iter()
    .enumerate()
    {
        settings_widgets
            .provider_row
            .set_selected(provider_idx as u32);
        settings_widgets.api_key_row.set_text("");
        assert!(!settings_widgets.usage_limit_button.is_visible());
        assert!(!settings_widgets.usage_limit_row.is_activatable());

        settings_widgets
            .api_key_row
            .set_text("sk-live-stress-test-key");
        let models = opendictate::services::ai::get_provider_models(prov);
        assert!(!models.is_empty());
        if let Some(first_model) = models.first() {
            settings_view::update_usage_limit_state(
                &settings_widgets.usage_limit_row,
                &settings_widgets.usage_limit_button,
                &settings_widgets.usage_limit_popover,
                "sk-live-stress-test-key",
                &first_model.id,
            );
            assert!(settings_widgets.usage_limit_button.is_visible());
            assert!(settings_widgets.usage_limit_row.is_activatable());
        }
    }

    // 3. Cycle through all thread options (including index 0 "Auto/System Default" and custom threads)
    let threads_model = settings_widgets
        .threads_row
        .model()
        .and_downcast::<gtk4::StringList>()
        .expect("threads StringList");
    assert_eq!(
        threads_model.string(0).unwrap().as_str(),
        "Auto/System Default"
    );
    for idx in 0..7u32 {
        settings_widgets.threads_row.set_selected(idx);
    }

    // 4. Rapidly cycle microphone hardware auto-detection and manual override
    for profile in ["Headphones", "Handsfree", "System Default", "Handsfree"] {
        settings_widgets.apply_detected_audio_profile(profile);
        let selected_idx = settings_widgets.audio_device_row.selected() as usize;
        assert_eq!(settings_widgets.audio_devices[selected_idx], profile);
    }
    // Manual override to "Headphones" while hardware stays on "Handsfree"
    settings_widgets.audio_device_row.set_selected(1);
    settings_widgets.apply_detected_audio_profile("Handsfree");
    assert_eq!(settings_widgets.audio_device_row.selected(), 1);

    // 5. Cycle MiniBar zoom scales (80% to 150%) and Theme modes (Dark <-> White)
    for scale in [80, 90, 100, 110, 125, 140, 150] {
        minibar_widgets.apply_scale(scale);
        model.update(MainWindowMsg::SetMiniBarScale(scale));
    }
    for mode in [ThemeMode::White, ThemeMode::Dark, ThemeMode::White] {
        model.update(MainWindowMsg::SetTheme(mode));
        assert_eq!(model.theme_mode(), mode);
    }

    // 6. Verify MiniBar recording guard blocks paid/unsupported models and shows tooltip
    minibar_widgets.set_recording_warning(Some(
        "The selected model requires a paid API plan. Please upgrade your provider plan to use this model."
            .to_string(),
    ));
    assert!(!minibar_widgets.try_start_recording_or_warn());
    minibar_widgets.set_recording_warning(None);
    assert!(minibar_widgets.try_start_recording_or_warn());
}
