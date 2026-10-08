//! OpenDictate native binary entry point and Relm4 application bootstrap.

use gtk4::prelude::*;
use opendictate::cli::CliArgs;
use opendictate::config::Config;
use opendictate::services::hotkey::{HotkeyAction, HotkeyService};
use opendictate::services::storage::StorageService;
use opendictate::services::tray::{OpenDictateTray, TrayCallbacks, TrayService};
use opendictate::ui::main_window::{MainWindowInit, MainWindowModel, MainWindowMsg};
use opendictate::ui::theme::{ThemeMode, sync_theme_with_adwaita};
use relm4::MessageBroker;

static MAIN_BROKER: MessageBroker<MainWindowMsg> = MessageBroker::new();

fn main() {
    env_logger::init();
    opendictate::services::crash_reporter::CrashReporter::install_panic_hook();

    let cli = CliArgs::parse();

    if cli.version {
        println!("opendictate {}", opendictate::VERSION);
        return;
    }

    if cli.help {
        println!(
            "OpenDictate {} - Native Linux Voice Dictation & Speech-to-Text",
            opendictate::VERSION
        );
        println!();
        println!("USAGE:");
        println!("    opendictate [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -m, --minibar    Launch directly showing minibar floating island");
        println!("    -d, --dashboard  Launch directly showing main dashboard window");
        println!("    -t, --toggle     Toggle dictation recording");
        println!("    -v, --version    Print version information and exit");
        println!("    -h, --help       Print help information and exit");
        return;
    }

    let instance_sock_path = Config::default_path()
        .parent()
        .map(|p| p.join("instance.sock"))
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp/opendictate-instance.sock"));

    if let Ok(mut stream) = std::os::unix::net::UnixStream::connect(&instance_sock_path) {
        use std::io::Write;
        let cmd = if cli.toggle {
            "TOGGLE\n"
        } else if cli.minibar {
            "MINIBAR\n"
        } else {
            "DASHBOARD\n"
        };
        if stream.write_all(cmd.as_bytes()).is_ok() {
            log::info!(
                "Forwarded '{}' command to existing OpenDictate instance via {:?}",
                cmd.trim(),
                instance_sock_path
            );
            return;
        }
    }

    if cli.toggle {
        log::info!("Toggle dictation flag received");
        println!("OpenDictate dictation toggle requested.");
    }

    log::info!("Starting OpenDictate {}", opendictate::VERSION);

    // Initialize configuration
    let config = Config::load_or_default();

    // Ensure background Tokio runtime is active
    let _tokio_guard = opendictate::services::dictation_worker::tokio_handle().enter();

    // Start single-instance Unix domain socket listener for desktop launcher re-activation and CLI --toggle
    if let Some(parent) = instance_sock_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::remove_file(&instance_sock_path);
    if let Ok(listener) = std::os::unix::net::UnixListener::bind(&instance_sock_path) {
        std::thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            for stream in listener.incoming().flatten() {
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                if reader.read_line(&mut line).is_ok() {
                    match line.trim() {
                        "TOGGLE" => MAIN_BROKER.send(MainWindowMsg::TriggerDictationToggle),
                        "MINIBAR" => MAIN_BROKER.send(MainWindowMsg::ToggleMiniBar),
                        _ => MAIN_BROKER.send(MainWindowMsg::ShowDashboard),
                    }
                }
            }
        });
    }

    // Synchronize theme with Libadwaita StyleManager
    let theme_mode = ThemeMode::from_str(&config.theme);
    sync_theme_with_adwaita(theme_mode);

    // Initialize database storage
    let storage = StorageService::default_service().unwrap_or_else(|e| {
        log::warn!(
            "Could not initialize default storage: {}, using in-memory database",
            e
        );
        StorageService::in_memory().expect("Failed to initialize in-memory fallback database")
    });
    let _ = storage.seed_demo_dictations_if_empty();

    // Start DBus StatusNotifierItem system tray in background
    let callbacks = TrayCallbacks {
        on_toggle_dictation: Some(std::sync::Arc::new(|| {
            log::info!("Tray: toggle dictation clicked");
            MAIN_BROKER.send(MainWindowMsg::TriggerDictationToggle);
        })),
        on_show_minibar: Some(std::sync::Arc::new(|| {
            log::info!("Tray: show minibar clicked");
            MAIN_BROKER.send(MainWindowMsg::ToggleMiniBar);
        })),
        on_open_dashboard: Some(std::sync::Arc::new(|| {
            log::info!("Tray: open dashboard clicked");
            MAIN_BROKER.send(MainWindowMsg::ShowDashboard);
        })),
        on_set_theme: Some(std::sync::Arc::new(|dark| {
            log::info!("Tray: set theme (dark={})", dark);
            let mode = if dark {
                ThemeMode::Dark
            } else {
                ThemeMode::White
            };
            MAIN_BROKER.send(MainWindowMsg::SetTheme(mode));
        })),
        on_quit: Some(std::sync::Arc::new(|| {
            log::info!("Tray: quit requested");
            gtk4::glib::idle_add_once(|| {
                relm4::main_application().quit();
            });
        })),
    };
    let mut tray = OpenDictateTray::with_callbacks(callbacks);
    tray.set_dark_theme(theme_mode == ThemeMode::Dark);
    let tray_handle = TrayService::try_spawn(tray).ok();

    // Initialize global hotkey listener
    let mut hotkey_service = HotkeyService::new();
    if let Err(e) = hotkey_service.register_action(&config.hotkey, HotkeyAction::ToggleDictation) {
        log::warn!(
            "Could not register configured hotkey '{}': {}",
            config.hotkey,
            e
        );
    }
    hotkey_service.register_callback(|action| match action {
        HotkeyAction::ToggleDictation => {
            log::info!("Hotkey: toggle dictation triggered");
            MAIN_BROKER.send(MainWindowMsg::TriggerDictationToggle);
        }
        HotkeyAction::ShowMiniBar => {
            log::info!("Hotkey: show minibar triggered");
            MAIN_BROKER.send(MainWindowMsg::ToggleMiniBar);
        }
        HotkeyAction::ShowMainWindow => {
            log::info!("Hotkey: show main window triggered");
            MAIN_BROKER.send(MainWindowMsg::ShowDashboard);
        }
        _ => {}
    });

    // Bootstrap Relm4 application (filtering out app-specific flags from GTK parser)
    let exec_name = std::env::args()
        .next()
        .unwrap_or_else(|| "opendictate".to_string());

    let (show_minibar, show_dashboard) = if cli.minibar {
        (true, false)
    } else if cli.dashboard {
        (false, true)
    } else {
        (true, false) // Default launch: only floating MiniBar island appears at startup
    };

    log::info!(
        "Launching OpenDictate (show_minibar={}, show_dashboard={})...",
        show_minibar,
        show_dashboard
    );

    let app = relm4::RelmApp::new("io.github.opendictate.OpenDictate")
        .with_args(vec![exec_name])
        .with_broker(&MAIN_BROKER)
        .visible_on_activate(show_dashboard);

    // Use NON_UNIQUE so strict Snap AppArmor confinement never rejects D-Bus service name
    // registration (`org.freedesktop.DBus.Error.AccessDenied`). Single-instance IPC is
    // handled portably across Snap, Flatpak, AppImage, and native installs via `instance.sock`.
    relm4::main_application().set_flags(gtk4::gio::ApplicationFlags::NON_UNIQUE);

    let is_first_activate = std::cell::Cell::new(true);
    relm4::main_application().connect_activate(move |_| {
        if is_first_activate.replace(false) {
            return;
        }
        log::info!("Application re-activated via desktop launcher; presenting dashboard");
        MAIN_BROKER.send(MainWindowMsg::ShowDashboard);
    });

    app.run::<MainWindowModel>(MainWindowInit {
        config,
        storage,
        show_minibar,
        show_dashboard,
        tray_handle,
    });
    let _ = std::fs::remove_file(&instance_sock_path);
}
