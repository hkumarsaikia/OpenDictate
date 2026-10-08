//! Global Hotkey Service for shortcut parsing, normalization, and action dispatch.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

/// High-level application actions triggered by global hotkeys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HotkeyAction {
    ToggleDictation,
    ShowMiniBar,
    ShowMainWindow,
    Cancel,
}

impl HotkeyAction {
    /// Returns a string slice representation of the action.
    pub fn as_str(&self) -> &'static str {
        match self {
            HotkeyAction::ToggleDictation => "toggle_dictation",
            HotkeyAction::ShowMiniBar => "show_minibar",
            HotkeyAction::ShowMainWindow => "show_main_window",
            HotkeyAction::Cancel => "cancel",
        }
    }

    /// Parses an action from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, HotkeyError> {
        match s.trim().to_lowercase().as_str() {
            "toggle_dictation" | "toggle" => Ok(HotkeyAction::ToggleDictation),
            "show_minibar" | "minibar" => Ok(HotkeyAction::ShowMiniBar),
            "show_main_window" | "main_window" | "dashboard" => Ok(HotkeyAction::ShowMainWindow),
            "cancel" | "escape" => Ok(HotkeyAction::Cancel),
            other => Err(HotkeyError::InvalidAction(format!(
                "Unknown hotkey action: {}",
                other
            ))),
        }
    }
}

impl std::fmt::Display for HotkeyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Errors occurring during hotkey parsing or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotkeyError {
    InvalidShortcut(String),
    InvalidAction(String),
    PortalUnavailable(String),
}

impl std::fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotkeyError::InvalidShortcut(msg) => write!(f, "Invalid shortcut: {}", msg),
            HotkeyError::InvalidAction(msg) => write!(f, "Invalid action: {}", msg),
            HotkeyError::PortalUnavailable(msg) => {
                write!(f, "Global shortcut portal unavailable: {}", msg)
            }
        }
    }
}

impl std::error::Error for HotkeyError {}

/// Represents a parsed shortcut with modifiers and key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedShortcut {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
    pub key: String,
}

impl ParsedShortcut {
    /// Formats the parsed shortcut into canonical normalized representation.
    pub fn to_normalized_string(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.super_key {
            parts.push("Super");
        }
        parts.push(&self.key);
        parts.join("+")
    }

    /// Formats the parsed shortcut into a GTK4 accelerator string (e.g., `<Control><Alt>d`).
    pub fn to_gtk_accelerator(&self) -> String {
        let mut out = String::new();
        if self.ctrl {
            out.push_str("<Control>");
        }
        if self.alt {
            out.push_str("<Alt>");
        }
        if self.shift {
            out.push_str("<Shift>");
        }
        if self.super_key {
            out.push_str("<Super>");
        }
        let gtk_key = match self.key.as_str() {
            "Space" => "space".to_string(),
            k if k.chars().count() == 1 => k.to_lowercase(),
            other => other.to_string(),
        };
        out.push_str(&gtk_key);
        out
    }
}

/// Service managing hotkey registration, event parsing, and distribution.
#[derive(Clone, Default)]
pub struct HotkeyService {
    actions: HashMap<String, HotkeyAction>,
    senders: Vec<Sender<HotkeyAction>>,
    callbacks: Vec<Arc<dyn Fn(HotkeyAction) + Send + Sync + 'static>>,
}

impl HotkeyService {
    /// Creates a new empty HotkeyService.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses and normalizes a shortcut string into a `ParsedShortcut`.
    pub fn parse_shortcut(raw: &str) -> Result<ParsedShortcut, HotkeyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(HotkeyError::InvalidShortcut("Empty shortcut".to_string()));
        }

        // Normalize angle brackets: <Control><Shift>space -> Control+Shift+space
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_bracket = false;

        for ch in trimmed.chars() {
            match ch {
                '<' => {
                    if !current.trim().is_empty() {
                        tokens.push(current.trim().to_string());
                        current.clear();
                    }
                    in_bracket = true;
                }
                '>' => {
                    if in_bracket {
                        if !current.trim().is_empty() {
                            tokens.push(current.trim().to_string());
                            current.clear();
                        }
                        in_bracket = false;
                    }
                }
                '+' | '-' if !in_bracket => {
                    if !current.trim().is_empty() {
                        tokens.push(current.trim().to_string());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        if !current.trim().is_empty() {
            tokens.push(current.trim().to_string());
        }

        if tokens.is_empty() {
            return Err(HotkeyError::InvalidShortcut(format!(
                "No keys in shortcut: {}",
                raw
            )));
        }

        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut super_key = false;
        let mut key_candidate: Option<String> = None;

        for tok in tokens {
            let lower = tok.to_lowercase();
            match lower.as_str() {
                "ctrl" | "control" | "crtl" => ctrl = true,
                "alt" | "mod1" | "option" => alt = true,
                "shift" => shift = true,
                "super" | "win" | "windows" | "meta" | "cmd" | "command" | "mod4" => {
                    super_key = true
                }
                _ => {
                    // Normalize standard key names
                    let normalized_key = match lower.as_str() {
                        "space" => "Space".to_string(),
                        "enter" | "return" => "Return".to_string(),
                        "esc" | "escape" => "Escape".to_string(),
                        "tab" => "Tab".to_string(),
                        "backspace" => "Backspace".to_string(),
                        "up" => "Up".to_string(),
                        "down" => "Down".to_string(),
                        "left" => "Left".to_string(),
                        "right" => "Right".to_string(),
                        k if k.starts_with('f') && k[1..].chars().all(|c| c.is_ascii_digit()) => {
                            k.to_uppercase()
                        }
                        single if single.chars().count() == 1 => single.to_uppercase(),
                        other => {
                            // Capitalize first char
                            let mut c = other.chars();
                            match c.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + c.as_str()
                                }
                            }
                        }
                    };
                    key_candidate = Some(normalized_key);
                }
            }
        }

        let key = key_candidate.ok_or_else(|| {
            HotkeyError::InvalidShortcut(format!("No primary key specified in shortcut: {}", raw))
        })?;

        Ok(ParsedShortcut {
            ctrl,
            alt,
            shift,
            super_key,
            key,
        })
    }

    /// Normalizes a shortcut string to its canonical form (e.g., `Ctrl+Alt+D`).
    pub fn normalize(raw: &str) -> Result<String, HotkeyError> {
        let parsed = Self::parse_shortcut(raw)?;
        Ok(parsed.to_normalized_string())
    }

    /// Converts a shortcut string into a GTK4 accelerator string (e.g., `<Control><Alt>d`).
    pub fn to_gtk_accelerator(raw: &str) -> Result<String, HotkeyError> {
        let parsed = Self::parse_shortcut(raw)?;
        Ok(parsed.to_gtk_accelerator())
    }

    /// Registers a shortcut associated with an action.
    pub fn register_action(
        &mut self,
        shortcut: &str,
        action: HotkeyAction,
    ) -> Result<(), HotkeyError> {
        let normalized = Self::normalize(shortcut)?;
        self.actions.insert(normalized, action);
        Ok(())
    }

    /// Registers a tokio mpsc Sender channel to receive hotkey actions.
    pub fn register_sender(&mut self, sender: Sender<HotkeyAction>) {
        self.senders.push(sender);
    }

    /// Registers a closure callback to be invoked on hotkey trigger.
    pub fn register_callback<F>(&mut self, callback: F)
    where
        F: Fn(HotkeyAction) + Send + Sync + 'static,
    {
        self.callbacks.push(Arc::new(callback));
    }

    /// Dispatches an action to all registered receivers and callbacks.
    pub fn trigger_action(&self, action: HotkeyAction) {
        for sender in &self.senders {
            let _ = sender.try_send(action);
        }
        for cb in &self.callbacks {
            cb(action);
        }
    }

    /// Triggers the action associated with the given shortcut string, if registered.
    pub fn trigger_shortcut(&self, shortcut: &str) -> bool {
        if let Ok(normalized) = Self::normalize(shortcut)
            && let Some(&action) = self.actions.get(&normalized)
        {
            self.trigger_action(action);
            return true;
        }
        false
    }

    /// Returns a list of all registered shortcuts and actions.
    pub fn registered_shortcuts(&self) -> Vec<(String, HotkeyAction)> {
        self.actions.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}
