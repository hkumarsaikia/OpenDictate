//! Background Relm4 Worker managing microphone capture, AI transcription, SQLite persistence, and text injection.

use crate::audio::recorder::AudioRecorder;
use crate::config::Config;
use crate::services::ai::AiManager;
use crate::services::storage::StorageService;
use relm4::Worker;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;
use tokio::runtime::{Handle, Runtime};

static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();
static GLOBAL_SESSION_GATE: OnceLock<SessionCommitGate> = OnceLock::new();

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

/// Returns the shared process-wide `SessionCommitGate` used to coordinate cancellation
/// and session validity between `DictationWorker` and the UI thread.
pub fn global_session_gate() -> &'static SessionCommitGate {
    GLOBAL_SESSION_GATE.get_or_init(SessionCommitGate::new)
}

/// Explicit lifecycle state of the dictation worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecordingLifecycleState {
    #[default]
    Idle,
    Recording,
    Paused,
    Processing,
}

/// Monotonic session commit gate ensuring stale asynchronous transcription or enhancement
/// results can never overwrite clipboard, SQLite history, or UI state after cancellation
/// or after a newer dictation session starts.
#[derive(Debug, Clone)]
pub struct SessionCommitGate {
    active_session_id: Arc<AtomicU64>,
    output_session_id: Arc<AtomicU64>,
}

impl Default for SessionCommitGate {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionCommitGate {
    /// Creates a new `SessionCommitGate` starting at session `0`.
    pub fn new() -> Self {
        Self {
            active_session_id: Arc::new(AtomicU64::new(0)),
            output_session_id: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Advances to a new monotonic session ID, invalidating all in-flight work from prior sessions.
    pub fn advance(&self) -> u64 {
        self.active_session_id.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Returns the currently active session ID.
    pub fn active_session_id(&self) -> u64 {
        self.active_session_id.load(Ordering::SeqCst)
    }

    /// Returns the session ID attached to the most recently emitted asynchronous output.
    pub fn last_output_session_id(&self) -> u64 {
        self.output_session_id.load(Ordering::SeqCst)
    }

    /// Checks whether `session_id` is still the active session without modifying output tags.
    pub fn is_valid_commit(&self, session_id: u64) -> bool {
        session_id != 0 && self.active_session_id.load(Ordering::SeqCst) == session_id
    }

    /// Validates that `session_id` is still active at the final worker commit point and tags the output.
    pub fn validate_and_tag(&self, session_id: u64) -> bool {
        if self.is_valid_commit(session_id) {
            self.output_session_id.store(session_id, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Final UI commit-point check verifying that the most recently tagged output still matches
    /// the currently active session (and was not invalidated while in flight across the channel).
    pub fn is_last_output_valid(&self) -> bool {
        let out_id = self.output_session_id.load(Ordering::SeqCst);
        out_id != 0 && self.active_session_id.load(Ordering::SeqCst) == out_id
    }
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
    state: Arc<Mutex<RecordingLifecycleState>>,
    session_gate: SessionCommitGate,
    start_time: Option<Instant>,
    active_tone: String,
    level_task: Option<tokio::task::AbortHandle>,
    stream_task: Option<tokio::task::AbortHandle>,
    processing_task: Option<tokio::task::AbortHandle>,
    level_active: Option<Arc<AtomicBool>>,
    last_partial: Arc<Mutex<Option<(String, String)>>>,
}

impl std::fmt::Debug for DictationWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DictationWorker")
            .field("lifecycle_state", &self.lifecycle_state())
            .field("is_recording", &self.is_recording())
            .field("is_paused", &self.is_paused())
            .field("active_session_id", &self.active_session_id())
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
            state: Arc::new(Mutex::new(RecordingLifecycleState::Idle)),
            session_gate: global_session_gate().clone(),
            start_time: None,
            active_tone: "Clean".to_string(),
            level_task: None,
            stream_task: None,
            processing_task: None,
            level_active: None,
            last_partial: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns the explicit lifecycle state (`Idle`, `Recording`, `Paused`, `Processing`).
    pub fn lifecycle_state(&self) -> RecordingLifecycleState {
        self.state
            .lock()
            .map(|g| *g)
            .unwrap_or(RecordingLifecycleState::Idle)
    }

    /// Returns true if currently recording (active or paused).
    pub fn is_recording(&self) -> bool {
        matches!(
            self.lifecycle_state(),
            RecordingLifecycleState::Recording | RecordingLifecycleState::Paused
        )
    }

    /// Returns true if recording is paused.
    pub fn is_paused(&self) -> bool {
        self.lifecycle_state() == RecordingLifecycleState::Paused
    }

    /// Returns true if transcription/enhancement processing is currently in flight.
    pub fn is_processing(&self) -> bool {
        self.lifecycle_state() == RecordingLifecycleState::Processing
    }

    /// Returns the current monotonic session ID.
    pub fn active_session_id(&self) -> u64 {
        self.session_gate.active_session_id()
    }

    /// Returns a reference to the worker's `SessionCommitGate`.
    pub fn session_gate(&self) -> &SessionCommitGate {
        &self.session_gate
    }

    /// Returns a reference to the underlying `AudioRecorder`.
    pub fn recorder(&self) -> &AudioRecorder {
        &self.recorder
    }

    /// Returns the currently active enhancement tone.
    pub fn active_tone(&self) -> &str {
        &self.active_tone
    }

    /// Returns the current configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    fn set_lifecycle_state(&self, new_state: RecordingLifecycleState) {
        if let Ok(mut g) = self.state.lock() {
            *g = new_state;
        }
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
                if self.lifecycle_state() != RecordingLifecycleState::Idle {
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
                if self.lifecycle_state() != RecordingLifecycleState::Idle {
                    return;
                }
                if let Ok(mut guard) = self.last_partial.lock() {
                    *guard = None;
                }
                if let Some(handle) = self.stream_task.take() {
                    handle.abort();
                }
                if let Some(handle) = self.processing_task.take() {
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

                let task_session_id = self.session_gate.advance();
                self.set_lifecycle_state(RecordingLifecycleState::Recording);
                self.start_time = Some(Instant::now());
                let _ = sender.output(DictationWorkerOutput::RecordingStarted);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording...".to_string(),
                ));

                let level_active = Arc::new(AtomicBool::new(true));
                self.level_active = Some(level_active.clone());

                let sender_levels = sender.clone();
                let level_active_clone = level_active.clone();
                let gate_levels = self.session_gate.clone();
                let task = tokio_handle().spawn(async move {
                    while let Some(levels) = level_rx.recv().await {
                        if level_active_clone.load(Ordering::SeqCst)
                            && gate_levels.is_valid_commit(task_session_id)
                        {
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
                let gate_stream = self.session_gate.clone();
                let state_stream = self.state.clone();

                let stream_job = tokio_handle().spawn(async move {
                    let poll_ms = if is_local_ai { 450 } else { 1500 };
                    let mut last_sample_len: usize = 0;

                    loop {
                        tokio::time::sleep(std::time::Duration::from_millis(poll_ms)).await;
                        if !gate_stream.is_valid_commit(task_session_id) {
                            break;
                        }
                        if snapshot_handle.take_stream_error() {
                            if gate_stream.validate_and_tag(task_session_id) {
                                stream_active.store(false, Ordering::SeqCst);
                                if let Ok(mut g) = state_stream.lock() {
                                    *g = RecordingLifecycleState::Idle;
                                }
                                let _ =
                                    sender_stream.output(DictationWorkerOutput::AudioLevel(0.0));
                                let _ = sender_stream.output(DictationWorkerOutput::Error(
                                    "Microphone disconnected or audio stream error".to_string(),
                                ));
                            }
                            break;
                        }
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
                            if !raw_trimmed.is_empty()
                                && stream_active.load(Ordering::SeqCst)
                                && gate_stream.validate_and_tag(task_session_id)
                            {
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
                if self.lifecycle_state() != RecordingLifecycleState::Recording {
                    return;
                }
                self.recorder.pause();
                self.set_lifecycle_state(RecordingLifecycleState::Paused);
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
                if self.lifecycle_state() != RecordingLifecycleState::Paused {
                    return;
                }
                self.recorder.resume();
                self.set_lifecycle_state(RecordingLifecycleState::Recording);
                if let Some(ref active) = self.level_active {
                    active.store(true, Ordering::SeqCst);
                }
                let _ = sender.output(DictationWorkerOutput::RecordingResumed);
                let _ = sender.output(DictationWorkerOutput::StatusMessage(
                    "Recording...".to_string(),
                ));
            }
            DictationWorkerInput::CancelRecording => {
                // Advance session_gate immediately so any in-flight blocking Whisper inference
                // or cloud HTTP request cannot commit results after cancellation.
                self.session_gate.advance();
                let _ = self.recorder.stop();
                self.set_lifecycle_state(RecordingLifecycleState::Idle);
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
                if let Some(handle) = self.processing_task.take() {
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
                match self.lifecycle_state() {
                    RecordingLifecycleState::Idle => {
                        self.update(DictationWorkerInput::StartRecording, sender);
                    }
                    RecordingLifecycleState::Recording | RecordingLifecycleState::Paused => {
                        self.update(DictationWorkerInput::StopAndProcess { tone }, sender);
                    }
                    RecordingLifecycleState::Processing => {
                        // Ignore duplicate ToggleRecording while already processing a recording
                    }
                }
            }
            DictationWorkerInput::StopAndProcess { tone } => {
                if !matches!(
                    self.lifecycle_state(),
                    RecordingLifecycleState::Recording | RecordingLifecycleState::Paused
                ) {
                    return;
                }
                self.set_lifecycle_state(RecordingLifecycleState::Processing);
                let task_session_id = self.session_gate.active_session_id();
                let had_stream_error = self.recorder.take_stream_error();

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
                if let Some(handle) = self.processing_task.take() {
                    handle.abort();
                }
                let cached_partial = self.last_partial.lock().ok().and_then(|mut g| g.take());

                let wav_bytes = match self.recorder.stop() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        self.set_lifecycle_state(RecordingLifecycleState::Idle);
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
                        if self.session_gate.validate_and_tag(task_session_id) {
                            self.set_lifecycle_state(RecordingLifecycleState::Idle);
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
                        }
                        return;
                    }
                    self.set_lifecycle_state(RecordingLifecycleState::Idle);
                    if had_stream_error {
                        let _ = sender.output(DictationWorkerOutput::AudioLevel(0.0));
                        let _ = sender.output(DictationWorkerOutput::Error(
                            "Microphone disconnected or audio stream error".to_string(),
                        ));
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
                let gate_proc = self.session_gate.clone();
                let state_proc = self.state.clone();

                let proc_job = tokio_handle().spawn(async move {
                    let raw_result = ai.transcribe(&wav_bytes).await;
                    if !gate_proc.is_valid_commit(task_session_id) {
                        return;
                    }

                    let raw_trimmed = match raw_result {
                        Ok(raw_text) if !raw_text.trim().is_empty() => raw_text.trim().to_string(),
                        Ok(_) => {
                            if let Some((ref partial_raw, _)) = cached_partial
                                && !partial_raw.trim().is_empty()
                            {
                                partial_raw.trim().to_string()
                            } else {
                                if gate_proc.validate_and_tag(task_session_id) {
                                    if let Ok(mut g) = state_proc.lock() {
                                        *g = RecordingLifecycleState::Idle;
                                    }
                                    crate::services::crash_reporter::CrashReporter::record_event(
                                        "AIEngine",
                                        "Transcription returned empty text",
                                    );
                                    let _ =
                                        sender_clone.output(DictationWorkerOutput::AudioLevel(0.0));
                                    let _ =
                                        sender_clone.output(DictationWorkerOutput::StatusMessage(
                                            "No speech detected".to_string(),
                                        ));
                                    let _ = sender_clone
                                        .output(DictationWorkerOutput::NoSpeechDetected);
                                }
                                return;
                            }
                        }
                        Err(e) => {
                            if let Some((ref partial_raw, _)) = cached_partial
                                && !partial_raw.trim().is_empty()
                            {
                                partial_raw.trim().to_string()
                            } else {
                                if gate_proc.validate_and_tag(task_session_id) {
                                    if let Ok(mut g) = state_proc.lock() {
                                        *g = RecordingLifecycleState::Idle;
                                    }
                                    crate::services::crash_reporter::CrashReporter::record_event(
                                        "AIEngine",
                                        &format!("Transcription failed: {}", e),
                                    );
                                    let _ = sender_clone.output(DictationWorkerOutput::Error(
                                        format!("AI error: {}", e),
                                    ));
                                }
                                return;
                            }
                        }
                    };

                    if !gate_proc.is_valid_commit(task_session_id) {
                        return;
                    }

                    let enhanced_text = if tone.eq_ignore_ascii_case("Raw") {
                        raw_trimmed.clone()
                    } else {
                        match ai.enhance(&raw_trimmed, &tone).await {
                            Ok(s) if !s.trim().is_empty() => s,
                            Ok(_) => raw_trimmed.clone(),
                            Err(e) => {
                                crate::services::crash_reporter::CrashReporter::record_event(
                                    "AIEngine",
                                    &format!("Enhancement fallback due to error: {}", e),
                                );
                                raw_trimmed.clone()
                            }
                        }
                    };

                    // Final commit gate: verify session was not cancelled or superseded by a newer session
                    // before writing to SQLite history or emitting Success to UI/clipboard.
                    if !gate_proc.validate_and_tag(task_session_id) {
                        return;
                    }

                    if let Ok(mut g) = state_proc.lock() {
                        *g = RecordingLifecycleState::Idle;
                    }

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
                self.processing_task = Some(proc_job.abort_handle());
            }
        }
    }
}
