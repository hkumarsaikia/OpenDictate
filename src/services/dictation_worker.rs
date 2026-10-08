//! Background Relm4 Worker managing microphone capture, AI transcription, SQLite persistence, and text injection.

use crate::audio::recorder::AudioRecorder;
use crate::config::Config;
use crate::services::ai::AiManager;
use crate::services::storage::StorageService;
use relm4::Worker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Instant;
use tokio::runtime::{Handle, Runtime};

static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Returns a reference to the global background Tokio runtime handle.
pub fn tokio_handle() -> &'static Handle {
    TOKIO_RUNTIME
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name("opendictate-tokio")
                .build()
                .expect("Failed to initialize background Tokio runtime")
        })
        .handle()
}

/// Messages sent to the DictationWorker to control recording and pipeline processing.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum DictationWorkerInput {
    StartRecording,
    PauseRecording,
    ResumeRecording,
    StopAndProcess { tone: String },
    CancelRecording,
    ToggleRecording { tone: String },
    UpdateConfig(Config),
}

/// Messages emitted by the DictationWorker back to parent components (e.g. MiniBar).
#[derive(Debug, Clone, PartialEq)]
pub enum DictationWorkerOutput {
    RecordingStarted,
    RecordingPaused,
    RecordingResumed,
    RecordingCancelled,
    AudioLevel(f32),
    PartialTranscript {
        raw_text: String,
        enhanced_text: String,
    },
    ProcessingStarted,
    Success {
        raw_text: String,
        enhanced_text: String,
        duration_seconds: f64,
    },
    Error(String),
    StatusMessage(String),
    NoSpeechDetected,
}

/// Relm4 Worker managing dictation lifecycle and asynchronous background jobs.
pub struct DictationWorker {
    recorder: AudioRecorder,
    ai_manager: AiManager,
    storage: StorageService,
    config: Config,
    is_recording: bool,
    is_paused: bool,
    start_time: Option<Instant>,
    active_tone: String,
    level_task: Option<tokio::task::AbortHandle>,
    stream_task: Option<tokio::task::AbortHandle>,
    level_active: Option<Arc<AtomicBool>>,
    last_partial: Arc<std::sync::Mutex<Option<(String, String)>>>,
}

impl std::fmt::Debug for DictationWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DictationWorker")
            .field("is_recording", &self.is_recording)
            .field("is_paused", &self.is_paused)
            .field("active_tone", &self.active_tone)
            .field("config", &self.config)
            .finish()
    }
}

impl DictationWorker {
    /// Constructs a new DictationWorker instance with the given configuration and storage.
    pub fn new(config: Config, storage: StorageService) -> Self {
        let recorder = AudioRecorder::new(config.audio_device.clone())
            .unwrap_or_else(|_| AudioRecorder::default());
        let ai_manager = AiManager::from_config(&config);
        Self {
            recorder,
            ai_manager,
            storage,
            config,
            is_recording: false,
            is_paused: false,
            start_time: None,
            active_tone: "Clean".to_string(),
            level_task: None,
            stream_task: None,
            level_active: None,
            last_partial: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    /// Returns true if currently recording.
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// Returns true if recording is paused.
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns the currently active enhancement tone.
    pub fn active_tone(&self) -> &str {
        &self.active_tone
    }

    /// Returns the current configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Worker for DictationWorker {
    type Init = (Config, StorageService);
    type Input = DictationWorkerInput;
    type Output = DictationWorkerOutput;

    fn init((config, storage): Self::Init, _sender: relm4::ComponentSender<Self>) -> Self {
        Self::new(config, storage)
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::ComponentSender<Self>) {
        match msg {
            DictationWorkerInput::UpdateConfig(cfg) => {
                if self.is_recording {
                    return;
                }
                self.ai_manager = AiManager::from_config(&cfg);
                if self.config.audio_device != cfg.audio_device
                    && let Ok(rec) = AudioRecorder::new(cfg.audio_device.clone())
                {
                    self.recorder = rec;
                }
                self.config = cfg;
            }
            DictationWorkerInput::StartRecording => {
                if self.is_recording {
                    return;
                }
                if let Ok(mut guard) = self.last_partial.lock() {
                    *guard = None;
                }
                if let Some(handle) = self.stream_task.take() {
                    handle.abort();
                }

                let (level_tx, mut level_rx) = tokio::sync::mpsc::channel::<[f32; 5]>(100);
                if let Err(e) = self.recorder.start_recording(level_tx) {
                    crate::services::crash_reporter::CrashReporter::record_event(
                        "AudioCapture",
                        &format!("Failed to start microphone stream: {}", e),
                    );
                    let _ = sender.output(DictationWorkerOutput::Error(format!(
                        "Failed to start recording: {}",
                        e
                    )));
                    return;
                }
                self.is_recording = true;
                self.is_paused = false;
                self.start_time = Some(Instant::now());
                let _ = sender.output(DictationWorkerOutput::RecordingStarted);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording...".to_string(),
                ));

                let level_active = Arc::new(AtomicBool::new(true));
                self.level_active = Some(level_active.clone());

                let sender_levels = sender.clone();
                let level_active_clone = level_active.clone();
                let task = tokio_handle().spawn(async move {
                    while let Some(levels) = level_rx.recv().await {
                        if level_active_clone.load(Ordering::SeqCst) {
                            let avg = levels.iter().sum::<f32>() / levels.len() as f32;
                            let _ = sender_levels.output(DictationWorkerOutput::AudioLevel(avg));
                        }
                    }
                });
                self.level_task = Some(task.abort_handle());

                // Spawn real-time streaming transcription loop so text appears as the user speaks
                let snapshot_handle = self.recorder.snapshot_handle();
                let ai_stream = self.ai_manager.clone();
                let sender_stream = sender.clone();
                let stream_active = level_active.clone();
                let stream_tone = self.active_tone.clone();
                let is_local_ai = self.config.ai_mode.eq_ignore_ascii_case("local");
                let last_partial_stream = self.last_partial.clone();

                let stream_job = tokio_handle().spawn(async move {
                    let poll_ms = if is_local_ai { 450 } else { 1500 };
                    let mut last_sample_len: usize = 0;

                    loop {
                        tokio::time::sleep(std::time::Duration::from_millis(poll_ms)).await;
                        if !stream_active.load(Ordering::SeqCst) {
                            continue;
                        }

                        let snap = match snapshot_handle.snapshot_wav(60.0) {
                            Ok(Some(res)) => res,
                            _ => continue,
                        };
                        let (wav_bytes, total_samples) = snap;
                        if total_samples <= last_sample_len {
                            continue;
                        }
                        last_sample_len = total_samples;

                        if let Ok(raw_text) = ai_stream.transcribe(&wav_bytes).await {
                            let raw_trimmed = raw_text.trim().to_string();
                            if !raw_trimmed.is_empty() && stream_active.load(Ordering::SeqCst) {
                                let enhanced_text = if stream_tone.eq_ignore_ascii_case("Raw") {
                                    raw_trimmed.clone()
                                } else {
                                    crate::services::ai::utils::apply_smart_local_formatting(
                                        &raw_trimmed,
                                        &stream_tone,
                                    )
                                };
                                if let Ok(mut guard) = last_partial_stream.lock() {
                                    *guard = Some((raw_trimmed.clone(), enhanced_text.clone()));
                                }
                                let _ = sender_stream.output(
                                    DictationWorkerOutput::PartialTranscript {
                                        raw_text: raw_trimmed,
                                    enhanced_text,
                                    },
                                );
                            }
                        }
                    }
                });
                self.stream_task = Some(stream_job.abort_handle());
            }
            DictationWorkerInput::PauseRecording => {
                if !self.is_recording || self.is_paused {
                    return;
                }
                self.recorder.pause();
                self.is_paused = true;
                if let Some(ref active) = self.level_active {
                    active.store(false, Ordering::SeqCst);
                }
                let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                let _ = sender.output(DictationWorkerOutput::RecordingPaused);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording paused".to_string(),
                ));
            }
            DictationWorkerInput::ResumeRecording => {
                if !self.is_recording || !self.is_paused {
                    return;
                }
                self.recorder.resume();
                self.is_paused = false;
                if let Some(ref active) = self.level_active {
                    active.store(true, Ordering::SeqCst);
                }
                let _ = sender.output(DictationWorkerOutput::RecordingResumed);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording...".to_string(),
                ));
            }
            DictationWorkerInput::CancelRecording => {
                let _ = self.recorder.stop();
                self.is_recording = false;
                self.is_paused = false;
                self.start_time = None;
                if let Some(active) = self.level_active.take() {
                    active.store(false, Ordering::SeqCst);
                }
                if let Some(handle) = self.level_task.take() {
                    handle.abort();
                }
                if let Some(handle) = self.stream_task.take() {
                    handle.abort();
                }
                if let Ok(mut guard) = self.last_partial.lock() {
                    *guard = None;
                }
                let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                let _ = sender.output(DictationWorkerOutput::RecordingCancelled);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording cancelled".to_string(),
                ));
            }
            DictationWorkerInput::ToggleRecording { tone } => {
                self.active_tone = tone.clone();
                if self.is_recording {
                    self.update(DictationWorkerInput::StopAndProcess { tone }, sender);
                } else {
                    self.update(DictationWorkerInput::StartRecording, sender);
                }
            }
            DictationWorkerInput::StopAndProcess { tone } => {
                if !self.is_recording {
                    return;
                }
                self.is_recording = false;
                self.is_paused = false;
                let duration = self
                    .start_time
                    .map(|t| t.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                self.start_time = None;
                if let Some(active) = self.level_active.take() {
                    active.store(false, Ordering::SeqCst);
                }
                if let Some(handle) = self.level_task.take() {
                    handle.abort();
                }
                if let Some(handle) = self.stream_task.take() {
                    handle.abort();
                }
                let cached_partial = self
                    .last_partial
                    .lock()
                    .ok()
                    .and_then(|mut g| g.take());

                let wav_bytes = match self.recorder.stop() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        crate::services::crash_reporter::CrashReporter::record_event(
                            "AudioCapture",
                            &format!("Audio stop error: {}", e),
                        );
                        let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                        let _ = sender.output(DictationWorkerOutput::Error(format!(
                            "Audio stop error: {}",
                            e
                        )));
                        return;
                    }
                };

                let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                let _ = sender.output(DictationWorkerOutput::ProcessingStarted);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Transcribing...".to_string(),
                ));

                if wav_bytes.is_empty() || wav_bytes.len() <= 44 {
                    if let Some((partial_raw, partial_enhanced)) = cached_partial
                        && !partial_raw.trim().is_empty()
                    {
                        let _ = self.storage.insert_dictation(
                            &partial_raw,
                            &partial_enhanced,
                            &tone,
                            "dictation",
                            duration,
                        );
                        let _ = sender.output(DictationWorkerOutput::Success {
                            raw_text: partial_raw,
                            enhanced_text: partial_enhanced,
                            duration_seconds: duration,
                        });
                        return;
                    }
                    crate::services::crash_reporter::CrashReporter::record_event(
                        "AudioCapture",
                        &format!(
                            "No speech / 0 audio frames captured ({} bytes over {:.2}s)",
                            wav_bytes.len(),
                            duration
                        ),
                    );
                    let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                    let _ = sender.output(DictationWorkerOutput::StatusMessage(
                        "No speech detected".to_string(),
                    ));
                    let _ = sender.output(DictationWorkerOutput::NoSpeechDetected);
                    return;
                }

                // Asynchronous AI transcribe + enhance in Tokio runtime
                let ai = self.ai_manager.clone();
                let storage = self.storage.clone();
                let sender_clone = sender.clone();

                tokio_handle().spawn(async move {
                    let raw_result = ai.transcribe(&wav_bytes).await;
                    let raw_trimmed = match raw_result {
                        Ok(raw_text) if !raw_text.trim().is_empty() => raw_text.trim().to_string(),
                        Ok(_) => {
                            if let Some((ref partial_raw, _)) = cached_partial
                                && !partial_raw.trim().is_empty()
                            {
                                partial_raw.trim().to_string()
                            } else {
                                crate::services::crash_reporter::CrashReporter::record_event(
                                    "AIEngine",
                                    "Transcription returned empty text",
                                );
                                let _ = sender_clone.output(DictationWorkerOutput::AudioLevel(0.0));
                                let _ = sender_clone.output(DictationWorkerOutput::StatusMessage(
                                    "No speech detected".to_string(),
                                ));
                                let _ =
                                    sender_clone.output(DictationWorkerOutput::NoSpeechDetected);
                                return;
                            }
                        }
                        Err(e) => {
                            if let Some((ref partial_raw, _)) = cached_partial
                                && !partial_raw.trim().is_empty()
                            {
                                partial_raw.trim().to_string()
                            } else {
                                crate::services::crash_reporter::CrashReporter::record_event(
                                    "AIEngine",
                                    &format!("Transcription failed: {}", e),
                                );
                                let _ = sender_clone.output(DictationWorkerOutput::Error(
                                    format!("AI error: {}", e),
                                ));
                                return;
                            }
                        }
                    };

                    let enhanced_text = if tone.eq_ignore_ascii_case("Raw") {
                        raw_trimmed.clone()
                    } else {
                        ai.enhance(&raw_trimmed, &tone).await.unwrap_or_else(|e| {
                            crate::services::crash_reporter::CrashReporter::record_event(
                                "AIEngine",
                                &format!("Enhancement fallback due to error: {}", e),
                            );
                            raw_trimmed.clone()
                        })
                    };

                    let _ = storage.insert_dictation(
                        &raw_trimmed,
                        &enhanced_text,
                        &tone,
                        "dictation",
                        duration,
                    );

                    let _ = sender_clone.output(DictationWorkerOutput::Success {
                        raw_text: raw_trimmed,
                        enhanced_text,
                        duration_seconds: duration,
                    });
                });
            }
        }
    }
}
