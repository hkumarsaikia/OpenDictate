//! MiniBar floating island pill, 60 FPS visualizer, and preview drawer.

pub mod drawer;
pub mod visualizer;

use drawer::{PreviewDrawerWidgets, build_preview_drawer};
use gtk4::prelude::*;
use relm4::ComponentSender;
use relm4::component::{ComponentParts, SimpleComponent};
use std::cell::RefCell;
use std::rc::Rc;
use visualizer::{VisualizerState, build_visualizer_drawing_area};

/// High-level lifecycle state of the MiniBar floating pill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MiniBarState {
    #[default]
    Idle,
    Recording,
    Paused,
    Processing,
}

/// Messages received and processed by MiniBar.
#[derive(Debug, Clone, PartialEq)]
pub enum MiniBarMsg {
    StartRecording,
    PauseRecording,
    ResumeRecording,
    StopRecording,
    CancelRecording,
    ToggleRecord,
    TogglePause,
    UpdateAudioLevels([f32; 5]),
    UpdateTranscript(String),
    ToggleDrawer,
    SetDrawerExpanded(bool),
    SetState(MiniBarState),
}

/// Model holding the state for MiniBar.
#[derive(Debug, Clone)]
pub struct MiniBarModel {
    state: MiniBarState,
    transcript: String,
    drawer_expanded: bool,
    audio_levels: [f32; 5],
}

impl Default for MiniBarModel {
    fn default() -> Self {
        Self::new()
    }
}

impl MiniBarModel {
    /// Creates a new default MiniBar model.
    pub fn new() -> Self {
        Self {
            state: MiniBarState::Idle,
            transcript: String::new(),
            drawer_expanded: false,
            audio_levels: [0.0; 5],
        }
    }

    /// Returns the current state of MiniBar.
    pub fn state(&self) -> MiniBarState {
        self.state
    }

    /// Returns the current transcription text.
    pub fn transcript(&self) -> &str {
        &self.transcript
    }

    /// Returns true if the preview drawer is expanded.
    pub fn is_drawer_expanded(&self) -> bool {
        self.drawer_expanded
    }

    /// Returns current audio levels.
    pub fn audio_levels(&self) -> [f32; 5] {
        self.audio_levels
    }

    /// Updates model state based on message.
    pub fn update(&mut self, msg: MiniBarMsg) {
        match msg {
            MiniBarMsg::StartRecording => {
                self.state = MiniBarState::Recording;
                self.transcript.clear();
                self.audio_levels = [0.0; 5];
            }
            MiniBarMsg::PauseRecording => {
                if self.state == MiniBarState::Recording {
                    self.state = MiniBarState::Paused;
                }
            }
            MiniBarMsg::ResumeRecording => {
                if self.state == MiniBarState::Paused {
                    self.state = MiniBarState::Recording;
                }
            }
            MiniBarMsg::StopRecording => {
                self.state = MiniBarState::Processing;
                self.audio_levels = [0.0; 5];
            }
            MiniBarMsg::CancelRecording => {
                self.state = MiniBarState::Idle;
                self.transcript.clear();
                self.audio_levels = [0.0; 5];
                self.drawer_expanded = false;
            }
            MiniBarMsg::ToggleRecord => match self.state {
                MiniBarState::Idle => {
                    self.state = MiniBarState::Recording;
                    self.transcript.clear();
                    self.audio_levels = [0.0; 5];
                }
                MiniBarState::Recording | MiniBarState::Paused => {
                    self.state = MiniBarState::Processing;
                    self.audio_levels = [0.0; 5];
                }
                MiniBarState::Processing => {}
            },
            MiniBarMsg::TogglePause => match self.state {
                MiniBarState::Recording => {
                    self.state = MiniBarState::Paused;
                }
                MiniBarState::Paused => {
                    self.state = MiniBarState::Recording;
                }
                _ => {}
            },
            MiniBarMsg::UpdateAudioLevels(levels) => {
                self.audio_levels = levels;
            }
            MiniBarMsg::UpdateTranscript(text) => {
                self.transcript = text;
            }
            MiniBarMsg::ToggleDrawer => {
                self.drawer_expanded = !self.drawer_expanded;
            }
            MiniBarMsg::SetDrawerExpanded(expanded) => {
                self.drawer_expanded = expanded;
            }
            MiniBarMsg::SetState(state) => {
                self.state = state;
            }
        }
    }
}

/// Widgets tracked for the MiniBar floating island.
#[derive(Clone, Debug)]
pub struct MiniBarWidgets {
    pub window: gtk4::Window,
    pub pill_box: gtk4::Box,
    pub drag_grip: gtk4::Image,
    pub record_button: gtk4::Button,
    pub pause_button: gtk4::Button,
    pub cancel_button: gtk4::Button,
    pub visualizer_area: gtk4::DrawingArea,
    pub visualizer_state: Rc<RefCell<VisualizerState>>,
    pub spinner: gtk4::Spinner,
    pub tone_dropdown: gtk4::DropDown,
    pub copy_button: gtk4::Button,
    pub theme_button: gtk4::Button,
    pub dashboard_button: gtk4::Button,
    pub drawer_button: gtk4::Button,
    pub drawer_revealer: gtk4::Revealer,
    pub drawer_widgets: PreviewDrawerWidgets,
    pub warning_tooltip_popover: gtk4::Popover,
    pub warning_tooltip_revealer: gtk4::Revealer,
    pub warning_tooltip_label: gtk4::Label,
    pub recording_warning: Rc<RefCell<Option<String>>>,
    pub tooltip_generation: Rc<std::cell::Cell<u64>>,
}

impl MiniBarWidgets {
    /// Sets an active recording prevention warning (e.g. non-audio model or unpaid plan for paid model).
    pub fn set_recording_warning(&self, warning: Option<String>) {
        *self.recording_warning.borrow_mut() = warning;
    }

    /// Returns the currently active recording prevention warning, if any.
    pub fn recording_warning(&self) -> Option<String> {
        self.recording_warning.borrow().clone()
    }

    /// Smoothly displays an auto-dismissing tooltip popover for 2.5 seconds.
    pub fn show_warning_tooltip(&self, message: &str) {
        self.warning_tooltip_label.set_text(message);
        let next_gen = self.tooltip_generation.get() + 1;
        self.tooltip_generation.set(next_gen);

        self.warning_tooltip_revealer.set_reveal_child(false);
        self.warning_tooltip_popover.popup();
        self.warning_tooltip_revealer.set_reveal_child(true);

        let popover_clone = self.warning_tooltip_popover.clone();
        let revealer_clone = self.warning_tooltip_revealer.clone();
        let gen_cell = self.tooltip_generation.clone();

        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(2500), move || {
            if gen_cell.get() == next_gen {
                revealer_clone.set_reveal_child(false);
                gtk4::glib::timeout_add_local_once(
                    std::time::Duration::from_millis(220),
                    move || {
                        if gen_cell.get() == next_gen {
                            popover_clone.popdown();
                        }
                    },
                );
            }
        });
    }

    /// Checks if recording is allowed. If blocked by a warning, shows the smooth tooltip and returns false.
    pub fn try_start_recording_or_warn(&self) -> bool {
        if let Some(ref warning) = *self.recording_warning.borrow() {
            self.show_warning_tooltip(warning);
            false
        } else {
            true
        }
    }

    /// Dynamically scales the MiniBar pill, buttons, icons, visualizer, and dropdown.
    /// `scale_percent` is clamped between 70 and 200 (default 100).
    pub fn apply_scale(&self, scale_percent: u32) {
        let clamped = scale_percent.clamp(70, 200);
        let factor = clamped as f64 / 100.0;

        let btn_size = (30.0 * factor).round() as i32;
        for btn in [
            &self.record_button,
            &self.pause_button,
            &self.cancel_button,
            &self.theme_button,
            &self.dashboard_button,
            &self.drawer_button,
        ] {
            btn.set_size_request(btn_size, btn_size);
        }

        let vis_w = (46.0 * factor).round() as i32;
        let vis_h = (24.0 * factor).round() as i32;
        self.visualizer_area.set_content_width(vis_w);
        self.visualizer_area.set_content_height(vis_h);

        let spin_sz = (20.0 * factor).round() as i32;
        self.spinner.set_size_request(spin_sz, spin_sz);

        let spacing = (5.0 * factor).round() as i32;
        self.pill_box.set_spacing(spacing);

        apply_minibar_scale_css(clamped);

        self.visualizer_area.queue_draw();
        self.window.queue_resize();
    }
}

/// Applies dynamic CSS scaling rules for the MiniBar based on `scale_percent` (70%..=200%).
pub fn apply_minibar_scale_css(scale_percent: u32) {
    if !gtk4::is_initialized() {
        return;
    }
    thread_local! {
        static SCALE_PROVIDER: gtk4::CssProvider = {
            let p = gtk4::CssProvider::new();
            if let Some(display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(
                    &display,
                    &p,
                    gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
                );
            }
            p
        };
    }

    let factor = (scale_percent.clamp(70, 200) as f64) / 100.0;
    let pad_v = (5.0 * factor).round() as i32;
    let pad_h = (10.0 * factor).round() as i32;
    let radius = (20.0 * factor).round() as i32;
    let btn_min = (30.0 * factor).round() as i32;
    let icon_sz = (16.0 * factor).round() as i32;
    let dd_h = (28.0 * factor).round() as i32;
    let dd_pad_v = (2.0 * factor).round() as i32;
    let dd_pad_h = (8.0 * factor).round() as i32;
    let dd_font = (12.0 * factor).round() as i32;
    let dd_rad = (14.0 * factor).round() as i32;

    let css = format!(
        r#"
        .minibar-pill {{
            padding: {pad_v}px {pad_h}px;
            border-radius: {radius}px;
        }}
        .minibar-pill button {{
            min-width: {btn_min}px;
            min-height: {btn_min}px;
            -gtk-icon-size: {icon_sz}px;
        }}
        .minibar-pill dropdown {{
            font-size: {dd_font}px;
            border-radius: {dd_rad}px;
        }}
        .minibar-pill dropdown > button {{
            min-height: {dd_h}px;
            padding: {dd_pad_v}px {dd_pad_h}px;
            font-size: {dd_font}px;
            border-radius: {dd_rad}px;
        }}
        "#
    );

    SCALE_PROVIDER.with(|provider| {
        provider.load_from_string(&css);
    });
}

/// Ensures the MiniBar CSS theme classes are registered in GTK.
pub fn ensure_minibar_css() {
    if !gtk4::is_initialized() {
        return;
    }
    use std::sync::atomic::{AtomicBool, Ordering};
    static LOADED: AtomicBool = AtomicBool::new(false);
    if LOADED.swap(true, Ordering::SeqCst) {
        return;
    }

    let provider = gtk4::CssProvider::new();
    let css = r#"
    window.minibar-window,
    window.minibar-window.white-theme,
    window.minibar-window.dark-theme,
    window.minibar-window.background,
    window.minibar-window.white-theme.background,
    window.minibar-window > .background,
    window.minibar-window.white-theme > .background,
    window.minibar-window.dark-theme > .background {
        background-color: transparent;
        background: transparent;
        box-shadow: none;
        border: none;
    }

    /* ------------------------------------------------------------------------- */
    /* Dark Theme Default Styling                                               */
    /* ------------------------------------------------------------------------- */
    .minibar-pill {
        background-color: #242424;
        color: #ffffff;
        border: 1px solid rgba(255, 255, 255, 0.14);
        border-radius: 20px;
        padding: 5px 10px;
        box-shadow: 0 4px 18px rgba(0, 0, 0, 0.40);
    }
    .minibar-pill button {
        border-radius: 9999px;
        min-width: 30px;
        min-height: 30px;
        padding: 3px;
        color: #ffffff;
    }
    .minibar-pill button:hover {
        background-color: rgba(255, 255, 255, 0.10);
    }
    .minibar-pill button:active {
        background-color: rgba(255, 255, 255, 0.18);
    }
    .minibar-pill button:disabled {
        color: rgba(255, 255, 255, 0.35);
    }
    .minibar-pill dropdown {
        border-radius: 14px;
        font-size: 12px;
    }
    .minibar-pill dropdown > button {
        border-radius: 14px;
        min-height: 28px;
        padding: 2px 8px;
        font-size: 12px;
        color: #ffffff;
        background-color: rgba(255, 255, 255, 0.08);
        border: 1px solid rgba(255, 255, 255, 0.14);
    }
    .minibar-pill dropdown > button:hover {
        background-color: rgba(255, 255, 255, 0.15);
    }
    .minibar-pill separator {
        margin-top: 5px;
        margin-bottom: 5px;
        background-color: rgba(255, 255, 255, 0.15);
    }
    @keyframes pulse-recording {
        0% {
            box-shadow: 0 0 0 0 rgba(224, 27, 36, 0.7);
        }
        50% {
            box-shadow: 0 0 0 5px rgba(224, 27, 36, 0);
        }
        100% {
            box-shadow: 0 0 0 0 rgba(224, 27, 36, 0);
        }
    }
    .minibar-pill button.recording-active {
        background-color: #e01b24;
        color: #ffffff;
        animation: pulse-recording 1.2s infinite;
    }
    .minibar-drawer {
        background-color: #242424;
        color: #ffffff;
        border: 1px solid rgba(255, 255, 255, 0.14);
        border-radius: 16px;
        padding: 10px;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35);
    }
    .minibar-drawer button {
        color: #ffffff;
    }
    .minibar-drawer button:hover {
        background-color: rgba(255, 255, 255, 0.10);
    }
    .minibar-drawer-textbox {
        background-color: #1e1e1e;
        border: 1px solid rgba(255, 255, 255, 0.12);
        border-radius: 10px;
        padding: 6px;
    }
    .minibar-drawer-textbox textview,
    .minibar-drawer-textbox textview text {
        background-color: transparent;
        color: #ffffff;
    }
    .minibar-drawer .placeholder-text {
        color: rgba(255, 255, 255, 0.45);
    }
    window.minibar-window popover {
        background-color: transparent;
        padding: 0;
    }
    window.minibar-window popover > contents {
        background-color: #2a2a2a;
        color: #ffffff;
        border: 1px solid rgba(255, 255, 255, 0.16);
        border-radius: 12px;
        padding: 6px;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.45);
    }
    window.minibar-window popover listview row {
        padding: 6px 12px;
        border-radius: 8px;
        min-height: 28px;
        color: #ffffff;
    }
    window.minibar-window popover listview row:hover {
        background-color: rgba(255, 255, 255, 0.08);
    }
    window.minibar-window popover listview row:selected {
        background-color: @accent_bg_color;
        color: @accent_fg_color;
    }

    popover.warning-tooltip-popover,
    .warning-tooltip-popover {
        background-color: transparent;
        padding: 0;
    }
    popover.warning-tooltip-popover > contents,
    .warning-tooltip-popover > contents {
        background-color: #2b2b2b;
        color: #f6f8fa;
        border: 1px solid rgba(255, 255, 255, 0.16);
        border-radius: 10px;
        padding: 6px 12px;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35);
    }
    .warning-tooltip-card {
        padding: 4px 6px;
    }
    .warning-tooltip-card label {
        font-size: 12px;
        font-weight: 500;
        color: #f6f8fa;
    }

    /* ------------------------------------------------------------------------- */
    /* Primary Record Action Button - Always maintains pure white center icon    */
    /* ------------------------------------------------------------------------- */
    .minibar-pill button.minibar-record-button,
    .minibar-pill button.minibar-record-button:hover,
    .minibar-pill button.minibar-record-button:active,
    .minibar-pill button.suggested-action,
    .minibar-pill button.suggested-action:hover,
    .minibar-pill button.suggested-action:active,
    .minibar-pill button.destructive-action,
    .minibar-pill button.destructive-action:hover,
    .minibar-pill button.destructive-action:active,
    .white-theme .minibar-pill button.minibar-record-button,
    .white-theme .minibar-pill button.minibar-record-button:hover,
    .white-theme .minibar-pill button.minibar-record-button:active,
    .white-theme .minibar-pill button.suggested-action,
    .white-theme .minibar-pill button.suggested-action:hover,
    .white-theme .minibar-pill button.suggested-action:active,
    .white-theme .minibar-pill button.destructive-action,
    .white-theme .minibar-pill button.destructive-action:hover,
    .white-theme .minibar-pill button.destructive-action:active {
        color: #ffffff;
        -gtk-icon-palette: default;
    }
    .minibar-pill button.minibar-record-button image,
    .minibar-pill button.suggested-action image,
    .minibar-pill button.destructive-action image,
    .white-theme .minibar-pill button.minibar-record-button image,
    .white-theme .minibar-pill button.suggested-action image,
    .white-theme .minibar-pill button.destructive-action image {
        color: #ffffff;
    }

    /* ------------------------------------------------------------------------- */
    /* Pale White Theme Styling (.white-theme) - Soft, Pale, Eye-Friendly        */
    /* ------------------------------------------------------------------------- */
    window:not(.minibar-window).white-theme,
    window.white-theme:not(.minibar-window),
    .white-theme.background:not(.minibar-window),
    window:not(.minibar-window).white-theme > .background,
    window.white-theme:not(.minibar-window) > .background,
    window.white-theme preferencespage,
    window.white-theme .preferences-page,
    window.white-theme scrolledwindow,
    window.white-theme viewport {
        background-color: #e8ecf0;
        color: #24292f;
    }
    window:not(.minibar-window).white-theme headerbar,
    window.white-theme:not(.minibar-window) headerbar {
        background-color: #dfe4ea;
        color: #24292f;
        border-bottom: 1px solid #cbd2dc;
    }

    /* MiniBar floating window strictly transparent in white theme */
    window.minibar-window,
    window.minibar-window.white-theme,
    window.minibar-window.dark-theme,
    window.minibar-window.background,
    window.minibar-window.white-theme.background,
    window.minibar-window > .background,
    window.minibar-window.white-theme > .background,
    window.minibar-window.dark-theme > .background {
        background-color: transparent;
        background: transparent;
        box-shadow: none;
        border: none;
    }
    .white-theme .minibar-pill {
        background-color: #e8ecf0;
        color: #24292f;
        border: 1px solid #cbd2dc;
        box-shadow: 0 4px 18px rgba(0, 0, 0, 0.10);
    }
    .white-theme .minibar-pill button.flat {
        color: #24292f;
    }
    .white-theme .minibar-pill button.flat:hover {
        background-color: rgba(0, 0, 0, 0.08);
    }
    .white-theme .minibar-pill button.flat:active {
        background-color: rgba(0, 0, 0, 0.14);
    }
    .white-theme .minibar-pill button.flat:disabled {
        color: rgba(36, 41, 47, 0.35);
    }
    .white-theme .minibar-pill dropdown > button {
        color: #24292f;
        background-color: #dfe4ea;
        border: 1px solid #cbd2dc;
    }
    .white-theme .minibar-pill dropdown > button:hover {
        background-color: #d5dce4;
    }
    /* Clean transparent dropdown interiors preventing white background patches */
    .white-theme dropdown > button > box,
    .white-theme dropdown > button label,
    .white-theme dropdown > button listitem,
    window.white-theme dropdown > button > box,
    window.white-theme dropdown > button label,
    window.white-theme dropdown > button listitem,
    preferencespage dropdown > button > box,
    preferencespage dropdown > button label {
        background-color: transparent;
        background: transparent;
        box-shadow: none;
    }
    .white-theme .minibar-pill separator {
        background-color: #cbd2dc;
    }
    .white-theme .minibar-drawer {
        background-color: #eef2f6;
        color: #24292f;
        border: 1px solid #cbd2dc;
        box-shadow: 0 4px 18px rgba(0, 0, 0, 0.10);
    }
    .white-theme .minibar-drawer button {
        color: #24292f;
    }
    .white-theme .minibar-drawer button:hover {
        background-color: rgba(0, 0, 0, 0.08);
    }
    .white-theme .minibar-drawer-textbox {
        background-color: #dfe4ea;
        border: 1px solid #cbd2dc;
        border-radius: 10px;
        padding: 6px;
    }
    .white-theme .minibar-drawer-textbox textview,
    .white-theme .minibar-drawer-textbox textview text {
        background-color: transparent;
        color: #24292f;
    }
    .white-theme .minibar-drawer .placeholder-text {
        color: rgba(36, 41, 47, 0.45);
    }
    .white-theme .minibar-drawer .dim-label {
        color: rgba(36, 41, 47, 0.65);
    }
    .white-theme popover > contents,
    window.minibar-window.white-theme popover > contents {
        background-color: #eef2f6;
        color: #24292f;
        border: 1px solid #cbd2dc;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.12);
    }
    .white-theme popover listview row,
    window.minibar-window.white-theme popover listview row {
        color: #24292f;
    }
    .white-theme popover listview row:hover,
    window.minibar-window.white-theme popover listview row:hover {
        background-color: rgba(0, 0, 0, 0.07);
    }
    .white-theme popover listview row:selected,
    window.minibar-window.white-theme popover listview row:selected {
        background-color: @accent_bg_color;
        color: @accent_fg_color;
    }

    /* MainWindow Settings View Styles in Pale White Theme */
    .white-theme .boxed-list,
    window.white-theme .boxed-list {
        background-color: #f4f6f9;
        border: 1px solid #cbd2dc;
        border-radius: 12px;
    }
    .white-theme .boxed-list row,
    window.white-theme .boxed-list row {
        background-color: #f4f6f9;
        color: #24292f;
        border-bottom: 1px solid #e2e6eb;
    }
    .white-theme .boxed-list row:hover,
    window.white-theme .boxed-list row:hover {
        background-color: #eaeef3;
    }
    .white-theme .boxed-list row:selected,
    window.white-theme .boxed-list row:selected {
        background-color: @accent_bg_color;
        color: @accent_fg_color;
    }
    .white-theme .boxed-list row:selected label,
    window.white-theme .boxed-list row:selected label {
        color: @accent_fg_color;
    }
    .white-theme searchentry,
    window.white-theme searchentry {
        background-color: #dfe4ea;
        color: #24292f;
        border: 1px solid #cbd2dc;
    }
    .white-theme .dim-label,
    window.white-theme .dim-label {
        color: rgba(36, 41, 47, 0.65);
    }
    .dark-theme .history-transcript-box {
        background-color: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.12);
    }
    .dark-theme .history-transcript-text {
        background-color: transparent;
        color: #ffffff;
    }
    /* OpenCode-style Model Picker & Capabilities Hover Card */
    .opencode-model-popover {
        padding: 4px;
    }
    .model-detail-popover > contents {
        padding: 8px 10px;
    }
    .free-badge {
        background-color: transparent;
        background: transparent;
        border: none;
        padding: 0;
    }
    .model-detail-card {
        background: transparent;
        border: none;
        box-shadow: none;
        padding: 0;
        min-width: 190px;
    }
    .local-model-detail-card {
        min-width: 190px;
    }
    .model-list-box {
        background-color: transparent;
    }
    .model-list-box row {
        border-radius: 8px;
        padding: 5px 8px;
        margin: 1px 2px;
        transition: background-color 100ms ease;
    }
    .model-list-box row:hover {
        background-color: alpha(currentColor, 0.08);
    }
    .model-list-box row:selected {
        background-color: alpha(@accent_bg_color, 0.22);
    }
    /* Local AI model picker main dropdown menu */
    .opencode-model-popover,
    .opencode-model-popover contents,
    .opencode-model-popover listview {
        min-width: 270px;
    }
    /* Compact model detail hover card popover */
    .model-detail-popover,
    .model-detail-popover contents,
    .model-detail-popover > contents {
        min-width: 190px;
        padding: 8px 12px;
    }
    "#;
    provider.load_from_string(css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// Builds the MiniBar widgets and attaches them directly to the provided window.
pub fn build_minibar_widgets(window: &gtk4::Window) -> MiniBarWidgets {
    ensure_minibar_css();

    // Outer column holding pill and expandable drawer
    let main_col = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_col.set_margin_start(6);
    main_col.set_margin_end(6);
    main_col.set_margin_top(4);
    main_col.set_margin_bottom(6);
    main_col.set_halign(gtk4::Align::Center);
    main_col.set_valign(gtk4::Align::Start);

    // Native WindowHandle allows dragging on non-interactive pill space without intercepting button clicks
    let handle = gtk4::WindowHandle::new();
    handle.set_halign(gtk4::Align::Center);
    handle.set_valign(gtk4::Align::Center);

    // Floating pill chassis
    let pill_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    pill_box.add_css_class("minibar-pill");
    pill_box.set_valign(gtk4::Align::Center);
    pill_box.set_halign(gtk4::Align::Center);
    handle.set_child(Some(&pill_box));

    // 1. Drag handle (kept in widgets for API compatibility, but hidden)
    let drag_grip = gtk4::Image::from_icon_name("view-more-horizontal-symbolic");
    drag_grip.set_visible(false);

    // 2. Record / Stop main action button
    let record_button = gtk4::Button::from_icon_name("media-record-symbolic");
    record_button.add_css_class("circular");
    record_button.add_css_class("suggested-action");
    record_button.add_css_class("minibar-record-button");
    record_button.set_tooltip_text(Some("Start Recording"));
    pill_box.append(&record_button);

    // 3. Pause / Resume button
    let pause_button = gtk4::Button::from_icon_name("media-playback-pause-symbolic");
    pause_button.add_css_class("circular");
    pause_button.add_css_class("flat");
    pause_button.set_tooltip_text(Some("Pause Recording"));
    pause_button.set_sensitive(false);
    pill_box.append(&pause_button);

    // 4. 60 FPS Visualizer drawing area (compact 46x24)
    let visualizer_state = Rc::new(RefCell::new(VisualizerState::new()));
    let visualizer_area = build_visualizer_drawing_area(visualizer_state.clone());
    pill_box.append(&visualizer_area);

    // 5. Spinner for processing state
    let spinner = gtk4::Spinner::new();
    spinner.set_spinning(false);
    spinner.set_visible(false);
    pill_box.append(&spinner);

    // 6. Cancel button
    let cancel_button = gtk4::Button::from_icon_name("edit-delete-symbolic");
    cancel_button.add_css_class("circular");
    cancel_button.add_css_class("flat");
    cancel_button.set_tooltip_text(Some("Cancel Recording"));
    cancel_button.set_sensitive(false);
    pill_box.append(&cancel_button);

    // 7. Separator line
    let sep = gtk4::Separator::new(gtk4::Orientation::Vertical);
    pill_box.append(&sep);

    // 8. Tone selector dropdown
    let tone_dropdown = gtk4::DropDown::from_strings(&["Clean", "Professional", "Concise", "Raw"]);
    tone_dropdown.set_tooltip_text(Some("Select Tone"));
    tone_dropdown.add_css_class("flat");
    pill_box.append(&tone_dropdown);

    // Expandable text preview drawer
    let (drawer_revealer, drawer_widgets) = build_preview_drawer();

    // 9. Quick Copy button (kept in struct for API compatibility, omitted from pill to keep compact)
    let copy_button = gtk4::Button::from_icon_name("edit-copy-symbolic");
    copy_button.add_css_class("circular");
    copy_button.add_css_class("flat");
    copy_button.set_tooltip_text(Some("Quick Copy to Clipboard"));
    copy_button.set_visible(false);

    let copy_btn_clone = copy_button.clone();
    let drawer_w_clone = drawer_widgets.clone();
    copy_button.connect_clicked(move |_| {
        let text = drawer_w_clone.text();
        if !text.is_empty()
            && let Ok(mut clip) = arboard::Clipboard::new()
        {
            let _ = clip.set_text(text);
        }
        copy_btn_clone.set_icon_name("object-select-symbolic");
        let btn = copy_btn_clone.clone();
        gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
            btn.set_icon_name("edit-copy-symbolic");
        });
    });

    // 10. Theme Toggle button (single click handler wired by MainWindow to prevent double-toggle)
    let theme_button = gtk4::Button::from_icon_name("weather-clear-symbolic");
    theme_button.add_css_class("circular");
    theme_button.add_css_class("flat");
    theme_button.set_tooltip_text(Some("Switch Theme"));
    pill_box.append(&theme_button);

    // 11. Settings button with proper org.gnome.Settings-symbolic gear icon
    let dashboard_button = gtk4::Button::from_icon_name("org.gnome.Settings-symbolic");
    dashboard_button.add_css_class("circular");
    dashboard_button.add_css_class("flat");
    dashboard_button.set_tooltip_text(Some("Settings"));
    pill_box.append(&dashboard_button);

    // 12. Drawer expander button
    let drawer_button = gtk4::Button::from_icon_name("pan-down-symbolic");
    drawer_button.add_css_class("circular");
    drawer_button.add_css_class("flat");
    drawer_button.set_tooltip_text(Some("Toggle Transcription Preview"));
    pill_box.append(&drawer_button);

    main_col.append(&handle);
    main_col.append(&drawer_revealer);

    // Wire local drawer button click to toggle revealer and request window resize
    let rev_clone = drawer_revealer.clone();
    let btn_clone = drawer_button.clone();
    let win_weak_drawer = window.downgrade();
    drawer_button.connect_clicked(move |_| {
        let is_revealed = rev_clone.reveals_child();
        rev_clone.set_reveal_child(!is_revealed);
        if !is_revealed {
            btn_clone.set_icon_name("pan-up-symbolic");
            btn_clone.set_tooltip_text(Some("Collapse Transcription Preview"));
        } else {
            btn_clone.set_icon_name("pan-down-symbolic");
            btn_clone.set_tooltip_text(Some("Expand Transcription Preview"));
        }
        if let Some(win) = win_weak_drawer.upgrade() {
            win.queue_resize();
        }
    });

    // Warning tooltip popover anchored to record button
    let warning_tooltip_popover = gtk4::Popover::new();
    warning_tooltip_popover.set_parent(&record_button);
    warning_tooltip_popover.set_position(gtk4::PositionType::Bottom);
    warning_tooltip_popover.set_autohide(false);
    warning_tooltip_popover.set_can_focus(false);
    warning_tooltip_popover.add_css_class("warning-tooltip-popover");

    let warning_tooltip_revealer = gtk4::Revealer::new();
    warning_tooltip_revealer.set_transition_type(gtk4::RevealerTransitionType::Crossfade);
    warning_tooltip_revealer.set_transition_duration(220);

    let warn_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    warn_card.add_css_class("warning-tooltip-card");

    let warn_icon = gtk4::Image::from_icon_name("dialog-warning-symbolic");
    warn_icon.set_pixel_size(16);
    warn_card.append(&warn_icon);

    let warning_tooltip_label = gtk4::Label::new(None);
    warning_tooltip_label.set_wrap(true);
    warning_tooltip_label.set_max_width_chars(36);
    warning_tooltip_label.set_halign(gtk4::Align::Start);
    warn_card.append(&warning_tooltip_label);

    warning_tooltip_revealer.set_child(Some(&warn_card));
    warning_tooltip_popover.set_child(Some(&warning_tooltip_revealer));

    let recording_warning = Rc::new(RefCell::new(None));
    let tooltip_generation = Rc::new(std::cell::Cell::new(0));

    window.set_child(Some(&main_col));

    MiniBarWidgets {
        window: window.clone(),
        pill_box,
        drag_grip,
        record_button,
        pause_button,
        cancel_button,
        visualizer_area,
        visualizer_state,
        spinner,
        tone_dropdown,
        copy_button,
        theme_button,
        dashboard_button,
        drawer_button,
        drawer_revealer,
        drawer_widgets,
        warning_tooltip_popover,
        warning_tooltip_revealer,
        warning_tooltip_label,
        recording_warning,
        tooltip_generation,
    }
}

/// Wires standard click signals for the record, pause, and cancel buttons using a shared model.
pub fn wire_minibar_internal_signals(widgets: &MiniBarWidgets, model: Rc<RefCell<MiniBarModel>>) {
    let m = model.clone();
    let w = widgets.clone();
    widgets.record_button.connect_clicked(move |_| {
        if m.borrow().state() == MiniBarState::Idle && !w.try_start_recording_or_warn() {
            return;
        }
        m.borrow_mut().update(MiniBarMsg::ToggleRecord);
        update_minibar_ui(&m.borrow(), &w);
    });

    let m = model.clone();
    let w = widgets.clone();
    widgets.pause_button.connect_clicked(move |_| {
        m.borrow_mut().update(MiniBarMsg::TogglePause);
        update_minibar_ui(&m.borrow(), &w);
    });

    let m = model.clone();
    let w = widgets.clone();
    widgets.cancel_button.connect_clicked(move |_| {
        m.borrow_mut().update(MiniBarMsg::CancelRecording);
        update_minibar_ui(&m.borrow(), &w);
    });
}

/// Builds the standalone frameless MiniBar floating window and widgets.
pub fn build_minibar_window() -> (gtk4::Window, MiniBarWidgets) {
    crate::ui::theme::ensure_app_icons_registered();
    ensure_minibar_css();

    let window = gtk4::Window::builder()
        .title("OpenDictate MiniBar")
        .icon_name("opendictate-dark")
        .decorated(false)
        .resizable(true)
        .build();

    window.add_css_class("minibar-window");

    let widgets = build_minibar_widgets(&window);

    (window, widgets)
}

/// Updates MiniBar widgets based on model state.
pub fn update_minibar_ui(model: &MiniBarModel, widgets: &MiniBarWidgets) {
    match model.state() {
        MiniBarState::Idle => {
            widgets.record_button.set_icon_name("media-record-symbolic");
            widgets
                .record_button
                .set_tooltip_text(Some("Start Recording"));
            widgets.record_button.remove_css_class("destructive-action");
            widgets.record_button.remove_css_class("recording-active");
            widgets.record_button.add_css_class("suggested-action");
            widgets.record_button.set_sensitive(true);

            widgets
                .pause_button
                .set_icon_name("media-playback-pause-symbolic");
            widgets
                .pause_button
                .set_tooltip_text(Some("Pause Recording"));
            widgets.pause_button.set_sensitive(false);

            widgets.cancel_button.set_sensitive(false);

            widgets.spinner.set_spinning(false);
            widgets.spinner.set_visible(false);
            widgets.visualizer_area.set_visible(true);
            widgets.visualizer_state.borrow_mut().set_recording(false);
            widgets.visualizer_state.borrow_mut().reset_idle();
            widgets.visualizer_area.queue_draw();
            widgets.drawer_widgets.set_status("Ready");
        }
        MiniBarState::Recording => {
            widgets
                .record_button
                .set_icon_name("media-playback-stop-symbolic");
            widgets
                .record_button
                .set_tooltip_text(Some("Stop Recording"));
            widgets.record_button.remove_css_class("suggested-action");
            widgets.record_button.add_css_class("destructive-action");
            widgets.record_button.add_css_class("recording-active");
            widgets.record_button.set_sensitive(true);

            widgets
                .pause_button
                .set_icon_name("media-playback-pause-symbolic");
            widgets
                .pause_button
                .set_tooltip_text(Some("Pause Recording"));
            widgets.pause_button.set_sensitive(true);

            widgets.cancel_button.set_sensitive(true);

            widgets.spinner.set_spinning(false);
            widgets.spinner.set_visible(false);
            widgets.visualizer_area.set_visible(true);
            widgets.visualizer_state.borrow_mut().set_recording(true);
            widgets
                .visualizer_state
                .borrow_mut()
                .update_levels(model.audio_levels());
            widgets.visualizer_area.queue_draw();
            widgets.drawer_widgets.set_status("Recording…");
        }
        MiniBarState::Paused => {
            widgets
                .record_button
                .set_icon_name("media-playback-stop-symbolic");
            widgets
                .record_button
                .set_tooltip_text(Some("Stop Recording"));
            widgets.record_button.remove_css_class("suggested-action");
            widgets.record_button.remove_css_class("recording-active");
            widgets.record_button.add_css_class("destructive-action");
            widgets.record_button.set_sensitive(true);

            widgets
                .pause_button
                .set_icon_name("media-playback-start-symbolic");
            widgets
                .pause_button
                .set_tooltip_text(Some("Resume Recording"));
            widgets.pause_button.set_sensitive(true);

            widgets.cancel_button.set_sensitive(true);

            widgets.spinner.set_spinning(false);
            widgets.spinner.set_visible(false);
            widgets.visualizer_area.set_visible(true);
            widgets.visualizer_state.borrow_mut().set_recording(false);
            widgets
                .visualizer_state
                .borrow_mut()
                .update_levels(model.audio_levels());
            widgets.visualizer_area.queue_draw();
            widgets.drawer_widgets.set_status("Paused");
        }
        MiniBarState::Processing => {
            widgets.record_button.set_sensitive(false);
            widgets.record_button.remove_css_class("recording-active");
            widgets.pause_button.set_sensitive(false);
            widgets.cancel_button.set_sensitive(false);

            widgets.visualizer_state.borrow_mut().set_recording(false);
            widgets.visualizer_area.set_visible(false);
            widgets.spinner.set_visible(true);
            widgets.spinner.set_spinning(true);
            widgets.drawer_widgets.set_status("Processing…");
        }
    }

    // Synchronize drawer expander button icon and state
    let is_expanded = model.is_drawer_expanded();
    widgets.drawer_widgets.set_expanded(is_expanded);
    if is_expanded {
        widgets.drawer_button.set_icon_name("pan-up-symbolic");
        widgets
            .drawer_button
            .set_tooltip_text(Some("Collapse Transcription Preview"));
    } else {
        widgets.drawer_button.set_icon_name("pan-down-symbolic");
        widgets
            .drawer_button
            .set_tooltip_text(Some("Expand Transcription Preview"));
    }

    // Synchronize drawer text
    widgets.drawer_widgets.set_text(model.transcript());
}

impl SimpleComponent for MiniBarModel {
    type Input = MiniBarMsg;
    type Output = ();
    type Init = ();
    type Root = gtk4::Window;
    type Widgets = MiniBarWidgets;

    fn init_root() -> Self::Root {
        crate::ui::theme::ensure_app_icons_registered();
        ensure_minibar_css();
        let window = gtk4::Window::builder()
            .title("OpenDictate MiniBar")
            .icon_name("opendictate-dark")
            .decorated(false)
            .resizable(true)
            .build();
        window.add_css_class("minibar-window");
        window
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MiniBarModel::new();
        let widgets = build_minibar_widgets(&root);
        root.present();

        // Wire record button
        {
            let sender_clone = sender.clone();
            let widgets_clone = widgets.clone();
            widgets.record_button.connect_clicked(move |_| {
                if widgets_clone.try_start_recording_or_warn() {
                    sender_clone.input(MiniBarMsg::ToggleRecord);
                }
            });
        }

        // Wire pause button
        {
            let sender_clone = sender.clone();
            widgets.pause_button.connect_clicked(move |_| {
                sender_clone.input(MiniBarMsg::TogglePause);
            });
        }

        // Wire cancel button
        {
            let sender_clone = sender.clone();
            widgets.cancel_button.connect_clicked(move |_| {
                sender_clone.input(MiniBarMsg::CancelRecording);
            });
        }

        // Wire drawer toggle button
        {
            let sender_clone = sender.clone();
            widgets.drawer_button.connect_clicked(move |_| {
                sender_clone.input(MiniBarMsg::ToggleDrawer);
            });
        }

        update_minibar_ui(&model, &widgets);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.update(message);
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        update_minibar_ui(self, widgets);
    }
}
