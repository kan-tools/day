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

/// The file holding how many prompts have passed since day last re-displayed a
/// standing condition. **Display state, not process state** — see
/// [`cadence_allows`].
pub const CADENCE_FILE: &str = "cadence";

/// How many user prompts pass before a *standing* condition is re-displayed.
///
/// A guess, and marked as one: day#82 exists to tune it against measured recall
/// rather than intuition, and `v0.7.0-beta.3` makes it declarable. day#30 found
/// that a general standing rule injected always becomes background — a periodic
/// re-display is specific in content but ambient in cadence, so this number is
/// the whole lever between re-orienting a drifting session and becoming that
/// noise. Erring quiet would be defensible too; 10 is chosen to produce data.
pub const DEFAULT_CADENCE: u32 = 10;

fn status_line_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(STATUS_LINE_FILE)
}

fn cadence_path(root: &Path) -> PathBuf {
    root.join(CACHE_DIR).join(CADENCE_FILE)
}

/// Whether enough prompts have passed to re-display a standing condition, and
/// records this prompt either way.
///
/// **This is the one place the carve-out is extended, and the boundary it keeps
/// is a stated test: delete `.day/` and day's answer must not change — only
/// *when* it next repeats itself.** The counter is not consulted to decide
/// anything day reports. It decides whether to say something again, which is a
/// display decision in the same sense the status line is: losing it costs a
/// repetition, never a fact. `docs/CONVENTIONS.md` and `CLAUDE.md` say the
/// carve-out is abused the moment the cache is read to *decide* rather than to
/// *display*, so the distinction is drawn here explicitly rather than left to a
/// reader's judgement.
///
/// Infallible: any IO problem returns `true`, so a broken cache makes day
/// slightly noisier rather than silent. Failing *open* is the right direction —
/// the condition being re-displayed is one that makes day's other output
/// partial, and losing it is worse than repeating it.
pub fn cadence_allows(root: &Path, cadence: u32) -> bool {
    let path = cadence_path(root);
    let seen: u32 = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    let next = seen.saturating_add(1);
    let fire = next >= cadence.max(1);
    let record = if fire { 0 } else { next };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(&path, record.to_string());
    fire
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
