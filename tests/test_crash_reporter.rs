//! Integration and UI tests for the CrashReporter service and editable Crash Report window.

use gtk4::prelude::*;
use opendictate::config::Config;
use opendictate::services::crash_reporter::{
    CrashReporter, CrashSubmissionResult, DEFAULT_CRASH_RELAY_URL,
};
use opendictate::ui::main_window::crash_dialog::build_crash_report_window;

#[test]
fn test_crash_report_generation_and_privacy_sanitization() {
    let cfg = Config {
        ai_api_key: "gsk_super_secret_key_should_never_leak".to_string(),
        ..Default::default()
    };

    let fake_home = std::env::var("HOME").unwrap_or_else(|_| "/home/testuser".to_string());
    let panic_msg = format!(
        "Audio stream buffer underrun while reading {}/recordings/chunk.pcm",
        fake_home
    );
    let location = "src/audio/capture.rs:142:18";
    let backtrace = format!(
        "   0: opendictate::audio::capture::start_stream\n             at {}/Project/Dictation/src/audio/capture.rs:142:18\n   1: opendictate::services::dictation_worker::run\n             at {}/Project/Dictation/src/services/dictation_worker.rs:88:9",
        fake_home, fake_home
    );

    let report = CrashReporter::build_report(&panic_msg, location, &backtrace, &cfg);

    assert!(report.title.contains("Crash Report: OpenDictate v2.0.0"));
    assert!(report.title.contains("src/audio/capture.rs:142:18"));
    assert!(!report.message.contains(&fake_home));
    assert!(report.message.contains("~/recordings/chunk.pcm"));
    assert!(!report.body.contains(&fake_home));
    assert!(!report.body.contains("gsk_super_secret_key_should_never_leak"));
    assert!(report.body.contains("Configured (redacted)"));
    assert!(report.body.contains("### User Notes (Editable)"));
    assert!(report.body.contains("### Stack Backtrace"));
}

#[test]
fn test_crash_report_disk_persistence_and_archiving() {
    let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
    let crash_path = temp_dir.path().join("last_crash.log");
    let archived_path = temp_dir.path().join("previous_crash.log");

    unsafe {
        std::env::set_var("OPENDICTATE_CRASH_LOG_PATH", &crash_path);
    }

    let cfg = Config::default();
    let report = CrashReporter::build_report(
        "Simulated ALSA device disconnect",
        "src/audio/capture.rs:210:5",
        "0: opendictate::audio::capture::poll_device",
        &cfg,
    );

    CrashReporter::write_pending_crash(&report).expect("Failed to write pending crash");
    assert!(crash_path.exists());

    let loaded = CrashReporter::load_pending_crash().expect("Expected pending crash to load");
    assert_eq!(loaded.location, "src/audio/capture.rs:210:5");
    assert_eq!(loaded.message, "Simulated ALSA device disconnect");

    CrashReporter::archive_pending_crash().expect("Failed to archive crash");
    assert!(!crash_path.exists());
    assert!(archived_path.exists());
    assert!(CrashReporter::load_pending_crash().is_none());

    let summary =
        CrashReporter::load_latest_crash_summary().expect("Expected archived crash summary");
    assert!(summary.contains("Simulated ALSA device disconnect"));

    unsafe {
        std::env::remove_var("OPENDICTATE_CRASH_LOG_PATH");
    }
}

#[test]
fn test_crash_dialog_editable_report_and_states() {
    if gtk4::init().is_err() {
        return;
    }
    let _ = libadwaita::init();

    let cfg = Config::default();
    let report = CrashReporter::build_report(
        "Unexpected PipeWire stream termination (ERR_NODE_SUSPENDED)",
        "src/audio/capture.rs:184:13",
        "   0: opendictate::audio::capture::AudioCapture::start\n             at ~/Project/OpenDictate/src/audio/capture.rs:184:13\n   1: opendictate::services::dictation_worker::DictationWorker::begin_capture\n             at ~/Project/OpenDictate/src/services/dictation_worker.rs:112:21",
        &cfg,
    );

    let widgets = build_crash_report_window(&report, None::<&gtk4::Window>);
    widgets.window.set_visible(true);
    assert!(!widgets.status_banner_box.is_visible());

    // Verify the crash report is displayed and editable on the same screen as "Send Crash Report"
    assert!(widgets.report_text_view.is_editable());
    assert!(widgets.report_text_view.is_monospace());
    assert_eq!(widgets.send_button.label().as_deref(), Some("Send Crash Report"));
    assert_eq!(widgets.copy_button.label().as_deref(), Some("Copy Report"));
    assert_eq!(widgets.dismiss_button.label().as_deref(), Some("Dismiss"));
    assert!(widgets.current_body().contains("ERR_NODE_SUSPENDED"));

    // Simulate user editing the title and body directly in the UI before clicking Send
    widgets
        .title_entry
        .set_text("Crash Report: Bluetooth headset disconnected during dictation");
    let custom_body = format!(
        "User Note: I unplugged my USB headset while recording.\n\n{}",
        widgets.current_body()
    );
    widgets.report_text_view.buffer().set_text(&custom_body);

    assert_eq!(
        widgets.current_title(),
        "Crash Report: Bluetooth headset disconnected during dictation"
    );
    assert!(
        widgets
            .current_body()
            .starts_with("User Note: I unplugged my USB headset while recording.")
    );

    // Verify Sending state
    widgets.set_sending_state();
    assert!(!widgets.send_button.is_sensitive());
    assert_eq!(
        widgets.send_button.label().as_deref(),
        Some("Sending Crash Report...")
    );
    assert!(widgets.status_banner_box.is_visible());

    // Verify Sent confirmation state
    let submission = CrashSubmissionResult {
        issue_number: 1,
        issue_url: "https://github.com/hkumarsaikia/OpenDictate/issues/1".to_string(),
    };
    widgets.set_success_state(&submission);
    assert_eq!(
        widgets.send_button.label().as_deref(),
        Some("✓ Crash Report Sent (Issue #1)")
    );
    assert!(widgets.view_issue_button.is_visible());
}

#[test]
fn test_default_cloudflare_relay_url_configured() {
    assert_eq!(
        DEFAULT_CRASH_RELAY_URL,
        "https://opendictate-crash-relay.hkumarsaikia.workers.dev"
    );
}

#[test]
#[ignore]
fn render_crash_window_screenshots_and_verify_live_relay() {
    gtk4::init().expect("GTK4 init failed");
    let _ = libadwaita::init();

    let cfg = Config::default();
    let report = CrashReporter::build_report(
        "Audio capture stream disconnected unexpectedly (ERR_NODE_SUSPENDED)",
        "src/audio/capture.rs:184:13",
        "   0: opendictate::audio::capture::AudioCapture::start\n             at ~/Project/OpenDictate/src/audio/capture.rs:184:13\n   1: opendictate::services::dictation_worker::DictationWorker::begin_capture\n             at ~/Project/OpenDictate/src/services/dictation_worker.rs:112:21\n   2: relm4::component::r#async::builder::AsyncComponentBuilder::launch\n             at ~/.cargo/registry/src/relm4-0.9.1/src/component/async/builder.rs:204:9",
        &cfg,
    );

    // 1. Live end-to-end verification against Cloudflare Worker -> GitHub Issues
    let rt = tokio::runtime::Runtime::new().expect("Tokio runtime failed");
    let live_submission = rt
        .block_on(CrashReporter::send_crash_report(
            &report.title,
            &report.body,
        ))
        .expect("Live crash report submission to Cloudflare Worker failed");
    println!(
        "LIVE_ISSUE_CREATED: #{} -> {}",
        live_submission.issue_number, live_submission.issue_url
    );
    assert!(live_submission.issue_number >= 1);
    assert!(
        live_submission
            .issue_url
            .contains("github.com/hkumarsaikia/OpenDictate/issues/")
    );

    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/screenshots");
    std::fs::create_dir_all(&out_dir).expect("Failed to create docs/screenshots");

    let pump = |ms: u64| {
        let ctx = gtk4::glib::MainContext::default();
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_millis(ms) {
            while ctx.iteration(false) {}
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    };

    let capture_widget = |window: &libadwaita::Window, filename: &str| {
        window.set_default_size(680, 600);
        window.present();
        pump(350);

        let paintable = gtk4::WidgetPaintable::new(Some(window));
        let width = paintable.intrinsic_width().max(680);
        let height = paintable.intrinsic_height().max(600);
        let snapshot = gtk4::Snapshot::new();
        paintable.snapshot(&snapshot, width as f64, height as f64);
        if let Some(node) = snapshot.to_node() {
            let renderer = gtk4::gsk::CairoRenderer::new();
            if let Some(surface) = window.surface()
                && renderer.realize(Some(&surface)).is_ok()
            {
                let texture = renderer.render_texture(
                    &node,
                    Some(&gtk4::graphene::Rect::new(
                        0.0,
                        0.0,
                        width as f32,
                        height as f32,
                    )),
                );
                let path = out_dir.join(filename);
                texture
                    .save_to_png(&path)
                    .expect("Failed to save PNG screenshot");
                renderer.unrealize();
                println!("SAVED_SCREENSHOT: {}", path.display());
            }
        }
    };

    // Screenshot 1: Dark Mode - Initial Editable Crash Report Window
    libadwaita::StyleManager::default().set_color_scheme(libadwaita::ColorScheme::ForceDark);
    let widgets_dark = build_crash_report_window(&report, None::<&gtk4::Window>);
    capture_widget(&widgets_dark.window, "crash-report-window-dark.png");
    widgets_dark.window.close();
    pump(100);

    // Screenshot 2: Light Mode - Initial Editable Crash Report Window
    libadwaita::StyleManager::default().set_color_scheme(libadwaita::ColorScheme::ForceLight);
    let widgets_light = build_crash_report_window(&report, None::<&gtk4::Window>);
    capture_widget(&widgets_light.window, "crash-report-window-light.png");
    widgets_light.window.close();
    pump(100);

    // Screenshot 3: Dark Mode - User Edited Report + "Sending Crash Report..." State
    libadwaita::StyleManager::default().set_color_scheme(libadwaita::ColorScheme::ForceDark);
    let widgets_sending = build_crash_report_window(&report, None::<&gtk4::Window>);
    widgets_sending
        .title_entry
        .set_text("Crash Report: Bluetooth HFP headset disconnected mid-dictation");
    let edited_body = report.body.replace(
        "Application crashed unexpectedly during runtime.",
        "I turned off my Bluetooth headset while speaking a sentence and the audio stream disconnected.",
    );
    widgets_sending
        .report_text_view
        .buffer()
        .set_text(&edited_body);
    widgets_sending.set_sending_state();
    capture_widget(
        &widgets_sending.window,
        "crash-report-window-edited-sending.png",
    );
    widgets_sending.window.close();
    pump(100);

    // Screenshot 4: Dark Mode - "✓ Crash Report Sent (Issue #N)" Confirmation State
    let widgets_sent = build_crash_report_window(&report, None::<&gtk4::Window>);
    widgets_sent
        .title_entry
        .set_text("Crash Report: Bluetooth HFP headset disconnected mid-dictation");
    widgets_sent
        .report_text_view
        .buffer()
        .set_text(&edited_body);
    widgets_sent.set_success_state(&live_submission);
    capture_widget(&widgets_sent.window, "crash-report-window-sent.png");
    widgets_sent.window.close();
    pump(100);

    // Screenshot 5: Dark Mode - Offline / Error Retry State
    let widgets_err = build_crash_report_window(&report, None::<&gtk4::Window>);
    widgets_err.set_error_state("Network unreachable — check your internet connection and retry.");
    capture_widget(&widgets_err.window, "crash-report-window-error.png");
    widgets_err.window.close();
    pump(100);
}

