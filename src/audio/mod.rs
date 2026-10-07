//! Audio capture, real-time RMS metering, and in-memory WAV encoding.

pub mod level;
pub mod recorder;

pub use level::{calculate_5_bar_levels, calculate_rms, LevelSmoother};
pub use recorder::{encode_pcm_wav, resample_to_16k, AudioError, AudioRecorder};
