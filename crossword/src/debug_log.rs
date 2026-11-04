use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Non-blocking debug logger that writes to logs/debug.log
/// Silently ignores any I/O errors so it never crashes the TUI
pub fn debug_log(message: impl Debug) {
    let _ = std::fs::create_dir_all("logs");
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/debug.log")
    {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(f, "{} {:?}", ts, message);
    }
}

/// Log with Debug formatting
pub fn debug_log_dbg(value: impl Debug) {
    let _ = std::fs::create_dir_all("logs");
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/debug.log")
    {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(f, "{} {:?}", ts, value);
    }
}
