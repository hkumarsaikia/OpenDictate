//! Tests for Main Menu in-popover sliding stack navigation, 25-language picker, history viewer,
//! MiniBar zoom controls, and About / Sponsor on GitHub dialogs.

use gtk4::prelude::*;
use libadwaita::prelude::*;
use opendictate::config::Config;
use opendictate::services::i18n::get_languages;
use opendictate::services::storage::StorageService;
use opendictate::ui::main_window::header::{
    MAX_SPONSOR_AMOUNT, MIN_SPONSOR_AMOUNT, build_about_dialog, build_github_sponsors_url,
    build_header_bar, build_sponsor_dialog, format_history_snippet, parse_custom_sponsor_amount,
};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_header_bar_popover_structure() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (header_bar, widgets) = build_header_bar(&config, &storage);

        assert!(header_bar.is::<libadwaita::HeaderBar>());

        // Verify popover attached to main menu button
        let popover = widgets.main_menu_button.popover();
        assert!(
            popover.is_some(),
            "Main menu button must have a Popover attached"
        );

        let popover = popover.unwrap();
        assert_eq!(popover, widgets.popover);

        // Stack must be the popover child
        let stack = widgets.stack.clone();
        assert_eq!(
            stack.transition_type(),
            gtk4::StackTransitionType::SlideLeftRight,
            "Stack transition must be SlideLeftRight"
        );
        assert!(
            !stack.is_vhomogeneous(),
            "Stack must not be vertically homogeneous"
        );
        assert!(
            !stack.is_hhomogeneous(),
            "Stack must not be horizontally homogeneous"
        );
        assert!(
            stack.interpolates_size(),
            "Stack must interpolate size across pages"
        );

        // Verify stack contains all 3 required pages: "main", "language", "history"
        assert!(
            stack.child_by_name("main").is_some(),
            "Stack must have 'main' page"
        );
        assert!(
            stack.child_by_name("language").is_some(),
            "Stack must have 'language' page"
        );
        assert!(
            stack.child_by_name("history").is_some(),
            "Stack must have 'history' page"
        );

        // Subpage headings must use smaller "subpage-heading" class instead of "title-4"
        assert!(widgets.language_title.has_css_class("subpage-heading"));
        assert!(!widgets.language_title.has_css_class("title-4"));
        assert!(widgets.history_title.has_css_class("subpage-heading"));
        assert!(!widgets.history_title.has_css_class("title-4"));

        // Initial visible page must be "main"
        assert_eq!(stack.visible_child_name().as_deref(), Some("main"));
    }
}

#[test]
fn test_main_menu_minibar_zoom_controls() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, widgets) = build_header_bar(&config, &storage);

        assert_eq!(widgets.zoom_label.label().as_str(), "100%");
        assert_eq!(*widgets.minibar_scale.borrow(), 100);

        let tracked_scale = Rc::new(RefCell::new(100u32));
        let tracked_clone = tracked_scale.clone();
        widgets.set_on_scale_change(move |s| {
            *tracked_clone.borrow_mut() = s;
        });

        // Click Zoom In (+) twice -> 110%, 120%
        widgets.zoom_in_button.emit_clicked();
        assert_eq!(widgets.zoom_label.label().as_str(), "110%");
        assert_eq!(*tracked_scale.borrow(), 110);

        widgets.zoom_in_button.emit_clicked();
        assert_eq!(widgets.zoom_label.label().as_str(), "120%");
        assert_eq!(*tracked_scale.borrow(), 120);

        // Click Zoom Out (-) -> 110%
        widgets.zoom_out_button.emit_clicked();
        assert_eq!(widgets.zoom_label.label().as_str(), "110%");
        assert_eq!(*tracked_scale.borrow(), 110);

        // Click Reset (100%) -> 100%
        widgets.zoom_reset_button.emit_clicked();
        assert_eq!(widgets.zoom_label.label().as_str(), "100%");
        assert_eq!(*tracked_scale.borrow(), 100);
    }
}

#[test]
fn test_main_menu_navigation_to_subpages_without_popdown() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, widgets) = build_header_bar(&config, &storage);

        assert_eq!(widgets.stack.visible_child_name().as_deref(), Some("main"));

        // 1. Navigate to Language page
        widgets.language_button.emit_clicked();
        assert_eq!(
            widgets.stack.visible_child_name().as_deref(),
            Some("language"),
            "Clicking Language button must switch stack to 'language'"
        );

        // 2. Navigate back to Main page
        widgets.language_back_button.emit_clicked();
        assert_eq!(
            widgets.stack.visible_child_name().as_deref(),
            Some("main"),
            "Clicking Language back button must return stack to 'main'"
        );

        // 3. Navigate to History page
        widgets.history_button.emit_clicked();
        assert_eq!(
            widgets.stack.visible_child_name().as_deref(),
            Some("history"),
            "Clicking History button must switch stack to 'history'"
        );

        // 4. Navigate back to Main page
        widgets.history_back_button.emit_clicked();
        assert_eq!(
            widgets.stack.visible_child_name().as_deref(),
            Some("main"),
            "Clicking History back button must return stack to 'main'"
        );
    }
}

#[test]
fn test_language_page_catalog_and_selection_callback() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config {
            ui_language: "en".to_string(),
            ..Default::default()
        };
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, widgets) = build_header_bar(&config, &storage);

        let languages = get_languages();
        assert_eq!(languages.len(), 25, "Catalog must contain 25 languages");

        // Verify list box contains 25 rows
        let list_box = &widgets.language_list_box;
        let mut row_count = 0;
        let mut curr = list_box.first_child();
        while let Some(c) = curr {
            if c.is::<gtk4::ListBoxRow>() {
                row_count += 1;
            }
            curr = c.next_sibling();
        }
        assert_eq!(row_count, 25, "Language ListBox must have 25 rows");

        // Initial active language
        assert_eq!(*widgets.active_language.borrow(), "en");

        // Register on_language_change callback
        let selected_lang_tracker = Rc::new(RefCell::new(String::new()));
        let tracker_clone = selected_lang_tracker.clone();
        widgets.set_on_language_change(move |code| {
            *tracker_clone.borrow_mut() = code;
        });

        // Activate Spanish ("es" is index 1)
        if let Some(es_row) = list_box.row_at_index(1) {
            list_box.emit_by_name::<()>("row-activated", &[&es_row]);
        }

        assert_eq!(*selected_lang_tracker.borrow(), "es");
        assert_eq!(*widgets.active_language.borrow(), "es");

        // Activate German ("de" is index 2)
        if let Some(de_row) = list_box.row_at_index(2) {
            list_box.emit_by_name::<()>("row-activated", &[&de_row]);
        }

        assert_eq!(*selected_lang_tracker.borrow(), "de");
        assert_eq!(*widgets.active_language.borrow(), "de");
    }
}

#[test]
fn test_history_snippet_shows_at_least_five_words() {
    let sample1 = "The quick brown fox jumps over the lazy dog while testing OpenDictate's real-time voice transcription.";
    let snippet1 = format_history_snippet(sample1);
    let words1: Vec<&str> = snippet1.split_whitespace().collect();
    assert!(
        words1.len() >= 5,
        "Snippet must show at least 5 words, got: {}",
        snippet1
    );
    assert!(snippet1.starts_with("The quick brown fox jumps"));

    let sample2 =
        "Remember to review the pull request for the pure Rust GTK4 and Libadwaita interface.";
    let snippet2 = format_history_snippet(sample2);
    let words2: Vec<&str> = snippet2.split_whitespace().collect();
    assert!(
        words2.len() >= 5,
        "Snippet must show at least 5 words, got: {}",
        snippet2
    );
    assert!(snippet2.starts_with("Remember to review the pull"));
}

#[test]
fn test_history_page_empty_and_populated_and_delete() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let storage = StorageService::in_memory().unwrap();
        let (_header_bar, widgets) = build_header_bar(&config, &storage);

        // 1. Initial empty state
        assert!(
            widgets.history_empty_label.is_visible(),
            "Empty label must be visible when storage has no records"
        );
        assert!(
            !widgets.history_list_box.is_visible(),
            "History list box must be hidden when storage has no records"
        );

        // 2. Insert a record into storage
        let rec_id = storage
            .insert_dictation(
                "Raw hello world dictation snippet",
                "Processed hello world full text transcription for testing",
                "Clean",
                "cloud",
                3.5,
            )
            .expect("insert dictation");

        // Clicking history button or calling refresh_history populates the list
        widgets.history_button.emit_clicked();

        assert!(
            !widgets.history_empty_label.is_visible(),
            "Empty label must be hidden after inserting record"
        );
        assert!(
            widgets.history_list_box.is_visible(),
            "History list box must be visible after inserting record"
        );

        // 3. Inspect the history row
        let first_row = widgets
            .history_list_box
            .row_at_index(0)
            .expect("Must have at least 1 history row");

        // Tooltip must contain full text
        let tooltip = first_row.tooltip_text().unwrap_or_default();
        assert!(
            tooltip.contains("Processed hello world full text transcription for testing"),
            "Row tooltip should contain full text, got: {}",
            tooltip
        );

        // 4. Find delete button inside the row and click it
        let mut delete_btn_opt: Option<gtk4::Button> = None;
        if let Some(row_box) = first_row
            .child()
            .and_then(|c| c.downcast::<gtk4::Box>().ok())
        {
            let mut child = row_box.first_child();
            while let Some(c) = child {
                if let Ok(actions_box) = c.clone().downcast::<gtk4::Box>() {
                    let mut btn_child = actions_box.first_child();
                    while let Some(b) = btn_child {
                        if let Ok(btn) = b.clone().downcast::<gtk4::Button>()
                            && btn.icon_name().as_deref() == Some("user-trash-symbolic")
                        {
                            delete_btn_opt = Some(btn);
                            break;
                        }
                        btn_child = b.next_sibling();
                    }
                }
                child = c.next_sibling();
            }
        }

        let delete_btn = delete_btn_opt.expect("Row must have a delete button");
        delete_btn.emit_clicked();

        // 5. Verify record is deleted from SQLite
        let remaining = storage.list_dictations().expect("list dictations");
        assert!(
            remaining.is_empty(),
            "Record with ID {} should have been deleted from storage",
            rec_id
        );

        // List box should be empty and empty label visible again
        assert!(
            widgets.history_empty_label.is_visible(),
            "Empty label must be restored after deleting the last record"
        );
        assert!(
            !widgets.history_list_box.is_visible(),
            "History list box should be hidden when empty"
        );
    }
}

#[test]
fn test_about_dialog_details_credits_and_sponsor_on_github() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let config = Config::default();
        let about = build_about_dialog(&config);

        // Website and Support Questions buttons must be removed
        assert_eq!(about.website().as_str(), "");
        assert_eq!(about.support_url().as_str(), "");

        // Dual-theme icon verification
        assert_eq!(about.application_icon().as_str(), "opendictate-dark");
        let white_cfg = Config {
            theme: "white".to_string(),
            ..Default::default()
        };
        let about_white = build_about_dialog(&white_cfg);
        assert_eq!(about_white.application_icon().as_str(), "opendictate-light");

        // "Code by" (developers) must be deleted, and "Design by" (designers) must keep
        // only H. K. Saikia (with both OpenDictate Contributors and GNOME Design Team removed)
        let developers: Vec<String> = about
            .developers()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert!(developers.is_empty(), "'Code by' section must be removed");

        let designers: Vec<String> = about
            .designers()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(designers, vec!["H. K. Saikia".to_string()]);
        assert!(!designers.iter().any(|d| d == "OpenDictate Contributors"));
        assert!(!designers.iter().any(|d| d == "GNOME Design Team"));

        // Comments must match the exact 2-sentence description
        let comments = about.comments();
        assert_eq!(
            comments.as_str(),
            "OpenDictate is a native Linux voice dictation and AI speech-to-text application. It combines private offline transcription with multi-provider Cloud AI models to turn spoken voice into clean, polished text."
        );

        // Portable, privacy-safe runtime troubleshooting diagnostics
        let diag = about.debug_info();
        assert!(diag.contains("OpenDictate "));
        assert!(diag.contains("Hardware: "));
        assert!(diag.contains("Toolkit: GTK "));
        assert!(diag.contains("[AI Engine]"));
        assert!(diag.contains("[Audio & Input]"));
        assert!(diag.contains("Audio Server: "));
        assert!(diag.contains("[Paths]"));
        assert!(diag.contains("~/.config/opendictate/config.json"));
        assert!(diag.contains("~/.local/share/opendictate"));
        if let Ok(home) = std::env::var("HOME")
            && !home.is_empty()
        {
            assert!(
                !diag.contains(&home),
                "Diagnostics must anonymize $HOME ({}) to ~ so it is not tied to one user's machine",
                home
            );
        }

        // Helper to check ActionRow titles and subtitles in the AboutDialog widget tree
        fn has_action_row(w: &gtk4::Widget, title: &str) -> bool {
            if let Some(row) = w.downcast_ref::<libadwaita::ActionRow>()
                && row.title().as_str() == title
            {
                return true;
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if has_action_row(&c, title) {
                    return true;
                }
                child = c.next_sibling();
            }
            false
        }

        fn has_action_row_subtitle(w: &gtk4::Widget, sub_substr: &str) -> bool {
            if let Some(row) = w.downcast_ref::<libadwaita::ActionRow>()
                && let Some(sub) = row.subtitle()
                && sub.contains(sub_substr)
            {
                return true;
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if has_action_row_subtitle(&c, sub_substr) {
                    return true;
                }
                child = c.next_sibling();
            }
            false
        }

        let about_root = about
            .child()
            .unwrap_or_else(|| about.clone().upcast::<gtk4::Widget>());

        // Verify all 5 default downloadable Local AI model full URLs are in the Details subpage
        for expected_bin in [
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin",
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin",
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
        ] {
            assert!(
                has_action_row_subtitle(&about_root, expected_bin),
                "Details page must include full model link: {}",
                expected_bin
            );
        }

        // Verify "Sponsor on GitHub" ActionRow is present below Acknowledgements on the About main page
        assert!(
            has_action_row(&about_root, "Sponsor on GitHub"),
            "About dialog must contain 'Sponsor on GitHub' ActionRow below Acknowledgements"
        );
        assert!(
            !has_action_row(&about_root, "Buy me a coffee"),
            "About dialog must not contain stale 'Buy me a coffee' ActionRow"
        );

        // Verify Sponsor on GitHub dialog structure (subtitle removed, EntryRow titled "Custom amount")
        let sponsor_dlg = build_sponsor_dialog();
        assert_eq!(sponsor_dlg.title().as_str(), "Sponsor on GitHub");
        let sponsor_root = sponsor_dlg
            .child()
            .unwrap_or_else(|| sponsor_dlg.clone().upcast::<gtk4::Widget>());
        fn has_entry_row(w: &gtk4::Widget, title: &str) -> bool {
            if let Some(row) = w.downcast_ref::<libadwaita::EntryRow>()
                && row.title().as_str() == title
            {
                return true;
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if has_entry_row(&c, title) {
                    return true;
                }
                child = c.next_sibling();
            }
            false
        }
        fn has_label_containing(w: &gtk4::Widget, needle: &str) -> bool {
            if let Some(lbl) = w.downcast_ref::<gtk4::Label>()
                && lbl.text().contains(needle)
            {
                return true;
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if has_label_containing(&c, needle) {
                    return true;
                }
                child = c.next_sibling();
            }
            false
        }
        assert!(
            has_entry_row(&sponsor_root, "Custom amount"),
            "Sponsor dialog must have EntryRow titled 'Custom amount'"
        );
        assert!(
            !has_entry_row(&sponsor_root, "Custom amount in dollars ($)"),
            "Old title 'Custom amount in dollars ($)' must be removed"
        );
        assert!(
            !has_label_containing(&sponsor_root, "Fuel independent open-source"),
            "Subtitle sentence must be removed from Sponsor dialog"
        );
        assert!(
            !has_label_containing(&sponsor_root, "Buy Me a Coffee"),
            "Stale 'Buy Me a Coffee' label must not be present in Sponsor dialog"
        );
        assert!(
            !has_label_containing(&sponsor_root, "buymeacoffee"),
            "Stale 'buymeacoffee' text must not be present in Sponsor dialog"
        );
    }
}

#[test]
fn test_sponsor_amount_parsing_and_validation() {
    // 1. Valid preset values (consistent with whole-dollar tiers)
    assert_eq!(parse_custom_sponsor_amount("2"), Some(2));
    assert_eq!(parse_custom_sponsor_amount("5"), Some(5));
    assert_eq!(parse_custom_sponsor_amount("10"), Some(10));
    assert_eq!(parse_custom_sponsor_amount("20"), Some(20));

    // Smallest accepted whole-dollar amount ($1)
    assert_eq!(parse_custom_sponsor_amount("1"), Some(1));
    assert_eq!(
        parse_custom_sponsor_amount(&MIN_SPONSOR_AMOUNT.to_string()),
        Some(MIN_SPONSOR_AMOUNT)
    );

    // Mid-range valid custom positive whole-dollar integer input
    assert_eq!(parse_custom_sponsor_amount("42"), Some(42));
    assert_eq!(parse_custom_sponsor_amount("100"), Some(100));
    assert_eq!(parse_custom_sponsor_amount("  250  "), Some(250)); // handles leading/trailing whitespace
    assert_eq!(parse_custom_sponsor_amount("1000"), Some(1000));
    assert_eq!(parse_custom_sponsor_amount("11999"), Some(11999));

    // Largest accepted whole-dollar amount ($12,000)
    assert_eq!(parse_custom_sponsor_amount("12000"), Some(12000));
    assert_eq!(
        parse_custom_sponsor_amount(&MAX_SPONSOR_AMOUNT.to_string()),
        Some(MAX_SPONSOR_AMOUNT)
    );

    // Immediately above supported maximum ($12,001)
    assert_eq!(parse_custom_sponsor_amount("12001"), None);
    assert_eq!(
        parse_custom_sponsor_amount(&(MAX_SPONSOR_AMOUNT + 1).to_string()),
        None
    );

    // Amounts significantly exceeding maximum
    assert_eq!(parse_custom_sponsor_amount("15000"), None);
    assert_eq!(parse_custom_sponsor_amount("50000"), None);
    assert_eq!(parse_custom_sponsor_amount("1000000"), None);

    // Clearing a previously valid custom amount
    assert_eq!(parse_custom_sponsor_amount(""), None);

    // Empty custom input
    assert_eq!(parse_custom_sponsor_amount(""), None);
    assert_eq!(parse_custom_sponsor_amount("   "), None);
    assert_eq!(parse_custom_sponsor_amount("\t\n"), None);

    // Zero and negative values
    assert_eq!(parse_custom_sponsor_amount("0"), None);
    assert_eq!(parse_custom_sponsor_amount("00"), None);
    assert_eq!(parse_custom_sponsor_amount("-1"), None);
    assert_eq!(parse_custom_sponsor_amount("-5"), None);
    assert_eq!(parse_custom_sponsor_amount("-0"), None);
    assert_eq!(parse_custom_sponsor_amount("-12000"), None);

    // Decimal and malformed input (MUST NOT silently strip '.' or convert to different amount)
    assert_eq!(parse_custom_sponsor_amount("1.0"), None);
    assert_eq!(parse_custom_sponsor_amount("5.0"), None);
    assert_eq!(parse_custom_sponsor_amount("5.50"), None);
    assert_eq!(parse_custom_sponsor_amount("12000.0"), None);
    assert_eq!(parse_custom_sponsor_amount("12000.50"), None);
    assert_eq!(parse_custom_sponsor_amount("5."), None);
    assert_eq!(parse_custom_sponsor_amount(".5"), None);
    assert_eq!(parse_custom_sponsor_amount("5.0.0"), None);
    assert_eq!(parse_custom_sponsor_amount("$5"), None);
    assert_eq!(parse_custom_sponsor_amount("$12000"), None);
    assert_eq!(parse_custom_sponsor_amount("5$"), None);
    assert_eq!(parse_custom_sponsor_amount("5 0"), None);
    assert_eq!(parse_custom_sponsor_amount("1,000"), None);
    assert_eq!(parse_custom_sponsor_amount("12,000"), None);

    // Alphabetic input
    assert_eq!(parse_custom_sponsor_amount("abc"), None);
    assert_eq!(parse_custom_sponsor_amount("five"), None);
    assert_eq!(parse_custom_sponsor_amount("5usd"), None);
    assert_eq!(parse_custom_sponsor_amount("usd5"), None);
    assert_eq!(parse_custom_sponsor_amount("12k"), None);
    assert_eq!(parse_custom_sponsor_amount("NaN"), None);

    // Numeric overflow (larger than u32::MAX)
    assert_eq!(parse_custom_sponsor_amount("4294967296"), None);
    assert_eq!(
        parse_custom_sponsor_amount("999999999999999999999999999999"),
        None
    );
    assert_eq!(
        parse_custom_sponsor_amount(&format!("{}", u64::from(u32::MAX) + 1)),
        None
    );
}

#[test]
fn test_sponsor_url_construction_and_stale_prevention() {
    // Ensuring invalid, absent, or out-of-range amounts cannot produce a sponsorship URL
    assert_eq!(build_github_sponsors_url(None), None);
    assert_eq!(
        build_github_sponsors_url(parse_custom_sponsor_amount("")),
        None
    );
    assert_eq!(
        build_github_sponsors_url(parse_custom_sponsor_amount("0")),
        None
    );
    assert_eq!(
        build_github_sponsors_url(parse_custom_sponsor_amount("12001")),
        None
    );
    assert_eq!(
        build_github_sponsors_url(parse_custom_sponsor_amount("invalid")),
        None
    );
    assert_eq!(
        build_github_sponsors_url(parse_custom_sponsor_amount("5.50")),
        None
    );

    // Ensuring direct out-of-range Option<u32> values cannot construct a URL
    assert_eq!(build_github_sponsors_url(Some(0)), None);
    assert_eq!(build_github_sponsors_url(Some(12_001)), None);
    assert_eq!(build_github_sponsors_url(Some(50_000)), None);
    assert_eq!(build_github_sponsors_url(Some(u32::MAX)), None);

    // Ensuring the displayed amount matches the amount that would be sent to GitHub
    assert_eq!(
        build_github_sponsors_url(Some(1)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=1".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(MIN_SPONSOR_AMOUNT)),
        Some(format!(
            "https://github.com/sponsors/hkumarsaikia?amount={}",
            MIN_SPONSOR_AMOUNT
        ))
    );
    assert_eq!(
        build_github_sponsors_url(Some(2)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=2".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(5)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=5".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(10)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=10".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(20)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=20".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(50)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=50".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(12_000)),
        Some("https://github.com/sponsors/hkumarsaikia?amount=12000".to_string())
    );
    assert_eq!(
        build_github_sponsors_url(Some(MAX_SPONSOR_AMOUNT)),
        Some(format!(
            "https://github.com/sponsors/hkumarsaikia?amount={}",
            MAX_SPONSOR_AMOUNT
        ))
    );

    // Verify canonical URL preserves existing parameter schema (no frequency parameter added)
    let url = build_github_sponsors_url(Some(15)).unwrap();
    assert!(url.starts_with("https://github.com/sponsors/hkumarsaikia?amount="));
    assert!(!url.contains("frequency="));
}

#[test]
fn test_sponsor_dialog_amount_state_and_ui_transitions() {
    if gtk4::is_initialized_main_thread() || (!gtk4::is_initialized() && gtk4::init().is_ok()) {
        let _ = libadwaita::init();

        let sponsor_dlg = build_sponsor_dialog();
        let root = sponsor_dlg
            .child()
            .unwrap_or_else(|| sponsor_dlg.clone().upcast::<gtk4::Widget>());

        fn find_toggle_buttons(w: &gtk4::Widget, list: &mut Vec<gtk4::ToggleButton>) {
            if let Some(tb) = w.downcast_ref::<gtk4::ToggleButton>() {
                list.push(tb.clone());
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                find_toggle_buttons(&c, list);
                child = c.next_sibling();
            }
        }

        fn find_entry_row(w: &gtk4::Widget) -> Option<libadwaita::EntryRow> {
            if let Some(entry) = w.downcast_ref::<libadwaita::EntryRow>() {
                return Some(entry.clone());
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if let Some(found) = find_entry_row(&c) {
                    return Some(found);
                }
                child = c.next_sibling();
            }
            None
        }

        fn find_cta_button(w: &gtk4::Widget) -> Option<gtk4::Button> {
            if let Some(btn) = w.downcast_ref::<gtk4::Button>()
                && btn.has_css_class("suggested-action")
            {
                return Some(btn.clone());
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if let Some(found) = find_cta_button(&c) {
                    return Some(found);
                }
                child = c.next_sibling();
            }
            None
        }

        fn has_label_containing(w: &gtk4::Widget, needle: &str) -> bool {
            if let Some(lbl) = w.downcast_ref::<gtk4::Label>()
                && lbl.text().contains(needle)
            {
                return true;
            }
            let mut child = w.first_child();
            while let Some(c) = child {
                if has_label_containing(&c, needle) {
                    return true;
                }
                child = c.next_sibling();
            }
            false
        }

        // Verify preset buttons do NOT contain unverified tier titles (Supporter, Backer, etc.)
        assert!(
            !has_label_containing(&root, "Supporter"),
            "Preset buttons must not contain unverified tier label 'Supporter'"
        );
        assert!(
            !has_label_containing(&root, "Backer"),
            "Preset buttons must not contain unverified tier label 'Backer'"
        );
        assert!(
            !has_label_containing(&root, "Bronze Sponsor"),
            "Preset buttons must not contain unverified tier label 'Bronze Sponsor'"
        );
        assert!(
            !has_label_containing(&root, "Silver Sponsor"),
            "Preset buttons must not contain unverified tier label 'Silver Sponsor'"
        );

        let mut toggles = Vec::new();
        find_toggle_buttons(&root, &mut toggles);
        assert_eq!(toggles.len(), 4, "Must have 4 preset toggle buttons");

        let entry = find_entry_row(&root).expect("Must have custom amount EntryRow");
        let cta_btn = find_cta_button(&root).expect("Must have suggested-action CTA button");

        // 1. Initial State: $5 preset selected, CTA enabled, label "Sponsor $5 on GitHub 💖"
        assert!(toggles[1].is_active(), "Default preset $5 must be active");
        assert!(
            cta_btn.is_sensitive(),
            "CTA button must be sensitive initially"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Sponsor $5 on GitHub 💖"));
        assert_eq!(entry.text().as_str(), "");

        // 2. Switching between preset amounts: click $10 (index 2)
        toggles[2].set_active(true);
        assert!(toggles[2].is_active());
        assert!(!toggles[1].is_active());
        assert!(cta_btn.is_sensitive());
        assert_eq!(cta_btn.label().as_deref(), Some("Sponsor $10 on GitHub 💖"));
        assert_eq!(entry.text().as_str(), "");

        // 3. Valid custom amount: enter "25"
        entry.set_text("25");
        assert!(
            !toggles[2].is_active(),
            "Preset toggles must be inactive when custom amount is entered"
        );
        assert!(cta_btn.is_sensitive());
        assert_eq!(cta_btn.label().as_deref(), Some("Sponsor $25 on GitHub 💖"));

        // 4. Minimum accepted whole-dollar amount: enter "1"
        entry.set_text("1");
        assert!(cta_btn.is_sensitive());
        assert_eq!(cta_btn.label().as_deref(), Some("Sponsor $1 on GitHub 💖"));

        // 5. Maximum supported amount: enter "12000"
        entry.set_text("12000");
        assert!(cta_btn.is_sensitive());
        assert_eq!(
            cta_btn.label().as_deref(),
            Some("Sponsor $12000 on GitHub 💖")
        );

        // 6. Immediately above maximum supported amount: enter "12001"
        entry.set_text("12001");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled for amount exceeding $12,000"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Invalid amount"));

        // 7. Clearing a previously valid custom amount
        entry.set_text("");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled when custom amount is cleared"
        );
        assert_eq!(
            cta_btn.label().as_deref(),
            Some("Select an amount to sponsor")
        );

        // 8. Zero value
        entry.set_text("0");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled for zero amount"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Invalid amount"));

        // 9. Decimal input
        entry.set_text("12.50");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled for decimal amount"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Invalid amount"));

        // 10. Alphabetic input
        entry.set_text("fifty");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled for alphabetic amount"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Invalid amount"));

        // 11. Overflow input
        entry.set_text("99999999999999999999999");
        assert!(
            !cta_btn.is_sensitive(),
            "CTA button must be disabled for overflow amount"
        );
        assert_eq!(cta_btn.label().as_deref(), Some("Invalid amount"));

        // Re-selecting a preset: click $20 (index 3)
        toggles[3].set_active(true);
        assert!(toggles[3].is_active());
        assert!(cta_btn.is_sensitive());
        assert_eq!(cta_btn.label().as_deref(), Some("Sponsor $20 on GitHub 💖"));
        assert_eq!(
            entry.text().as_str(),
            "",
            "Selecting a preset must clear the custom entry field"
        );
    }
}
