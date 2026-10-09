//! Reproducible baseline and regression benchmark suite for speech transcription,
//! resampling fidelity, hallucination suppression, and conservative local formatting.

use opendictate::audio::recorder::resample_to_16k;
use opendictate::services::ai::local_ai::{
    LocalAiProvider, clean_whisper_segment, resolve_model_path,
};
use opendictate::services::ai::utils::apply_smart_local_formatting;
use std::time::Instant;

/// Computes word-level Levenshtein distance (Substitutions + Deletions + Insertions)
/// and Word Error Rate (WER = edit_distance / reference_word_count).
fn compute_wer(reference: &str, hypothesis: &str) -> (usize, usize, f64) {
    let normalize = |s: &str| -> Vec<String> {
        s.split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != ':')
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
    let threads = 4u32;

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
        "[BENCHMARK] Whisper tiny.en (threads=4): cold_start={:.1} ms, silence_2s={:.1} ms, out={:?}",
        cold_start_ms, silence_ms, silence_out
    );
    assert!(
        silence_out.is_empty(),
        "2s low-noise silence must not hallucinate text, got: {:?}",
        silence_out
    );

    // 2. Measure short (3s), medium (8s), and long (22s) inference latency
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
            "[BENCHMARK] Whisper {} ({}s audio @ 16kHz, threads=4): latency={:.1} ms, out={:?}",
            label, seconds, elapsed_ms, out
        );
        assert!(
            out.is_empty(),
            "{} silence vector must not hallucinate text, got: {:?}",
            label,
            out
        );
    }

    // 3. Real spoken English WAV sample benchmark across short (4s), medium (10s), and long (29.4s) slices
    let speech_wav_path =
        std::path::Path::new("/usr/share/sounds/speech-dispatcher/dummy-message.wav");
    if speech_wav_path.is_file()
        && let Ok(wav_bytes) = std::fs::read(speech_wav_path)
        && let Ok(speech_samples) =
            opendictate::services::ai::local_ai::decode_wav_to_f32(&wav_bytes)
    {
        let slices: &[(&str, usize, &str)] = &[
            (
                "spoken_short_2.2s",
                (22 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module.",
            ),
            (
                "spoken_medium_7.8s",
                (78 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me.",
            ),
            (
                "spoken_long_12.5s",
                (125 * 16_000 / 10).min(speech_samples.len()),
                "This is the dummy output module. It seems your speech dispatcher is working, but none of its output modules is, except me. Please check out your log file to see what the problem is.",
            ),
        ];

        for &(label, len, reference) in slices {
            let slice = &speech_samples[..len];
            let audio_secs = slice.len() as f64 / 16_000.0;
            let t_speech = Instant::now();
            let speech_out = LocalAiProvider::transcribe_wav_samples(slice, &model_path, threads)
                .expect("Spoken WAV inference should succeed");
            let speech_ms = t_speech.elapsed().as_secs_f64() * 1000.0;
            let (dist, ref_words, wer) = compute_wer(reference, &speech_out);
            println!(
                "[BENCHMARK] {} ({:.2}s @ 16kHz): latency={:.1} ms, WER={:.4} (edits={}/{}), transcript={:?}",
                label, audio_secs, speech_ms, wer, dist, ref_words, speech_out
            );
            assert!(
                !speech_out.is_empty(),
                "{} must produce a non-empty transcript",
                label
            );
        }
    }
}
