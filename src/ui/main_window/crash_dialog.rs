//! Native GTK4 and Libadwaita Crash Recovery & 1-Click Report Window.
//!
//! Displays the captured crash report in an editable monospace text view on the
//! same screen as the "Send Crash Report", "Copy Report", and "Dismiss" actions.
//! Clicking "Send Crash Report" submits the user-reviewed (and optionally edited)
//! crash report directly to GitHub Issues via the OpenDictate Cloudflare Worker relay.

use crate::services::crash_reporter::{CrashReport, CrashReporter, CrashSubmissionResult};
use gtk4::prelude::*;
use libadwaita::prelude::*;

/// Widget handles for the Crash Recovery & Report Window.
#[derive(Clone)]
pub struct CrashDialogWidgets {
    pub window: libadwaita::Window,
    pub header_bar: libadwaita::HeaderBar,
    pub title_entry: gtk4::Entry,
    pub report_text_view: gtk4::TextView,
    pub status_banner_box: gtk4::Box,
    pub status_icon: gtk4::Image,
    pub status_label: gtk4::Label,
    pub view_issue_button: gtk4::Button,
    pub dismiss_button: gtk4::Button,
    pub copy_button: gtk4::Button,
    pub send_button: gtk4::Button,
}

impl CrashDialogWidgets {
    /// Returns the current (potentially user-edited) issue title from the title entry.
    pub fn current_title(&self) -> String {
        self.title_entry.text().to_string()
    }

    /// Returns the current (potentially user-edited) crash report body from the text view buffer.
    pub fn current_body(&self) -> String {
        let buffer = self.report_text_view.buffer();
        buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string()
    }

    /// Transitions the window UI into the "Sending..." in-progress state.
    pub fn set_sending_state(&self) {
        self.send_button.set_sensitive(false);
        self.send_button.set_label("Sending Crash Report...");
        self.status_banner_box.remove_css_class("crash-status-ok");
        self.status_banner_box.remove_css_class("crash-status-err");
        self.status_banner_box
            .add_css_class("crash-status-progress");
        self.status_icon
            .set_icon_name(Some("content-loading-symbolic"));
        self.status_label
            .set_label("Submitting crash report to OpenDictate GitHub Issues...");
        self.view_issue_button.set_visible(false);
        self.status_banner_box.set_visible(true);
    }

    /// Transitions the window UI into the "Sent Successfully" state with the created GitHub Issue number.
    pub fn set_success_state(&self, result: &CrashSubmissionResult) {
        self.send_button.set_sensitive(false);
        self.send_button.set_label(&format!(
            "✓ Crash Report Sent (Issue #{})",
            result.issue_number
        ));
        self.status_banner_box
            .remove_css_class("crash-status-progress");
        self.status_banner_box.remove_css_class("crash-status-err");
        self.status_banner_box.add_css_class("crash-status-ok");
        self.status_icon
            .set_icon_name(Some("object-select-symbolic"));
        self.status_label.set_label(&format!(
            "Crash report submitted directly to GitHub Issues (#{}). Thank you for helping improve OpenDictate!",
            result.issue_number
        ));
        self.view_issue_button
            .set_tooltip_text(Some(&result.issue_url));
        self.view_issue_button.set_visible(true);
        self.status_banner_box.set_visible(true);
    }

    /// Transitions the window UI into the error state if submission fails (e.g., offline).
    pub fn set_error_state(&self, error_msg: &str) {
        self.send_button.set_sensitive(true);
        self.send_button.set_label("Retry Send Crash Report");
        self.status_banner_box
            .remove_css_class("crash-status-progress");
        self.status_banner_box.remove_css_class("crash-status-ok");
        self.status_banner_box.add_css_class("crash-status-err");
        self.status_icon
            .set_icon_name(Some("dialog-error-symbolic"));
        self.status_label
            .set_label(&format!("Could not send automatically: {}", error_msg));
        self.view_issue_button.set_visible(false);
        self.status_banner_box.set_visible(true);
    }
}

fn ensure_crash_dialog_css() {
    if let Some(display) = gtk4::gdk::Display::default() {
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(
            "
            .crash-warning-banner {
                background-color: alpha(@warning_bg_color, 0.16);
                border: 1px solid alpha(@warning_bg_color, 0.40);
                border-radius: 10px;
                padding: 8px 12px;
            }
            .crash-warning-icon {
                color: @warning_color;
            }
            .crash-editor-frame {
                border-radius: 10px;
                border: 1px solid alpha(currentColor, 0.16);
            }
            .crash-editor-view {
                font-family: monospace;
                font-size: 9pt;
                padding: 8px;
            }
            .crash-status-banner {
                border-radius: 10px;
                padding: 8px 10px;
            }
            .crash-status-progress {
                background-color: alpha(@accent_bg_color, 0.14);
                border: 1px solid alpha(@accent_bg_color, 0.35);
            }
            .crash-status-ok {
                background-color: alpha(@success_bg_color, 0.18);
                border: 1px solid alpha(@success_bg_color, 0.45);
            }
            .crash-status-err {
                background-color: alpha(@error_bg_color, 0.16);
                border: 1px solid alpha(@error_bg_color, 0.42);
            }
            ",
        );
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// Builds the native Libadwaita Crash Recovery & Reporting Window.
pub fn build_crash_report_window(
    report: &CrashReport,
    parent: Option<&impl IsA<gtk4::Window>>,
) -> CrashDialogWidgets {
    ensure_crash_dialog_css();

    let window = libadwaita::Window::builder()
        .title("Unexpected Crash Detected — OpenDictate")
        .default_width(520)
        .default_height(410)
        .modal(true)
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let toolbar_view = libadwaita::ToolbarView::new();
    let header_bar = libadwaita::HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::new(
            "Unexpected Crash Detected",
            "",
        ))
        .build();
    toolbar_view.add_top_bar(&header_bar);

    let root_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    root_box.set_margin_top(10);
    root_box.set_margin_bottom(12);
    root_box.set_margin_start(14);
    root_box.set_margin_end(14);

    // 1. Top explanation banner
    let banner_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    banner_box.add_css_class("crash-warning-banner");

    let warn_icon = gtk4::Image::from_icon_name("dialog-warning-symbolic");
    warn_icon.set_pixel_size(20);
    warn_icon.set_valign(gtk4::Align::Center);
    warn_icon.add_css_class("crash-warning-icon");
    banner_box.append(&warn_icon);

    let banner_heading = gtk4::Label::builder()
        .label("OpenDictate Recovered from an Unexpected Crash")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .css_classes(vec!["heading".to_string()])
        .build();
    banner_box.append(&banner_heading);
    root_box.append(&banner_box);

    // 2. Editable Title Row
    let title_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    let title_label = gtk4::Label::builder()
        .label("Report Summary Title (Editable)")
        .halign(gtk4::Align::Start)
        .css_classes(vec!["caption-heading".to_string()])
        .build();
    let title_entry = gtk4::Entry::builder()
        .text(&report.title)
        .hexpand(true)
        .build();
    title_box.append(&title_label);
    title_box.append(&title_entry);
    root_box.append(&title_box);

    // 3. Editable Crash Report TextView on the same screen
    let editor_header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let editor_label = gtk4::Label::builder()
        .label("Crash Report Details & Stack Backtrace (Editable)")
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .css_classes(vec!["caption-heading".to_string()])
        .build();
    editor_header_box.append(&editor_label);
    root_box.append(&editor_header_box);

    let scrolled = gtk4::ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .min_content_height(150)
        .css_classes(vec!["crash-editor-frame".to_string()])
        .build();

    let report_text_view = gtk4::TextView::builder()
        .editable(true)
        .cursor_visible(true)
        .monospace(true)
        .wrap_mode(gtk4::WrapMode::WordChar)
        .top_margin(8)
        .bottom_margin(8)
        .left_margin(10)
        .right_margin(10)
        .css_classes(vec!["crash-editor-view".to_string()])
        .build();
    report_text_view.set_monospace(true);
    report_text_view.buffer().set_text(&report.body);
    scrolled.set_child(Some(&report_text_view));
    root_box.append(&scrolled);

    // 4. Live Status Feedback Banner (hidden until Send is clicked)
    let status_banner_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    status_banner_box.add_css_class("crash-status-banner");
    status_banner_box.set_visible(false);

    let status_icon = gtk4::Image::from_icon_name("object-select-symbolic");
    status_icon.set_pixel_size(18);
    status_banner_box.append(&status_icon);

    let status_label = gtk4::Label::builder()
        .label("")
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .wrap(true)
        .xalign(0.0)
        .build();
    status_banner_box.append(&status_label);

    let view_issue_button = gtk4::Button::builder()
        .label("View on GitHub")
        .valign(gtk4::Align::Center)
        .visible(false)
        .css_classes(vec!["flat".to_string()])
        .build();
    status_banner_box.append(&view_issue_button);
    root_box.append(&status_banner_box);

    // 5. Bottom Action Bar (Dismiss, Copy Report, Send Crash Report)
    let action_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    action_bar.set_margin_top(4);

    let dismiss_button = gtk4::Button::builder()
        .label("Dismiss")
        .tooltip_text("Dismiss and archive this crash report")
        .build();

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let copy_button = gtk4::Button::builder()
        .label("Copy Report")
        .tooltip_text("Copy the edited crash report to clipboard")
        .build();

    let send_button = gtk4::Button::builder()
        .label("Send Crash Report")
        .tooltip_text("Send this crash report directly to OpenDictate GitHub Issues")
        .css_classes(vec!["suggested-action".to_string(), "pill".to_string()])
        .build();

    action_bar.append(&dismiss_button);
    action_bar.append(&spacer);
    action_bar.append(&copy_button);
    action_bar.append(&send_button);
    root_box.append(&action_bar);

    toolbar_view.set_content(Some(&root_box));
    window.set_content(Some(&toolbar_view));
    gtk4::prelude::GtkWindowExt::set_focus(&window, Some(&send_button));

    let widgets = CrashDialogWidgets {
        window: window.clone(),
        header_bar,
        title_entry,
        report_text_view,
        status_banner_box,
        status_icon,
        status_label,
        view_issue_button: view_issue_button.clone(),
        dismiss_button: dismiss_button.clone(),
        copy_button: copy_button.clone(),
        send_button: send_button.clone(),
    };

    // Wire "Dismiss" button -> archive crash and close window
    let win_dismiss = window.clone();
    dismiss_button.connect_clicked(move |_| {
        let _ = CrashReporter::archive_pending_crash();
        win_dismiss.close();
    });

    // Wire "Copy Report" button -> copy live edited text from TextView
    let widgets_copy = widgets.clone();
    let copy_btn_clone = copy_button.clone();
    copy_button.connect_clicked(move |_| {
        let full_text = format!(
            "# {}\n\n{}",
            widgets_copy.current_title(),
            widgets_copy.current_body()
        );
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&full_text);
            copy_btn_clone.set_label("Copied!");
        }
    });

    // Wire "View on GitHub" button
    view_issue_button.connect_clicked(move |btn| {
        if let Some(url) = btn.tooltip_text() {
            let _ = std::process::Command::new("xdg-open")
                .arg(url.as_str())
                .spawn();
        }
    });

    // Wire "Send Crash Report" button -> 1-click async submission to Cloudflare Worker Relay
    let widgets_send = widgets.clone();
    send_button.connect_clicked(move |_| {
        let edited_title = widgets_send.current_title();
        let edited_body = widgets_send.current_body();
        widgets_send.set_sending_state();

        let (tx, rx) = std::sync::mpsc::channel::<Result<CrashSubmissionResult, String>>();
        crate::services::dictation_worker::tokio_handle().spawn(async move {
            let res = CrashReporter::send_crash_report(&edited_title, &edited_body).await;
            let _ = tx.send(res);
        });

        let widgets_poll = widgets_send.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(80), move || {
            match rx.try_recv() {
                Ok(Ok(submission)) => {
                    let _ = CrashReporter::archive_pending_crash();
                    widgets_poll.set_success_state(&submission);
                    gtk4::glib::ControlFlow::Break
                }
                Ok(Err(err_msg)) => {
                    widgets_poll.set_error_state(&err_msg);
                    gtk4::glib::ControlFlow::Break
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    widgets_poll
                        .set_error_state("Background submission worker disconnected unexpectedly.");
                    gtk4::glib::ControlFlow::Break
                }
            }
        });
    });

    widgets
}
