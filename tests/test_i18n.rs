use opendictate::config::Config;
use opendictate::services::i18n::{get_languages, tr};

#[test]
fn test_languages_catalog_contains_25_languages_without_hi_ar_bn() {
    let languages = get_languages();
    assert_eq!(
        languages.len(),
        25,
        "Expected 25 languages, found {}",
        languages.len()
    );
    assert!(languages.iter().any(|l| l.code == "en"));
    assert!(languages.iter().any(|l| l.code == "es"));
    assert!(languages.iter().any(|l| l.code == "de"));
    assert!(languages.iter().any(|l| l.code == "fr"));
    assert!(languages.iter().any(|l| l.code == "zh-CN"));
    assert!(languages.iter().any(|l| l.code == "ja"));
    assert!(languages.iter().any(|l| l.code == "ru"));
    assert!(languages.iter().any(|l| l.code == "pt-BR"));
    assert!(
        !languages.iter().any(|l| l.code == "hi"),
        "Hindi should be removed"
    );
    assert!(
        !languages.iter().any(|l| l.code == "ar"),
        "Arabic should be removed"
    );
    assert!(
        !languages.iter().any(|l| l.code == "bn"),
        "Bengali should be removed"
    );
}

#[test]
fn test_translations_coverage_and_fallback() {
    // English
    assert_eq!(tr("settings", "en"), "Settings");
    assert_eq!(tr("language", "en"), "Language");
    assert_eq!(tr("history", "en"), "History");
    assert_eq!(tr("about_opendictate", "en"), "About OpenDictate");
    assert_eq!(tr("local_ai", "en"), "Local AI");

    // Spanish
    assert_eq!(tr("settings", "es"), "Configuración");
    assert_eq!(tr("language", "es"), "Idioma");
    assert_eq!(tr("history", "es"), "Historial");

    // German
    assert_eq!(tr("settings", "de"), "Einstellungen");
    assert_eq!(tr("language", "de"), "Sprache");

    // French
    assert_eq!(tr("settings", "fr"), "Paramètres");
    assert_eq!(tr("language", "fr"), "Langue");

    // Simplified Chinese
    assert_eq!(tr("settings", "zh-CN"), "设置");
    assert_eq!(tr("language", "zh-CN"), "语言");

    // Japanese
    assert_eq!(tr("settings", "ja"), "設定");
    assert_eq!(tr("language", "ja"), "言語");

    // Fallback for unknown language or key
    assert_eq!(tr("settings", "unknown_lang"), "Settings");
    assert_eq!(tr("non_existent_key", "es"), "non_existent_key");
}

#[test]
fn test_config_ui_language_serialization() {
    let mut config = Config::default();
    assert_eq!(config.ui_language, "en");
    config.ui_language = "es".to_string();

    let json = serde_json::to_string(&config).unwrap();
    let loaded: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(loaded.ui_language, "es");
}
