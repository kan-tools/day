//! `.design/assess-telos.md` AC-1..AC-12 — telos assessment through the
//! shipped binary.
//!
//! The load-bearing ones here are AC-4 and AC-5: a command probe is a program
//! named by a kan claim, so "it did not run" and "no shell was involved" have
//! to be demonstrated by a probe that *would* leave a trace and then does
//! not, rather than asserted from day's own output.

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

/// A stub `git` answering the two reads a probe uses.
fn write_git_stub(dir: &Path, tags: &[&str], tracked: &[&str]) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
case "$1" in
  tag) printf '%s' "{tags}" ;;
  ls-files) printf '%s' "{tracked}" ;;
  *) echo "git stub: unsupported read $1" >&2; exit 1 ;;
esac
"#,
            tags = tags.join("\n"),
            tracked = tracked.join("\n"),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn telos_claim(slug: &str, cid: &str, witnesses: &[&str]) -> StubClaim {
    let list = witnesses
        .iter()
        .map(|w| format!("\"{w}\""))
        .collect::<Vec<_>>()
        .join(",");
    claim(
        &format!("telos/{slug}"),
        cid,
        &format!("A telos.\n\n```day-telos\n{{\"witnesses\":[{list}]}}\n```\n"),
    )
}

fn witness_schema(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes.\n\n```day-witness\n{body}\n```\n"),
    )
}

#[test]
fn ac1_each_declared_witness_is_named_with_a_status() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact", "design-doc"]),
            witness_schema(
                "bafyreiw",
                r#"{"published-artifact":{"tag":"v*"},"design-doc":{"path":".design/*.md"}}"#,
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("published-artifact"), "{stdout}");
    assert!(stdout.contains("design-doc"), "{stdout}");
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(stdout.contains("v1.0.0"), "{stdout}");
}

#[test]
fn ac3_a_probe_matching_nothing_is_unsatisfied_and_fails_the_run() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    // No tags at all.
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    // AC-9: an unsatisfied material probe exits non-zero.
    assert_eq!(out.status.code(), Some(1), "{stdout}");
}

/// AC-4 and AC-5, the ones that matter. The probe would create a sentinel
/// file if it ran, and would create a *different* one if a shell interpreted
/// it. Asserting day's output says "NOT RUN" proves nothing on its own; the
/// filesystem does.
#[test]
fn ac4_a_command_probe_is_not_executed_without_run() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("probe-ran");
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["passing-tests"]),
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

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[NOT RUN]"), "{stdout}");
    assert!(stdout.contains("--run"), "{stdout}");
    assert!(
        !sentinel.exists(),
        "a command probe executed without --run being passed"
    );
    // Not-run is absence of evidence, not failure.
    assert_eq!(out.status.code(), Some(0), "{stdout}");

    // And with --run, the same probe does execute.
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(sentinel.exists(), "--run should have executed the probe");
}

/// AC-5 end to end: metacharacters arriving from a claim stay literal.
#[test]
fn ac5_metacharacters_in_a_declared_probe_never_reach_a_shell() {
    let dir = tempfile::tempdir().unwrap();
    let pwned = dir.path().join("pwned");
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["passing-tests"]),
            witness_schema(
                "bafyreiw",
                &format!(
                    r#"{{"passing-tests":{{"command":"true; touch {}"}}}}"#,
                    pwned.display()
                ),
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run"],
    );
    assert!(
        !pwned.exists(),
        "a `;` in a claim-declared probe reached a shell: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn ac7_a_probe_exceeding_the_timeout_is_killed_and_reported() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["slow"]),
            witness_schema("bafyreiw", r#"{"slow":{"command":"sleep 30"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let started = std::time::Instant::now();
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run", "--timeout", "1"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[TIMEOUT]"), "{stdout}");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "the assessment should return at the timeout rather than waiting the probe out"
    );
    // Unknown evidence is not absent evidence.
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

#[test]
fn ac2_with_no_witness_schema_day_explains_and_offers_a_starter() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[telos_claim("shipped", "bafyreit", &["published-artifact"])],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(combined.contains("schema/witness"), "{combined}");
    assert!(combined.contains("day-witness"), "{combined}");
    assert!(combined.contains("kan observe"), "{combined}");
}

#[test]
fn ac11_a_telos_without_witnesses_is_reported_as_not_checkable() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[claim("telos/vague", "bafyreit", "A telos.")]);
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "vague"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("declares no witnesses"), "{stdout}");
    assert!(stdout.contains("--witness"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "unassessable is not failing");
}

#[test]
fn ac11_a_witness_with_no_declared_probe_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["adoption"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[NO PROBE]"), "{stdout}");
    assert!(stdout.contains("adoption"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "nothing checked is not failing");
}

/// AC-12: the assessment reads. It appends nothing, and the command it prints
/// for the reader uses `kan result`'s real argument order — subject first,
/// positionally. `tests/kan_conformance.rs` proves that form is the one a
/// real kan accepts.
#[test]
fn ac12_assessing_writes_nothing_and_prints_a_runnable_record_command() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        common::appends(dir.path()).is_empty(),
        "assessing must append nothing to the log"
    );
    assert!(
        stdout.contains("kan result telos/shipped"),
        "the record command must put the subject first, positionally: {stdout}"
    );
    assert!(
        !stdout.contains("result --subject"),
        "the record command must not use --subject, which kan result rejects: {stdout}"
    );
    assert!(stdout.contains("--cites"), "{stdout}");
}

#[test]
fn ac10_assess_telos_is_a_subcommand_of_assess() {
    let dir = tempfile::tempdir().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["assess", "--help"])
        .current_dir(dir.path())
        .output()
        .expect("failed to run day assess --help");
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("telos"), "{help}");
}

/// AC-2's second half, which the missing-schema test above does not cover:
/// the probe map is *data*, so changing the claim changes what is checked
/// with no code and no config file edited — the property `schema/design-doc`
/// and `schema/docs` already have.
#[test]
fn ac2_changing_the_witness_claim_changes_what_is_checked() {
    let dir = tempfile::tempdir().unwrap();
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    // Probing tags: satisfied, because a tag exists.
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert_eq!(out.status.code(), Some(0));

    // Same telos, same binary, same working tree — only the claim changed.
    // Now it probes tracked files, and the stub tracks none.
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw2", r#"{"published-artifact":{"path":"dist/*"}}"#),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    assert!(stdout.contains("dist/*"), "{stdout}");
    assert_eq!(out.status.code(), Some(1));
}

/// Found by the adversarial review: a telos that cannot be assessed exited 0,
/// so a typo'd slug read as a clean assessment. "Could not check" must not be
/// spelled the same way as "checked and found nothing wrong".
#[test]
fn an_unassessable_telos_does_not_exit_zero() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[claim("telos/real", "bafyreit", "A telos.")]);
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "does-not-exist"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("no telos"), "{stdout}");
    assert_eq!(
        out.status.code(),
        Some(2),
        "a check that could not run must not be indistinguishable from a clean one: {stdout}"
    );

    // An --all sweep still reports every telos it *can* assess.
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "--all"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("telos/real"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}
