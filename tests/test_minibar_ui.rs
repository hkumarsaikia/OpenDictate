//! Integration tests for OpenDictate MiniBar floating island and 60 FPS visualizer.

use gtk4::prelude::*;
use opendictate::ui::mini_bar::drawer::{calculate_word_count, format_word_count};
use opendictate::ui::mini_bar::visualizer::VisualizerState;
use opendictate::ui::mini_bar::{MiniBarModel, MiniBarMsg, MiniBarState};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_visualizer_state_initialization_and_bounds() {
    let state = VisualizerState::new();
    let levels = state.current_levels();
    assert_eq!(levels.len(), 5);
    for level in levels {
        assert!((0.0..=1.0).contains(&level));
        assert!((level - 0.0).abs() < 1e-4);
    }

    let targets = state.target_levels();
    assert_eq!(targets.len(), 5);
    for target in targets {
        assert!((target - 0.0).abs() < 1e-4);
    }
}

#[test]
fn test_visualizer_state_clamping() {
    let mut state = VisualizerState::new();

    // Input with values outside [0.0, 1.0]
    state.update_levels([-0.5, 0.2, 0.8, 1.5, 2.0]);
    let targets = state.target_levels();
    assert_eq!(targets[0], 0.0);
    assert_eq!(targets[1], 0.2);
    assert_eq!(targets[2], 0.8);
    assert_eq!(targets[3], 1.0);
    assert_eq!(targets[4], 1.0);
}

#[test]
fn test_visualizer_state_interpolation() {
    let mut state = VisualizerState::new();
    state.update_levels([1.0, 1.0, 1.0, 1.0, 1.0]);

    // Interpolate halfway with factor 0.5: 0.0 + (1.0 - 0.0) * 0.5 = 0.50
    state.interpolate(0.5);
    for level in state.current_levels() {
        assert!((level - 0.50).abs() < 1e-4);
    }

    // Interpolate with factor 1.0 reaches target exactly
    state.interpolate(1.0);
    for level in state.current_levels() {
        assert!((level - 1.0).abs() < 1e-4);
    }

    // Downward interpolation with clamped target
    state.update_levels([0.0, 0.0, 0.0, 0.0, 0.0]);
    state.interpolate(0.5);
    for level in state.current_levels() {
        assert!((level - 0.50).abs() < 1e-4);
    }

    // Invalid negative factor should clamp or not overshoot
    state.interpolate(-0.2);
    for level in state.current_levels() {
        assert!((0.0..=1.0).contains(&level));
    }
}

#[test]
fn test_visualizer_state_recording_silence_stays_idle() {
    let mut state = VisualizerState::new();
    state.set_recording(true);
    // Silent targets (ambient room noise gated to 0.0)
    state.update_levels([0.0, 0.0, 0.0, 0.0, 0.0]);

    // Interpolation during silence should remain at exact 0.0 resting state with changed = false
    let changed = state.interpolate(0.5);
    assert!(
        !changed,
        "Silence during recording must not trigger animation/draw updates"
    );
    for level in state.current_levels() {
        assert_eq!(
            level, 0.0,
            "Bars must stay at 0.0 (idle round dots) during silence"
        );
    }

    // Now active voice arrives
    state.update_levels([0.2, 0.5, 0.8, 0.5, 0.2]);
    let voice_changed = state.interpolate(0.5);
    assert!(
        voice_changed,
        "Real voice must trigger dynamic soundwave animation"
    );
    let max_lvl = state
        .current_levels()
        .iter()
        .cloned()
        .fold(0.0f32, f32::max);
    assert!(max_lvl > 0.1, "Bars must rise when voice is detected");
}

#[test]
fn test_minibar_state_transitions() {
    let mut model = MiniBarModel::new();
    assert_eq!(model.state(), MiniBarState::Idle);

    // Idle -> Recording
    model.update(MiniBarMsg::StartRecording);
    assert_eq!(model.state(), MiniBarState::Recording);

    // Recording -> Paused
    model.update(MiniBarMsg::PauseRecording);
    assert_eq!(model.state(), MiniBarState::Paused);

    // Paused -> Recording
    model.update(MiniBarMsg::ResumeRecording);
    assert_eq!(model.state(), MiniBarState::Recording);

    // Recording -> Processing
    model.update(MiniBarMsg::StopRecording);
    assert_eq!(model.state(), MiniBarState::Processing);

    // Processing -> Idle
    model.update(MiniBarMsg::CancelRecording);
    assert_eq!(model.state(), MiniBarState::Idle);

    // Direct SetState
    model.update(MiniBarMsg::SetState(MiniBarState::Recording));
    assert_eq!(model.state(), MiniBarState::Recording);

    model.update(MiniBarMsg::SetState(MiniBarState::Idle));
    assert_eq!(model.state(), MiniBarState::Idle);
}

#[test]
fn test_minibar_transcript_and_drawer_state() {
    let mut model = MiniBarModel::new();
    assert_eq!(model.transcript(), "");
    assert!(!model.is_drawer_expanded());

    // Toggle drawer
    model.update(MiniBarMsg::ToggleDrawer);
    assert!(model.is_drawer_expanded());

    model.update(MiniBarMsg::ToggleDrawer);
    assert!(!model.is_drawer_expanded());

    // Explicit set drawer expanded
    model.update(MiniBarMsg::SetDrawerExpanded(true));
    assert!(model.is_drawer_expanded());

    // Update transcript
    let test_text = "This is a live transcribed sentence.";
    model.update(MiniBarMsg::UpdateTranscript(test_text.to_string()));
    assert_eq!(model.transcript(), test_text);

    // Update audio levels
    let levels = [0.15, 0.45, 0.75, 0.55, 0.25];
    model.update(MiniBarMsg::UpdateAudioLevels(levels));
    assert_eq!(model.audio_levels(), levels);

    // Cancel recording resets transcript and audio levels
    model.update(MiniBarMsg::CancelRecording);
    assert_eq!(model.state(), MiniBarState::Idle);
    assert_eq!(model.transcript(), "");
    assert_eq!(model.audio_levels(), [0.0; 5]);
}

#[test]
fn test_preview_drawer_word_count_calculation() {
    assert_eq!(calculate_word_count(""), 0);
    assert_eq!(calculate_word_count("   \n\t  "), 0);
    assert_eq!(calculate_word_count("hello"), 1);
    assert_eq!(calculate_word_count("hello world"), 2);
    assert_eq!(
        calculate_word_count("The quick brown fox jumps over the lazy dog"),
        9
    );
    assert_eq!(
        calculate_word_count("multiple   spaces   between\twords"),
        4
    );

    assert_eq!(format_word_count(0), "0 words");
    assert_eq!(format_word_count(1), "1 word");
    assert_eq!(format_word_count(42), "42 words");
}

#[test]
fn test_visualizer_state_nan_handling() {
    let mut state = VisualizerState::new();

    // Verify non-finite and NaN values are guarded and set to 0.0
    state.update_levels([f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.75, -1.0]);
    let targets = state.target_levels();
    assert_eq!(targets[0], 0.0);
    assert_eq!(targets[1], 0.0);
    assert_eq!(targets[2], 0.0);
    assert_eq!(targets[3], 0.75);
    assert_eq!(targets[4], 0.0);

    for target in targets {
        assert!(!target.is_nan());
        assert!(target.is_finite());
    }

    // Interpolate with NaN factor should safely do nothing and return false
    let changed = state.interpolate(f32::NAN);
    assert!(!changed);
    for level in state.current_levels() {
        assert!(!level.is_nan());
        assert!(level.is_finite());
    }
}

#[test]
fn test_minibar_toggle_record_and_pause() {
    let mut model = MiniBarModel::new();
    assert_eq!(model.state(), MiniBarState::Idle);

    // ToggleRecord from Idle -> Recording (clears transcript)
    model.update(MiniBarMsg::UpdateTranscript("Draft text".into()));
    assert_eq!(model.transcript(), "Draft text");
    model.update(MiniBarMsg::ToggleRecord);
    assert_eq!(model.state(), MiniBarState::Recording);
    assert_eq!(model.transcript(), "");

    // TogglePause from Recording -> Paused
    model.update(MiniBarMsg::TogglePause);
    assert_eq!(model.state(), MiniBarState::Paused);

    // TogglePause from Paused -> Recording
    model.update(MiniBarMsg::TogglePause);
    assert_eq!(model.state(), MiniBarState::Recording);

    // ToggleRecord from Recording -> Processing
    model.update(MiniBarMsg::ToggleRecord);
    assert_eq!(model.state(), MiniBarState::Processing);

    // While Processing, ToggleRecord and TogglePause have no effect
    model.update(MiniBarMsg::ToggleRecord);
    assert_eq!(model.state(), MiniBarState::Processing);
    model.update(MiniBarMsg::TogglePause);
    assert_eq!(model.state(), MiniBarState::Processing);

    // ToggleRecord from Paused -> Processing
    model.update(MiniBarMsg::SetState(MiniBarState::Paused));
    model.update(MiniBarMsg::ToggleRecord);
    assert_eq!(model.state(), MiniBarState::Processing);

    // TogglePause from Idle does nothing
    model.update(MiniBarMsg::SetState(MiniBarState::Idle));
    model.update(MiniBarMsg::TogglePause);
    assert_eq!(model.state(), MiniBarState::Idle);
}

#[test]
fn test_minibar_gtk_widgets_and_states() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        // 1. Visualizer DrawingArea
        let state = Rc::new(RefCell::new(VisualizerState::new()));
        let drawing_area =
            opendictate::ui::mini_bar::visualizer::build_visualizer_drawing_area(state.clone());
        assert!(drawing_area.is::<gtk4::DrawingArea>());
        assert!(drawing_area.content_width() >= 36);
        assert!(drawing_area.content_height() >= 18);

        // 2. Preview Drawer
        let (revealer, drawer_widgets) = opendictate::ui::mini_bar::drawer::build_preview_drawer();
        assert!(revealer.is::<gtk4::Revealer>());
        assert!(drawer_widgets.text_label.is::<gtk4::Label>());
        assert!(drawer_widgets.text_view.is::<gtk4::TextView>());
        assert!(drawer_widgets.text_view.is_editable());
        assert!(drawer_widgets.word_count_label.is::<gtk4::Label>());
        assert!(drawer_widgets.copy_button.is::<gtk4::Button>());

        // Text and word count updates
        drawer_widgets.set_text("Voice dictation in pure Rust");
        assert_eq!(drawer_widgets.text(), "Voice dictation in pure Rust");
        assert_eq!(drawer_widgets.word_count_label.text(), "5 words");

        // User edits text directly in buffer
        drawer_widgets
            .text_buffer
            .set_text("Directly edited transcription note");
        assert_eq!(drawer_widgets.text(), "Directly edited transcription note");
        assert_eq!(drawer_widgets.word_count_label.text(), "4 words");

        // Expansion state
        assert!(!drawer_widgets.is_expanded());
        drawer_widgets.set_expanded(true);
        assert!(drawer_widgets.is_expanded());
        drawer_widgets.set_expanded(false);
        assert!(!drawer_widgets.is_expanded());

        // 3. MiniBar Window and Widgets
        let (window, minibar_widgets) = opendictate::ui::mini_bar::build_minibar_window();
        assert!(window.is::<gtk4::Window>());
        assert!(!window.is_decorated());
        assert!(window.has_css_class("minibar-window"));

        assert!(minibar_widgets.pill_box.is::<gtk4::Box>());
        assert!(minibar_widgets.pill_box.has_css_class("minibar-pill"));
        assert!(minibar_widgets.record_button.is::<gtk4::Button>());
        assert!(minibar_widgets.pause_button.is::<gtk4::Button>());
        assert!(minibar_widgets.cancel_button.is::<gtk4::Button>());
        assert!(minibar_widgets.drawer_button.is::<gtk4::Button>());
        assert!(minibar_widgets.dashboard_button.is::<gtk4::Button>());
        assert!(minibar_widgets.visualizer_area.is::<gtk4::DrawingArea>());
        assert!(minibar_widgets.spinner.is::<gtk4::Spinner>());

        let test_win = gtk4::Window::new();
        let widgets_direct = opendictate::ui::mini_bar::build_minibar_widgets(&test_win);
        assert!(widgets_direct.pill_box.is::<gtk4::Box>());
        assert!(widgets_direct.dashboard_button.is::<gtk4::Button>());

        // 4. Update UI across all states
        let (state_window, state_widgets) = opendictate::ui::mini_bar::build_minibar_window();
        state_window.set_visible(true);
        let mut model = MiniBarModel::new();

        // Idle State
        model.update(MiniBarMsg::SetState(MiniBarState::Idle));
        opendictate::ui::mini_bar::update_minibar_ui(&model, &state_widgets);
        assert!(state_widgets.record_button.is_sensitive());
        assert!(
            !state_widgets
                .record_button
                .has_css_class("recording-active")
        );
        assert_eq!(
            state_widgets.record_button.icon_name().unwrap().as_str(),
            "media-record-symbolic"
        );
        assert!(!state_widgets.pause_button.is_sensitive());
        assert_eq!(
            state_widgets.pause_button.icon_name().unwrap().as_str(),
            "media-playback-pause-symbolic"
        );
        assert!(!state_widgets.cancel_button.is_sensitive());
        assert!(state_widgets.visualizer_area.is_visible());
        assert!(!state_widgets.spinner.is_visible());
        assert!(!state_widgets.spinner.is_spinning());

        // Recording State
        model.update(MiniBarMsg::SetState(MiniBarState::Recording));
        opendictate::ui::mini_bar::update_minibar_ui(&model, &state_widgets);
        assert!(state_widgets.record_button.is_sensitive());
        assert!(
            state_widgets
                .record_button
                .has_css_class("recording-active")
        );
        assert_eq!(
            state_widgets.record_button.icon_name().unwrap().as_str(),
            "media-playback-stop-symbolic"
        );
        assert!(state_widgets.pause_button.is_sensitive());
        assert_eq!(
            state_widgets.pause_button.icon_name().unwrap().as_str(),
            "media-playback-pause-symbolic"
        );
        assert!(state_widgets.cancel_button.is_sensitive());
        assert!(state_widgets.visualizer_area.is_visible());
        assert!(!state_widgets.spinner.is_visible());
        assert!(!state_widgets.spinner.is_spinning());

        // Paused State
        model.update(MiniBarMsg::SetState(MiniBarState::Paused));
        opendictate::ui::mini_bar::update_minibar_ui(&model, &state_widgets);
        assert!(state_widgets.record_button.is_sensitive());
        assert!(
            !state_widgets
                .record_button
                .has_css_class("recording-active")
        );
        assert_eq!(
            state_widgets.record_button.icon_name().unwrap().as_str(),
            "media-playback-stop-symbolic"
        );
        assert!(state_widgets.pause_button.is_sensitive());
        assert_eq!(
            state_widgets.pause_button.icon_name().unwrap().as_str(),
            "media-playback-start-symbolic"
        );
        assert!(state_widgets.cancel_button.is_sensitive());
        assert!(state_widgets.visualizer_area.is_visible());
        assert!(!state_widgets.spinner.is_visible());
        assert!(!state_widgets.spinner.is_spinning());

        // Processing State
        model.update(MiniBarMsg::SetState(MiniBarState::Processing));
        opendictate::ui::mini_bar::update_minibar_ui(&model, &state_widgets);
        assert!(!state_widgets.record_button.is_sensitive());
        assert!(
            !state_widgets
                .record_button
                .has_css_class("recording-active")
        );
        assert!(!state_widgets.pause_button.is_sensitive());
        assert!(!state_widgets.cancel_button.is_sensitive());
        assert!(!state_widgets.visualizer_area.is_visible());
        assert!(state_widgets.spinner.is_visible());
        assert!(state_widgets.spinner.is_spinning());

        // 5. Full control set and badges
        assert!(minibar_widgets.tone_dropdown.is::<gtk4::DropDown>());
        assert!(minibar_widgets.copy_button.is::<gtk4::Button>());
        assert!(minibar_widgets.theme_button.is::<gtk4::Button>());
        assert!(minibar_widgets.drag_grip.is::<gtk4::Image>());
        assert!(!minibar_widgets.drag_grip.is_visible());
        assert_eq!(
            minibar_widgets
                .dashboard_button
                .icon_name()
                .unwrap()
                .as_str(),
            "org.gnome.Settings-symbolic"
        );
        assert_eq!(
            minibar_widgets
                .dashboard_button
                .tooltip_text()
                .unwrap()
                .as_str(),
            "Settings"
        );
        assert!(
            minibar_widgets
                .drawer_widgets
                .status_badge
                .is::<gtk4::Label>()
        );
        assert!(
            minibar_widgets
                .drawer_widgets
                .clear_button
                .is::<gtk4::Button>()
        );

        // Initial preview drawer state displays placeholder
        assert_eq!(minibar_widgets.drawer_widgets.text(), "");
        assert_eq!(
            minibar_widgets.drawer_widgets.text_label.text(),
            "Dictated text will appear here…"
        );

        minibar_widgets.drawer_widgets.set_status("Listening…");
        assert_eq!(
            minibar_widgets.drawer_widgets.status_badge.text(),
            "Listening…"
        );

        minibar_widgets
            .drawer_widgets
            .set_text("Hello world testing");
        assert_eq!(minibar_widgets.drawer_widgets.text(), "Hello world testing");
        assert_eq!(
            minibar_widgets.drawer_widgets.word_count_label.text(),
            "3 words"
        );

        minibar_widgets.drawer_widgets.clear();
        assert_eq!(minibar_widgets.drawer_widgets.text(), "");
        assert_eq!(
            minibar_widgets.drawer_widgets.text_label.text(),
            "Dictated text will appear here…"
        );
        assert_eq!(
            minibar_widgets.drawer_widgets.word_count_label.text(),
            "0 words"
        );

        // Verify minibar window maintains minibar-window class and cleanly toggles themes
        assert!(minibar_widgets.window.has_css_class("minibar-window"));
        minibar_widgets.window.add_css_class("white-theme");
        assert!(minibar_widgets.window.has_css_class("white-theme"));
        assert!(minibar_widgets.window.has_css_class("minibar-window"));
        minibar_widgets.window.remove_css_class("white-theme");
        minibar_widgets.window.add_css_class("dark-theme");
        assert!(minibar_widgets.window.has_css_class("dark-theme"));
    }
}

#[test]
fn test_minibar_recording_guard_with_warning_tooltip() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let (_window, minibar_widgets) = opendictate::ui::mini_bar::build_minibar_window();

        // 1. Verify warning popover exists on MiniBar
        assert!(
            minibar_widgets
                .warning_tooltip_popover
                .is::<gtk4::Popover>()
        );

        // 2. Set warning on minibar
        let warning_msg = opendictate::services::ai::PAID_MODEL_UNPAID_PLAN_WARNING;
        minibar_widgets.set_recording_warning(Some(warning_msg.to_string()));
        assert_eq!(
            minibar_widgets.recording_warning(),
            Some(warning_msg.to_string())
        );

        // 3. Try starting recording while guarded
        let allowed = minibar_widgets.try_start_recording_or_warn();
        assert!(!allowed, "Recording must be blocked when warning is active");
        assert_eq!(
            minibar_widgets.warning_tooltip_label.text().as_str(),
            warning_msg
        );

        // 4. Clear warning: recording is allowed
        minibar_widgets.set_recording_warning(None);
        let allowed_now = minibar_widgets.try_start_recording_or_warn();
        assert!(
            allowed_now,
            "Recording must be allowed when warning is cleared"
        );
    }
}

#[test]
fn test_evaluate_recording_readiness_warning() {
    use opendictate::config::Config;

    // 1. Default config (Cloud AI with empty API key) must return a clear readiness warning
    let default_cfg = Config::default();
    let warn = opendictate::ui::mini_bar::evaluate_recording_readiness_warning(&default_cfg);
    assert!(
        warn.is_some(),
        "Cloud AI with empty API key must return a readiness warning before recording"
    );
    assert!(
        warn.as_ref().unwrap().contains("API key") || warn.as_ref().unwrap().contains("Settings")
    );

    // 2. Cloud AI with valid non-empty API key returns None
    let configured_cloud = Config {
        ai_mode: "cloud".to_string(),
        ai_provider: "groq".to_string(),
        ai_api_key: "gsk_test_valid_key".to_string(),
        ..Default::default()
    };
    assert_eq!(
        opendictate::ui::mini_bar::evaluate_recording_readiness_warning(&configured_cloud),
        None
    );

    // 3. Local AI with uninstalled custom model path returns a warning
    let uninstalled_local = Config {
        ai_mode: "local".to_string(),
        local_model_id: "custom".to_string(),
        local_custom_path: Some("/nonexistent/path/ggml-missing.bin".to_string()),
        ..Default::default()
    };
    let local_warn =
        opendictate::ui::mini_bar::evaluate_recording_readiness_warning(&uninstalled_local);
    assert!(
        local_warn.is_some(),
        "Local AI with missing model file must return a readiness warning before recording"
    );
}

#[test]
fn test_minibar_expand_drawer_and_clipboard_helper() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        let (_window, minibar_widgets) = opendictate::ui::mini_bar::build_minibar_window();

        assert!(!minibar_widgets.drawer_revealer.reveals_child());
        minibar_widgets.expand_drawer();
        assert!(minibar_widgets.drawer_revealer.reveals_child());
        assert_eq!(
            minibar_widgets.drawer_button.icon_name().as_deref(),
            Some("pan-up-symbolic")
        );

        // Verify copy_text_to_clipboard helper executes cleanly without dropping X11 selection prematurely
        opendictate::ui::mini_bar::copy_text_to_clipboard("Test persistent clipboard copy");
    }
}

#[test]
fn test_sanitize_dictation_for_output_preserves_multiline_unicode_and_strips_trailing_newlines() {
    use opendictate::ui::mini_bar::sanitize_dictation_for_output;

    // Strips trailing \r\n and \n so pasting into terminals/chat apps never submits
    assert_eq!(
        sanitize_dictation_for_output("git commit -m 'fix'\r\n\n"),
        "git commit -m 'fix'"
    );

    // Preserves internal newlines while normalizing CRLF -> LF
    assert_eq!(
        sanitize_dictation_for_output("First paragraph.\r\nSecond paragraph.\n"),
        "First paragraph.\nSecond paragraph."
    );

    // Preserves Unicode (Devanagari, CJK, accented characters, emojis)
    let unicode_input = "नमस्ते दुनिया — 안녕하세요 — こんにちは — Café 🚀\n";
    assert_eq!(
        sanitize_dictation_for_output(unicode_input),
        "नमस्ते दुनिया — 안녕하세요 — こんにちは — Café 🚀"
    );
}

#[test]
fn test_minibar_focus_protection_retranslation_and_idle_visualizer_snap() {
    let mut vis = VisualizerState::new();
    vis.update_levels([0.00005; 5]);
    assert!(
        !vis.interpolate(0.5),
        "Sub-threshold idle levels must short-circuit without scheduling redraws"
    );
    assert_eq!(vis.current_levels(), [0.0; 5]);

    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();
        let (_window, widgets) = opendictate::ui::mini_bar::build_minibar_window();

        // Focus protection: clicking MiniBar buttons must not steal focus
        assert!(!widgets.record_button.property::<bool>("focus-on-click"));
        assert!(!widgets.pause_button.property::<bool>("focus-on-click"));
        assert!(!widgets.cancel_button.property::<bool>("focus-on-click"));
        assert!(!widgets.theme_button.property::<bool>("focus-on-click"));
        assert!(!widgets.dashboard_button.property::<bool>("focus-on-click"));
        assert!(!widgets.drawer_button.property::<bool>("focus-on-click"));
        assert!(
            !widgets
                .drawer_widgets
                .copy_button
                .property::<bool>("focus-on-click")
        );
        assert!(
            !widgets
                .drawer_widgets
                .clear_button
                .property::<bool>("focus-on-click")
        );

        // Dynamic retranslation in Spanish and Arabic (RTL)
        widgets.retranslate("es");
        assert_eq!(
            widgets.dashboard_button.tooltip_text().as_deref(),
            Some("Configuración")
        );
        assert_eq!(
            widgets.drawer_widgets.copy_button.tooltip_text().as_deref(),
            Some("Copiar")
        );

        widgets.retranslate("ar");
        assert_eq!(widgets.window.direction(), gtk4::TextDirection::Rtl);

        widgets.retranslate("en");
        assert_eq!(widgets.window.direction(), gtk4::TextDirection::Ltr);
    }
}
