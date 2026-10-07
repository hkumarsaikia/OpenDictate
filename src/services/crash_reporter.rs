//! Crash detection, panic hook persistence, and 1-click GitHub crash report submission.
//!
//! Captures unexpected Rust panics with sanitized stack backtraces and hardware/audio
//! diagnostics to `~/.local/share/opendictate/last_crash.log`. On the next application launch,
//! OpenDictate presents a native Libadwaita recovery window where the user can inspect,
//! edit, and submit the crash report directly to GitHub Issues with a single click via
//! the OpenDictate Cloudflare Worker relay.

use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_DIAGNOSTIC_EVENTS: usize = 25;

static DIAGNOSTIC_EVENTS: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

fn events_buffer() -> &'static Mutex<VecDeque<String>> {
    DIAGNOSTIC_EVENTS.get_or_init(|| Mutex::new(VecDeque::with_capacity(MAX_DIAGNOSTIC_EVENTS)))
}

/// Public Cloudflare Worker relay endpoint that forwards sanitized crash reports
/// to `https://github.com/hkumarsaikia/OpenDictate/issues` using a server-side
/// fine-grained token scoped strictly to `Issues: Write`.
pub const DEFAULT_CRASH_RELAY_URL: &str =
    "https://opendictate-crash-relay.hkumarsaikia.workers.dev";

/// Fallback GitHub Issues new-issue URL if the user is offline or prefers browser submission.
pub const GITHUB_NEW_ISSUE_URL: &str = "https://github.com/hkumarsaikia/OpenDictate/issues/new";

/// Structured crash report persisted to disk on panic and loaded into the recovery UI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrashReport {
    /// Default issue title (e.g., `Crash Report: v2.0.0 panic at src/audio/capture.rs:142`).
    pub title: String,
    /// Source location (`file:line:col`) where the panic occurred.
    pub location: String,
    /// Sanitized panic payload message.
    pub message: String,
    /// UTC timestamp when the crash occurred.
    pub timestamp_utc: String,
    /// Full editable Markdown report body (user notes placeholder, diagnostics, and backtrace).
    pub body: String,
}

/// Confirmation returned by the Cloudflare Worker relay when a GitHub Issue is created.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrashSubmissionResult {
    /// GitHub Issue number (e.g., `42`).
    pub issue_number: u64,
    /// Full URL to the created GitHub Issue.
    pub issue_url: String,
}

#[derive(Debug, Deserialize)]
struct RelayResponse {
    ok: Option<bool>,
    issue_number: Option<u64>,
    url: Option<String>,
    error: Option<String>,
}

/// Service managing local crash persistence, subsystem diagnostic telemetry, and remote report submission.
pub struct CrashReporter;

impl CrashReporter {
    /// Records a timestamped runtime diagnostic event (e.g. audio stream failure, zero-frame capture,
    /// AI provider error, or hotkey portal failure) into the in-memory diagnostic ring buffer
    /// so it is included in crash and troubleshooting reports.
    pub fn record_event(subsystem: &str, detail: &str) {
        let entry = format!(
            "[{}] [{}] {}",
            Self::current_utc_timestamp(),
            subsystem.trim(),
            Self::anonymize_text(detail.trim())
        );
        if let Ok(mut buf) = events_buffer().lock() {
            if buf.len() >= MAX_DIAGNOSTIC_EVENTS {
                buf.pop_front();
            }
            buf.push_back(entry);
        }
    }

    /// Returns a formatted multi-line log of all recorded runtime subsystem events in the current session.
    pub fn recent_events_summary() -> String {
        if let Ok(buf) = events_buffer().lock() {
            if buf.is_empty() {
                "No subsystem errors recorded in current session".to_string()
            } else {
                buf.iter().cloned().collect::<Vec<_>>().join("\n")
            }
        } else {
            "Unavailable".to_string()
        }
    }
    /// Returns the path to the pending (unacknowledged) crash report file.
    ///
    /// Respects `OPENDICTATE_CRASH_LOG_PATH` when set (used by automated tests).
    pub fn crash_log_path() -> PathBuf {
        if let Ok(custom) = std::env::var("OPENDICTATE_CRASH_LOG_PATH")
            && !custom.trim().is_empty()
        {
            return PathBuf::from(custom);
        }
        Self::default_data_dir().join("last_crash.log")
    }

    /// Returns the path to the archived crash report file (after dismissal or submission).
    pub fn archived_crash_log_path() -> PathBuf {
        let active = Self::crash_log_path();
        if let Some(parent) = active.parent() {
            parent.join("previous_crash.log")
        } else {
            Self::default_data_dir().join("previous_crash.log")
        }
    }

    /// Returns the active Cloudflare Worker crash relay URL.
    pub fn relay_url() -> String {
        if let Ok(custom) = std::env::var("OPENDICTATE_CRASH_RELAY_URL")
            && !custom.trim().is_empty()
        {
            return custom;
        }
        DEFAULT_CRASH_RELAY_URL.to_string()
    }

    fn default_data_dir() -> PathBuf {
        directories::BaseDirs::new()
            .map(|b| b.data_local_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("opendictate")
    }

    /// Scrubs `/home/<username>` prefixes from arbitrary text or stack traces, replacing them with `~`.
    pub fn anonymize_text(input: &str) -> String {
        let mut result = input.to_string();
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let home = base_dirs.home_dir().to_string_lossy().to_string();
            if !home.is_empty() && home != "/" {
                result = result.replace(&home, "~");
            }
        }
        if let Ok(home) = std::env::var("HOME")
            && !home.is_empty()
            && home != "/"
        {
            result = result.replace(&home, "~");
        }
        result
    }

    /// Formats a current UTC timestamp in ISO-8601 format without external chrono dependencies.
    pub fn current_utc_timestamp() -> String {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        // Convert UNIX epoch seconds to YYYY-MM-DDTHH:MM:SSZ
        let days = secs / 86_400;
        let rem = secs % 86_400;
        let hour = rem / 3600;
        let minute = (rem % 3600) / 60;
        let second = rem % 60;

        // Civil date calculation from days since 1970-01-01 (Howard Hinnant algorithm)
        let z = days as i64 + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = (z - era * 146_097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if m <= 2 { y + 1 } else { y };

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            year, m, d, hour, minute, second
        )
    }

    /// Constructs a structured, privacy-sanitized [`CrashReport`] ready for user review and editing.
    pub fn build_report(
        panic_message: &str,
        location: &str,
        backtrace: &str,
        config: &Config,
    ) -> CrashReport {
        let clean_msg = Self::anonymize_text(panic_message.trim());
        let clean_loc = Self::anonymize_text(location.trim());
        let clean_bt = Self::anonymize_text(backtrace.trim());
        let timestamp_utc = Self::current_utc_timestamp();
        let diagnostics =
            crate::ui::main_window::header::generate_troubleshooting_diagnostics(config);

        let short_msg: String = clean_msg.chars().take(72).collect();
        let title = format!(
            "Crash Report: OpenDictate v{} ({}) — {}",
            crate::VERSION,
            clean_loc,
            short_msg
        );

        let body = format!(
            "### User Notes (Editable)\n\
             <!-- Optional: Describe what you were doing when OpenDictate crashed, or edit any details below -->\n\
             Application crashed unexpectedly during runtime.\n\n\
             ### Crash Summary\n\
             - **Application**: OpenDictate v{ver}\n\
             - **Timestamp (UTC)**: {ts}\n\
             - **Location**: `{loc}`\n\
             - **Panic Message**: `{msg}`\n\n\
             ### System & Audio Diagnostics\n\
             ```text\n\
             {diag}\
             ```\n\n\
             ### Stack Backtrace\n\
             ```text\n\
             {bt}\n\
             ```\n",
            ver = crate::VERSION,
            ts = timestamp_utc,
            loc = clean_loc,
            msg = clean_msg,
            diag = diagnostics,
            bt = if clean_bt.is_empty() {
                "<backtrace unavailable>"
            } else {
                &clean_bt
            }
        );

        CrashReport {
            title,
            location: clean_loc,
            message: clean_msg,
            timestamp_utc,
            body,
        }
    }

    /// Persists a [`CrashReport`] atomically to `last_crash.log`.
    pub fn write_pending_crash(report: &CrashReport) -> std::io::Result<()> {
        let path = Self::crash_log_path();
        Self::write_crash_to_path(&path, report)
    }

    /// Persists a [`CrashReport`] to a specific file path.
    pub fn write_crash_to_path(path: &Path, report: &CrashReport) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(report).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    /// Loads the pending crash report from `last_crash.log`, if one exists.
    pub fn load_pending_crash() -> Option<CrashReport> {
        let path = Self::crash_log_path();
        Self::load_crash_from_path(&path)
    }

    /// Loads a [`CrashReport`] from a specific path, supporting both structured JSON and plain-text logs.
    pub fn load_crash_from_path(path: &Path) -> Option<CrashReport> {
        let raw = std::fs::read_to_string(path).ok()?;
        if raw.trim().is_empty() {
            return None;
        }
        if let Ok(report) = serde_json::from_str::<CrashReport>(&raw) {
            return Some(report);
        }
        // Fallback if plain text was written
        Some(CrashReport {
            title: format!("Crash Report: OpenDictate v{}", crate::VERSION),
            location: "unknown".to_string(),
            message: "Unexpected termination".to_string(),
            timestamp_utc: Self::current_utc_timestamp(),
            body: Self::anonymize_text(&raw),
        })
    }

    /// Loads the most recent crash summary (either pending or previously archived) for
    /// inclusion in `About OpenDictate -> Troubleshooting -> Debugging Information`.
    pub fn load_latest_crash_summary() -> Option<String> {
        let report = Self::load_pending_crash()
            .or_else(|| Self::load_crash_from_path(&Self::archived_crash_log_path()))?;
        Some(format!(
            "Timestamp: {}\nLocation: {}\nMessage: {}",
            report.timestamp_utc, report.location, report.message
        ))
    }

    /// Archives `last_crash.log` to `previous_crash.log` so the startup dialog only prompts once per crash.
    pub fn archive_pending_crash() -> std::io::Result<()> {
        let pending = Self::crash_log_path();
        let archived = Self::archived_crash_log_path();
        if pending.exists() {
            if let Some(parent) = archived.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::rename(&pending, &archived)?;
        }
        Ok(())
    }

    /// Installs the global Rust panic hook (`std::panic::set_hook`) that captures stack backtraces
    /// and system diagnostics to `last_crash.log` before delegating to the standard panic handler.
    pub fn install_panic_hook() {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };

            let location = panic_info
                .location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "unknown:0:0".to_string());

            let backtrace = std::backtrace::Backtrace::force_capture().to_string();
            let config = Config::load_or_default();
            let report = Self::build_report(&payload, &location, &backtrace, &config);

            if let Err(e) = Self::write_pending_crash(&report) {
                eprintln!("OpenDictate: failed to write crash report: {}", e);
            }

            default_hook(panic_info);
        }));
    }

    /// Sends the user-reviewed and potentially edited crash report directly to the
    /// OpenDictate Cloudflare Worker relay, which creates an issue on GitHub.
    pub async fn send_crash_report(
        title: &str,
        edited_body: &str,
    ) -> Result<CrashSubmissionResult, String> {
        let clean_title = Self::anonymize_text(title.trim());
        let mut clean_body = Self::anonymize_text(edited_body.trim());
        // Ensure the body contains the "OpenDictate" identifier required by the relay validator
        if !clean_body.contains("OpenDictate") {
            clean_body = format!(
                "OpenDictate v{} Crash Report\n\n{}",
                crate::VERSION,
                clean_body
            );
        }

        let payload = serde_json::json!({
            "title": if clean_title.is_empty() {
                format!("Crash Report: OpenDictate v{}", crate::VERSION)
            } else {
                clean_title
            },
            "body": clean_body,
            "version": crate::VERSION,
        });

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| format!("Failed to initialize HTTP client: {}", e))?;

        let response = client
            .post(Self::relay_url())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error contacting crash relay: {}", e))?;

        let status = response.status();
        let parsed: RelayResponse = response
            .json()
            .await
            .map_err(|e| format!("Invalid response from crash relay ({}): {}", status, e))?;

        if !status.is_success() || parsed.ok != Some(true) {
            return Err(parsed
                .error
                .unwrap_or_else(|| format!("Crash relay returned HTTP {}", status)));
        }

        let issue_number = parsed.issue_number.unwrap_or(0);
        let issue_url = parsed.url.unwrap_or_else(|| {
            format!(
                "https://github.com/hkumarsaikia/OpenDictate/issues/{}",
                issue_number
            )
        });

        Ok(CrashSubmissionResult {
            issue_number,
            issue_url,
        })
    }
}
