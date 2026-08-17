#![cfg(unix)]

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn executable(path: &Path, body: &str) {
    std::fs::write(path, format!("#!/bin/sh\n{body}")).unwrap();
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().into()
}

#[test]
fn v013_cutter_tags_the_prequalified_candidate_without_a_source_commit() {
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    executable(&bin.join("cargo"), "exit 0\n");
    executable(
        &bin.join("gh"),
        "case \"$1 $*\" in\n\
         'issue '*'--json state'*) echo CLOSED ;;\n\
         'issue '*'closedByPullRequestsReferences'*) echo 999 ;;\n\
         'pr '*) echo 2026-08-17T00:00:00Z ;;\n\
         *) exit 97 ;;\n\
         esac\n",
    );
    executable(
        &bin.join("kan"),
        "printf '%s\\n' \"$*\" > \"$KAN_LOG\"\n\
         echo bafyreistubbedrelease\n",
    );

    git(root.path(), &["init", "-q", "-b", "main"]);
    git(root.path(), &["config", "user.name", "candidate test"]);
    git(
        root.path(),
        &["config", "user.email", "candidate@example.invalid"],
    );
    std::fs::create_dir_all(root.path().join("tests/fixtures/block-corpus")).unwrap();
    std::fs::write(
        root.path()
            .join("tests/fixtures/migration-expectations.tsv"),
        "v0.13.0-beta.1\trefused-honestly\n",
    )
    .unwrap();
    std::fs::write(
        root.path()
            .join("tests/fixtures/block-corpus/v0.13.0-beta.1.jsonl"),
        "{}\n",
    )
    .unwrap();
    git(root.path(), &["add", "-A"]);
    git(root.path(), &["commit", "-qm", "qualified candidate"]);
    let candidate = git(root.path(), &["rev-parse", "HEAD"]);
    let kan_log = root.path().with_extension("kan-log");

    let mut child = Command::new(repository_root().join("scripts/cut-v013-candidate.sh"))
        .arg("v0.13.0-beta.1")
        .current_dir(root.path())
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .env("KAN_LOG", &kan_log)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"verified omnibus candidate\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        git(root.path(), &["rev-list", "-n", "1", "v0.13.0-beta.1"]),
        candidate
    );
    assert_eq!(git(root.path(), &["rev-parse", "HEAD"]), candidate);
    assert!(git(root.path(), &["status", "--porcelain"]).is_empty());
    let claim = std::fs::read_to_string(kan_log).unwrap();
    assert!(claim.contains("v0.13.0-beta.1") && claim.contains(&candidate));
}
