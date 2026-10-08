//! Native GNOME MainWindow settings dashboard module (Relm4 + Libadwaita).

pub mod crash_dialog;
pub mod header;
pub mod settings_view;


use crate::config::Config;
use crate::services::dictation_worker::{
    DictationWorker, DictationWorkerInput, DictationWorkerOutput,
};
use crate::services::storage::StorageService;
use crate::services::tray::{OpenDictateTray, TrayState};
use crate::ui::theme::{ThemeMode, sync_theme_with_adwaita};
use gtk4::gio;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use relm4::component::{ComponentParts, SimpleComponent};
use relm4::{Component, ComponentSender};
use std::path::PathBuf;

/// Messages processed by the MainWindow component.
#[derive(Debug, Clone, PartialEq)]
pub enum MainWindowMsg {
    ToggleTheme,
    SetTheme(ThemeMode),
    ToggleMiniBar,
    ShowDashboard,
    HideDashboard,
    SetAiProvider(String),
    SetAiApiKey(String),
    SetAiModel(String),
    SetAiMode(String),
    SetLocalModelId(String),
    SetLocalCustomPath(Option<String>),
    SetLocalThreads(u32),
    SetAudioDevice(Option<String>),
    SetHotkey(String),
    SaveSettings,
    DictationStatus(String),
    DictationAudioLevel(f32),
    DictationProcessing,
    DictationSuccess { raw: String, enhanced: String },
    DictationError(String),
    TriggerDictationToggle,
    SetTone(String),
    SetUiLanguage(String),
    SetMiniBarScale(u32),
}

/// Model holding the state for the MainWindow dashboard.
#[derive(Debug, Clone)]
pub struct MainWindowModel {
    config: Config,
    config_path: Option<PathBuf>,
    storage: StorageService,
    theme_mode: ThemeMode,
    minibar_visible: bool,
    dashboard_visible: bool,
    dictation_status: String,
    audio_level: f32,
    is_processing: bool,
    last_raw_text: String,
    last_enhanced_text: String,
    tone: String,
    worker_sender: Option<relm4::Sender<DictationWorkerInput>>,
}

impl MainWindowModel {
    /// Initializes a new MainWindowModel with default config path.
    pub fn new(config: Config, storage: StorageService) -> Self {
        Self::new_with_config_path(config, storage, Config::default_path())
    }

    /// Initializes a new MainWindowModel with a custom config path.
    pub fn new_with_config_path(
        config: Config,
        storage: StorageService,
        config_path: PathBuf,
    ) -> Self {
        let theme_mode = ThemeMode::from_str(&config.theme);
        sync_theme_with_adwaita(theme_mode);
        let tone = if config.tone.trim().is_empty() {
            "Clean".to_string()
        } else {
            config.tone.clone()
        };

        Self {
            config,
            config_path: Some(config_path),
            storage,
            theme_mode,
            minibar_visible: false,
            dashboard_visible: false,
            dictation_status: "Ready".to_string(),
            audio_level: 0.0,
            is_processing: false,
            last_raw_text: String::new(),
            last_enhanced_text: String::new(),
            tone,
            worker_sender: None,
        }
    }

    /// Returns the current active theme mode.
    pub fn theme_mode(&self) -> ThemeMode {
        self.theme_mode
    }

    /// Returns a reference to the active configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns a reference to the active storage service.
    pub fn storage(&self) -> &StorageService {
        &self.storage
    }

    /// Returns a mutable reference to the active configuration.
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Returns whether the floating minibar is toggled visible.
    pub fn is_minibar_visible(&self) -> bool {
        self.minibar_visible
    }

    /// Sets whether the floating minibar is visible.
    pub fn set_minibar_visible(&mut self, visible: bool) {
        self.minibar_visible = visible;
    }

    /// Returns whether the dashboard window is toggled visible.
    pub fn is_dashboard_visible(&self) -> bool {
        self.dashboard_visible
    }

    /// Sets whether the dashboard window is visible.
    pub fn set_dashboard_visible(&mut self, visible: bool) {
        self.dashboard_visible = visible;
    }

    /// Returns current dictation worker status string.
    pub fn dictation_status(&self) -> &str {
        &self.dictation_status
    }

    /// Returns current live audio level in range [0.0, 1.0].
    pub fn audio_level(&self) -> f32 {
        self.audio_level
    }

    /// Returns whether dictation is currently in AI processing/transcription state.
    pub fn is_processing(&self) -> bool {
        self.is_processing
    }

    /// Returns the last transcribed raw text.
    pub fn last_raw_text(&self) -> &str {
        &self.last_raw_text
    }

    /// Returns the last AI-enhanced text.
    pub fn last_enhanced_text(&self) -> &str {
        &self.last_enhanced_text
    }

    /// Returns active transcription tone.
    pub fn tone(&self) -> &str {
        &self.tone
    }

    /// Sets active transcription tone.
    pub fn set_tone(&mut self, tone: String) {
        self.tone = tone;
    }

    /// Sets the UI localization language and persists configuration.
    pub fn set_ui_language(&mut self, lang: &str) {
        self.config.ui_language = lang.to_string();
        self.persist_config();
    }

    /// Sets the MiniBar zoom scale percentage (clamped 70..=200) and persists configuration.
    pub fn set_minibar_scale(&mut self, scale: u32) {
        self.config.minibar_scale = scale.clamp(70, 200);
        self.persist_config();
    }

    /// Sets background dictation worker sender.
    pub fn set_worker_sender(&mut self, sender: relm4::Sender<DictationWorkerInput>) {
        self.worker_sender = Some(sender);
    }

    /// Returns reference to worker sender, if attached.
    pub fn worker_sender(&self) -> Option<&relm4::Sender<DictationWorkerInput>> {
        self.worker_sender.as_ref()
    }

    /// Updates the model state based on the incoming message.
    pub fn update(&mut self, msg: MainWindowMsg) {
        match msg {
            MainWindowMsg::ToggleTheme => {
                self.theme_mode = self.theme_mode.toggle();
                self.config.theme = self.theme_mode.as_str().to_string();
                sync_theme_with_adwaita(self.theme_mode);
                self.persist_config();
            }
            MainWindowMsg::SetTheme(mode) => {
                self.theme_mode = mode;
                self.config.theme = mode.as_str().to_string();
                sync_theme_with_adwaita(self.theme_mode);
                self.persist_config();
            }
            MainWindowMsg::ToggleMiniBar => {
                self.minibar_visible = !self.minibar_visible;
            }
            MainWindowMsg::ShowDashboard => {
                self.dashboard_visible = true;
            }
            MainWindowMsg::HideDashboard => {
                self.dashboard_visible = false;
                self.minibar_visible = true;
            }
            MainWindowMsg::SetAiProvider(provider) => {
                self.config.ai_provider = provider;
            }
            MainWindowMsg::SetAiApiKey(key) => {
                self.config.ai_api_key = key;
            }
            MainWindowMsg::SetAiModel(model) => {
                self.config.ai_model = model;
            }
            MainWindowMsg::SetAiMode(mode) => {
                self.config.ai_mode = mode;
            }
            MainWindowMsg::SetLocalModelId(model_id) => {
                self.config.local_model_id = model_id;
            }
            MainWindowMsg::SetLocalCustomPath(path) => {
                self.config.local_custom_path = path;
            }
            MainWindowMsg::SetLocalThreads(threads) => {
                self.config.local_threads = threads;
            }
            MainWindowMsg::SetAudioDevice(device) => {
                self.config.audio_device = device;
                if let Some(ref sender) = self.worker_sender {
                    let _ = sender.send(DictationWorkerInput::UpdateConfig(self.config.clone()));
                }
            }
            MainWindowMsg::SetHotkey(hotkey) => {
                self.config.hotkey = hotkey;
            }
            MainWindowMsg::SaveSettings => {
                self.persist_config();
                if let Some(ref sender) = self.worker_sender {
                    let _ = sender.send(DictationWorkerInput::UpdateConfig(self.config.clone()));
                }
            }
            MainWindowMsg::DictationStatus(status) => {
                if status == "No speech detected" {
                    self.is_processing = false;
                    self.audio_level = 0.0;
                }
                self.dictation_status = status;
            }
            MainWindowMsg::DictationAudioLevel(lvl) => {
                self.audio_level = lvl;
            }
            MainWindowMsg::DictationProcessing => {
                self.is_processing = true;
                self.dictation_status = "Transcribing...".to_string();
            }
            MainWindowMsg::DictationSuccess { raw, enhanced } => {
                self.is_processing = false;
                self.dictation_status = "Ready".to_string();
                self.last_raw_text = raw;
                self.last_enhanced_text = enhanced;
                self.audio_level = 0.0;
            }
            MainWindowMsg::DictationError(err) => {
                self.is_processing = false;
                self.dictation_status = format!("Error: {}", err);
                self.audio_level = 0.0;
            }
            MainWindowMsg::TriggerDictationToggle => {
                if let Some(ref sender) = self.worker_sender {
                    let _ = sender.send(DictationWorkerInput::ToggleRecording {
                        tone: self.tone.clone(),
                    });
                }
            }
            MainWindowMsg::SetTone(tone) => {
                self.tone = tone.clone();
                self.config.tone = tone;
                self.persist_config();
            }
            MainWindowMsg::SetUiLanguage(lang) => {
                self.set_ui_language(&lang);
            }
            MainWindowMsg::SetMiniBarScale(scale) => {
                self.set_minibar_scale(scale);
            }
        }
    }

    fn persist_config(&self) {
        if let Some(ref path) = self.config_path {
            let _ = self.config.save_to(path);
        } else {
            let _ = self.config.save();
        }
    }
}

/// Widgets tracked for the Relm4 MainWindow component.
pub struct MainWindowWidgets {
    pub window: libadwaita::ApplicationWindow,
    pub header_widgets: header::HeaderBarWidgets,
    pub settings_widgets: settings_view::SettingsViewWidgets,
    pub minibar_window: gtk4::Window,
    pub minibar_widgets: crate::ui::mini_bar::MiniBarWidgets,
    pub worker: relm4::WorkerController<DictationWorker>,
    pub tray_handle: Option<ksni::Handle<OpenDictateTray>>,
    pub applied_ui_language: std::cell::RefCell<String>,
    pub applied_minibar_scale: std::cell::RefCell<u32>,
}

impl MainWindowWidgets {
    /// Dynamically retranslates all MainWindow sub-components in-place.
    pub fn retranslate(&self, lang: &str) {
        *self.applied_ui_language.borrow_mut() = lang.to_string();

        let dir = if crate::services::i18n::is_rtl(lang) {
            gtk4::TextDirection::Rtl
        } else {
            gtk4::TextDirection::Ltr
        };
        self.window.set_direction(dir);

        self.window
            .set_title(Some(crate::services::i18n::tr("settings", lang)));
        self.header_widgets.retranslate(lang);
        self.settings_widgets.retranslate(lang);
    }
}

/// Initialization parameter for MainWindow Relm4 component.
#[derive(Clone)]
pub struct MainWindowInit {
    pub config: Config,
    pub storage: StorageService,
    pub show_minibar: bool,
    pub show_dashboard: bool,
    pub tray_handle: Option<ksni::Handle<OpenDictateTray>>,
}

impl std::fmt::Debug for MainWindowInit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainWindowInit")
            .field("config", &self.config)
            .field("storage", &self.storage)
            .field("show_minibar", &self.show_minibar)
            .field("show_dashboard", &self.show_dashboard)
            .field("has_tray_handle", &self.tray_handle.is_some())
            .finish()
    }
}

impl MainWindowInit {
    /// Creates a new MainWindowInit defaulting to showing both floating MiniBar and dashboard.
    pub fn new(config: Config, storage: StorageService) -> Self {
        Self {
            config,
            storage,
            show_minibar: true,
            show_dashboard: true,
            tray_handle: None,
        }
    }

    /// Sets the DBus system tray handle.
    pub fn with_tray_handle(mut self, handle: Option<ksni::Handle<OpenDictateTray>>) -> Self {
        self.tray_handle = handle;
        self
    }
}

impl SimpleComponent for MainWindowModel {
    type Input = MainWindowMsg;
    type Output = ();
    type Init = MainWindowInit;
    type Root = libadwaita::ApplicationWindow;
    type Widgets = MainWindowWidgets;

    fn init_root() -> Self::Root {
        libadwaita::ApplicationWindow::builder()
            .title("Settings")
            .default_width(750)
            .default_height(600)
            .build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = MainWindowModel::new(init.config.clone(), init.storage.clone());
        model.set_minibar_visible(init.show_minibar);
        model.set_dashboard_visible(init.show_dashboard);

        // Build floating MiniBar island window and widgets
        let (minibar_window, minibar_widgets) = crate::ui::mini_bar::build_minibar_window();
        let main_app = relm4::main_application();
        if main_app.is_registered() {
            main_app.add_window(&minibar_window);
        }
        minibar_widgets.apply_scale(init.config.minibar_scale);

        // Attach DictationWorker background worker
        let worker_handle =
            DictationWorker::builder().detach_worker((init.config.clone(), init.storage.clone()));

        let mb_widgets_clone = minibar_widgets.clone();
        let sender_for_worker = sender.clone();
        let tray_handle_for_worker = init.tray_handle.clone();
        let worker = worker_handle.connect_receiver(move |_worker_in_sender, output| {
            if let Some(ref handle) = tray_handle_for_worker {
                match &output {
                    DictationWorkerOutput::RecordingStarted
                    | DictationWorkerOutput::RecordingResumed => {
                        handle.update(|tray| tray.set_state(TrayState::Recording));
                    }
                    DictationWorkerOutput::RecordingPaused => {
                        handle.update(|tray| tray.set_state(TrayState::Paused));
                    }
                    DictationWorkerOutput::ProcessingStarted => {
                        handle.update(|tray| tray.set_state(TrayState::Processing));
                    }
                    DictationWorkerOutput::Success { .. }
                    | DictationWorkerOutput::Error(_)
                    | DictationWorkerOutput::RecordingCancelled
                    | DictationWorkerOutput::NoSpeechDetected => {
                        handle.update(|tray| tray.set_state(TrayState::Idle));
                    }
                    _ => {}
                }
            }

            match output {
                DictationWorkerOutput::AudioLevel(lvl) => {
                    let boosted = (lvl * 1.6).clamp(0.0, 1.0);
                    let levels = [
                        boosted * 0.65,
                        boosted * 0.9,
                        boosted,
                        boosted * 0.9,
                        boosted * 0.65,
                    ];
                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .update_levels(levels);
                    mb_widgets_clone.visualizer_area.queue_draw();
                }
                DictationWorkerOutput::ProcessingStarted => {
                    mb_widgets_clone.record_button.set_sensitive(false);
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("recording-active");
                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(false);
                    mb_widgets_clone.pause_button.set_sensitive(false);
                    mb_widgets_clone.cancel_button.set_sensitive(false);
                    mb_widgets_clone.visualizer_area.set_visible(false);
                    mb_widgets_clone.spinner.set_visible(true);
                    mb_widgets_clone.spinner.set_spinning(true);
                    mb_widgets_clone
                        .drawer_widgets
                        .set_status("Transcribing...");
                    sender_for_worker.input(MainWindowMsg::DictationProcessing);
                }
                DictationWorkerOutput::Success {
                    raw_text,
                    enhanced_text,
                    ..
                } => {
                    mb_widgets_clone
                        .record_button
                        .set_icon_name("media-record-symbolic");
                    mb_widgets_clone
                        .record_button
                        .set_tooltip_text(Some("Start Recording"));
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("destructive-action");
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("recording-active");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("suggested-action");
                    mb_widgets_clone.record_button.set_sensitive(true);

                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-pause-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Pause Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(false);

                    mb_widgets_clone.cancel_button.set_sensitive(false);

                    mb_widgets_clone.spinner.set_spinning(false);
                    mb_widgets_clone.spinner.set_visible(false);
                    mb_widgets_clone.visualizer_area.set_visible(true);
                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(false);
                    mb_widgets_clone.visualizer_state.borrow_mut().reset_idle();
                    mb_widgets_clone.visualizer_area.queue_draw();

                    let display_text = if !enhanced_text.is_empty() {
                        &enhanced_text
                    } else {
                        &raw_text
                    };
                    mb_widgets_clone.drawer_widgets.set_text(display_text);
                    mb_widgets_clone.drawer_widgets.set_status("Ready");
                    crate::ui::mini_bar::copy_text_to_clipboard(display_text);
                    mb_widgets_clone.expand_drawer();

                    sender_for_worker.input(MainWindowMsg::DictationSuccess {
                        raw: raw_text,
                        enhanced: enhanced_text,
                    });
                }
                DictationWorkerOutput::Error(err) => {
                    mb_widgets_clone
                        .record_button
                        .set_icon_name("media-record-symbolic");
                    mb_widgets_clone
                        .record_button
                        .set_tooltip_text(Some("Start Recording"));
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("destructive-action");
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("recording-active");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("suggested-action");
                    mb_widgets_clone.record_button.set_sensitive(true);

                    mb_widgets_clone.pause_button.set_sensitive(false);
                    mb_widgets_clone.cancel_button.set_sensitive(false);

                    mb_widgets_clone.spinner.set_spinning(false);
                    mb_widgets_clone.spinner.set_visible(false);
                    mb_widgets_clone.visualizer_area.set_visible(true);
                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(false);
                    mb_widgets_clone.visualizer_state.borrow_mut().reset_idle();
                    mb_widgets_clone.visualizer_area.queue_draw();

                    mb_widgets_clone.drawer_widgets.set_status("Error");
                    mb_widgets_clone.show_warning_tooltip(&err);
                    sender_for_worker.input(MainWindowMsg::DictationError(err));
                }
                DictationWorkerOutput::StatusMessage(msg) => {
                    mb_widgets_clone.drawer_widgets.set_status(&msg);
                    sender_for_worker.input(MainWindowMsg::DictationStatus(msg));
                }
                DictationWorkerOutput::RecordingStarted => {
                    mb_widgets_clone
                        .record_button
                        .set_icon_name("media-playback-stop-symbolic");
                    mb_widgets_clone
                        .record_button
                        .set_tooltip_text(Some("Stop Recording"));
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("suggested-action");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("destructive-action");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("recording-active");
                    mb_widgets_clone.record_button.set_sensitive(true);

                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-pause-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Pause Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(true);

                    mb_widgets_clone.cancel_button.set_sensitive(true);

                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(true);
                    mb_widgets_clone.visualizer_area.queue_draw();

                    mb_widgets_clone.drawer_widgets.set_status("Recording...");
                    sender_for_worker
                        .input(MainWindowMsg::DictationStatus("Recording...".to_string()));
                }
                DictationWorkerOutput::RecordingPaused => {
                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-start-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Resume Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(true);

                    mb_widgets_clone.drawer_widgets.set_status("Paused");
                    sender_for_worker.input(MainWindowMsg::DictationStatus("Paused".to_string()));
                }
                DictationWorkerOutput::RecordingResumed => {
                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-pause-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Pause Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(true);

                    mb_widgets_clone.drawer_widgets.set_status("Recording...");
                    sender_for_worker
                        .input(MainWindowMsg::DictationStatus("Recording...".to_string()));
                }
                DictationWorkerOutput::RecordingCancelled => {
                    mb_widgets_clone
                        .record_button
                        .set_icon_name("media-record-symbolic");
                    mb_widgets_clone
                        .record_button
                        .set_tooltip_text(Some("Start Recording"));
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("destructive-action");
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("recording-active");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("suggested-action");
                    mb_widgets_clone.record_button.set_sensitive(true);

                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-pause-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Pause Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(false);

                    mb_widgets_clone.cancel_button.set_sensitive(false);

                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(false);
                    mb_widgets_clone.visualizer_state.borrow_mut().reset_idle();
                    mb_widgets_clone.visualizer_area.queue_draw();

                    mb_widgets_clone.drawer_widgets.set_status("Cancelled");
                    sender_for_worker
                        .input(MainWindowMsg::DictationStatus("Cancelled".to_string()));
                }
                DictationWorkerOutput::NoSpeechDetected => {
                    mb_widgets_clone
                        .record_button
                        .set_icon_name("media-record-symbolic");
                    mb_widgets_clone
                        .record_button
                        .set_tooltip_text(Some("Start Recording"));
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("destructive-action");
                    mb_widgets_clone
                        .record_button
                        .remove_css_class("recording-active");
                    mb_widgets_clone
                        .record_button
                        .add_css_class("suggested-action");
                    mb_widgets_clone.record_button.set_sensitive(true);

                    mb_widgets_clone
                        .pause_button
                        .set_icon_name("media-playback-pause-symbolic");
                    mb_widgets_clone
                        .pause_button
                        .set_tooltip_text(Some("Pause Recording"));
                    mb_widgets_clone.pause_button.set_sensitive(false);

                    mb_widgets_clone.cancel_button.set_sensitive(false);

                    mb_widgets_clone.spinner.set_spinning(false);
                    mb_widgets_clone.spinner.set_visible(false);
                    mb_widgets_clone.visualizer_area.set_visible(true);
                    mb_widgets_clone
                        .visualizer_state
                        .borrow_mut()
                        .set_recording(false);
                    mb_widgets_clone.visualizer_state.borrow_mut().reset_idle();
                    mb_widgets_clone.visualizer_area.queue_draw();

                    mb_widgets_clone
                        .drawer_widgets
                        .set_status("No speech detected");
                    mb_widgets_clone.show_warning_tooltip("No speech detected");
                    sender_for_worker.input(MainWindowMsg::DictationStatus(
                        "No speech detected".to_string(),
                    ));
                }
            }
        });

        // Store worker sender in model for triggering actions
        model.set_worker_sender(worker.sender().clone());

        let active_cfg = std::rc::Rc::new(std::cell::RefCell::new(init.config.clone()));

        // Connect record button to worker: Toggle recording (guarded by active model & readiness warnings)
        let worker_sender_rec = worker.sender().clone();
        let tone_ref = model.tone.clone();
        let mb_rec_guard = minibar_widgets.clone();
        let active_cfg_rec = active_cfg.clone();
        let sender_rec_dash = sender.clone();
        let root_rec_dash = root.clone();
        let mb_win_rec_dash = minibar_window.clone();
        minibar_widgets.record_button.connect_clicked(move |_| {
            let is_currently_recording = mb_rec_guard
                .record_button
                .has_css_class("recording-active");
            if !is_currently_recording
                && let Some(readiness_warn) =
                    crate::ui::mini_bar::evaluate_recording_readiness_warning(
                        &active_cfg_rec.borrow(),
                    )
            {
                mb_rec_guard.show_warning_tooltip(&readiness_warn);
                sender_rec_dash.input(MainWindowMsg::ShowDashboard);
                root_rec_dash.present();
                mb_win_rec_dash.set_transient_for(Some(&root_rec_dash));
                mb_win_rec_dash.present();
                return;
            }
            if mb_rec_guard.try_start_recording_or_warn() {
                let _ = worker_sender_rec.send(DictationWorkerInput::ToggleRecording {
                    tone: tone_ref.clone(),
                });
            }
        });

        // Connect pause button to worker
        let worker_sender_pause = worker.sender().clone();
        minibar_widgets.pause_button.connect_clicked(move |btn| {
            let is_resume = btn
                .icon_name()
                .map(|n| n == "media-playback-start-symbolic")
                .unwrap_or(false);
            if is_resume {
                let _ = worker_sender_pause.send(DictationWorkerInput::ResumeRecording);
            } else {
                let _ = worker_sender_pause.send(DictationWorkerInput::PauseRecording);
            }
        });

        // Connect cancel button to worker
        let worker_sender_cancel = worker.sender().clone();
        minibar_widgets.cancel_button.connect_clicked(move |_| {
            let _ = worker_sender_cancel.send(DictationWorkerInput::CancelRecording);
        });

        // Connect minibar theme button to model theme toggle
        let sender_theme = sender.clone();
        minibar_widgets.theme_button.connect_clicked(move |_| {
            sender_theme.input(MainWindowMsg::ToggleTheme);
        });

        // Connect drawer clear button
        let dw_clear = minibar_widgets.drawer_widgets.clone();
        minibar_widgets
            .drawer_widgets
            .clear_button
            .connect_clicked(move |_| {
                dw_clear.clear();
            });

        // Sync tone dropdown with model
        let sender_tone = sender.clone();
        minibar_widgets
            .tone_dropdown
            .connect_selected_notify(move |dd| {
                let tone = match dd.selected() {
                    0 => "Clean",
                    1 => "Professional",
                    2 => "Concise",
                    3 => "Raw",
                    _ => "Clean",
                };
                sender_tone.input(MainWindowMsg::SetTone(tone.to_string()));
            });

        // Hide minibar when its close request fires instead of destroying
        let sender_clone = sender.clone();
        let mb_win_clone = minibar_window.clone();
        minibar_window.connect_close_request(move |_| {
            mb_win_clone.set_visible(false);
            sender_clone.input(MainWindowMsg::ToggleMiniBar);
            gtk4::glib::Propagation::Stop
        });

        // Hide dashboard when its close request fires instead of terminating app
        let root_clone = root.clone();
        let mb_win_close = minibar_window.clone();
        let sender_close = sender.clone();
        root.connect_close_request(move |_| {
            mb_win_close.set_transient_for(None::<&gtk4::Window>);
            root_clone.set_visible(false);
            mb_win_close.present();
            sender_close.input(MainWindowMsg::HideDashboard);
            gtk4::glib::Propagation::Stop
        });

        let (header_bar, header_widgets) = header::build_header_bar(&init.config, &init.storage);
        let (settings_view, settings_widgets) = settings_view::build_settings_view(&init.config);

        // Sync Cloud AI model warnings (paid plan required / audio unsupported) to MiniBar recording guard
        let mb_warn_sync = minibar_widgets.clone();
        settings_widgets.set_on_model_warning_change(move |warning| {
            mb_warn_sync.set_recording_warning(warning);
        });

        // Connect minibar settings button to present MainWindow while keeping minibar floating on top
        let root_clone = root.clone();
        let mb_win_dash = minibar_window.clone();
        let sender_dash = sender.clone();
        minibar_widgets.dashboard_button.connect_clicked(move |_| {
            sender_dash.input(MainWindowMsg::ShowDashboard);
            root_clone.present();
            mb_win_dash.set_transient_for(Some(&root_clone));
            mb_win_dash.present();
        });

        let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content_box.append(&header_bar);
        settings_view.set_vexpand(true);
        content_box.append(&settings_view);

        root.set_content(Some(&content_box));

        // Setup Main Menu actions
        setup_main_menu_actions(&root, active_cfg.clone());

        // Helper closure to extract settings widgets and synchronize with model + worker
        let sync_settings_to_sender = {
            let sender_sync = sender.clone();
            let sw_sync = settings_widgets.clone();
            let active_cfg_sync = active_cfg.clone();
            move || {
                let current_cfg = active_cfg_sync.borrow().clone();
                let extracted = settings_view::extract_config_from_widgets(&sw_sync, &current_cfg);
                *active_cfg_sync.borrow_mut() = extracted.clone();
                sender_sync.input(MainWindowMsg::SetAiMode(extracted.ai_mode));
                sender_sync.input(MainWindowMsg::SetLocalModelId(extracted.local_model_id));
                sender_sync.input(MainWindowMsg::SetLocalCustomPath(
                    extracted.local_custom_path,
                ));
                sender_sync.input(MainWindowMsg::SetLocalThreads(extracted.local_threads));
                sender_sync.input(MainWindowMsg::SetAiProvider(extracted.ai_provider));
                sender_sync.input(MainWindowMsg::SetAiApiKey(extracted.ai_api_key));
                sender_sync.input(MainWindowMsg::SetAiModel(extracted.ai_model));
                sender_sync.input(MainWindowMsg::SetAudioDevice(extracted.audio_device));
                sender_sync.input(MainWindowMsg::SetHotkey(extracted.hotkey));
                sender_sync.input(MainWindowMsg::SaveSettings);
            }
        };

        let sync_for_save = sync_settings_to_sender.clone();
        settings_widgets.save_button.connect_clicked(move |_| {
            sync_for_save();
        });

        let sync_for_auto = sync_settings_to_sender;
        settings_widgets.set_on_config_auto_sync(move || {
            sync_for_auto();
        });

        let sender_clone = sender.clone();
        let active_cfg_theme = active_cfg.clone();
        settings_widgets
            .theme_dropdown
            .connect_selected_notify(move |dd| {
                let mode = if dd.selected() == 1 {
                    ThemeMode::White
                } else {
                    ThemeMode::Dark
                };
                active_cfg_theme.borrow_mut().theme = mode.as_str().to_string();
                sender_clone.input(MainWindowMsg::SetTheme(mode));
            });

        // Sync initial or auto-detected audio device and react to hotplug/manual changes immediately
        {
            let initial_idx = settings_widgets.audio_device_row.selected() as usize;
            if initial_idx > 0 && initial_idx < settings_widgets.audio_devices.len() {
                let initial_dev = Some(settings_widgets.audio_devices[initial_idx].clone());
                active_cfg.borrow_mut().audio_device = initial_dev.clone();
                sender.input(MainWindowMsg::SetAudioDevice(initial_dev));
            }
            let sender_audio = sender.clone();
            let active_cfg_audio = active_cfg.clone();
            let audio_devices_list = settings_widgets.audio_devices.clone();
            settings_widgets
                .audio_device_row
                .connect_selected_notify(move |dd| {
                    let idx = dd.selected() as usize;
                    let dev = if idx == 0 || idx >= audio_devices_list.len() {
                        None
                    } else {
                        Some(audio_devices_list[idx].clone())
                    };
                    active_cfg_audio.borrow_mut().audio_device = dev.clone();
                    sender_audio.input(MainWindowMsg::SetAudioDevice(dev));
                });
        }

        // Attach GTK4 keyboard shortcut controllers to both MiniBar and Settings windows
        attach_window_shortcuts(
            &minibar_window,
            &init.config.hotkey,
            &minibar_widgets.record_button,
            &minibar_widgets.cancel_button,
        );
        attach_window_shortcuts(
            root.upcast_ref::<gtk4::Window>(),
            &init.config.hotkey,
            &minibar_widgets.record_button,
            &minibar_widgets.cancel_button,
        );

        // Apply initial theme CSS class and window icons
        crate::ui::theme::ensure_app_icons_registered();
        let initial_icon = model.theme_mode.icon_name();
        root.set_icon_name(Some(initial_icon));
        minibar_window.set_icon_name(Some(initial_icon));
        if init.config.theme.to_lowercase() == "white" {
            minibar_window.add_css_class("white-theme");
            root.add_css_class("white-theme");
        } else {
            minibar_window.add_css_class("dark-theme");
            root.add_css_class("dark-theme");
        }

        // Control initial visibility of both windows
        if init.show_minibar {
            minibar_window.present();
        } else {
            minibar_window.set_visible(false);
        }

        if init.show_dashboard {
            root.present();
        } else {
            root.set_visible(false);
        }

        // Connect language change from header menu to sender
        let sender_lang = sender.clone();
        header_widgets.set_on_language_change(move |lang| {
            sender_lang.input(MainWindowMsg::SetUiLanguage(lang));
        });

        // Connect minibar scale change from header menu to minibar widgets & sender
        let sender_scale = sender.clone();
        let mb_scale_widgets = minibar_widgets.clone();
        let active_cfg_scale = active_cfg.clone();
        header_widgets.set_on_scale_change(move |scale| {
            active_cfg_scale.borrow_mut().minibar_scale = scale;
            mb_scale_widgets.apply_scale(scale);
            sender_scale.input(MainWindowMsg::SetMiniBarScale(scale));
        });

        root.set_title(Some(crate::services::i18n::tr(
            "settings",
            &init.config.ui_language,
        )));

        // If a pending crash report exists from a previous session panic, present the recovery dialog
        if let Some(pending_crash) =
            crate::services::crash_reporter::CrashReporter::load_pending_crash()
        {
            let root_for_crash = root.clone();
            gtk4::glib::idle_add_local_once(move || {
                let crash_ui =
                    crash_dialog::build_crash_report_window(&pending_crash, Some(&root_for_crash));
                crash_ui.window.present();
            });
        }

        let widgets = MainWindowWidgets {
            window: root.clone(),
            header_widgets,
            settings_widgets,
            minibar_window,
            minibar_widgets,
            worker,
            tray_handle: init.tray_handle,
            applied_ui_language: std::cell::RefCell::new(init.config.ui_language.clone()),
            applied_minibar_scale: std::cell::RefCell::new(init.config.minibar_scale),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.update(message);
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        let theme_idx = if self.theme_mode == ThemeMode::White {
            1
        } else {
            0
        };
        if widgets.settings_widgets.theme_dropdown.selected() != theme_idx {
            widgets
                .settings_widgets
                .theme_dropdown
                .set_selected(theme_idx);
        }

        // Synchronize tone dropdown with model
        let expected_tone_idx = match self.tone.to_lowercase().as_str() {
            "clean" => 0,
            "professional" => 1,
            "concise" => 2,
            "raw" => 3,
            _ => 0,
        };
        if widgets.minibar_widgets.tone_dropdown.selected() != expected_tone_idx {
            widgets
                .minibar_widgets
                .tone_dropdown
                .set_selected(expected_tone_idx);
        }

        // Synchronize minibar theme button icon and window CSS classes
        let theme_icon = if self.theme_mode.is_dark() {
            "weather-clear-symbolic"
        } else {
            "weather-clear-night-symbolic"
        };
        widgets
            .minibar_widgets
            .theme_button
            .set_icon_name(theme_icon);
        widgets
            .minibar_widgets
            .theme_button
            .set_tooltip_text(Some("Switch Theme"));

        let app_icon = self.theme_mode.icon_name();
        widgets.window.set_icon_name(Some(app_icon));
        widgets.minibar_window.set_icon_name(Some(app_icon));

        if self.theme_mode.is_dark() {
            widgets.minibar_window.remove_css_class("white-theme");
            widgets.minibar_window.add_css_class("dark-theme");
            widgets.window.remove_css_class("white-theme");
            widgets.window.add_css_class("dark-theme");
        } else {
            widgets.minibar_window.remove_css_class("dark-theme");
            widgets.minibar_window.add_css_class("white-theme");
            widgets.window.remove_css_class("dark-theme");
            widgets.window.add_css_class("white-theme");
        }

        // Synchronize minibar visibility
        if self.minibar_visible {
            if !widgets.minibar_window.is_visible() {
                widgets.minibar_window.present();
            }
        } else if widgets.minibar_window.is_visible() {
            widgets.minibar_window.set_visible(false);
        }

        // Synchronize dashboard presentation
        if self.dashboard_visible {
            if !widgets.window.is_visible() {
                widgets.window.present();
                if self.minibar_visible {
                    widgets
                        .minibar_window
                        .set_transient_for(Some(&widgets.window));
                    widgets.minibar_window.present();
                }
            }
        } else if widgets.window.is_visible() {
            widgets
                .minibar_window
                .set_transient_for(None::<&gtk4::Window>);
            widgets.window.set_visible(false);
            if self.minibar_visible {
                widgets.minibar_window.present();
            }
        }

        // Synchronize tray theme checkmark
        if let Some(ref handle) = widgets.tray_handle {
            let is_dark = self.theme_mode.is_dark();
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                handle.update(|tray| {
                    if tray.is_dark_theme() != is_dark {
                        tray.set_dark_theme(is_dark);
                    }
                });
            }));
        }

        // Synchronize UI language
        if *widgets.applied_ui_language.borrow() != self.config.ui_language {
            *widgets.applied_ui_language.borrow_mut() = self.config.ui_language.clone();
            widgets.retranslate(&self.config.ui_language);
        }

        // Synchronize MiniBar scale
        if *widgets.applied_minibar_scale.borrow() != self.config.minibar_scale {
            *widgets.applied_minibar_scale.borrow_mut() = self.config.minibar_scale;
            widgets
                .minibar_widgets
                .apply_scale(self.config.minibar_scale);
            widgets.header_widgets.set_scale(self.config.minibar_scale);
        }
    }
}

/// Constructs and returns a standalone Libadwaita ApplicationWindow for Settings.
pub fn build_main_window(
    config: &Config,
    storage: &StorageService,
) -> libadwaita::ApplicationWindow {
    crate::ui::theme::ensure_app_icons_registered();
    crate::ui::mini_bar::ensure_minibar_css();
    crate::ui::mini_bar::apply_minibar_scale_css(config.minibar_scale);

    let lang = if config.ui_language.is_empty() {
        "en"
    } else {
        &config.ui_language
    };

    let theme_mode = ThemeMode::from_str(&config.theme);
    let window = libadwaita::ApplicationWindow::builder()
        .title(crate::services::i18n::tr("settings", lang))
        .icon_name(theme_mode.icon_name())
        .default_width(750)
        .default_height(600)
        .build();

    if config.theme.to_lowercase() == "white" {
        window.add_css_class("white-theme");
    } else {
        window.add_css_class("dark-theme");
    }

    let (header_bar, header_widgets) = header::build_header_bar(config, storage);
    let (settings_view, settings_widgets) = settings_view::build_settings_view(config);

    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content_box.append(&header_bar);
    settings_view.set_vexpand(true);
    content_box.append(&settings_view);

    window.set_content(Some(&content_box));

    let model = std::rc::Rc::new(std::cell::RefCell::new(MainWindowModel::new(
        config.clone(),
        storage.clone(),
    )));

    // Setup Main Menu actions
    let active_cfg = std::rc::Rc::new(std::cell::RefCell::new(config.clone()));
    setup_main_menu_actions(&window, active_cfg.clone());

    // Theme dropdown in settings
    {
        let model = model.clone();
        let win = window.clone();
        let active_cfg_theme = active_cfg.clone();
        settings_widgets
            .theme_dropdown
            .connect_selected_notify(move |dd| {
                let mode = if dd.selected() == 1 {
                    ThemeMode::White
                } else {
                    ThemeMode::Dark
                };
                active_cfg_theme.borrow_mut().theme = mode.as_str().to_string();
                model.borrow_mut().update(MainWindowMsg::SetTheme(mode));
                win.set_icon_name(Some(mode.icon_name()));
                if mode.is_dark() {
                    win.remove_css_class("white-theme");
                    win.add_css_class("dark-theme");
                } else {
                    win.remove_css_class("dark-theme");
                    win.add_css_class("white-theme");
                }
            });
    }

    // Language change callback in settings
    {
        let model = model.clone();
        let hw = header_widgets.clone();
        let sw = settings_widgets.clone();
        let active_cfg = active_cfg.clone();
        let win = window.clone();
        header_widgets.set_on_language_change(move |code| {
            hw.retranslate(&code);
            sw.retranslate(&code);
            win.set_title(Some(crate::services::i18n::tr("settings", &code)));
            active_cfg.borrow_mut().ui_language = code.clone();
            model.borrow_mut().set_ui_language(&code);
        });
    }

    // MiniBar zoom scale change callback in settings
    {
        let model = model.clone();
        let active_cfg = active_cfg.clone();
        header_widgets.set_on_scale_change(move |scale| {
            crate::ui::mini_bar::apply_minibar_scale_css(scale);
            active_cfg.borrow_mut().minibar_scale = scale;
            model.borrow_mut().set_minibar_scale(scale);
        });
    }

    // Save button & live auto-sync in settings
    {
        let sync_standalone = {
            let model = model.clone();
            let sw = settings_widgets.clone();
            let active_cfg_for_save = active_cfg.clone();
            move || {
                let current_cfg = active_cfg_for_save.borrow().clone();
                let extracted = settings_view::extract_config_from_widgets(&sw, &current_cfg);
                *active_cfg_for_save.borrow_mut() = extracted.clone();
                *model.borrow_mut().config_mut() = extracted;
                model.borrow_mut().update(MainWindowMsg::SaveSettings);
            }
        };
        let sync_btn = sync_standalone.clone();
        settings_widgets.save_button.connect_clicked(move |_| {
            sync_btn();
        });
        settings_widgets.set_on_config_auto_sync(move || {
            sync_standalone();
        });
    }

    window
}

/// Attaches a GTK4 `ShortcutController` (`ShortcutScope::Global`) to a window so the user can
/// toggle recording with their configured shortcut (default `<Control><Alt>d`) or cancel an
/// active recording with `Escape`.
fn attach_window_shortcuts(
    window: &gtk4::Window,
    hotkey_raw: &str,
    record_button: &gtk4::Button,
    cancel_button: &gtk4::Button,
) {
    let controller = gtk4::ShortcutController::new();
    controller.set_scope(gtk4::ShortcutScope::Global);

    let accel = crate::services::hotkey::HotkeyService::to_gtk_accelerator(hotkey_raw)
        .unwrap_or_else(|_| "<Control><Alt>d".to_string());
    if let Some(trigger) = gtk4::ShortcutTrigger::parse_string(&accel) {
        let rec_btn = record_button.clone();
        let action = gtk4::CallbackAction::new(move |_, _| {
            if rec_btn.is_sensitive() {
                rec_btn.emit_clicked();
            }
            gtk4::glib::Propagation::Stop
        });
        controller.add_shortcut(gtk4::Shortcut::new(Some(trigger), Some(action)));
    }

    if let Some(esc_trigger) = gtk4::ShortcutTrigger::parse_string("Escape") {
        let cancel_btn = cancel_button.clone();
        let action = gtk4::CallbackAction::new(move |_, _| {
            if cancel_btn.is_sensitive() {
                cancel_btn.emit_clicked();
                gtk4::glib::Propagation::Stop
            } else {
                gtk4::glib::Propagation::Proceed
            }
        });
        controller.add_shortcut(gtk4::Shortcut::new(Some(esc_trigger), Some(action)));
    }

    window.add_controller(controller);
}

/// Helper function to configure Main Menu actions on an ApplicationWindow.
fn setup_main_menu_actions(
    window: &libadwaita::ApplicationWindow,
    active_cfg: std::rc::Rc<std::cell::RefCell<Config>>,
) {
    let action_group = gio::SimpleActionGroup::new();

    // app.report_issue
    let act_report = gio::SimpleAction::new("report_issue", None);
    let win_weak = window.downgrade();
    act_report.connect_activate(move |_, _| {
        let parent_win = win_weak.upgrade().map(|w| w.upcast::<gtk4::Window>());
        header::open_external_url(
            "https://github.com/hkumarsaikia/OpenDictate/issues",
            parent_win.as_ref(),
        );
    });
    action_group.add_action(&act_report);

    // app.troubleshooting
    let act_trouble = gio::SimpleAction::new("troubleshooting", None);
    let root_trouble = window.clone();
    let cfg_trouble = active_cfg.clone();
    act_trouble.connect_activate(move |_, _| {
        let current_cfg = cfg_trouble.borrow().clone();
        header::show_troubleshooting_dialog(&root_trouble, &current_cfg);
    });
    action_group.add_action(&act_trouble);

    // app.credits
    let act_credits = gio::SimpleAction::new("credits", None);
    let root_credits = window.clone();
    act_credits.connect_activate(move |_, _| {
        header::show_credits_dialog(&root_credits);
    });
    action_group.add_action(&act_credits);

    // app.legal
    let act_legal = gio::SimpleAction::new("legal", None);
    let root_legal = window.clone();
    act_legal.connect_activate(move |_, _| {
        header::show_legal_dialog(&root_legal);
    });
    action_group.add_action(&act_legal);

    // app.about
    let act_about = gio::SimpleAction::new("about", None);
    let root_about = window.clone();
    let cfg_about = active_cfg.clone();
    act_about.connect_activate(move |_, _| {
        let current_cfg = cfg_about.borrow().clone();
        let about = header::build_about_dialog(&current_cfg);
        about.present(Some(&root_about));
    });
    action_group.add_action(&act_about);

    window.insert_action_group("app", Some(&action_group));
    window.insert_action_group("win", Some(&action_group));
}
