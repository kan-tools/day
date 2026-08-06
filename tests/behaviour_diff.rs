//! `scripts/behaviour-diff.py` — **the harness that answers "did this change
//! alter behaviour it was not meant to alter".**
//!
//! `mutate.py` and `revert-demo.py` each have tests here for the same reason
//! this does: a verification tool that fails toward false confidence is worse
//! than no tool, and this one demonstrated that on its very first run. It
//! reported `IDENTICAL` — clean — while checking nothing, because its kan stub
//! answered `status --json` with prose, day refused it on *both* binaries, and
//! two identical errors compare equal.
//!
//! So the assertions here are about the guard, not about the diff. The diff is
//! easy and visible; the guard is what stops a green run over nothing, and it is
//! the thing a future edit to the stub will break silently.

#![cfg(unix)]

mod common;

use common::repo_root;
use std::path::Path;
use std::process::Command;

/// A corpus directory containing one fixture, written from the parts a real one
/// has. `log` is the `show --all --json` envelope; passing something day cannot
/// read is how the unrunnable case is reached.
fn corpus_with(dir: &Path, log: &str) -> std::path::PathBuf {
    let corpus = dir.join("corpus");
    let fixture = corpus.join("only");
    std::fs::create_dir_all(&fixture).unwrap();
    std::fs::write(fixture.join("log.json"), log).unwrap();
    std::fs::write(
        fixture.join("status.json"),
        r#"{"v":1,"subjects":[{"subject":"telos/t","subjects":["telos/t"],"state":"Unclassified"}]}"#,
    )
    .unwrap();
    std::fs::write(
        fixture.join("case.json"),
        r#"{"why":"fixture","tags":[],"tracked":[],"invocations":["assess telos t"]}"#,
    )
    .unwrap();
    corpus
}

fn diff(args: &[&str]) -> (String, Option<i32>) {
    let out = Command::new("python3")
        .arg(repo_root().join("scripts/behaviour-diff.py"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("behaviour-diff.py should be runnable");
    (
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr),
        out.status.code(),
    )
}

/// **A fixture that cannot produce a verdict is not a fixture that agreed.**
///
/// This is the defect the harness shipped with for one run. Both binaries error
/// identically, the outputs compare equal, and a content diff calls that
/// `IDENTICAL` — could-not-check reported as checked-and-clean, in the tool
/// built to catch could-not-check reported as checked-and-clean.
///
/// The fixture here hands day a `show --all` envelope of the wrong shape, which
/// is exactly how it happened: the corpus is the part that rots, and it rots
/// toward silence.
#[test]
fn a_fixture_that_cannot_run_is_not_reported_as_identical() {
    let dir = tempfile::tempdir().unwrap();
    let corpus = corpus_with(dir.path(), r#"{"v":1,"subjects":"not-an-envelope"}"#);

    let (text, code) = diff(&[
        "--since",
        "HEAD",
        "--corpus",
        corpus.to_str().unwrap(),
        "--expect-fixtures",
        "1",
    ]);

    assert!(
        text.contains("CORPUS-EMPTY"),
        "an unrunnable fixture must be reported, not diffed: {text}"
    );
    // Line-exact, not `contains`. The explanation itself says the word
    // ("would otherwise have reported IDENTICAL while checking nothing"), so a
    // substring test passes on the message that proves the guard WORKED — which
    // is CLAUDE.md's rule about keying a classifier on a phrase, caught by
    // writing the assertion badly first.
    assert!(
        !text.lines().any(|l| l.trim() == "IDENTICAL"),
        "and it must never render the agreement verdict: {text}"
    );
    assert_eq!(code, Some(2), "could-not-check exits 2, not 0: {text}");
}

/// **A corpus that shrank is a could-not-check, not a clean run.**
///
/// `capture-block-corpus.sh` silently omitted three of seven block types,
/// twice, and the coverage was quietly smaller with no error. A derived list
/// fixes the missing-member case; the exact count is what catches a reader that
/// broke. Neither substitutes for the other, so both are asserted.
#[test]
fn a_corpus_whose_size_changed_is_refused_before_anything_is_compared() {
    let dir = tempfile::tempdir().unwrap();
    let corpus = corpus_with(dir.path(), r#"{"v":1,"subjects":[]}"#);

    let (text, code) = diff(&[
        "--since",
        "HEAD",
        "--corpus",
        corpus.to_str().unwrap(),
        // One fixture exists; claiming two must fail rather than diff one.
        "--expect-fixtures",
        "2",
    ]);

    assert!(text.contains("CORPUS-EMPTY"), "{text}");
    assert!(
        text.contains("expected 2") && text.contains("found 1"),
        "the message must name both numbers, or a reader cannot tell which way \
         it drifted: {text}"
    );
    assert_eq!(code, Some(2), "{text}");
}

/// The corpus this repo actually ships is the size the harness is invoked with.
///
/// Not a duplicate of the flag: this asserts the **committed** corpus has not
/// silently lost a fixture, which is the state a passing `--expect-fixtures 2`
/// elsewhere would hide by being wrong in the same direction.
#[test]
fn the_shipped_corpus_has_the_fixtures_it_claims() {
    let corpus = repo_root().join("fixtures/behaviour");
    let mut found: Vec<String> = std::fs::read_dir(&corpus)
        .expect("fixtures/behaviour should ship")
        .flatten()
        .filter(|e| e.path().join("case.json").is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    found.sort();

    assert_eq!(
        found,
        vec![
            "glob-named-subject".to_string(),
            "scoped-universal".to_string()
        ],
        "the corpus encodes the two regressions two cold reviews found by hand; \
         losing one loses the evidence that this harness catches them"
    );
}
