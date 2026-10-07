use opendictate::services::ai::local_ai::*;
use opendictate::services::ai::utils::apply_smart_local_formatting;
use opendictate::services::ai::{AiError, AiProvider};
use std::path::PathBuf;

#[test]
fn test_local_model_catalog() {
    let catalog = get_local_model_catalog();
    assert!(!catalog.is_empty());
    let tiny_en = catalog
        .iter()
        .find(|m| m.id == "tiny.en")
        .expect("tiny.en in catalog");
    assert_eq!(tiny_en.filename, "ggml-tiny.en.bin");
    assert!(tiny_en.url.contains("huggingface.co"));
    assert_eq!(tiny_en.parameters, "39M");
    assert_eq!(tiny_en.ram_mb, 273);
    assert!(tiny_en.languages.contains("English"));
    assert!(tiny_en.speed.contains("real-time"));
    assert!(tiny_en.license.contains("MIT"));
    assert!(tiny_en.source.contains("whisper.cpp"));

    let base = catalog
        .iter()
        .find(|m| m.id == "base")
        .expect("base in catalog");
    assert_eq!(base.parameters, "74M");
    assert_eq!(base.ram_mb, 388);
    assert!(base.languages.contains("99"));
}

#[test]
fn test_local_ai_auto_thread_handling() {
    let provider = LocalAiProvider::with_config("tiny.en".to_string(), None, 0);
    assert_eq!(provider.threads, 0);
}

#[test]
fn test_smart_local_formatting() {
    let raw = "um hello world   this is a test  uh you know ";
    let clean = apply_smart_local_formatting(raw, "Clean");
    assert_eq!(clean, "Hello world this is a test.");

    let raw_passthrough = apply_smart_local_formatting(raw, "Raw");
    assert_eq!(raw_passthrough, raw);
}

#[test]
fn test_smart_local_formatting_fillers_and_preservation() {
    // Legitimate words containing filler substrings shouldn't be altered
    let raw = "the umbrella in summer is yellow";
    let clean = apply_smart_local_formatting(raw, "Clean");
    assert_eq!(clean, "The umbrella in summer is yellow.");

    // Multiple sentences capitalization and ending punctuation
    let multi = "first sentence. second sentence? third sentence! and fourth";
    let clean_multi = apply_smart_local_formatting(multi, "Clean");
    assert_eq!(
        clean_multi,
        "First sentence. Second sentence? Third sentence! And fourth."
    );
}

#[test]
fn test_wav_to_f32_conversion() {
    // Generate valid 16kHz mono WAV bytes
    let mut buffer = Vec::new();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    {
        let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec).unwrap();
        writer.write_sample(0i16).unwrap();
        writer.write_sample(16384i16).unwrap();
        writer.write_sample(-16384i16).unwrap();
        writer.finalize().unwrap();
    }

    let samples = decode_wav_to_f32(&buffer).expect("Decode WAV");
    assert_eq!(samples.len(), 3);
    assert!((samples[0] - 0.0).abs() < 1e-3);
    assert!((samples[1] - 0.5).abs() < 1e-3);
    assert!((samples[2] - (-0.5)).abs() < 1e-3);
}

#[test]
fn test_wav_to_f32_8bit_conversion() {
    let mut buffer = Vec::new();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 8,
        sample_format: hound::SampleFormat::Int,
    };
    {
        let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec).unwrap();
        // 8-bit unsigned PCM: 128 is center/silence (0.0), 255 is +0.992, 0 is -1.0
        writer.write_sample(128u8 as i8).unwrap();
        writer.write_sample(255u8 as i8).unwrap();
        writer.write_sample(0u8 as i8).unwrap();
        writer.finalize().unwrap();
    }

    let samples = decode_wav_to_f32(&buffer).expect("Decode 8-bit WAV");
    assert_eq!(samples.len(), 3);
    assert!((samples[0] - 0.0).abs() < 1e-2);
    assert!((samples[1] - 0.992).abs() < 1e-2);
    assert!((samples[2] - (-1.0)).abs() < 1e-2);
}

#[test]
fn test_wav_to_f32_24bit_conversion() {
    let mut buffer = Vec::new();
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };
    {
        let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec).unwrap();
        // 24-bit signed PCM in i32: 0 is center (0.0), 4194304 is 0.5, -4194304 is -0.5
        writer.write_sample(0i32).unwrap();
        writer.write_sample(4194304i32).unwrap();
        writer.write_sample(-4194304i32).unwrap();
        writer.finalize().unwrap();
    }

    let samples = decode_wav_to_f32(&buffer).expect("Decode 24-bit WAV");
    assert_eq!(samples.len(), 3);
    assert!((samples[0] - 0.0).abs() < 1e-3);
    assert!((samples[1] - 0.5).abs() < 1e-3);
    assert!((samples[2] - (-0.5)).abs() < 1e-3);
}

#[test]
fn test_resolve_model_path() {
    let path = resolve_model_path("tiny.en", None);
    assert!(path.ends_with("ggml-tiny.en.bin"));

    let custom = resolve_model_path("custom", Some("/tmp/custom-model.bin"));
    assert_eq!(custom, PathBuf::from("/tmp/custom-model.bin"));
}

#[test]
fn test_is_model_installed() {
    // A non-existent model ID should not be installed
    assert!(!is_model_installed("non_existent_model_id_xyz", None));
}

#[tokio::test]
async fn test_local_ai_provider_not_installed_error() {
    let provider = LocalAiProvider::new(Some("non_existent_model".to_string()));
    let audio = vec![1, 2, 3, 4];
    let err = provider.transcribe(audio, None).await.unwrap_err();
    match err {
        AiError::ApiError(msg) => {
            assert!(msg.contains("not installed"));
            assert!(msg.contains("download"));
        }
        other => panic!("Expected ApiError, got {:?}", other),
    }
}

#[tokio::test]
async fn test_local_ai_provider_empty_audio() {
    let provider = LocalAiProvider::new(Some("tiny.en".to_string()));
    let result = provider.transcribe(vec![], None).await.unwrap();
    assert_eq!(result, "");
}

#[tokio::test]
async fn test_local_ai_provider_enhance_and_list() {
    let provider = LocalAiProvider::new(None);
    let enhanced = provider
        .enhance("um hello world uh", "Clean")
        .await
        .unwrap();
    assert_eq!(enhanced, "Hello world.");

    let models = provider.list_models().await.unwrap();
    assert!(models.contains(&"tiny.en".to_string()));
    assert!(models.contains(&"base".to_string()));
}

#[tokio::test]
async fn test_download_local_model_invalid_id() {
    let result = download_local_model("nonexistent_model_123", |_, _| {}).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AiError::ApiError(msg) => assert!(msg.contains("not found in catalog")),
        other => panic!("Expected ApiError, got {:?}", other),
    }
}

#[test]
fn test_decode_wav_empty() {
    let samples = decode_wav_to_f32(&[]).unwrap();
    assert!(samples.is_empty());
}

#[tokio::test]
async fn test_real_local_ai_transcription() {
    if !is_model_installed("tiny.en", None) {
        eprintln!("Skipping test_real_local_ai_transcription: model not installed");
        return;
    }

    let provider = LocalAiProvider::new(Some("tiny.en".to_string()));
    let jfk_path = "tests/fixtures/jfk.wav";
    let wav_path = if std::path::Path::new(jfk_path).exists() {
        jfk_path
    } else {
        "recordings/meeting_1790413136_Level_Test.wav"
    };
    let audio_bytes = std::fs::read(wav_path).expect("Failed to read sample WAV");
    assert!(!audio_bytes.is_empty());

    let result = provider
        .transcribe(audio_bytes.clone(), None)
        .await
        .expect("Local AI provider transcription should succeed");
    println!("Local AI transcribed text: '{}'", result);
    assert!(
        !result.is_empty(),
        "Transcription result should not be empty"
    );

    if wav_path == jfk_path {
        let lower = result.to_lowercase();
        assert!(
            lower.contains("fellow americans") || lower.contains("country"),
            "Expected speech transcription to contain recognized words, got: {}",
            result
        );
    }

    // Also test end-to-end AiManager local routing
    let config = opendictate::config::Config {
        ai_mode: "local".to_string(),
        local_model_id: "tiny.en".to_string(),
        local_threads: 4,
        ..Default::default()
    };

    let manager = opendictate::services::ai::AiManager::with_config(&config);
    assert_eq!(manager.active_mode(), "local");

    let mgr_result = manager
        .transcribe(&audio_bytes)
        .await
        .expect("AiManager local transcription should succeed");
    println!("AiManager transcribed text: '{}'", mgr_result);
    assert_eq!(result, mgr_result);

    // Test enhance in local mode
    let enhanced = manager
        .enhance("um hello world uh", "Clean")
        .await
        .expect("Enhance should succeed");
    assert_eq!(enhanced, "Hello world.");
}

#[tokio::test]
async fn test_unlisted_custom_model_transcription() {
    let custom_model_path = "/tmp/custom_models/ggml-tiny.en-q5_1.bin";
    if !std::path::Path::new(custom_model_path).exists() {
        eprintln!("Skipping test_unlisted_custom_model_transcription: model file not found");
        return;
    }

    // Verify this model is NOT listed in the app catalog
    let catalog = get_local_model_catalog();
    assert!(
        !catalog
            .iter()
            .any(|m| m.filename == "ggml-tiny.en-q5_1.bin"),
        "The model ggml-tiny.en-q5_1.bin must NOT be in the built-in catalog"
    );

    // Initialize LocalAiProvider with custom model and threads = 0 (Auto / System default)
    let provider = LocalAiProvider::with_config(
        "custom".to_string(),
        Some(custom_model_path.to_string()),
        0, // 0 = Auto / System default
    );
    assert_eq!(provider.threads, 0);

    let jfk_path = "tests/fixtures/jfk.wav";
    let wav_path = if std::path::Path::new(jfk_path).exists() {
        jfk_path
    } else {
        "recordings/meeting_1790413136_Level_Test.wav"
    };
    let audio_bytes = std::fs::read(wav_path).expect("Failed to read audio file");
    assert!(!audio_bytes.is_empty());

    let transcribed = provider
        .transcribe(audio_bytes.clone(), None)
        .await
        .expect("Transcription with unlisted custom model must succeed");
    println!("Unlisted custom model transcribed text: '{}'", transcribed);
    assert!(
        !transcribed.is_empty(),
        "Transcribed text must not be empty"
    );

    if wav_path == jfk_path {
        let lower = transcribed.to_lowercase();
        assert!(
            lower.contains("fellow americans") || lower.contains("country"),
            "Expected unlisted custom model to transcribe JFK speech, got: {}",
            transcribed
        );
    }

    // Also verify AiManager routing with custom model path and auto threads
    let config = opendictate::config::Config {
        ai_mode: "local".to_string(),
        local_model_id: "custom".to_string(),
        local_custom_path: Some(custom_model_path.to_string()),
        local_threads: 0,
        ..Default::default()
    };

    let manager = opendictate::services::ai::AiManager::with_config(&config);
    assert_eq!(manager.active_mode(), "local");

    let mgr_transcribed = manager
        .transcribe(&audio_bytes)
        .await
        .expect("AiManager routing to custom unlisted model must succeed");
    assert_eq!(transcribed, mgr_transcribed);
}
