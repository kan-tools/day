//! `.design/rigor-as-artifact.md` AC-1, AC-5, AC-13 — atom completion criteria
//! checked through the same probes teloi use, and the guarantee that position
//! inference never executes a command probe.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, git: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", git)
        .output()
        .expect("failed to run day")
}

fn write_git_stub(dir: &Path, tags: &[&str], tracked: &[&str]) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            "#!/bin/sh\npattern=\"$3\"\nmatch() {{ for i in $1; do case \"$i\" in $pattern) printf '%s\\n' \"$i\";; esac; done; }}\ncase \"$1\" in\n  ls-files) match \"{}\" ;;\n  tag) match \"{}\" ;;\n  *) echo unsupported >&2; exit 1 ;;\nesac\n",
            tracked.join(" "),
            tags.join(" "),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn atom_done(slug: &str, cid: &str, inputs: &[&str], outputs: &[&str], done: &[&str]) -> StubClaim {
    let list = |xs: &[&str]| {
        xs.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(",")
    };
    claim(
        &format!("atom/{slug}"),
        cid,
        &format!(
            "The {slug} atom.\n\n```day-atom\n{{\"in\":[{}],\"out\":[{}],\"next\":[],\"done\":[{}]}}\n```\n",
            list(inputs), list(outputs), list(done),
        ),
    )
}

fn witness_schema(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes.\n\n```day-witness\n{body}\n```\n"),
    )
}

/// AC-1 + AC-13: a met criterion passes and exits zero; an unmet one fails
/// and exits non-zero, so CI can gate.
#[test]
fn ac13_done_criteria_gate_the_exit_code() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_done(
                "build",
                "bafyreia",
                &["design-doc"],
                &["code-change"],
                &["tests-tracked"],
            ),
            witness_schema("bafyreiw", r#"{"tests-tracked":{"path":"tests/*.rs"}}"#),
        ],
    );

    // Criterion met: a tracked test file exists.
    let git = write_git_stub(dir.path(), &[], &["tests/x.rs"]);
    let out = day(dir.path(), &kan, &git, &["assess", "atom", "build"]);
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("[MATERIAL]"),
        "{:?}",
        out
    );
    assert_eq!(out.status.code(), Some(0));

    // Criterion unmet: no tracked test file.
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "atom", "build"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    assert_eq!(
        out.status.code(),
        Some(1),
        "an unmet criterion must gate: {stdout}"
    );
}

/// An atom with no `done` criteria reports that and does not fail.
#[test]
fn an_atom_without_criteria_says_so_and_exits_zero() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[atom_done(
            "design",
            "bafyreia",
            &["intent"],
            &["design-doc"],
            &[],
        )],
    );
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "atom", "design"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("no `done` criteria"), "{stdout}");
    assert_eq!(out.status.code(), Some(0));
}

/// AC-5, the load-bearing one: position inference runs on session start and
/// must NEVER execute a command probe. The probe would create a sentinel if
/// it ran; the session-start hook must leave that sentinel absent.
#[test]
fn ac5_session_start_never_executes_a_command_probe() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("inference-ran-a-command");
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_done(
                "build",
                "bafyreia",
                &["passing-tests"],
                &["code-change"],
                &[],
            ),
            witness_schema(
                "bafyreiw",
                &format!(
                    r#"{{"passing-tests":{{"command":"touch {}"}}}}"#,
                    sentinel.display()
                ),
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["hook", "session-start"]);
    assert!(out.status.success());
    assert!(
        !sentinel.exists(),
        "position inference executed a command probe on session start; it must never"
    );
}
