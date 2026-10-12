//! Reproducible baseline and regression benchmark suite for speech transcription,
//! resampling fidelity, hallucination suppression, and conservative local formatting.

use opendictate::audio::recorder::{
    AudioRecorder, has_voice_activity, has_voice_activity_at_rate, resample_to_16k,
};
use opendictate::services::ai::AiProvider;
use opendictate::services::ai::local_ai::{
    CompiledX86SimdFeatures, LocalAiProvider, check_x86_simd_requirements, clean_whisper_segment,
    compiled_x86_simd_features, detected_x86_simd_features, is_executable_file,
    resolve_avx2_sibling_path, resolve_model_path, should_dispatch_to_avx2_binary,
    verify_cpu_simd_compatibility,
};
use opendictate::services::ai::utils::apply_smart_local_formatting;
use std::time::Instant;

/// Computes word-level Levenshtein distance (Substitutions + Deletions + Insertions)
/// and Word Error Rate (WER = edit_distance / reference_word_count).
fn compute_wer(reference: &str, hypothesis: &str) -> (usize, usize, f64) {
    let normalize = |s: &str| -> Vec<String> {
        s.split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .filter(|w| !w.is_empty())
            .collect()
    };

    let ref_words = normalize(reference);
    let hyp_words = normalize(hypothesis);
    let n = ref_words.len();
    let m = hyp_words.len();

    if n == 0 {
        return (m, 0, if m == 0 { 0.0 } else { 1.0 });
    }

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, val) in dp[0].iter_mut().enumerate().take(m + 1) {
        *val = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = usize::from(ref_words[i - 1] != hyp_words[j - 1]);
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    let dist = dp[n][m];
    (dist, n, dist as f64 / n as f64)
}

/// Counts how many required reference tokens are missing from the formatted output.
fn count_false_deletions(required_tokens: &[&str], output: &str) -> usize {
    let lower_out = output.to_lowercase();
    required_tokens
        .iter()
        .filter(|tok| !lower_out.contains(&tok.to_lowercase()))
        .count()
}

/// Generates a deterministic low-amplitude noise vector (RMS ~ 0.0005) using a fixed LCG seed.
fn deterministic_noise(num_samples: usize, amplitude: f32) -> Vec<f32> {
    let mut state: u64 = 0x5DEECE66D;
    let mut out = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let unit = ((state >> 33) as f32 / (1u64 << 31) as f32) * 2.0 - 1.0;
        out.push(unit * amplitude);
    }
    out
}

#[test]
fn test_reference_corpus_formatting_preservation_and_zero_false_deletions() {
    // 10 fixed representative utterances covering technical terms, Indian English,
    // proper names, numbers, negation, emphasis, and false-deletion trap phrases.
    let corpus: &[(&str, &str, &[&str])] = &[
        (
            "um we updated Cargo.toml and v2.0.0 in src/main.rs. do not delete it",
            "We updated Cargo.toml and v2.0.0 in src/main.rs. Do not delete it.",
            &["Cargo.toml", "v2.0.0", "src/main.rs", "do", "not", "delete"],
        ),
        (
            "uh Aarav and Priya from Bengaluru said please do the needful by 5:30 PM IST",
            "Aarav and Priya from Bengaluru said please do the needful by 5:30 PM IST.",
            &["Aarav", "Priya", "Bengaluru", "needful", "5:30", "IST"],
        ),
        (
            "er we preponed the GTK4 and PipeWire release to Tuesday",
            "We preponed the GTK4 and PipeWire release to Tuesday.",
            &["preponed", "GTK4", "PipeWire", "Tuesday"],
        ),
        (
            "Never overwrite config.json without atomic rename and 0600 permissions",
            "Never overwrite config.json without atomic rename and 0600 permissions.",
            &["Never", "overwrite", "config.json", "0600"],
        ),
        (
            "Thank you for watching the demo and testing the build",
            "Thank you for watching the demo and testing the build.",
            &["Thank", "you", "for", "watching", "demo", "testing"],
        ),
        (
            "Please subscribe to the release notes on GitHub",
            "Please subscribe to the release notes on GitHub.",
            &["Please", "subscribe", "release", "notes", "GitHub"],
        ),
        (
            "Like I said, I like how fast the 16 kHz SQLite storage works",
            "Like I said, I like how fast the 16 kHz SQLite storage works.",
            &["Like", "like", "16", "kHz", "SQLite", "storage"],
        ),
        (
            "This is actually critical: kind of a blocker if async/await hangs",
            "This is actually critical: kind of a blocker if async/await hangs.",
            &["actually", "critical", "kind", "of", "async/await"],
        ),
        (
            "The result is very very good and we said no no no to data loss",
            "The result is very very good and we said no no no to data loss.",
            &["very very", "no no no", "data", "loss"],
        ),
        (
            "Check port 8080 and 127.0.0.1:11434 for local Ollama inference",
            "Check port 8080 and 127.0.0.1:11434 for local Ollama inference.",
            &["8080", "127.0.0.1:11434", "Ollama"],
        ),
    ];

    let mut total_edit_dist = 0usize;
    let mut total_ref_words = 0usize;
    let mut total_false_deletions = 0usize;

    for (raw_input, expected_clean, required_tokens) in corpus {
        // 1. Raw mode must preserve input verbatim
        let out_raw = apply_smart_local_formatting(raw_input, "Raw");
        assert_eq!(
            out_raw, *raw_input,
            "Raw mode must be a verbatim passthrough"
        );

        // 2. Clean, Professional, and Concise modes in offline formatting
        for tone in ["Clean", "Professional", "Concise"] {
            let formatted = apply_smart_local_formatting(raw_input, tone);
            assert_eq!(
                &formatted, expected_clean,
                "Unexpected formatting for tone {} on input {:?}",
                tone, raw_input
            );

            let false_dels = count_false_deletions(required_tokens, &formatted);
            total_false_deletions += false_dels;
            assert_eq!(
                false_dels, 0,
                "False deletion detected in tone {} for input {:?} -> {:?}",
                tone, raw_input, formatted
            );
        }

        let (dist, ref_len, _wer) = compute_wer(
            expected_clean,
            &apply_smart_local_formatting(raw_input, "Clean"),
        );
        total_edit_dist += dist;
        total_ref_words += ref_len;
    }

    let corpus_wer = total_edit_dist as f64 / total_ref_words.max(1) as f64;
    println!(
        "[BENCHMARK] Reference Corpus Formatting: WER={:.4} ({}/{} words), FalseDeletions={}",
        corpus_wer, total_edit_dist, total_ref_words, total_false_deletions
    );
    assert_eq!(total_false_deletions, 0);
    assert_eq!(total_edit_dist, 0);
}

#[test]
fn test_hallucination_filter_preserves_spoken_phrases_and_strips_markers() {
    // Bracketed/asterisked non-speech markers must be stripped
    assert_eq!(clean_whisper_segment("[BLANK_AUDIO]"), "");
    assert_eq!(clean_whisper_segment("(blank audio)"), "");
    assert_eq!(clean_whisper_segment("[ Silence ]"), "");
    assert_eq!(clean_whisper_segment("*inaudible*"), "");

    // Punctuation-only silence tokens emitted when no_timestamps=true must be stripped
    assert_eq!(clean_whisper_segment("."), "");
    assert_eq!(clean_whisper_segment(" . "), "");
    assert_eq!(clean_whisper_segment("..."), "");
    assert_eq!(clean_whisper_segment("?,!"), "");

    // Single alphanumeric characters and numeric tokens must be preserved
    assert_eq!(clean_whisper_segment("A"), "A");
    assert_eq!(clean_whisper_segment("1"), "1");
    assert_eq!(clean_whisper_segment("v2.0"), "v2.0");

    // Legitimate English phrases must NEVER be deleted by clean_whisper_segment
    assert_eq!(
        clean_whisper_segment("Thank you for watching."),
        "Thank you for watching."
    );
    assert_eq!(
        clean_whisper_segment("Please subscribe to the channel."),
        "Please subscribe to the channel."
    );
    assert_eq!(
        clean_whisper_segment("Hello [BLANK_AUDIO] world"),
        "Hello world"
    );

    // Pathological >= 4x repetition loops are collapsed while 2x/3x natural emphasis is kept
    assert_eq!(
        clean_whisper_segment("the the the the the system is ready"),
        "the system is ready"
    );
    assert_eq!(clean_whisper_segment("very very good"), "very very good");
    assert_eq!(clean_whisper_segment("no no no"), "no no no");
}

#[test]
fn test_cpu_simd_compatibility_and_portable_x86_64_v3_build_guard() {
    // 1. Verify runtime CPU compatibility check passes on the host CPU
    verify_cpu_simd_compatibility()
        .expect("Host CPU must satisfy compiled ggml-cpu SIMD requirements");

    // 2. On x86_64, verify ggml-cpu and rustc target_feature match the exact configured build tier:
    //    - Default / Portable build (`GGML_AVX2 != ON`): pure x86-64-v1 (SSE2) baseline with zero optional SIMD requirements
    //    - Optimized AVX2 build (`GGML_AVX2 == ON`): 9-feature AVX2 tier (SSE3/SSSE3/SSE4.1/SSE4.2/AVX/AVX2/FMA/F16C/BMI2),
    //      with BMI1/LZCNT/POPCNT/MOVBE/AVX-512/AVX-VNNI explicitly disabled
    #[cfg(target_arch = "x86_64")]
    {
        let compiled = compiled_x86_simd_features();
        let expecting_avx2 = matches!(
            option_env!("GGML_AVX2"),
            Some("ON" | "on" | "1" | "TRUE" | "true")
        );
        // Verify non-contract scalar/vector extensions remain disabled in both tiers
        const { assert!(!cfg!(target_feature = "bmi1"), "bmi1 must remain disabled") };
        const { assert!(!cfg!(target_feature = "lzcnt"), "lzcnt must remain disabled") };
        const { assert!(!cfg!(target_feature = "popcnt"), "popcnt must remain disabled") };
        const { assert!(!cfg!(target_feature = "movbe"), "movbe must remain disabled") };
        const { assert!(!cfg!(target_feature = "avx512f"), "avx512f must remain disabled") };
        const { assert!(!cfg!(target_feature = "avxvnni"), "avxvnni must remain disabled") };

        for (feat_name, is_enabled) in [
            ("sse3", cfg!(target_feature = "sse3")),
            ("ssse3", cfg!(target_feature = "ssse3")),
            ("sse4.1", cfg!(target_feature = "sse4.1")),
            ("sse4.2", cfg!(target_feature = "sse4.2")),
            ("avx", cfg!(target_feature = "avx")),
            ("avx2", cfg!(target_feature = "avx2")),
            ("fma", cfg!(target_feature = "fma")),
            ("f16c", cfg!(target_feature = "f16c")),
            ("bmi2", cfg!(target_feature = "bmi2")),
        ] {
            assert_eq!(
                is_enabled, expecting_avx2,
                "rustc target_feature={feat_name} must match expecting_avx2={expecting_avx2}"
            );
        }

        if expecting_avx2 {
            assert_eq!(
                compiled,
                CompiledX86SimdFeatures::avx2_optimized_tier(),
                "Optimized AVX2 build must match the exact 9-feature AVX2 profile without AVX-512"
            );
        } else {
            assert_eq!(
                compiled,
                CompiledX86SimdFeatures::portable_x86_64_v1(),
                "Portable baseline build must not require any optional x86_64 SIMD extensions beyond SSE2"
            );
        }

        let portable_tier = CompiledX86SimdFeatures::portable_x86_64_v1();
        let avx2_tier = CompiledX86SimdFeatures::avx2_optimized_tier();

        // 3. Supported CPU combinations:
        //    - Portable binary must pass even on a pure SSE2 (x86-64-v1) processor
        check_x86_simd_requirements(portable_tier, portable_tier)
            .expect("Portable x86-64-v1 binary must pass on a baseline SSE2-only CPU");
        //    - Optimized AVX2 binary must pass on exact match and AVX-512 superset CPU
        check_x86_simd_requirements(avx2_tier, avx2_tier)
            .expect("Exact 9-feature AVX2 tier match must pass");
        let mut avx512_superset = avx2_tier;
        avx512_superset.avx_vnni = true;
        avx512_superset.avx512 = true;
        avx512_superset.avx512_vbmi = true;
        avx512_superset.avx512_vnni = true;
        avx512_superset.avx512_bf16 = true;
        check_x86_simd_requirements(avx2_tier, avx512_superset)
            .expect("AVX-512 superset CPU running AVX2-tier binary must pass");

        // 4. Simulate a pre-2008 Core 2 CPU (has SSE3/SSSE3, lacks SSE4.1/SSE4.2/AVX/AVX2/FMA/F16C/BMI2)
        let core2_detected = CompiledX86SimdFeatures {
            sse3: true,
            ssse3: true,
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
        };
        check_x86_simd_requirements(portable_tier, core2_detected)
            .expect("Core 2 CPU must pass portable x86-64-v1 check");
        let core2_msg = check_x86_simd_requirements(avx2_tier, core2_detected)
            .expect_err("Core 2 CPU must fail AVX2-tier check")
            .to_string();
        for expected in &["SSE4.1", "SSE4.2", "AVX", "AVX2", "FMA", "F16C", "BMI2"] {
            assert!(
                core2_msg.contains(expected),
                "Core 2 error must list {expected}: {core2_msg}"
            );
        }

        // 5. Simulate a 2008 Penryn CPU (has SSE4.1, lacks SSE4.2/AVX/AVX2/FMA/F16C/BMI2)
        let mut penryn_detected = core2_detected;
        penryn_detected.sse41 = true;
        let penryn_msg = check_x86_simd_requirements(avx2_tier, penryn_detected)
            .expect_err("Penryn CPU must fail SSE4.2+ check")
            .to_string();
        assert!(
            !penryn_msg.contains("SSE4.1"),
            "Penryn supports SSE4.1 so it must not be listed as missing: {penryn_msg}"
        );
        assert!(
            penryn_msg.contains("SSE4.2"),
            "Penryn error must list missing SSE4.2: {penryn_msg}"
        );

        // 6. Simulate a 2011 Sandy Bridge CPU (has SSE4.1/SSE4.2/AVX, lacks AVX2/FMA/F16C/BMI2)
        let mut sandy_bridge_detected = penryn_detected;
        sandy_bridge_detected.sse42 = true;
        sandy_bridge_detected.avx = true;
        check_x86_simd_requirements(portable_tier, sandy_bridge_detected)
            .expect("Sandy Bridge CPU must pass portable x86-64-v1 check");
        let sandy_msg = check_x86_simd_requirements(avx2_tier, sandy_bridge_detected)
            .expect_err("Sandy Bridge CPU must return a descriptive error before FFI init")
            .to_string();
        assert!(
            !sandy_msg.contains("SSE4.1") && !sandy_msg.contains("SSE4.2"),
            "Sandy Bridge supports SSE4.1/4.2: {sandy_msg}"
        );
        for expected in &["AVX2", "FMA", "F16C", "BMI2"] {
            assert!(
                sandy_msg.contains(expected),
                "Sandy Bridge error must list missing {expected}: {sandy_msg}"
            );
        }

        // 7. Simulate an AVX-512-compiled binary running on an AVX2-tier (Zen 3) host
        let mut avx512_required = avx2_tier;
        avx512_required.avx512 = true;
        avx512_required.avx512_vnni = true;
        let zen3_msg = check_x86_simd_requirements(avx512_required, avx2_tier)
            .expect_err("AVX-512 binary on Zen 3 must fail")
            .to_string();
        assert!(zen3_msg.contains("AVX512F"), "Must report AVX512F: {zen3_msg}");
        assert!(
            zen3_msg.contains("AVX512_VNNI"),
            "Must report AVX512_VNNI: {zen3_msg}"
        );

        // 8. Explicitly test SSE4.1-only and SSE4.2-only required builds without AVX
        let sse41_only_required = CompiledX86SimdFeatures {
            sse3: true,
            ssse3: true,
            sse41: true,
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
        };
        let sse41_on_core2 = check_x86_simd_requirements(sse41_only_required, core2_detected)
            .expect_err("SSE4.1 requirement must fail on Core 2")
            .to_string();
        assert!(
            sse41_on_core2.contains("SSE4.1") && !sse41_on_core2.contains("SSE4.2"),
            "SSE4.1-only requirement on Core 2 must report only SSE4.1: {sse41_on_core2}"
        );
        check_x86_simd_requirements(sse41_only_required, penryn_detected)
            .expect("SSE4.1-only requirement must pass on Penryn (SSE4.1 supported)");

        let mut sse42_only_required = sse41_only_required;
        sse42_only_required.sse42 = true;
        let sse42_on_penryn = check_x86_simd_requirements(sse42_only_required, penryn_detected)
            .expect_err("SSE4.2 requirement must fail on Penryn")
            .to_string();
        assert!(
            sse42_on_penryn.contains("SSE4.2") && !sse42_on_penryn.contains("SSE4.1"),
            "SSE4.2 requirement on Penryn must report only SSE4.2: {sse42_on_penryn}"
        );
        let mut nehalem_detected = penryn_detected;
        nehalem_detected.sse42 = true;
        check_x86_simd_requirements(sse42_only_required, nehalem_detected)
            .expect("SSE4.2 requirement must pass on Nehalem (SSE4.1 + SSE4.2 supported)");

        // 9. Verify dual-binary runtime dispatch decision logic (`should_dispatch_to_avx2_binary`)
        //    - Portable launcher on AVX2-capable CPU with valid sibling -> MUST dispatch (true)
        assert!(
            should_dispatch_to_avx2_binary(portable_tier, avx2_tier, false, false, true),
            "Portable launcher on AVX2-compatible CPU must dispatch when sibling exists"
        );
        //    - Already inside AVX2 binary (`compiled_self.avx2 == true`) -> MUST NOT dispatch (false)
        assert!(
            !should_dispatch_to_avx2_binary(avx2_tier, avx2_tier, false, false, true),
            "AVX2 binary must never re-dispatch"
        );
        //    - Loop-prevention env flag (`already_dispatched == true`) -> MUST NOT dispatch (false)
        assert!(
            !should_dispatch_to_avx2_binary(portable_tier, avx2_tier, true, false, true),
            "Already-dispatched process must never re-dispatch"
        );
        //    - User opt-out (`disable_dispatch == true`) -> MUST NOT dispatch (false)
        assert!(
            !should_dispatch_to_avx2_binary(portable_tier, avx2_tier, false, true, true),
            "OPENDICTATE_DISABLE_AVX2_DISPATCH=1 must prevent dispatch"
        );
        //    - Missing or non-executable sibling (`candidate_exists_and_executable == false`) -> MUST NOT dispatch (false)
        assert!(
            !should_dispatch_to_avx2_binary(portable_tier, avx2_tier, false, false, false),
            "Missing or non-executable sibling must fall back to portable binary"
        );
        //    - Incompatible CPUs (SSE2-only, Core 2, Penryn, Nehalem, Sandy Bridge, or missing any single AVX2-tier flag) -> MUST NOT dispatch (false)
        for incompatible_cpu in [
            portable_tier,
            core2_detected,
            penryn_detected,
            nehalem_detected,
            sandy_bridge_detected,
        ] {
            assert!(
                !should_dispatch_to_avx2_binary(
                    portable_tier,
                    incompatible_cpu,
                    false,
                    false,
                    true
                ),
                "Incompatible CPU {incompatible_cpu:?} must fall back to portable binary"
            );
        }
        //    - Missing any single required flag in the 9-feature AVX2 tier must block dispatch
        for (label, partial_cpu) in [
            ("missing SSE3", CompiledX86SimdFeatures { sse3: false, ..avx2_tier }),
            ("missing SSSE3", CompiledX86SimdFeatures { ssse3: false, ..avx2_tier }),
            ("missing SSE4.1", CompiledX86SimdFeatures { sse41: false, ..avx2_tier }),
            ("missing SSE4.2", CompiledX86SimdFeatures { sse42: false, ..avx2_tier }),
            ("missing AVX", CompiledX86SimdFeatures { avx: false, ..avx2_tier }),
            ("missing AVX2", CompiledX86SimdFeatures { avx2: false, ..avx2_tier }),
            ("missing FMA", CompiledX86SimdFeatures { fma: false, ..avx2_tier }),
            ("missing F16C", CompiledX86SimdFeatures { f16c: false, ..avx2_tier }),
            ("missing BMI2", CompiledX86SimdFeatures { bmi2: false, ..avx2_tier }),
        ] {
            assert!(
                !should_dispatch_to_avx2_binary(portable_tier, partial_cpu, false, false, true),
                "CPU {label} must not dispatch to opendictate-avx2"
            );
        }

        // 10. Verify sibling path resolution and executable file checks
        assert_eq!(
            resolve_avx2_sibling_path(std::path::Path::new("/usr/bin/opendictate")),
            Some(std::path::PathBuf::from("/usr/bin/opendictate-avx2"))
        );
        assert_eq!(
            resolve_avx2_sibling_path(std::path::Path::new("/usr/bin/opendictate-avx2")),
            None,
            "opendictate-avx2 must not resolve a sibling to itself"
        );

        let tmp_dir = tempfile::tempdir().expect("create temp dir for executable test");
        let missing_bin = tmp_dir.path().join("opendictate-avx2");
        assert!(
            !is_executable_file(&missing_bin),
            "Missing file must not be reported as executable"
        );
        std::fs::write(&missing_bin, b"#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&missing_bin, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(
                !is_executable_file(&missing_bin),
                "0644 file must not be reported as executable"
            );
            std::fs::set_permissions(&missing_bin, std::fs::Permissions::from_mode(0o755)).unwrap();
            assert!(
                is_executable_file(&missing_bin),
                "0755 file must be reported as executable"
            );
        }
        check_x86_simd_requirements(portable_tier, detected_x86_simd_features())
            .expect("Portable x86-64-v1 tier must be satisfied on any x86_64 CPU");
    }
}


#[test]
fn test_short_time_windowed_vad_and_streaming_pause_resume_edge_cases() {
    // 1. Background noise rejection across 20s at 16 kHz and 48 kHz
    let noise_16k = deterministic_noise(20 * 16_000, 0.0005);
    let noise_48k = deterministic_noise(20 * 48_000, 0.0005);
    assert!(
        !has_voice_activity(&noise_16k),
        "20s background noise at 16kHz must not trigger VAD"
    );
    assert!(
        !has_voice_activity_at_rate(&noise_48k, 48_000),
        "20s background noise at 48kHz must not trigger VAD"
    );

    // 2. Quiet speech detection (peak = 0.0025, rms ≈ 0.00177 — above 0.0008/0.002 floor,
    //    below legacy 0.0015/0.004 threshold) across 16 kHz, 44.1 kHz, and 48 kHz
    for &rate in &[16_000u32, 44_100, 48_000] {
        let len = (rate as usize) / 2; // 0.5s
        let quiet_speech: Vec<f32> = (0..len)
            .map(|i| {
                let t = i as f32 / rate as f32;
                (2.0 * std::f32::consts::PI * 300.0 * t).sin() * 0.0025
            })
            .collect();
        assert!(
            has_voice_activity_at_rate(&quiet_speech, rate),
            "Quiet speech (peak=0.0025, rms=0.00177) at {rate} Hz must be detected as voice"
        );
    }

    // 3. Speech beginning near the end of a 450ms streaming tick (410ms silence + 40ms speech @ 48 kHz)
    //    Global RMS over the 450ms tick is diluted to ~0.00074 (< 0.0008), but the final 40ms
    //    has peak = 0.0035, rms = 0.00247 and must be detected by sample-rate-aware 20ms windows.
    let rec_tick = AudioRecorder::default();
    let snap_tick = rec_tick.snapshot_handle();
    // Initial 1.0s spoken segment + 0.25s quiet tail so the session becomes finalized at N1 = 60,000
    let mut initial_48k: Vec<f32> = (0..48_000)
        .map(|i| (2.0 * std::f32::consts::PI * 350.0 * (i as f32 / 48_000.0)).sin() * 0.05)
        .collect();
    initial_48k.extend_from_slice(&deterministic_noise(12_000, 0.0003));
    rec_tick.push_test_samples(&initial_48k, 48_000);

    let snap1 = snap_tick
        .snapshot_samples_16k(60.0, 0, false)
        .expect("initial snapshot")
        .expect("initial speech must produce snapshot");
    assert!(snap1.tail_is_quiet, "Initial speech + 250ms silence must have quiet tail");
    let mut last_transcribed_len = snap1.total_native_samples;
    let mut prev_finalized = true;

    // Push a silent 450ms tick (21,600 samples) -> must return None and MUST NOT advance last_transcribed_len
    let silent_tick_48k = deterministic_noise(21_600, 0.0003);
    rec_tick.push_test_samples(&silent_tick_48k, 48_000);
    let snap_silent = snap_tick
        .snapshot_samples_16k(60.0, last_transcribed_len, prev_finalized)
        .expect("silent tick snapshot");
    assert!(
        snap_silent.is_none(),
        "Silent tick after finalization must return None so last_transcribed_len is never advanced past untranscribed audio"
    );

    // Push a 450ms tick where the first 410ms (19,680 samples) are silent and only the last 40ms (1,920 samples) have speech onset
    let mut onset_tick_48k = deterministic_noise(19_680, 0.0003);
    for i in 0..1_920 {
        let t = i as f32 / 48_000.0;
        onset_tick_48k.push((2.0 * std::f32::consts::PI * 350.0 * t).sin() * 0.0035);
    }
    rec_tick.push_test_samples(&onset_tick_48k, 48_000);
    let snap_onset = snap_tick
        .snapshot_samples_16k(60.0, last_transcribed_len, prev_finalized)
        .expect("onset tick snapshot")
        .expect("40ms speech onset at the end of a 450ms tick must be detected");
    assert!(
        snap_onset.delta_has_voice && !snap_onset.samples_16k.is_empty(),
        "40ms speech onset must produce non-empty 16kHz snapshot"
    );
    last_transcribed_len = snap_onset.total_native_samples;
    prev_finalized = snap_onset.tail_is_quiet;
    assert!(!prev_finalized, "Active speech at end of tick must mark tail_is_quiet=false");
    let _ = last_transcribed_len;

    // 4. Short word spoken immediately before Stop after 18 seconds of silence (pause-and-resume)
    //    Simulate: finalized at N1 = 60,000 -> 18.0s of silence (864,000 samples @ 48 kHz)
    //    -> 0.35s short word (16,800 samples @ 48 kHz, peak = 0.004, rms = 0.0028) -> immediate Stop.
    let mut rec_pause = AudioRecorder::default();
    let snap_pause = rec_pause.snapshot_handle();
    rec_pause.push_test_samples(&initial_48k, 48_000);
    let snap_init = snap_pause
        .snapshot_samples_16k(60.0, 0, false)
        .expect("init")
        .expect("init snap");
    let finalized_at = Some(snap_init.total_native_samples);

    // 18 seconds of silence (40 ticks of 450ms @ 48 kHz = 864,000 samples)
    let silence_18s = deterministic_noise(18 * 48_000, 0.0003);
    rec_pause.push_test_samples(&silence_18s, 48_000);
    assert!(
        snap_pause
            .snapshot_samples_16k(60.0, snap_init.total_native_samples, true)
            .expect("18s silence check")
            .is_none(),
        "18s of silence must not trigger a streaming pass"
    );

    // User speaks a short 0.35s word (16,800 samples) and immediately presses Stop before the next tick
    let short_word_48k: Vec<f32> = (0..16_800)
        .map(|i| (2.0 * std::f32::consts::PI * 320.0 * (i as f32 / 48_000.0)).sin() * 0.004)
        .collect();
    rec_pause.push_test_samples(&short_word_48k, 48_000);

    let (final_samples_16k, reused_after_word) = rec_pause
        .stop_recording_samples_16k(finalized_at)
        .expect("stop after 18s silence + 0.35s word");
    assert!(
        !reused_after_word,
        "Stop after 18s silence + 0.35s word MUST NOT reuse stale finalized transcript"
    );
    assert!(
        !final_samples_16k.is_empty(),
        "Stop must return full normalized 16kHz buffer including the newly spoken word"
    );
}


#[test]
fn test_resampling_fidelity_across_hardware_sample_rates() {
    // Verify 44.1 kHz, 48 kHz, and 96 kHz resampling of a 440 Hz + 1200 Hz speech-band signal
    for &src_rate in &[44_100u32, 48_000, 96_000] {
        let duration_secs = 1.0f32;
        let num_samples = (src_rate as f32 * duration_secs) as usize;
        let signal: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f32 / src_rate as f32;
                0.5 * (2.0 * std::f32::consts::PI * 440.0 * t).sin()
                    + 0.3 * (2.0 * std::f32::consts::PI * 1200.0 * t).sin()
            })
            .collect();

        let t0 = Instant::now();
        let resampled = resample_to_16k(&signal, src_rate).expect("Resampling should succeed");
        let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let expected_len = 16_000usize;
        let len_diff = (resampled.len() as isize - expected_len as isize).unsigned_abs();
        assert!(
            len_diff <= 160,
            "Resampled length {} from {} Hz should be within 1% of 16000",
            resampled.len(),
            src_rate
        );

        let rms: f32 =
            (resampled.iter().map(|s| s * s).sum::<f32>() / resampled.len() as f32).sqrt();
        println!(
            "[BENCHMARK] Resample {} Hz -> 16 kHz: len={}, rms={:.4}, latency={:.2} ms",
            src_rate,
            resampled.len(),
            rms,
            elapsed_ms
        );
        // Theoretical RMS of 0.5*sin(f1) + 0.3*sin(f2) is sqrt(0.5^2/2 + 0.3^2/2) = sqrt(0.17) ~= 0.4123
        assert!(
            (rms - 0.4123).abs() < 0.03,
            "Resampled RMS {} should preserve signal energy (~0.4123)",
            rms
        );
    }
}

#[test]
fn test_reproducible_whisper_latency_and_silence_benchmark() {
    let candidate_paths = [
        resolve_model_path("tiny.en", None),
        std::path::PathBuf::from(
            "/home/hksaikia/snap/opendictate/x6/.local/share/opendictate/models/ggml-tiny.en.bin",
        ),
        resolve_model_path("tiny", None),
    ];
    let Some(model_path) = candidate_paths
        .into_iter()
        .find(|p| p.is_file() && std::fs::metadata(p).map(|m| m.len() > 0).unwrap_or(false))
    else {
        println!("[BENCHMARK] Skipping live Whisper benchmark: no local ggml-tiny model found");
        return;
    };
    let threads = 6u32;

    // Warm up model cache once so cold-start vs warm inference are measured separately
    let warmup_samples = deterministic_noise(16_000, 0.0005);
    let t_cold = Instant::now();
    let _ = LocalAiProvider::transcribe_wav_samples(&warmup_samples, &model_path, threads)
        .expect("Warmup inference should succeed");
    let cold_start_ms = t_cold.elapsed().as_secs_f64() * 1000.0;

    // 1. Silence / low-noise 2s vector: must produce empty transcript (0 hallucinations)
    let silence_2s = deterministic_noise(32_000, 0.0005);
    let t0 = Instant::now();
    let silence_out = LocalAiProvider::transcribe_wav_samples(&silence_2s, &model_path, threads)
        .expect("2s silence inference should succeed");
    let silence_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!(
        "[BENCHMARK] Whisper tiny.en (threads={}): cold_start={:.1} ms, silence_2s={:.1} ms, out={:?}",
        threads, cold_start_ms, silence_ms, silence_out
    );
    assert!(
        silence_out.is_empty(),
        "2s low-noise silence must not hallucinate text, got: {:?}",
        silence_out
    );

    // 2. Measure short (3s), medium (8s), and long (22s) silence vectors (0 hallucinations)
    for &(label, seconds) in &[
        ("short_3s", 3usize),
        ("medium_8s", 8usize),
        ("long_22s", 22usize),
    ] {
        let samples = deterministic_noise(seconds * 16_000, 0.0005);
        let start = Instant::now();
        let out = LocalAiProvider::transcribe_wav_samples(&samples, &model_path, threads)
            .expect("Benchmark inference should succeed");
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        println!(
            "[BENCHMARK] Whisper {} ({}s audio @ 16kHz, threads={}): latency={:.1} ms, out={:?}",
            label, seconds, threads, elapsed_ms, out
        );
        assert!(
            out.is_empty(),
            "{} silence vector must not hallucinate text, got: {:?}",
            label,
            out
        );
    }

    // 3. Comprehensive Short, Medium, and Long spoken accuracy regression checks with hard WER ceilings
    //    and required keyword preservation across both JFK and Speech Dispatcher fixtures
    let jfk_bytes = std::fs::read("tests/fixtures/jfk.wav").expect("jfk.wav must exist");
    let jfk_samples =
        opendictate::services::ai::local_ai::decode_wav_to_f32(&jfk_bytes).expect("decode jfk.wav");
    let mut jfk_22s = jfk_samples.clone();
    jfk_22s.extend_from_slice(&jfk_samples);

    type SpokenRegressionCase<'a> = (&'a str, &'a [f32], &'a str, &'a [&'a str], f64);
    let jfk_cases: &[SpokenRegressionCase<'_>] = &[
        (
            "short_jfk_3.0s",
            &jfk_samples[..(30 * 16_000 / 10).min(jfk_samples.len())],
            "And so my fellow Americans",
            &["And", "so", "my", "fellow", "Americans"],
            0.0,
        ),
        (
            "medium_jfk_7.9s",
            &jfk_samples[..(79 * 16_000 / 10).min(jfk_samples.len())],
            "And so my fellow Americans, ask not what your country can do for you",
            &["fellow", "Americans", "ask", "not", "country", "can", "do"],
            0.0,
        ),
        (
            "long_jfk_11.0s",
            &jfk_samples,
            "And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country.",
            &["fellow", "Americans", "country", "ask", "not"],
            0.0,
        ),
        (
            "long_jfk_22.0s",
            &jfk_22s,
            "And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country. And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country.",
            &["fellow", "Americans", "country"],
            0.0,
        ),
    ];

    for &(label, slice, reference, required_tokens, max_wer) in jfk_cases {
        let audio_secs = slice.len() as f64 / 16_000.0;
        let t_speech = Instant::now();
        let speech_out = LocalAiProvider::transcribe_wav_samples(slice, &model_path, threads)
            .expect("JFK spoken inference should succeed");
        let speech_ms = t_speech.elapsed().as_secs_f64() * 1000.0;
        let (dist, ref_words, wer) = compute_wer(reference, &speech_out);
        let false_dels = count_false_deletions(required_tokens, &speech_out);
        println!(
            "[BENCHMARK] {} ({:.2}s @ 16kHz): latency={:.1} ms, WER={:.4} (edits={}/{}), transcript={:?}",
            label, audio_secs, speech_ms, wer, dist, ref_words, speech_out
        );
        assert!(
            wer <= max_wer,
            "{} WER {:.4} (edits={}/{}) exceeded regression ceiling {:.4}; transcript={:?}",
            label,
            wer,
            dist,
            ref_words,
            max_wer,
            speech_out
        );
        assert_eq!(
            false_dels, 0,
            "{} missing required tokens {:?} in {:?}",
            label, required_tokens, speech_out
        );
    }

    let speech_wav_path =
        std::path::Path::new("/usr/share/sounds/speech-dispatcher/dummy-message.wav");
    if speech_wav_path.is_file()
        && let Ok(wav_bytes) = std::fs::read(speech_wav_path)
        && let Ok(speech_samples) =
            opendictate::services::ai::local_ai::decode_wav_to_f32(&wav_bytes)
    {
        type SpokenSliceCase<'a> = (&'a str, usize, &'a str, &'a [&'a str], f64);
        let slices: &[SpokenSliceCase<'_>] = &[
            (
                "spoken_short_2.2s",
                (22 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module.",
                &["This", "is", "the", "output", "module"],
                0.17, // <= 1/6 edits (16.67%)
            ),
            (
                "spoken_medium_7.8s",
                (78 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me.",
                &["dummy", "output", "module", "seems", "speech", "working", "none", "modules", "except", "me"],
                0.10, // <= 2/22 edits (9.09%)
            ),
            (
                "spoken_long_11.2s",
                (112 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me. Please check out your log file to see what the problem is.",
                &["dummy", "output", "module", "working", "except", "Please", "check", "out", "log", "file", "problem"],
                0.06, // <= 2/34 edits (5.88%)
            ),
        ];

        for &(label, len, reference, required_tokens, max_wer) in slices {
            let slice = &speech_samples[..len];
            let audio_secs = slice.len() as f64 / 16_000.0;
            let t_speech = Instant::now();
            let speech_out = LocalAiProvider::transcribe_wav_samples(slice, &model_path, threads)
                .expect("Spoken WAV inference should succeed");
            let speech_ms = t_speech.elapsed().as_secs_f64() * 1000.0;
            let (dist, ref_words, wer) = compute_wer(reference, &speech_out);
            let false_dels = count_false_deletions(required_tokens, &speech_out);
            println!(
                "[BENCHMARK] {} ({:.2}s @ 16kHz): latency={:.1} ms, WER={:.4} (edits={}/{}), transcript={:?}",
                label, audio_secs, speech_ms, wer, dist, ref_words, speech_out
            );
            assert!(
                wer <= max_wer,
                "{} WER {:.4} (edits={}/{}) exceeded regression ceiling {:.4}; transcript={:?}",
                label,
                wer,
                dist,
                ref_words,
                max_wer,
                speech_out
            );
            assert_eq!(
                false_dels, 0,
                "{} missing required tokens {:?} in {:?}",
                label, required_tokens, speech_out
            );
        }
    }

    // 4. Verify Cached WhisperState Isolation (no_context = true) & Direct f32 vs WAV Parity
    //    After running long speech and silence above, re-transcribe short_jfk_3.0s via both
    //    WAV round-trip and direct 16kHz f32 samples and assert identical 0.00% WER output.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let provider = LocalAiProvider::with_config(
        "custom".to_string(),
        Some(model_path.to_string_lossy().to_string()),
        threads,
    );
    let short_3s = jfk_samples[..(30 * 16_000 / 10).min(jfk_samples.len())].to_vec();
    let short_48k = upsample_16k_to_48k(&short_3s);
    let mut rec_wav = AudioRecorder::default();
    rec_wav.push_test_samples(&short_48k, 48_000);
    let wav_bytes = rec_wav.stop().expect("stop wav");

    let mut rec_f32 = AudioRecorder::default();
    rec_f32.push_test_samples(&short_48k, 48_000);
    let (samples_16k, reused_none) = rec_f32
        .stop_recording_samples_16k(None)
        .expect("stop samples 16k");
    assert!(!reused_none);

    let out_from_wav = rt
        .block_on(provider.transcribe(wav_bytes, Some("en")))
        .expect("transcribe from wav");
    let out_from_f32 = rt
        .block_on(provider.transcribe_samples(samples_16k, Some("en")))
        .expect("transcribe from f32");
    assert_eq!(
        out_from_f32, out_from_wav,
        "Direct 16kHz f32 transcription must match WAV transcription"
    );
    let (dist_short, _, wer_short) = compute_wer("And so my fellow Americans", &out_from_f32);
    assert_eq!(
        dist_short, 0,
        "Cached WhisperState reuse must have 0.00% WER without prior context bleed, got {:?} (WER {})",
        out_from_f32, wer_short
    );

    // 5. Verify Streaming Pause Finalization & Stop-Reuse Accuracy
    //    Simulate 3.0s speech + 1.35s quiet pause (3 ticks of 450ms):
    //    - Speech ticks must transcribe live
    //    - Once trailing 200ms is quiet, snap.tail_is_quiet becomes true and subsequent silent ticks return None
    //    - stop_recording_samples_16k(opt_finalized_len) returns reused=true with 0.00% WER
    let mut rec_stream = AudioRecorder::default();
    let snap_stream = rec_stream.snapshot_handle();
    let silence_48k = deterministic_noise(64_800, 0.0003); // 1.35s quiet pause @ 48kHz (3 ticks)
    let mut session_48k = short_48k.clone();
    session_48k.extend_from_slice(&silence_48k);
    let tick_48k = 21_600usize; // 450ms @ 48kHz
    let mut last_len = 0usize;
    let mut prev_finalized = false;
    let mut finalized_len: Option<usize> = None;
    let mut cached_partial = String::new();
    let mut silent_ticks_skipped = 0usize;
    let mut offset = 0usize;
    while offset < session_48k.len() {
        let end = (offset + tick_48k).min(session_48k.len());
        rec_stream.push_test_samples(&session_48k[offset..end], 48_000);
        offset = end;
        match snap_stream
            .snapshot_samples_16k(60.0, last_len, prev_finalized)
            .expect("snapshot_samples_16k")
        {
            Some(snap) => {
                last_len = snap.total_native_samples;
                if !snap.samples_16k.is_empty() {
                    let partial = rt
                        .block_on(provider.transcribe_samples(snap.samples_16k, Some("en")))
                        .expect("stream partial transcribe");
                    if !partial.trim().is_empty() {
                        prev_finalized = snap.tail_is_quiet;
                        finalized_len = if snap.tail_is_quiet {
                            Some(snap.total_native_samples)
                        } else {
                            None
                        };
                        cached_partial = partial;
                    }
                } else {
                    silent_ticks_skipped += 1;
                }
            }
            None => {
                silent_ticks_skipped += 1;
            }
        }
    }
    assert!(
        prev_finalized && finalized_len.is_some(),
        "Streaming session must mark partial transcript finalized after quiet tail"
    );
    assert!(
        silent_ticks_skipped >= 2,
        "Sustained silent pause must skip redundant Whisper passes, skipped={}",
        silent_ticks_skipped
    );
    let (_final_samples, reused_on_stop) = rec_stream
        .stop_recording_samples_16k(finalized_len)
        .expect("stop_recording_samples_16k");
    assert!(
        reused_on_stop,
        "Stop after silent pause must reuse finalized streaming transcript"
    );
    let (stream_dist, _, _) = compute_wer("And so my fellow Americans", &cached_partial);
    assert_eq!(
        stream_dist, 0,
        "Finalized streaming transcript must have 0.00% WER, got {:?}",
        cached_partial
    );

    // 6. Real-Speech Pause-and-Resume: 3.0s speech -> 16.0s silence -> short word spoken immediately before Stop
    let mut rec_resume = AudioRecorder::default();
    let snap_resume = rec_resume.snapshot_handle();
    rec_resume.push_test_samples(&short_48k, 48_000);
    rec_resume.push_test_samples(&deterministic_noise(12_000, 0.0003), 48_000); // 250ms quiet tail
    let snap_first = snap_resume
        .snapshot_samples_16k(60.0, 0, false)
        .expect("initial resume snap")
        .expect("initial resume speech");
    assert!(snap_first.tail_is_quiet);
    let resume_finalized_at = Some(snap_first.total_native_samples);
    // 16 seconds of silence (768,000 samples @ 48 kHz)
    rec_resume.push_test_samples(&deterministic_noise(16 * 48_000, 0.0003), 48_000);
    // Speak a short phrase (first 2.1s of short_48k: "And so my fellow Americans") immediately before pressing Stop
    rec_resume.push_test_samples(&short_48k[..100_800], 48_000);
    let (resumed_samples_16k, reused_after_resume) = rec_resume
        .stop_recording_samples_16k(resume_finalized_at)
        .expect("stop after 16s pause + spoken word");
    assert!(
        !reused_after_resume,
        "Stop after 16s silence + spoken word MUST NOT reuse stale finalized transcript"
    );
    let resumed_transcript = rt
        .block_on(provider.transcribe_samples(resumed_samples_16k, Some("en")))
        .expect("transcribe resumed session");
    let resumed_lower = resumed_transcript.to_ascii_lowercase();
    let americans_count = resumed_lower.matches("americans").count();
    assert!(
        americans_count >= 2,
        "Final transcript after 16s pause + spoken 'Americans' before Stop must include both utterances, got {:?}",
        resumed_transcript
    );
}

/// Returns total process CPU time (user + system across all threads) in milliseconds
/// using POSIX `clock_gettime(CLOCK_PROCESS_CPUTIME_ID)`.
fn process_cpu_time_ms() -> f64 {
    #[repr(C)]
    struct Timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    unsafe extern "C" {
        fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
    }
    const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let rc = unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut ts) };
    if rc == 0 {
        (ts.tv_sec as f64) * 1000.0 + (ts.tv_nsec as f64) / 1_000_000.0
    } else {
        0.0
    }
}

/// Returns `(VmRSS_MB, VmHWM_MB)` from `/proc/self/status`.
fn process_memory_mb() -> (f64, f64) {
    let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
        return (0.0, 0.0);
    };
    let mut rss_kb = 0u64;
    let mut hwm_kb = 0u64;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            rss_kb = rest
                .split_whitespace()
                .next()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        } else if let Some(rest) = line.strip_prefix("VmHWM:") {
            hwm_kb = rest
                .split_whitespace()
                .next()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
        }
    }
    (rss_kb as f64 / 1024.0, hwm_kb as f64 / 1024.0)
}

/// Linearly upsamples 16 kHz mono samples to 48 kHz to simulate hardware microphone capture
/// feeding `AudioBufferSnapshot`.
fn upsample_16k_to_48k(samples_16k: &[f32]) -> Vec<f32> {
    if samples_16k.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(samples_16k.len() * 3);
    for i in 0..samples_16k.len() {
        let s0 = samples_16k[i];
        let s1 = samples_16k.get(i + 1).copied().unwrap_or(s0);
        out.push(s0);
        out.push(s0 + (s1 - s0) * (1.0 / 3.0));
        out.push(s0 + (s1 - s0) * (2.0 / 3.0));
    }
    out
}

#[test]
#[ignore = "Detailed benchmark matrix run explicitly with --ignored --nocapture"]
fn test_detailed_whisper_benchmark_matrix() {
    use opendictate::audio::recorder::normalize_speech_samples;
    use opendictate::services::ai::AiProvider;

    let model_env = std::env::var("BENCH_MODEL_PATH").ok();
    let model_path = if let Some(ref custom) = model_env {
        std::path::PathBuf::from(custom)
    } else {
        resolve_model_path("tiny.en", None)
    };
    assert!(
        model_path.is_file(),
        "Model file does not exist: {:?}",
        model_path
    );

    let jfk_bytes = std::fs::read("tests/fixtures/jfk.wav").expect("jfk.wav must exist");
    let jfk_samples =
        opendictate::services::ai::local_ai::decode_wav_to_f32(&jfk_bytes).expect("decode jfk");

    let sd_bytes = std::fs::read("/usr/share/sounds/speech-dispatcher/dummy-message.wav")
        .expect("dummy-message.wav must exist");
    let sd_samples =
        opendictate::services::ai::local_ai::decode_wav_to_f32(&sd_bytes).expect("decode sd");

    // Identical Short, Medium, and Long recordings & reference transcripts
    let jfk_short_3s = jfk_samples[..(30 * 16_000 / 10).min(jfk_samples.len())].to_vec();
    let jfk_med_7_9s = jfk_samples[..(79 * 16_000 / 10).min(jfk_samples.len())].to_vec();
    let jfk_long_11s = jfk_samples.clone();
    let mut jfk_long_22s = jfk_samples.clone();
    jfk_long_22s.extend_from_slice(&jfk_samples);

    let sd_short_2_2s = sd_samples[..(22 * 16_000 / 10).min(sd_samples.len())].to_vec();
    let sd_med_7_8s = sd_samples[..(78 * 16_000 / 10).min(sd_samples.len())].to_vec();
    let sd_long_11_2s = sd_samples[..(112 * 16_000 / 10).min(sd_samples.len())].to_vec();

    let fixtures: Vec<(&str, Vec<f32>, &str)> = vec![
        ("short_jfk_3.0s", jfk_short_3s, "And so my fellow Americans"),
        (
            "short_sd_2.2s",
            sd_short_2_2s,
            "This is the dummy output module.",
        ),
        (
            "medium_jfk_7.9s",
            jfk_med_7_9s,
            "And so my fellow Americans, ask not what your country can do for you",
        ),
        (
            "medium_sd_7.8s",
            sd_med_7_8s,
            "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me.",
        ),
        (
            "long_jfk_11.0s",
            jfk_long_11s,
            "And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country.",
        ),
        (
            "long_sd_11.2s",
            sd_long_11_2s,
            "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me. Please check out your log file to see what the problem is.",
        ),
        (
            "long_jfk_22.0s",
            jfk_long_22s,
            "And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country. And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country.",
        ),
    ];

    let (rss_before, _) = process_memory_mb();
    let warmup_noise = deterministic_noise(16_000, 0.0005);
    let t_cold0 = Instant::now();
    let _ = LocalAiProvider::transcribe_wav_samples(&warmup_noise, &model_path, 4)
        .expect("Cold start should succeed");
    let cold_start_ms = t_cold0.elapsed().as_secs_f64() * 1000.0;
    let (rss_after_cold, hwm_after_cold) = process_memory_mb();

    println!("================================================================================");
    println!("SYSTEM_INFO: {}", local_speech_engine::print_system_info().trim());
    println!("MODEL: {:?}", model_path);
    println!(
        "COLD_START_MS: {:.2} ms | RSS_BEFORE: {:.1} MB | RSS_AFTER: {:.1} MB | PEAK_HWM: {:.1} MB",
        cold_start_ms, rss_before, rss_after_cold, hwm_after_cold
    );
    println!("================================================================================");

    let thread_counts: Vec<u32> = if let Ok(t_str) = std::env::var("BENCH_THREADS") {
        t_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    } else {
        vec![8, 6, 4]
    };

    let iters: usize = std::env::var("BENCH_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
        .max(1);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio runtime");

    for &threads in &thread_counts {
        println!("\n--- THREAD COUNT: {} (iters={}) ---", threads, iters);
        println!(
            "{:<18} | {:>6} | {:>10} | {:>10} | {:>8} | {:>9} | {:>8} | {:>8}",
            "Clip",
            "Sec",
            "Warm(ms)",
            "E2E(ms)",
            "WER(%)",
            "Edits",
            "CPU(%)",
            "RSS(MB)"
        );
        println!("{}", "-".repeat(96));

        for (label, samples_16k, reference) in &fixtures {
            let duration_s = samples_16k.len() as f64 / 16_000.0;

            // 1. Measure Warm Inference
            let mut warm_times = Vec::with_capacity(iters);
            let mut cpu_pcts = Vec::with_capacity(iters);
            let mut last_transcript = String::new();
            for _ in 0..iters {
                let cpu0 = process_cpu_time_ms();
                let t0 = Instant::now();
                let out =
                    LocalAiProvider::transcribe_wav_samples(samples_16k, &model_path, threads)
                        .expect("Warm inference should succeed");
                let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
                let cpu_ms = process_cpu_time_ms() - cpu0;
                let cpu_pct_12t = (cpu_ms / wall_ms.max(0.001)) * 100.0 / 12.0;
                warm_times.push(wall_ms);
                cpu_pcts.push(cpu_pct_12t);
                last_transcript = out;
            }
            warm_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            cpu_pcts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let warm_median_ms = warm_times[warm_times.len() / 2];
            let cpu_median_pct = cpu_pcts[cpu_pcts.len() / 2];

            // 2. Measure End-to-End Latency (48kHz raw buffer -> resample_to_16k -> normalize -> WAV -> transcribe -> enhance)
            let samples_48k = upsample_16k_to_48k(samples_16k);
            let provider = LocalAiProvider::with_config(
                "custom".to_string(),
                Some(model_path.to_string_lossy().to_string()),
                threads,
            );

            let mut e2e_times = Vec::with_capacity(iters);
            let mut last_e2e_transcript = String::new();
            for _ in 0..iters {
                let t_e2e = Instant::now();
                let resampled = resample_to_16k(&samples_48k, 48_000).expect("resample_to_16k");
                let normalized = normalize_speech_samples(&resampled);
                let raw_text = rt
                    .block_on(provider.transcribe_samples(normalized, Some("en")))
                    .expect("E2E transcribe_samples");
                let _formatted = rt
                    .block_on(provider.enhance(&raw_text, "Clean"))
                    .expect("E2E enhance");
                e2e_times.push(t_e2e.elapsed().as_secs_f64() * 1000.0);
                last_e2e_transcript = raw_text;
            }
            e2e_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let e2e_median_ms = e2e_times[e2e_times.len() / 2];

            let (dist, ref_words, wer) = compute_wer(reference, &last_transcript);
            let (e2e_dist, e2e_ref_words, e2e_wer) = compute_wer(reference, &last_e2e_transcript);
            let (rss_mb, _hwm_mb) = process_memory_mb();

            println!(
                "{:<18} | {:>5.1}s | {:>10.1} | {:>10.1} | {:>7.2}% | {:>4}/{:<4} | {:>7.1}% | {:>7.1}",
                label,
                duration_s,
                warm_median_ms,
                e2e_median_ms,
                wer * 100.0,
                dist,
                ref_words,
                cpu_median_pct,
                rss_mb
            );
            if std::env::var("BENCH_VERBOSE").is_ok() {
                println!("   -> warm transcript: {:?}", last_transcript);
                println!(
                    "   -> e2e  transcript (WER {:.2}%, {}/{}): {:?}",
                    e2e_wer * 100.0,
                    e2e_dist,
                    e2e_ref_words,
                    last_e2e_transcript
                );
            }
        }
    }

    if std::env::var("BENCH_STREAMING").is_ok() {
        use opendictate::audio::recorder::AudioRecorder;
        println!("\n=== STREAMING PARTIAL-TRANSCRIPTION PIPELINE BENCHMARK (3.0s speech + 2.25s pause before Stop @ 48kHz, 450ms ticks) ===");
        let provider = LocalAiProvider::with_config(
            "custom".to_string(),
            Some(model_path.to_string_lossy().to_string()),
            6,
        );
        let speech_48k = upsample_16k_to_48k(&fixtures[0].1); // 3.0s JFK @ 48kHz (144,000 samples)
        let silence_48k = deterministic_noise(108_000, 0.0003); // 2.25s quiet pause @ 48kHz
        let mut full_session_48k = speech_48k.clone();
        full_session_48k.extend_from_slice(&silence_48k);
        let tick_samples_48k = 21_600usize; // 450ms @ 48kHz

        // 1. Legacy Streaming Pipeline (snapshot_wav on every tick + unconditional final stop() + transcribe(wav))
        let mut rec_legacy = AudioRecorder::default();
        let snap_legacy = rec_legacy.snapshot_handle();
        let cpu_leg0 = process_cpu_time_ms();
        let t_leg0 = Instant::now();
        let mut leg_last_len = 0usize;
        let mut leg_passes = 0usize;
        let mut offset = 0usize;
        while offset < full_session_48k.len() {
            let end = (offset + tick_samples_48k).min(full_session_48k.len());
            rec_legacy.push_test_samples(&full_session_48k[offset..end], 48_000);
            offset = end;
            if let Ok(Some((wav_bytes, total_len))) = snap_legacy.snapshot_wav(60.0)
                && total_len > leg_last_len
            {
                leg_last_len = total_len;
                let _ = rt
                    .block_on(provider.transcribe(wav_bytes, Some("en")))
                    .expect("legacy stream transcribe");
                leg_passes += 1;
            }
        }
        let t_leg_stop0 = Instant::now();
        let final_wav = rec_legacy.stop().expect("legacy stop");
        let leg_final_text = rt
            .block_on(provider.transcribe(final_wav, Some("en")))
            .expect("legacy final transcribe");
        leg_passes += 1;
        let leg_stop_e2e_ms = t_leg_stop0.elapsed().as_secs_f64() * 1000.0;
        let leg_total_wall_ms = t_leg0.elapsed().as_secs_f64() * 1000.0;
        let leg_total_cpu_ms = process_cpu_time_ms() - cpu_leg0;

        // 2. Optimized Streaming Pipeline (snapshot_samples_16k + silence-aware finalization + stop_recording_samples_16k reuse)
        let mut rec_opt = AudioRecorder::default();
        let snap_opt = rec_opt.snapshot_handle();
        let cpu_opt0 = process_cpu_time_ms();
        let t_opt0 = Instant::now();
        let mut opt_last_len = 0usize;
        let mut opt_finalized = false;
        let mut opt_finalized_len: Option<usize> = None;
        let mut opt_cached_text = String::new();
        let mut opt_passes = 0usize;
        let mut offset = 0usize;
        while offset < full_session_48k.len() {
            let end = (offset + tick_samples_48k).min(full_session_48k.len());
            rec_opt.push_test_samples(&full_session_48k[offset..end], 48_000);
            offset = end;
            if let Ok(Some(snap)) =
                snap_opt.snapshot_samples_16k(60.0, opt_last_len, opt_finalized)
            {
                opt_last_len = snap.total_native_samples;
                if snap.samples_16k.is_empty() {
                    continue;
                }
                let text = rt
                    .block_on(provider.transcribe_samples(snap.samples_16k, Some("en")))
                    .expect("opt stream transcribe");
                opt_passes += 1;
                if !text.trim().is_empty() {
                    opt_finalized = snap.tail_is_quiet;
                    opt_finalized_len = if snap.tail_is_quiet {
                        Some(snap.total_native_samples)
                    } else {
                        None
                    };
                    opt_cached_text = text;
                }
            }
        }
        let t_opt_stop0 = Instant::now();
        let (final_samples_16k, reused) = rec_opt
            .stop_recording_samples_16k(opt_finalized_len)
            .expect("opt stop");
        let opt_final_text = if reused && !opt_cached_text.is_empty() {
            opt_cached_text.clone()
        } else {
            opt_passes += 1;
            rt.block_on(provider.transcribe_samples(final_samples_16k, Some("en")))
                .expect("opt final transcribe")
        };
        let opt_stop_e2e_ms = t_opt_stop0.elapsed().as_secs_f64() * 1000.0;
        let opt_total_wall_ms = t_opt0.elapsed().as_secs_f64() * 1000.0;
        let opt_total_cpu_ms = process_cpu_time_ms() - cpu_opt0;

        println!(
            "LEGACY STREAMING   : passes={} | total_compute={:.1} ms | total_cpu={:.1} ms | stop_to_text_latency={:.2} ms | text={:?}",
            leg_passes, leg_total_wall_ms, leg_total_cpu_ms, leg_stop_e2e_ms, leg_final_text
        );
        println!(
            "OPTIMIZED STREAMING: passes={} | total_compute={:.1} ms | total_cpu={:.1} ms | stop_to_text_latency={:.2} ms | reused_final={} | text={:?}",
            opt_passes, opt_total_wall_ms, opt_total_cpu_ms, opt_stop_e2e_ms, reused, opt_final_text
        );
    }
}
