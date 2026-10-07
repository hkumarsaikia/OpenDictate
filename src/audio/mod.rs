//! Audio capture, real-time RMS metering, and in-memory WAV encoding.

pub mod level;
pub mod recorder;

pub use level::{LevelSmoother, calculate_5_bar_levels, calculate_rms};
pub use recorder::{AudioError, AudioRecorder, encode_pcm_wav, resample_to_16k};
