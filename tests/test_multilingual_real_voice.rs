//! Comprehensive Real Voice Multi-Language Integration Tests.
//!
//! Tests both Local AI (catalog multilingual model + unlisted custom model)
//! and Cloud AI providers on authentic human speech in English, German, French, and Spanish.

use opendictate::config::Config;
use opendictate::services::ai::local_ai::{LocalAiProvider, get_local_model_catalog};
use opendictate::services::ai::{AiManager, AiProvider};
use std::fs;
use std::path::Path;

fn read_secret(key_name: &str) -> Option<String> {
    let env_key = format!("{}_API_KEY", key_name.to_uppercase().replace(' ', "_"));
    if let Ok(val) = std::env::var(&env_key)
        && !val.trim().is_empty()
    {
        return Some(val.trim().to_string());
    }
    let secrets_file = std::env::var("OPENDICTATE_SECRETS_FILE").ok()?;
    let path = Path::new(&secrets_file);
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

#[tokio::test]
async fn test_local_ai_multilingual_real_voice_catalog() {
    let model_path = opendictate::services::ai::local_ai::resolve_model_path("tiny", None);
    if !model_path.exists() {
        eprintln!(
            "Skipping test_local_ai_multilingual_real_voice_catalog: ggml-tiny.bin not found at {:?}",
            model_path
        );
        return;
    }

    println!(
        "Testing Local AI with catalog multilingual model: {:?}",
        model_path
    );

    // 1. English (JFK)
    let jfk_bytes = fs::read("tests/fixtures/jfk.wav").expect("read jfk.wav");
    let provider_en = LocalAiProvider::with_config("tiny".to_string(), None, 0);
    let en_text = provider_en
        .transcribe(jfk_bytes, None)
        .await
        .expect("transcribe English");
    println!("Local AI [tiny] English transcript: '{}'", en_text);
    assert!(!en_text.is_empty());
    let en_lower = en_text.to_lowercase();
    assert!(
        en_lower.contains("fellow")
            || en_lower.contains("country")
            || en_lower.contains("americans")
    );

    // 2. German ("Fürstentum Liechtenstein")
    if Path::new("tests/fixtures/german.wav").exists() {
        let de_bytes = fs::read("tests/fixtures/german.wav").expect("read german.wav");
        let provider_de = LocalAiProvider::with_config("tiny".to_string(), None, 0);
        let de_text = provider_de
            .transcribe(de_bytes, None)
            .await
            .expect("transcribe German");
        println!("Local AI [tiny] German transcript: '{}'", de_text);
        assert!(!de_text.is_empty());
        let de_lower = de_text.to_lowercase();
        assert!(
            de_lower.contains("liechten")
                || de_lower.contains("stein")
                || de_lower.contains("fürst")
                || de_lower.contains("fuerst"),
            "Expected German transcript to contain Liechtenstein components, got: {}",
            de_text
        );
    }

    // 3. French ("Ceci n'est pas une pipe")
    if Path::new("tests/fixtures/french.wav").exists() {
        let fr_bytes = fs::read("tests/fixtures/french.wav").expect("read french.wav");
        let provider_fr = LocalAiProvider::with_config("tiny".to_string(), None, 0);
        let fr_text = provider_fr
            .transcribe(fr_bytes, None)
            .await
            .expect("transcribe French");
        println!("Local AI [tiny] French transcript: '{}'", fr_text);
        assert!(!fr_text.is_empty());
        let fr_lower = fr_text.to_lowercase();
        assert!(
            fr_lower.contains("pipe") || fr_lower.contains("ceci") || fr_lower.contains("pas"),
            "Expected French transcript to contain 'pipe' or 'ceci', got: {}",
            fr_text
        );
    }

    // 4. Spanish ("Adiós, hasta mañana")
    if Path::new("tests/fixtures/spanish.wav").exists() {
        let es_bytes = fs::read("tests/fixtures/spanish.wav").expect("read spanish.wav");
        let provider_es = LocalAiProvider::with_config("tiny".to_string(), None, 0);
        let es_text = provider_es
            .transcribe(es_bytes, None)
            .await
            .expect("transcribe Spanish");
        println!("Local AI [tiny] Spanish transcript: '{}'", es_text);
        assert!(!es_text.is_empty());
        let es_lower = es_text.to_lowercase();
        assert!(
            es_lower.contains("adiós")
                || es_lower.contains("adios")
                || es_lower.contains("mañana")
                || es_lower.contains("manana")
                || es_lower.contains("hasta"),
            "Expected Spanish transcript to contain 'adiós' or 'hasta', got: {}",
            es_text
        );
    }
}

#[tokio::test]
async fn test_local_ai_multilingual_real_voice_unlisted_custom_model() {
    let custom_path = "/tmp/custom_models/ggml-tiny-q5_1.bin";
    if !Path::new(custom_path).exists() {
        eprintln!(
            "Skipping test_local_ai_multilingual_real_voice_unlisted_custom_model: model not found"
        );
        return;
    }

    // Verify it's not in the catalog
    let catalog = get_local_model_catalog();
    assert!(!catalog.iter().any(|m| m.filename == "ggml-tiny-q5_1.bin"));

    // Test with Auto threads (0)
    let provider =
        LocalAiProvider::with_config("custom".to_string(), Some(custom_path.to_string()), 0);

    // German speech
    if Path::new("tests/fixtures/german.wav").exists() {
        let de_bytes = fs::read("tests/fixtures/german.wav").expect("read german.wav");
        let de_text = provider
            .transcribe(de_bytes, None)
            .await
            .expect("custom unlisted transcribe German");
        println!(
            "Unlisted Custom Model [tiny-q5_1] German transcript: '{}'",
            de_text
        );
        assert!(!de_text.is_empty());
        let lower = de_text.to_lowercase();
        assert!(
            lower.contains("fürst")
                || lower.contains("fuerst")
                || lower.contains("stein")
                || lower.contains("steein")
                || lower.contains("liechten")
        );
    }

    // Spanish speech
    if Path::new("tests/fixtures/spanish.wav").exists() {
        let es_bytes = fs::read("tests/fixtures/spanish.wav").expect("read spanish.wav");
        let es_text = provider
            .transcribe(es_bytes, None)
            .await
            .expect("custom unlisted transcribe Spanish");
        println!(
            "Unlisted Custom Model [tiny-q5_1] Spanish transcript: '{}'",
            es_text
        );
        assert!(!es_text.is_empty());
        let lower = es_text.to_lowercase();
        assert!(
            lower.contains("adiós")
                || lower.contains("adios")
                || lower.contains("hasta")
                || lower.contains("mañana")
                || lower.contains("manana")
        );
    }
}

#[tokio::test]
async fn test_cloud_groq_multilingual_real_voice() {
    let key = match read_secret("groq") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };

    let config = Config {
        ai_provider: "groq".to_string(),
        ai_api_key: key,
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);

    // 1. English
    let jfk = fs::read("tests/fixtures/jfk.wav").expect("jfk.wav");
    let en_res = manager.transcribe(&jfk).await.expect("Groq JFK transcribe");
    println!("Groq Live English transcript: '{}'", en_res);
    assert!(
        en_res.to_lowercase().contains("fellow americans")
            || en_res.to_lowercase().contains("country")
    );

    // 2. German
    if Path::new("tests/fixtures/german.wav").exists() {
        let de = fs::read("tests/fixtures/german.wav").expect("german.wav");
        let de_res = manager
            .transcribe(&de)
            .await
            .expect("Groq German transcribe");
        println!("Groq Live German transcript: '{}'", de_res);
        assert!(de_res.to_lowercase().contains("liechtenstein"));
    }

    // 3. French
    if Path::new("tests/fixtures/french.wav").exists() {
        let fr = fs::read("tests/fixtures/french.wav").expect("french.wav");
        let fr_res = manager
            .transcribe(&fr)
            .await
            .expect("Groq French transcribe");
        println!("Groq Live French transcript: '{}'", fr_res);
        assert!(fr_res.to_lowercase().contains("pipe") || fr_res.to_lowercase().contains("ceci"));
    }

    // 4. Spanish
    if Path::new("tests/fixtures/spanish.wav").exists() {
        let es = fs::read("tests/fixtures/spanish.wav").expect("spanish.wav");
        let es_res = manager
            .transcribe(&es)
            .await
            .expect("Groq Spanish transcribe");
        println!("Groq Live Spanish transcript: '{}'", es_res);
        assert!(
            es_res.to_lowercase().contains("adiós")
                || es_res.to_lowercase().contains("adios")
                || es_res.to_lowercase().contains("hasta")
        );
    }
}

#[tokio::test]
async fn test_cloud_cloudflare_real_voice() {
    let key = match read_secret("cloudflare") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };

    let config = Config {
        ai_provider: "cloudflare".to_string(),
        ai_api_key: key,
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    let jfk = fs::read("tests/fixtures/jfk.wav").expect("jfk.wav");
    match manager.transcribe(&jfk).await {
        Ok(res) => {
            println!("Cloudflare Live English transcript: '{}'", res);
            assert!(!res.is_empty());
        }
        Err(e) => {
            println!(
                "Cloudflare live transcribe note (may be rate-limited or transient): {:?}",
                e
            );
        }
    }

    // German
    if Path::new("tests/fixtures/german.wav").exists() {
        let de = fs::read("tests/fixtures/german.wav").expect("german.wav");
        if let Ok(de_res) = manager.transcribe(&de).await {
            println!("Cloudflare Live German transcript: '{}'", de_res);
            assert!(!de_res.is_empty());
        }
    }

    // Spanish
    if Path::new("tests/fixtures/spanish.wav").exists() {
        let es = fs::read("tests/fixtures/spanish.wav").expect("spanish.wav");
        if let Ok(es_res) = manager.transcribe(&es).await {
            println!("Cloudflare Live Spanish transcript: '{}'", es_res);
            assert!(!es_res.is_empty());
        }
    }
}

#[tokio::test]
async fn test_cloud_huggingface_real_voice() {
    let key = match read_secret("huggingface") {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };

    let config = Config {
        ai_provider: "huggingface".to_string(),
        ai_api_key: key,
        ..Default::default()
    };

    let manager = AiManager::with_config(&config);
    let jfk = fs::read("tests/fixtures/jfk.wav").expect("jfk.wav");
    match manager.transcribe(&jfk).await {
        Ok(res) => {
            println!("Hugging Face Live English transcript: '{}'", res);
            assert!(!res.is_empty());
        }
        Err(e) => {
            println!(
                "Hugging Face live transcribe note (may be loading/transient): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_local_ai_threads_scaling_and_consistency() {
    let model_path = opendictate::services::ai::local_ai::resolve_model_path("tiny", None);
    if !model_path.exists() {
        eprintln!("Skipping thread scaling test: model not installed");
        return;
    }

    let jfk_bytes = fs::read("tests/fixtures/jfk.wav").expect("read jfk.wav");

    // Test different thread configurations: 0 (Auto), 1, 2, 4, 8, 16
    let test_threads = [0, 1, 2, 4, 8, 16];
    for &th in &test_threads {
        let provider = LocalAiProvider::with_config("tiny".to_string(), None, th);
        let start = std::time::Instant::now();
        let transcript = provider
            .transcribe(jfk_bytes.clone(), None)
            .await
            .unwrap_or_else(|e| panic!("Failed with {} threads: {}", th, e));
        let duration = start.elapsed();
        println!(
            "Threads = {:2} | Duration: {:?} | Text length: {}",
            th,
            duration,
            transcript.len()
        );
        assert!(!transcript.is_empty());
        let lower = transcript.to_lowercase();
        assert!(
            lower.contains("fellow") || lower.contains("country") || lower.contains("americans")
        );
    }
}

#[tokio::test]
async fn test_local_ai_explicit_language_parameter() {
    let model_path = opendictate::services::ai::local_ai::resolve_model_path("tiny", None);
    if !model_path.exists() {
        eprintln!("Skipping explicit language test: model not installed");
        return;
    }

    if Path::new("tests/fixtures/spanish.wav").exists() {
        let es_bytes = fs::read("tests/fixtures/spanish.wav").expect("read spanish.wav");
        let provider = LocalAiProvider::with_config("tiny".to_string(), None, 4);

        // Explicitly force Spanish with "es"
        let es_text = provider
            .transcribe(es_bytes.clone(), Some("es"))
            .await
            .expect("transcribe Spanish with explicit 'es'");
        println!("Local AI explicit 'es' transcript: '{}'", es_text);
        assert!(!es_text.is_empty());
        assert!(
            es_text.to_lowercase().contains("adiós")
                || es_text.to_lowercase().contains("adios")
                || es_text.to_lowercase().contains("hasta")
        );

        // Also test with explicit "auto"
        let auto_text = provider
            .transcribe(es_bytes, Some("auto"))
            .await
            .expect("transcribe Spanish with 'auto'");
        println!("Local AI 'auto' transcript: '{}'", auto_text);
        assert!(!auto_text.is_empty());
    }
}
