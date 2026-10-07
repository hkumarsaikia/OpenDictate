//! Theme synchronization with Libadwaita and GNOME styling.

use serde::{Deserialize, Serialize};

/// Application theme modes supported by OpenDictate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    White,
}

impl ThemeMode {
    /// Returns the lowercase string identifier for the theme mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::White => "white",
        }
    }

    /// Parses a theme mode from a string slice (case-insensitive), defaulting to Dark.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "white" | "light" => Self::White,
            _ => Self::Dark,
        }
    }

    /// Returns true if this is dark mode.
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Toggles between Dark and White theme mode.
    pub fn toggle(&self) -> Self {
        match self {
            Self::Dark => Self::White,
            Self::White => Self::Dark,
        }
    }

    /// Returns the icon name corresponding to the active theme mode.
    pub fn icon_name(&self) -> &'static str {
        match self {
            Self::Dark => "opendictate-dark",
            Self::White => "opendictate-light",
        }
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

const SVG_ICON_DARK: &str = include_str!("assets/brand/opendictate-dark.svg");
const SVG_ICON_LIGHT: &str = include_str!("assets/brand/opendictate-light.svg");

/// Ensures the OpenDictate dark, light, and standard SVG icons are installed in the user's
/// XDG icon directory and registered with the active `gtk4::IconTheme`.
pub fn ensure_app_icons_registered() {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};

    static REGISTERED: AtomicBool = AtomicBool::new(false);
    if !REGISTERED.swap(true, Ordering::SeqCst) {
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let apps_dir = base_dirs
                .data_local_dir()
                .join("icons/hicolor/scalable/apps");
            if std::fs::create_dir_all(&apps_dir).is_ok() {
                let _ = std::fs::write(apps_dir.join("opendictate.svg"), SVG_ICON_DARK);
                let _ = std::fs::write(apps_dir.join("opendictate-dark.svg"), SVG_ICON_DARK);
                let _ = std::fs::write(apps_dir.join("opendictate-light.svg"), SVG_ICON_LIGHT);
                let _ = std::fs::write(
                    apps_dir.join("io.github.opendictate.OpenDictate.svg"),
                    SVG_ICON_DARK,
                );
            }
        }
    }

    if gtk4::is_initialized_main_thread() {
        static ICON_THEME_REGISTERED: AtomicBool = AtomicBool::new(false);
        if !ICON_THEME_REGISTERED.swap(true, Ordering::SeqCst) {
            if let Some(display) = gtk4::gdk::Display::default() {
                let icon_theme = gtk4::IconTheme::for_display(&display);
                let manifest_brand =
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ui/assets/brand");
                if manifest_brand.exists() {
                    icon_theme.add_search_path(&manifest_brand);
                    let icons_sub = manifest_brand.join("icons");
                    if icons_sub.exists() {
                        icon_theme.add_search_path(&icons_sub);
                    }
                }
                if let Some(base_dirs) = directories::BaseDirs::new() {
                    let data_dir = base_dirs.data_local_dir();
                    let user_icons = data_dir.join("icons");
                    if user_icons.exists() {
                        icon_theme.add_search_path(&user_icons);
                    }
                    let user_apps = data_dir.join("icons/hicolor/scalable/apps");
                    if user_apps.exists() {
                        icon_theme.add_search_path(&user_apps);
                    }
                }
            }
        }
    }
}

/// Synchronizes the global Libadwaita StyleManager color scheme and default window icon with the given theme mode.
pub fn sync_theme_with_adwaita(mode: ThemeMode) {
    if gtk4::is_initialized_main_thread() {
        let _ = libadwaita::init();
        ensure_app_icons_registered();
        gtk4::Window::set_default_icon_name(mode.icon_name());
        let style_manager = libadwaita::StyleManager::default();
        match mode {
            ThemeMode::Dark => {
                style_manager.set_color_scheme(libadwaita::ColorScheme::ForceDark);
            }
            ThemeMode::White => {
                style_manager.set_color_scheme(libadwaita::ColorScheme::ForceLight);
            }
        }
    } else if gtk4::is_initialized() {
        // Safely dispatch to GTK main thread if called from a background thread
        gtk4::glib::idle_add_once(move || {
            sync_theme_with_adwaita(mode);
        });
    }
}

/// Retrieves the current theme mode from Libadwaita StyleManager if GTK is initialized.
pub fn get_current_theme_mode() -> ThemeMode {
    if gtk4::is_initialized_main_thread() {
        let _ = libadwaita::init();
        let style_manager = libadwaita::StyleManager::default();
        if style_manager.is_dark() {
            ThemeMode::Dark
        } else {
            ThemeMode::White
        }
    } else {
        ThemeMode::Dark
    }
}
