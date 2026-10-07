//! Real-time audio RMS level calculation, logarithmic decibel mapping,
//! and 5-bar visualizer smoothing.

/// Compute Root Mean Square (RMS) energy with DC offset removal.
///
/// Acoustic audio is an alternating (AC) signal. Microphone hardware and virtual
/// audio streams frequently carry a static DC bias (mean offset), which is electric
/// offset rather than acoustic sound. Removing the DC offset ensures that silent
/// microphones with DC bias correctly yield 0.0 RMS energy.
pub fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
    let var: f32 = samples
        .iter()
        .map(|&s| {
            let diff = s - mean;
            diff * diff
        })
        .sum::<f32>()
        / samples.len() as f32;
    var.sqrt()
}

/// Minimum RMS threshold for real human voice (~ -30 dB). Ambient room silence and mic hiss below this are gated to 0.0.
pub const NOISE_GATE_THRESHOLD: f32 = 0.030;

/// Calculate 5 visualizer bar levels in range `[0.0, 1.0]`.
///
/// Uses logarithmic decibel mapping (-30 dB to 0 dB) and derives 5 normalized
/// bar levels with organic harmonic variation and frequency-slice distribution
/// so the visualizer bars animate organically when voice is present.
pub fn calculate_5_bar_levels(samples: &[f32]) -> [f32; 5] {
    let rms = calculate_rms(samples);
    if rms < NOISE_GATE_THRESHOLD {
        return [0.0; 5];
    }

    // Logarithmic decibel mapping: -30 dB to 0 dB mapped to [0.0, 1.0]
    let db = 20.0 * rms.max(NOISE_GATE_THRESHOLD).log10();
    let min_db = -30.0f32;
    let max_db = 0.0f32;
    let norm = ((db - min_db) / (max_db - min_db)).clamp(0.0, 1.0);

    // Harmonic visualizer weights (center-heavy wave distribution)
    const WEIGHTS: [f32; 5] = [0.60, 0.88, 1.00, 0.82, 0.55];

    let mut levels = [0.0f32; 5];
    if samples.len() >= 5 {
        let chunk_size = samples.len() / 5;
        for i in 0..5 {
            let chunk = &samples[i * chunk_size..(i + 1) * chunk_size];
            let chunk_rms = calculate_rms(chunk);
            let local_factor = if chunk_rms > 1e-5 {
                (chunk_rms / rms).clamp(0.5, 1.5)
            } else {
                1.0
            };
            levels[i] = (norm * WEIGHTS[i] * local_factor).clamp(0.0, 1.0);
        }
    } else {
        for i in 0..5 {
            levels[i] = (norm * WEIGHTS[i]).clamp(0.0, 1.0);
        }
    }

    levels
}

/// Exponential moving average smoother for 60 FPS transitions without jarring flickers.
#[derive(Debug, Clone)]
pub struct LevelSmoother {
    decay: f32,
    current: [f32; 5],
}

impl LevelSmoother {
    /// Creates a new smoother with specified decay factor in range `[0.0, 1.0]`.
    /// E.g. decay 0.7 retains 70% of previous frame and mixes 30% of new target.
    pub fn new(decay: f32) -> Self {
        Self {
            decay: decay.clamp(0.0, 1.0),
            current: [0.0; 5],
        }
    }

    /// Update smoothed levels towards target and return current values.
    pub fn update(&mut self, target: [f32; 5]) -> [f32; 5] {
        let alpha = 1.0 - self.decay;
        for (i, &tgt) in target.iter().enumerate() {
            self.current[i] = (alpha * tgt + self.decay * self.current[i]).clamp(0.0, 1.0);
        }
        self.current
    }

    /// Return current 5-bar levels.
    pub fn current(&self) -> [f32; 5] {
        self.current
    }

    /// Reset smoother state to all zeros.
    pub fn reset(&mut self) {
        self.current = [0.0; 5];
    }
}

impl Default for LevelSmoother {
    fn default() -> Self {
        Self::new(0.7)
    }
}
