//! Integration tests for OpenDictate Native GNOME MainWindow (Relm4 + Libadwaita).

use gtk4::prelude::*;
use libadwaita::prelude::*;
use opendictate::config::Config;
use opendictate::services::storage::StorageService;
use opendictate::ui::main_window::{MainWindowInit, MainWindowModel, MainWindowMsg};
use opendictate::ui::theme::{ThemeMode, get_current_theme_mode, sync_theme_with_adwaita};
use tempfile::NamedTempFile;

#[test]
fn test_theme_mode_enum_and_style_manager_wiring() {
    let dark = ThemeMode::Dark;
    assert_eq!(dark.as_str(), "dark");
    assert!(dark.is_dark());

    let white = ThemeMode::White;
    assert_eq!(white.as_str(), "white");
    assert!(!white.is_dark());

    assert_eq!(ThemeMode::default(), ThemeMode::Dark);

    // Case-insensitive parsing
    assert_eq!(ThemeMode::from_str("dark"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("DARK"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("white"), ThemeMode::White);
    assert_eq!(ThemeMode::from_str("White"), ThemeMode::White);
    assert_eq!(ThemeMode::from_str("unknown"), ThemeMode::Dark);

    // Serde roundtrip
    let serialized = serde_json::to_string(&ThemeMode::Dark).unwrap();
    assert_eq!(serialized, "\"dark\"");
    let deserialized: ThemeMode = serde_json::from_str("\"white\"").unwrap();
    assert_eq!(deserialized, ThemeMode::White);
}

#[test]
fn test_theme_mode_sync_with_adwaita() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        sync_theme_with_adwaita(ThemeMode::Dark);
        assert_eq!(get_current_theme_mode(), ThemeMode::Dark);

        sync_theme_with_adwaita(ThemeMode::White);
        assert_eq!(get_current_theme_mode(), ThemeMode::White);

        // Reset to Dark
        sync_theme_with_adwaita(ThemeMode::Dark);
        assert_eq!(get_current_theme_mode(), ThemeMode::Dark);
    }
}

#[test]
fn test_main_window_model_settings_and_theme_toggle() {
    let tmp = NamedTempFile::new().unwrap();
    let storage = StorageService::new(tmp.path()).unwrap();

    let config_tmp = NamedTempFile::new().unwrap();
    let config = Config {
        theme: "dark".to_string(),
        ai_provider: "gemini".to_string(),
        ..Default::default()
    };
    config.save_to(config_tmp.path()).unwrap();

    let mut model =
        MainWindowModel::new_with_config_path(config, storage, config_tmp.path().to_path_buf());

    assert_eq!(model.theme_mode(), ThemeMode::Dark);

    // Toggle theme
    model.update(MainWindowMsg::ToggleTheme);
    assert_eq!(model.theme_mode(), ThemeMode::White);
    assert_eq!(model.config().theme, "white");

    // Toggle theme back
    model.update(MainWindowMsg::ToggleTheme);
    assert_eq!(model.theme_mode(), ThemeMode::Dark);
    assert_eq!(model.config().theme, "dark");

    // Update AI settings
    model.update(MainWindowMsg::SetAiProvider("groq".to_string()));
    assert_eq!(model.config().ai_provider, "groq");

    model.update(MainWindowMsg::SetAiApiKey("gsk_secret123".to_string()));
    assert_eq!(model.config().ai_api_key, "gsk_secret123");

    model.update(MainWindowMsg::SetAiModel(
        "whisper-large-v3-turbo".to_string(),
    ));
    assert_eq!(model.config().ai_model, "whisper-large-v3-turbo");

    // Update Local AI settings & mode
    model.update(MainWindowMsg::SetAiMode("local".to_string()));
    assert_eq!(model.config().ai_mode, "local");

    model.update(MainWindowMsg::SetLocalModelId("tiny.en".to_string()));
    assert_eq!(model.config().local_model_id, "tiny.en");

    model.update(MainWindowMsg::SetLocalCustomPath(Some(
        "/tmp/custom.bin".to_string(),
    )));
    assert_eq!(
        model.config().local_custom_path,
        Some("/tmp/custom.bin".to_string())
    );

    model.update(MainWindowMsg::SetLocalThreads(6));
    assert_eq!(model.config().local_threads, 6);

    // Update Audio & System settings
    model.update(MainWindowMsg::SetAudioDevice(Some(
        "PulseAudio Default".to_string(),
    )));
    assert_eq!(
        model.config().audio_device,
        Some("PulseAudio Default".to_string())
    );

    model.update(MainWindowMsg::SetHotkey("Ctrl+Shift+Space".to_string()));
    assert_eq!(model.config().hotkey, "Ctrl+Shift+Space");

    // Save settings
    model.update(MainWindowMsg::SaveSettings);

    // Verify persisted config on disk
    let loaded = Config::load_from(config_tmp.path()).unwrap();
    assert_eq!(loaded.theme, "dark");
    assert_eq!(loaded.ai_mode, "local");
    assert_eq!(loaded.local_model_id, "tiny.en");
    assert_eq!(
        loaded.local_custom_path,
        Some("/tmp/custom.bin".to_string())
    );
    assert_eq!(loaded.local_threads, 6);
    assert_eq!(loaded.ai_provider, "groq");
    assert_eq!(loaded.ai_api_key, "gsk_secret123");
    assert_eq!(loaded.ai_model, "whisper-large-v3-turbo");
    assert_eq!(loaded.audio_device, Some("PulseAudio Default".to_string()));
    assert_eq!(loaded.hotkey, "Ctrl+Shift+Space");
}

#[test]
fn test_ui_widget_builders_construct_valid_gtk_objects() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let tmp = NamedTempFile::new().unwrap();
        let storage = StorageService::new(tmp.path()).unwrap();
        let config = Config::default();

        let (header, header_widgets) =
            opendictate::ui::main_window::header::build_header_bar(&config, &storage);
        assert!(header.is::<libadwaita::HeaderBar>());
        assert_eq!(header_widgets.title_widget.subtitle().as_str(), "");

        let (settings_widget, _settings_widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);
        assert!(settings_widget.is::<libadwaita::PreferencesPage>());

        let window = opendictate::ui::main_window::build_main_window(&config, &storage);
        assert!(window.is::<libadwaita::ApplicationWindow>());
    }
}

#[test]
fn test_main_window_minibar_integration() {
    let tmp = NamedTempFile::new().unwrap();
    let storage = StorageService::new(tmp.path()).unwrap();
    let config = Config::default();
    let cfg_tmp = NamedTempFile::new().unwrap();

    // Verify MainWindowInit defaults
    let init = MainWindowInit::new(config.clone(), storage.clone());
    assert!(init.show_minibar);
    assert!(init.show_dashboard);

    // Verify model responds to minibar toggle
    let mut model =
        MainWindowModel::new_with_config_path(config, storage, cfg_tmp.path().to_path_buf());
    assert!(!model.is_minibar_visible());

    model.set_minibar_visible(true);
    assert!(model.is_minibar_visible());

    model.update(MainWindowMsg::ToggleMiniBar);
    assert!(!model.is_minibar_visible());

    model.update(MainWindowMsg::ToggleMiniBar);
    assert!(model.is_minibar_visible());
}

#[test]
fn test_main_window_dictation_lifecycle_events() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let storage = StorageService::new(tmp.path()).unwrap();
    let config = Config::default();
    let cfg_tmp = tempfile::NamedTempFile::new().unwrap();
    let mut model =
        MainWindowModel::new_with_config_path(config, storage, cfg_tmp.path().to_path_buf());

    model.update(MainWindowMsg::DictationStatus("Recording...".to_string()));
    assert_eq!(model.dictation_status(), "Recording...");

    model.update(MainWindowMsg::DictationAudioLevel(0.85));
    assert!((model.audio_level() - 0.85).abs() < 1e-4);

    model.update(MainWindowMsg::DictationProcessing);
    assert!(model.is_processing());
    assert_eq!(model.dictation_status(), "Transcribing...");

    model.update(MainWindowMsg::DictationSuccess {
        raw: "Hello world".to_string(),
        enhanced: "Hello, world!".to_string(),
    });
    assert!(!model.is_processing());
    assert_eq!(model.dictation_status(), "Ready");
    assert_eq!(model.last_raw_text(), "Hello world");
    assert_eq!(model.last_enhanced_text(), "Hello, world!");

    // Test error handling
    model.update(MainWindowMsg::DictationError("Audio failure".to_string()));
    assert_eq!(model.dictation_status(), "Error: Audio failure");
    assert_eq!(model.audio_level(), 0.0);

    // Test tone setting and config synchronization
    assert_eq!(model.tone(), "Clean");
    model.update(MainWindowMsg::SetTone("Concise".to_string()));
    assert_eq!(model.tone(), "Concise");
    assert_eq!(model.config().tone, "Concise");

    // Test No speech detected resets processing state and audio level
    model.update(MainWindowMsg::DictationProcessing);
    model.update(MainWindowMsg::DictationAudioLevel(0.42));
    assert!(model.is_processing());
    model.update(MainWindowMsg::DictationStatus(
        "No speech detected".to_string(),
    ));
    assert!(!model.is_processing());
    assert_eq!(model.audio_level(), 0.0);
    assert_eq!(model.dictation_status(), "No speech detected");
}

#[test]
fn test_main_window_worker_channel_integration() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let storage = StorageService::new(tmp.path()).unwrap();
    let config_tmp = tempfile::NamedTempFile::new().unwrap();
    let config = Config {
        tone: "Professional".to_string(),
        ..Default::default()
    };
    config.save_to(config_tmp.path()).unwrap();

    let mut model =
        MainWindowModel::new_with_config_path(config, storage, config_tmp.path().to_path_buf());
    assert_eq!(model.tone(), "Professional");

    // Attach a test channel for worker_sender
    let (tx, rx) =
        relm4::channel::<opendictate::services::dictation_worker::DictationWorkerInput>();
    model.set_worker_sender(tx);

    // Test TriggerDictationToggle sends ToggleRecording with current tone
    model.update(MainWindowMsg::TriggerDictationToggle);
    let input = rx.recv_sync().unwrap();
    assert_eq!(
        input,
        opendictate::services::dictation_worker::DictationWorkerInput::ToggleRecording {
            tone: "Professional".to_string()
        }
    );

    // Test SaveSettings sends UpdateConfig to worker
    model.update(MainWindowMsg::SetAiMode("local".to_string()));
    model.update(MainWindowMsg::SetLocalModelId("tiny.en".to_string()));
    model.update(MainWindowMsg::SetLocalCustomPath(Some(
        "/path/to/custom.bin".to_string(),
    )));
    model.update(MainWindowMsg::SetLocalThreads(8));
    model.update(MainWindowMsg::SetAiProvider("groq".to_string()));
    model.update(MainWindowMsg::SaveSettings);
    let input = rx.recv_sync().unwrap();
    match input {
        opendictate::services::dictation_worker::DictationWorkerInput::UpdateConfig(cfg) => {
            assert_eq!(cfg.ai_provider, "groq");
            assert_eq!(cfg.ai_mode, "local");
            assert_eq!(cfg.local_model_id, "tiny.en");
            assert_eq!(
                cfg.local_custom_path,
                Some("/path/to/custom.bin".to_string())
            );
            assert_eq!(cfg.local_threads, 8);
        }
        other => panic!("Expected UpdateConfig, got {:?}", other),
    }

    // Test SetTone persists to disk
    model.update(MainWindowMsg::SetTone("Raw".to_string()));
    assert_eq!(model.tone(), "Raw");
    let loaded = Config::load_from(config_tmp.path()).unwrap();
    assert_eq!(loaded.tone, "Raw");
}

#[test]
fn test_white_and_dark_theme_ui_classes_and_icons() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let tmp = tempfile::NamedTempFile::new().unwrap();
        let storage = StorageService::new(tmp.path()).unwrap();
        let mut config = Config {
            theme: "dark".to_string(),
            ..Default::default()
        };

        let window = opendictate::ui::main_window::build_main_window(&config, &storage);
        assert!(window.has_css_class("dark-theme"));
        assert!(!window.has_css_class("white-theme"));

        // Build with white theme
        config.theme = "white".to_string();
        let white_window = opendictate::ui::main_window::build_main_window(&config, &storage);
        assert!(white_window.has_css_class("white-theme"));
        assert!(!white_window.has_css_class("dark-theme"));

        // Build minibar widgets and verify theme_button icon and tooltip
        let minibar_win = gtk4::Window::new();
        let mb_widgets = opendictate::ui::mini_bar::build_minibar_widgets(&minibar_win);
        assert_eq!(
            mb_widgets.theme_button.icon_name().unwrap().as_str(),
            "weather-clear-symbolic"
        );
        assert_eq!(
            mb_widgets.theme_button.tooltip_text().unwrap().as_str(),
            "Switch Theme"
        );
        assert_eq!(
            mb_widgets.tone_dropdown.tooltip_text().unwrap().as_str(),
            "Select Tone"
        );
        assert!(mb_widgets.visualizer_area.tooltip_text().is_none());
        assert!(
            mb_widgets
                .record_button
                .has_css_class("minibar-record-button")
        );
        assert!(mb_widgets.record_button.has_css_class("suggested-action"));
        assert!(mb_widgets.record_button.has_css_class("circular"));
    }
}

#[test]
fn test_model_picker_gating_and_hover_card_visibility() {
    use libadwaita::prelude::*;

    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_api_key: String::new(),
            ai_provider: "gemini".to_string(),
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // 1. When no API key is provided, model_row must NOT be activatable, and dropdown/detail popover must be hidden
        assert!(!widgets.model_row.is_activatable());
        assert_eq!(
            widgets.picker_components.status_label.text().as_str(),
            "Paste API Key to fetch models..."
        );
        assert!(!widgets.picker_components.dropdown_button.is_visible());
        assert!(!widgets.picker_components.detail_popover.is_visible());

        // 2. Cross-provider key prevention tests:
        // 2a. Pasting Groq key into NVIDIA NIM must immediately reject with specific message
        opendictate::ui::main_window::settings_view::trigger_model_fetch(
            &widgets.model_row,
            &widgets.picker_components,
            widgets.current_models.clone(),
            widgets.selected_model_id.clone(),
            "nvidia",
            "gsk_N7e4ZBayh1PvkUIbKLBuWGdyb3FY",
            None,
        );
        assert!(!widgets.model_row.is_activatable());
        assert!(!widgets.picker_components.dropdown_button.is_visible());
        assert!(
            widgets
                .picker_components
                .status_label
                .text()
                .as_str()
                .contains("Groq API key")
        );
        assert!(widgets.current_models.borrow().is_empty());

        // 2b. Pasting invalid non-nvapi key into NVIDIA must reject with prefix requirement
        opendictate::ui::main_window::settings_view::trigger_model_fetch(
            &widgets.model_row,
            &widgets.picker_components,
            widgets.current_models.clone(),
            widgets.selected_model_id.clone(),
            "nvidia",
            "invalid_key_random",
            None,
        );
        assert!(!widgets.model_row.is_activatable());
        assert!(!widgets.picker_components.dropdown_button.is_visible());
        assert!(
            widgets
                .picker_components
                .status_label
                .text()
                .as_str()
                .contains("nvapi-")
        );
        assert!(widgets.current_models.borrow().is_empty());

        // 2c. Pasting NVIDIA key into Groq must immediately reject
        opendictate::ui::main_window::settings_view::trigger_model_fetch(
            &widgets.model_row,
            &widgets.picker_components,
            widgets.current_models.clone(),
            widgets.selected_model_id.clone(),
            "groq",
            "nvapi-1s5fQoiYSLVw6jqKwbXL3QKBe",
            None,
        );
        assert!(!widgets.model_row.is_activatable());
        assert!(!widgets.picker_components.dropdown_button.is_visible());
        assert!(
            widgets
                .picker_components
                .status_label
                .text()
                .as_str()
                .contains("NVIDIA NIM API key")
        );
        assert!(widgets.current_models.borrow().is_empty());

        // 2d. Pasting Hugging Face token into Cerebras must immediately reject
        opendictate::ui::main_window::settings_view::trigger_model_fetch(
            &widgets.model_row,
            &widgets.picker_components,
            widgets.current_models.clone(),
            widgets.selected_model_id.clone(),
            "cerebras",
            "hf_qSaAMTIDvbQMxAgLafqCUKDQnl",
            None,
        );
        assert!(!widgets.model_row.is_activatable());
        assert!(!widgets.picker_components.dropdown_button.is_visible());
        assert!(
            widgets
                .picker_components
                .status_label
                .text()
                .as_str()
                .contains("Hugging Face")
        );
        assert!(widgets.current_models.borrow().is_empty());

        // 3. When a valid format API key is entered, placeholder text immediately clears and downward arrow appears
        opendictate::ui::main_window::settings_view::trigger_model_fetch(
            &widgets.model_row,
            &widgets.picker_components,
            widgets.current_models.clone(),
            widgets.selected_model_id.clone(),
            "gemini",
            "dummy_key_123",
            None,
        );
        assert!(widgets.model_row.is_activatable());
        assert_eq!(widgets.picker_components.status_label.text().as_str(), "");
        assert!(widgets.picker_components.dropdown_button.is_visible());

        // 4. When populate_model_picker is called with empty models, detail_popover remains hidden
        let sel_id = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        opendictate::ui::main_window::settings_view::populate_model_picker(
            &widgets.picker_components,
            &widgets.model_row,
            &[],
            "gemini",
            &sel_id,
        );
        assert!(!widgets.picker_components.detail_popover.is_visible());

        // 5. When populate_model_picker is called with models, detail_popover MUST STILL BE HIDDEN (hover-only)
        let sample_models = vec![
            opendictate::services::ai::ModelInfo {
                id: "gemini-1.5-flash".to_string(),
                name: "Gemini 1.5 Flash".to_string(),
                is_free: true,
                provider: "Google Gemini".to_string(),
                inputs: vec!["text".to_string(), "image".to_string(), "audio".to_string()],
                reasoning: false,
                context_limit: 1_000_000,
            },
            opendictate::services::ai::ModelInfo {
                id: "gemini-1.5-pro".to_string(),
                name: "Gemini 1.5 Pro".to_string(),
                is_free: false,
                provider: "Google Gemini".to_string(),
                inputs: vec!["text".to_string(), "image".to_string(), "audio".to_string()],
                reasoning: true,
                context_limit: 2_000_000,
            },
        ];

        opendictate::ui::main_window::settings_view::populate_model_picker(
            &widgets.picker_components,
            &widgets.model_row,
            &sample_models,
            "Google Gemini",
            &sel_id,
        );

        // Hover detail popover MUST remain hidden even after population until hovered
        assert!(!widgets.picker_components.detail_popover.is_visible());

        // Right-side status label is set to selected model display label
        assert!(
            widgets
                .picker_components
                .status_label
                .text()
                .as_str()
                .contains("Gemini 1.5 Flash")
        );

        // Hover card update works and formats fields accurately
        opendictate::ui::main_window::settings_view::update_hover_card(
            &widgets.picker_components,
            &sample_models[0],
        );
        assert_eq!(
            widgets.picker_components.val_model.text().as_str(),
            "Gemini 1.5 Flash (Free)"
        );
        assert_eq!(
            widgets.picker_components.val_prov.text().as_str(),
            "Google Gemini"
        );
        assert_eq!(
            widgets.picker_components.val_inputs.text().as_str(),
            "text, image, audio"
        );
        assert_eq!(
            widgets.picker_components.val_reason.text().as_str(),
            "No reasoning"
        );
        assert_eq!(
            widgets.picker_components.val_context.text().as_str(),
            "1,000,000"
        );

        // 6. Verify single background: card_box must NOT have "card" class
        assert!(!widgets.picker_components.card_box.has_css_class("card"));
        assert!(
            widgets
                .picker_components
                .card_box
                .has_css_class("model-detail-card")
        );

        // 7. Verify no white checkmark icon exists on any model row
        if let Some(first_row) = widgets
            .picker_components
            .list_box
            .first_child()
            .and_then(|c| c.downcast::<gtk4::ListBoxRow>().ok())
            && let Some(h_box) = first_row
                .child()
                .and_then(|c| c.downcast::<gtk4::Box>().ok())
        {
            let mut child = h_box.first_child();
            while let Some(c) = child {
                assert!(
                    !c.is::<gtk4::Image>(),
                    "Model row should not contain checkmark or image icon"
                );
                child = c.next_sibling();
            }
        }
    }
}

#[test]
fn test_settings_dual_engine_widgets_and_mutual_exclusion() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        let config = Config::default();
        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // Initial state: Cloud ON, Local OFF
        assert!(widgets.cloud_switch.is_active());
        assert!(!widgets.local_switch.is_active());

        // Verify newly exposed widgets exist
        assert!(widgets.download_button.is::<gtk4::Button>());
        assert!(widgets.download_progress_bar.is::<gtk4::ProgressBar>());
        assert!(widgets.local_model_row.is::<libadwaita::ComboRow>());
        assert!(widgets.model_status_row.is::<libadwaita::ActionRow>());
        assert!(widgets.download_progress_row.is::<libadwaita::ActionRow>());
        assert!(widgets.custom_file_row.is::<libadwaita::ActionRow>());
        assert!(widgets.threads_row.is::<libadwaita::ComboRow>());

        // Switch Local ON -> Cloud must turn OFF
        widgets.local_switch.set_active(true);
        assert!(widgets.local_switch.is_active());
        assert!(!widgets.cloud_switch.is_active());

        let extracted = opendictate::ui::main_window::settings_view::extract_config_from_widgets(
            &widgets, &config,
        );
        assert_eq!(extracted.ai_mode, "local");

        // Switch Cloud ON -> Local must turn OFF
        widgets.cloud_switch.set_active(true);
        assert!(widgets.cloud_switch.is_active());
        assert!(!widgets.local_switch.is_active());

        let extracted_cloud =
            opendictate::ui::main_window::settings_view::extract_config_from_widgets(
                &widgets, &config,
            );
        assert_eq!(extracted_cloud.ai_mode, "cloud");

        // Initialize with local ai_mode
        let local_config = Config {
            ai_mode: "local".to_string(),
            ..Default::default()
        };
        let (_page2, widgets2) =
            opendictate::ui::main_window::settings_view::build_settings_view(&local_config);
        assert!(!widgets2.cloud_switch.is_active());
        assert!(widgets2.local_switch.is_active());

        // Sensitivity checks
        assert!(widgets2.local_model_row.is_sensitive());
        assert!(widgets2.model_status_row.is_sensitive());
        assert!(widgets2.threads_row.is_sensitive());
        assert!(!widgets2.provider_row.is_sensitive());
        assert!(!widgets2.api_key_row.is_sensitive());
        assert!(!widgets2.model_row.is_sensitive());
    }
}

#[test]
fn test_settings_local_ai_controls_and_status_ui() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_mode: "local".to_string(),
            local_model_id: "tiny.en".to_string(),
            local_threads: 8,
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // Threads row should be initialized to index 5 (8 threads) in ["0", "1", "2", "4", "6", "8", "Custom"]
        assert_eq!(widgets.threads_row.selected(), 5);

        // Custom file row initially hidden for catalog model
        assert!(!widgets.custom_file_row.is_visible());

        // Select "Custom Model (.bin from disk)" (index 5 in catalog list)
        widgets.select_local_model_index(5);
        assert!(widgets.custom_file_row.is_visible());

        // Set custom path in RefCell
        *widgets.custom_path.borrow_mut() = Some("/tmp/custom-model.bin".to_string());

        // Set threads to index 2 (2 threads)
        widgets.threads_row.set_selected(2);

        let extracted = opendictate::ui::main_window::settings_view::extract_config_from_widgets(
            &widgets, &config,
        );
        assert_eq!(extracted.ai_mode, "local");
        assert_eq!(extracted.local_model_id, "custom");
        assert_eq!(
            extracted.local_custom_path,
            Some("/tmp/custom-model.bin".to_string())
        );
        assert_eq!(extracted.local_threads, 2);

        // Test Auto/System Default threads (index 0) and verify label has no "0"
        let threads_list = widgets
            .threads_row
            .model()
            .unwrap()
            .downcast::<gtk4::StringList>()
            .unwrap();
        assert_eq!(
            threads_list.string(0).as_deref(),
            Some("Auto/System Default"),
            "First thread option must be 'Auto/System Default' without leading 0"
        );
        widgets.threads_row.set_selected(0);
        let extracted_auto =
            opendictate::ui::main_window::settings_view::extract_config_from_widgets(
                &widgets, &config,
            );
        assert_eq!(extracted_auto.local_threads, 0);

        // Verify microphone options strictly contain System Default, Headphones, Handsfree and support auto-detection + manual override
        assert_eq!(
            widgets.audio_devices,
            vec![
                "System Default".to_string(),
                "Headphones".to_string(),
                "Handsfree".to_string(),
            ]
        );
        widgets.apply_detected_audio_profile("Handsfree");
        assert_eq!(widgets.audio_device_row.selected(), 2);
        widgets.apply_detected_audio_profile("Headphones");
        assert_eq!(widgets.audio_device_row.selected(), 1);
        // Manual override by user
        widgets.audio_device_row.set_selected(0);
        assert_eq!(widgets.audio_device_row.selected(), 0);
        // Same hardware state does not overwrite user's manual override
        widgets.apply_detected_audio_profile("Headphones");
        assert_eq!(widgets.audio_device_row.selected(), 0);
        // New hardware plug transition (e.g. Bluetooth connected) auto-switches to Handsfree
        widgets.apply_detected_audio_profile("Handsfree");
        assert_eq!(widgets.audio_device_row.selected(), 2);

        // Test Custom Threads option (index 6)
        widgets.threads_row.set_selected(6);
        assert!(widgets.custom_threads_row.is_visible());
        widgets.custom_threads_spin.set_value(16.0);
        let extracted_custom_threads =
            opendictate::ui::main_window::settings_view::extract_config_from_widgets(
                &widgets, &config,
            );
        assert_eq!(extracted_custom_threads.local_threads, 16);

        // Select a catalog model again (index 2: base.en)
        widgets.select_local_model_index(2);
        assert!(!widgets.custom_file_row.is_visible());

        let extracted_catalog =
            opendictate::ui::main_window::settings_view::extract_config_from_widgets(
                &widgets, &config,
            );
        assert_eq!(extracted_catalog.local_model_id, "base.en");
        assert_eq!(extracted_catalog.local_custom_path, None);

        // Verify local_model_row has broad popover CSS class
        assert!(widgets.local_model_row.has_css_class("local-model-combo"));

        // Verify local model picker detail hover card structure and narrowed broadness
        assert!(
            widgets
                .local_picker_components
                .card_box
                .has_css_class("model-detail-card")
        );
        assert!(
            widgets
                .local_picker_components
                .card_box
                .has_css_class("local-model-detail-card")
        );
        assert_eq!(
            widgets.local_picker_components.card_box.width_request(),
            200
        );

        // Verify Custom Model row has no "Custom" badge label on the right
        let custom_row = widgets
            .local_picker_components
            .list_box
            .row_at_index(5)
            .expect("custom model row");
        let custom_child_box = custom_row.child().unwrap().downcast::<gtk4::Box>().unwrap();
        let mut custom_child_count = 0;
        let mut curr = custom_child_box.first_child();
        while let Some(c) = curr {
            custom_child_count += 1;
            curr = c.next_sibling();
        }
        assert_eq!(
            custom_child_count, 1,
            "Custom model row must not have any badge on the right"
        );

        let catalog = opendictate::services::ai::local_ai::get_local_model_catalog();
        let tiny_en = catalog.iter().find(|m| m.id == "tiny.en").unwrap();
        opendictate::ui::main_window::settings_view::update_local_hover_card(
            &widgets.local_picker_components,
            Some(tiny_en),
        );
        assert_eq!(
            widgets.local_picker_components.val_params.text().as_str(),
            "39M"
        );
        assert!(
            widgets
                .local_picker_components
                .val_langs
                .text()
                .as_str()
                .contains("English")
        );
        assert!(
            widgets
                .local_picker_components
                .val_speed
                .text()
                .as_str()
                .contains("Fast")
        );

        // Verify License and Source rows were removed (only 4 rows remain in card_box: Model, Parameters, Languages, Speed)
        let mut card_row_count = 0;
        let mut card_child = widgets.local_picker_components.card_box.first_child();
        while let Some(c) = card_child {
            card_row_count += 1;
            card_child = c.next_sibling();
        }
        assert_eq!(
            card_row_count, 4,
            "Local model detail card should have exactly 4 rows (License and Source removed)"
        );

        // Verify delete_button is part of SettingsViewWidgets
        assert_eq!(
            widgets.delete_button.icon_name().map(|g| g.to_string()),
            Some("user-trash-symbolic".to_string())
        );

        // Test update_local_model_status_ui helper directly with delete_button
        let test_status_label = gtk4::Label::new(None);
        let test_download_btn = gtk4::Button::with_label("Initial");
        let test_delete_btn = gtk4::Button::from_icon_name("user-trash-symbolic");

        // Non-installed catalog model -> delete button must be hidden
        opendictate::ui::main_window::settings_view::update_local_model_status_ui(
            "nonexistent-model-id",
            None,
            &test_status_label,
            &test_download_btn,
            &test_delete_btn,
        );
        assert_eq!(test_status_label.text().as_str(), "Not installed");
        assert_eq!(test_download_btn.label().unwrap().as_str(), "Download");
        assert!(test_download_btn.is_visible());
        assert!(!test_delete_btn.is_visible());

        // Custom model with empty (0-byte) file -> must be guarded and treated as "Not installed"
        let mut tmp_bin = tempfile::NamedTempFile::new().unwrap();
        opendictate::ui::main_window::settings_view::update_local_model_status_ui(
            "custom",
            Some(tmp_bin.path().to_str().unwrap()),
            &test_status_label,
            &test_download_btn,
            &test_delete_btn,
        );
        assert_eq!(test_status_label.text().as_str(), "Not installed");
        assert!(!test_delete_btn.is_visible());

        // Custom model with non-empty file -> must be "Installed" and delete button visible
        use std::io::Write;
        tmp_bin.write_all(b"GGML-LOCAL-MODEL-DATA").unwrap();
        opendictate::ui::main_window::settings_view::update_local_model_status_ui(
            "custom",
            Some(tmp_bin.path().to_str().unwrap()),
            &test_status_label,
            &test_download_btn,
            &test_delete_btn,
        );
        assert!(test_status_label.text().as_str().contains("Installed"));
        assert!(!test_download_btn.is_visible());
        assert!(test_delete_btn.is_visible());

        // Test delete_button click handler on widgets with a custom model file
        let mut delete_target = tempfile::NamedTempFile::new().unwrap();
        delete_target.write_all(b"GGML-MODEL-FOR-DELETION").unwrap();
        let target_path_str = delete_target.path().to_str().unwrap().to_string();
        *widgets.custom_path.borrow_mut() = Some(target_path_str.clone());
        widgets.select_local_model_index(5); // custom
        assert!(widgets.delete_button.is_visible());
        assert!(std::path::Path::new(&target_path_str).exists());

        // Click delete
        widgets.delete_button.emit_clicked();
        assert!(!std::path::Path::new(&target_path_str).exists());
        assert!(!widgets.delete_button.is_visible());
        assert!(widgets.custom_path.borrow().is_none());

        // Local model hover card should only have 4 rows: Model, Parameters, Languages, Speed (License and Source removed)
        let local_card_box = &widgets.local_picker_components.card_box;
        let mut row_count = 0;
        let mut curr = local_card_box.first_child();
        while let Some(c) = curr {
            row_count += 1;
            curr = c.next_sibling();
        }
        assert_eq!(
            row_count, 4,
            "Local model hover card must have exactly 4 rows (License and Source removed)"
        );
    }
}

#[test]
fn test_download_progress_row_visibility_and_button_sensitivity() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_mode: "local".to_string(),
            local_model_id: "tiny.en".to_string(),
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // Before clicking download: progress row is hidden, download button and model selector are sensitive
        assert!(!widgets.download_progress_row.is_visible());
        assert!(widgets.download_button.is_sensitive());
        assert!(widgets.local_model_row.is_sensitive());

        // Emit click on download button
        widgets.download_button.emit_clicked();

        // Immediately upon starting download: progress row unveiled, download button and model row disabled
        assert!(widgets.download_progress_row.is_visible());
        assert!(!widgets.download_button.is_sensitive());
        assert!(!widgets.local_model_row.is_sensitive());
    }
}

#[test]
fn test_main_menu_button_and_dialogs() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config_header = Config::default();
        let storage_header = StorageService::in_memory().unwrap();
        let (header, header_widgets) =
            opendictate::ui::main_window::header::build_header_bar(&config_header, &storage_header);
        assert!(header.is::<libadwaita::HeaderBar>());

        // Header bar should expose the Main Menu button
        let menu_btn = &header_widgets.main_menu_button;
        assert_eq!(menu_btn.icon_name().as_deref(), Some("open-menu-symbolic"));
        assert_eq!(menu_btn.tooltip_text().as_deref(), Some("Main Menu"));
        assert!(menu_btn.popover().is_some());

        // Verify diagnostics generator
        let config = Config {
            ai_mode: "local".to_string(),
            local_model_id: "tiny.en".to_string(),
            local_threads: 4,
            ..Default::default()
        };
        let diag =
            opendictate::ui::main_window::header::generate_troubleshooting_diagnostics(&config);
        assert!(diag.contains("OpenDictate 2.0.0"));
        assert!(diag.contains("Local AI"));
        assert!(diag.contains("tiny.en"));
        assert!(diag.contains("Inference Threads: 4"));

        // Verify About dialog construction
        let about = opendictate::ui::main_window::header::build_about_dialog(&config);
        assert_eq!(about.application_name(), "OpenDictate");
        assert_eq!(about.version(), "2.0.0");
        assert_eq!(about.developer_name(), "OpenDictate Team");

        // Verify troubleshooting, credits, and legal dialog launchers construct without panic
        let parent = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        opendictate::ui::main_window::header::show_troubleshooting_dialog(&parent, &config);
        opendictate::ui::main_window::header::show_credits_dialog(&parent);
        opendictate::ui::main_window::header::show_legal_dialog(&parent);

        // Verify actions registered on window activate without panic
        let tmp = NamedTempFile::new().unwrap();
        let storage = StorageService::new(tmp.path()).unwrap();
        let window = opendictate::ui::main_window::build_main_window(&config, &storage);
        assert!(WidgetExt::activate_action(&window, "app.troubleshooting", None).is_ok());
        assert!(WidgetExt::activate_action(&window, "app.credits", None).is_ok());
        assert!(WidgetExt::activate_action(&window, "app.legal", None).is_ok());
        assert!(WidgetExt::activate_action(&window, "app.about", None).is_ok());
        assert!(WidgetExt::activate_action(&window, "app.report_issue", None).is_ok());
    }
}

#[test]
fn test_open_external_url_helper_does_not_panic() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        opendictate::ui::main_window::header::open_external_url(
            "https://github.com/hkumarsaikia/OpenDictate/issues",
            None,
        );
    }
}

#[test]
fn test_cloud_ai_usage_limit_row_and_popover() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_mode: "cloud".to_string(),
            ai_provider: "groq".to_string(),
            ai_model: "whisper-large-v3-turbo".to_string(),
            ai_api_key: "".to_string(),
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // 1. Without an API key, Usage Limit row must be INACTIVE and button hidden
        assert_eq!(widgets.usage_limit_row.title().as_str(), "Usage Limit");
        assert!(
            !widgets.usage_limit_row.is_activatable(),
            "Usage Limit row must be inactive without an API key"
        );
        assert!(
            !widgets.usage_limit_button.is_visible(),
            "Usage Limit button must be hidden without an API key"
        );
        assert!(
            !widgets.usage_limit_button.is_sensitive(),
            "Usage Limit button must be insensitive without an API key"
        );

        // 2. Now configure valid API key and select model: Usage Limit row becomes activatable and button visible
        widgets.api_key_row.set_text("gsk_valid_test_key_123");
        *widgets.selected_model_id.borrow_mut() = "whisper-large-v3-turbo".to_string();
        opendictate::ui::main_window::settings_view::update_usage_limit_state(
            &widgets.usage_limit_row,
            &widgets.usage_limit_button,
            &widgets.usage_limit_popover,
            "gsk_valid_test_key_123",
            "whisper-large-v3-turbo",
        );
        assert!(widgets.usage_limit_row.is_activatable());
        assert!(widgets.usage_limit_button.is_visible());
        assert!(widgets.usage_limit_button.is_sensitive());
        assert_eq!(
            widgets.usage_limit_button.icon_name().as_deref(),
            Some("pan-down-symbolic")
        );

        // 3. Verify initial usage limit fields for Groq whisper-large-v3-turbo and that inline grey summary is removed
        let ul = &widgets.usage_limit_components;
        opendictate::ui::main_window::settings_view::apply_usage_limit_info(
            ul,
            &opendictate::services::ai::default_model_usage_limits(
                "groq",
                "whisper-large-v3-turbo",
                true,
            ),
        );
        assert_eq!(ul.val_provider.text().as_str(), "Groq");
        assert_eq!(ul.val_model.text().as_str(), "whisper-large-v3-turbo");
        assert!(ul.val_rpm.text().as_str().contains("20 RPM"));
        assert!(ul.val_rpd.text().as_str().contains("2,000 RPD"));
        assert_eq!(ul.summary_label.text().as_str(), "");
        assert!(!ul.summary_label.is_visible());

        // Verify compact Usage Limit popover card has width_request 210 and exactly 3 metric rows (plus header + separator = 5 children)
        let card_box = ul.popover.child().unwrap().downcast::<gtk4::Box>().unwrap();
        assert_eq!(card_box.width_request(), 210);
        let mut child_count = 0;
        let mut curr = card_box.first_child();
        while let Some(c) = curr {
            child_count += 1;
            curr = c.next_sibling();
        }
        assert_eq!(
            child_count, 5,
            "Compact Usage Limit card must have header + separator + 3 metric rows"
        );

        // 4. Retranslation updates usage_limit_row title
        widgets.retranslate("es");
        assert_eq!(widgets.usage_limit_row.title().as_str(), "Límite de uso");
        widgets.retranslate("en");
        assert_eq!(widgets.usage_limit_row.title().as_str(), "Usage Limit");
    }
}

#[test]
fn test_cloud_model_free_badge_no_oval_and_single_free_word() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_mode: "cloud".to_string(),
            ai_provider: "openrouter".to_string(),
            ai_api_key: "sk-or-test-key".to_string(),
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        let models = vec![
            opendictate::services::ai::ModelInfo::with_details(
                "apodex/apodex-1.1-mini:free",
                "Apodex: Apodex 1.1 Mini (free)",
                "OpenRouter",
                vec!["text".to_string()],
                false,
                128_000,
                true,
            ),
            opendictate::services::ai::ModelInfo::with_details(
                "openai/gpt-4o",
                "GPT-4o",
                "OpenRouter",
                vec!["text".to_string()],
                false,
                128_000,
                false,
            ),
        ];

        let sel_id = std::rc::Rc::new(std::cell::RefCell::new(
            "apodex/apodex-1.1-mini:free".to_string(),
        ));
        opendictate::ui::main_window::settings_view::populate_model_picker(
            &widgets.picker_components,
            &widgets.model_row,
            &models,
            "OpenRouter",
            &sel_id,
        );

        // Verify status label contains only one "Free"
        let status = widgets.picker_components.status_label.text();
        assert_eq!(status.as_str(), "Apodex: Apodex 1.1 Mini [Free]");
        assert_eq!(
            status.to_lowercase().matches("free").count(),
            1,
            "Status label must not have duplicate 'Free'"
        );

        // Inspect the row inside the listbox
        let first_row = widgets
            .picker_components
            .list_box
            .row_at_index(0)
            .expect("First row must exist");
        let h_box = first_row.child().unwrap().downcast::<gtk4::Box>().unwrap();
        let name_lbl = h_box
            .first_child()
            .unwrap()
            .downcast::<gtk4::Label>()
            .unwrap();
        assert_eq!(name_lbl.text().as_str(), "Apodex: Apodex 1.1 Mini");

        let free_badge = h_box
            .last_child()
            .unwrap()
            .downcast::<gtk4::Label>()
            .unwrap();
        assert_eq!(free_badge.text().as_str(), "Free");
        assert!(
            free_badge.has_css_class("dim-label"),
            "Free badge must have dim-label styling"
        );
        assert!(
            !free_badge.has_css_class("free-badge"),
            "Free badge must NOT have oval free-badge class"
        );
    }
}

#[test]
fn test_cloud_model_selection_warning_tooltip() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ai_mode: "cloud".to_string(),
            ai_provider: "openrouter".to_string(),
            ai_api_key: "sk-or-free-tier-key".to_string(),
            ..Default::default()
        };

        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        // Selecting a non-audio free model should trigger AUDIO_UNSUPPORTED_WARNING
        let non_audio_free = opendictate::services::ai::ModelInfo::with_details(
            "apodex/apodex-1.1-mini:free",
            "Apodex: Apodex 1.1 Mini",
            "OpenRouter",
            vec!["text".to_string()],
            false,
            128_000,
            true,
        );
        let warn_audio =
            opendictate::services::ai::evaluate_model_selection_warning(&non_audio_free, false);
        assert_eq!(
            warn_audio.as_deref(),
            Some(opendictate::services::ai::AUDIO_UNSUPPORTED_WARNING)
        );

        // Selecting a paid model on unpaid plan should trigger PAID_MODEL_UNPAID_PLAN_WARNING
        let paid_model = opendictate::services::ai::ModelInfo::with_details(
            "openai/gpt-4o",
            "GPT-4o",
            "OpenRouter",
            vec!["text".to_string()],
            false,
            128_000,
            false,
        );
        let warn_paid =
            opendictate::services::ai::evaluate_model_selection_warning(&paid_model, false);
        assert_eq!(
            warn_paid.as_deref(),
            Some(opendictate::services::ai::PAID_MODEL_UNPAID_PLAN_WARNING)
        );

        // Verify warning tooltip popover exists on widgets
        assert!(widgets.warning_tooltip_popover.is::<gtk4::Popover>());
        widgets.show_warning_tooltip(opendictate::services::ai::AUDIO_UNSUPPORTED_WARNING);
        assert_eq!(
            widgets.warning_tooltip_label.text().as_str(),
            opendictate::services::ai::AUDIO_UNSUPPORTED_WARNING
        );

        // Verify selecting a model row in the picker automatically evaluates and triggers the warning tooltip and callback
        let received_warning = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
        let recv_clone = received_warning.clone();
        widgets.set_on_model_warning_change(move |w| {
            *recv_clone.borrow_mut() = w;
        });

        let test_models = vec![non_audio_free.clone(), paid_model.clone()];
        *widgets.current_models.borrow_mut() = test_models.clone();
        opendictate::ui::main_window::settings_view::populate_model_picker(
            &widgets.picker_components,
            &widgets.model_row,
            &test_models,
            "OpenRouter",
            &widgets.selected_model_id,
        );

        // Select row 0 (non-audio free model)
        let row0 = widgets.picker_components.list_box.row_at_index(0).unwrap();
        widgets
            .picker_components
            .list_box
            .emit_by_name::<()>("row-activated", &[&row0]);
        assert_eq!(
            widgets.warning_tooltip_label.text().as_str(),
            opendictate::services::ai::AUDIO_UNSUPPORTED_WARNING
        );
        assert_eq!(
            received_warning.borrow().as_deref(),
            Some(opendictate::services::ai::AUDIO_UNSUPPORTED_WARNING)
        );

        // Select row 1 (paid model on unpaid plan)
        let row1 = widgets.picker_components.list_box.row_at_index(1).unwrap();
        widgets
            .picker_components
            .list_box
            .emit_by_name::<()>("row-activated", &[&row1]);
        assert_eq!(
            widgets.warning_tooltip_label.text().as_str(),
            opendictate::services::ai::PAID_MODEL_UNPAID_PLAN_WARNING
        );
        assert_eq!(
            received_warning.borrow().as_deref(),
            Some(opendictate::services::ai::PAID_MODEL_UNPAID_PLAN_WARNING)
        );
    }
}

#[test]
fn test_dual_theme_svg_icons_and_window_icon_names() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dark_svg = manifest_dir.join("src/ui/assets/brand/opendictate-dark.svg");
    let light_svg = manifest_dir.join("src/ui/assets/brand/opendictate-light.svg");
    let base_svg = manifest_dir.join("src/ui/assets/brand/opendictate.svg");
    let appid_svg = manifest_dir.join("src/ui/assets/brand/io.github.opendictate.OpenDictate.svg");

    for path in [&dark_svg, &light_svg, &base_svg, &appid_svg] {
        assert!(
            path.exists(),
            "SVG icon file must exist: {}",
            path.display()
        );
        let content = std::fs::read_to_string(path).unwrap();
        assert!(
            content.contains("<svg"),
            "File must contain valid SVG: {}",
            path.display()
        );
    }

    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        let tmp = NamedTempFile::new().unwrap();
        let storage = StorageService::new(tmp.path()).unwrap();

        let dark_cfg = Config {
            theme: "dark".to_string(),
            ..Default::default()
        };
        let win_dark = opendictate::ui::main_window::build_main_window(&dark_cfg, &storage);
        assert_eq!(win_dark.icon_name().as_deref(), Some("opendictate-dark"));

        let white_cfg = Config {
            theme: "white".to_string(),
            ..Default::default()
        };
        let win_white = opendictate::ui::main_window::build_main_window(&white_cfg, &storage);
        assert_eq!(win_white.icon_name().as_deref(), Some("opendictate-light"));
    }
}

#[test]
fn test_settings_live_auto_sync_callback() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        let config = Config::default();
        let (_page, widgets) =
            opendictate::ui::main_window::settings_view::build_settings_view(&config);

        let synced_count = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let count_clone = synced_count.clone();
        widgets.set_on_config_auto_sync(move || {
            count_clone.set(count_clone.get() + 1);
        });

        // Toggling Local AI switch must immediately fire the auto-sync callback even without clicking Save
        widgets.local_switch.set_active(true);
        assert!(
            synced_count.get() >= 1,
            "Switching to Local AI must immediately trigger live config sync"
        );

        let before_key = synced_count.get();
        widgets.api_key_row.set_text("gsk_live_test_key");
        assert!(
            synced_count.get() > before_key,
            "Updating API key must immediately trigger live config sync"
        );
    }
}

#[test]
fn test_hotkey_gtk_accelerator_conversion() {
    use opendictate::services::hotkey::HotkeyService;

    assert_eq!(
        HotkeyService::to_gtk_accelerator("Ctrl+Alt+D").unwrap(),
        "<Control><Alt>d"
    );
    assert_eq!(
        HotkeyService::to_gtk_accelerator("<Control><Shift>space").unwrap(),
        "<Control><Shift>space"
    );
    assert_eq!(
        HotkeyService::to_gtk_accelerator("Super+F9").unwrap(),
        "<Super>F9"
    );
}

#[test]
fn test_embedded_symbolic_icons_and_purge_legacy_demo_dictations() {
    let storage = StorageService::in_memory().expect("in_memory storage");
    storage
        .seed_demo_dictations_if_empty()
        .expect("seed demo dictations");
    storage
        .insert_dictation(
            "Real user dictation",
            "Real user dictation",
            "Clean",
            "local",
            3.5,
        )
        .expect("insert real dictation");
    assert_eq!(storage.list_dictations().unwrap().len(), 4);

    storage
        .purge_legacy_demo_dictations()
        .expect("purge legacy demo dictations");
    let remaining = storage.list_dictations().unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].processed_text, "Real user dictation");

    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        opendictate::ui::theme::ensure_app_icons_registered();
        if let Some(display) = gtk4::gdk::Display::default() {
            let settings = gtk4::Settings::for_display(&display);
            assert_eq!(
                settings.gtk_icon_theme_name().as_deref(),
                Some("Adwaita"),
                "GTK icon theme must be locked to Adwaita so symbolic icons never fall back to broken host themes"
            );
            let icon_theme = gtk4::IconTheme::for_display(&display);
            for icon in [
                "weather-clear-symbolic",
                "weather-clear-night-symbolic",
                "org.gnome.Settings-symbolic",
                "media-record-symbolic",
                "media-playback-pause-symbolic",
                "edit-delete-symbolic",
                "user-trash-symbolic",
                "view-conceal-symbolic",
                "view-reveal-symbolic",
                "document-edit-symbolic",
                "adw-entry-edit-symbolic",
                "adw-entry-apply-symbolic",
                "process-working-symbolic",
            ] {
                assert!(
                    icon_theme.has_icon(icon),
                    "IconTheme must resolve embedded symbolic icon '{}'",
                    icon
                );
            }
        }
    }
}
