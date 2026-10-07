//! Integration tests for system integration services:
//! - Text Injection (Clipboard paste and typewriter mode)
//! - Global Hotkey Service (parsing, normalization, dispatch)
//! - DBus System Tray (ksni StatusNotifierItem, dynamic icons, menus, states)
//! - CLI argument parsing (flags, help, version)

use opendictate::cli::CliArgs;
use opendictate::services::hotkey::{HotkeyAction, HotkeyError, HotkeyService};
use opendictate::services::tray::{OpenDictateTray, TrayCallbacks, TrayState};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn test_hotkey_shortcut_parsing_and_normalization() {
    // Normalization of various case and formats
    let parsed1 = HotkeyService::parse_shortcut("Ctrl+Alt+D").unwrap();
    assert!(parsed1.ctrl);
    assert!(parsed1.alt);
    assert!(!parsed1.shift);
    assert!(!parsed1.super_key);
    assert_eq!(parsed1.key, "D");
    assert_eq!(
        HotkeyService::normalize("Ctrl+Alt+D").unwrap(),
        "Ctrl+Alt+D"
    );

    let parsed2 = HotkeyService::parse_shortcut("<Super>d").unwrap();
    assert!(!parsed2.ctrl);
    assert!(!parsed2.alt);
    assert!(!parsed2.shift);
    assert!(parsed2.super_key);
    assert_eq!(parsed2.key, "D");
    assert_eq!(HotkeyService::normalize("<Super>d").unwrap(), "Super+D");

    let parsed3 = HotkeyService::parse_shortcut("<Control><Shift>space").unwrap();
    assert!(parsed3.ctrl);
    assert!(!parsed3.alt);
    assert!(parsed3.shift);
    assert!(!parsed3.super_key);
    assert_eq!(parsed3.key, "Space");
    assert_eq!(
        HotkeyService::normalize("<Control><Shift>space").unwrap(),
        "Ctrl+Shift+Space"
    );

    let parsed_f9 = HotkeyService::parse_shortcut("F9").unwrap();
    assert!(!parsed_f9.ctrl);
    assert!(!parsed_f9.alt);
    assert!(!parsed_f9.shift);
    assert!(!parsed_f9.super_key);
    assert_eq!(parsed_f9.key, "F9");
    assert_eq!(HotkeyService::normalize("F9").unwrap(), "F9");

    // Invalid shortcut
    let err = HotkeyService::parse_shortcut("").unwrap_err();
    match err {
        HotkeyError::InvalidShortcut(msg) => assert!(msg.contains("Empty")),
        _ => panic!("Expected InvalidShortcut error"),
    }
}

#[test]
fn test_hotkey_action_registration_and_dispatch() {
    let mut service = HotkeyService::new();

    service
        .register_action("Ctrl+Alt+D", HotkeyAction::ToggleDictation)
        .unwrap();
    service
        .register_action("<Super>m", HotkeyAction::ShowMiniBar)
        .unwrap();
    service
        .register_action("F10", HotkeyAction::ShowMainWindow)
        .unwrap();
    service
        .register_action("Escape", HotkeyAction::Cancel)
        .unwrap();

    let shortcuts = service.registered_shortcuts();
    assert_eq!(shortcuts.len(), 4);

    // Test callback dispatch
    let triggered_action = Arc::new(std::sync::Mutex::new(None));
    let t_clone = triggered_action.clone();
    service.register_callback(move |action| {
        *t_clone.lock().unwrap() = Some(action);
    });

    let ok = service.trigger_shortcut("ctrl+alt+d");
    assert!(ok);
    assert_eq!(
        *triggered_action.lock().unwrap(),
        Some(HotkeyAction::ToggleDictation)
    );

    let ok2 = service.trigger_shortcut("Super+M");
    assert!(ok2);
    assert_eq!(
        *triggered_action.lock().unwrap(),
        Some(HotkeyAction::ShowMiniBar)
    );

    let ok_missing = service.trigger_shortcut("Alt+F4");
    assert!(!ok_missing);
}

#[tokio::test]
async fn test_hotkey_tokio_channel_dispatch() {
    let mut service = HotkeyService::new();
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    service.register_sender(tx);

    service
        .register_action("Ctrl+Alt+D", HotkeyAction::ToggleDictation)
        .unwrap();

    // Trigger action directly
    service.trigger_action(HotkeyAction::ToggleDictation);

    let received = rx.recv().await;
    assert_eq!(received, Some(HotkeyAction::ToggleDictation));

    // Trigger via shortcut
    service.trigger_shortcut("Ctrl+Alt+D");
    let received2 = rx.recv().await;
    assert_eq!(received2, Some(HotkeyAction::ToggleDictation));
}

#[test]
fn test_opendictate_tray_properties_and_state_icons() {
    use ksni::Tray;

    let mut tray = OpenDictateTray::new();
    assert_eq!(tray.id(), "opendictate");
    assert_eq!(tray.title(), "OpenDictate");

    // Check tooltip
    let tooltip = tray.tool_tip();
    assert_eq!(tooltip.title, "OpenDictate");
    assert_eq!(
        tooltip.description,
        "OpenDictate - Voice Dictation & Intelligence"
    );

    // Icon based on state & theme
    assert_eq!(tray.state(), TrayState::Idle);
    assert_eq!(tray.icon_name(), "opendictate-dark");
    assert!(!tray.icon_theme_path().is_empty());

    tray.set_dark_theme(false);
    assert_eq!(tray.icon_name(), "opendictate-light");

    tray.set_dark_theme(true);
    assert_eq!(tray.icon_name(), "opendictate-dark");

    tray.set_state(TrayState::Recording);
    assert_eq!(tray.state(), TrayState::Recording);
    assert_eq!(tray.icon_name(), "media-record-symbolic");

    tray.set_state(TrayState::Paused);
    assert_eq!(tray.state(), TrayState::Paused);
    assert_eq!(tray.icon_name(), "media-playback-pause-symbolic");

    tray.set_state(TrayState::Processing);
    assert_eq!(tray.state(), TrayState::Processing);
    assert_eq!(tray.icon_name(), "process-working-symbolic");
}

#[test]
fn test_opendictate_tray_menu_structure_and_callbacks() {
    use ksni::Tray;

    let toggle_fired = Arc::new(AtomicBool::new(false));
    let minibar_fired = Arc::new(AtomicBool::new(false));
    let dashboard_fired = Arc::new(AtomicBool::new(false));
    let theme_mode_set = Arc::new(std::sync::Mutex::new(None));

    let callbacks = TrayCallbacks {
        on_toggle_dictation: {
            let flag = toggle_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_show_minibar: {
            let flag = minibar_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_open_dashboard: {
            let flag = dashboard_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_set_theme: {
            let val = theme_mode_set.clone();
            Some(Arc::new(move |dark| *val.lock().unwrap() = Some(dark)))
        },
        on_quit: Some(Arc::new(|| {})),
    };

    let mut tray = OpenDictateTray::with_callbacks(callbacks);
    assert!(tray.is_dark_theme());

    let menu = tray.menu();
    // Verify menu items present (Open Dashboard removed):
    // 0: Toggle Dictation
    // 1: Show Mini-Bar
    // 2: Separator
    // 3: SubMenu Theme (Dark, White)
    // 4: Separator
    // 5: Quit OpenDictate
    assert_eq!(menu.len(), 6);

    // Verify items
    match &menu[0] {
        ksni::MenuItem::Standard(item) => {
            assert_eq!(item.label, "Toggle Dictation");
            (item.activate)(&mut tray);
            assert!(toggle_fired.load(Ordering::SeqCst));
        }
        _ => panic!("Expected Standard item for Toggle Dictation"),
    }

    match &menu[1] {
        ksni::MenuItem::Standard(item) => {
            assert_eq!(item.label, "Show Mini-Bar");
            (item.activate)(&mut tray);
            assert!(minibar_fired.load(Ordering::SeqCst));
        }
        _ => panic!("Expected Standard item for Show Mini-Bar"),
    }

    match &menu[2] {
        ksni::MenuItem::Separator => {}
        _ => panic!("Expected Separator at position 2"),
    }

    match &menu[3] {
        ksni::MenuItem::SubMenu(sub) => {
            assert_eq!(sub.label, "Theme");
            assert_eq!(sub.submenu.len(), 2);
            match &sub.submenu[0] {
                ksni::MenuItem::Checkmark(item) => {
                    assert_eq!(item.label, "Dark");
                    assert!(item.checked);
                }
                _ => panic!("Expected Dark Checkmark"),
            }
            match &sub.submenu[1] {
                ksni::MenuItem::Checkmark(item) => {
                    assert_eq!(item.label, "White");
                    assert!(!item.checked);
                    (item.activate)(&mut tray);
                    assert_eq!(*theme_mode_set.lock().unwrap(), Some(false));
                    assert!(!tray.is_dark_theme());
                }
                _ => panic!("Expected White Checkmark"),
            }
        }
        _ => panic!("Expected SubMenu at position 3"),
    }

    match &menu[4] {
        ksni::MenuItem::Separator => {}
        _ => panic!("Expected Separator at position 4"),
    }

    match &menu[5] {
        ksni::MenuItem::Standard(item) => {
            assert_eq!(item.label, "Quit OpenDictate");
        }
        _ => panic!("Expected Standard item for Quit OpenDictate"),
    }
}

#[test]
fn test_cli_arguments_parser() {
    let args_default = CliArgs::parse_from(Vec::<String>::new());
    assert!(!args_default.minibar);
    assert!(!args_default.dashboard);
    assert!(!args_default.toggle);
    assert!(!args_default.version);
    assert!(!args_default.help);

    let args_minibar = CliArgs::parse_from(["--minibar"]);
    assert!(args_minibar.minibar);
    assert!(!args_minibar.dashboard);
    assert!(!args_minibar.toggle);

    let args_short_minibar = CliArgs::parse_from(["-m"]);
    assert!(args_short_minibar.minibar);

    let args_dashboard = CliArgs::parse_from(["--dashboard"]);
    assert!(args_dashboard.dashboard);
    assert!(!args_dashboard.minibar);

    let args_short_dashboard = CliArgs::parse_from(["-d"]);
    assert!(args_short_dashboard.dashboard);

    let args_toggle = CliArgs::parse_from(["--toggle"]);
    assert!(args_toggle.toggle);

    let args_short_toggle = CliArgs::parse_from(["-t"]);
    assert!(args_short_toggle.toggle);

    let args_version = CliArgs::parse_from(["--version"]);
    assert!(args_version.version);

    let args_short_version = CliArgs::parse_from(["-v"]);
    assert!(args_short_version.version);

    let args_help = CliArgs::parse_from(["--help"]);
    assert!(args_help.help);

    let args_short_help = CliArgs::parse_from(["-h"]);
    assert!(args_short_help.help);

    let args_combined = CliArgs::parse_from(["--minibar", "--toggle"]);
    assert!(args_combined.minibar);
    assert!(args_combined.toggle);
}

#[test]
fn test_opendictate_tray_trigger_helpers() {
    let toggle_fired = Arc::new(AtomicBool::new(false));
    let minibar_fired = Arc::new(AtomicBool::new(false));
    let dashboard_fired = Arc::new(AtomicBool::new(false));
    let theme_mode_set = Arc::new(std::sync::Mutex::new(None));
    let quit_fired = Arc::new(AtomicBool::new(false));

    let callbacks = TrayCallbacks {
        on_toggle_dictation: {
            let flag = toggle_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_show_minibar: {
            let flag = minibar_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_open_dashboard: {
            let flag = dashboard_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
        on_set_theme: {
            let val = theme_mode_set.clone();
            Some(Arc::new(move |dark| *val.lock().unwrap() = Some(dark)))
        },
        on_quit: {
            let flag = quit_fired.clone();
            Some(Arc::new(move || flag.store(true, Ordering::SeqCst)))
        },
    };

    let mut tray = OpenDictateTray::with_callbacks(callbacks);

    tray.trigger_toggle();
    assert!(toggle_fired.load(Ordering::SeqCst));

    tray.trigger_show_minibar();
    assert!(minibar_fired.load(Ordering::SeqCst));

    tray.trigger_open_dashboard();
    assert!(dashboard_fired.load(Ordering::SeqCst));

    tray.trigger_set_theme(false);
    assert_eq!(*theme_mode_set.lock().unwrap(), Some(false));
    assert!(!tray.is_dark_theme());

    tray.trigger_set_theme(true);
    assert_eq!(*theme_mode_set.lock().unwrap(), Some(true));
    assert!(tray.is_dark_theme());

    tray.trigger_quit();
    assert!(quit_fired.load(Ordering::SeqCst));
}

#[test]
fn test_hotkey_and_tray_message_broker_integration() {
    use opendictate::ui::main_window::MainWindowMsg;
    use opendictate::ui::theme::ThemeMode;

    let received_msgs = Arc::new(std::sync::Mutex::new(Vec::<MainWindowMsg>::new()));
    let rx_clone = received_msgs.clone();

    // Mock broker dispatch via channel or direct callback
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MainWindowMsg>();

    let tx_toggle = tx.clone();
    let tx_minibar = tx.clone();
    let tx_dashboard = tx.clone();
    let tx_theme = tx.clone();

    let callbacks = TrayCallbacks {
        on_toggle_dictation: Some(Arc::new(move || {
            let _ = tx_toggle.send(MainWindowMsg::TriggerDictationToggle);
        })),
        on_show_minibar: Some(Arc::new(move || {
            let _ = tx_minibar.send(MainWindowMsg::ToggleMiniBar);
        })),
        on_open_dashboard: Some(Arc::new(move || {
            let _ = tx_dashboard.send(MainWindowMsg::ShowDashboard);
        })),
        on_set_theme: Some(Arc::new(move |dark| {
            let mode = if dark {
                ThemeMode::Dark
            } else {
                ThemeMode::White
            };
            let _ = tx_theme.send(MainWindowMsg::SetTheme(mode));
        })),
        on_quit: None,
    };

    let mut tray = OpenDictateTray::with_callbacks(callbacks);
    tray.trigger_toggle();
    tray.trigger_show_minibar();
    tray.trigger_open_dashboard();
    tray.trigger_set_theme(false);

    // Also trigger via HotkeyService
    let mut hotkey_service = HotkeyService::new();
    let tx_hk_toggle = tx.clone();
    let tx_hk_minibar = tx.clone();
    let tx_hk_dashboard = tx.clone();

    hotkey_service.register_callback(move |action| match action {
        HotkeyAction::ToggleDictation => {
            let _ = tx_hk_toggle.send(MainWindowMsg::TriggerDictationToggle);
        }
        HotkeyAction::ShowMiniBar => {
            let _ = tx_hk_minibar.send(MainWindowMsg::ToggleMiniBar);
        }
        HotkeyAction::ShowMainWindow => {
            let _ = tx_hk_dashboard.send(MainWindowMsg::ShowDashboard);
        }
        _ => {}
    });

    hotkey_service.trigger_action(HotkeyAction::ToggleDictation);
    hotkey_service.trigger_action(HotkeyAction::ShowMiniBar);
    hotkey_service.trigger_action(HotkeyAction::ShowMainWindow);

    // Collect messages
    drop(tx);
    while let Ok(msg) = rx.try_recv() {
        rx_clone.lock().unwrap().push(msg);
    }

    let msgs = received_msgs.lock().unwrap().clone();
    assert_eq!(msgs.len(), 7);
    assert_eq!(msgs[0], MainWindowMsg::TriggerDictationToggle);
    assert_eq!(msgs[1], MainWindowMsg::ToggleMiniBar);
    assert_eq!(msgs[2], MainWindowMsg::ShowDashboard);
    assert_eq!(msgs[3], MainWindowMsg::SetTheme(ThemeMode::White));
    assert_eq!(msgs[4], MainWindowMsg::TriggerDictationToggle);
    assert_eq!(msgs[5], MainWindowMsg::ToggleMiniBar);
    assert_eq!(msgs[6], MainWindowMsg::ShowDashboard);
}

#[test]
fn test_main_window_show_dashboard_and_init_tray() {
    use opendictate::config::Config;
    use opendictate::services::storage::StorageService;
    use opendictate::ui::main_window::{MainWindowInit, MainWindowModel, MainWindowMsg};

    let config = Config::default();
    let storage = StorageService::in_memory().unwrap();
    let init = MainWindowInit::new(config.clone(), storage.clone());
    assert!(init.tray_handle.is_none());

    let mut model = MainWindowModel::new(config, storage);
    assert!(!model.is_dashboard_visible());

    model.update(MainWindowMsg::ShowDashboard);
    assert!(model.is_dashboard_visible());
}
