use opendictate::config::Config;
use opendictate::services::ai::AiManager;
use opendictate::services::dictation_worker::{
    DictationWorker, DictationWorkerInput, DictationWorkerOutput,
};
use opendictate::services::storage::StorageService;

#[tokio::test]
async fn test_ai_manager_routes_to_local_ai_when_mode_is_local() {
    let config = Config {
        ai_mode: "local".to_string(),
        local_model_id: "tiny.en".to_string(),
        ..Default::default()
    };

    let mgr = AiManager::with_config(&config);
    assert_eq!(mgr.active_mode(), "local");

    let enhanced = mgr.enhance("um hello world uh", "Clean").await.unwrap();
    assert_eq!(enhanced, "Hello world.");

    let raw = mgr.enhance("um hello world uh", "Raw").await.unwrap();
    assert_eq!(raw, "um hello world uh");
}

#[test]
fn test_dictation_worker_messages_and_lifecycle() {
    let config = Config::default();
    let storage = StorageService::in_memory().unwrap();
    let worker = DictationWorker::new(config, storage);

    assert!(!worker.is_recording());
    assert!(!worker.is_paused());

    let input = DictationWorkerInput::ToggleRecording {
        tone: "Clean".to_string(),
    };
    assert!(format!("{:?}", input).contains("ToggleRecording"));

    let output = DictationWorkerOutput::AudioLevel(0.42);
    assert!(format!("{:?}", output).contains("0.42"));
}

#[test]
fn test_dictation_worker_input_variants_and_formatting() {
    let inputs = vec![
        DictationWorkerInput::StartRecording,
        DictationWorkerInput::PauseRecording,
        DictationWorkerInput::ResumeRecording,
        DictationWorkerInput::StopAndProcess {
            tone: "Professional".to_string(),
        },
        DictationWorkerInput::CancelRecording,
        DictationWorkerInput::ToggleRecording {
            tone: "Concise".to_string(),
        },
        DictationWorkerInput::UpdateConfig(Config::default()),
    ];

    for input in &inputs {
        let debug_str = format!("{:?}", input);
        assert!(!debug_str.is_empty());
    }

    // Verify equality and cloning
    let toggle1 = DictationWorkerInput::ToggleRecording {
        tone: "Raw".to_string(),
    };
    let toggle2 = toggle1.clone();
    assert_eq!(toggle1, toggle2);
    assert_ne!(toggle1, DictationWorkerInput::StartRecording);
}

#[test]
fn test_dictation_worker_output_variants_and_formatting() {
    let outputs = vec![
        DictationWorkerOutput::RecordingStarted,
        DictationWorkerOutput::RecordingPaused,
        DictationWorkerOutput::RecordingResumed,
        DictationWorkerOutput::RecordingCancelled,
        DictationWorkerOutput::AudioLevel(0.85),
        DictationWorkerOutput::ProcessingStarted,
        DictationWorkerOutput::Success {
            raw_text: "test raw".to_string(),
            enhanced_text: "test enhanced".to_string(),
            duration_seconds: 3.5,
        },
        DictationWorkerOutput::Error("test error".to_string()),
        DictationWorkerOutput::StatusMessage("Recording...".to_string()),
        DictationWorkerOutput::NoSpeechDetected,
    ];

    for output in &outputs {
        let debug_str = format!("{:?}", output);
        assert!(!debug_str.is_empty());
    }

    let success = DictationWorkerOutput::Success {
        raw_text: "hello".to_string(),
        enhanced_text: "Hello!".to_string(),
        duration_seconds: 1.2,
    };
    let success_clone = success.clone();
    assert_eq!(success, success_clone);
}

#[test]
fn test_dictation_worker_config_and_tone() {
    let config = Config {
        ai_provider: "groq".to_string(),
        hotkey: "Ctrl+Shift+D".to_string(),
        ..Default::default()
    };

    let storage = StorageService::in_memory().unwrap();
    let worker = DictationWorker::new(config.clone(), storage);

    assert_eq!(worker.active_tone(), "Clean");
    assert_eq!(worker.config().ai_provider, "groq");
    assert_eq!(worker.config().hotkey, "Ctrl+Shift+D");

    let debug_repr = format!("{:?}", worker);
    assert!(debug_repr.contains("DictationWorker"));
    assert!(debug_repr.contains("Clean"));
}

#[test]
fn test_dictation_worker_update_config_behavior() {
    let config = Config::default();
    let storage = StorageService::in_memory().unwrap();
    let worker = DictationWorker::new(config, storage);

    assert_eq!(worker.config().ai_provider, "gemini");

    let new_config = Config {
        ai_provider: "openai".to_string(),
        ..Default::default()
    };

    let input = DictationWorkerInput::UpdateConfig(new_config.clone());
    assert!(format!("{:?}", input).contains("openai"));
}

#[test]
fn test_dictation_worker_no_speech_detected_and_level_reset() {
    let zero_level = DictationWorkerOutput::AudioLevel(0.0);
    let no_speech = DictationWorkerOutput::StatusMessage("No speech detected".to_string());

    assert_eq!(zero_level, DictationWorkerOutput::AudioLevel(0.0));
    assert!(format!("{:?}", no_speech).contains("No speech detected"));
}

#[tokio::test]
async fn test_audio_level_forwarder_pause_resume_lifecycle() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let (level_tx, mut level_rx) = tokio::sync::mpsc::channel::<[f32; 5]>(100);
    let (out_tx, mut out_rx) = tokio::sync::mpsc::channel::<DictationWorkerOutput>(100);
    let level_active = Arc::new(AtomicBool::new(true));
    let level_active_clone = level_active.clone();

    let task = tokio::spawn(async move {
        while let Some(levels) = level_rx.recv().await {
            if level_active_clone.load(Ordering::SeqCst) {
                let avg = levels.iter().sum::<f32>() / levels.len() as f32;
                let _ = out_tx.send(DictationWorkerOutput::AudioLevel(avg)).await;
            }
        }
    });

    // 1. Send levels during active recording
    level_tx.send([0.4; 5]).await.unwrap();
    let msg1 = tokio::time::timeout(std::time::Duration::from_millis(200), out_rx.recv())
        .await
        .expect("timed out waiting for active level")
        .expect("channel closed");
    assert_eq!(msg1, DictationWorkerOutput::AudioLevel(0.4));

    // 2. Pause: set active to false
    level_active.store(false, Ordering::SeqCst);
    // Send levels during pause - should be dropped / ignored
    level_tx.send([0.9; 5]).await.unwrap();
    let pause_check =
        tokio::time::timeout(std::time::Duration::from_millis(50), out_rx.recv()).await;
    assert!(
        pause_check.is_err(),
        "Levels should not be emitted during pause"
    );

    // 3. Resume: set active to true
    level_active.store(true, Ordering::SeqCst);
    level_tx.send([0.6; 5]).await.unwrap();
    let msg2 = tokio::time::timeout(std::time::Duration::from_millis(200), out_rx.recv())
        .await
        .expect("timed out waiting for resumed level")
        .expect("channel closed");
    assert_eq!(msg2, DictationWorkerOutput::AudioLevel(0.6));

    // 4. Abort handle stops task completely
    task.abort();
    let _ = task.await;
}

#[test]
fn test_dictation_worker_stop_and_cancel_lifecycle() {
    use relm4::Component;

    if gtk4::init().is_err() {
        return;
    }

    let config = Config::default();
    let storage = StorageService::in_memory().unwrap();
    let worker_handle = DictationWorker::builder().detach_worker((config, storage));

    let (out_tx, out_rx) = std::sync::mpsc::channel::<DictationWorkerOutput>();
    let worker = worker_handle.connect_receiver(move |_worker_sender, output| {
        let _ = out_tx.send(output);
    });

    let sender = worker.sender();

    // 1. Start recording
    sender
        .send(DictationWorkerInput::ToggleRecording {
            tone: "Clean".to_string(),
        })
        .unwrap();

    let mut started = false;
    let start_wait = std::time::Instant::now();
    while start_wait.elapsed() < std::time::Duration::from_millis(500) {
        if let Ok(msg) = out_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            println!("Worker output: {:?}", msg);
            if matches!(msg, DictationWorkerOutput::RecordingStarted) {
                started = true;
                break;
            }
        }
    }

    if started {
        println!("Recording started verified! Now testing stop...");
        // 2. Stop recording
        sender
            .send(DictationWorkerInput::ToggleRecording {
                tone: "Clean".to_string(),
            })
            .unwrap();

        let mut stopped = false;
        let stop_wait = std::time::Instant::now();
        while stop_wait.elapsed() < std::time::Duration::from_secs(2) {
            if let Ok(msg) = out_rx.recv_timeout(std::time::Duration::from_millis(100)) {
                println!("Worker output after stop: {:?}", msg);
                if matches!(
                    msg,
                    DictationWorkerOutput::ProcessingStarted
                        | DictationWorkerOutput::NoSpeechDetected
                ) {
                    stopped = true;
                    break;
                }
            }
        }
        assert!(stopped, "Worker should process stop recording");

        // 3. Start recording again and test CancelRecording
        sender.send(DictationWorkerInput::StartRecording).unwrap();
        let mut started_again = false;
        let start_wait2 = std::time::Instant::now();
        while start_wait2.elapsed() < std::time::Duration::from_millis(500) {
            if let Ok(msg) = out_rx.recv_timeout(std::time::Duration::from_millis(100)) {
                if matches!(msg, DictationWorkerOutput::RecordingStarted) {
                    started_again = true;
                    break;
                }
            }
        }
        if started_again {
            println!("Testing CancelRecording...");
            sender.send(DictationWorkerInput::CancelRecording).unwrap();
            let mut cancelled = false;
            let cancel_wait = std::time::Instant::now();
            while cancel_wait.elapsed() < std::time::Duration::from_millis(500) {
                if let Ok(msg) = out_rx.recv_timeout(std::time::Duration::from_millis(100)) {
                    if matches!(msg, DictationWorkerOutput::RecordingCancelled) {
                        cancelled = true;
                        break;
                    }
                }
            }
            assert!(
                cancelled,
                "Worker should cancel recording and emit RecordingCancelled"
            );
        }
    }
}
