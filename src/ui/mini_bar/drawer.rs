//! Expandable text preview drawer for OpenDictate MiniBar.
//!
//! Provides a slide-down revealer showing real-time transcription, word count,
//! editable text view, and one-click copy to system clipboard.

use gtk4::prelude::*;

/// Widgets contained in the preview drawer.
#[derive(Clone, Debug)]
pub struct PreviewDrawerWidgets {
    pub revealer: gtk4::Revealer,
    pub status_badge: gtk4::Label,
    pub word_count_badge: gtk4::Label,
    pub word_count_label: gtk4::Label,
    pub text_label: gtk4::Label,
    pub text_view: gtk4::TextView,
    pub text_buffer: gtk4::TextBuffer,
    pub placeholder_label: gtk4::Label,
    pub clear_button: gtk4::Button,
    pub copy_button: gtk4::Button,
}

impl PreviewDrawerWidgets {
    /// Updates the displayed transcription text and synchronizes the word count badge.
    pub fn set_text(&self, text: &str) {
        self.text_buffer.set_text(text);
    }

    /// Sets the status text displayed in the status badge.
    pub fn set_status(&self, status: &str) {
        self.status_badge.set_text(status);
    }

    /// Clears the preview text and resets word count and displays placeholder.
    pub fn clear(&self) {
        self.text_buffer.set_text("");
    }

    /// Returns the currently displayed preview text.
    pub fn text(&self) -> String {
        let start = self.text_buffer.start_iter();
        let end = self.text_buffer.end_iter();
        self.text_buffer.text(&start, &end, false).to_string()
    }

    /// Expands or collapses the preview drawer revealer.
    pub fn set_expanded(&self, expanded: bool) {
        self.revealer.set_reveal_child(expanded);
    }

    /// Returns true if the preview drawer is expanded or animating open.
    pub fn is_expanded(&self) -> bool {
        self.revealer.reveals_child()
    }
}

/// Calculates the number of words in a text string.
pub fn calculate_word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Formats a word count into a human-readable string (e.g., "1 word", "12 words").
pub fn format_word_count(count: usize) -> String {
    if count == 1 {
        "1 word".to_string()
    } else {
        format!("{} words", count)
    }
}

/// Constructs the preview drawer revealer and associated widgets.
pub fn build_preview_drawer() -> (gtk4::Revealer, PreviewDrawerWidgets) {
    let revealer = gtk4::Revealer::new();
    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideDown);
    revealer.set_reveal_child(false);
    revealer.set_margin_top(6);

    // Card chassis container
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    card.add_css_class("card");
    card.add_css_class("minibar-drawer");
    card.set_margin_start(2);
    card.set_margin_end(2);
    card.set_margin_bottom(2);

    // Header metadata row
    let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    header_row.set_valign(gtk4::Align::Center);

    // Status badge (e.g., "Ready", "Recording…", "Paused", "Processing…")
    let status_badge = gtk4::Label::new(Some("Ready"));
    status_badge.add_css_class("caption");
    status_badge.add_css_class("dim-label");
    status_badge.set_halign(gtk4::Align::Start);
    header_row.append(&status_badge);

    // Spacer
    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    // Word count badge / label
    let word_count_label = gtk4::Label::new(Some("0 words"));
    word_count_label.add_css_class("caption");
    word_count_label.add_css_class("dim-label");
    header_row.append(&word_count_label);

    // Clear preview text button
    let clear_button = gtk4::Button::from_icon_name("edit-clear-symbolic");
    clear_button.add_css_class("flat");
    clear_button.set_tooltip_text(Some("Clear preview text"));
    header_row.append(&clear_button);

    // Copy to clipboard button
    let copy_button = gtk4::Button::from_icon_name("edit-copy-symbolic");
    copy_button.add_css_class("flat");
    copy_button.set_tooltip_text(Some("Copy to clipboard"));
    header_row.append(&copy_button);

    card.append(&header_row);

    // Text content container inside a styled text box card
    let text_box_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    text_box_card.add_css_class("minibar-drawer-textbox");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_min_content_height(60);
    scrolled.set_max_content_height(140);
    scrolled.set_propagate_natural_height(true);
    scrolled.set_hscrollbar_policy(gtk4::PolicyType::Never);
    scrolled.set_vscrollbar_policy(gtk4::PolicyType::Automatic);

    let text_buffer = gtk4::TextBuffer::new(None::<&gtk4::TextTagTable>);
    let text_view = gtk4::TextView::with_buffer(&text_buffer);
    text_view.set_editable(true);
    text_view.set_cursor_visible(true);
    text_view.set_wrap_mode(gtk4::WrapMode::WordChar);
    text_view.set_accepts_tab(false);
    text_view.add_css_class("body");
    text_view.add_css_class("minibar-drawer-editor");
    text_view.set_margin_start(4);
    text_view.set_margin_end(4);
    text_view.set_margin_top(2);
    text_view.set_margin_bottom(2);

    let placeholder_label = gtk4::Label::new(Some("Dictated text will appear here…"));
    placeholder_label.add_css_class("dim-label");
    placeholder_label.add_css_class("placeholder-text");
    placeholder_label.set_can_target(false);
    placeholder_label.set_halign(gtk4::Align::Start);
    placeholder_label.set_valign(gtk4::Align::Start);
    placeholder_label.set_margin_start(6);
    placeholder_label.set_margin_top(4);

    let overlay = gtk4::Overlay::new();
    overlay.set_child(Some(&text_view));
    overlay.add_overlay(&placeholder_label);

    scrolled.set_child(Some(&overlay));
    text_box_card.append(&scrolled);
    card.append(&text_box_card);

    revealer.set_child(Some(&card));

    // Connect text buffer changed signal to toggle placeholder and recalculate word count dynamically
    let pl_clone = placeholder_label.clone();
    let wc_label_clone = word_count_label.clone();
    text_buffer.connect_changed(move |buf| {
        let start = buf.start_iter();
        let end = buf.end_iter();
        let text = buf.text(&start, &end, false);
        let trimmed = text.trim();
        pl_clone.set_visible(trimmed.is_empty());
        let count = calculate_word_count(&text);
        let formatted = format_word_count(count);
        wc_label_clone.set_text(&formatted);
    });

    // Connect clear button
    let tb_clear = text_buffer.clone();
    clear_button.connect_clicked(move |_| {
        tb_clear.set_text("");
    });

    // Connect copy button to arboard clipboard
    let tb_copy = text_buffer.clone();
    copy_button.connect_clicked(move |_| {
        let start = tb_copy.start_iter();
        let end = tb_copy.end_iter();
        let text = tb_copy.text(&start, &end, false).to_string();
        if !text.trim().is_empty()
            && let Ok(mut clip) = arboard::Clipboard::new()
        {
            let _ = clip.set_text(text);
        }
    });

    let widgets = PreviewDrawerWidgets {
        revealer: revealer.clone(),
        status_badge,
        word_count_badge: word_count_label.clone(),
        word_count_label,
        text_label: placeholder_label.clone(),
        text_view,
        text_buffer,
        placeholder_label,
        clear_button,
        copy_button,
    };

    (revealer, widgets)
}
