//! Integration tests for dynamic in-place UI retranslation across MainWindow and Settings.

use gtk4::prelude::*;
use libadwaita::prelude::*;
use opendictate::config::Config;
use opendictate::services::i18n::tr;
use opendictate::services::storage::StorageService;
use opendictate::ui::main_window::header::build_header_bar;
use opendictate::ui::main_window::settings_view::build_settings_view;
use opendictate::ui::main_window::{MainWindowModel, MainWindowMsg, build_main_window};
use tempfile::NamedTempFile;

#[test]
fn test_model_set_ui_language_and_config_persistence() {
    let tmp = NamedTempFile::new().unwrap();
    let storage = StorageService::new(tmp.path()).unwrap();

    let config_tmp = NamedTempFile::new().unwrap();
    let config = Config {
        ui_language: "en".to_string(),
        ..Default::default()
    };
    config.save_to(config_tmp.path()).unwrap();

    let mut model =
        MainWindowModel::new_with_config_path(config, storage, config_tmp.path().to_path_buf());

    assert_eq!(model.config().ui_language, "en");

    // Change language via set_ui_language directly
    model.set_ui_language("es");
    assert_eq!(model.config().ui_language, "es");

    // Verify written to disk
    let loaded = Config::load_from(config_tmp.path()).unwrap();
    assert_eq!(loaded.ui_language, "es");

    // Change language via MainWindowMsg::SetUiLanguage
    model.update(MainWindowMsg::SetUiLanguage("de".to_string()));
    assert_eq!(model.config().ui_language, "de");

    let loaded_de = Config::load_from(config_tmp.path()).unwrap();
    assert_eq!(loaded_de.ui_language, "de");
}

#[test]
fn test_header_widgets_retranslation() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, header_widgets) = build_header_bar(&config, &storage);

        // Verify initial English
        assert_eq!(header_widgets.title_widget.title(), tr("settings", "en"));
        assert_eq!(header_widgets.language_label.text(), tr("language", "en"));
        assert_eq!(header_widgets.history_label.text(), tr("history", "en"));
        assert_eq!(
            header_widgets.about_label.text(),
            tr("about_opendictate", "en")
        );
        assert_eq!(header_widgets.language_title.text(), tr("language", "en"));
        assert_eq!(header_widgets.history_title.text(), tr("history", "en"));
        assert_eq!(header_widgets.language_back_label.text(), tr("back", "en"));
        assert_eq!(header_widgets.history_back_label.text(), tr("back", "en"));
        assert_eq!(
            header_widgets.history_empty_label.text(),
            tr("no_history", "en")
        );

        let test_languages = ["es", "de", "zh-CN", "ja"];

        for lang in test_languages {
            header_widgets.retranslate(lang);

            assert_eq!(
                header_widgets.title_widget.title(),
                tr("settings", lang),
                "Failed title for {}",
                lang
            );
            assert_eq!(
                header_widgets.language_label.text(),
                tr("language", lang),
                "Failed language_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.history_label.text(),
                tr("history", lang),
                "Failed history_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.about_label.text(),
                tr("about_opendictate", lang),
                "Failed about_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.language_title.text(),
                tr("language", lang),
                "Failed language_title for {}",
                lang
            );
            assert_eq!(
                header_widgets.history_title.text(),
                tr("history", lang),
                "Failed history_title for {}",
                lang
            );
            assert_eq!(
                header_widgets.language_back_label.text(),
                tr("back", lang),
                "Failed language_back_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.history_back_label.text(),
                tr("back", lang),
                "Failed history_back_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.history_empty_label.text(),
                tr("no_history", lang),
                "Failed history_empty_label for {}",
                lang
            );
            assert_eq!(
                header_widgets.main_menu_button.tooltip_text().as_deref(),
                Some(tr("main_menu", lang)),
                "Failed main_menu tooltip for {}",
                lang
            );
            assert_eq!(*header_widgets.active_language.borrow(), lang);
        }
    }
}

#[test]
fn test_settings_widgets_retranslation() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let (_page, settings_widgets) = build_settings_view(&config);

        // Verify initial English
        assert_eq!(settings_widgets.page.title().as_str(), tr("settings", "en"));
        assert_eq!(
            settings_widgets.appearance_group.title().as_str(),
            tr("appearance", "en")
        );
        assert_eq!(
            settings_widgets.cloud_group.title().as_str(),
            tr("cloud_ai", "en")
        );
        assert_eq!(
            settings_widgets.local_group.title().as_str(),
            tr("local_ai", "en")
        );
        assert_eq!(
            settings_widgets.general_group.title().as_str(),
            tr("general", "en")
        );

        assert_eq!(settings_widgets.theme_dropdown.title(), tr("theme", "en"));
        assert_eq!(
            settings_widgets.provider_row.title(),
            tr("ai_provider", "en")
        );
        assert_eq!(settings_widgets.api_key_row.title(), tr("api_key", "en"));
        assert_eq!(settings_widgets.model_row.title(), tr("ai_model", "en"));
        assert_eq!(
            settings_widgets.local_model_row.title(),
            tr("whisper_model", "en")
        );
        assert_eq!(settings_widgets.threads_row.title(), tr("threads", "en"));
        assert_eq!(
            settings_widgets.audio_device_row.title(),
            tr("microphone", "en")
        );
        assert_eq!(
            settings_widgets.hotkey_row.title(),
            tr("global_shortcut", "en")
        );
        assert_eq!(
            settings_widgets.save_button.label().as_deref(),
            Some(tr("save", "en"))
        );

        let test_languages = ["es", "de", "zh-CN", "ja"];

        for lang in test_languages {
            settings_widgets.retranslate(lang);

            // Group titles
            assert_eq!(
                settings_widgets.appearance_group.title().as_str(),
                tr("appearance", lang),
                "Failed appearance_group title for {}",
                lang
            );
            assert_eq!(
                settings_widgets.cloud_group.title().as_str(),
                tr("cloud_ai", lang),
                "Failed cloud_group title for {}",
                lang
            );
            assert_eq!(
                settings_widgets.local_group.title().as_str(),
                tr("local_ai", lang),
                "Failed local_group title for {}",
                lang
            );
            assert_eq!(
                settings_widgets.general_group.title().as_str(),
                tr("general", lang),
                "Failed general_group title for {}",
                lang
            );

            // Row titles
            assert_eq!(settings_widgets.theme_dropdown.title(), tr("theme", lang));
            assert_eq!(
                settings_widgets.provider_row.title(),
                tr("ai_provider", lang)
            );
            assert_eq!(settings_widgets.api_key_row.title(), tr("api_key", lang));
            assert_eq!(settings_widgets.model_row.title(), tr("ai_model", lang));
            assert_eq!(
                settings_widgets.local_model_row.title(),
                tr("whisper_model", lang)
            );
            assert_eq!(settings_widgets.threads_row.title(), tr("threads", lang));
            assert_eq!(
                settings_widgets.audio_device_row.title(),
                tr("microphone", lang)
            );
            assert_eq!(
                settings_widgets.hotkey_row.title(),
                tr("global_shortcut", lang)
            );

            // Save button
            assert_eq!(
                settings_widgets.save_button.label().as_deref(),
                Some(tr("save", lang)),
                "Failed save_button label for {}",
                lang
            );
        }
    }
}

#[test]
fn test_history_actions_localization_across_languages() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, header_widgets) = build_header_bar(&config, &storage);

        storage
            .insert_dictation(
                "Test raw snippet",
                "Test full transcription text",
                "Clean",
                "local",
                2.0,
            )
            .unwrap();

        header_widgets.refresh_history();

        let list_box = &header_widgets.history_list_box;
        let row = list_box.row_at_index(0).expect("History row must exist");

        // Find copy and delete buttons
        let mut copy_btn_opt: Option<gtk4::Button> = None;
        let mut delete_btn_opt: Option<gtk4::Button> = None;

        if let Some(row_box) = row.child().and_then(|c| c.downcast::<gtk4::Box>().ok()) {
            let mut child = row_box.first_child();
            while let Some(c) = child {
                if let Ok(actions_box) = c.clone().downcast::<gtk4::Box>() {
                    let mut btn_child = actions_box.first_child();
                    while let Some(b) = btn_child {
                        if let Ok(btn) = b.clone().downcast::<gtk4::Button>() {
                            if btn.icon_name().as_deref() == Some("edit-copy-symbolic") {
                                copy_btn_opt = Some(btn);
                            } else if btn.icon_name().as_deref() == Some("user-trash-symbolic") {
                                delete_btn_opt = Some(btn);
                            }
                        }
                        btn_child = b.next_sibling();
                    }
                }
                child = c.next_sibling();
            }
        }

        let copy_btn = copy_btn_opt.expect("Copy button must exist");
        let delete_btn = delete_btn_opt.expect("Delete button must exist");

        assert_eq!(copy_btn.tooltip_text().as_deref(), Some(tr("copy", "en")));
        assert_eq!(
            delete_btn.tooltip_text().as_deref(),
            Some(tr("delete", "en"))
        );

        // Retranslate to Spanish
        header_widgets.retranslate("es");

        let row_es = list_box
            .row_at_index(0)
            .expect("History row must exist after retranslate");
        let mut copy_btn_es: Option<gtk4::Button> = None;
        let mut delete_btn_es: Option<gtk4::Button> = None;

        if let Some(row_box) = row_es.child().and_then(|c| c.downcast::<gtk4::Box>().ok()) {
            let mut child = row_box.first_child();
            while let Some(c) = child {
                if let Ok(actions_box) = c.clone().downcast::<gtk4::Box>() {
                    let mut btn_child = actions_box.first_child();
                    while let Some(b) = btn_child {
                        if let Ok(btn) = b.clone().downcast::<gtk4::Button>() {
                            if btn.icon_name().as_deref() == Some("edit-copy-symbolic") {
                                copy_btn_es = Some(btn);
                            } else if btn.icon_name().as_deref() == Some("user-trash-symbolic") {
                                delete_btn_es = Some(btn);
                            }
                        }
                        btn_child = b.next_sibling();
                    }
                }
                child = c.next_sibling();
            }
        }

        assert_eq!(
            copy_btn_es.unwrap().tooltip_text().as_deref(),
            Some(tr("copy", "es"))
        );
        assert_eq!(
            delete_btn_es.unwrap().tooltip_text().as_deref(),
            Some(tr("delete", "es"))
        );
    }
}

#[test]
fn test_build_main_window_language_activation_wiring() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config_tmp = NamedTempFile::new().unwrap();
        let mut config = Config {
            ui_language: "en".to_string(),
            ..Default::default()
        };
        config.save_to(config_tmp.path()).unwrap();

        let storage_tmp = NamedTempFile::new().unwrap();
        let storage = StorageService::new(storage_tmp.path()).unwrap();

        let window = build_main_window(&config, &storage);
        assert!(window.is::<libadwaita::ApplicationWindow>());
        assert_eq!(window.title().as_deref(), Some(tr("settings", "en")));

        // Test with initial Spanish
        config.ui_language = "es".to_string();
        let window_es = build_main_window(&config, &storage);
        assert_eq!(window_es.title().as_deref(), Some(tr("settings", "es")));
    }
}

#[test]
fn test_build_main_window_save_preserves_ui_language() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config_tmp = NamedTempFile::new().unwrap();
        let config = Config {
            ui_language: "de".to_string(),
            ..Default::default()
        };
        config.save_to(config_tmp.path()).unwrap();

        let storage_tmp = NamedTempFile::new().unwrap();
        let storage = StorageService::new(storage_tmp.path()).unwrap();

        let window = build_main_window(&config, &storage);
        assert_eq!(window.title().as_deref(), Some(tr("settings", "de")));
    }
}
