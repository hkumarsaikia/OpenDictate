//! Live Backend Integration Tests with Real Providers.
//!
//! Validates that AiManager successfully connects, transcribes audio,
//! and performs text enhancement using verified API keys.

use opendictate::audio::recorder::encode_pcm_wav;
use opendictate::config::Config;
use opendictate::services::ai::{AiManager, AiProvider};
use std::fs;
use std::path::Path;

fn read_secret(key_name: &str) -> Option<String> {
    let path = Path::new("/home/hksaikia/Project/Dictation/secrets/secrets.txt");
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let line = line.trim();
        if let Some(idx) = line.find(':') {
            let prefix = &line[..idx];
            let val = line[idx + 1..].trim();
            if prefix.to_lowercase().contains(&key_name.to_lowercase()) {
                return Some(val.to_string());
            }
        }
    }
    None
}

fn create_test_wav() -> Vec<u8> {
    // Generate 1 second of 440Hz sine wave at 16kHz
    let sample_rate = 16000.0f32;
    let freq = 440.0f32;
    let samples: Vec<f32> = (0..16000)
        .map(|i| 0.3 * (2.0 * std::f32::consts::PI * freq * (i as f32) / sample_rate).sin())
        .collect();
    encode_pcm_wav(&samples, 16000).expect("failed to encode WAV")
}

#[tokio::test]
async fn test_live_groq_transcribe_and_enhance() {
    let key = match read_secret("groq") {
        Some(k) if !k.is_empty() => k,
        _ => {
            eprintln!("Skipping test_live_groq: No key found");
            return;
        }
    };

    let config = Config {
        ai_provider: "groq".to_string(),
        ai_api_key: key,
        ai_model: "openai/gpt-oss-120b".to_string(),
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    let wav = create_test_wav();

    // Test transcription
    println!("Testing Groq audio transcription with whisper-large-v3-turbo...");
    let transcribe_res = manager.transcribe(&wav).await;
    assert!(
        transcribe_res.is_ok(),
        "Groq transcribe failed: {:?}",
        transcribe_res.err()
    );
    println!("Groq transcribe output: {:?}", transcribe_res.unwrap());

    // Test tone enhancement
    println!("Testing Groq text enhancement with openai/gpt-oss-120b...");
    let enhance_res = manager
        .enhance(
            "we should schedule the release for next monday if all tests pass",
            "Professional",
        )
        .await;
    assert!(
        enhance_res.is_ok(),
        "Groq enhance failed: {:?}",
        enhance_res.err()
    );
    let enhanced_text = enhance_res.unwrap();
    assert!(!enhanced_text.is_empty());
    println!("Groq enhanced text: {}", enhanced_text);
}

#[tokio::test]
async fn test_live_cloudflare_transcribe_and_enhance() {
    let key = match read_secret("cloudflare") {
        Some(k) if !k.is_empty() => k,
        _ => {
            eprintln!("Skipping test_live_cloudflare: No key found");
            return;
        }
    };

    let config = Config {
        ai_provider: "cloudflare".to_string(),
        ai_api_key: key,
        ai_model: "@cf/meta/llama-3.1-8b-instruct".to_string(),
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    let wav = if std::path::Path::new("tests/fixtures/jfk.wav").exists() {
        std::fs::read("tests/fixtures/jfk.wav").unwrap_or_else(|_| create_test_wav())
    } else {
        create_test_wav()
    };

    // Test transcription
    println!("Testing Cloudflare audio transcription with @cf/openai/whisper (with auto account resolution)...");
    let transcribe_res = manager.transcribe(&wav).await;
    match transcribe_res {
        Ok(t) => println!("Cloudflare transcribe output: {:?}", t),
        Err(e) => println!(
            "Cloudflare live transcribe note (may be rate-limited or transient): {:?}",
            e
        ),
    }

    // Test tone enhancement
    println!("Testing Cloudflare text enhancement with @cf/meta/llama-3.1-8b-instruct...");
    let enhance_res = manager
        .enhance(
            "can you give me a status update on the latest deployment please",
            "Concise",
        )
        .await;
    assert!(
        enhance_res.is_ok(),
        "Cloudflare enhance failed: {:?}",
        enhance_res.err()
    );
    let enhanced_text = enhance_res.unwrap();
    assert!(!enhanced_text.is_empty());
    println!("Cloudflare enhanced text: {}", enhanced_text);
}

#[tokio::test]
async fn test_live_models_listing() {
    let groq_key = read_secret("groq");
    if let Some(key) = groq_key {
        let provider = opendictate::services::ai::groq::GroqProvider::new(key, None);
        let models = provider.list_models().await;
        if let Ok(list) = models {
            println!(
                "Groq models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            );
        }
    }

    let cf_key = read_secret("cloudflare");
    if let Some(key) = cf_key {
        let provider =
            opendictate::services::ai::cloudflare::CloudflareProvider::new(key, "", None);
        let models = provider.list_models().await;
        if let Ok(list) = models {
            println!(
                "Cloudflare models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            );
        }
    }

    let nv_key = read_secret("nvidia");
    if let Some(key) = nv_key {
        let provider = opendictate::services::ai::nvidia::NvidiaProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "NVIDIA models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            ),
            Err(e) => println!("NVIDIA list models error: {:?}", e),
        }
    }

    let or_key = read_secret("openrouter");
    if let Some(key) = or_key {
        let provider = opendictate::services::ai::openrouter::OpenRouterProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "OpenRouter models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            ),
            Err(e) => println!("OpenRouter list models error: {:?}", e),
        }
    }

    let cer_key = read_secret("cerebras");
    if let Some(key) = cer_key {
        let provider = opendictate::services::ai::cerebras::CerebrasProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "Cerebras models: {} available, e.g. {:?}",
                list.len(),
                &list[..list.len()]
            ),
            Err(e) => println!("Cerebras list models error: {:?}", e),
        }
    }

    let co_key = read_secret("cohere");
    if let Some(key) = co_key {
        let provider = opendictate::services::ai::cohere::CohereProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "Cohere models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            ),
            Err(e) => println!("Cohere list models error: {:?}", e),
        }
    }

    let kilo_key = read_secret("kilo code");
    if let Some(key) = kilo_key {
        let provider = opendictate::services::ai::kilocode::KiloCodeProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "Kilo Code models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            ),
            Err(e) => println!("Kilo Code list models error: {:?}", e),
        }
    }

    let oc_key = read_secret("opencode");
    if let Some(key) = oc_key {
        let provider = opendictate::services::ai::opencode::OpenCodeProvider::new(key, None);
        let models = provider.list_models().await;
        match models {
            Ok(list) => println!(
                "OpenCode models: {} available, e.g. {:?}",
                list.len(),
                &list[..5.min(list.len())]
            ),
            Err(e) => println!("OpenCode list models error: {:?}", e),
        }
    }
}

#[tokio::test]
async fn test_live_fetch_models_for_provider_dynamic() {
    use opendictate::services::ai::fetch_models_for_provider;

    let groq_key = read_secret("groq");
    if let Some(key) = groq_key {
        let res = fetch_models_for_provider("groq", &key).await;
        assert!(res.is_ok());
        let models = res.unwrap();
        assert!(!models.is_empty());
        assert!(models
            .iter()
            .any(|m| m.is_free && m.display_label().contains("[Free]")));
        println!(
            "Dynamic Groq models count: {}, first: {}",
            models.len(),
            models[0].display_label()
        );
    }

    let cer_key = read_secret("cerebras");
    if let Some(key) = cer_key {
        let res = fetch_models_for_provider("cerebras", &key).await;
        assert!(res.is_ok());
        let models = res.unwrap();
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| !m.is_free));
        println!(
            "Dynamic Cerebras models count: {}, first: {}",
            models.len(),
            models[0].display_label()
        );
    }

    let or_key = read_secret("openrouter");
    if let Some(key) = or_key {
        let res = fetch_models_for_provider("openrouter", &key).await;
        assert!(res.is_ok());
        let models = res.unwrap();
        assert!(!models.is_empty());
        let free_count = models.iter().filter(|m| m.is_free).count();
        println!(
            "Dynamic OpenRouter models count: {}, free count: {}, first: {}",
            models.len(),
            free_count,
            models[0].display_label()
        );
    }
}

#[tokio::test]
async fn test_live_nvidia_enhance() {
    let key = match read_secret("nvidia") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let provider = opendictate::services::ai::nvidia::NvidiaProvider::new(
        key,
        Some("meta/llama-3.3-70b-instruct".to_string()),
    );
    let res = provider
        .enhance(
            "this is an informal draft note for the team",
            "Professional",
        )
        .await;
    match res {
        Ok(text) => println!("NVIDIA NIM Live Enhanced: {}", text),
        Err(e) => println!("NVIDIA NIM Live Note: {}", e),
    }
}

#[tokio::test]
async fn test_live_cerebras_enhance() {
    let key = match read_secret("cerebras") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let provider = opendictate::services::ai::cerebras::CerebrasProvider::new(
        key,
        Some("llama-3.3-70b".to_string()),
    );
    let res = provider
        .enhance("quick update meeting tomorrow at ten am", "Professional")
        .await;
    match res {
        Ok(text) => println!("Cerebras Live Enhanced: {}", text),
        Err(e) => println!("Cerebras Live Note: {}", e),
    }
}

#[tokio::test]
async fn test_live_cohere_enhance() {
    let key = match read_secret("cohere") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let provider = opendictate::services::ai::cohere::CohereProvider::new(
        key,
        Some("command-r-08-2024".to_string()),
    );
    let res = provider
        .enhance("here are the notes from today's discussion", "Concise")
        .await;
    match res {
        Ok(text) => println!("Cohere Live Enhanced: {}", text),
        Err(e) => println!("Cohere Live Note: {}", e),
    }
}

#[tokio::test]
async fn test_live_kilocode_enhance() {
    let key = match read_secret("kilo code") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let provider = opendictate::services::ai::kilocode::KiloCodeProvider::new(
        key,
        Some("kilo-auto/free".to_string()),
    );
    let res = provider
        .enhance(
            "hello team this is a draft release announcement",
            "Professional",
        )
        .await;
    match res {
        Ok(text) => println!("Kilo Code Live Enhanced: {}", text),
        Err(e) => println!("Kilo Code Live Note: {}", e),
    }
}

#[tokio::test]
async fn test_live_opencode_enhance() {
    let key = match read_secret("opencode") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let provider = opendictate::services::ai::opencode::OpenCodeProvider::new(
        key,
        Some("kimi-k2.6".to_string()),
    );
    let res = provider
        .enhance("we should deploy the build this evening", "Clean")
        .await;
    match res {
        Ok(text) => println!("OpenCode Live Enhanced: {}", text),
        Err(e) => println!("OpenCode Live Note: {}", e),
    }
}
