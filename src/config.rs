//! Application configuration for OpenDictate.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Strongly-typed configuration schema for OpenDictate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub theme: String,
    pub ui_language: String,
    pub ai_provider: String,
    pub ai_api_key: String,
    pub ai_model: String,
    pub audio_device: Option<String>,
    pub hotkey: String,
    pub minibar_theme: String,
    pub tone: String,
    pub ai_mode: String,
    pub local_model_id: String,
    pub local_custom_path: Option<String>,
    pub local_threads: u32,
    pub minibar_scale: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            ui_language: "en".to_string(),
            ai_provider: "gemini".to_string(),
            ai_api_key: String::new(),
            ai_model: String::new(),
            audio_device: None,
            hotkey: "Ctrl+Alt+D".to_string(),
            minibar_theme: "dark".to_string(),
            tone: "Clean".to_string(),
            ai_mode: "cloud".to_string(),
            local_model_id: "tiny.en".to_string(),
            local_custom_path: None,
            local_threads: 4,
            minibar_scale: 100,
        }
    }
}

/// Errors that may occur when loading or saving configuration.
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Json(serde_json::Error),
    PathNotFound,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "I/O error: {}", err),
            ConfigError::Json(err) => write!(f, "JSON serialization error: {}", err),
            ConfigError::PathNotFound => write!(f, "Could not determine config path"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Json(err) => Some(err),
            ConfigError::PathNotFound => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Json(err)
    }
}

impl Config {
    /// Returns the default path to the configuration file (`~/.config/opendictate/config.json`).
    pub fn default_path() -> PathBuf {
        if let Ok(custom) = std::env::var("OPENDICTATE_CONFIG_PATH") {
            let trimmed = custom.trim();
            if !trimmed.is_empty() {
                return PathBuf::from(trimmed);
            }
        }
        if let Some(base_dirs) = directories::BaseDirs::new() {
            base_dirs
                .config_dir()
                .join("opendictate")
                .join("config.json")
        } else {
            PathBuf::from(".config/opendictate/config.json")
        }
    }

    /// Ensures `ui_language` is a valid supported language code, defaulting to `"en"` (English).
    pub fn normalize_language(&mut self) {
        let trimmed = self.ui_language.trim();
        let is_valid = crate::services::i18n::LANGUAGES
            .iter()
            .any(|l| l.code == trimmed);
        if is_valid {
            self.ui_language = trimmed.to_string();
        } else {
            self.ui_language = "en".to_string();
        }
    }

    /// Loads configuration from the specified path.
    pub fn load_from(path: &Path) -> Result<Config, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = serde_json::from_str(&content)?;
        config.normalize_language();
        Ok(config)
    }

    /// Loads configuration from the default path or returns default.
    pub fn load_or_default() -> Config {
        Self::load()
    }

    /// Loads configuration from the default path, creating the directory and default file if missing.
    pub fn load() -> Config {
        let path = Self::default_path();
        if path.exists() {
            match Self::load_from(&path) {
                Ok(cfg) => cfg,
                Err(e) => {
                    log::warn!(
                        "Failed to load config from {}: {}, falling back to defaults",
                        path.display(),
                        e
                    );
                    Self::default()
                }
            }
        } else {
            let cfg = Self::default();
            if let Err(e) = cfg.save_to(&path) {
                log::warn!(
                    "Failed to create default config at {}: {}",
                    path.display(),
                    e
                );
            }
            cfg
        }
    }

    /// Saves configuration to the specified path, creating any missing parent directories.
    pub fn save_to(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Saves configuration to the default path.
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::default_path())
    }
}
