//! Soundwave visualizer engine for OpenDictate MiniBar.
//!
//! Provides a 60 FPS animated 5-bar equalizer rendered with Cairo.

use gtk4::cairo;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// State for the 5-bar soundwave visualizer.
#[derive(Debug, Clone, PartialEq)]
pub struct VisualizerState {
    current_levels: [f32; 5],
    target_levels: [f32; 5],
    is_recording: bool,
    phase: f32,
}

impl Default for VisualizerState {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualizerState {
    /// Creates a new visualizer state with idle round dot levels (0.0).
    pub fn new() -> Self {
        Self {
            current_levels: [0.0; 5],
            target_levels: [0.0; 5],
            is_recording: false,
            phase: 0.0,
        }
    }

    /// Sets whether active recording is taking place.
    pub fn set_recording(&mut self, recording: bool) {
        self.is_recording = recording;
        self.phase = 0.0;
        if !recording {
            self.target_levels = [0.0; 5];
        }
    }

    /// Returns true if recording animation is active.
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// Updates target levels for the 5 equalizer bars, clamped to [0.0, 1.0].
    /// Guarded against NaN or non-finite values.
    pub fn update_levels(&mut self, levels: [f32; 5]) {
        for (i, &lvl) in levels.iter().enumerate() {
            self.target_levels[i] = if lvl.is_finite() {
                lvl.clamp(0.0, 1.0)
            } else {
                0.0
            };
        }
    }

    /// Smoothly interpolates current levels toward targets.
    ///
    /// `factor` is clamped between `0.0` and `1.0`. Guarded against NaN or non-finite values.
    /// Returns `true` if any bar's current level changed by more than `1e-4`, else `false`.
    pub fn interpolate(&mut self, factor: f32) -> bool {
        let f = if factor.is_finite() {
            factor.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let max_target = self.target_levels.iter().cloned().fold(0.0f32, f32::max);
        let max_current = self.current_levels.iter().cloned().fold(0.0f32, f32::max);

        if !self.is_recording && max_target <= 1e-4 && max_current <= 1e-4 {
            self.current_levels = [0.0; 5];
            self.phase = 0.0;
            return false;
        }

        let mut changed = false;
        const WEIGHTS: [f32; 5] = [0.55, 0.85, 1.0, 0.85, 0.55];
        let voice_active = max_target > 0.04;

        if self.is_recording && voice_active {
            self.phase = (self.phase + 0.18) % (2.0 * std::f32::consts::PI);
            for (i, &weight) in WEIGHTS.iter().enumerate() {
                let wave = 0.5 * (1.0 + (self.phase + i as f32 * 1.05).sin());
                let lvl = self.target_levels[i];
                // Modulation is strictly multiplied by voice level so zero voice level yields zero motion
                let modulation = 0.80 + 0.35 * wave;
                let amp = (weight * lvl * modulation).clamp(0.0, 1.0);
                let prev = self.current_levels[i];
                self.current_levels[i] += (amp - self.current_levels[i]) * 0.40;
                if (self.current_levels[i] - prev).abs() > 1e-4 {
                    changed = true;
                }
            }
        } else {
            self.phase = 0.0;
            for i in 0..5 {
                let prev = self.current_levels[i];
                self.current_levels[i] += (self.target_levels[i] - self.current_levels[i]) * f;
                self.current_levels[i] = if self.current_levels[i].is_finite() {
                    let clamped = self.current_levels[i].clamp(0.0, 1.0);
                    if self.target_levels[i] <= 1e-4 && clamped <= 1e-4 {
                        0.0
                    } else {
                        clamped
                    }
                } else {
                    0.0
                };
                if (self.current_levels[i] - prev).abs() > 1e-4 {
                    changed = true;
                }
            }
        }
        changed
    }

    /// Returns the current smoothed bar levels in range [0.0, 1.0].
    pub fn current_levels(&self) -> [f32; 5] {
        self.current_levels
    }

    /// Returns the target bar levels in range [0.0, 1.0].
    pub fn target_levels(&self) -> [f32; 5] {
        self.target_levels
    }

    /// Resets both current and target levels to idle baseline.
    pub fn reset_idle(&mut self) {
        self.current_levels = [0.0; 5];
        self.target_levels = [0.0; 5];
        self.is_recording = false;
        self.phase = 0.0;
    }
}

/// Helper function to construct a rounded rectangle (capsule) in Cairo.
fn draw_capsule(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let r = r.min(w / 2.0).min(h / 2.0);
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(
        x + r,
        y + h - r,
        r,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
    );
    cr.arc(
        x + r,
        y + r,
        r,
        std::f64::consts::PI,
        3.0 * std::f64::consts::FRAC_PI_2,
    );
    cr.close_path();
}

/// Constructs a 60 FPS Cairo DrawingArea for the 5-bar soundwave visualizer.
pub fn build_visualizer_drawing_area(state: Rc<RefCell<VisualizerState>>) -> gtk4::DrawingArea {
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_content_width(46);
    drawing_area.set_content_height(24);
    drawing_area.set_valign(gtk4::Align::Center);
    drawing_area.set_halign(gtk4::Align::Center);

    let draw_state = state.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let w = width as f64;
        let h = height as f64;
        let scale = (h / 24.0).max(0.5);

        let num_bars = 5;
        let bar_w = 3.5 * scale;
        let radius = 1.75 * scale;
        let spacing = 3.0 * scale;
        let min_h = 3.5 * scale;
        let max_h = (18.0 * scale).min(h - 2.0 * scale);

        let total_w = num_bars as f64 * bar_w + (num_bars - 1) as f64 * spacing;
        let start_x = ((w - total_w) / 2.0).max(0.0);

        let state_ref = draw_state.borrow();
        let levels = state_ref.current_levels();
        let is_recording = state_ref.is_recording();
        let is_dark = if gtk4::is_initialized() {
            libadwaita::StyleManager::default().is_dark()
        } else {
            true
        };

        for (i, &lvl) in levels.iter().enumerate().take(num_bars) {
            let x = start_x + i as f64 * (bar_w + spacing);
            let lvl = lvl.clamp(0.0, 1.0) as f64;

            if lvl > 0.015 {
                let bar_h = min_h + lvl * (max_h - min_h);
                let y = (h - bar_h) / 2.0;

                draw_capsule(cr, x, y, bar_w, bar_h, radius);

                // Vibrant Adwaita blue gradient (#62a0ea -> #1c71d8)
                let grad = cairo::LinearGradient::new(x, y, x, y + bar_h);
                grad.add_color_stop_rgb(0.0, 98.0 / 255.0, 160.0 / 255.0, 234.0 / 255.0);
                grad.add_color_stop_rgb(1.0, 28.0 / 255.0, 113.0 / 255.0, 216.0 / 255.0);
                let _ = cr.set_source(&grad);
                let _ = cr.fill();
            } else {
                // Sleek idle round dot with high-contrast theme-aware color
                let bar_h = min_h;
                let y = (h - bar_h) / 2.0;

                draw_capsule(cr, x, y, bar_w, bar_h, radius);
                if is_recording {
                    // Subtle blue resting dot during recording silence
                    cr.set_source_rgba(0.38, 0.63, 0.92, 0.55);
                } else if is_dark {
                    cr.set_source_rgba(0.70, 0.70, 0.75, 0.45);
                } else {
                    cr.set_source_rgba(0.20, 0.20, 0.25, 0.45);
                }
                let _ = cr.fill();
            }
        }
    });

    // 60 FPS tick animation callback
    let tick_state = state;
    drawing_area.add_tick_callback(move |da, _frame_clock| {
        let changed = tick_state.borrow_mut().interpolate(0.25);
        if changed {
            da.queue_draw();
        }
        gtk4::glib::ControlFlow::Continue
    });

    drawing_area
}
