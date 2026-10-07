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
