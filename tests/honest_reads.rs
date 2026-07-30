//! `.design/honest-reads.md` — day reports the declarations it cannot account
//! for, end to end.
//!
//! The unit tests in `src/atoms.rs`'s `version_gate` module cover the contract
//! itself across all seven block types. These cover what only a real run can:
//! that the refusal reaches an actual verb's output, that one unreadable
//! declaration costs only itself, and that a failed *read* is never reported as
//! an absent artifact.
//!
//! Both reproductions in AC-2 were run against the shipped binary before this
//! milestone and passed at exit 0, which is why they are the fixtures.

#![cfg(unix)]

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::{claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", write_git_stub(dir))
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

/// A minimal git stub: one `v*` tag and no changed files. `assess docs` reads
/// git for the release boundary, so without this the whole assessment errors
/// before producing a report — and a test asserting on an empty stdout would
/// have "passed" for the wrong reason on the negative half.
fn write_git_stub(dir: &Path) -> PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        r#"#!/bin/sh
case "$1" in
  tag) printf 'v9.9.9\n' ;;
  diff|ls-files|log) ;;
  *) exit 0 ;;
esac
"#,
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

/// A `kan` stub that **fails** on the named subjects and serves the rest, so a
/// read error can be told apart from an empty result — the distinction day#81
/// was about.
fn write_failing_kan_stub(dir: &Path, claims: &[StubClaim], fail_on: &[&str]) -> PathBuf {
    let real = write_kan_stub(dir, claims);
    let wrapper = dir.join("kan-failing.sh");
    let guards: String = fail_on
        .iter()
        .map(|s| {
            format!("  if [ \"$1\" = \"show\" ] && [ \"$2\" = \"{s}\" ]; then echo 'kan: could not decrypt log' >&2; exit 1; fi\n")
        })
        .collect();
    std::fs::write(
        &wrapper,
        format!("#!/bin/sh\n{guards}exec {} \"$@\"\n", real.display()),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
    wrapper
}

fn atom_block(slug: &str, cid: &str, body: &str) -> StubClaim {
    claim(
        &format!("atom/{slug}"),
        cid,
        &format!("The {slug} atom.\n\n```day-atom\n{body}\n```\n"),
    )
}

/// AC-2, first half: an atom declaring a field this day does not know is
/// refused, where it used to load as though the field were absent and report
/// `composition: ok` at exit 0.
///
/// `requires` stands for whatever a later day adds. The point is not that
/// `requires` is meaningful — it is that day cannot know whether it is, and so
/// must not certify a vocabulary it read only part of.
#[test]
fn ac2_an_atom_declaring_an_unknown_field_is_refused_not_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "design",
                "bafyd",
                r#"{"in":["intent"],"out":["design-doc"],"next":["build"],"requires":["approval"]}"#,
            ),
            atom_block(
                "build",
                "bafyb",
                r#"{"in":["design-doc"],"out":["code-change"],"next":[]}"#,
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let all = format!("{stdout}{stderr}");

    assert!(
        all.contains("requires"),
        "the refusal should name the field day could not account for: {all}"
    );
    assert!(
        !all.contains("composition: ok"),
        "day must not certify a vocabulary it read only part of: {all}"
    );
    assert_ne!(
        out.status.code(),
        Some(0),
        "an unreadable declaration is not a clean run: {all}"
    );

    // AC-5: the readable atom still resolves — one unreadable declaration costs
    // itself, not the vocabulary.
    assert!(
        all.contains("atom/build") || all.contains("build"),
        "the readable atom should still load: {all}"
    );
}

/// AC-2's negative control, and the half that makes the test above mean
/// something: the same vocabulary **without** the unknown field is clean.
#[test]
fn ac2_the_same_vocabulary_without_the_unknown_field_is_clean() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "design",
                "bafyd",
                r#"{"in":["intent"],"out":["design-doc"],"next":["build"]}"#,
            ),
            atom_block(
                "build",
                "bafyb",
                r#"{"in":["design-doc"],"out":["code-change"],"next":[]}"#,
            ),
        ],
    );
    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("composition: ok"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// AC-4: the two causes are distinguishable in real output, and the reader is
/// told which one is theirs to fix.
///
/// A block declaring a newer `_version` is valid JSON that this day is simply
/// too old to read — telling that reader their claim is malformed sends them to
/// fix something that is not broken, which is day#60's misdirection repeated.
#[test]
fn ac4_version_skew_and_a_broken_block_read_differently() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
            atom_block("broken", "bafyx", r#"{"in":["a"],"out":["b"],}"#),
            atom_block("fine", "bafyok", r#"{"in":["a"],"out":["b"]}"#),
        ],
    );
    let out = day(dir.path(), &kan, &["doctor"]);
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let skew = all
        .lines()
        .find(|l| l.contains("atom/future"))
        .unwrap_or_default();
    assert!(
        skew.contains("upgrade day"),
        "a too-new block should tell the reader their binary is behind: {all}"
    );
    let broken = all
        .lines()
        .find(|l| l.contains("atom/broken"))
        .unwrap_or_default();
    assert!(
        !broken.contains("upgrade day"),
        "a malformed block must not tell the reader to upgrade: {all}"
    );

    // AC-5: two unreadable declarations, and the third still loads.
    assert!(
        all.contains("atom/fine") || all.contains("fine "),
        "the readable atom should still load: {all}"
    );
    // Each unreadable subject is named exactly once — prefixing an error that
    // already names its subject is what printed `telos/bad: telos/bad: …`.
    assert_eq!(
        all.matches("atom/future").count(),
        1,
        "the subject should be named once, not twice: {all}"
    );
}

/// AC-9 (day#81): a kan read that **failed** is reported as unchecked, never as
/// the artifact being absent.
///
/// The two states used to be spelled the same way, because a failed
/// `client.show` was folded into an empty claim list. That made "day could not
/// read the release subject" indistinguishable from "no release was ever
/// recorded" — a false negative dressed as evidence, which is precisely what
/// `src/probe.rs` refuses by name for claim probes.
#[test]
fn ac9_an_unreadable_subject_is_unchecked_not_absent() {
    let docs_schema = claim(
        "schema/docs",
        "bafyds",
        "Docs schema.\n\n```day-docs\n{\"version_source\":\"Cargo.toml\",\"version_key\":\"version\"}\n```\n",
    );

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "version = \"9.9.9\"\n").unwrap();
    let claims = [
        docs_schema.clone(),
        claim("release", "bafyr", "v9.9.9 published."),
    ];
    let kan = write_failing_kan_stub(dir.path(), &claims, &["release"]);

    let out = day(dir.path(), &kan, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[UNCHECKED]") && stdout.contains("release"),
        "a failed read should report as unchecked, naming the subject: {stdout}"
    );
    assert!(
        !stdout.contains("nobody wrote down"),
        "day must not conclude a release was never recorded from a read that \
         never happened: {stdout}"
    );
    assert_eq!(
        out.status.code(),
        Some(2),
        "could-not-check outranks checked-and-found-something: {stdout}"
    );

    // Negative control: the same run with kan readable and genuinely no release
    // claim still reports the absent case as absent. Without this, the
    // assertions above would pass if day reported everything as unchecked.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "version = \"9.9.9\"\n").unwrap();
    let kan = write_kan_stub(dir.path(), &[docs_schema]);
    let out = day(dir.path(), &kan, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("[UNCHECKED]"),
        "a readable kan with no release claim is not an unchecked state: {stdout}"
    );
    assert_ne!(
        out.status.code(),
        Some(2),
        "a check that ran must not exit as one that could not: {stdout}"
    );
}
