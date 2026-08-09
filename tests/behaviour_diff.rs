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

/// A corpus directory containing one fixture with a caller-chosen `case.json`.
/// Separate from [`corpus_with`] because day#145's case is a *well-formed* log
/// whose case declares nothing — the log is not what makes it empty.
fn corpus_with_case(dir: &Path, case: &str) -> std::path::PathBuf {
    let corpus = corpus_with(dir, r#"{"v":1,"subjects":[]}"#);
    std::fs::write(corpus.join("only").join("case.json"), case).unwrap();
    corpus
}

fn diff(args: &[&str]) -> (String, Option<i32>) {
    diff_with_path(args, None)
}

/// `path_prefix` is prepended to `PATH`, which is how a test makes `cargo`
/// fail without touching this repo's source. Editing `src/` from a test would
/// race the other tests in this file — they all shell out to a script that now
/// builds — and a panic would leave the tree broken for everything after.
fn diff_with_path(args: &[&str], path_prefix: Option<&Path>) -> (String, Option<i32>) {
    let mut cmd = Command::new("python3");
    cmd.arg(repo_root().join("scripts/behaviour-diff.py"))
        .args(args)
        .current_dir(repo_root());
    if let Some(prefix) = path_prefix {
        let existing = std::env::var("PATH").unwrap_or_default();
        cmd.env("PATH", format!("{}:{existing}", prefix.display()));
    }
    let out = cmd.output().expect("behaviour-diff.py should be runnable");
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

/// **day#145 — a fixture that invokes nothing counts as a fixture.**
///
/// `observe()` iterated `case["invocations"]`, so an empty list produced `{}`,
/// the comparison loop iterated nothing, and the fixture contributed no
/// evidence — while still satisfying `--expect-fixtures`, the guard meant to
/// catch a corpus that stopped covering things. A corpus of N such fixtures
/// reported a clean run having executed day zero times.
///
/// Neither existing guard could see it: the count counts *directories*, and the
/// derived-list test checks membership by *name*. The corpus list is
/// exhaustive; the corpus contents were not.
#[test]
fn a_fixture_that_invokes_nothing_is_not_reported_as_identical() {
    let dir = tempfile::tempdir().unwrap();
    let corpus = corpus_with_case(
        dir.path(),
        r#"{"why":"no invocations at all","tags":[],"tracked":[],"invocations":[]}"#,
    );

    let (text, code) = diff(&[
        "--since",
        "HEAD",
        "--corpus",
        corpus.to_str().unwrap(),
        "--expect-fixtures",
        "1",
    ]);

    assert!(
        text.contains("declares no invocations"),
        "the empty fixture must be named: {text}"
    );
    assert!(
        !text.lines().any(|l| l.trim() == "IDENTICAL"),
        "and it must never render the agreement verdict: {text}"
    );
    assert_eq!(code, Some(2), "could-not-check exits 2, not 0: {text}");
}

/// **day#144 — the guard caught one shape of unrunnable and not day's.**
///
/// `assess telos` reports an unanswerable witness as `[ERROR]` on **stdout**
/// with exit **0**, so neither `returncode not in (0, 1)` nor
/// `"could not read" in stderr` fired. Two binaries that both declined to
/// answer compared equal, and the harness said `IDENTICAL`.
///
/// The fixture reaches it the way the issue did: an `also_carries` entry
/// declaring its own `subject`, which `every` refuses by design.
#[test]
fn a_fixture_whose_verdict_is_an_error_is_not_reported_as_identical() {
    let dir = tempfile::tempdir().unwrap();
    // A witness day parses and then refuses on the merits — the log is
    // well-formed, which is what makes exit 0 and a stdout `[ERROR]`.
    let log = r#"{"v":1,"subjects":[
        {"subject":"schema/witness","claims":[{"subject":"schema/witness","cid":"bafys","kind":"Observation","author":"did:key:zFixtureAuthor","recorded_at":10,
          "text":"```day-witness\n{\"u\": {\"every\": {\"subject_with\": {\"kind\":\"Plan\",\"subject\":\"design/*\"}, \"also_carries\": [{\"kind\":\"Decision\",\"subject\":\"other/b\"}]}}}\n```"}]},
        {"subject":"telos/t","claims":[{"subject":"telos/t","cid":"bafyt","kind":"Decision","text":"T.\n\n```day-telos\n{\"witnesses\":[\"u\"]}\n```","author":"did:key:zFixtureAuthor","recorded_at":11}]}
    ]}"#;
    let corpus = corpus_with(dir.path(), log);

    let (text, code) = diff(&[
        "--since",
        "HEAD",
        "--corpus",
        corpus.to_str().unwrap(),
        "--expect-fixtures",
        "1",
    ]);

    // The premise is the whole point: day must have exited 0 here. If it ever
    // starts exiting non-zero, the pre-existing guard catches it and this test
    // stops measuring what it was written for.
    assert!(
        text.contains("exited 0"),
        "premise broken — this fixture is supposed to reach the exit-0 `[ERROR]` \
         shape the old guard could not see: {text}"
    );
    assert!(
        !text.lines().any(|l| l.trim() == "IDENTICAL"),
        "an errored verdict must never compare as agreement: {text}"
    );
    assert_eq!(code, Some(2), "could-not-check exits 2, not 0: {text}");
}

/// **A relative `--corpus` made every fixture unrunnable.**
///
/// `observe()` puts the fixture path in `FIXTURE` and runs day with
/// `cwd=work`, so a relative path made the stub's `cat "$FIXTURE/status.json"`
/// miss, day got empty stdout, and the run reported `CORPUS-EMPTY` against a
/// corpus that was fine. The default is `ROOT`-based and absolute, so the
/// documented invocation always worked and the flag never did — a mechanism
/// with two modes, exercised only in the mode this repo happens to use.
///
/// Fails toward could-not-check rather than toward clean, which is why it
/// survived: it looked like a broken corpus, not a broken harness.
#[test]
fn a_relative_corpus_path_resolves_against_the_repo_not_the_work_dir() {
    let (text, code) = diff(&[
        "--since",
        "HEAD",
        "--corpus",
        "fixtures/behaviour",
        "--expect-fixtures",
        "2",
    ]);

    assert!(
        !text.contains("CORPUS-EMPTY"),
        "the shipped corpus is runnable; a relative path must not make it \
         look otherwise: {text}"
    );
    assert_eq!(code, Some(0), "a clean comparison exits 0: {text}");
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

/// **A stale `target/debug/day` made the harness compare the base against
/// itself.**
///
/// The head binary was built only `if not head.exists()`. An existing one was
/// reused, so the question "did the WORKING TREE change behaviour" was answered
/// with a binary built from whatever was there last — and two builds of the
/// same revision compare equal, so the verdict was `IDENTICAL`. Clean, for the
/// reason this harness exists to reject, and reachable by the most ordinary
/// state there is: edit source, do not build, ask the question.
///
/// Asserted through a `cargo` that cannot succeed, which is the one input that
/// separates "rebuilt" from "reused" without measuring the machine. Under the
/// old code `target/debug/day` exists during any test run, the build was
/// skipped, a broken `cargo` was never consulted, and `--since HEAD` diffed the
/// revision against itself: `IDENTICAL`. Under the new code the build is
/// attempted first and its failure is the verdict. Two different outcomes from
/// the same fixture, so the test observes the change rather than the feature
/// around it.
#[test]
fn the_head_binary_is_rebuilt_rather_than_reused() {
    let dir = tempfile::tempdir().unwrap();
    let corpus = corpus_with(dir.path(), r#"{"v":1,"subjects":[]}"#);

    let bin = dir.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let cargo = bin.join("cargo");
    std::fs::write(
        &cargo,
        "#!/bin/sh\necho 'stub cargo: refusing' >&2\nexit 1\n",
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&cargo, std::fs::Permissions::from_mode(0o755)).unwrap();

    let (text, code) = diff_with_path(
        &[
            "--since",
            "HEAD",
            "--corpus",
            corpus.to_str().unwrap(),
            "--expect-fixtures",
            "1",
        ],
        Some(&bin),
    );

    assert!(
        text.contains("HEAD-DID-NOT-BUILD"),
        "a head that cannot be built must be refused, and named as the HEAD end \
         so a reader looks at the right revision: {text}"
    );
    assert!(
        !text.lines().any(|l| l.trim() == "IDENTICAL"),
        "reusing whatever was in target/debug is how this reported clean: {text}"
    );
    assert_eq!(code, Some(2), "could-not-check exits 2: {text}");
}

/// **`--expect-fixtures` was documented as required and defaulted to `None`.**
///
/// So the count guard — the half that catches a corpus reader which stopped
/// matching, as against the derived list which catches a missing member — ran
/// only when someone remembered the flag. Every invocation in this file passes
/// it, which is precisely why nothing here noticed.
#[test]
fn the_fixture_count_cannot_be_omitted() {
    let dir = tempfile::tempdir().unwrap();
    let corpus = corpus_with(dir.path(), r#"{"v":1,"subjects":[]}"#);

    let (text, code) = diff(&["--since", "HEAD", "--corpus", corpus.to_str().unwrap()]);

    assert!(
        text.contains("expect-fixtures"),
        "the refusal must name the missing flag: {text}"
    );
    assert!(
        !text.lines().any(|l| l.trim() == "IDENTICAL"),
        "and must never reach a verdict: {text}"
    );
    assert_eq!(
        code,
        Some(2),
        "argparse's own usage exit, which is a could-not-check: {text}"
    );
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
