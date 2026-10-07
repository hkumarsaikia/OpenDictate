use opendictate::services::storage::StorageService;
use tempfile::NamedTempFile;

#[test]
fn test_storage_crud_lifecycle() {
    let tmp = NamedTempFile::new().expect("failed to create named temp file");
    let storage = StorageService::new(tmp.path()).expect("failed to initialize StorageService");

    // Insert dictation
    let record_id = storage
        .insert_dictation("raw text", "cleaned text", "clean", "dictation", 3.5)
        .expect("failed to insert dictation");
    assert!(record_id > 0);

    // List dictations
    let records = storage
        .list_dictations()
        .expect("failed to list dictations");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].id, record_id);
    assert_eq!(records[0].raw_text, "raw text");
    assert_eq!(records[0].processed_text, "cleaned text");
    assert_eq!(records[0].tone, "clean");
    assert_eq!(records[0].mode, "dictation");
    assert!((records[0].duration_seconds - 3.5).abs() < f64::EPSILON);
    assert_eq!(records[0].audio_path, None);

    // Get dictation
    let fetched = storage
        .get_dictation(record_id)
        .expect("failed to get dictation")
        .expect("dictation not found");
    assert_eq!(fetched.id, record_id);
    assert_eq!(fetched.raw_text, "raw text");

    // Delete dictation
    storage
        .delete_dictation(record_id)
        .expect("failed to delete dictation");
    let records_after = storage
        .list_dictations()
        .expect("failed to list dictations");
    assert_eq!(records_after.len(), 0);

    let fetched_after = storage
        .get_dictation(record_id)
        .expect("failed to get dictation");
    assert!(fetched_after.is_none());
}

#[test]
fn test_storage_meetings_crud_lifecycle() {
    let tmp = NamedTempFile::new().expect("failed to create named temp file");
    let storage = StorageService::new(tmp.path()).expect("failed to initialize StorageService");

    let meeting_id = storage
        .insert_meeting(
            "Design Sync",
            120.0,
            "Bob: Hello. Alice: Hi.",
            "Discussed architecture.",
            "- Alice to write specs",
        )
        .expect("failed to insert meeting");
    assert!(meeting_id > 0);

    let meetings = storage.list_meetings().expect("failed to list meetings");
    assert_eq!(meetings.len(), 1);
    assert_eq!(meetings[0].id, meeting_id);
    assert_eq!(meetings[0].title, "Design Sync");
    assert!((meetings[0].duration_seconds - 120.0).abs() < f64::EPSILON);
    assert_eq!(meetings[0].transcript, "Bob: Hello. Alice: Hi.");
    assert_eq!(meetings[0].summary, "Discussed architecture.");
    assert_eq!(meetings[0].action_items, "- Alice to write specs");

    storage
        .delete_meeting(meeting_id)
        .expect("failed to delete meeting");
    let meetings_after = storage.list_meetings().expect("failed to list meetings");
    assert_eq!(meetings_after.len(), 0);
}

#[test]
fn test_storage_default_db_path() {
    let path = StorageService::default_db_path();
    assert!(
        path.ends_with("opendictate/opendictate.db"),
        "Default DB path should end with opendictate/opendictate.db, got: {:?}",
        path
    );
}

#[test]
fn test_seed_demo_dictations_if_empty() {
    let tmp = NamedTempFile::new().expect("failed to create named temp file");
    let storage = StorageService::new(tmp.path()).expect("failed to initialize StorageService");

    // Initially empty
    let initial = storage
        .list_dictations()
        .expect("failed to list dictations");
    assert_eq!(initial.len(), 0);

    // First call seeds demo records
    storage
        .seed_demo_dictations_if_empty()
        .expect("failed to seed demo dictations");
    let seeded = storage
        .list_dictations()
        .expect("failed to list dictations");
    assert_eq!(seeded.len(), 3);
    assert!(seeded
        .iter()
        .any(|d| d.processed_text.contains("OpenDictate")));
    assert!(seeded
        .iter()
        .any(|d| d.processed_text.contains("whisper.cpp")));
    assert!(seeded
        .iter()
        .any(|d| d.processed_text.contains("quick brown fox")));

    // Second call is idempotent (does not duplicate)
    storage
        .seed_demo_dictations_if_empty()
        .expect("failed to re-seed demo dictations");
    let seeded_again = storage
        .list_dictations()
        .expect("failed to list dictations");
    assert_eq!(seeded_again.len(), 3);

    // Delete 1 record out of the 3 and verify after restart that only 2 remain (deleted one is never restored)
    let first_deleted_id = seeded_again[0].id;
    storage
        .delete_dictation(first_deleted_id)
        .expect("failed to delete single dictation");
    assert_eq!(storage.list_dictations().unwrap().len(), 2);

    let reopened_once = StorageService::new(tmp.path()).expect("failed to reopen StorageService");
    reopened_once
        .seed_demo_dictations_if_empty()
        .expect("failed on reopened seed check");
    let after_first_restart = reopened_once
        .list_dictations()
        .expect("failed to list after restart");
    assert_eq!(
        after_first_restart.len(),
        2,
        "Deleting 1 record must leave 2 records after restart without restoring the deleted one"
    );
    assert!(after_first_restart.iter().all(|r| r.id != first_deleted_id));

    // Delete the remaining 2 records and verify after restart that 0 remain
    for rec in after_first_restart {
        reopened_once
            .delete_dictation(rec.id)
            .expect("failed to delete dictation");
    }
    assert_eq!(reopened_once.list_dictations().unwrap().len(), 0);

    let reopened_twice = StorageService::new(tmp.path()).expect("failed to reopen StorageService");
    reopened_twice
        .seed_demo_dictations_if_empty()
        .expect("failed on reopened seed check");
    let after_second_restart = reopened_twice
        .list_dictations()
        .expect("failed to list after restart");
    assert_eq!(
        after_second_restart.len(),
        0,
        "Deleted history must not be restored when the app is restarted"
    );
}

#[test]
fn test_production_arbitrary_history_deletions_never_restored_across_restarts() {
    let tmp = NamedTempFile::new().expect("failed to create named temp file");

    // Session 1: User dictates N arbitrary production history records (e.g. 8 records)
    let mut inserted_ids = Vec::new();
    {
        let storage = StorageService::new(tmp.path()).expect("failed to initialize StorageService");
        for i in 1..=8 {
            let id = storage
                .insert_dictation(
                    &format!("Raw production dictation number {}", i),
                    &format!(
                        "Processed production dictation number {} with extra words",
                        i
                    ),
                    "Clean",
                    "local",
                    i as f64 * 1.5,
                )
                .expect("failed to insert production dictation");
            inserted_ids.push(id);
        }
        assert_eq!(storage.list_dictations().unwrap().len(), 8);

        // User deletes 3 specific records (e.g. #2, #5, #7)
        for &del_id in &[inserted_ids[1], inserted_ids[4], inserted_ids[6]] {
            storage
                .delete_dictation(del_id)
                .expect("failed to delete dictation");
        }
        assert_eq!(storage.list_dictations().unwrap().len(), 5);
    }

    // Session 2 (App Restart): Verify the 3 deleted records are NOT restored and exact 5 remain
    {
        let storage = StorageService::new(tmp.path()).expect("failed to reopen StorageService");
        storage
            .seed_demo_dictations_if_empty()
            .expect("startup seed check");
        let remaining = storage.list_dictations().expect("list dictations");
        assert_eq!(remaining.len(), 5);
        for &del_id in &[inserted_ids[1], inserted_ids[4], inserted_ids[6]] {
            assert!(
                remaining.iter().all(|r| r.id != del_id),
                "Deleted record ID {} must never be restored after restart",
                del_id
            );
        }

        // User dictates 2 more new records (#9, #10), then deletes 4 records
        let id9 = storage
            .insert_dictation("Raw 9", "Processed 9", "Professional", "cloud", 4.0)
            .unwrap();
        let id10 = storage
            .insert_dictation("Raw 10", "Processed 10", "Concise", "local", 5.0)
            .unwrap();
        assert_eq!(storage.list_dictations().unwrap().len(), 7);

        storage.delete_dictation(inserted_ids[0]).unwrap();
        storage.delete_dictation(inserted_ids[2]).unwrap();
        storage.delete_dictation(id9).unwrap();
        assert_eq!(storage.list_dictations().unwrap().len(), 4);

        // Verify id10 is still present
        assert!(storage.get_dictation(id10).unwrap().is_some());
    }

    // Session 3 (App Restart): Verify exact 4 records persist, then delete all 4 down to 0
    {
        let storage = StorageService::new(tmp.path()).expect("failed to reopen StorageService");
        storage
            .seed_demo_dictations_if_empty()
            .expect("startup seed check");
        let remaining = storage.list_dictations().expect("list dictations");
        assert_eq!(remaining.len(), 4);

        for rec in remaining {
            storage.delete_dictation(rec.id).unwrap();
        }
        assert_eq!(storage.list_dictations().unwrap().len(), 0);
    }

    // Session 4 (App Restart): Verify 0 records remain and nothing is restored
    {
        let storage = StorageService::new(tmp.path()).expect("failed to reopen StorageService");
        storage
            .seed_demo_dictations_if_empty()
            .expect("startup seed check");
        let remaining = storage.list_dictations().expect("list dictations");
        assert_eq!(remaining.len(), 0);
    }
}
