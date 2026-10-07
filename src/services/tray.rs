//! DBus System Tray implementation via KDE/Freedesktop StatusNotifierItem (`ksni`).

use ksni::menu::{CheckmarkItem, MenuItem, StandardItem, SubMenu};
use ksni::{ToolTip, Tray};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Dynamic status state of OpenDictate represented in the system tray.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TrayState {
    #[default]
    Idle,
    Recording,
    Paused,
    Processing,
}

impl TrayState {
    /// Returns the string slice representation of the tray state.
    pub fn as_str(&self) -> &'static str {
        match self {
            TrayState::Idle => "idle",
            TrayState::Recording => "recording",
            TrayState::Paused => "paused",
            TrayState::Processing => "processing",
        }
    }
}

/// Callbacks invoked when tray menu items or activations trigger.
#[derive(Clone, Default)]
pub struct TrayCallbacks {
    pub on_toggle_dictation: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    pub on_show_minibar: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    pub on_open_dashboard: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    pub on_set_theme: Option<Arc<dyn Fn(bool /* is_dark */) + Send + Sync + 'static>>,
    pub on_quit: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
}

/// OpenDictate StatusNotifierItem system tray item.
#[derive(Clone)]
pub struct OpenDictateTray {
    state: TrayState,
    is_dark_theme: bool,
    callbacks: TrayCallbacks,
}

impl Default for OpenDictateTray {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenDictateTray {
    /// Constructs a new OpenDictateTray with default settings and empty callbacks.
    pub fn new() -> Self {
        Self {
            state: TrayState::Idle,
            is_dark_theme: true,
            callbacks: TrayCallbacks::default(),
        }
    }

    /// Constructs a new OpenDictateTray with specific callbacks.
    pub fn with_callbacks(callbacks: TrayCallbacks) -> Self {
        Self {
            state: TrayState::Idle,
            is_dark_theme: true,
            callbacks,
        }
    }

    /// Returns the current dynamic tray state.
    pub fn state(&self) -> TrayState {
        self.state
    }

    /// Updates the dynamic tray state.
    pub fn set_state(&mut self, state: TrayState) {
        self.state = state;
    }

    /// Returns whether the dark theme is active.
    pub fn is_dark_theme(&self) -> bool {
        self.is_dark_theme
    }

    /// Sets the active theme indicator.
    pub fn set_dark_theme(&mut self, dark: bool) {
        self.is_dark_theme = dark;
    }

    /// Returns a reference to the callbacks.
    pub fn callbacks(&self) -> &TrayCallbacks {
        &self.callbacks
    }

    /// Returns a mutable reference to the callbacks.
    pub fn callbacks_mut(&mut self) -> &mut TrayCallbacks {
        &mut self.callbacks
    }

    /// Triggers the toggle dictation callback if configured.
    pub fn trigger_toggle(&self) {
        if let Some(cb) = &self.callbacks.on_toggle_dictation {
            cb();
        }
    }

    /// Triggers the show minibar callback if configured.
    pub fn trigger_show_minibar(&self) {
        if let Some(cb) = &self.callbacks.on_show_minibar {
            cb();
        }
    }

    /// Triggers the open dashboard callback if configured.
    pub fn trigger_open_dashboard(&self) {
        if let Some(cb) = &self.callbacks.on_open_dashboard {
            cb();
        }
    }

    /// Sets theme state and triggers the theme change callback if configured.
    pub fn trigger_set_theme(&mut self, is_dark: bool) {
        self.is_dark_theme = is_dark;
        if let Some(cb) = &self.callbacks.on_set_theme {
            cb(is_dark);
        }
    }

    /// Triggers the quit callback or exits the process.
    pub fn trigger_quit(&self) {
        if let Some(cb) = &self.callbacks.on_quit {
            cb();
        } else {
            std::process::exit(0);
        }
    }
}

impl Tray for OpenDictateTray {
    fn id(&self) -> String {
        "opendictate".to_string()
    }

    fn title(&self) -> String {
        "OpenDictate".to_string()
    }

    fn icon_theme_path(&self) -> String {
        let manifest_icons = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/ui/assets/brand/icons/hicolor/scalable/apps");
        if manifest_icons.exists() {
            return manifest_icons.to_string_lossy().to_string();
        }
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let user_icons = base_dirs
                .data_local_dir()
                .join("icons/hicolor/scalable/apps");
            if user_icons.exists() {
                return user_icons.to_string_lossy().to_string();
            }
        }
        String::new()
    }

    fn icon_name(&self) -> String {
        match self.state {
            TrayState::Idle => {
                if self.is_dark_theme {
                    "opendictate-dark".to_string()
                } else {
                    "opendictate-light".to_string()
                }
            }
            TrayState::Recording => "media-record-symbolic".to_string(),
            TrayState::Paused => "media-playback-pause-symbolic".to_string(),
            TrayState::Processing => "process-working-symbolic".to_string(),
        }
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "OpenDictate".to_string(),
            description: "OpenDictate - Voice Dictation & Intelligence".to_string(),
            ..Default::default()
        }
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        self.trigger_show_minibar();
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let is_dark = self.is_dark_theme;

        vec![
            StandardItem {
                label: "Toggle Dictation".to_string(),
                activate: Box::new(|this: &mut Self| {
                    this.trigger_toggle();
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Show Mini-Bar".to_string(),
                activate: Box::new(|this: &mut Self| {
                    this.trigger_show_minibar();
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            SubMenu {
                label: "Theme".to_string(),
                submenu: vec![
                    CheckmarkItem {
                        label: "Dark".to_string(),
                        checked: is_dark,
                        activate: Box::new(|this: &mut Self| {
                            this.trigger_set_theme(true);
                        }),
                        ..Default::default()
                    }
                    .into(),
                    CheckmarkItem {
                        label: "White".to_string(),
                        checked: !is_dark,
                        activate: Box::new(|this: &mut Self| {
                            this.trigger_set_theme(false);
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit OpenDictate".to_string(),
                activate: Box::new(|this: &mut Self| {
                    this.trigger_quit();
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

/// Service helper to spawn and manage the DBus system tray thread.
pub struct TrayService;

impl TrayService {
    /// Spawns the system tray in a background thread and returns the `ksni::Handle`.
    pub fn spawn(tray: OpenDictateTray) -> ksni::Handle<OpenDictateTray> {
        let service = ksni::TrayService::new(tray);
        let handle = service.handle();
        service.spawn();
        handle
    }

    /// Spawns the system tray safely, logging any DBus errors instead of panicking.
    pub fn try_spawn(tray: OpenDictateTray) -> Result<ksni::Handle<OpenDictateTray>, String> {
        let service = ksni::TrayService::new(tray);
        let handle = service.handle();
        std::thread::spawn(move || {
            if let Err(e) = service.run() {
                log::warn!("DBus TrayService stopped: {:?}", e);
            }
        });
        Ok(handle)
    }
}
