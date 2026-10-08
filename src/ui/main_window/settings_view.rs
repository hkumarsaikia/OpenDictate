//! Settings view component for OpenDictate MainWindow using Libadwaita PreferencesPage.

use crate::audio::recorder::AudioRecorder;
use crate::config::Config;
use crate::services::ai::ModelInfo;
use crate::services::ai::local_ai::{
    download_local_model, get_local_model_catalog, is_model_installed, resolve_model_path,
};
use gtk4::gio::prelude::*;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub const AI_PROVIDERS: &[&str] = &[
    "groq",
    "openai",
    "claude",
    "gemini",
    "mistral",
    "openrouter",
    "cloudflare",
    "nvidia",
    "cohere",
    "kilocode",
    "cerebras",
    "opencode",
    "huggingface",
    "ollama",
];

pub const AI_PROVIDER_LABELS: &[&str] = &[
    "Groq",
    "OpenAI",
    "Anthropic Claude",
    "Google Gemini",
    "Mistral AI",
    "OpenRouter",
    "Cloudflare Workers AI",
    "NVIDIA NIM",
    "Cohere",
    "Kilo Code",
    "Cerebras",
    "OpenCode",
    "Hugging Face",
    "Ollama",
];

/// Type alias for the model selection warning callback closure.
pub type ModelWarningFn = Rc<dyn Fn(Option<String>)>;

/// Wrapper for model warning change callback supporting Debug.
#[derive(Clone, Default)]
pub struct ModelWarningCallback(pub Rc<RefCell<Option<ModelWarningFn>>>);

impl std::fmt::Debug for ModelWarningCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelWarningCallback")
            .field("has_callback", &self.0.borrow().is_some())
            .finish()
    }
}

/// Type alias for live configuration auto-sync callback closure.
pub type ConfigSyncFn = Rc<dyn Fn()>;

/// Wrapper for live configuration auto-sync callback supporting Debug.
#[derive(Clone, Default)]
pub struct ConfigSyncCallback(pub Rc<RefCell<Option<ConfigSyncFn>>>);

impl std::fmt::Debug for ConfigSyncCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigSyncCallback")
            .field("has_callback", &self.0.borrow().is_some())
            .finish()
    }
}

/// Widgets composing the settings view tab.
#[derive(Clone, Debug)]
pub struct SettingsViewWidgets {
    pub page: libadwaita::PreferencesPage,
    pub appearance_group: libadwaita::PreferencesGroup,
    pub cloud_group: libadwaita::PreferencesGroup,
    pub local_group: libadwaita::PreferencesGroup,
    pub general_group: libadwaita::PreferencesGroup,
    pub theme_dropdown: libadwaita::ComboRow,
    pub provider_row: libadwaita::ComboRow,
    pub api_key_row: libadwaita::PasswordEntryRow,
    pub model_row: libadwaita::ActionRow,
    pub usage_limit_row: libadwaita::ActionRow,
    pub usage_limit_button: gtk4::Button,
    pub usage_limit_popover: gtk4::Popover,
    pub usage_limit_components: UsageLimitComponents,
    pub audio_device_row: libadwaita::ComboRow,
    pub hotkey_row: libadwaita::EntryRow,
    pub save_button: gtk4::Button,
    pub audio_devices: Vec<String>,
    pub current_models: Rc<RefCell<Vec<ModelInfo>>>,
    pub selected_model_id: Rc<RefCell<String>>,
    pub picker_components: ModelPickerComponents,
    pub cloud_switch: gtk4::Switch,
    pub local_switch: gtk4::Switch,
    pub local_model_row: libadwaita::ActionRow,
    pub local_picker_components: LocalModelPickerComponents,
    pub selected_local_model_id: Rc<RefCell<String>>,
    pub model_status_row: libadwaita::ActionRow,
    pub download_button: gtk4::Button,
    pub delete_button: gtk4::Button,
    pub download_progress_row: libadwaita::ActionRow,
    pub download_progress_bar: gtk4::ProgressBar,
    pub custom_file_row: libadwaita::ActionRow,
    pub threads_row: libadwaita::ComboRow,
    pub custom_threads_row: libadwaita::ActionRow,
    pub custom_threads_spin: gtk4::SpinButton,
    pub custom_path: Rc<RefCell<Option<String>>>,
    pub warning_tooltip_popover: gtk4::Popover,
    pub warning_tooltip_revealer: gtk4::Revealer,
    pub warning_tooltip_label: gtk4::Label,
    pub tooltip_generation: Rc<RefCell<u64>>,
    pub active_model_warning: Rc<RefCell<Option<String>>>,
    pub on_model_warning_change: ModelWarningCallback,
    pub on_config_auto_sync: ConfigSyncCallback,
    pub last_detected_audio_profile: Rc<RefCell<String>>,
}

fn show_warning_tooltip_with_revealer(
    popover: &gtk4::Popover,
    revealer: &gtk4::Revealer,
    label: &gtk4::Label,
    generation: &Rc<RefCell<u64>>,
    msg: &str,
) {
    label.set_text(msg);
    revealer.set_reveal_child(true);
    popover.popup();

    let popover_clone = popover.clone();
    let revealer_clone = revealer.clone();
    let gen_cell = generation.clone();
    let current_gen = {
        let mut g = gen_cell.borrow_mut();
        *g = g.wrapping_add(1);
        *g
    };

    gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(2500), move || {
        if *gen_cell.borrow() == current_gen {
            revealer_clone.set_reveal_child(false);
            let gen_inner = gen_cell.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(220), move || {
                if *gen_inner.borrow() == current_gen {
                    popover_clone.popdown();
                }
            });
        }
    });
}

impl SettingsViewWidgets {
    pub fn select_local_model_index(&self, index: usize) {
        if let Some(row) = self
            .local_picker_components
            .list_box
            .row_at_index(index as i32)
        {
            self.local_picker_components.list_box.select_row(Some(&row));
            self.local_picker_components
                .list_box
                .emit_by_name::<()>("row-activated", &[&row]);
        }
    }

    /// Dynamically retranslates all settings view group titles, row labels, and buttons in-place.
    pub fn retranslate(&self, lang: &str) {
        use crate::services::i18n::tr;
        self.page.set_title(tr("settings", lang));
        self.appearance_group.set_title(tr("appearance", lang));
        self.cloud_group.set_title(tr("cloud_ai", lang));
        self.local_group.set_title(tr("local_ai", lang));
        self.general_group.set_title(tr("general", lang));

        self.theme_dropdown.set_title(tr("theme", lang));
        self.provider_row.set_title(tr("ai_provider", lang));
        self.api_key_row.set_title(tr("api_key", lang));
        self.model_row.set_title(tr("ai_model", lang));
        self.usage_limit_row.set_title(tr("usage_limit", lang));
        self.local_model_row.set_title(tr("whisper_model", lang));
        self.threads_row.set_title(tr("threads", lang));
        self.audio_device_row.set_title(tr("microphone", lang));
        self.hotkey_row.set_title(tr("global_shortcut", lang));
        self.save_button.set_label(tr("save", lang));
    }

    /// Shows a compact auto-dismissing warning tooltip for 2.5 seconds with smooth fade-in/out.
    pub fn show_warning_tooltip(&self, msg: &str) {
        show_warning_tooltip_with_revealer(
            &self.warning_tooltip_popover,
            &self.warning_tooltip_revealer,
            &self.warning_tooltip_label,
            &self.tooltip_generation,
            msg,
        );
    }

    /// Registers a callback invoked whenever the active Cloud AI model warning changes.
    pub fn set_on_model_warning_change<F: Fn(Option<String>) + 'static>(&self, callback: F) {
        *self.on_model_warning_change.0.borrow_mut() = Some(Rc::new(callback));
    }

    /// Registers a callback invoked whenever any setting control changes in-place so the live worker
    /// and persisted config stay synchronized even if the user does not click Save.
    pub fn set_on_config_auto_sync<F: Fn() + 'static>(&self, callback: F) {
        *self.on_config_auto_sync.0.borrow_mut() = Some(Rc::new(callback));
    }

    /// Applies a newly detected hardware audio profile ("System Default", "Headphones", or "Handsfree")
    /// to the Microphone dropdown whenever the hardware connection state transitions.
    /// Users can freely override this selection manually between hardware state changes.
    pub fn apply_detected_audio_profile(&self, detected_profile: &str) {
        let mut last = self.last_detected_audio_profile.borrow_mut();
        if last.as_str() != detected_profile {
            *last = detected_profile.to_string();
            if let Some(idx) = self
                .audio_devices
                .iter()
                .position(|d| d == detected_profile)
            {
                self.audio_device_row.set_selected(idx as u32);
            }
        }
    }
}

/// Updates the Usage Limit row and button activation state.
/// The row and button are only active when both a non-empty API key and a selected model are present.
pub fn update_usage_limit_state(
    row: &libadwaita::ActionRow,
    button: &gtk4::Button,
    popover: &gtk4::Popover,
    api_key: &str,
    model_id: &str,
) {
    let active = !api_key.trim().is_empty() && !model_id.trim().is_empty();
    row.set_activatable(active);
    button.set_sensitive(active);
    button.set_visible(active);
    if !active {
        popover.popdown();
    }
}

/// Components composing the Cloud AI Usage Limit popover window.
#[derive(Clone, Debug)]
pub struct UsageLimitComponents {
    pub popover: gtk4::Popover,
    pub refresh_button: gtk4::Button,
    pub summary_label: gtk4::Label,
    pub dropdown_button: gtk4::Button,
    pub val_provider: gtk4::Label,
    pub val_model: gtk4::Label,
    pub val_key_status: gtk4::Label,
    pub val_rpm: gtk4::Label,
    pub val_rpd: gtk4::Label,
    pub val_tpm: gtk4::Label,
    pub val_context: gtk4::Label,
    pub val_remaining: gtk4::Label,
}

fn create_usage_limit_row(label: &str) -> (gtk4::Box, gtk4::Label) {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.set_margin_top(2);
    row.set_margin_bottom(2);

    let key = gtk4::Label::new(Some(label));
    key.add_css_class("dim-label");
    key.add_css_class("caption");
    key.set_halign(gtk4::Align::Start);

    let val = gtk4::Label::new(None);
    val.add_css_class("caption");
    val.add_css_class("numeric");
    val.set_halign(gtk4::Align::End);
    val.set_hexpand(true);
    val.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    val.set_max_width_chars(16);

    row.append(&key);
    row.append(&val);
    (row, val)
}

fn build_usage_limit_popover(
    parent: &libadwaita::ActionRow,
    dropdown_button: &gtk4::Button,
    summary_label: &gtk4::Label,
) -> UsageLimitComponents {
    let popover = gtk4::Popover::new();
    popover.set_parent(parent);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.add_css_class("opencode-model-popover");

    let card_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    card_box.add_css_class("model-detail-card");
    card_box.set_width_request(210);
    card_box.set_margin_start(6);
    card_box.set_margin_end(6);
    card_box.set_margin_top(4);
    card_box.set_margin_bottom(4);

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let title_lbl = gtk4::Label::new(Some("Model Usage Limits"));
    title_lbl.add_css_class("caption");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_hexpand(true);

    let refresh_button = gtk4::Button::from_icon_name("view-refresh-symbolic");
    refresh_button.add_css_class("flat");
    refresh_button.set_valign(gtk4::Align::Center);
    refresh_button.set_tooltip_text(Some("Refresh usage limits from API"));

    header_box.append(&title_lbl);
    header_box.append(&refresh_button);
    card_box.append(&header_box);

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    card_box.append(&sep);

    let (_r_prov, val_provider) = create_usage_limit_row("Provider");
    let (_r_model, val_model) = create_usage_limit_row("Selected Model");
    let (_r_status, val_key_status) = create_usage_limit_row("API Key Status");
    let (r_rpm, val_rpm) = create_usage_limit_row("Requests / Min (RPM)");
    let (r_rpd, val_rpd) = create_usage_limit_row("Requests / Day (RPD)");
    let (r_tpm, val_tpm) = create_usage_limit_row("Tokens / Min (TPM)");
    let (_r_ctx, val_context) = create_usage_limit_row("Context / Audio Limit");
    let (_r_rem, val_remaining) = create_usage_limit_row("Remaining Quota");

    // Display only the 3 most important rate limit options in the compact card
    card_box.append(&r_rpm);
    card_box.append(&r_rpd);
    card_box.append(&r_tpm);

    popover.set_child(Some(&card_box));

    UsageLimitComponents {
        popover,
        refresh_button,
        summary_label: summary_label.clone(),
        dropdown_button: dropdown_button.clone(),
        val_provider,
        val_model,
        val_key_status,
        val_rpm,
        val_rpd,
        val_tpm,
        val_context,
        val_remaining,
    }
}

pub fn apply_usage_limit_info(
    comp: &UsageLimitComponents,
    info: &crate::services::ai::ModelUsageLimitInfo,
) {
    comp.val_provider.set_text(&info.provider);
    comp.val_model.set_text(&info.model_id);
    comp.val_key_status.set_text(&info.key_status);
    comp.val_rpm.set_text(&info.requests_per_minute);
    comp.val_rpd.set_text(&info.requests_per_day);
    comp.val_tpm.set_text(&info.tokens_per_minute);
    comp.val_context.set_text(&info.context_or_audio_limit);
    comp.val_remaining.set_text(&info.remaining_quota);
    comp.summary_label.set_text("");
    comp.summary_label.set_visible(false);
}

pub fn trigger_usage_limit_refresh(
    comp: &UsageLimitComponents,
    provider: &str,
    api_key: &str,
    model_id: &str,
) {
    let has_key = !api_key.trim().is_empty() || provider == "ollama";
    let initial = crate::services::ai::default_model_usage_limits(provider, model_id, has_key);
    apply_usage_limit_info(comp, &initial);

    if !has_key {
        return;
    }

    let comp_clone = comp.clone();
    let prov = provider.to_string();
    let key = api_key.to_string();
    let mid = model_id.to_string();

    let (tx, rx) = tokio::sync::oneshot::channel();
    crate::services::dictation_worker::tokio_handle().spawn(async move {
        let res = crate::services::ai::fetch_model_usage_limits(&prov, &key, &mid).await;
        let _ = tx.send(res);
    });

    gtk4::glib::spawn_future_local(async move {
        if let Ok(Ok(live_info)) = rx.await {
            apply_usage_limit_info(&comp_clone, &live_info);
        }
    });
}

/// Components composing the OpenCode-style model picker popover.
#[derive(Clone, Debug)]
pub struct ModelPickerComponents {
    pub popover: gtk4::Popover,
    pub detail_popover: gtk4::Popover,
    pub left_box: gtk4::Box,
    pub provider_label: gtk4::Label,
    pub list_box: gtk4::ListBox,
    pub search_entry: gtk4::SearchEntry,
    pub query_filter: Rc<RefCell<String>>,
    pub card_box: gtk4::Box,
    pub dropdown_button: gtk4::Button,
    pub status_label: gtk4::Label,
    pub val_model: gtk4::Label,
    pub val_prov: gtk4::Label,
    pub val_inputs: gtk4::Label,
    pub val_reason: gtk4::Label,
    pub val_context: gtk4::Label,
}

fn create_card_row(label: &str) -> (gtk4::Box, gtk4::Label) {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.set_margin_top(1);
    row.set_margin_bottom(1);

    let key = gtk4::Label::new(Some(label));
    key.add_css_class("dim-label");
    key.add_css_class("caption");
    key.set_halign(gtk4::Align::Start);
    key.set_width_request(60);

    let val = gtk4::Label::new(None);
    val.add_css_class("caption");
    val.add_css_class("bold");
    val.set_halign(gtk4::Align::End);
    val.set_hexpand(true);
    val.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    row.append(&key);
    row.append(&val);
    (row, val)
}

fn build_model_picker(
    parent: &libadwaita::ActionRow,
    dropdown_button: &gtk4::Button,
    status_label: &gtk4::Label,
) -> ModelPickerComponents {
    let popover = gtk4::Popover::new();
    popover.set_parent(parent);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.add_css_class("opencode-model-popover");

    // Search + Model List container
    let left_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    left_box.set_width_request(290);
    left_box.set_margin_start(4);
    left_box.set_margin_end(4);
    left_box.set_margin_top(4);
    left_box.set_margin_bottom(4);

    let search_entry = gtk4::SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search models"));
    left_box.append(&search_entry);

    let provider_label = gtk4::Label::new(None);
    provider_label.set_halign(gtk4::Align::Start);
    provider_label.add_css_class("dim-label");
    provider_label.add_css_class("caption");
    provider_label.set_margin_start(4);
    provider_label.set_margin_top(2);
    left_box.append(&provider_label);

    let scrolled_window = gtk4::ScrolledWindow::new();
    scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_window.set_propagate_natural_height(true);
    scrolled_window.set_max_content_height(340);
    scrolled_window.set_min_content_height(200);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);
    list_box.add_css_class("model-list-box");
    scrolled_window.set_child(Some(&list_box));

    left_box.append(&scrolled_window);

    // Main popover contains ONLY the search + model list (completely separate from the card)
    popover.set_child(Some(&left_box));

    // Detail Popover: Completely separate floating popover positioned to the right
    let detail_popover = gtk4::Popover::new();
    detail_popover.set_parent(&left_box);
    detail_popover.set_position(gtk4::PositionType::Right);
    detail_popover.set_autohide(false);
    detail_popover.add_css_class("model-detail-popover");

    let card_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    card_box.add_css_class("model-detail-card");
    card_box.set_width_request(200);

    let (row_model, val_model) = create_card_row("Model");
    let (row_prov, val_prov) = create_card_row("Provider");
    let (row_inputs, val_inputs) = create_card_row("Inputs");
    let (row_reason, val_reason) = create_card_row("Reasoning");
    let (row_context, val_context) = create_card_row("Context");

    card_box.append(&row_model);
    card_box.append(&row_prov);
    card_box.append(&row_inputs);
    card_box.append(&row_reason);
    card_box.append(&row_context);

    detail_popover.set_child(Some(&card_box));

    let query_filter = Rc::new(RefCell::new(String::new()));

    // Hide detail popover on main popover close
    let detail_pop_closed = detail_popover.clone();
    popover.connect_closed(move |_| {
        detail_pop_closed.popdown();
    });

    // Hide detail popover when cursor leaves list_box entirely
    let detail_pop_lb_leave = detail_popover.clone();
    let lb_motion = gtk4::EventControllerMotion::new();
    lb_motion.connect_leave(move |_| {
        detail_pop_lb_leave.popdown();
    });
    list_box.add_controller(lb_motion);

    // Hide detail popover when hovering search entry
    let detail_pop_search = detail_popover.clone();
    let search_motion = gtk4::EventControllerMotion::new();
    search_motion.connect_enter(move |_, _, _| {
        detail_pop_search.popdown();
    });
    search_entry.add_controller(search_motion);

    ModelPickerComponents {
        popover,
        detail_popover,
        left_box,
        provider_label,
        list_box,
        search_entry,
        query_filter,
        card_box,
        dropdown_button: dropdown_button.clone(),
        status_label: status_label.clone(),
        val_model,
        val_prov,
        val_inputs,
        val_reason,
        val_context,
    }
}

pub fn update_hover_card(components: &ModelPickerComponents, m: &ModelInfo) {
    components.val_model.set_text(&m.card_model_title());
    components.val_prov.set_text(&m.provider);
    components.val_inputs.set_text(&m.inputs_label());
    components.val_reason.set_text(m.reasoning_label());
    components.val_context.set_text(&m.formatted_context());
}

/// Components composing the OpenCode-style local model picker popover.
#[derive(Clone, Debug)]
pub struct LocalModelPickerComponents {
    pub popover: gtk4::Popover,
    pub detail_popover: gtk4::Popover,
    pub left_box: gtk4::Box,
    pub list_box: gtk4::ListBox,
    pub dropdown_button: gtk4::Button,
    pub status_label: gtk4::Label,
    pub card_box: gtk4::Box,
    pub val_model: gtk4::Label,
    pub val_params: gtk4::Label,
    pub val_langs: gtk4::Label,
    pub val_speed: gtk4::Label,
}

impl LocalModelPickerComponents {
    pub fn select_model_by_id(&self, model_id: &str) {
        let catalog = get_local_model_catalog();
        if model_id == "custom" {
            let last_idx = catalog.len();
            if let Some(row) = self.list_box.row_at_index(last_idx as i32) {
                self.list_box.select_row(Some(&row));
                self.list_box.emit_by_name::<()>("row-activated", &[&row]);
            }
        } else if let Some(idx) = catalog.iter().position(|m| m.id == model_id)
            && let Some(row) = self.list_box.row_at_index(idx as i32)
        {
            self.list_box.select_row(Some(&row));
            self.list_box.emit_by_name::<()>("row-activated", &[&row]);
        }
    }
}

fn build_local_model_picker(
    parent: &libadwaita::ActionRow,
    dropdown_button: &gtk4::Button,
    status_label: &gtk4::Label,
) -> LocalModelPickerComponents {
    let popover = gtk4::Popover::new();
    popover.set_parent(parent);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.add_css_class("opencode-model-popover");

    let left_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    left_box.set_width_request(260);
    left_box.set_margin_start(4);
    left_box.set_margin_end(4);
    left_box.set_margin_top(4);
    left_box.set_margin_bottom(4);

    let scrolled_window = gtk4::ScrolledWindow::new();
    scrolled_window.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled_window.set_propagate_natural_height(true);
    scrolled_window.set_max_content_height(340);
    scrolled_window.set_min_content_height(180);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);
    list_box.add_css_class("model-list-box");
    scrolled_window.set_child(Some(&list_box));

    left_box.append(&scrolled_window);
    popover.set_child(Some(&left_box));

    // Detail Popover: Floating popover positioned to the right
    let detail_popover = gtk4::Popover::new();
    detail_popover.set_parent(&left_box);
    detail_popover.set_position(gtk4::PositionType::Right);
    detail_popover.set_autohide(false);
    detail_popover.add_css_class("model-detail-popover");

    let card_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    card_box.add_css_class("model-detail-card");
    card_box.add_css_class("local-model-detail-card");
    card_box.set_width_request(200);

    let (row_model, val_model) = create_card_row("Model");
    let (row_params, val_params) = create_card_row("Parameters");
    let (row_langs, val_langs) = create_card_row("Languages");
    let (row_speed, val_speed) = create_card_row("Speed");

    card_box.append(&row_model);
    card_box.append(&row_params);
    card_box.append(&row_langs);
    card_box.append(&row_speed);

    detail_popover.set_child(Some(&card_box));

    let detail_pop_closed = detail_popover.clone();
    popover.connect_closed(move |_| {
        detail_pop_closed.popdown();
    });

    let detail_pop_lb_leave = detail_popover.clone();
    let lb_motion = gtk4::EventControllerMotion::new();
    lb_motion.connect_leave(move |_| {
        detail_pop_lb_leave.popdown();
    });
    list_box.add_controller(lb_motion);

    LocalModelPickerComponents {
        popover,
        detail_popover,
        left_box,
        list_box,
        dropdown_button: dropdown_button.clone(),
        status_label: status_label.clone(),
        card_box,
        val_model,
        val_params,
        val_langs,
        val_speed,
    }
}

pub fn update_local_hover_card(
    components: &LocalModelPickerComponents,
    model: Option<&crate::services::ai::local_ai::LocalModelInfo>,
) {
    if let Some(m) = model {
        components.val_model.set_text(&m.label);
        components.val_params.set_text(&m.parameters);
        components.val_langs.set_text(&m.languages);
        components.val_speed.set_text(&m.speed);
    }
}

/// Populates the local Whisper GGML model dropdown list and wires selection handlers.
#[allow(clippy::too_many_arguments)]
pub fn populate_local_model_picker(
    components: &LocalModelPickerComponents,
    selected_id: &Rc<RefCell<String>>,
    custom_path: &Rc<RefCell<Option<String>>>,
    status_label: &gtk4::Label,
    custom_file_row: &libadwaita::ActionRow,
    local_status_label: &gtk4::Label,
    download_button: &gtk4::Button,
    delete_button: &gtk4::Button,
    on_config_auto_sync: &ConfigSyncCallback,
) {
    while let Some(child) = components.list_box.first_child() {
        components.list_box.remove(&child);
    }

    let catalog = get_local_model_catalog();
    let current_mid = selected_id.borrow().clone();

    let initial_display = if current_mid == "custom" {
        "Custom Model (.bin from disk)".to_string()
    } else if let Some(m) = catalog.iter().find(|m| m.id == current_mid) {
        format!("{} ({} MB)", m.label, m.size_mb)
    } else {
        format!("{} ({} MB)", catalog[0].label, catalog[0].size_mb)
    };
    status_label.set_text(&initial_display);

    for m in &catalog {
        let row = gtk4::ListBoxRow::new();
        let h_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        h_box.set_margin_start(8);
        h_box.set_margin_end(8);
        h_box.set_margin_top(6);
        h_box.set_margin_bottom(6);

        let name_label = gtk4::Label::new(Some(&m.label));
        name_label.set_halign(gtk4::Align::Start);
        name_label.set_hexpand(true);
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        h_box.append(&name_label);

        let size_badge = gtk4::Label::new(Some(&format!("{} MB", m.size_mb)));
        size_badge.add_css_class("dim-label");
        size_badge.add_css_class("numeric");
        size_badge.set_halign(gtk4::Align::End);
        size_badge.set_valign(gtk4::Align::Center);
        h_box.append(&size_badge);

        row.set_child(Some(&h_box));

        let motion = gtk4::EventControllerMotion::new();
        let m_clone = m.clone();
        let comp_clone = components.clone();
        let row_weak = row.downgrade();

        motion.connect_enter(move |_controller, _x, _y| {
            update_local_hover_card(&comp_clone, Some(&m_clone));
            if let Some(r) = row_weak.upgrade() {
                #[allow(deprecated)]
                if let Some((_x, y)) = r.translate_coordinates(&comp_clone.left_box, 0.0, 0.0) {
                    let w = comp_clone.left_box.width().max(250);
                    let h = r.height().max(28);
                    let rect = gtk4::gdk::Rectangle::new(w - 2, y as i32, 2, h);
                    comp_clone.detail_popover.set_pointing_to(Some(&rect));
                }
            }
            comp_clone.detail_popover.popup();
        });
        row.add_controller(motion);

        components.list_box.append(&row);
    }

    // Custom model row
    {
        let row = gtk4::ListBoxRow::new();
        let h_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        h_box.set_margin_start(8);
        h_box.set_margin_end(8);
        h_box.set_margin_top(6);
        h_box.set_margin_bottom(6);

        let name_label = gtk4::Label::new(Some("Custom Model (.bin from disk)"));
        name_label.set_halign(gtk4::Align::Start);
        name_label.set_hexpand(true);
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        h_box.append(&name_label);

        row.set_child(Some(&h_box));

        let motion = gtk4::EventControllerMotion::new();
        let comp_clone = components.clone();

        motion.connect_enter(move |_controller, _x, _y| {
            comp_clone.detail_popover.popdown();
        });
        row.add_controller(motion);

        components.list_box.append(&row);
    }

    let comp_clone = components.clone();
    let sel_id_row = selected_id.clone();
    let stat_lbl_row = status_label.clone();
    let custom_row_sel = custom_file_row.clone();
    let loc_stat_sel = local_status_label.clone();
    let dl_btn_sel = download_button.clone();
    let del_btn_sel = delete_button.clone();
    let cp_sel = custom_path.clone();
    let auto_sync_local_row = on_config_auto_sync.clone();

    components.list_box.connect_row_activated(move |_lb, row| {
        let idx = row.index() as usize;
        let catalog = get_local_model_catalog();
        if idx >= catalog.len() {
            *sel_id_row.borrow_mut() = "custom".to_string();
            stat_lbl_row.set_text("Custom Model (.bin from disk)");
            custom_row_sel.set_visible(true);
            update_local_model_status_ui(
                "custom",
                cp_sel.borrow().as_deref(),
                &loc_stat_sel,
                &dl_btn_sel,
                &del_btn_sel,
            );
        } else {
            let m = &catalog[idx];
            *sel_id_row.borrow_mut() = m.id.clone();
            stat_lbl_row.set_text(&format!("{} ({} MB)", m.label, m.size_mb));
            custom_row_sel.set_visible(false);
            update_local_model_status_ui(
                &m.id,
                cp_sel.borrow().as_deref(),
                &loc_stat_sel,
                &dl_btn_sel,
                &del_btn_sel,
            );
        }
        if let Some(cb) = auto_sync_local_row.0.borrow().as_ref() {
            cb();
        }
        comp_clone.popover.popdown();
        comp_clone.detail_popover.popdown();
    });
}

pub fn display_provider_name(provider: &str) -> &'static str {
    match provider.trim().to_lowercase().as_str() {
        "groq" => "Groq",
        "nvidia" => "NVIDIA NIM",
        "openrouter" => "OpenRouter",
        "cloudflare" => "Cloudflare",
        "cerebras" => "Cerebras",
        "cohere" => "Cohere",
        "kilocode" => "Kilo Code",
        "opencode" => "OpenCode Zen",
        "gemini" => "Google Gemini",
        "openai" => "OpenAI",
        "claude" => "Anthropic Claude",
        "mistral" => "Mistral AI",
        "ollama" => "Ollama",
        "huggingface" => "Hugging Face",
        _ => "AI Provider",
    }
}

pub fn validate_provider_key_prefix(provider: &str, key: &str) -> Result<(), String> {
    let p = provider.trim().to_lowercase();
    let k = key.trim();
    if k.is_empty() {
        return Ok(());
    }

    // Check if the key matches a known prefix belonging to ANOTHER provider
    let known_prefixes = [
        ("groq", "gsk_", "Groq"),
        ("nvidia", "nvapi-", "NVIDIA NIM"),
        ("openrouter", "sk-or-", "OpenRouter"),
        ("cerebras", "csk-", "Cerebras"),
        ("huggingface", "hf_", "Hugging Face"),
        ("claude", "sk-ant-", "Anthropic Claude"),
        ("cohere", "cohere_", "Cohere"),
        ("cloudflare", "cfat_", "Cloudflare"),
    ];

    for (prov_id, prefix, prov_name) in known_prefixes {
        if k.starts_with(prefix) && p != prov_id {
            return Err(format!(
                "Invalid key: '{}' is a {} API key",
                prefix, prov_name
            ));
        }
    }

    // Check required prefix for providers that mandate one
    match p.as_str() {
        "nvidia" => {
            if !k.starts_with("nvapi-") {
                return Err("NVIDIA API key must start with 'nvapi-'".to_string());
            }
        }
        "groq" => {
            if !k.starts_with("gsk_") {
                return Err("Groq API key must start with 'gsk_'".to_string());
            }
        }
        "openrouter" => {
            if !k.starts_with("sk-or-") {
                return Err("OpenRouter API key must start with 'sk-or-'".to_string());
            }
        }
        "cerebras" => {
            if !k.starts_with("csk-") {
                return Err("Cerebras API key must start with 'csk-'".to_string());
            }
        }
        "huggingface" => {
            if !k.starts_with("hf_") {
                return Err("Hugging Face API token must start with 'hf_'".to_string());
            }
        }
        "claude" if !k.starts_with("sk-ant-") => {
            return Err("Anthropic Claude API key must start with 'sk-ant-'".to_string());
        }
        _ => {}
    }

    Ok(())
}

pub fn populate_model_picker(
    components: &ModelPickerComponents,
    _model_row: &libadwaita::ActionRow,
    models: &[ModelInfo],
    provider_name: &str,
    selected_id: &Rc<RefCell<String>>,
) {
    components.provider_label.set_text(provider_name);
    components.detail_popover.popdown();

    // Clear existing rows
    while let Some(child) = components.list_box.first_child() {
        components.list_box.remove(&child);
    }

    if models.is_empty() {
        components.detail_popover.popdown();
        let row = gtk4::ListBoxRow::new();
        row.set_activatable(false);
        row.set_selectable(false);
        let label = gtk4::Label::new(Some("No models available"));
        label.add_css_class("dim-label");
        label.set_margin_start(8);
        label.set_margin_top(8);
        label.set_margin_bottom(8);
        row.set_child(Some(&label));
        components.list_box.append(&row);
        return;
    }

    let current_id = selected_id.borrow().clone();
    let active_model = models
        .iter()
        .find(|m| m.id == current_id)
        .unwrap_or(&models[0]);
    *selected_id.borrow_mut() = active_model.id.clone();
    components
        .status_label
        .set_text(&active_model.display_label());
    components.status_label.remove_css_class("dim-label");

    for m in models {
        let row = gtk4::ListBoxRow::new();
        let h_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        h_box.set_margin_start(8);
        h_box.set_margin_end(8);
        h_box.set_margin_top(6);
        h_box.set_margin_bottom(6);

        let name_label = gtk4::Label::new(Some(&m.name));
        name_label.set_halign(gtk4::Align::Start);
        name_label.set_hexpand(true);
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        h_box.append(&name_label);

        if m.is_free {
            let free_badge = gtk4::Label::new(Some("Free"));
            free_badge.add_css_class("dim-label");
            free_badge.set_valign(gtk4::Align::Center);
            free_badge.set_halign(gtk4::Align::End);
            h_box.append(&free_badge);
        }

        row.set_child(Some(&h_box));

        // Smooth hover motion: updates card and repositions arrow to hovered row;
        // NEVER dismisses on row leave so moving across rows does not blink!
        let motion = gtk4::EventControllerMotion::new();
        let m_clone = m.clone();
        let comp_clone = components.clone();
        let row_weak = row.downgrade();

        motion.connect_enter(move |_controller, _x, _y| {
            update_hover_card(&comp_clone, &m_clone);
            if let Some(r) = row_weak.upgrade() {
                #[allow(deprecated)]
                if let Some((_x, y)) = r.translate_coordinates(&comp_clone.left_box, 0.0, 0.0) {
                    let w = comp_clone.left_box.width().max(280);
                    let h = r.height().max(28);
                    let rect = gtk4::gdk::Rectangle::new(w - 2, y as i32, 2, h);
                    comp_clone.detail_popover.set_pointing_to(Some(&rect));
                }
            }
            comp_clone.detail_popover.popup();
        });

        row.add_controller(motion);
        components.list_box.append(&row);
    }
}

/// Triggers asynchronous fetching of models from the provider using the API key.
pub fn trigger_model_fetch(
    model_row: &libadwaita::ActionRow,
    picker_components: &ModelPickerComponents,
    current_models: Rc<RefCell<Vec<ModelInfo>>>,
    selected_model_id: Rc<RefCell<String>>,
    provider: &str,
    api_key: &str,
    target_model_id: Option<String>,
) {
    let prov = provider.to_string();
    let key = api_key.trim().to_string();

    if key.is_empty() && prov != "ollama" {
        current_models.borrow_mut().clear();
        selected_model_id.borrow_mut().clear();
        picker_components
            .status_label
            .set_text("Paste API Key to fetch models...");
        picker_components.status_label.add_css_class("dim-label");
        model_row.set_activatable(false);
        picker_components.dropdown_button.set_visible(false);
        picker_components.popover.popdown();
        picker_components.detail_popover.popdown();
        populate_model_picker(picker_components, model_row, &[], &prov, &selected_model_id);
        return;
    }

    // Validate prefix
    if let Err(err_msg) = validate_provider_key_prefix(&prov, &key) {
        current_models.borrow_mut().clear();
        selected_model_id.borrow_mut().clear();
        picker_components.status_label.set_text(&err_msg);
        picker_components.status_label.add_css_class("dim-label");
        model_row.set_activatable(false);
        picker_components.dropdown_button.set_visible(false);
        picker_components.popover.popdown();
        picker_components.detail_popover.popdown();
        populate_model_picker(picker_components, model_row, &[], &prov, &selected_model_id);
        return;
    }

    // Key format is valid: show blank while validating or loading
    picker_components.status_label.set_text("");
    picker_components.status_label.remove_css_class("dim-label");
    model_row.set_activatable(true);
    picker_components.dropdown_button.set_visible(true);
    picker_components.detail_popover.popdown();

    let row = model_row.clone();
    let comp = picker_components.clone();
    let models_store = current_models.clone();
    let sel_store = selected_model_id.clone();
    let target = target_model_id.clone();

    gtk4::glib::spawn_future_local(async move {
        match crate::services::ai::fetch_models_for_provider(&prov, &key).await {
            Ok(models) if !models.is_empty() => {
                if let Some(t) = target {
                    if models.iter().any(|m| m.id == t) {
                        *sel_store.borrow_mut() = t;
                    } else {
                        *sel_store.borrow_mut() = models[0].id.clone();
                    }
                } else {
                    *sel_store.borrow_mut() = models[0].id.clone();
                }
                *models_store.borrow_mut() = models.clone();
                row.set_activatable(true);
                comp.dropdown_button.set_visible(true);
                populate_model_picker(&comp, &row, &models, &prov, &sel_store);
            }
            Ok(_) => {
                comp.status_label.set_text("No models available");
                comp.status_label.add_css_class("dim-label");
                row.set_activatable(false);
                comp.dropdown_button.set_visible(false);
                comp.popover.popdown();
                comp.detail_popover.popdown();
                models_store.borrow_mut().clear();
                sel_store.borrow_mut().clear();
                populate_model_picker(&comp, &row, &[], &prov, &sel_store);
            }
            Err(e) => {
                // If API authentication or fetching fails:
                // NEVER populate fallback models! Show clear error and keep models hidden.
                let prov_name = display_provider_name(&prov);
                comp.status_label
                    .set_text(&format!("Invalid API key for {}", prov_name));
                comp.status_label
                    .set_tooltip_text(Some(&format!("Error: {}", e)));
                comp.status_label.add_css_class("dim-label");
                row.set_activatable(false);
                comp.dropdown_button.set_visible(false);
                comp.popover.popdown();
                comp.detail_popover.popdown();
                models_store.borrow_mut().clear();
                sel_store.borrow_mut().clear();
                populate_model_picker(&comp, &row, &[], &prov, &sel_store);
            }
        }
    });
}

/// Updates the local model status row UI: status label, download button, and delete button.
pub fn update_local_model_status_ui(
    model_id: &str,
    custom_path: Option<&str>,
    status_label: &gtk4::Label,
    download_button: &gtk4::Button,
    delete_button: &gtk4::Button,
) {
    if model_id == "custom" {
        download_button.set_visible(false);
        if let Some(cp) = custom_path
            && !cp.trim().is_empty()
            && let Ok(m) = std::fs::metadata(cp)
            && m.is_file()
            && m.len() > 0
        {
            let size_mb = m.len() as f64 / 1_048_576.0;
            status_label.set_text(&format!("Installed ({:.1} MB)", size_mb));
            status_label.remove_css_class("dim-label");
            status_label.add_css_class("success");
            delete_button.set_visible(true);
            return;
        }
        status_label.set_text("Not installed");
        status_label.remove_css_class("success");
        status_label.add_css_class("dim-label");
        delete_button.set_visible(false);
        return;
    }

    download_button.set_visible(true);
    let catalog = get_local_model_catalog();
    let model_info = catalog.iter().find(|m| m.id == model_id);

    let path = resolve_model_path(model_id, custom_path);
    let installed = is_model_installed(model_id, custom_path)
        && std::fs::metadata(&path)
            .map(|m| m.len() > 0)
            .unwrap_or(false);

    if installed {
        let size_str = if let Some(info) = model_info {
            format!("Installed ({} MB)", info.size_mb)
        } else {
            "Installed".to_string()
        };
        status_label.set_text(&size_str);
        status_label.remove_css_class("dim-label");
        status_label.add_css_class("success");
        download_button.set_label("Redownload");
        delete_button.set_visible(true);
    } else {
        status_label.set_text("Not installed");
        status_label.remove_css_class("success");
        status_label.add_css_class("dim-label");
        download_button.set_label("Download");
        delete_button.set_visible(false);
    }
}

/// Constructs the Libadwaita PreferencesPage configured with OpenDictate settings cards.
pub fn build_settings_view(config: &Config) -> (libadwaita::PreferencesPage, SettingsViewWidgets) {
    let lang = if config.ui_language.is_empty() {
        "en"
    } else {
        &config.ui_language
    };

    let page = libadwaita::PreferencesPage::new();
    page.set_title(crate::services::i18n::tr("settings", lang));
    page.set_icon_name(Some("emblem-system-symbolic"));

    // 1. Appearance Card
    let appearance_group = libadwaita::PreferencesGroup::new();
    appearance_group.set_title(crate::services::i18n::tr("appearance", lang));

    let theme_row = libadwaita::ComboRow::new();
    theme_row.set_title(crate::services::i18n::tr("theme", lang));
    let theme_model = gtk4::StringList::new(&["Dark", "White"]);
    theme_row.set_model(Some(&theme_model));
    if config.theme.to_lowercase() == "white" {
        theme_row.set_selected(1);
    } else {
        theme_row.set_selected(0);
    }
    appearance_group.add(&theme_row);
    page.add(&appearance_group);

    // 2. Cloud AI Card
    let cloud_group = libadwaita::PreferencesGroup::new();
    cloud_group.set_title(crate::services::i18n::tr("cloud_ai", lang));
    let cloud_switch = gtk4::Switch::new();
    cloud_switch.set_valign(gtk4::Align::Center);
    cloud_group.set_header_suffix(Some(&cloud_switch));

    let provider_row = libadwaita::ComboRow::new();
    provider_row.set_title(crate::services::i18n::tr("ai_provider", lang));
    let provider_model = gtk4::StringList::new(AI_PROVIDER_LABELS);
    provider_row.set_model(Some(&provider_model));

    let selected_provider =
        if let Some(idx) = AI_PROVIDERS.iter().position(|&p| p == config.ai_provider) {
            provider_row.set_selected(idx as u32);
            AI_PROVIDERS[idx]
        } else {
            provider_row.set_selected(0);
            AI_PROVIDERS[0]
        };
    cloud_group.add(&provider_row);

    let api_key_row = libadwaita::PasswordEntryRow::new();
    api_key_row.set_title(crate::services::i18n::tr("api_key", lang));
    api_key_row.set_text(&config.ai_api_key);
    cloud_group.add(&api_key_row);

    let current_models = Rc::new(RefCell::new(Vec::<ModelInfo>::new()));
    let selected_model_id = Rc::new(RefCell::new(config.ai_model.clone()));

    let model_row = libadwaita::ActionRow::new();
    model_row.set_title(crate::services::i18n::tr("ai_model", lang));
    model_row.set_activatable(false);

    let suffix_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    suffix_box.set_valign(gtk4::Align::Center);

    let status_label = gtk4::Label::new(Some("Paste API Key to fetch models..."));
    status_label.add_css_class("dim-label");
    status_label.set_valign(gtk4::Align::Center);
    status_label.set_halign(gtk4::Align::End);
    status_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    suffix_box.append(&status_label);

    let dropdown_button = gtk4::Button::from_icon_name("pan-down-symbolic");
    dropdown_button.add_css_class("flat");
    dropdown_button.set_valign(gtk4::Align::Center);
    dropdown_button.set_visible(false);
    dropdown_button.set_tooltip_text(Some("Select Model"));
    suffix_box.append(&dropdown_button);

    model_row.add_suffix(&suffix_box);

    let picker_components = build_model_picker(&model_row, &dropdown_button, &status_label);

    // Wire opening popover on dropdown_button click
    let popover_clone_btn = picker_components.popover.clone();
    let search_entry_clone_btn = picker_components.search_entry.clone();
    let detail_pop_btn = picker_components.detail_popover.clone();
    dropdown_button.connect_clicked(move |_| {
        detail_pop_btn.popdown();
        popover_clone_btn.popup();
        search_entry_clone_btn.grab_focus();
    });

    // Wire model selection on click in listbox
    let comp_clone = picker_components.clone();
    let current_models_clone = current_models.clone();
    let selected_model_id_clone = selected_model_id.clone();
    picker_components
        .list_box
        .connect_row_activated(move |_lb, row| {
            let idx = row.index() as usize;
            if let Some(m) = current_models_clone.borrow().get(idx) {
                *selected_model_id_clone.borrow_mut() = m.id.clone();
                comp_clone.status_label.set_text(&m.display_label());
                comp_clone.status_label.remove_css_class("dim-label");

                comp_clone.detail_popover.popdown();
                comp_clone.popover.popdown();
            }
        });

    // Wire search filtering
    let query_filter = picker_components.query_filter.clone();
    let models_ref = current_models.clone();
    picker_components.list_box.set_filter_func(move |row| {
        let q = query_filter.borrow().to_lowercase();
        if q.is_empty() {
            return true;
        }
        let idx = row.index() as usize;
        if let Some(m) = models_ref.borrow().get(idx) {
            m.name.to_lowercase().contains(&q) || m.id.to_lowercase().contains(&q)
        } else {
            true
        }
    });

    let q_filter_clone = picker_components.query_filter.clone();
    let list_box_clone = picker_components.list_box.clone();
    let detail_pop_search_changed = picker_components.detail_popover.clone();
    picker_components
        .search_entry
        .connect_search_changed(move |entry| {
            detail_pop_search_changed.popdown();
            *q_filter_clone.borrow_mut() = entry.text().to_string();
            list_box_clone.invalidate_filter();
        });

    // Wire opening popover on row activation
    let popover_clone = picker_components.popover.clone();
    let search_entry_clone = picker_components.search_entry.clone();
    let detail_pop_row_act = picker_components.detail_popover.clone();
    model_row.connect_activated(move |_| {
        detail_pop_row_act.popdown();
        popover_clone.popup();
        search_entry_clone.grab_focus();
    });

    // Initial trigger
    trigger_model_fetch(
        &model_row,
        &picker_components,
        current_models.clone(),
        selected_model_id.clone(),
        selected_provider,
        &config.ai_api_key,
        Some(config.ai_model.clone()),
    );

    cloud_group.add(&model_row);

    // Usage Limit Row with right-side downward arrow button
    let usage_limit_row = libadwaita::ActionRow::new();
    usage_limit_row.set_title(crate::services::i18n::tr("usage_limit", lang));
    usage_limit_row.set_activatable(true);

    let usage_limit_suffix_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    usage_limit_suffix_box.set_valign(gtk4::Align::Center);

    let usage_limit_summary_label = gtk4::Label::new(None);
    usage_limit_summary_label.set_visible(false);

    let usage_limit_button = gtk4::Button::from_icon_name("pan-down-symbolic");
    usage_limit_button.add_css_class("flat");
    usage_limit_button.set_valign(gtk4::Align::Center);
    usage_limit_button.set_tooltip_text(Some("Show Usage Limits"));
    usage_limit_suffix_box.append(&usage_limit_button);

    usage_limit_row.add_suffix(&usage_limit_suffix_box);

    let usage_limit_components = build_usage_limit_popover(
        &usage_limit_row,
        &usage_limit_button,
        &usage_limit_summary_label,
    );
    let usage_limit_popover = usage_limit_components.popover.clone();

    // Initial usage limit populate
    trigger_usage_limit_refresh(
        &usage_limit_components,
        selected_provider,
        &config.ai_api_key,
        &config.ai_model,
    );

    // Wire opening usage limit popover on downward arrow button click
    {
        let ul_comp = usage_limit_components.clone();
        let prov_row_ul = provider_row.clone();
        let key_row_ul = api_key_row.clone();
        let sel_mod_ul = selected_model_id.clone();
        usage_limit_button.connect_clicked(move |_| {
            let idx = prov_row_ul.selected() as usize;
            let prov = AI_PROVIDERS.get(idx).copied().unwrap_or("groq");
            let key = key_row_ul.text().to_string();
            let mid = sel_mod_ul.borrow().clone();
            trigger_usage_limit_refresh(&ul_comp, prov, &key, &mid);
            ul_comp.popover.popup();
        });
    }

    // Wire opening usage limit popover on row activation
    {
        let ul_comp = usage_limit_components.clone();
        let prov_row_ul = provider_row.clone();
        let key_row_ul = api_key_row.clone();
        let sel_mod_ul = selected_model_id.clone();
        usage_limit_row.connect_activated(move |_| {
            let idx = prov_row_ul.selected() as usize;
            let prov = AI_PROVIDERS.get(idx).copied().unwrap_or("groq");
            let key = key_row_ul.text().to_string();
            let mid = sel_mod_ul.borrow().clone();
            trigger_usage_limit_refresh(&ul_comp, prov, &key, &mid);
            ul_comp.popover.popup();
        });
    }

    // Wire Refresh button inside the usage limit popover
    {
        let ul_comp = usage_limit_components.clone();
        let prov_row_ul = provider_row.clone();
        let key_row_ul = api_key_row.clone();
        let sel_mod_ul = selected_model_id.clone();
        usage_limit_components
            .refresh_button
            .connect_clicked(move |_| {
                let idx = prov_row_ul.selected() as usize;
                let prov = AI_PROVIDERS.get(idx).copied().unwrap_or("groq");
                let key = key_row_ul.text().to_string();
                let mid = sel_mod_ul.borrow().clone();
                trigger_usage_limit_refresh(&ul_comp, prov, &key, &mid);
            });
    }

    // Warning tooltip popover for model selection warnings (audio unsupported, paid plan required)
    let warning_tooltip_label = gtk4::Label::new(None);
    warning_tooltip_label.set_wrap(true);
    warning_tooltip_label.set_max_width_chars(42);
    warning_tooltip_label.set_xalign(0.0);

    let warning_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    warning_card.add_css_class("warning-tooltip-card");
    let warning_icon = gtk4::Image::from_icon_name("dialog-warning-symbolic");
    warning_card.append(&warning_icon);
    warning_card.append(&warning_tooltip_label);

    let warning_tooltip_revealer = gtk4::Revealer::new();
    warning_tooltip_revealer.set_transition_type(gtk4::RevealerTransitionType::Crossfade);
    warning_tooltip_revealer.set_transition_duration(220);
    warning_tooltip_revealer.set_reveal_child(false);
    warning_tooltip_revealer.set_child(Some(&warning_card));

    let warning_tooltip_popover = gtk4::Popover::new();
    warning_tooltip_popover.set_parent(&model_row);
    warning_tooltip_popover.set_position(gtk4::PositionType::Bottom);
    warning_tooltip_popover.set_autohide(false);
    warning_tooltip_popover.set_can_focus(false);
    warning_tooltip_popover.add_css_class("warning-tooltip-popover");
    warning_tooltip_popover.set_child(Some(&warning_tooltip_revealer));

    let tooltip_generation = Rc::new(RefCell::new(0u64));
    let active_model_warning = Rc::new(RefCell::new(None::<String>));
    let on_model_warning_change = ModelWarningCallback::default();
    let on_config_auto_sync = ConfigSyncCallback::default();

    // Refresh usage limits and check live model capabilities/paid plan when a model is selected
    {
        let ul_comp = usage_limit_components.clone();
        let ul_row_sel = usage_limit_row.clone();
        let ul_btn_sel = usage_limit_button.clone();
        let ul_pop_sel = usage_limit_popover.clone();
        let prov_row_ul = provider_row.clone();
        let key_row_ul = api_key_row.clone();
        let current_models_ul = current_models.clone();
        let selected_mod_ul = selected_model_id.clone();
        let warn_pop_sel = warning_tooltip_popover.clone();
        let warn_rev_sel = warning_tooltip_revealer.clone();
        let warn_lbl_sel = warning_tooltip_label.clone();
        let warn_gen_sel = tooltip_generation.clone();
        let active_warn_sel = active_model_warning.clone();
        let warn_cb_sel = on_model_warning_change.clone();
        let cloud_sw_sel = cloud_switch.clone();
        let auto_sync_model = on_config_auto_sync.clone();

        picker_components
            .list_box
            .connect_row_activated(move |_lb, row| {
                let idx = row.index() as usize;
                let maybe_model = current_models_ul.borrow().get(idx).cloned();
                if let Some(m) = maybe_model {
                    let p_idx = prov_row_ul.selected() as usize;
                    let prov = AI_PROVIDERS.get(p_idx).copied().unwrap_or("groq");
                    let key = key_row_ul.text().to_string();

                    update_usage_limit_state(&ul_row_sel, &ul_btn_sel, &ul_pop_sel, &key, &m.id);
                    trigger_usage_limit_refresh(&ul_comp, prov, &key, &m.id);
                    if let Some(cb) = auto_sync_model.0.borrow().as_ref() {
                        cb();
                    }

                    // Immediate model capability & paid-plan check
                    let immediate_warn =
                        crate::services::ai::evaluate_model_selection_warning(&m, false);
                    *active_warn_sel.borrow_mut() = immediate_warn.clone();
                    if let Some(cb) = warn_cb_sel.0.borrow().as_ref() {
                        cb(if cloud_sw_sel.is_active() {
                            immediate_warn.clone()
                        } else {
                            None
                        });
                    }
                    if let Some(ref msg) = immediate_warn {
                        show_warning_tooltip_with_revealer(
                            &warn_pop_sel,
                            &warn_rev_sel,
                            &warn_lbl_sel,
                            &warn_gen_sel,
                            msg,
                        );
                    } else {
                        warn_rev_sel.set_reveal_child(false);
                        warn_pop_sel.popdown();
                    }

                    // Live asynchronous verification using the user's API key
                    if !key.trim().is_empty() || prov == "ollama" {
                        let prov_str = prov.to_string();
                        let key_str = key.clone();
                        let mid_str = m.id.clone();
                        let m_cached = m.clone();
                        let (tx, rx) = tokio::sync::oneshot::channel();
                        crate::services::dictation_worker::tokio_handle().spawn(async move {
                            let res = crate::services::ai::verify_model_selection_live(
                                &prov_str,
                                &key_str,
                                &mid_str,
                                Some(&m_cached),
                            )
                            .await;
                            let _ = tx.send(res);
                        });

                        let active_warn_async = active_warn_sel.clone();
                        let warn_cb_async = warn_cb_sel.clone();
                        let cloud_sw_async = cloud_sw_sel.clone();
                        let sel_mod_check = selected_mod_ul.clone();
                        let pop_async = warn_pop_sel.clone();
                        let rev_async = warn_rev_sel.clone();
                        let lbl_async = warn_lbl_sel.clone();
                        let gen_async = warn_gen_sel.clone();
                        let mid_expected = m.id.clone();

                        gtk4::glib::spawn_future_local(async move {
                            if let Ok(validation) = rx.await
                                && *sel_mod_check.borrow() == mid_expected
                            {
                                let changed =
                                    *active_warn_async.borrow() != validation.warning_message;
                                *active_warn_async.borrow_mut() =
                                    validation.warning_message.clone();
                                if let Some(cb) = warn_cb_async.0.borrow().as_ref() {
                                    cb(if cloud_sw_async.is_active() {
                                        validation.warning_message.clone()
                                    } else {
                                        None
                                    });
                                }
                                if changed {
                                    if let Some(ref msg) = validation.warning_message {
                                        show_warning_tooltip_with_revealer(
                                            &pop_async, &rev_async, &lbl_async, &gen_async, msg,
                                        );
                                    } else {
                                        rev_async.set_reveal_child(false);
                                        pop_async.popdown();
                                    }
                                }
                            }
                        });
                    }
                }
            });
    }

    cloud_group.add(&usage_limit_row);

    // Dynamic model updating when provider changes
    let model_row_clone_prov = model_row.clone();
    let picker_comp_clone_prov = picker_components.clone();
    let current_models_clone_prov = current_models.clone();
    let selected_model_id_clone_prov = selected_model_id.clone();
    let api_key_row_clone_prov = api_key_row.clone();
    let ul_comp_prov = usage_limit_components.clone();
    let ul_row_prov = usage_limit_row.clone();
    let ul_btn_prov = usage_limit_button.clone();
    let ul_pop_prov = usage_limit_popover.clone();
    let auto_sync_prov = on_config_auto_sync.clone();
    provider_row.connect_selected_notify(move |row| {
        let idx = row.selected() as usize;
        if let Some(&prov) = AI_PROVIDERS.get(idx) {
            let key = api_key_row_clone_prov.text().to_string();
            selected_model_id_clone_prov.borrow_mut().clear();
            update_usage_limit_state(&ul_row_prov, &ul_btn_prov, &ul_pop_prov, &key, "");
            trigger_model_fetch(
                &model_row_clone_prov,
                &picker_comp_clone_prov,
                current_models_clone_prov.clone(),
                selected_model_id_clone_prov.clone(),
                prov,
                &key,
                None,
            );
            let mid = selected_model_id_clone_prov.borrow().clone();
            trigger_usage_limit_refresh(&ul_comp_prov, prov, &key, &mid);
            if let Some(cb) = auto_sync_prov.0.borrow().as_ref() {
                cb();
            }
        }
    });

    // Dynamic model updating when API key is entered/pasted
    let model_row_clone_key = model_row.clone();
    let picker_comp_clone_key = picker_components.clone();
    let current_models_clone_key = current_models.clone();
    let selected_model_id_clone_key = selected_model_id.clone();
    let prov_row_clone_key = provider_row.clone();
    let ul_comp_key = usage_limit_components.clone();
    let ul_row_key = usage_limit_row.clone();
    let ul_btn_key = usage_limit_button.clone();
    let ul_pop_key = usage_limit_popover.clone();
    let auto_sync_key = on_config_auto_sync.clone();
    api_key_row.connect_changed(move |entry| {
        let idx = prov_row_clone_key.selected() as usize;
        if let Some(&prov) = AI_PROVIDERS.get(idx) {
            let key = entry.text().to_string();
            selected_model_id_clone_key.borrow_mut().clear();
            update_usage_limit_state(&ul_row_key, &ul_btn_key, &ul_pop_key, &key, "");
            trigger_model_fetch(
                &model_row_clone_key,
                &picker_comp_clone_key,
                current_models_clone_key.clone(),
                selected_model_id_clone_key.clone(),
                prov,
                &key,
                None,
            );
            let mid = selected_model_id_clone_key.borrow().clone();
            trigger_usage_limit_refresh(&ul_comp_key, prov, &key, &mid);
            if let Some(cb) = auto_sync_key.0.borrow().as_ref() {
                cb();
            }
        }
    });

    page.add(&cloud_group);

    // 3. Local AI Card
    let local_group = libadwaita::PreferencesGroup::new();
    local_group.set_title(crate::services::i18n::tr("local_ai", lang));
    let local_switch = gtk4::Switch::new();
    local_switch.set_valign(gtk4::Align::Center);
    local_group.set_header_suffix(Some(&local_switch));

    let local_model_row = libadwaita::ActionRow::new();
    local_model_row.set_title(crate::services::i18n::tr("whisper_model", lang));
    local_model_row.set_activatable(true);
    local_model_row.add_css_class("local-model-combo");

    let local_model_status_label = gtk4::Label::new(None);
    local_model_status_label.set_valign(gtk4::Align::Center);
    local_model_status_label.set_halign(gtk4::Align::End);
    local_model_status_label.set_hexpand(true);
    local_model_status_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    local_model_row.add_suffix(&local_model_status_label);

    let local_dropdown_button = gtk4::Button::from_icon_name("pan-down-symbolic");
    local_dropdown_button.set_valign(gtk4::Align::Center);
    local_dropdown_button.add_css_class("flat");
    local_dropdown_button.set_tooltip_text(Some("Select speech model"));
    local_model_row.add_suffix(&local_dropdown_button);

    let local_picker_components = build_local_model_picker(
        &local_model_row,
        &local_dropdown_button,
        &local_model_status_label,
    );

    let selected_local_model_id =
        Rc::new(RefCell::new(if config.local_model_id.trim().is_empty() {
            "tiny.en".to_string()
        } else {
            config.local_model_id.clone()
        }));

    let pop_click = local_picker_components.popover.clone();
    local_model_row.connect_activated(move |_| {
        pop_click.popup();
    });
    let pop_click2 = local_picker_components.popover.clone();
    local_dropdown_button.connect_clicked(move |_| {
        pop_click2.popup();
    });

    local_group.add(&local_model_row);

    // Model Status Row with status label, download button, and delete button
    let model_status_row = libadwaita::ActionRow::new();
    model_status_row.set_title("Model Status");

    let local_status_label = gtk4::Label::new(None);
    local_status_label.set_valign(gtk4::Align::Center);
    model_status_row.add_suffix(&local_status_label);

    let download_button = gtk4::Button::with_label("Download");
    download_button.set_valign(gtk4::Align::Center);
    download_button.add_css_class("flat");
    model_status_row.add_suffix(&download_button);

    let delete_button = gtk4::Button::from_icon_name("user-trash-symbolic");
    delete_button.set_valign(gtk4::Align::Center);
    delete_button.add_css_class("flat");
    delete_button.add_css_class("destructive-action");
    delete_button.set_tooltip_text(Some("Delete downloaded model file"));
    delete_button.set_visible(false);
    model_status_row.add_suffix(&delete_button);

    local_group.add(&model_status_row);

    // Download Progress Row
    let download_progress_row = libadwaita::ActionRow::new();
    download_progress_row.set_title("Download Progress");
    download_progress_row.set_visible(false);

    let download_progress_bar = gtk4::ProgressBar::new();
    download_progress_bar.set_valign(gtk4::Align::Center);
    download_progress_bar.set_width_request(180);
    download_progress_bar.set_show_text(true);
    download_progress_bar.set_text(Some("0%"));
    download_progress_bar.set_fraction(0.0);
    download_progress_row.add_suffix(&download_progress_bar);

    local_group.add(&download_progress_row);

    // Custom File Row
    let custom_file_row = libadwaita::ActionRow::new();
    custom_file_row.set_title("Custom Model File");
    let custom_path = Rc::new(RefCell::new(config.local_custom_path.clone()));

    if let Some(ref cp) = config.local_custom_path {
        custom_file_row.set_subtitle(cp);
    } else {
        custom_file_row.set_subtitle("No .bin file selected");
    }

    let browse_button = gtk4::Button::with_label("Browse…");
    browse_button.set_valign(gtk4::Align::Center);
    browse_button.add_css_class("flat");
    custom_file_row.add_suffix(&browse_button);
    custom_file_row.set_activatable_widget(Some(&browse_button));
    custom_file_row.set_visible(config.local_model_id == "custom");

    local_group.add(&custom_file_row);

    // Populate model picker list and hover cards
    populate_local_model_picker(
        &local_picker_components,
        &selected_local_model_id,
        &custom_path,
        &local_model_status_label,
        &custom_file_row,
        &local_status_label,
        &download_button,
        &delete_button,
        &on_config_auto_sync,
    );

    // Threads Row
    let threads_row = libadwaita::ComboRow::new();
    threads_row.set_title(crate::services::i18n::tr("threads", lang));
    let thread_labels = &[
        "Auto/System Default",
        "1 thread",
        "2 threads",
        "4 threads",
        "6 threads",
        "8 threads",
        "Custom Threads…",
    ];
    let threads_model = gtk4::StringList::new(thread_labels);
    threads_row.set_model(Some(&threads_model));

    let selected_threads_idx = match config.local_threads {
        0 => 0,
        1 => 1,
        2 => 2,
        4 => 3,
        6 => 4,
        8 => 5,
        _ => 6,
    };
    threads_row.set_selected(selected_threads_idx);
    local_group.add(&threads_row);

    // Custom Threads Row
    let custom_threads_row = libadwaita::ActionRow::new();
    custom_threads_row.set_title("Custom Thread Count");
    custom_threads_row.set_subtitle("Specify number of CPU execution threads (1–128)");
    let custom_threads_adj = gtk4::Adjustment::new(
        if config.local_threads > 0 {
            config.local_threads as f64
        } else {
            4.0
        },
        1.0,
        128.0,
        1.0,
        4.0,
        0.0,
    );
    let custom_threads_spin = gtk4::SpinButton::new(Some(&custom_threads_adj), 1.0, 0);
    custom_threads_spin.set_valign(gtk4::Align::Center);
    custom_threads_row.add_suffix(&custom_threads_spin);
    custom_threads_row.set_visible(selected_threads_idx == 6);
    local_group.add(&custom_threads_row);

    let ctr_clone = custom_threads_row.clone();
    let auto_sync_threads = on_config_auto_sync.clone();
    threads_row.connect_selected_notify(move |row| {
        let is_custom = row.selected() == 6;
        ctr_clone.set_visible(is_custom);
        if let Some(cb) = auto_sync_threads.0.borrow().as_ref() {
            cb();
        }
    });
    let auto_sync_spin = on_config_auto_sync.clone();
    custom_threads_spin.connect_value_changed(move |_| {
        if let Some(cb) = auto_sync_spin.0.borrow().as_ref() {
            cb();
        }
    });

    page.add(&local_group);

    // Initial status update
    update_local_model_status_ui(
        &selected_local_model_id.borrow(),
        custom_path.borrow().as_deref(),
        &local_status_label,
        &download_button,
        &delete_button,
    );

    // Wire browse button with FileDialog
    let cp_browse = custom_path.clone();
    let custom_row_browse = custom_file_row.clone();
    let status_lbl_browse = local_status_label.clone();
    let dl_btn_browse = download_button.clone();
    let del_btn_browse = delete_button.clone();
    let auto_sync_browse = on_config_auto_sync.clone();

    browse_button.connect_clicked(move |_| {
        let dialog = gtk4::FileDialog::new();
        dialog.set_title("Select Speech Model (.bin)");

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Speech Model File (*.bin)"));
        filter.add_pattern("*.bin");
        let filters = gtk4::gio::ListStore::new::<gtk4::FileFilter>();
        filters.append(&filter);
        dialog.set_filters(Some(&filters));
        dialog.set_default_filter(Some(&filter));

        let cp = cp_browse.clone();
        let row = custom_row_browse.clone();
        let st = status_lbl_browse.clone();
        let btn = dl_btn_browse.clone();
        let del_btn = del_btn_browse.clone();
        let sync_cb = auto_sync_browse.clone();

        dialog.open(
            None::<&gtk4::Window>,
            None::<&gtk4::gio::Cancellable>,
            move |res| {
                if let Ok(file) = res
                    && let Some(path) = file.path()
                {
                    let path_str = path.to_string_lossy().to_string();
                    *cp.borrow_mut() = Some(path_str.clone());
                    row.set_subtitle(&path_str);
                    update_local_model_status_ui("custom", Some(&path_str), &st, &btn, &del_btn);
                    if let Some(cb) = sync_cb.0.borrow().as_ref() {
                        cb();
                    }
                }
            },
        );
    });

    // Wire download button
    let dl_btn_click = download_button.clone();
    let prog_row_click = download_progress_row.clone();
    let prog_bar_click = download_progress_bar.clone();
    let lm_row_click = local_model_row.clone();
    let lm_drop_click = local_dropdown_button.clone();
    let status_lbl_click = local_status_label.clone();
    let custom_path_click = custom_path.clone();
    let del_btn_dl = delete_button.clone();
    let sel_id_dl = selected_local_model_id.clone();
    let auto_sync_dl = on_config_auto_sync.clone();

    download_button.connect_clicked(move |_| {
        let catalog = get_local_model_catalog();
        let mid = sel_id_dl.borrow().clone();
        let selected_model = match catalog.into_iter().find(|m| m.id == mid) {
            Some(m) => m,
            None => return,
        };
        let model_id = selected_model.id.clone();

        // Immediately disable download button and model selection to prevent concurrent downloads
        dl_btn_click.set_sensitive(false);
        lm_row_click.set_sensitive(false);
        lm_drop_click.set_sensitive(false);

        // Immediately reveal download progress row and reset progress bar
        prog_row_click.set_visible(true);
        prog_row_click.set_subtitle(&format!("Downloading {}...", selected_model.filename));
        prog_bar_click.set_fraction(0.0);
        prog_bar_click.set_text(Some("0%"));

        enum DownloadMsg {
            Progress(u64, u64),
            Finished(Result<std::path::PathBuf, String>),
        }

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<DownloadMsg>();
        let mid_clone = model_id.clone();
        let tx_prog = tx.clone();

        crate::services::dictation_worker::tokio_handle().spawn(async move {
            let res = download_local_model(&mid_clone, move |d, t| {
                let _ = tx_prog.send(DownloadMsg::Progress(d, t));
            })
            .await;
            let _ = tx.send(DownloadMsg::Finished(res.map_err(|e| e.to_string())));
        });

        let btn = dl_btn_click.clone();
        let del_btn = del_btn_dl.clone();
        let pr_row = prog_row_click.clone();
        let pr_bar = prog_bar_click.clone();
        let st_lbl = status_lbl_click.clone();
        let cp_ref = custom_path_click.clone();
        let lm_row = lm_row_click.clone();
        let lm_drop = lm_drop_click.clone();
        let sync_dl_done = auto_sync_dl.clone();

        gtk4::glib::spawn_future_local(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    DownloadMsg::Progress(downloaded, total) => {
                        if total > 0 {
                            let frac = (downloaded as f64) / (total as f64);
                            pr_bar.set_fraction(frac);
                            pr_bar.set_text(Some(&format!("{:.0}%", frac * 100.0)));
                            pr_row.set_subtitle(&format!(
                                "{:.1} / {:.1} MB",
                                downloaded as f64 / 1_048_576.0,
                                total as f64 / 1_048_576.0
                            ));
                        }
                    }
                    DownloadMsg::Finished(Ok(_path)) => {
                        pr_bar.set_fraction(1.0);
                        pr_bar.set_text(Some("100%"));
                        pr_row.set_subtitle("Download complete");
                        update_local_model_status_ui(
                            &model_id,
                            cp_ref.borrow().as_deref(),
                            &st_lbl,
                            &btn,
                            &del_btn,
                        );
                        pr_row.set_visible(false);
                        if let Some(cb) = sync_dl_done.0.borrow().as_ref() {
                            cb();
                        }
                    }
                    DownloadMsg::Finished(Err(err)) => {
                        pr_row.set_subtitle(&format!("Download failed: {}", err));
                        pr_row.set_visible(true);
                        st_lbl.set_text("Download failed");
                        st_lbl.remove_css_class("success");
                        st_lbl.add_css_class("error");
                        btn.set_label("Retry Download");
                    }
                }
            }
            btn.set_sensitive(true);
            lm_row.set_sensitive(true);
            lm_drop.set_sensitive(true);
        });
    });

    // Wire delete button
    let del_btn_click = delete_button.clone();
    let dl_btn_del = download_button.clone();
    let status_lbl_del = local_status_label.clone();
    let sel_id_del = selected_local_model_id.clone();
    let custom_path_del = custom_path.clone();
    let custom_row_del = custom_file_row.clone();
    let auto_sync_del = on_config_auto_sync.clone();

    delete_button.connect_clicked(move |_| {
        let mid = sel_id_del.borrow().clone();
        let path = resolve_model_path(&mid, custom_path_del.borrow().as_deref());
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }

        if mid == "custom" {
            *custom_path_del.borrow_mut() = None;
            custom_row_del.set_subtitle("No .bin file selected");
        }

        update_local_model_status_ui(
            &mid,
            custom_path_del.borrow().as_deref(),
            &status_lbl_del,
            &dl_btn_del,
            &del_btn_click,
        );
        if let Some(cb) = auto_sync_del.0.borrow().as_ref() {
            cb();
        }
    });

    // Synchronize switches with mutual exclusivity
    let is_syncing = Rc::new(RefCell::new(false));

    let set_cloud_sensitive = {
        let provider_row = provider_row.clone();
        let api_key_row = api_key_row.clone();
        let model_row = model_row.clone();
        let usage_limit_row = usage_limit_row.clone();
        let usage_limit_button = usage_limit_button.clone();
        let selected_model_id = selected_model_id.clone();
        move |sensitive: bool| {
            provider_row.set_sensitive(sensitive);
            api_key_row.set_sensitive(sensitive);
            model_row.set_sensitive(sensitive);
            usage_limit_row.set_sensitive(sensitive);
            let ul_active = sensitive
                && !api_key_row.text().trim().is_empty()
                && !selected_model_id.borrow().trim().is_empty();
            usage_limit_button.set_sensitive(ul_active);
        }
    };

    let set_local_sensitive = {
        let local_model_row = local_model_row.clone();
        let local_dropdown_btn = local_dropdown_button.clone();
        let model_status_row = model_status_row.clone();
        let download_progress_row = download_progress_row.clone();
        let custom_file_row = custom_file_row.clone();
        let threads_row = threads_row.clone();
        let custom_threads_row = custom_threads_row.clone();
        move |sensitive: bool| {
            local_model_row.set_sensitive(sensitive);
            local_dropdown_btn.set_sensitive(sensitive);
            model_status_row.set_sensitive(sensitive);
            download_progress_row.set_sensitive(sensitive);
            custom_file_row.set_sensitive(sensitive);
            threads_row.set_sensitive(sensitive);
            custom_threads_row.set_sensitive(sensitive);
        }
    };

    let is_local = config.ai_mode == "local";
    *is_syncing.borrow_mut() = true;
    cloud_switch.set_active(!is_local);
    local_switch.set_active(is_local);
    set_cloud_sensitive(!is_local);
    set_local_sensitive(is_local);
    *is_syncing.borrow_mut() = false;

    let is_syncing_cloud = is_syncing.clone();
    let local_switch_cloud = local_switch.clone();
    let set_cloud_sens_c = set_cloud_sensitive.clone();
    let set_local_sens_c = set_local_sensitive.clone();
    let active_warn_cloud = active_model_warning.clone();
    let warn_cb_cloud = on_model_warning_change.clone();
    let auto_sync_cloud = on_config_auto_sync.clone();

    cloud_switch.connect_active_notify(move |switch| {
        if *is_syncing_cloud.borrow() {
            return;
        }
        *is_syncing_cloud.borrow_mut() = true;
        if switch.is_active() {
            local_switch_cloud.set_active(false);
            set_cloud_sens_c(true);
            set_local_sens_c(false);
            if let Some(cb) = warn_cb_cloud.0.borrow().as_ref() {
                cb(active_warn_cloud.borrow().clone());
            }
        } else {
            local_switch_cloud.set_active(true);
            set_cloud_sens_c(false);
            set_local_sens_c(true);
            if let Some(cb) = warn_cb_cloud.0.borrow().as_ref() {
                cb(None);
            }
        }
        *is_syncing_cloud.borrow_mut() = false;
        if let Some(cb) = auto_sync_cloud.0.borrow().as_ref() {
            cb();
        }
    });

    let is_syncing_local = is_syncing.clone();
    let cloud_switch_local = cloud_switch.clone();
    let set_cloud_sens_l = set_cloud_sensitive.clone();
    let set_local_sens_l = set_local_sensitive.clone();
    let active_warn_local = active_model_warning.clone();
    let warn_cb_local = on_model_warning_change.clone();
    let auto_sync_local = on_config_auto_sync.clone();

    local_switch.connect_active_notify(move |switch| {
        if *is_syncing_local.borrow() {
            return;
        }
        *is_syncing_local.borrow_mut() = true;
        if switch.is_active() {
            cloud_switch_local.set_active(false);
            set_local_sens_l(true);
            set_cloud_sens_l(false);
            if let Some(cb) = warn_cb_local.0.borrow().as_ref() {
                cb(None);
            }
        } else {
            cloud_switch_local.set_active(true);
            set_local_sens_l(false);
            set_cloud_sens_l(true);
            if let Some(cb) = warn_cb_local.0.borrow().as_ref() {
                cb(active_warn_local.borrow().clone());
            }
        }
        *is_syncing_local.borrow_mut() = false;
        if let Some(cb) = auto_sync_local.0.borrow().as_ref() {
            cb();
        }
    });

    // 4. Audio & System Card
    let general_group = libadwaita::PreferencesGroup::new();
    general_group.set_title(crate::services::i18n::tr("general", lang));

    let device_items = AudioRecorder::list_friendly_input_devices();
    let device_slices: Vec<&str> = device_items.iter().map(|s| s.as_str()).collect();
    let audio_device_row = libadwaita::ComboRow::new();
    audio_device_row.set_title(crate::services::i18n::tr("microphone", lang));
    let audio_model = gtk4::StringList::new(&device_slices);
    audio_device_row.set_model(Some(&audio_model));

    let initial_hw_profile = AudioRecorder::detect_connected_audio_profile();
    let last_detected_audio_profile = Rc::new(RefCell::new(initial_hw_profile.to_string()));

    if let Some(ref target) = config.audio_device {
        if let Some(idx) = device_items.iter().position(|d| d == target) {
            audio_device_row.set_selected(idx as u32);
        } else if let Some(idx) = device_items.iter().position(|d| d == initial_hw_profile) {
            audio_device_row.set_selected(idx as u32);
        } else {
            audio_device_row.set_selected(0);
        }
    } else if let Some(idx) = device_items.iter().position(|d| d == initial_hw_profile) {
        audio_device_row.set_selected(idx as u32);
    } else {
        audio_device_row.set_selected(0);
    }
    general_group.add(&audio_device_row);

    // Live hardware hotplug monitor: polls connected audio hardware every 2 seconds
    // off the GTK thread and automatically switches the Microphone option when a
    // Bluetooth earphone ("Handsfree") or wired/USB headset ("Headphones") is plugged
    // or unplugged, while preserving manual user overrides between state transitions.
    {
        let row_weak = audio_device_row.downgrade();
        let last_prof = last_detected_audio_profile.clone();
        let dev_list = device_items.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
            let Some(row) = row_weak.upgrade() else {
                return gtk4::glib::ControlFlow::Break;
            };
            let last_prof_inner = last_prof.clone();
            let dev_list_inner = dev_list.clone();
            let (tx, rx) = std::sync::mpsc::channel::<&'static str>();
            crate::services::dictation_worker::tokio_handle().spawn_blocking(move || {
                let detected = AudioRecorder::detect_connected_audio_profile();
                let _ = tx.send(detected);
            });
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(40), move || {
                match rx.try_recv() {
                    Ok(detected) => {
                        let mut last = last_prof_inner.borrow_mut();
                        if last.as_str() != detected {
                            *last = detected.to_string();
                            if let Some(idx) = dev_list_inner.iter().position(|d| d == detected) {
                                row.set_selected(idx as u32);
                            }
                        }
                        gtk4::glib::ControlFlow::Break
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => gtk4::glib::ControlFlow::Continue,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        gtk4::glib::ControlFlow::Break
                    }
                }
            });
            gtk4::glib::ControlFlow::Continue
        });
    }

    let hotkey_row = libadwaita::EntryRow::new();
    hotkey_row.set_title(crate::services::i18n::tr("global_shortcut", lang));
    hotkey_row.set_text(&config.hotkey);
    let auto_sync_hotkey = on_config_auto_sync.clone();
    hotkey_row.connect_changed(move |_| {
        if let Some(cb) = auto_sync_hotkey.0.borrow().as_ref() {
            cb();
        }
    });
    general_group.add(&hotkey_row);

    page.add(&general_group);

    // Save Button Group
    let save_group = libadwaita::PreferencesGroup::new();
    let save_button = gtk4::Button::with_label(crate::services::i18n::tr("save", lang));
    save_button.add_css_class("suggested-action");
    save_button.add_css_class("pill");
    save_button.set_halign(gtk4::Align::Center);
    save_button.set_margin_top(16);
    save_button.set_margin_bottom(16);
    save_group.add(&save_button);
    page.add(&save_group);

    // Apply initial usage limit state gating
    update_usage_limit_state(
        &usage_limit_row,
        &usage_limit_button,
        &usage_limit_popover,
        &config.ai_api_key,
        &config.ai_model,
    );

    let widgets = SettingsViewWidgets {
        page: page.clone(),
        appearance_group,
        cloud_group,
        local_group,
        general_group,
        theme_dropdown: theme_row,
        provider_row,
        api_key_row,
        model_row,
        usage_limit_row,
        usage_limit_button,
        usage_limit_popover,
        usage_limit_components,
        audio_device_row,
        hotkey_row,
        save_button,
        audio_devices: device_items,
        current_models,
        selected_model_id,
        picker_components,
        cloud_switch,
        local_switch,
        local_model_row,
        local_picker_components,
        selected_local_model_id,
        model_status_row,
        download_button,
        delete_button,
        download_progress_row,
        download_progress_bar,
        custom_file_row,
        threads_row,
        custom_threads_row,
        custom_threads_spin,
        custom_path,
        warning_tooltip_popover,
        warning_tooltip_revealer,
        warning_tooltip_label,
        tooltip_generation,
        active_model_warning,
        on_model_warning_change,
        on_config_auto_sync,
        last_detected_audio_profile,
    };

    (page, widgets)
}

/// Reads current widget values into a Config instance.
pub fn extract_config_from_widgets(widgets: &SettingsViewWidgets, base_config: &Config) -> Config {
    let mut updated = base_config.clone();

    // Theme
    let theme_idx = widgets.theme_dropdown.selected();
    updated.theme = if theme_idx == 1 {
        "white".to_string()
    } else {
        "dark".to_string()
    };

    // AI Mode
    updated.ai_mode = if widgets.local_switch.is_active() {
        "local".to_string()
    } else {
        "cloud".to_string()
    };

    // AI Provider
    let prov_idx = widgets.provider_row.selected() as usize;
    let prov = AI_PROVIDERS.get(prov_idx).copied().unwrap_or("groq");
    updated.ai_provider = prov.to_string();

    // AI Key
    updated.ai_api_key = widgets.api_key_row.text().to_string();

    // AI Model
    let selected_id = widgets.selected_model_id.borrow().clone();
    if !selected_id.is_empty() {
        updated.ai_model = selected_id;
    } else if let Some(first) = widgets.current_models.borrow().first() {
        updated.ai_model = first.id.clone();
    } else {
        let models = crate::services::ai::get_provider_models(prov);
        if let Some(first) = models.first() {
            updated.ai_model = first.id.clone();
        }
    }

    // Local AI Model
    updated.local_model_id = widgets.selected_local_model_id.borrow().clone();

    // Local Custom Path
    updated.local_custom_path = if updated.local_model_id == "custom" {
        widgets.custom_path.borrow().clone()
    } else {
        None
    };

    // Local Threads
    updated.local_threads = match widgets.threads_row.selected() {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 6,
        5 => 8,
        _ => widgets.custom_threads_spin.value() as u32,
    };

    // Audio Device
    let dev_idx = widgets.audio_device_row.selected() as usize;
    if dev_idx == 0 || dev_idx >= widgets.audio_devices.len() {
        updated.audio_device = None;
    } else {
        updated.audio_device = Some(widgets.audio_devices[dev_idx].clone());
    }

    // Hotkey
    updated.hotkey = widgets.hotkey_row.text().to_string();

    updated
}
