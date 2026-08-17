//! day#152 — handoff measurements retain the coordinates at which they were
//! made. The structured reader never consults moving HEAD, and legacy prose is
//! explicitly uncheckable rather than silently rebound to current state.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{claim, write_kan_stub};

const BASE: &str = "1111111111111111111111111111111111111111";
const MEASURED: &str = "2222222222222222222222222222222222222222";

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {}: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn day(dir: &Path, kan: &Path, thread: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["stream", "scopes", thread])
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .unwrap()
}

#[test]
fn scoped_coordinates_survive_an_advance_and_merge_while_legacy_stays_uncheckable() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "base"]);
    git(
        dir.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(dir.path(), &["config", "user.name", "test"]);
    git(dir.path(), &["commit", "-qm", "root", "--allow-empty"]);

    let text = format!(
        "Scoped handoff.\n\n```day-handoff-scopes\n{{\"_version\":1,\"suites\":[{{\"argv\":[\"cargo\",\"test\",\"--workspace\"],\"commit\":\"{MEASURED}\",\"tree_clean\":true}}],\"censuses\":[{{\"base\":\"{BASE}\",\"head\":\"{MEASURED}\",\"unaccounted\":0}}],\"ci\":[{{\"provider\":\"github-actions\",\"workflow\":\"CI\",\"run_id\":42,\"head_sha\":\"{MEASURED}\",\"conclusion\":\"success\"}}]}}\n```\n"
    );
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim("agents/handoff/main", "cid-scoped", &text),
            claim(
                "agents/handoff/legacy",
                "cid-legacy",
                "Suite passes and CI is green.",
            ),
        ],
    );

    let before = day(dir.path(), &kan, "main");
    assert!(before.status.success());

    git(dir.path(), &["switch", "-qc", "feature"]);
    git(dir.path(), &["commit", "-qm", "feature", "--allow-empty"]);
    git(dir.path(), &["switch", "-q", "base"]);
    git(dir.path(), &["commit", "-qm", "advance", "--allow-empty"]);
    git(dir.path(), &["merge", "-qm", "merge feature", "feature"]);

    let after = day(dir.path(), &kan, "main");
    assert!(after.status.success());
    assert_eq!(
        before.stdout, after.stdout,
        "moving HEAD retargeted the scope"
    );
    let scoped = String::from_utf8(after.stdout).unwrap();
    assert!(scoped.contains(&format!("commit={MEASURED}")));
    assert!(scoped.contains(&format!("base={BASE}; head={MEASURED}")));
    assert!(scoped.contains("run_id=42"));
    let current_head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let current_head = String::from_utf8(current_head.stdout).unwrap();
    assert!(!scoped.contains(current_head.trim()));

    let legacy = day(dir.path(), &kan, "legacy");
    assert!(legacy.status.success());
    let legacy = String::from_utf8(legacy.stdout).unwrap();
    assert!(legacy.contains("UNCHECKABLE"));
    assert!(legacy.contains("legacy unscoped"));
    assert!(legacy.contains("do not substitute current HEAD"));
}
