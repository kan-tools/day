#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn run(program: impl AsRef<std::ffi::OsStr>, args: &[&str]) -> Output {
    Command::new(program)
        .args(args)
        .current_dir(root())
        .output()
        .unwrap()
}

fn assert_parity(script: &str, native_args: &[&str], legacy_args: &[&str]) {
    let old = run(root().join(script), legacy_args);
    let new = run(env!("CARGO_BIN_EXE_xtask"), native_args);
    assert_eq!(new.status.code(), old.status.code());
    assert_eq!(new.stdout, old.stdout);
    assert!(new.stderr.is_empty());
    assert!(String::from_utf8_lossy(&old.stderr).contains("deprecated:"));
}

#[test]
fn formal_obligation_shim_matches_the_native_self_test() {
    assert_parity(
        "scripts/check-rfc1-formal-obligations.py",
        &["validate", "formal", "--self-test"],
        &["--self-test"],
    );
}

#[test]
fn vector_shim_matches_the_native_self_test() {
    assert_parity(
        "scripts/check-rfc1-vectors.py",
        &["validate", "vectors", "--self-test"],
        &["--self-test"],
    );
}

#[test]
fn publication_shims_match_the_native_self_tests() {
    for (script, rfc) in [
        ("scripts/check-rfc0-publication.py", "0"),
        ("scripts/check-rfc1-denotational-publication.py", "1"),
    ] {
        assert_parity(
            script,
            &["validate", "publication", "--rfc", rfc, "--self-test"],
            &["--self-test"],
        );
    }
}

#[test]
fn leaf_shims_are_policy_free_and_python_free() {
    for script in [
        "scripts/check-rfc1-formal-obligations.py",
        "scripts/check-rfc1-vectors.py",
        "scripts/check-rfc0-publication.py",
        "scripts/check-rfc1-denotational-publication.py",
        "scripts/check-rfc-review.py",
    ] {
        let source = std::fs::read_to_string(root().join(script)).unwrap();
        assert!(source.starts_with("#!/bin/sh\nset -eu\n"));
        assert!(!source.contains("python"));
        assert!(!source.contains("if "));
        assert_eq!(source.matches("exec cargo run").count(), 1);
    }
}
