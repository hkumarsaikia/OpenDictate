//! Header bar component for OpenDictate MainWindow settings dashboard.

use crate::config::Config;
use crate::services::i18n::{get_languages, tr};
use crate::services::storage::StorageService;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use std::cell::RefCell;
use std::process::{Command, Stdio};
use std::rc::Rc;

/// Shared callback invoked when the MiniBar scale percentage changes.
pub type ScaleChangeCallback = Rc<RefCell<Option<Rc<dyn Fn(u32)>>>>;

/// Shared callback invoked when the UI language code changes.
pub type LanguageChangeCallback = Rc<RefCell<Option<Rc<dyn Fn(String)>>>>;

/// Widgets contained in the MainWindow header bar and Main Menu popover.
#[derive(Clone)]
pub struct HeaderBarWidgets {
    pub header_bar: libadwaita::HeaderBar,
    pub title_widget: libadwaita::WindowTitle,
    pub main_menu_button: gtk4::MenuButton,
    pub popover: gtk4::Popover,
    pub stack: gtk4::Stack,
    pub zoom_out_button: gtk4::Button,
    pub zoom_reset_button: gtk4::Button,
    pub zoom_in_button: gtk4::Button,
    pub zoom_label: gtk4::Label,
    pub minibar_scale: Rc<RefCell<u32>>,
    pub on_scale_change: ScaleChangeCallback,
    pub language_button: gtk4::Button,
    pub history_button: gtk4::Button,
    pub about_button: gtk4::Button,
    pub language_back_button: gtk4::Button,
    pub history_back_button: gtk4::Button,
    pub language_list_box: gtk4::ListBox,
    pub history_list_box: gtk4::ListBox,
    pub history_empty_label: gtk4::Label,
    pub active_language: Rc<RefCell<String>>,
    pub on_language_change: LanguageChangeCallback,
    pub storage: StorageService,
    pub language_label: gtk4::Label,
    pub history_label: gtk4::Label,
    pub about_label: gtk4::Label,
    pub language_title: gtk4::Label,
    pub history_title: gtk4::Label,
    pub language_back_label: gtk4::Label,
    pub history_back_label: gtk4::Label,
    pub check_imgs: Rc<RefCell<Vec<(&'static str, gtk4::Image)>>>,
}

impl std::fmt::Debug for HeaderBarWidgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeaderBarWidgets")
            .field("header_bar", &self.header_bar)
            .field("title_widget", &self.title_widget)
            .field("main_menu_button", &self.main_menu_button)
            .field("popover", &self.popover)
            .field("stack", &self.stack)
            .field("minibar_scale", &*self.minibar_scale.borrow())
            .field("active_language", &*self.active_language.borrow())
            .finish()
    }
}

impl HeaderBarWidgets {
    /// Sets the callback to be triggered when the user selects a language.
    pub fn set_on_language_change<F: Fn(String) + 'static>(&self, callback: F) {
        *self.on_language_change.borrow_mut() = Some(Rc::new(callback));
    }

    /// Sets the callback to be triggered when the user changes the MiniBar zoom scale.
    pub fn set_on_scale_change<F: Fn(u32) + 'static>(&self, callback: F) {
        *self.on_scale_change.borrow_mut() = Some(Rc::new(callback));
    }

    /// Updates the displayed zoom percentage and button sensitivities.
    pub fn set_scale(&self, scale_percent: u32) {
        let clamped = scale_percent.clamp(70, 200);
        *self.minibar_scale.borrow_mut() = clamped;
        self.zoom_label.set_label(&format!("{}%", clamped));
        self.zoom_out_button.set_sensitive(clamped > 70);
        self.zoom_in_button.set_sensitive(clamped < 200);
    }

    /// Refreshes the dictation history list from the SQLite storage service.
    pub fn refresh_history(&self) {
        refresh_history_list(
            &self.history_list_box,
            &self.history_empty_label,
            &self.storage,
            &self.active_language.borrow(),
        );
    }

    /// Dynamically retranslates all header bar widgets and Main Menu popover subpages in-place.
    pub fn retranslate(&self, lang: &str) {
        *self.active_language.borrow_mut() = lang.to_string();

        let dir = if crate::services::i18n::is_rtl(lang) {
            gtk4::TextDirection::Rtl
        } else {
            gtk4::TextDirection::Ltr
        };
        self.popover.set_direction(dir);

        self.title_widget.set_title(tr("settings", lang));
        self.main_menu_button
            .set_tooltip_text(Some(tr("main_menu", lang)));

        self.language_label.set_label(tr("language", lang));
        self.history_label.set_label(tr("history", lang));
        self.about_label.set_label(tr("about_opendictate", lang));

        self.language_title.set_label(tr("language", lang));
        self.history_title.set_label(tr("history", lang));

        self.language_back_button
            .set_tooltip_text(Some(tr("back", lang)));
        self.language_back_label.set_label(tr("back", lang));

        self.history_back_button
            .set_tooltip_text(Some(tr("back", lang)));
        self.history_back_label.set_label(tr("back", lang));

        self.history_empty_label.set_label(tr("no_history", lang));

        for (c, img) in self.check_imgs.borrow().iter() {
            img.set_visible(*c == lang);
        }

        self.refresh_history();
    }
}

/// Ensures custom CSS rules for the Main Menu popover and Coffee dialog are loaded once.
pub fn ensure_main_menu_css() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static LOADED: AtomicBool = AtomicBool::new(false);
    if LOADED.swap(true, Ordering::SeqCst) {
        return;
    }
    let provider = gtk4::CssProvider::new();
    let css = r#"
    .main-menu-popover,
    .main-menu-popover *,
    .main-menu-popover button,
    .main-menu-popover button label,
    .main-menu-popover label,
    .main-menu-popover .subpage-heading {
        font-weight: 400;
    }
    .main-menu-popover .subpage-heading {
        font-size: 13px;
    }
    .coffee-tier-btn {
        padding: 10px 14px;
        border-radius: 10px;
    }
    .coffee-badge-icon {
        color: #e5a50a;
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

/// Constructs the Libadwaita HeaderBar with settings title and Main Menu popover with in-popover stack navigation.
pub fn build_header_bar(
    config: &Config,
    storage: &StorageService,
) -> (libadwaita::HeaderBar, HeaderBarWidgets) {
    crate::ui::theme::ensure_app_icons_registered();
    ensure_main_menu_css();
    let header_bar = libadwaita::HeaderBar::new();
    header_bar.set_show_end_title_buttons(true);
    header_bar.set_show_start_title_buttons(true);

    let active_lang = if config.ui_language.is_empty() {
        "en"
    } else {
        &config.ui_language
    };

    // Center WindowTitle without subtitle
    let title_widget = libadwaita::WindowTitle::new(tr("settings", active_lang), "");
    header_bar.set_title_widget(Some(&title_widget));

    // Construct Main Menu popover with sliding Stack navigation
    let stack = gtk4::Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(200);
    stack.set_hhomogeneous(false);
    stack.set_vhomogeneous(false);
    stack.set_interpolate_size(true);

    // Page 1: "main" (Zoom Row, Language, History, About)
    let page_main = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    page_main.set_margin_top(4);
    page_main.set_margin_bottom(4);
    page_main.set_margin_start(4);
    page_main.set_margin_end(4);
    page_main.set_width_request(250);
    page_main.set_valign(gtk4::Align::Start);

    // GNOME Terminal-style Zoom Control Row above Language button
    let initial_scale = config.minibar_scale.clamp(70, 200);
    let minibar_scale = Rc::new(RefCell::new(initial_scale));
    let on_scale_change: ScaleChangeCallback = Rc::new(RefCell::new(None));

    let zoom_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    zoom_box.set_margin_top(2);
    zoom_box.set_margin_bottom(2);
    zoom_box.set_margin_start(4);
    zoom_box.set_margin_end(4);

    let zoom_out_button = gtk4::Button::builder()
        .icon_name("zoom-out-symbolic")
        .tooltip_text("Zoom Out")
        .sensitive(initial_scale > 70)
        .css_classes(vec!["flat".to_string(), "circular".to_string()])
        .build();
    zoom_out_button.update_property(&[gtk4::accessible::Property::Label("Zoom Out")]);

    let zoom_label = gtk4::Label::builder()
        .label(format!("{}%", initial_scale))
        .width_chars(5)
        .halign(gtk4::Align::Center)
        .build();

    let zoom_reset_button = gtk4::Button::builder()
        .tooltip_text("Reset Size")
        .hexpand(true)
        .css_classes(vec!["flat".to_string(), "numeric".to_string()])
        .child(&zoom_label)
        .build();
    zoom_reset_button.update_property(&[gtk4::accessible::Property::Label("Reset Size")]);

    let zoom_in_button = gtk4::Button::builder()
        .icon_name("zoom-in-symbolic")
        .tooltip_text("Zoom In")
        .sensitive(initial_scale < 200)
        .css_classes(vec!["flat".to_string(), "circular".to_string()])
        .build();
    zoom_in_button.update_property(&[gtk4::accessible::Property::Label("Zoom In")]);

    zoom_box.append(&zoom_out_button);
    zoom_box.append(&zoom_reset_button);
    zoom_box.append(&zoom_in_button);
    page_main.append(&zoom_box);

    let zoom_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    zoom_sep.set_margin_top(2);
    zoom_sep.set_margin_bottom(2);
    page_main.append(&zoom_sep);

    // Wire zoom out (-)
    {
        let scale_rc = minibar_scale.clone();
        let cb_rc = on_scale_change.clone();
        let lbl = zoom_label.clone();
        let out_btn = zoom_out_button.clone();
        let in_btn = zoom_in_button.clone();
        zoom_out_button.connect_clicked(move |_| {
            let next = scale_rc.borrow().saturating_sub(10).clamp(70, 200);
            *scale_rc.borrow_mut() = next;
            lbl.set_label(&format!("{}%", next));
            out_btn.set_sensitive(next > 70);
            in_btn.set_sensitive(next < 200);
            if let Some(ref cb) = *cb_rc.borrow() {
                cb(next);
            }
        });
    }

    // Wire zoom reset (100%)
    {
        let scale_rc = minibar_scale.clone();
        let cb_rc = on_scale_change.clone();
        let lbl = zoom_label.clone();
        let out_btn = zoom_out_button.clone();
        let in_btn = zoom_in_button.clone();
        zoom_reset_button.connect_clicked(move |_| {
            let next = 100u32;
            *scale_rc.borrow_mut() = next;
            lbl.set_label("100%");
            out_btn.set_sensitive(true);
            in_btn.set_sensitive(true);
            if let Some(ref cb) = *cb_rc.borrow() {
                cb(next);
            }
        });
    }

    // Wire zoom in (+)
    {
        let scale_rc = minibar_scale.clone();
        let cb_rc = on_scale_change.clone();
        let lbl = zoom_label.clone();
        let out_btn = zoom_out_button.clone();
        let in_btn = zoom_in_button.clone();
        zoom_in_button.connect_clicked(move |_| {
            let next = (*scale_rc.borrow() + 10).clamp(70, 200);
            *scale_rc.borrow_mut() = next;
            lbl.set_label(&format!("{}%", next));
            out_btn.set_sensitive(next > 70);
            in_btn.set_sensitive(next < 200);
            if let Some(ref cb) = *cb_rc.borrow() {
                cb(next);
            }
        });
    }

    // Language button
    let language_button = gtk4::Button::builder()
        .css_classes(vec!["flat".to_string()])
        .build();
    let lang_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    lang_box.set_margin_top(2);
    lang_box.set_margin_bottom(2);
    lang_box.set_margin_start(4);
    lang_box.set_margin_end(4);

    let lang_lbl = gtk4::Label::builder()
        .label(tr("language", active_lang))
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .build();
    let lang_arrow = gtk4::Image::from_icon_name("go-next-symbolic");
    lang_arrow.set_halign(gtk4::Align::End);
    lang_box.append(&lang_lbl);
    lang_box.append(&lang_arrow);
    language_button.set_child(Some(&lang_box));

    let stack_clone_lang = stack.clone();
    language_button.connect_clicked(move |_| {
        stack_clone_lang.set_visible_child_full("language", gtk4::StackTransitionType::SlideLeft);
    });
    page_main.append(&language_button);

    // History button
    let history_button = gtk4::Button::builder()
        .css_classes(vec!["flat".to_string()])
        .build();
    let hist_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hist_box.set_margin_top(2);
    hist_box.set_margin_bottom(2);
    hist_box.set_margin_start(4);
    hist_box.set_margin_end(4);

    let hist_lbl = gtk4::Label::builder()
        .label(tr("history", active_lang))
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .build();
    let hist_arrow = gtk4::Image::from_icon_name("go-next-symbolic");
    hist_arrow.set_halign(gtk4::Align::End);
    hist_box.append(&hist_lbl);
    hist_box.append(&hist_arrow);
    history_button.set_child(Some(&hist_box));

    page_main.append(&history_button);

    // Separator
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(4);
    sep.set_margin_bottom(4);
    page_main.append(&sep);

    // About OpenDictate button
    let about_button = gtk4::Button::builder()
        .css_classes(vec!["flat".to_string()])
        .build();
    let about_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    about_box.set_margin_top(2);
    about_box.set_margin_bottom(2);
    about_box.set_margin_start(4);
    about_box.set_margin_end(4);

    let about_lbl = gtk4::Label::builder()
        .label(tr("about_opendictate", active_lang))
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .build();
    about_box.append(&about_lbl);
    about_button.set_child(Some(&about_box));

    page_main.append(&about_button);

    // Page 2: "language" (25 languages ListBox)
    let page_language = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    page_language.set_margin_top(6);
    page_language.set_margin_bottom(6);
    page_language.set_margin_start(6);
    page_language.set_margin_end(6);
    page_language.set_width_request(250);

    let lang_top_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    lang_top_box.set_margin_bottom(4);

    let language_back_button = gtk4::Button::builder()
        .css_classes(vec!["flat".to_string()])
        .tooltip_text(tr("back", active_lang))
        .build();
    let back_box_l = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    back_box_l.append(&gtk4::Image::from_icon_name("go-previous-symbolic"));
    let language_back_label = gtk4::Label::new(Some(tr("back", active_lang)));
    back_box_l.append(&language_back_label);
    language_back_button.set_child(Some(&back_box_l));

    let stack_clone_lback = stack.clone();
    language_back_button.connect_clicked(move |_| {
        stack_clone_lback.set_visible_child_full("main", gtk4::StackTransitionType::SlideRight);
    });
    lang_top_box.append(&language_back_button);

    let lang_title = gtk4::Label::builder()
        .label(tr("language", active_lang))
        .css_classes(vec!["subpage-heading".to_string()])
        .halign(gtk4::Align::Center)
        .hexpand(true)
        .build();
    lang_top_box.append(&lang_title);

    let dummy_spacer_l = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    dummy_spacer_l.set_width_request(42);
    lang_top_box.append(&dummy_spacer_l);

    page_language.append(&lang_top_box);

    let lang_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    lang_sep.set_margin_bottom(4);
    page_language.append(&lang_sep);

    let lang_scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .min_content_width(238)
        .max_content_width(238)
        .propagate_natural_width(false)
        .min_content_height(90)
        .max_content_height(230)
        .propagate_natural_height(true)
        .build();

    let language_list_box = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .css_classes(vec!["navigation-sidebar".to_string()])
        .build();

    let active_language = Rc::new(RefCell::new(active_lang.to_string()));
    let on_language_change: LanguageChangeCallback = Rc::new(RefCell::new(None));

    let languages = get_languages();
    let check_imgs: Rc<RefCell<Vec<(&'static str, gtk4::Image)>>> =
        Rc::new(RefCell::new(Vec::new()));

    for lang in languages {
        let row = gtk4::ListBoxRow::new();
        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        row_box.set_margin_top(6);
        row_box.set_margin_bottom(6);
        row_box.set_margin_start(8);
        row_box.set_margin_end(8);

        let text_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        text_box.set_hexpand(true);
        let native_lbl = gtk4::Label::builder()
            .label(lang.native_name)
            .halign(gtk4::Align::Start)
            .build();
        text_box.append(&native_lbl);

        if lang.native_name != lang.name {
            let eng_lbl = gtk4::Label::builder()
                .label(format!("({})", lang.name))
                .halign(gtk4::Align::Start)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .max_width_chars(16)
                .css_classes(vec!["dim-label".to_string(), "caption".to_string()])
                .build();
            text_box.append(&eng_lbl);
        }
        row_box.append(&text_box);

        let check_img = gtk4::Image::from_icon_name("object-select-symbolic");
        check_img.set_halign(gtk4::Align::End);
        check_img.set_visible(lang.code == active_lang);
        row_box.append(&check_img);

        check_imgs.borrow_mut().push((lang.code, check_img));

        row.set_child(Some(&row_box));
        language_list_box.append(&row);
    }

    let active_lang_clone = active_language.clone();
    let on_change_clone = on_language_change.clone();
    let check_imgs_clone = check_imgs.clone();
    language_list_box.connect_row_activated(move |_lb, row| {
        let raw_idx = row.index();
        if raw_idx < 0 {
            return;
        }
        let idx = raw_idx as usize;
        let langs = get_languages();
        if let Some(target_lang) = langs.get(idx) {
            let code = target_lang.code.to_string();
            *active_lang_clone.borrow_mut() = code.clone();
            for (c, img) in check_imgs_clone.borrow().iter() {
                img.set_visible(*c == code);
            }
            if let Some(ref cb) = *on_change_clone.borrow() {
                cb(code);
            }
        }
    });

    lang_scrolled.set_child(Some(&language_list_box));
    page_language.append(&lang_scrolled);

    // Page 3: "history" (Dictation records)
    let page_history = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    page_history.set_margin_top(6);
    page_history.set_margin_bottom(6);
    page_history.set_margin_start(6);
    page_history.set_margin_end(6);
    page_history.set_width_request(250);

    let hist_top_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    hist_top_box.set_margin_bottom(4);

    let history_back_button = gtk4::Button::builder()
        .css_classes(vec!["flat".to_string()])
        .tooltip_text(tr("back", active_lang))
        .build();
    let back_box_h = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    back_box_h.append(&gtk4::Image::from_icon_name("go-previous-symbolic"));
    let history_back_label = gtk4::Label::new(Some(tr("back", active_lang)));
    back_box_h.append(&history_back_label);
    history_back_button.set_child(Some(&back_box_h));

    let stack_clone_hback = stack.clone();
    history_back_button.connect_clicked(move |_| {
        stack_clone_hback.set_visible_child_full("main", gtk4::StackTransitionType::SlideRight);
    });
    hist_top_box.append(&history_back_button);

    let hist_title = gtk4::Label::builder()
        .label(tr("history", active_lang))
        .css_classes(vec!["subpage-heading".to_string()])
        .halign(gtk4::Align::Center)
        .hexpand(true)
        .build();
    hist_top_box.append(&hist_title);

    let dummy_spacer_h = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    dummy_spacer_h.set_width_request(42);
    hist_top_box.append(&dummy_spacer_h);

    page_history.append(&hist_top_box);

    let hist_sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    hist_sep.set_margin_bottom(4);
    page_history.append(&hist_sep);

    let hist_scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .min_content_width(238)
        .max_content_width(238)
        .propagate_natural_width(false)
        .min_content_height(90)
        .max_content_height(230)
        .propagate_natural_height(true)
        .build();

    let hist_content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    hist_content_box.set_height_request(230);

    let history_list_box = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .css_classes(vec!["navigation-sidebar".to_string()])
        .build();

    let history_empty_label = gtk4::Label::builder()
        .label(tr("no_history", active_lang))
        .css_classes(vec!["dim-label".to_string()])
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .vexpand(true)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    hist_content_box.append(&history_empty_label);
    hist_content_box.append(&history_list_box);
    hist_scrolled.set_child(Some(&hist_content_box));
    page_history.append(&hist_scrolled);

    // Wire history button on main page to refresh SQLite history and transition
    let h_lb = history_list_box.clone();
    let h_empty = history_empty_label.clone();
    let h_storage = storage.clone();
    let h_stack = stack.clone();
    let h_lang = active_language.clone();
    history_button.connect_clicked(move |_| {
        refresh_history_list(&h_lb, &h_empty, &h_storage, &h_lang.borrow());
        h_stack.set_visible_child_full("history", gtk4::StackTransitionType::SlideLeft);
    });

    // Initial population of history
    refresh_history_list(
        &history_list_box,
        &history_empty_label,
        storage,
        active_lang,
    );

    // Add named pages to stack
    stack.add_named(&page_main, Some("main"));
    stack.add_named(&page_language, Some("language"));
    stack.add_named(&page_history, Some("history"));
    stack.set_visible_child_name("main");

    // Popover attached to main menu button
    let popover = gtk4::Popover::builder()
        .child(&stack)
        .autohide(true)
        .css_classes(vec!["main-menu-popover".to_string()])
        .build();

    let stack_reset = stack.clone();
    popover.connect_closed(move |_| {
        stack_reset.set_visible_child_full("main", gtk4::StackTransitionType::None);
    });

    let main_menu_button = gtk4::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text(tr("main_menu", active_lang))
        .primary(true)
        .popover(&popover)
        .build();
    main_menu_button.update_property(&[gtk4::accessible::Property::Label(tr(
        "main_menu",
        active_lang,
    ))]);

    // Pack into end of HeaderBar
    header_bar.pack_end(&main_menu_button);

    // Wire About OpenDictate button (reloads live config at click time)
    let popover_about = popover.clone();
    let config_about = config.clone();
    let btn_for_parent = about_button.clone();
    about_button.connect_clicked(move |_| {
        popover_about.popdown();
        let parent_win = btn_for_parent
            .root()
            .and_then(|r| r.downcast::<gtk4::Window>().ok());
        let live_cfg =
            Config::load_from(&Config::default_path()).unwrap_or_else(|_| config_about.clone());
        let about = build_about_dialog(&live_cfg);
        about.present(parent_win.as_ref());
    });

    let widgets = HeaderBarWidgets {
        header_bar: header_bar.clone(),
        title_widget,
        main_menu_button,
        popover,
        stack,
        zoom_out_button,
        zoom_reset_button,
        zoom_in_button,
        zoom_label,
        minibar_scale,
        on_scale_change,
        language_button,
        history_button,
        about_button,
        language_back_button,
        history_back_button,
        language_list_box,
        history_list_box,
        history_empty_label,
        active_language,
        on_language_change,
        storage: storage.clone(),
        language_label: lang_lbl,
        history_label: hist_lbl,
        about_label: about_lbl,
        language_title: lang_title,
        history_title: hist_title,
        language_back_label,
        history_back_label,
        check_imgs,
    };

    (header_bar, widgets)
}

/// Formats a history snippet ensuring at least 3 to 5 full words are shown as a preview.
pub fn format_history_snippet(text: &str) -> String {
    let single_line = text.replace(['\r', '\n'], " ");
    let words: Vec<&str> = single_line.split_whitespace().collect();
    if words.len() > 5 {
        format!("{}…", words[..5].join(" "))
    } else if words.len() >= 3 {
        words.join(" ")
    } else if single_line.chars().count() > 36 {
        let mut s: String = single_line.chars().take(36).collect();
        s.push('…');
        s
    } else {
        words.join(" ")
    }
}

/// Refreshes the dictation history ListBox from the SQLite storage service.
pub fn refresh_history_list(
    list_box: &gtk4::ListBox,
    empty_label: &gtk4::Label,
    storage: &StorageService,
    lang: &str,
) {
    empty_label.set_label(tr("no_history", lang));

    let records = match storage.list_dictations() {
        Ok(recs) => recs,
        Err(e) => {
            log::error!("Failed to list dictations from storage: {}", e);
            Vec::new()
        }
    };

    if records.is_empty() {
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }
        empty_label.set_visible(true);
        list_box.set_visible(false);
        return;
    }

    // Check if the rendered rows already match the current SQLite records and language to avoid
    // unnecessary widget destruction/recreation during popover transitions.
    let expected_names: Vec<String> = records
        .iter()
        .map(|r| format!("hist-{}-{}", r.id, lang))
        .collect();
    let mut existing_names: Vec<String> = Vec::new();
    let mut curr = list_box.first_child();
    while let Some(c) = curr {
        existing_names.push(c.widget_name().to_string());
        curr = c.next_sibling();
    }
    if existing_names == expected_names && list_box.is_visible() && !empty_label.is_visible() {
        return;
    }

    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    empty_label.set_visible(false);
    list_box.set_visible(true);

    for record in records {
        let full_text = if !record.processed_text.trim().is_empty() {
            record.processed_text.clone()
        } else {
            record.raw_text.clone()
        };

        let row = gtk4::ListBoxRow::new();
        row.set_widget_name(&format!("hist-{}-{}", record.id, lang));
        row.set_tooltip_text(Some(&full_text));

        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        row_box.set_margin_start(4);
        row_box.set_margin_end(4);
        row_box.set_margin_top(4);
        row_box.set_margin_bottom(4);

        let info_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        info_box.set_hexpand(true);
        info_box.set_halign(gtk4::Align::Fill);

        let snippet = format_history_snippet(&full_text);
        let snippet_lbl = gtk4::Label::builder()
            .label(&snippet)
            .hexpand(true)
            .halign(gtk4::Align::Fill)
            .xalign(0.0)
            .single_line_mode(true)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .width_chars(20)
            .max_width_chars(23)
            .build();
        info_box.append(&snippet_lbl);

        let time_lbl = gtk4::Label::builder()
            .label(&record.timestamp)
            .halign(gtk4::Align::Fill)
            .xalign(0.0)
            .single_line_mode(true)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .css_classes(vec!["caption".to_string(), "dim-label".to_string()])
            .build();
        info_box.append(&time_lbl);

        row_box.append(&info_box);

        let actions_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 2);
        actions_box.set_halign(gtk4::Align::End);
        actions_box.set_valign(gtk4::Align::Center);

        let copy_btn = gtk4::Button::builder()
            .icon_name("edit-copy-symbolic")
            .tooltip_text(tr("copy", lang))
            .css_classes(vec!["flat".to_string()])
            .build();
        let copy_text = full_text.clone();
        let copy_btn_clone = copy_btn.clone();
        let lang_copied = lang.to_string();
        let lang_copy_reset = lang.to_string();
        copy_btn.connect_clicked(move |_| {
            crate::ui::mini_bar::copy_text_to_clipboard(&copy_text);
            copy_btn_clone.set_tooltip_text(Some(tr("copied", &lang_copied)));
            let reset_btn = copy_btn_clone.clone();
            let reset_lang = lang_copy_reset.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_secs(2), move || {
                reset_btn.set_tooltip_text(Some(tr("copy", &reset_lang)));
            });
        });
        actions_box.append(&copy_btn);

        let delete_btn = gtk4::Button::builder()
            .icon_name("user-trash-symbolic")
            .tooltip_text(tr("delete", lang))
            .css_classes(vec!["flat".to_string()])
            .build();
        let storage_clone = storage.clone();
        let list_box_weak = list_box.downgrade();
        let row_weak = row.downgrade();
        let empty_label_clone = empty_label.clone();
        let record_id = record.id;
        delete_btn.connect_clicked(move |_| {
            if let Err(e) = storage_clone.delete_dictation(record_id) {
                log::error!("Failed to delete dictation {}: {}", record_id, e);
                return;
            }
            if let (Some(lb), Some(r)) = (list_box_weak.upgrade(), row_weak.upgrade()) {
                lb.remove(&r);
                if lb.first_child().is_none() {
                    empty_label_clone.set_visible(true);
                    lb.set_visible(false);
                }
            }
        });
        actions_box.append(&delete_btn);

        row_box.append(&actions_box);
        row.set_child(Some(&row_box));
        list_box.append(&row);
    }
}

/// Replaces any user-specific home directory prefix (e.g., `/home/username`) with `~`
/// so diagnostic reports are privacy-safe and portable across all users' machines.
fn anonymize_home_path(path: &std::path::Path) -> String {
    let raw = path.to_string_lossy().to_string();
    if let Some(base_dirs) = directories::BaseDirs::new() {
        let home = base_dirs.home_dir().to_string_lossy();
        if !home.is_empty() && raw.starts_with(home.as_ref()) {
            return format!("~{}", &raw[home.len()..]);
        }
    }
    if let Ok(home) = std::env::var("HOME")
        && !home.is_empty()
        && raw.starts_with(&home)
    {
        return format!("~{}", &raw[home.len()..]);
    }
    raw
}

/// Dynamically probes the current user's machine at runtime to generate a portable,
/// privacy-safe troubleshooting diagnostics report suitable for bug reports on any Linux system.
pub fn generate_troubleshooting_diagnostics(config: &Config) -> String {
    let os_info = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return Some(
                        line.trim_start_matches("PRETTY_NAME=")
                            .trim_matches('"')
                            .to_string(),
                    );
                }
            }
            None
        })
        .unwrap_or_else(|| std::env::consts::OS.to_string());

    let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let cpu_model = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|content| {
            for line in content.lines() {
                if (line.starts_with("model name") || line.starts_with("Model"))
                    && let Some((_, val)) = line.split_once(':')
                {
                    return Some(val.trim().to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| std::env::consts::ARCH.to_string());

    let ram_summary = std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|content| {
            let mut total_kb: Option<u64> = None;
            let mut avail_kb: Option<u64> = None;
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total_kb = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<u64>().ok());
                } else if line.starts_with("MemAvailable:") {
                    avail_kb = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<u64>().ok());
                }
            }
            match (total_kb, avail_kb) {
                (Some(t), Some(a)) => Some(format!(
                    "{:.1} GiB available / {:.1} GiB total",
                    a as f64 / 1_048_576.0,
                    t as f64 / 1_048_576.0
                )),
                (Some(t), None) => Some(format!("{:.1} GiB total", t as f64 / 1_048_576.0)),
                _ => None,
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    let arch = std::env::consts::ARCH;
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string());
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".to_string());

    let package_runtime =
        if std::env::var("FLATPAK_ID").is_ok() || std::path::Path::new("/.flatpak-info").exists() {
            "Flatpak Sandbox"
        } else if std::env::var("SNAP_NAME").is_ok() {
            "Snap Package"
        } else if std::env::var("APPIMAGE").is_ok() {
            "AppImage"
        } else {
            "Native Host Binary"
        };

    let audio_backend = if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let rt = std::path::Path::new(&runtime_dir);
        if rt.join("pipewire-0").exists() {
            "PipeWire"
        } else if rt.join("pulse/native").exists() {
            "PulseAudio"
        } else {
            "ALSA"
        }
    } else {
        "ALSA / System Default"
    };

    let (gtk_ver, adw_ver) = if gtk4::is_initialized_main_thread() {
        (
            format!(
                "{}.{}.{}",
                gtk4::major_version(),
                gtk4::minor_version(),
                gtk4::micro_version()
            ),
            format!(
                "{}.{}.{}",
                libadwaita::major_version(),
                libadwaita::minor_version(),
                libadwaita::micro_version()
            ),
        )
    } else {
        ("4.x".to_string(), "1.x".to_string())
    };

    let cpu_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let data_dir = directories::BaseDirs::new()
        .map(|b| b.data_local_dir().to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share"))
        .join("opendictate");
    let model_path = if let Some(ref custom) = config.local_custom_path {
        std::path::PathBuf::from(custom)
    } else {
        data_dir
            .join("models")
            .join(format!("ggml-{}.bin", config.local_model_id))
    };

    let portable_model_path = anonymize_home_path(&model_path);
    let model_file_status = match std::fs::metadata(&model_path) {
        Ok(meta) => format!(
            "Installed ({:.1} MB at {})",
            meta.len() as f64 / (1024.0 * 1024.0),
            portable_model_path
        ),
        Err(_) => format!("Not installed ({})", portable_model_path),
    };

    let api_key_status = if config.ai_api_key.trim().is_empty() {
        "Not set"
    } else {
        "Configured (redacted)"
    };

    let cloud_model = if config.ai_model.trim().is_empty() {
        "Default"
    } else {
        &config.ai_model
    };

    let audio_dev = config
        .audio_device
        .clone()
        .unwrap_or_else(|| "System Default".to_string());
    let detected_audio_profile = crate::audio::AudioRecorder::detect_connected_audio_profile();
    let detected_inputs = match crate::audio::AudioRecorder::list_input_devices() {
        Ok(devs) if !devs.is_empty() => {
            let preview: Vec<String> = devs.iter().take(6).cloned().collect();
            format!("{} detected ({})", devs.len(), preview.join(", "))
        }
        Ok(_) => "0 input devices detected (no microphone available)".to_string(),
        Err(e) => format!("Audio device enumeration error: {}", e),
    };
    let recent_events = crate::services::crash_reporter::CrashReporter::recent_events_summary();
    let ai_mode = if config.ai_mode == "local" {
        "Local AI"
    } else {
        "Cloud AI"
    };
    let hotkey = if config.hotkey.is_empty() {
        "Ctrl+Shift+Space"
    } else {
        &config.hotkey
    };
    let portable_cfg_path = anonymize_home_path(&Config::default_path());
    let portable_data_dir = anonymize_home_path(&data_dir);

    format!(
        "OpenDictate {ver} ({arch} • {pkg})\n\
         OS: {os} (Kernel {kernel})\n\
         Session: {session} ({desktop})\n\
         Hardware: {cpu} ({cores} logical cores) | RAM: {ram}\n\
         Toolkit: GTK {gtk_ver} / Libadwaita {adw_ver}\n\
         Theme: {theme} | UI Language: {lang} | MiniBar Scale: {scale}%\n\
         \n\
         [AI Engine]\n\
         Active Mode: {ai_mode}\n\
         Cloud Provider: {provider} | Model: {cloud_model} | API Key: {api_key}\n\
         Local Model ID: {local_id}\n\
         Local Model Status: {model_status}\n\
         Inference Threads: {threads}\n\
         \n\
         [Audio & Input]\n\
         Audio Server: {audio_srv}\n\
         Microphone: {audio_dev} (Auto Profile: {audio_profile} • 16 kHz Mono PCM)\n\
         Detected Input Devices: {detected_inputs}\n\
         Global Hotkey: {hotkey}\n\
         Tone Preset: {tone}\n\
         \n\
         [Subsystem Health & Recent Events]\n\
         {recent_events}\n\
         \n\
         [Paths]\n\
         Config: {cfg_path}\n\
         Data Dir: {data_dir}\n",
        ver = env!("CARGO_PKG_VERSION"),
        arch = arch,
        pkg = package_runtime,
        os = os_info,
        kernel = kernel,
        session = session_type,
        desktop = desktop,
        cpu = cpu_model,
        cores = cpu_cores,
        ram = ram_summary,
        gtk_ver = gtk_ver,
        adw_ver = adw_ver,
        theme = config.theme,
        lang = config.ui_language,
        scale = config.minibar_scale,
        ai_mode = ai_mode,
        provider = config.ai_provider,
        cloud_model = cloud_model,
        api_key = api_key_status,
        local_id = config.local_model_id,
        model_status = model_file_status,
        threads = config.local_threads,
        audio_srv = audio_backend,
        audio_dev = audio_dev,
        audio_profile = detected_audio_profile,
        detected_inputs = detected_inputs,
        hotkey = hotkey,
        tone = config.tone,
        recent_events = recent_events,
        cfg_path = portable_cfg_path,
        data_dir = portable_data_dir,
    )
}

/// Default downloadable Local AI model names and full URLs.
pub const DEFAULT_LOCAL_MODEL_LINKS: &[(&str, &str)] = &[
    (
        "Tiny (English)",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin",
    ),
    (
        "Tiny (Multilingual)",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
    ),
    (
        "Base (English)",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin",
    ),
    (
        "Base (Multilingual)",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
    ),
    (
        "Small (Multilingual)",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
    ),
];

/// Constructs the native Libadwaita About dialog.
pub fn build_about_dialog(config: &Config) -> libadwaita::AboutDialog {
    crate::ui::theme::ensure_app_icons_registered();
    ensure_main_menu_css();
    let mut debug_info = generate_troubleshooting_diagnostics(config);
    if let Some(crash_summary) =
        crate::services::crash_reporter::CrashReporter::load_latest_crash_summary()
    {
        debug_info.push_str("\n[Recent Crash Report]\n");
        debug_info.push_str(&crash_summary);
        debug_info.push('\n');
    }
    let app_icon = crate::ui::theme::ThemeMode::from_str(&config.theme).icon_name();

    let comments_text = "OpenDictate is a native Linux voice dictation and AI speech-to-text application. It combines private offline transcription with multi-provider Cloud AI models to turn spoken voice into clean, polished text.";

    let dialog = libadwaita::AboutDialog::builder()
        .application_name("OpenDictate")
        .application_icon(app_icon)
        .version("2.0.0")
        .developer_name("OpenDictate Team")
        .issue_url("https://github.com/hkumarsaikia/OpenDictate/issues")
        .copyright("© 2026 OpenDictate Team")
        .license_type(gtk4::License::MitX11)
        .comments(comments_text)
        .designers(vec!["H. K. Saikia"])
        .debug_info(&debug_info)
        .debug_info_filename("opendictate-diagnostics.txt")
        .build();

    // 1. Minimal legal section for whisper.cpp
    dialog.add_legal_section(
        "whisper.cpp",
        Some("© Georgi Gerganov"),
        gtk4::License::MitX11,
        None,
    );

    // 2. Minimal legal section for OpenAI Whisper models
    dialog.add_legal_section(
        "OpenAI Whisper Models",
        Some("© OpenAI"),
        gtk4::License::MitX11,
        None,
    );

    // 3. Core technologies acknowledgments
    dialog.add_acknowledgement_section(
        Some("Core Open Source Technologies"),
        &[
            "whisper.cpp https://github.com/ggerganov/whisper.cpp",
            "Relm4 GUI Framework https://relm4.org",
            "GTK4 & Libadwaita https://gitlab.gnome.org/GNOME/libadwaita",
            "CPAL Audio Engine https://github.com/RustAudio/cpal",
            "OpenAI Whisper https://github.com/openai/whisper",
        ],
    );

    // 4. Inject full links for the 5 default downloadable Local AI models into the Details subpage (replacing the Website button)
    inject_details_model_links(&dialog);

    // 5. Inject "Buy me a coffee" option below "Acknowledgements" on the About main page
    inject_buy_me_a_coffee_row(&dialog);

    dialog
}

/// Collects all `libadwaita::ActionRow`s whose title matches `target_title` (ignoring `_` mnemonics and case).
fn find_all_action_rows_by_title(
    root: &gtk4::Widget,
    target_title: &str,
    out: &mut Vec<libadwaita::ActionRow>,
) {
    if let Some(row) = root.downcast_ref::<libadwaita::ActionRow>() {
        let clean_title = row.title().replace('_', "").to_lowercase();
        if clean_title.contains(&target_title.to_lowercase()) {
            out.push(row.clone());
        }
    }
    let mut child = root.first_child();
    while let Some(c) = child {
        find_all_action_rows_by_title(&c, target_title, out);
        child = c.next_sibling();
    }
}

/// Recursively searches a widget tree for a `libadwaita::ActionRow` whose title matches `target_title`
/// (ignoring mnemonic underscores and case).
fn find_action_row_by_title(
    root: &gtk4::Widget,
    target_title: &str,
) -> Option<libadwaita::ActionRow> {
    let mut rows = Vec::new();
    find_all_action_rows_by_title(root, target_title, &mut rows);
    rows.into_iter().next()
}

/// Injects the 5 default downloadable Local AI model full URLs into the About -> Details subpage
/// in place of the removed "Website" button.
fn inject_details_model_links(dialog: &libadwaita::AboutDialog) {
    let root_widget = dialog
        .child()
        .unwrap_or_else(|| dialog.clone().upcast::<gtk4::Widget>());

    let mut website_rows = Vec::new();
    find_all_action_rows_by_title(&root_widget, "Website", &mut website_rows);

    // Ensure all Website rows remain hidden
    for w_row in &website_rows {
        w_row.set_visible(false);
    }

    // The second Website row lives inside the Details subpage's AdwPreferencesGroup -> GtkListBox
    if let Some(details_web_row) = website_rows.last()
        && let Some(list_box) = details_web_row
            .parent()
            .and_then(|p| p.downcast::<gtk4::ListBox>().ok())
    {
        list_box.set_visible(true);
        // Make sure ancestor AdwPreferencesGroup is visible and titled
        let mut anc = list_box.parent();
        while let Some(a) = anc {
            if let Some(group) = a.downcast_ref::<libadwaita::PreferencesGroup>() {
                group.set_title("Default Local AI Models");
                group.set_visible(true);
                break;
            }
            anc = a.parent();
        }

        for (model_name, model_url) in DEFAULT_LOCAL_MODEL_LINKS {
            let row = libadwaita::ActionRow::builder()
                .title(*model_name)
                .subtitle(*model_url)
                .subtitle_lines(1)
                .activatable(true)
                .build();
            let ext_icon = gtk4::Image::from_icon_name("adw-external-link-symbolic");
            row.add_suffix(&ext_icon);

            let url_str = (*model_url).to_string();
            row.connect_activated(move |r| {
                let parent_win = r.root().and_then(|rt| rt.downcast::<gtk4::Window>().ok());
                open_external_url(&url_str, parent_win.as_ref());
            });

            list_box.append(&row);
        }
    }
}

/// Recursively finds the last `gtk4::ListBox` with `.boxed-list` CSS class in the main page of `AboutDialog`.
fn find_boxed_list_boxes(root: &gtk4::Widget, out: &mut Vec<gtk4::ListBox>) {
    if let Some(lb) = root.downcast_ref::<gtk4::ListBox>()
        && lb.has_css_class("boxed-list")
    {
        out.push(lb.clone());
    }
    let mut child = root.first_child();
    while let Some(c) = child {
        find_boxed_list_boxes(&c, out);
        child = c.next_sibling();
    }
}

/// Injects the "Buy me a coffee" row directly below "Acknowledgements" on the About main page.
fn inject_buy_me_a_coffee_row(dialog: &libadwaita::AboutDialog) {
    let root_widget = dialog
        .child()
        .unwrap_or_else(|| dialog.clone().upcast::<gtk4::Widget>());

    let coffee_row = libadwaita::ActionRow::builder()
        .title("Buy me a coffee")
        .activatable(true)
        .build();
    let arrow = gtk4::Image::from_icon_name("go-next-symbolic");
    coffee_row.add_suffix(&arrow);

    let dialog_weak = dialog.downgrade();
    coffee_row.connect_activated(move |_| {
        if let Some(dlg) = dialog_weak.upgrade() {
            let coffee_dlg = build_buy_me_a_coffee_dialog();
            coffee_dlg.present(Some(&dlg));
        }
    });

    let target_row = find_action_row_by_title(&root_widget, "Acknowledgements")
        .or_else(|| find_action_row_by_title(&root_widget, "Legal"))
        .or_else(|| find_action_row_by_title(&root_widget, "Credits"));

    if let Some(ack_row) = target_row
        && let Some(parent) = ack_row.parent()
    {
        if let Some(list_box) = parent.downcast_ref::<gtk4::ListBox>() {
            list_box.append(&coffee_row);
            return;
        } else if let Some(group) = parent.downcast_ref::<libadwaita::PreferencesGroup>() {
            group.add(&coffee_row);
            return;
        } else if let Some(pbox) = parent.downcast_ref::<gtk4::Box>() {
            pbox.append(&coffee_row);
            return;
        }
    }

    let mut list_boxes = Vec::new();
    find_boxed_list_boxes(&root_widget, &mut list_boxes);
    if let Some(last_lb) = list_boxes.last() {
        last_lb.append(&coffee_row);
    } else if let Some(child) = dialog.child()
        && let Some(pbox) = child.downcast_ref::<gtk4::Box>()
    {
        pbox.append(&coffee_row);
    }
}

/// Constructs and returns the aesthetic, modern "Buy Me a Coffee" dialog with
/// $2, $5, $10, $20 presets and a "$ Custom amount" entry.
pub fn build_buy_me_a_coffee_dialog() -> libadwaita::Dialog {
    ensure_main_menu_css();

    let dialog = libadwaita::Dialog::builder()
        .title("Buy Me a Coffee")
        .content_width(420)
        .content_height(410)
        .build();

    let toolbar_view = libadwaita::ToolbarView::new();
    let header = libadwaita::HeaderBar::new();
    toolbar_view.add_top_bar(&header);

    let main_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(16)
        .margin_top(16)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // Hero section (clean, minimal — without extra explanatory sentences)
    let hero_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(6)
        .halign(gtk4::Align::Center)
        .build();

    let coffee_icon = gtk4::Label::builder()
        .label("☕")
        .css_classes(vec!["title-1".to_string(), "coffee-badge-icon".to_string()])
        .build();
    hero_box.append(&coffee_icon);

    let hero_title = gtk4::Label::builder()
        .label("Support OpenDictate")
        .css_classes(vec!["title-2".to_string()])
        .halign(gtk4::Align::Center)
        .build();
    hero_box.append(&hero_title);

    main_box.append(&hero_box);

    // Preset dollar tiers ($2, $5, $10, $20) in a 2x2 Grid
    let grid = gtk4::Grid::builder()
        .row_spacing(10)
        .column_spacing(10)
        .column_homogeneous(true)
        .build();

    let tiers: [(u32, &str, &str); 4] = [
        (2, "$2", "Espresso Shot"),
        (5, "$5", "Classic Coffee"),
        (10, "$10", "Double Latte"),
        (20, "$20", "Bag of Beans"),
    ];

    let selected_amount = Rc::new(RefCell::new("5".to_string()));
    let mut tier_buttons: Vec<(u32, gtk4::ToggleButton)> = Vec::with_capacity(4);

    for (idx, (amt, price_str, desc_str)) in tiers.iter().enumerate() {
        let btn_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(2)
            .halign(gtk4::Align::Center)
            .build();

        let price_lbl = gtk4::Label::builder()
            .label(*price_str)
            .css_classes(vec!["title-3".to_string()])
            .build();
        let desc_lbl = gtk4::Label::builder()
            .label(*desc_str)
            .css_classes(vec!["caption".to_string(), "dim-label".to_string()])
            .build();

        btn_box.append(&price_lbl);
        btn_box.append(&desc_lbl);

        let toggle = gtk4::ToggleButton::builder()
            .child(&btn_box)
            .active(*amt == 5)
            .css_classes(vec!["coffee-tier-btn".to_string()])
            .build();

        if let Some((_, first_btn)) = tier_buttons.first() {
            toggle.set_group(Some(first_btn));
        }

        let col = (idx % 2) as i32;
        let row = (idx / 2) as i32;
        grid.attach(&toggle, col, row, 1, 1);
        tier_buttons.push((*amt, toggle));
    }

    main_box.append(&grid);

    // "$ Custom amount" using AdwPreferencesGroup + AdwEntryRow
    let custom_group = libadwaita::PreferencesGroup::new();
    let custom_entry = libadwaita::EntryRow::builder()
        .title("Custom amount")
        .build();
    let dollar_prefix = gtk4::Label::builder()
        .label("$")
        .css_classes(vec!["title-4".to_string(), "dim-label".to_string()])
        .margin_start(8)
        .build();
    custom_entry.add_prefix(&dollar_prefix);
    custom_group.add(&custom_entry);
    main_box.append(&custom_group);

    // Primary Support CTA button
    let support_btn = gtk4::Button::builder()
        .label("Support with $5 ☕")
        .css_classes(vec!["suggested-action".to_string(), "pill".to_string()])
        .halign(gtk4::Align::Fill)
        .margin_top(4)
        .build();
    main_box.append(&support_btn);

    // Wire preset toggle buttons to update CTA label and clear custom entry
    let is_updating_from_tier = Rc::new(RefCell::new(false));
    for (amt, toggle) in &tier_buttons {
        let amt_val = *amt;
        let sel_amt = selected_amount.clone();
        let cta_btn = support_btn.clone();
        let cust_entry = custom_entry.clone();
        let guard = is_updating_from_tier.clone();
        toggle.connect_toggled(move |btn| {
            if btn.is_active() {
                *guard.borrow_mut() = true;
                cust_entry.set_text("");
                *guard.borrow_mut() = false;
                *sel_amt.borrow_mut() = amt_val.to_string();
                cta_btn.set_label(&format!("Support with ${} ☕", amt_val));
            }
        });
    }

    // Wire custom amount entry to update CTA label dynamically
    {
        let sel_amt = selected_amount.clone();
        let cta_btn = support_btn.clone();
        let toggles: Vec<gtk4::ToggleButton> =
            tier_buttons.iter().map(|(_, b)| b.clone()).collect();
        let guard = is_updating_from_tier.clone();
        custom_entry.connect_changed(move |entry| {
            if *guard.borrow() {
                return;
            }
            let raw = entry.text();
            let cleaned: String = raw
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if !cleaned.is_empty() {
                for t in &toggles {
                    t.set_active(false);
                }
                *sel_amt.borrow_mut() = cleaned.clone();
                cta_btn.set_label(&format!("Support with ${} ☕", cleaned));
            }
        });
    }

    // Wire Support button click to open external sponsor/coffee URL
    {
        let sel_amt = selected_amount.clone();
        let dlg_weak = dialog.downgrade();
        support_btn.connect_clicked(move |btn| {
            let amt = sel_amt.borrow().clone();
            let parent_win = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            let url = format!("https://github.com/sponsors/hkumarsaikia?amount={}", amt);
            open_external_url(&url, parent_win.as_ref());
            if let Some(d) = dlg_weak.upgrade() {
                d.close();
            }
        });
    }

    toolbar_view.set_content(Some(&main_box));
    dialog.set_child(Some(&toolbar_view));
    dialog
}

/// Displays the Troubleshooting Dialog with system diagnostics and copy button.
pub fn show_troubleshooting_dialog(parent: &impl IsA<gtk4::Widget>, config: &Config) {
    let dialog = libadwaita::Dialog::builder()
        .title("Troubleshooting & Diagnostics")
        .content_width(560)
        .content_height(480)
        .build();

    let vbox = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();

    let header_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(8)
        .build();

    let title_lbl = gtk4::Label::builder()
        .label("System & Audio Diagnostics")
        .css_classes(vec!["title-3".to_string()])
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .build();
    header_box.append(&title_lbl);

    let copy_btn = gtk4::Button::builder()
        .label("Copy Diagnostics")
        .icon_name("edit-copy-symbolic")
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    header_box.append(&copy_btn);

    vbox.append(&header_box);

    let scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .css_classes(vec!["card".to_string()])
        .build();

    let text_view = gtk4::TextView::builder()
        .editable(false)
        .monospace(true)
        .left_margin(12)
        .right_margin(12)
        .top_margin(12)
        .bottom_margin(12)
        .wrap_mode(gtk4::WrapMode::Word)
        .build();

    let diag_text = generate_troubleshooting_diagnostics(config);
    text_view.buffer().set_text(&diag_text);
    scrolled.set_child(Some(&text_view));
    vbox.append(&scrolled);

    let diag_for_copy = diag_text.clone();
    let copy_btn_clone = copy_btn.clone();
    copy_btn.connect_clicked(move |_| {
        let display = gtk4::gdk::Display::default();
        if let Some(disp) = display {
            disp.clipboard().set_text(&diag_for_copy);
            copy_btn_clone.set_label("Copied!");
            let btn_reset = copy_btn_clone.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_secs(2), move || {
                btn_reset.set_label("Copy Diagnostics");
            });
        }
    });

    let footer_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(8)
        .halign(gtk4::Align::End)
        .build();

    let issue_btn = gtk4::Button::builder()
        .label("Report an Issue on GitHub")
        .icon_name("system-help-symbolic")
        .build();
    let parent_win = parent
        .as_ref()
        .root()
        .and_then(|r| r.downcast::<gtk4::Window>().ok());
    issue_btn.connect_clicked(move |_| {
        open_external_url(
            "https://github.com/hkumarsaikia/OpenDictate/issues",
            parent_win.as_ref(),
        );
    });
    footer_box.append(&issue_btn);

    vbox.append(&footer_box);
    dialog.set_child(Some(&vbox));
    dialog.present(Some(parent));
}

/// Displays the Credits Dialog.
pub fn show_credits_dialog(parent: &impl IsA<gtk4::Widget>) {
    let dialog = libadwaita::Dialog::builder()
        .title("Credits")
        .content_width(500)
        .content_height(400)
        .build();

    let vbox = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(16)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let title_lbl = gtk4::Label::builder()
        .label("Credits & Contributors")
        .css_classes(vec!["title-2".to_string()])
        .halign(gtk4::Align::Center)
        .build();
    vbox.append(&title_lbl);

    let credits_text = "\
Authors & Maintainers:
• H. K. Saikia — Lead Architect & Maintainer
• OpenDictate Open Source Community

Core Technologies:
• Georgi Gerganov & whisper.cpp team — C++ Whisper engine
• OpenAI Whisper Research Team — Acoustic speech models
• Aaron Erhardt & Relm4 Contributors — Modern Elm-style GTK4 GUI
• GNOME Project & Libadwaita Team — Modern Linux HIG & Adwaita styling
• RustAudio Community — CPAL (Cross-Platform Audio Library)
• hound & rubato contributors — WAV audio processing & resamplers";

    let label = gtk4::Label::builder()
        .label(credits_text)
        .wrap(true)
        .xalign(0.0)
        .vexpand(true)
        .css_classes(vec!["card".to_string(), "body".to_string()])
        .margin_top(8)
        .margin_bottom(8)
        .build();
    vbox.append(&label);

    dialog.set_child(Some(&vbox));
    dialog.present(Some(parent));
}

/// Displays the Legal Dialog.
pub fn show_legal_dialog(parent: &impl IsA<gtk4::Widget>) {
    let dialog = libadwaita::Dialog::builder()
        .title("Legal & Privacy Architecture")
        .content_width(520)
        .content_height(440)
        .build();

    let vbox = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(16)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let title_lbl = gtk4::Label::builder()
        .label("Legal & Privacy Terms")
        .css_classes(vec!["title-2".to_string()])
        .halign(gtk4::Align::Center)
        .build();
    vbox.append(&title_lbl);

    let legal_text = "\
OpenDictate License:
OpenDictate is released under the MIT Open Source License.

On-Device Privacy Architecture:
When Local AI mode is active, 100% of audio processing, transcription, \
and text enhancement is executed locally on your CPU/machine. Zero audio samples, \
metadata, or transcripts are ever uploaded, recorded, or transmitted over any network.

Third-Party Components & Models:
• whisper.cpp is licensed under the MIT License (© Georgi Gerganov).
• OpenAI Whisper speech model weights are licensed under the MIT License (© OpenAI).
• GTK4 and Libadwaita are licensed under the GNU LGPL-2.1+ (© GNOME Foundation).";

    let label = gtk4::Label::builder()
        .label(legal_text)
        .wrap(true)
        .xalign(0.0)
        .vexpand(true)
        .css_classes(vec!["card".to_string(), "body".to_string()])
        .margin_top(8)
        .margin_bottom(8)
        .build();
    vbox.append(&label);

    dialog.set_child(Some(&vbox));
    dialog.present(Some(parent));
}

/// Launch an external web URL cleanly without leaking any browser stderr/stdout into the terminal.
///
/// First attempts to use GTK 4.10+ `UriLauncher` (which offloads to `xdg-desktop-portal`
/// `org.freedesktop.portal.OpenURI` via D-Bus, isolating process stdout/stderr from the app's terminal).
/// If that fails or if no parent window is available, falls back to spawning `xdg-open`
/// in a completely detached process group with all standard I/O (stdin, stdout, stderr)
/// redirected to `/dev/null`.
pub fn open_external_url(uri: &str, parent_window: Option<&gtk4::Window>) {
    let uri_string = uri.to_string();
    let launcher = gtk4::UriLauncher::new(&uri_string);
    let fallback_uri = uri_string.clone();

    launcher.launch(parent_window, gtk4::gio::Cancellable::NONE, move |result| {
        if let Err(err) = result {
            log::warn!(
                "GtkUriLauncher could not open '{}': {}. Falling back to detached process.",
                fallback_uri,
                err
            );
            spawn_detached_browser(&fallback_uri);
        }
    });
}

/// Fallback executor that spawns a fully detached browser process with stdio redirected to `/dev/null`.
pub fn spawn_detached_browser(url: &str) {
    let mut cmd = Command::new("xdg-open");
    cmd.arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    match cmd.spawn() {
        Ok(mut child) => {
            // Reap the child asynchronously in a detached thread to prevent zombie processes.
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        }
        Err(err) => {
            log::error!("Failed to launch detached browser for '{}': {}", url, err);
        }
    }
}
