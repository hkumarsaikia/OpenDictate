//! Automated high-quality screenshot generator for OpenDictate UI.
//! Renders native GTK4 / Libadwaita widgets directly to PNG matching the user's reference designs.

use gtk4::prelude::*;
use opendictate::config::Config;
use opendictate::services::storage::StorageService;
use opendictate::ui::main_window::build_main_window;
use opendictate::ui::mini_bar::build_minibar_window;
use opendictate::ui::theme::{ThemeMode, sync_theme_with_adwaita};

fn pump(ms: u64) {
    let ctx = gtk4::glib::MainContext::default();
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_millis(ms) {
        while ctx.iteration(false) {}
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn capture_window<W: IsA<gtk4::Window>>(
    win: &W,
    min_w: i32,
    min_h: i32,
    filename: &str,
    out_dir: &std::path::Path,
) {
    let win_ref = win.as_ref();
    win_ref.present();
    pump(400);

    let paintable = gtk4::WidgetPaintable::new(Some(win_ref));
    let width = paintable.intrinsic_width().max(min_w);
    let height = paintable.intrinsic_height().max(min_h);
    let snapshot = gtk4::Snapshot::new();
    paintable.snapshot(&snapshot, width as f64, height as f64);
    if let Some(node) = snapshot.to_node() {
        let renderer = gtk4::gsk::CairoRenderer::new();
        if let Some(surface) = win_ref.surface()
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
            println!("RENDERED: {}", path.display());
        }
    }
    win_ref.close();
    pump(100);
}

#[test]
#[ignore]
fn generate_all_screenshots() {
    if !gtk4::is_initialized_main_thread() && !gtk4::is_initialized() {
        let _ = gtk4::init();
    }
    let _ = libadwaita::init();

    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/screenshots");
    std::fs::create_dir_all(&out_dir).expect("Failed to create docs/screenshots");

    let storage = StorageService::in_memory().unwrap();

    // 1. main-window-settings.png (Dark Mode Settings matching Reference 4)
    sync_theme_with_adwaita(ThemeMode::Dark);
    let mut dark_config = Config::default();
    dark_config.theme = "dark".to_string();
    dark_config.ai_mode = "local".to_string();
    dark_config.ai_provider = "gemini".to_string();
    dark_config.local_model_id = "tiny.en".to_string();
    dark_config.audio_device = Some("Handsfree".to_string());
    dark_config.hotkey = "Ctrl+Alt+D".to_string();

    let dark_window = build_main_window(&dark_config, &storage);
    dark_window.set_default_size(780, 960);
    capture_window(&dark_window, 780, 960, "main-window-settings.png", &out_dir);

    // 2. main-window-light.png (Light Mode Settings matching Reference 5)
    sync_theme_with_adwaita(ThemeMode::White);
    let mut light_config = Config::default();
    light_config.theme = "white".to_string();
    light_config.ai_mode = "local".to_string();
    light_config.ai_provider = "gemini".to_string();
    light_config.local_model_id = "tiny.en".to_string();
    light_config.audio_device = Some("Handsfree".to_string());
    light_config.hotkey = "Ctrl+Alt+D".to_string();

    let light_window = build_main_window(&light_config, &storage);
    light_window.set_default_size(780, 960);
    capture_window(&light_window, 780, 960, "main-window-light.png", &out_dir);

    // 3. floating-minibar.png (MiniBar with expanded transcript drawer matching Reference 2)
    sync_theme_with_adwaita(ThemeMode::Dark);
    let (minibar_win, minibar_widgets) = build_minibar_window();
    minibar_widgets.drawer_revealer.set_reveal_child(true);
    minibar_widgets.drawer_widgets.status_badge.set_text("Recording cancelled");
    minibar_widgets.drawer_widgets.word_count_badge.set_text("0 words");
    minibar_widgets.drawer_widgets.set_text("Dictated text will appear here...");
    capture_window(&minibar_win, 447, 221, "floating-minibar.png", &out_dir);

    // 4. header-menu-popover.png (Main window with hamburger popover opened)
    sync_theme_with_adwaita(ThemeMode::Dark);
    let menu_window = build_main_window(&dark_config, &storage);
    menu_window.set_default_size(780, 960);
    menu_window.present();
    pump(300);

    // Find and activate the menu button to open popover
    fn find_menu_button(widget: &gtk4::Widget) -> Option<gtk4::MenuButton> {
        if let Ok(btn) = widget.clone().downcast::<gtk4::MenuButton>() {
            return Some(btn);
        }
        let mut curr = widget.first_child();
        while let Some(child) = curr {
            if let Some(found) = find_menu_button(&child) {
                return Some(found);
            }
            curr = child.next_sibling();
        }
        None
    }

    if let Some(btn) = find_menu_button(menu_window.upcast_ref()) {
        btn.popup();
        pump(400);

        if let Some(popover) = btn.popover() {
            let win_paintable = gtk4::WidgetPaintable::new(Some(&menu_window));
            let pop_paintable = gtk4::WidgetPaintable::new(Some(&popover));

            let width = win_paintable.intrinsic_width().max(780);
            let height = win_paintable.intrinsic_height().max(960);

            let snapshot = gtk4::Snapshot::new();
            win_paintable.snapshot(&snapshot, width as f64, height as f64);

            // 1. Draw card body: card is at x in [20, 302], y in [34, 248] in popover
            // Placed at card_x = 484, card_y = 38
            snapshot.save();
            snapshot.push_clip(&gtk4::graphene::Rect::new(484.0, 38.0, 282.0, 214.0));
            snapshot.translate(&gtk4::graphene::Point::new(484.0 - 20.0, 38.0 - 34.0));
            pop_paintable.snapshot(&snapshot, 322.0, 248.0);
            snapshot.pop();
            snapshot.restore();

            // 2. Draw arrow: arrow is at x in [145, 175], y in [21, 35] in popover
            // Placed so arrow tip aligns with menu button at x = 722
            snapshot.save();
            snapshot.push_clip(&gtk4::graphene::Rect::new(707.0, 25.0, 30.0, 15.0));
            snapshot.translate(&gtk4::graphene::Point::new(707.0 - 145.0, 25.0 - 21.0));
            pop_paintable.snapshot(&snapshot, 322.0, 248.0);
            snapshot.pop();
            snapshot.restore();

            if let Some(node) = snapshot.to_node() {
                let renderer = gtk4::gsk::CairoRenderer::new();
                if let Some(surface) = menu_window.surface()
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
                    let path = out_dir.join("header-menu-popover.png");
                    texture
                        .save_to_png(&path)
                        .expect("Failed to save header-menu-popover PNG");
                    renderer.unrealize();
                    println!("RENDERED COMPOSITED POPOVER TO: {}", path.display());
                }
            }
            menu_window.close();
            pump(100);
            return;
        }
    }
}
