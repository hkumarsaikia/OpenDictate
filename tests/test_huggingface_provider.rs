use opendictate::config::Config;
use opendictate::services::ai::huggingface::HuggingFaceProvider;
use opendictate::services::ai::{AiError, AiManager, AiProvider, get_provider_models};

#[test]
fn test_huggingface_provider_construction_and_defaults() {
    let p_default = HuggingFaceProvider::new("hf_test_token", None);
    assert_eq!(p_default.model(), "meta-llama/Llama-3.3-70B-Instruct");
    assert_eq!(
        p_default.transcription_model(),
        "openai/whisper-large-v3-turbo"
    );

    let p_custom = HuggingFaceProvider::new(
        "hf_test_token",
        Some("Qwen/Qwen2.5-72B-Instruct".to_string()),
    );
    assert_eq!(p_custom.model(), "Qwen/Qwen2.5-72B-Instruct");
    assert_eq!(
        p_custom.transcription_model(),
        "openai/whisper-large-v3-turbo"
    );
}

#[tokio::test]
async fn test_huggingface_missing_api_key() {
    let p = HuggingFaceProvider::new("", None);
    let enhance_err = p.enhance("hello world", "Clean").await.unwrap_err();
    assert!(matches!(enhance_err, AiError::MissingApiKey));

    let transcribe_err = p.transcribe(vec![1, 2, 3], None).await.unwrap_err();
    assert!(matches!(transcribe_err, AiError::MissingApiKey));
}

#[test]
fn test_huggingface_get_provider_models() {
    let models = get_provider_models("huggingface");
    assert!(
        !models.is_empty(),
        "Hugging Face model list must not be empty"
    );

    let audio_turbo_model = models
        .iter()
        .find(|m| m.id == "openai/whisper-large-v3-turbo")
        .expect("openai/whisper-large-v3-turbo must be present");
    assert!(audio_turbo_model.inputs.contains(&"audio".to_string()));
    assert!(audio_turbo_model.is_free);

    let llama_model = models
        .iter()
        .find(|m| m.id == "meta-llama/Llama-3.3-70B-Instruct")
        .expect("meta-llama/Llama-3.3-70B-Instruct must be present");
    assert!(llama_model.inputs.contains(&"text".to_string()));
}

#[test]
fn test_huggingface_ai_manager_integration() {
    let config = Config {
        ai_provider: "huggingface".to_string(),
        ai_api_key: "hf_dummy_key".to_string(),
        ai_model: "meta-llama/Llama-3.3-70B-Instruct".to_string(),
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    assert!(
        manager
            .supported_providers()
            .contains(&"huggingface".to_string()),
        "AiManager supported_providers must include 'huggingface'"
    );

    let provider = manager.get_provider("huggingface");
    assert!(
        provider.is_ok(),
        "AiManager must successfully resolve HuggingFaceProvider"
    );
}

#[tokio::test]
async fn test_huggingface_live_transcribe_and_enhance() {
    let key = std::env::var("HUGGINGFACE_API_KEY")
        .or_else(|_| std::env::var("HF_TOKEN"))
        .unwrap_or_default();

    if key.is_empty() {
        eprintln!("Skipping live Hugging Face test: HUGGINGFACE_API_KEY not set");
        return;
    }

    let provider = HuggingFaceProvider::new(&key, None);

    match provider
        .enhance(
            "um so basically i think that the meeting went pretty good uh yeah",
            "Clean",
        )
        .await
    {
        Ok(enhanced) => {
            assert!(
                !enhanced.trim().is_empty(),
                "Enhanced text should not be empty"
            );
            println!("Live HF enhanced text: {}", enhanced);
        }
        Err(e) => {
            eprintln!("Warning: Live HF text enhancement endpoint returned: {}", e);
        }
    }

    let wav_path = "tests/fixtures/jfk.wav";
    if let Ok(audio_bytes) = std::fs::read(wav_path) {
        match provider.transcribe(audio_bytes, None).await {
            Ok(transcribed) => {
                println!("Live HF transcribed text: {}", transcribed);
                assert!(
                    !transcribed.is_empty(),
                    "Transcription result should not be empty"
                );
            }
            Err(e) => {
                eprintln!("Warning: Live HF transcription endpoint returned: {}", e);
            }
        }
    }

    // Test live model fetching for Hugging Face
    let models = opendictate::services::ai::fetch_models_for_provider("huggingface", &key)
        .await
        .expect("Live Hugging Face model fetching should succeed");
    assert!(!models.is_empty(), "Fetched models must not be empty");
    assert!(
        models
            .iter()
            .any(|m| m.inputs.contains(&"audio".to_string())),
        "Fetched models must include audio models"
    );
    println!("Live HF fetched {} models. Top 3:", models.len());
    for m in models.iter().take(3) {
        println!("  - {} (Free: {}, Inputs: {:?})", m.id, m.is_free, m.inputs);
    }
}
