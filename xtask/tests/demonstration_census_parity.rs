use std::path::PathBuf;
use std::process::{Command, Output};

use xtask::capability::process::SystemProcess;
use xtask::command::{CensusCommand, TrailingArgs, Xtask};
use xtask::outcome::Outcome;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn legacy(range: &str) -> Output {
    Command::new(root().join("scripts/demonstration-census.py"))
        .arg(range)
        .current_dir(root())
        .output()
        .expect("compatibility shim should run during its compatibility window")
}

fn native(range: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["census", "demonstrations", range])
        .current_dir(root())
        .output()
        .expect("native census should run")
}

#[test]
fn native_census_matches_the_legacy_branch_report() {
    let old = legacy("main..HEAD");
    let new = native("main..HEAD");
    assert_eq!(new.status.code(), old.status.code());
    assert_eq!(new.stdout, old.stdout);
    assert!(new.stderr.is_empty());
    assert!(String::from_utf8_lossy(&old.stderr).contains("deprecated:"));
}

#[test]
fn native_census_preserves_the_distinct_empty_range_outcome() {
    let old = legacy("HEAD..HEAD");
    let new = native("HEAD..HEAD");
    assert_eq!(old.status.code(), Some(3), "legacy premise");
    assert_eq!(new.status.code(), old.status.code());
    assert_eq!(new.stdout, old.stdout);
    assert!(new.stderr.is_empty());
    assert!(String::from_utf8_lossy(&old.stderr).contains("deprecated:"));
}

#[test]
fn native_census_finds_a_real_unaccounted_commit() {
    let fixture = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(fixture.path())
            .env("GIT_AUTHOR_NAME", "fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
            .env("GIT_COMMITTER_NAME", "fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "-q", "-b", "main"]);
    git(&[
        "commit",
        "--allow-empty",
        "--no-gpg-sign",
        "-m",
        "base",
        "-m",
        "No trailer: fixture baseline.",
    ]);
    git(&[
        "commit",
        "--allow-empty",
        "--no-gpg-sign",
        "-m",
        "unaccounted change",
    ]);

    let outcome = xtask::run(
        Xtask::Census {
            command: CensusCommand::Demonstrations(TrailingArgs {
                args: vec!["HEAD~1..HEAD".into()],
            }),
        },
        fixture.path(),
        &SystemProcess,
    );
    assert!(matches!(outcome, Outcome::Finding(_)));
}
