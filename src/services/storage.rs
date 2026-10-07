//! Embedded SQLite storage service for dictation history and meeting records.

use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// A stored dictation history record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DictationRecord {
    pub id: i64,
    pub timestamp: String,
    pub duration_seconds: f64,
    pub raw_text: String,
    pub processed_text: String,
    pub tone: String,
    pub mode: String,
    pub audio_path: Option<String>,
}

/// A stored meeting notes / transcript record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingRecord {
    pub id: i64,
    pub title: String,
    pub created_at: String,
    pub duration_seconds: f64,
    pub transcript: String,
    pub summary: String,
    pub action_items: String,
}

/// Errors that may occur within the storage subsystem.
#[derive(Debug)]
pub enum StorageError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    LockError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Sqlite(err) => write!(f, "Database error: {}", err),
            StorageError::Io(err) => write!(f, "I/O error: {}", err),
            StorageError::LockError(msg) => write!(f, "Storage lock error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StorageError::Sqlite(err) => Some(err),
            StorageError::Io(err) => Some(err),
            StorageError::LockError(_) => None,
        }
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::Sqlite(err)
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::Io(err)
    }
}

/// Thread-safe SQLite storage service.
#[derive(Debug, Clone)]
pub struct StorageService {
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl StorageService {
    /// Returns the default path to the SQLite database (`~/.local/share/opendictate/opendictate.db`).
    pub fn default_db_path() -> PathBuf {
        if let Some(base_dirs) = directories::BaseDirs::new() {
            base_dirs
                .data_dir()
                .join("opendictate")
                .join("opendictate.db")
        } else {
            PathBuf::from(".local/share/opendictate/opendictate.db")
        }
    }

    /// Initializes a new StorageService connected to SQLite at `db_path`.
    /// Creates parent directories and schema tables if they do not exist.
    pub fn new(db_path: &Path) -> Result<Self, StorageError> {
        if let Some(parent) = db_path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }

        let conn = rusqlite::Connection::open(db_path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS dictations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                duration_seconds REAL NOT NULL DEFAULT 0.0,
                raw_text TEXT NOT NULL,
                processed_text TEXT NOT NULL,
                tone TEXT NOT NULL,
                mode TEXT NOT NULL,
                audio_path TEXT
            );

            CREATE TABLE IF NOT EXISTS meetings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                duration_seconds REAL NOT NULL DEFAULT 0.0,
                transcript TEXT NOT NULL,
                summary TEXT NOT NULL,
                action_items TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS app_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Initializes a StorageService using the default SQLite path.
    pub fn default_service() -> Result<Self, StorageError> {
        let path = Self::default_db_path();
        Self::new(&path)
    }

    /// Initializes an in-memory SQLite StorageService, primarily for tests or fallback.
    pub fn in_memory() -> Result<Self, StorageError> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS dictations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                duration_seconds REAL NOT NULL DEFAULT 0.0,
                raw_text TEXT NOT NULL,
                processed_text TEXT NOT NULL,
                tone TEXT NOT NULL,
                mode TEXT NOT NULL,
                audio_path TEXT
            );

            CREATE TABLE IF NOT EXISTS meetings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                duration_seconds REAL NOT NULL DEFAULT 0.0,
                transcript TEXT NOT NULL,
                summary TEXT NOT NULL,
                action_items TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS app_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Inserts a new dictation record without audio path and returns its auto-generated ID.
    pub fn insert_dictation(
        &self,
        raw_text: &str,
        processed_text: &str,
        tone: &str,
        mode: &str,
        duration_seconds: f64,
    ) -> Result<i64, StorageError> {
        self.insert_dictation_with_audio(
            raw_text,
            processed_text,
            tone,
            mode,
            duration_seconds,
            None,
        )
    }

    /// Inserts a new dictation record with optional audio path and returns its auto-generated ID.
    pub fn insert_dictation_with_audio(
        &self,
        raw_text: &str,
        processed_text: &str,
        tone: &str,
        mode: &str,
        duration_seconds: f64,
        audio_path: Option<&str>,
    ) -> Result<i64, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        conn.execute(
            "INSERT INTO dictations (duration_seconds, raw_text, processed_text, tone, mode, audio_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![duration_seconds, raw_text, processed_text, tone, mode, audio_path],
        )?;
        let row_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT OR REPLACE INTO app_meta (key, value) VALUES ('demo_seeded', '1')",
            [],
        )?;
        let _ = conn.pragma_update(None, "wal_checkpoint", "PASSIVE");
        Ok(row_id)
    }

    /// Lists all dictations in descending order of ID.
    pub fn list_dictations(&self) -> Result<Vec<DictationRecord>, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, duration_seconds, raw_text, processed_text, tone, mode, audio_path
             FROM dictations ORDER BY id DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(DictationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                duration_seconds: row.get(2)?,
                raw_text: row.get(3)?,
                processed_text: row.get(4)?,
                tone: row.get(5)?,
                mode: row.get(6)?,
                audio_path: row.get(7)?,
            })
        })?;

        let mut records = Vec::new();
        for record in rows {
            records.push(record?);
        }
        Ok(records)
    }

    /// Retrieves a single dictation by ID.
    pub fn get_dictation(&self, id: i64) -> Result<Option<DictationRecord>, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, duration_seconds, raw_text, processed_text, tone, mode, audio_path
             FROM dictations WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([id], |row| {
            Ok(DictationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                duration_seconds: row.get(2)?,
                raw_text: row.get(3)?,
                processed_text: row.get(4)?,
                tone: row.get(5)?,
                mode: row.get(6)?,
                audio_path: row.get(7)?,
            })
        })?;

        if let Some(record) = rows.next() {
            Ok(Some(record?))
        } else {
            Ok(None)
        }
    }

    /// Permanently deletes a dictation record by ID and flushes the SQLite WAL to disk
    /// so deleted records are never restored across app restarts.
    pub fn delete_dictation(&self, id: i64) -> Result<(), StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        conn.execute("DELETE FROM dictations WHERE id = ?1", [id])?;
        conn.execute(
            "INSERT OR REPLACE INTO app_meta (key, value) VALUES ('demo_seeded', '1')",
            [],
        )?;
        let _ = conn.pragma_update(None, "wal_checkpoint", "TRUNCATE");
        Ok(())
    }

    /// Seeds initial demo dictation records once if never seeded before.
    /// Once seeded, deleting records will permanently remove them across restarts.
    pub fn seed_demo_dictations_if_empty(&self) -> Result<(), StorageError> {
        let (already_seeded, count): (bool, i64) = {
            let conn = self
                .conn
                .lock()
                .map_err(|e| StorageError::LockError(e.to_string()))?;
            let seeded: Option<String> = conn
                .query_row(
                    "SELECT value FROM app_meta WHERE key = 'demo_seeded'",
                    [],
                    |row| row.get(0),
                )
                .ok();
            let c: i64 = conn.query_row("SELECT COUNT(*) FROM dictations", [], |row| row.get(0))?;
            (seeded.as_deref() == Some("1"), c)
        };

        if already_seeded {
            return Ok(());
        }

        if count == 0 {
            self.insert_dictation(
                "Welcome to OpenDictate. Press your shortcut or click the microphone to dictate text.",
                "Welcome to OpenDictate. Press your shortcut or click the microphone to dictate text.",
                "Professional",
                "local",
                12.5,
            )?;
            self.insert_dictation(
                "Testing real time speech to text transcription on desktop Linux with whisper.cpp.",
                "Testing real-time speech-to-text transcription on desktop Linux with whisper.cpp.",
                "Clean",
                "local",
                8.2,
            )?;
            self.insert_dictation(
                "The quick brown fox jumps over the lazy dog. Hover over this dictation to see the broader full transcript in the hover tooltip window.",
                "The quick brown fox jumps over the lazy dog. Hover over this dictation to see the broader full transcript in the hover tooltip window.",
                "Concise",
                "cloud",
                15.0,
            )?;
        }

        {
            let conn = self
                .conn
                .lock()
                .map_err(|e| StorageError::LockError(e.to_string()))?;
            conn.execute(
                "INSERT OR REPLACE INTO app_meta (key, value) VALUES ('demo_seeded', '1')",
                [],
            )?;
        }

        Ok(())
    }

    /// Inserts a new meeting record and returns its auto-generated ID.
    pub fn insert_meeting(
        &self,
        title: &str,
        duration_seconds: f64,
        transcript: &str,
        summary: &str,
        action_items: &str,
    ) -> Result<i64, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        conn.execute(
            "INSERT INTO meetings (title, duration_seconds, transcript, summary, action_items)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![title, duration_seconds, transcript, summary, action_items],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Lists all meetings in descending order of ID.
    pub fn list_meetings(&self) -> Result<Vec<MeetingRecord>, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, created_at, duration_seconds, transcript, summary, action_items
             FROM meetings ORDER BY id DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(MeetingRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                duration_seconds: row.get(3)?,
                transcript: row.get(4)?,
                summary: row.get(5)?,
                action_items: row.get(6)?,
            })
        })?;

        let mut records = Vec::new();
        for record in rows {
            records.push(record?);
        }
        Ok(records)
    }

    /// Retrieves a single meeting by ID.
    pub fn get_meeting(&self, id: i64) -> Result<Option<MeetingRecord>, StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, title, created_at, duration_seconds, transcript, summary, action_items
             FROM meetings WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([id], |row| {
            Ok(MeetingRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                duration_seconds: row.get(3)?,
                transcript: row.get(4)?,
                summary: row.get(5)?,
                action_items: row.get(6)?,
            })
        })?;

        if let Some(record) = rows.next() {
            Ok(Some(record?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a meeting record by ID.
    pub fn delete_meeting(&self, id: i64) -> Result<(), StorageError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| StorageError::LockError(e.to_string()))?;
        conn.execute("DELETE FROM meetings WHERE id = ?1", [id])?;
        Ok(())
    }
}
