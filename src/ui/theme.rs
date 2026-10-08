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

const SYMBOLIC_ICONS: &[(&str, &str)] = &[
    (
        "adw-entry-apply-symbolic.svg",
        include_str!("assets/symbolic/adw-entry-apply-symbolic.svg"),
    ),
    (
        "adw-entry-edit-symbolic.svg",
        include_str!("assets/symbolic/adw-entry-edit-symbolic.svg"),
    ),
    (
        "adw-external-link-symbolic.svg",
        include_str!("assets/symbolic/adw-external-link-symbolic.svg"),
    ),
    (
        "check-plain-symbolic.svg",
        include_str!("assets/symbolic/check-plain-symbolic.svg"),
    ),
    (
        "content-loading-symbolic.svg",
        include_str!("assets/symbolic/content-loading-symbolic.svg"),
    ),
    (
        "dialog-error-symbolic.svg",
        include_str!("assets/symbolic/dialog-error-symbolic.svg"),
    ),
    (
        "dialog-warning-symbolic.svg",
        include_str!("assets/symbolic/dialog-warning-symbolic.svg"),
    ),
    (
        "document-edit-symbolic.svg",
        include_str!("assets/symbolic/document-edit-symbolic.svg"),
    ),
    (
        "edit-clear-symbolic.svg",
        include_str!("assets/symbolic/edit-clear-symbolic.svg"),
    ),
    (
        "edit-copy-symbolic.svg",
        include_str!("assets/symbolic/edit-copy-symbolic.svg"),
    ),
    (
        "edit-delete-symbolic.svg",
        include_str!("assets/symbolic/edit-delete-symbolic.svg"),
    ),
    (
        "edit-find-symbolic.svg",
        include_str!("assets/symbolic/edit-find-symbolic.svg"),
    ),
    (
        "emblem-ok-symbolic.svg",
        include_str!("assets/symbolic/emblem-ok-symbolic.svg"),
    ),
    (
        "emblem-system-symbolic.svg",
        include_str!("assets/symbolic/emblem-system-symbolic.svg"),
    ),
    (
        "go-next-symbolic.svg",
        include_str!("assets/symbolic/go-next-symbolic.svg"),
    ),
    (
        "go-previous-symbolic.svg",
        include_str!("assets/symbolic/go-previous-symbolic.svg"),
    ),
    (
        "list-add-symbolic.svg",
        include_str!("assets/symbolic/list-add-symbolic.svg"),
    ),
    (
        "list-remove-symbolic.svg",
        include_str!("assets/symbolic/list-remove-symbolic.svg"),
    ),
    (
        "media-playback-pause-symbolic.svg",
        include_str!("assets/symbolic/media-playback-pause-symbolic.svg"),
    ),
    (
        "media-playback-start-symbolic.svg",
        include_str!("assets/symbolic/media-playback-start-symbolic.svg"),
    ),
    (
        "media-playback-stop-symbolic.svg",
        include_str!("assets/symbolic/media-playback-stop-symbolic.svg"),
    ),
    (
        "media-record-symbolic.svg",
        include_str!("assets/symbolic/media-record-symbolic.svg"),
    ),
    (
        "object-select-symbolic.svg",
        include_str!("assets/symbolic/object-select-symbolic.svg"),
    ),
    (
        "open-menu-symbolic.svg",
        include_str!("assets/symbolic/open-menu-symbolic.svg"),
    ),
    (
        "org.gnome.Settings-symbolic.svg",
        include_str!("assets/symbolic/org.gnome.Settings-symbolic.svg"),
    ),
    (
        "pan-down-symbolic.svg",
        include_str!("assets/symbolic/pan-down-symbolic.svg"),
    ),
    (
        "pan-end-symbolic.svg",
        include_str!("assets/symbolic/pan-end-symbolic.svg"),
    ),
    (
        "pan-start-symbolic.svg",
        include_str!("assets/symbolic/pan-start-symbolic.svg"),
    ),
    (
        "pan-up-symbolic.svg",
        include_str!("assets/symbolic/pan-up-symbolic.svg"),
    ),
    (
        "process-working-symbolic.svg",
        include_str!("assets/symbolic/process-working-symbolic.svg"),
    ),
    (
        "system-help-symbolic.svg",
        include_str!("assets/symbolic/system-help-symbolic.svg"),
    ),
    (
        "user-trash-symbolic.svg",
        include_str!("assets/symbolic/user-trash-symbolic.svg"),
    ),
    (
        "view-conceal-symbolic.svg",
        include_str!("assets/symbolic/view-conceal-symbolic.svg"),
    ),
    (
        "view-more-horizontal-symbolic.svg",
        include_str!("assets/symbolic/view-more-horizontal-symbolic.svg"),
    ),
    (
        "view-refresh-symbolic.svg",
        include_str!("assets/symbolic/view-refresh-symbolic.svg"),
    ),
    (
        "view-reveal-symbolic.svg",
        include_str!("assets/symbolic/view-reveal-symbolic.svg"),
    ),
    (
        "weather-clear-night-symbolic.svg",
        include_str!("assets/symbolic/weather-clear-night-symbolic.svg"),
    ),
    (
        "weather-clear-symbolic.svg",
        include_str!("assets/symbolic/weather-clear-symbolic.svg"),
    ),
    (
        "window-close-symbolic.svg",
        include_str!("assets/symbolic/window-close-symbolic.svg"),
    ),
    (
        "window-maximize-symbolic.svg",
        include_str!("assets/symbolic/window-maximize-symbolic.svg"),
    ),
    (
        "window-minimize-symbolic.svg",
        include_str!("assets/symbolic/window-minimize-symbolic.svg"),
    ),
    (
        "window-restore-symbolic.svg",
        include_str!("assets/symbolic/window-restore-symbolic.svg"),
    ),
    (
        "zoom-in-symbolic.svg",
        include_str!("assets/symbolic/zoom-in-symbolic.svg"),
    ),
    (
        "zoom-out-symbolic.svg",
        include_str!("assets/symbolic/zoom-out-symbolic.svg"),
    ),
];

/// Ensures the OpenDictate dark, light, standard, and symbolic SVG icons are installed in the user's
/// XDG icon directory and registered with the active `gtk4::IconTheme`.
pub fn ensure_app_icons_registered() {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};

    static REGISTERED: AtomicBool = AtomicBool::new(false);
    if !REGISTERED.swap(true, Ordering::SeqCst)
        && let Some(base_dirs) = directories::BaseDirs::new()
    {
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
            for &(filename, content) in SYMBOLIC_ICONS {
                let _ = std::fs::write(apps_dir.join(filename), content);
            }
        }
        let sym_dir = base_dirs
            .data_local_dir()
            .join("icons/hicolor/symbolic/apps");
        if std::fs::create_dir_all(&sym_dir).is_ok() {
            for &(filename, content) in SYMBOLIC_ICONS {
                let _ = std::fs::write(sym_dir.join(filename), content);
            }
        }
    }

    if gtk4::is_initialized_main_thread() {
        static ICON_THEME_REGISTERED: AtomicBool = AtomicBool::new(false);
        if !ICON_THEME_REGISTERED.swap(true, Ordering::SeqCst)
            && let Some(display) = gtk4::gdk::Display::default()
        {
            let settings = gtk4::Settings::for_display(&display);
            settings.set_gtk_icon_theme_name(Some("Adwaita"));
            settings.connect_gtk_icon_theme_name_notify(|s| {
                if s.gtk_icon_theme_name().as_deref() != Some("Adwaita") {
                    s.set_gtk_icon_theme_name(Some("Adwaita"));
                }
            });

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
            let manifest_symbolic =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ui/assets/symbolic");
            if manifest_symbolic.exists() {
                icon_theme.add_search_path(&manifest_symbolic);
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
                let user_sym = data_dir.join("icons/hicolor/symbolic/apps");
                if user_sym.exists() {
                    icon_theme.add_search_path(&user_sym);
                }
            }
            for sys_path in [
                "/usr/share/icons/Adwaita",
                "/usr/share/icons/hicolor",
                "/usr/share/icons",
            ] {
                let p = PathBuf::from(sys_path);
                if p.exists() {
                    icon_theme.add_search_path(&p);
                }
            }
            if let Ok(snap) = std::env::var("SNAP") {
                for sub in [
                    "data-dir/icons/Adwaita",
                    "data-dir/icons/hicolor",
                    "data-dir/icons",
                    "gnome-platform/usr/share/icons/Adwaita",
                    "gnome-platform/usr/share/icons/hicolor",
                    "gnome-platform/usr/share/icons",
                ] {
                    let p = PathBuf::from(&snap).join(sub);
                    if p.exists() {
                        icon_theme.add_search_path(&p);
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
