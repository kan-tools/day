#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn legacy(args: &[&str]) -> Output {
    Command::new(root().join("scripts/behaviour-diff.py"))
        .args(args)
        .current_dir(root())
        .output()
        .expect("compatibility shim should run during its compatibility window")
}

fn native(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["evidence", "behaviour-diff"])
        .args(args)
        .current_dir(root())
        .output()
        .expect("native behaviour diff should run")
}

#[test]
fn compatibility_shim_preserves_the_native_report_and_exit_status() {
    let args = ["--since", "HEAD", "--expect-fixtures", "4"];
    let old = legacy(&args);
    let new = native(&args);
    assert_eq!(new.status.code(), old.status.code());
    assert_eq!(new.stdout, old.stdout);
    assert!(new.stderr.is_empty(), "native stderr must stay policy-free");
    assert!(String::from_utf8_lossy(&old.stderr).contains("deprecated:"));
}

#[test]
fn compatibility_shim_contains_only_portable_delegation() {
    let source = std::fs::read_to_string(root().join("scripts/behaviour-diff.py")).unwrap();
    assert!(source.starts_with("#!/bin/sh\n"));
    assert_eq!(source.matches("set -eu").count(), 1);
    assert!(!source.contains("python"));
    assert!(!source.contains("if "));
    assert_eq!(source.matches("exec cargo run").count(), 1);
    assert!(source.contains("evidence behaviour-diff \"$@\""));
}

#[test]
fn native_invocation_accounting_does_not_depend_on_the_legacy_path() {
    let fixture = tempfile::tempdir().unwrap();
    let counter = fixture.path().join("counter");
    let output = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["evidence", "behaviour-diff"])
        .env("DAY_BEHAVIOUR_DIFF_COUNTER", &counter)
        .current_dir(root())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "missing required count");
    assert_eq!(std::fs::read_to_string(counter).unwrap(), "1\n");
}
