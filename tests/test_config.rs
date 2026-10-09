use opendictate::config::Config;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_default_config() {
    let cfg = Config::default();
    assert_eq!(cfg.theme, "dark");
    assert_eq!(cfg.ai_provider, "gemini");
    assert_eq!(cfg.ai_api_key, "");
    assert_eq!(cfg.ai_model, "");
    assert_eq!(cfg.audio_device, None);
    assert_eq!(cfg.hotkey, "Ctrl+Alt+D");
    assert_eq!(cfg.minibar_theme, "dark");
    assert_eq!(cfg.tone, "Clean");
}

#[test]
fn test_config_save_and_load_roundtrip() {
    let dir = tempdir().expect("failed to create temp dir");
    let file_path = dir.path().join("config.json");

    let cfg = Config {
        theme: "light".to_string(),
        ai_provider: "openai".to_string(),
        ai_api_key: "test-sk-12345".to_string(),
        ai_model: "gpt-4o-mini".to_string(),
        audio_device: Some("USB Audio Device".to_string()),
        hotkey: "Super+Shift+D".to_string(),
        minibar_theme: "light".to_string(),
        tone: "Concise".to_string(),
        ..Default::default()
    };

    cfg.save_to(&file_path).expect("failed to save config");
    assert!(file_path.exists());

    let loaded = Config::load_from(&file_path).expect("failed to load config");
    assert_eq!(cfg, loaded);
}

#[test]
fn test_config_partial_json_deserialization() {
    let dir = tempdir().expect("failed to create temp dir");
    let file_path = dir.path().join("partial_config.json");

    let json_data = r#"{"theme": "light", "ai_provider": "anthropic"}"#;
    fs::write(&file_path, json_data).expect("failed to write partial json");

    let loaded = Config::load_from(&file_path).expect("failed to load config");
    assert_eq!(loaded.theme, "light");
    assert_eq!(loaded.ai_provider, "anthropic");
    // Other fields should have their default values
    assert_eq!(loaded.ai_api_key, "");
    assert_eq!(loaded.hotkey, "Ctrl+Alt+D");
}

#[test]
fn test_config_default_path() {
    let path = Config::default_path();
    assert!(
        path.ends_with("opendictate/config.json"),
        "Default path should end with opendictate/config.json, got: {:?}",
        path
    );
}

#[test]
fn test_config_dual_engine_fields() {
    let mut config = Config::default();
    assert_eq!(config.ai_mode, "cloud");
    assert_eq!(config.local_model_id, "tiny.en");
    assert_eq!(config.local_custom_path, None);
    assert_eq!(config.local_threads, 4);

    config.ai_mode = "local".to_string();
    config.local_model_id = "base.en".to_string();
    config.local_custom_path = Some("/opt/models/model.bin".to_string());
    config.local_threads = 6;

    let json = serde_json::to_string(&config).expect("Serialize config");
    let deserialized: Config = serde_json::from_str(&json).expect("Deserialize config");
    assert_eq!(deserialized.ai_mode, "local");
    assert_eq!(deserialized.local_model_id, "base.en");
    assert_eq!(
        deserialized.local_custom_path,
        Some("/opt/models/model.bin".to_string())
    );
    assert_eq!(deserialized.local_threads, 6);
}

#[test]
fn test_config_default_language_is_english_and_normalizes_invalid() {
    let cfg = Config::default();
    assert_eq!(cfg.ui_language, "en");

    let dir = tempdir().expect("failed to create temp dir");
    let invalid_path = dir.path().join("invalid_lang.json");
    fs::write(&invalid_path, r#"{"ui_language": "unknown_locale_xyz"}"#)
        .expect("failed to write invalid_lang.json");
    let loaded = Config::load_from(&invalid_path).expect("failed to load config");
    assert_eq!(
        loaded.ui_language, "en",
        "Unrecognized ui_language must normalize to 'en'"
    );

    let empty_path = dir.path().join("empty_lang.json");
    fs::write(&empty_path, r#"{"ui_language": "   "}"#).expect("failed to write empty_lang.json");
    let loaded_empty = Config::load_from(&empty_path).expect("failed to load config");
    assert_eq!(
        loaded_empty.ui_language, "en",
        "Empty ui_language must normalize to 'en'"
    );
}

#[test]
fn test_config_validate_and_normalize_clamping_and_fallbacks() {
    let dir = tempdir().expect("failed to create temp dir");
    let bad_path = dir.path().join("bad_config.json");

    let json_data = r#"{
        "theme": "neon_purple",
        "minibar_theme": "WHITE",
        "tone": "RAW",
        "ai_mode": "invalid_mode",
        "ai_provider": "   ",
        "local_model_id": "",
        "hotkey": "  ",
        "local_threads": 0,
        "minibar_scale": 0
    }"#;
    fs::write(&bad_path, json_data).expect("failed to write bad_config.json");

    let loaded = Config::load_from(&bad_path).expect("failed to load bad_config.json");
    assert_eq!(loaded.theme, "dark");
    assert_eq!(loaded.minibar_theme, "white");
    assert_eq!(loaded.tone, "Raw");
    assert_eq!(loaded.ai_mode, "cloud");
    assert_eq!(loaded.ai_provider, "gemini");
    assert_eq!(loaded.local_model_id, "tiny.en");
    assert_eq!(loaded.hotkey, "Ctrl+Alt+D");
    assert_eq!(loaded.local_threads, 0);
    assert_eq!(loaded.minibar_scale, 100);

    let extreme_path = dir.path().join("extreme_config.json");
    fs::write(
        &extreme_path,
        r#"{"local_threads": 500, "minibar_scale": 10, "tone": "professional"}"#,
    )
    .expect("failed to write extreme_config.json");
    let loaded_extreme = Config::load_from(&extreme_path).expect("failed to load extreme_config");
    assert_eq!(loaded_extreme.local_threads, 128);
    assert_eq!(loaded_extreme.minibar_scale, 70);
    assert_eq!(loaded_extreme.tone, "Professional");

    let high_scale_path = dir.path().join("high_scale.json");
    fs::write(&high_scale_path, r#"{"minibar_scale": 999}"#).expect("failed to write high_scale");
    let loaded_high = Config::load_from(&high_scale_path).expect("failed to load high_scale");
    assert_eq!(loaded_high.minibar_scale, 200);
}

#[test]
fn test_config_atomic_save_and_unix_permissions() {
    let dir = tempdir().expect("failed to create temp dir");
    let file_path = dir.path().join("config.json");
    let tmp_path = file_path.with_extension("json.tmp");

    let cfg = Config {
        ai_api_key: "sk-secret-key-test".to_string(),
        minibar_scale: 350, // Should normalize to 200 on save
        ..Default::default()
    };

    cfg.save_to(&file_path).expect("failed to save config");
    assert!(file_path.exists());
    assert!(
        !tmp_path.exists(),
        "Temporary file must be renamed atomically and not left behind"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&file_path)
            .expect("failed to read metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(
            mode, 0o600,
            "Config file containing API keys must have 0o600 permissions on Unix"
        );
    }

    let reloaded = Config::load_from(&file_path).expect("failed to reload config");
    assert_eq!(reloaded.minibar_scale, 200);
    assert_eq!(reloaded.ai_api_key, "sk-secret-key-test");
}
