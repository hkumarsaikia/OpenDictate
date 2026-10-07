//! OpenDictate: Native Linux voice dictation and AI speech-to-text application.

pub mod audio;
pub mod cli;
pub mod config;
pub mod services;
pub mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
