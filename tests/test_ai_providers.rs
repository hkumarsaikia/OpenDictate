use opendictate::config::Config;
use opendictate::services::ai::{
    cerebras::CerebrasProvider,
    claude::ClaudeProvider,
    cohere::CohereProvider,
    gemini::{GeminiContent, GeminiInlineData, GeminiPart, GeminiProvider, GeminiRequest},
    get_provider_models,
    groq::GroqProvider,
    nvidia::NvidiaProvider,
    openai::OpenAiProvider,
    opencode::OpenCodeProvider,
    AiError, AiManager, AiProvider,
};

#[test]
fn test_ai_manager_with_config() {
    let config = Config {
        ai_provider: "gemini".to_string(),
        ai_api_key: "test_key_123".to_string(),
        ai_model: "gemini-2.0-flash".to_string(),
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    assert!(manager.get_provider("gemini").is_ok());
}

#[test]
fn test_ai_manager_supported_providers() {
    let manager = AiManager::new();
    let providers = manager.supported_providers();

    assert_eq!(providers.len(), 14);
    assert!(providers.contains(&"groq".to_string()));
    assert!(providers.contains(&"openai".to_string()));
    assert!(providers.contains(&"claude".to_string()));
    assert!(providers.contains(&"gemini".to_string()));
    assert!(providers.contains(&"mistral".to_string()));
    assert!(providers.contains(&"openrouter".to_string()));
    assert!(providers.contains(&"cloudflare".to_string()));
    assert!(providers.contains(&"nvidia".to_string()));
    assert!(providers.contains(&"cohere".to_string()));
    assert!(providers.contains(&"kilocode".to_string()));
    assert!(providers.contains(&"cerebras".to_string()));
    assert!(providers.contains(&"opencode".to_string()));
    assert!(providers.contains(&"huggingface".to_string()));
    assert!(providers.contains(&"ollama".to_string()));

    // Deepgram, SambaNova, and Local AI must NOT be in supported providers
    assert!(!providers.contains(&"deepgram".to_string()));
    assert!(!providers.contains(&"sambanova".to_string()));
    assert!(!providers.contains(&"local_ai".to_string()));

    // Test get_provider for all supported providers
    for p in &providers {
        assert!(
            manager.get_provider(p).is_ok(),
            "Failed to get provider: {}",
            p
        );
    }

    // Test get_provider with case-insensitivity
    assert!(manager.get_provider("NVIDIA").is_ok());
    assert!(manager.get_provider("Cohere").is_ok());
    assert!(manager.get_provider("Cerebras").is_ok());

    // Test unsupported provider
    let unknown = manager.get_provider("unsupported_provider");
    assert!(unknown.is_err());
    match unknown {
        Err(AiError::UnsupportedOperation(msg)) => {
            assert!(msg.contains("unsupported_provider"));
        }
        _ => panic!("Expected UnsupportedOperation error"),
    }
}

#[test]
fn test_provider_models_listing_and_free_tags() {
    let groq_models = get_provider_models("groq");
    assert!(!groq_models.is_empty());
    assert!(groq_models
        .iter()
        .any(|m| m.is_free && m.display_label().contains("[Free]")));

    let nvidia_models = get_provider_models("nvidia");
    assert!(!nvidia_models.is_empty());
    assert!(nvidia_models.iter().any(|m| m.id.contains("llama-3.3-70b")));

    let cerebras_models = get_provider_models("cerebras");
    assert!(!cerebras_models.is_empty());
    assert!(cerebras_models.iter().all(|m| !m.is_free));

    let opencode_models = get_provider_models("opencode");
    assert!(!opencode_models.is_empty());
    assert!(opencode_models
        .iter()
        .any(|m| m.id.contains("kimi") || m.id.contains("glm")));

    let cohere_models = get_provider_models("cohere");
    assert!(!cohere_models.is_empty());
}

#[test]
fn test_enhancement_prompt_formatting() {
    let input = "um hello world I mean hello there";

    let clean = AiManager::format_enhancement_prompt(input, "Clean");
    assert!(clean.contains(input));
    assert!(clean.to_lowercase().contains("filler"));

    let casual = AiManager::format_enhancement_prompt(input, "Casual");
    assert!(casual.contains(input));
    assert!(casual.to_lowercase().contains("casual"));

    let professional = AiManager::format_enhancement_prompt(input, "Professional");
    assert!(professional.contains(input));
    assert!(
        professional.to_lowercase().contains("professional")
            || professional.to_lowercase().contains("structure")
    );

    let bullets = AiManager::format_enhancement_prompt(input, "Bullet Points");
    assert!(bullets.contains(input));
    assert!(bullets.to_lowercase().contains("bullet"));

    let concise = AiManager::format_enhancement_prompt(input, "Concise");
    assert!(concise.contains(input));
    assert!(concise.to_lowercase().contains("concise"));

    // Case-insensitivity and alternative tone names
    let clean_lower = AiManager::format_enhancement_prompt(input, "clean");
    assert_eq!(clean, clean_lower);

    let message_ready = AiManager::format_enhancement_prompt(input, "message-ready");
    assert_eq!(clean, message_ready);

    // Fallback for unknown tone
    let fallback = AiManager::format_enhancement_prompt(input, "some-random-tone");
    assert!(fallback.contains(input));
}

#[tokio::test]
async fn test_missing_api_key_error_handling() {
    let fake_wav = vec![0u8; 100];

    // Gemini
    let gemini = GeminiProvider::new("", None);
    assert!(matches!(
        gemini.transcribe(fake_wav.clone(), None).await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        gemini.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        gemini.list_models().await,
        Err(AiError::MissingApiKey)
    ));

    // Groq
    let groq = GroqProvider::new("", None);
    assert!(matches!(
        groq.transcribe(fake_wav.clone(), None).await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        groq.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        groq.list_models().await,
        Err(AiError::MissingApiKey)
    ));

    // OpenAI
    let openai = OpenAiProvider::new("", None);
    assert!(matches!(
        openai.transcribe(fake_wav.clone(), None).await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        openai.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        openai.list_models().await,
        Err(AiError::MissingApiKey)
    ));

    // Claude
    let claude = ClaudeProvider::new("", None);
    assert!(matches!(
        claude.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        claude.list_models().await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        claude.transcribe(fake_wav.clone(), None).await,
        Err(AiError::UnsupportedOperation(_))
    ));

    // NVIDIA
    let nvidia = NvidiaProvider::new("", None);
    assert!(matches!(
        nvidia.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        nvidia.list_models().await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        nvidia.transcribe(fake_wav.clone(), None).await,
        Err(AiError::UnsupportedOperation(_))
    ));

    // Cohere
    let cohere = CohereProvider::new("", None);
    assert!(matches!(
        cohere.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        cohere.list_models().await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        cohere.transcribe(fake_wav.clone(), None).await,
        Err(AiError::UnsupportedOperation(_))
    ));

    // Cerebras
    let cerebras = CerebrasProvider::new("", None);
    assert!(matches!(
        cerebras.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        cerebras.list_models().await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        cerebras.transcribe(fake_wav.clone(), None).await,
        Err(AiError::UnsupportedOperation(_))
    ));

    // OpenCode
    let opencode = OpenCodeProvider::new("", None);
    assert!(matches!(
        opencode.enhance("test", "Clean").await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        opencode.list_models().await,
        Err(AiError::MissingApiKey)
    ));
    assert!(matches!(
        opencode.transcribe(fake_wav, None).await,
        Err(AiError::UnsupportedOperation(_))
    ));
}

#[test]
fn test_json_payload_serialization_structures() {
    // Gemini Request Serialization
    let gemini_req = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![
                GeminiPart::InlineData {
                    inline_data: GeminiInlineData {
                        mime_type: "audio/wav".to_string(),
                        data: "dGVzdA==".to_string(),
                    },
                },
                GeminiPart::Text {
                    text: "Transcribe this audio".to_string(),
                },
            ],
        }],
    };

    let serialized = serde_json::to_string(&gemini_req).expect("Failed to serialize GeminiRequest");
    assert!(serialized.contains("\"inlineData\""));
    assert!(serialized.contains("\"mimeType\":\"audio/wav\""));
    assert!(serialized.contains("\"data\":\"dGVzdA==\""));
    assert!(serialized.contains("\"text\":\"Transcribe this audio\""));

    let deserialized: GeminiRequest =
        serde_json::from_str(&serialized).expect("Failed to deserialize GeminiRequest");
    assert_eq!(deserialized.contents.len(), 1);
    assert_eq!(deserialized.contents[0].parts.len(), 2);
}

#[test]
fn test_is_model_free_heuristics() {
    use opendictate::services::ai::is_model_free;

    // Pattern-based free tags
    assert!(is_model_free(
        "openrouter",
        "meta-llama/llama-3.3-70b-instruct:free"
    ));
    assert!(is_model_free("kilocode", "deepseek/deepseek-r1:free"));
    assert!(is_model_free("opencode", "mimo-v2.5-free"));
    assert!(is_model_free("any_provider", "custom-model/free"));

    // Provider-specific free rules
    assert!(is_model_free("groq", "whisper-large-v3"));
    assert!(!is_model_free("groq", "llama-3.3-70b-versatile"));
    assert!(!is_model_free("cerebras", "qwen-3.8-27b"));
    assert!(!is_model_free("cerebras", "gpt-oss-120b"));
    assert!(!is_model_free("nvidia", "01-ai/yi-large"));
    assert!(is_model_free("nvidia", "meta/llama-3.3-70b-instruct:free"));
    assert!(is_model_free("ollama", "llama3.2"));
    assert!(is_model_free("gemini", "gemini-2.0-flash"));
    assert!(!is_model_free("gemini", "gemini-1.5-pro"));
    assert!(!is_model_free("openai", "gpt-4o"));
}

#[tokio::test]
async fn test_fetch_models_missing_api_key() {
    use opendictate::services::ai::{fetch_models_for_provider, AiError};

    let res = fetch_models_for_provider("groq", "").await;
    assert!(matches!(res, Err(AiError::MissingApiKey)));

    let res_nv = fetch_models_for_provider("nvidia", "   ").await;
    assert!(matches!(res_nv, Err(AiError::MissingApiKey)));
}

#[tokio::test]
async fn test_live_fetch_with_secrets() {
    use opendictate::services::ai::fetch_models_for_provider;
    use std::fs;

    let secrets_path = "secrets/secrets.txt";
    if !std::path::Path::new(secrets_path).exists() {
        return;
    }

    let content = fs::read_to_string(secrets_path).unwrap_or_default();
    let mut keys = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if let Some(pos) = line.find(':') {
            let prefix = line[..pos].trim();
            let key = line[pos + 1..].trim();
            let mut prov = prefix
                .trim_start_matches(|c: char| c.is_numeric() || c == '.' || c.is_whitespace())
                .to_lowercase();
            prov = prov.replace(' ', "");
            if !prov.is_empty() && !key.is_empty() {
                keys.insert(prov, key.to_string());
            }
        }
    }

    // Providers to verify
    let providers = [
        "openrouter",
        "nvidia",
        "groq",
        "opencode",
        "cohere",
        "kilocode",
        "cerebras",
    ];
    for prov in providers {
        if let Some(key) = keys.get(prov) {
            println!("Testing live fetch for {}...", prov);
            let mut models = fetch_models_for_provider(prov, key).await;
            if models.is_err() {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                models = fetch_models_for_provider(prov, key).await;
            }
            assert!(
                models.is_ok(),
                "Failed to fetch models for {}: {:?}",
                prov,
                models.err()
            );
            let list = models.unwrap();
            assert!(
                !list.is_empty(),
                "Models list should not be empty for {}",
                prov
            );
            println!(
                "  {} returned {} models. Top model: {} (inputs: {:?}, free: {})",
                prov,
                list.len(),
                list[0].id,
                list[0].inputs,
                list[0].is_free
            );

            // Verify model attributes
            for m in list.iter().take(5) {
                assert!(!m.id.is_empty());
                assert!(!m.name.is_empty());
                assert!(!m.provider.is_empty());
                assert!(!m.inputs.is_empty());
                assert!(m.context_limit > 0);
                assert!(!m.formatted_context().is_empty());
            }
        }
    }
}

#[test]
fn test_clean_free_suffix_and_single_free_display_label() {
    use opendictate::services::ai::{clean_free_suffix, ModelInfo};

    assert_eq!(
        clean_free_suffix("Apodex: Apodex 1.1 Mini (free)"),
        "Apodex: Apodex 1.1 Mini"
    );
    assert_eq!(
        clean_free_suffix("DeepSeek: DeepSeek R1 (Free)"),
        "DeepSeek: DeepSeek R1"
    );
    assert_eq!(clean_free_suffix("MiMo V2.6 Flash Free"), "MiMo V2.6 Flash");
    assert_eq!(clean_free_suffix("GLM 5.1 [Free]"), "GLM 5.1");
    assert_eq!(clean_free_suffix("Qwen 2.5 - Free"), "Qwen 2.5");
    assert_eq!(clean_free_suffix("llama-3.3-70b:free"), "llama-3.3-70b");

    let m = ModelInfo::with_details(
        "apodex/apodex-1.1-mini:free",
        "Apodex: Apodex 1.1 Mini (free)",
        "OpenRouter",
        vec!["text".to_string()],
        false,
        128_000,
        true,
    );
    assert_eq!(m.name, "Apodex: Apodex 1.1 Mini");
    assert_eq!(m.display_label(), "Apodex: Apodex 1.1 Mini [Free]");
    assert_eq!(m.card_model_title(), "Apodex: Apodex 1.1 Mini (Free)");
    assert_eq!(
        m.display_label().to_lowercase().matches("free").count(),
        1,
        "display_label must only contain 'free' once"
    );
    assert_eq!(
        m.card_model_title().to_lowercase().matches("free").count(),
        1,
        "card_model_title must only contain 'free' once"
    );
}

#[test]
fn test_model_audio_capability_and_paid_plan_validation() {
    use opendictate::services::ai::{
        evaluate_model_selection_warning, ModelInfo, AUDIO_UNSUPPORTED_WARNING,
        PAID_MODEL_UNPAID_PLAN_WARNING,
    };

    let free_audio = ModelInfo::with_details(
        "whisper-large-v3-turbo",
        "Whisper Large V3 Turbo",
        "Groq",
        vec!["audio".to_string(), "text".to_string()],
        false,
        25_000,
        true,
    );
    assert!(free_audio.supports_audio());
    assert_eq!(evaluate_model_selection_warning(&free_audio, false), None);

    let free_text_only = ModelInfo::with_details(
        "apodex/apodex-1.1-mini:free",
        "Apodex: Apodex 1.1 Mini (free)",
        "OpenRouter",
        vec!["text".to_string()],
        false,
        128_000,
        true,
    );
    assert!(!free_text_only.supports_audio());
    assert_eq!(
        evaluate_model_selection_warning(&free_text_only, false),
        Some(AUDIO_UNSUPPORTED_WARNING.to_string())
    );

    let paid_model = ModelInfo::with_details(
        "openai/gpt-4o",
        "GPT-4o",
        "OpenRouter",
        vec!["text".to_string(), "image".to_string()],
        false,
        128_000,
        false,
    );
    assert_eq!(
        evaluate_model_selection_warning(&paid_model, false),
        Some(PAID_MODEL_UNPAID_PLAN_WARNING.to_string())
    );
    // When user IS on a paid plan, paid model without audio triggers audio warning
    assert_eq!(
        evaluate_model_selection_warning(&paid_model, true),
        Some(AUDIO_UNSUPPORTED_WARNING.to_string())
    );

    let paid_audio_model = ModelInfo::with_details(
        "openai/gpt-4o-audio-preview",
        "GPT-4o Audio",
        "OpenRouter",
        vec!["text".to_string(), "audio".to_string()],
        false,
        128_000,
        false,
    );
    assert_eq!(
        evaluate_model_selection_warning(&paid_audio_model, false),
        Some(PAID_MODEL_UNPAID_PLAN_WARNING.to_string())
    );
    assert_eq!(
        evaluate_model_selection_warning(&paid_audio_model, true),
        None
    );
}
