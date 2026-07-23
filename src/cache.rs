//! The `.day/` render cache — **display-only, latency-only**.
//!
//! Claude Code re-runs a status line on session start, on every new assistant
//! message, on `/compact`, on permission-mode changes, and more; it debounces
//! at 300 ms and **cancels an in-flight script when a new update arrives**. A
//! status line that shells out to kan does not merely lag — it can be
//! cancelled before it renders anything, repeatedly, and show nothing at all.
//! So the expensive kan/git reads happen in `day hook session-start`, which
//! already runs and has time, and the rendered line is written here for the
//! status line to read back instantly.
//!
//! **This module is the only one that touches `.day/`.** `tests/plugin.rs`
//! greps the rest of `src/` for the cache path and asserts it appears nowhere
//! else — the guardrail that keeps *"display only"* from decaying into *"and
//! also decides things"*. If day ever read this cache to decide something
//! rather than to display something, the line `telos/no-store-of-its-own`
//! draws would have been crossed.
//!
//! **Why this is not a store** (`docs/ROADMAP.md` v0.6, stated so it can be
//! argued with): the cache is strictly derived from kan and git, gitignored,
//! regenerated next session, and never read as a source of truth for anything
//! but display. Delete it and nothing is lost. It stands in the same relation
//! to kan's log as kan's own disposable `.kan/index.sqlite` does — a derived
//! index that rebuilds from the durable record. If that pattern is acceptable
//! for the memory layer, it is acceptable for the process layer.

use std::io;
use std::path::{Path, PathBuf};

/// The cache directory, relative to the repo root. Gitignored.
pub const CACHE_DIR: &str = ".day";
/// The file holding the rendered status line. Its contents are **display
/// state only** — a string to print, never data to parse for a decision.
pub const STATUS_LINE_FILE: &str = "statusline";

fn status_line_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(STATUS_LINE_FILE)
}

/// Writes the rendered status line into the cache, creating `.day/` if
/// needed. Called by `day hook session-start`.
///
/// Errors are returned rather than swallowed so the caller can decide, but
/// the caller in `session_start` deliberately ignores them: a cache that
/// could not be written degrades the status line to showing nothing, which is
/// its documented empty state, not a session failure.
pub fn write_status_line(root: &Path, rendered: &str) -> io::Result<()> {
    let dir = root.join(CACHE_DIR);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(status_line_path(root), rendered)
}

/// Reads the rendered status line back, or `None` if the cache is absent or
/// unreadable. **Its absence is never an error** (REQ-9): a fresh repo, a
/// deleted cache, or a session that has not started yet all read as `None`,
/// and the status line simply shows nothing until the next session start
/// regenerates it.
///
/// The returned string is only ever printed. Nothing branches on it — that is
/// the whole point of confining the cache to this module.
pub fn read_status_line(root: &Path) -> Option<String> {
    std::fs::read_to_string(status_line_path(root)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_line_reads_back_verbatim() {
        let dir = tempfile::tempdir().unwrap();
        write_status_line(dir.path(), "day · build · next: review").unwrap();
        assert_eq!(
            read_status_line(dir.path()).as_deref(),
            Some("day · build · next: review")
        );
    }

    #[test]
    fn an_absent_cache_reads_as_none_not_an_error() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(read_status_line(dir.path()), None);
    }

    #[test]
    fn writing_creates_the_cache_dir_if_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!dir.path().join(CACHE_DIR).exists());
        write_status_line(dir.path(), "x").unwrap();
        assert!(dir.path().join(CACHE_DIR).is_dir());
    }

    /// A later session's render replaces the earlier one — the cache holds
    /// current display state, not a history.
    #[test]
    fn a_second_write_overwrites_the_first() {
        let dir = tempfile::tempdir().unwrap();
        write_status_line(dir.path(), "old").unwrap();
        write_status_line(dir.path(), "new").unwrap();
        assert_eq!(read_status_line(dir.path()).as_deref(), Some("new"));
    }
}
