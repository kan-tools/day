#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn drive(api_exit: i32) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let gh = dir.path().join("gh");
    std::fs::write(
        &gh,
        format!(
            "#!/bin/sh\ncase \"$1\" in\nrepo) echo owner/repo; exit 0;;\napi) exit {api_exit};;\nesac\nexit 2\n"
        ),
    )
    .unwrap();
    std::fs::set_permissions(&gh, std::fs::Permissions::from_mode(0o755)).unwrap();
    Command::new("scripts/foreign-contribution.sh")
        .arg("author")
        .env("PATH", dir.path())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap()
}

#[test]
fn a_real_negative_uses_the_declared_missing_exit() {
    let output = drive(1);
    assert_eq!(output.status.code(), Some(1), "{output:?}");
}

#[test]
fn infrastructure_failure_is_distinct_from_missing() {
    let output = drive(2);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}
