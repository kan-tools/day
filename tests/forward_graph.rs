//! `.design/forward-only-next.md` AC-5, AC-6, AC-7, AC-12, AC-13 — day#113 at
//! the surfaces a person reads.
//!
//! **Every fixture here is cyclic, and that is the point.** The defect survived
//! because every existing off-sequence test used an acyclic vocabulary, so the
//! check was only ever exercised in the mode where it was right. day's own
//! vocabulary was in the broken mode and the tests were not — CLAUDE.md's "a
//! mechanism with two modes gets tested in whichever mode this repo is in",
//! with the repo on the wrong side of it.
//!
//! Each test that asserts a cycle is handled therefore also asserts the fixture
//! *produced* one, rather than trusting that it did. A fixture that quietly
//! stopped being cyclic would otherwise make every assertion below pass.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{appends, claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, git: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", git)
        .stdin(std::process::Stdio::null())
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

fn atom(
    slug: &str,
    cid: &str,
    inputs: &[&str],
    outputs: &[&str],
    next: &[&str],
    revisits: &[&str],
) -> StubClaim {
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
            "The {slug} atom.\n\n```day-atom\n{{\"in\":[{}],\"out\":[{}],\"next\":[{}],\"revisits\":[{}]}}\n```\n",
            list(inputs),
            list(outputs),
            list(next),
            list(revisits),
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

/// day's own pre-day#113 vocabulary, reduced to the part that matters: build
/// and review each list the other in `next`.
fn cyclic_claims() -> Vec<StubClaim> {
    vec![
        atom(
            "design",
            "bafyreid",
            &["intent"],
            &["design-doc"],
            &["build"],
            &[],
        ),
        atom(
            "build",
            "bafyreib",
            &["design-doc"],
            &["code-change"],
            &["review"],
            &[],
        ),
        atom(
            "review",
            "bafyreir",
            &["code-change"],
            &["verdict"],
            &["build"],
            &[],
        ),
        witness_schema(
            "bafyreiw",
            r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"},"verdict":{"claim":{"kind":"Decision"}}}"#,
        ),
    ]
}

/// AC-5: a cycle is a **finding**, not a failure. `doctor` names it, says which
/// field the feedback edge belongs in, still reports `composition: ok`, and
/// exits zero.
///
/// The exit code is the assertion that matters. Every project whose vocabulary
/// predates `revisits` — day's own included, and mingus's — would otherwise
/// start failing `day doctor` on upgrade, before its author had touched
/// anything. day is advisory; an existing project gets told, not broken.
#[test]
fn ac5_a_cycle_is_reported_without_failing_the_composition_check() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &cyclic_claims());
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("cycle through"),
        "the cycle must be named: {stdout}"
    );
    assert!(stdout.contains("build"), "{stdout}");
    assert!(stdout.contains("review"), "{stdout}");
    assert!(
        stdout.contains("revisits"),
        "and the finding must say where the edge belongs: {stdout}"
    );
    assert!(
        stdout.contains("composition: ok"),
        "a cycle is not a composition failure: {stdout}"
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "an existing cyclic vocabulary must not start failing on upgrade: {stdout}"
    );
}

/// AC-6: the day#113 false positive is gone, and what replaced it is a
/// could-not-check rather than silence.
///
/// The premise is asserted first: `doctor` on this fixture must report a cycle.
/// Without that, a fixture that stopped being cyclic would satisfy "no skipped
/// step is reported" trivially, which is the shape of test that let this defect
/// through in the first place.
#[test]
fn ac6_a_cyclic_pair_reports_could_not_check_and_no_skipped_step() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &cyclic_claims());
    // code-change present (src/x.rs tracked), verdict absent (no Decision
    // claim), design-doc present. Pre-day#113 this reported
    // "build produced its output but upstream review did not".
    let git = write_git_stub(dir.path(), &[], &["src/x.rs", ".design/d.md"]);

    let doctor =
        String::from_utf8_lossy(&day(dir.path(), &kan, &git, &["doctor"]).stdout).into_owned();
    assert!(
        doctor.contains("cycle through"),
        "premise: this fixture must actually be cyclic, or the assertions below \
         are about nothing: {doctor}"
    );

    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        !stdout.contains("upstream review did not"),
        "the day#113 false positive is back: {stdout}"
    );
    assert!(
        !stdout.contains("upstream build did not"),
        "neither direction of the cycle can evidence a skip: {stdout}"
    );
    assert!(
        stdout.contains("is unchecked"),
        "and the check that did not run must say so: {stdout}"
    );
    assert!(
        stdout.contains("build") && stdout.contains("review"),
        "naming which pair it could not order: {stdout}"
    );
    assert_eq!(out.status.code(), Some(0));
}

/// AC-7: the true positive survives.
///
/// This is the assertion the rejected fix would have failed. Excluding cyclic
/// edges from the check silences both directions, and only one of them is
/// noise: a verdict recorded with no code change is a review of nothing — a
/// rubber stamp — and is exactly the process defect day exists to surface.
/// Here the skip is on the acyclic `design -> build` edge, which must still
/// report while the cycle beside it does not.
#[test]
fn ac7_an_acyclic_skipped_step_is_still_reported() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &cyclic_claims());
    // code-change present, design-doc absent: build produced without design.
    let git = write_git_stub(dir.path(), &[], &["src/x.rs"]);

    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("build produced its output but upstream design did not"),
        "an acyclic skip must still be reported: {stdout}"
    );
}

/// AC-12: `day next` stops presenting the fix loop as a next step.
///
/// Before day#113 this listed `build` and `release` as equal successors of
/// `review`. They are not the same relation, and a reader — or a model — acting
/// on the list had no way to tell.
#[test]
fn ac12_next_separates_successors_from_revisits() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom("build", "bafyreib", &[], &["code-change"], &["review"], &[]),
            atom(
                "review",
                "bafyreir",
                &["code-change"],
                &["verdict"],
                &["release"],
                &["build"],
            ),
            atom("release", "bafyrel", &["verdict"], &["tag"], &[], &[]),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["next", "review"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    let after = stdout
        .find("After atom/review")
        .unwrap_or_else(|| panic!("no successors section: {stdout}"));
    let back = stdout
        .find("sends you back to")
        .unwrap_or_else(|| panic!("no revisits section: {stdout}"));
    assert!(back > after, "{stdout}");

    let successors = &stdout[after..back];
    assert!(successors.contains("atom/release"), "{stdout}");
    assert!(
        !successors.contains("atom/build"),
        "the fix loop must not be listed as a next step: {stdout}"
    );
    assert!(stdout[back..].contains("atom/build"), "{stdout}");
}

/// AC-13: `--revisits` reaches the recorded block, and stamps the version that
/// tells an older day it is behind rather than blaming the claim.
#[test]
fn ac13_declaring_a_revisit_records_it_and_stamps_the_version() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[atom("build", "bafyreib", &[], &["code-change"], &[], &[])],
    );

    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .args([
            "atom",
            "declare",
            "review",
            "--in",
            "code-change",
            "--out",
            "verdict",
            "--revisits",
            "build",
        ])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day");
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let log = appends(dir.path());
    let recorded = log
        .iter()
        .find(|a| a.contains("atom/review"))
        .unwrap_or_else(|| panic!("nothing recorded on atom/review: {log:?}"));
    assert!(
        recorded.contains(r#""revisits":["build"]"#),
        "the revisit must reach the block: {recorded}"
    );
    // Read from the constant, not spelled `2`. A literal here would keep
    // passing while meaning something else the next time the block version
    // moves — which is precisely how the migration fixture went stale.
    assert!(
        recorded.contains(&format!(
            r#""{}":{}"#,
            day::atoms::VERSION_KEY,
            day::atoms::INTERFACE_VERSION_REVISITS
        )),
        "a block using revisits must tell an older day to upgrade rather than \
         look malformed to it: {recorded}"
    );
}

/// F1, from the cold review of this branch — **`day next` must not call an
/// order it could not establish a terminal step.**
///
/// `docs/CONVENTIONS.md` gained this guarantee in the same branch, naming this
/// verb: consumers that need the ordering "drop the cyclic edges **and say that
/// they did**: could-not-check, never checked-and-clean". `day next` said the
/// opposite — a positive, false claim ("this is a terminal step in the current
/// vocabulary") about an atom that declares a successor day merely could not
/// order.
///
/// **AC-12's fixture could not reach this.** It is migrated and acyclic, so
/// `successors` is non-empty and the early return never fires — the exact
/// "a mechanism with two modes gets tested in whichever mode this repo is in"
/// trap `CLAUDE.md` records, on the branch that quotes it. This fixture is
/// cyclic and asserts that it is.
#[test]
fn f1_next_never_calls_an_unorderable_atom_terminal() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &cyclic_claims());
    let git = write_git_stub(dir.path(), &[], &[]);

    // Premise: `review` really does declare a successor, and it really is one
    // day cannot order.
    let doctor =
        String::from_utf8_lossy(&day(dir.path(), &kan, &git, &["doctor"]).stdout).into_owned();
    assert!(
        doctor.contains("cycle through") && doctor.contains("review -> build"),
        "premise: review's only `next` edge must be a cyclic one: {doctor}"
    );

    let out = day(dir.path(), &kan, &git, &["next", "review"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    // Keyed on the exact false claim, not on the absence of the word
    // "terminal": the corrected output says "this is NOT a terminal step",
    // which contains that substring. Keying a check on a phrase being absent
    // is how day#? filed a reader as `errored` because an unrelated finding
    // suppressed the phrase it looked for — `CLAUDE.md` states the rule, and
    // the first version of this assertion broke it.
    assert!(
        !stdout.contains("declares no successors"),
        "`review` declares next: [build]; calling it a terminal step is a false \
         claim about a graph day could not order:\n{stdout}"
    );
    assert!(
        stdout.contains("cycle through"),
        "and the reason it has no orderable successor must be stated — \
         could-not-check, never checked-and-clean:\n{stdout}"
    );
    assert!(
        stdout.contains("not a terminal step"),
        "the positive signal: day must say plainly that this is not a sink:\n{stdout}"
    );
}
