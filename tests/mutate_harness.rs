//! `.design/verification-that-can-fail.md` AC-9, AC-10, AC-26 — the mutation
//! harness distinguishes "could not check" from "checked and found nothing".
//!
//! day#114, and it is the rule the harness exists to enforce, missing from the
//! harness. Every scenario is driven against a real scratch crate and a real
//! cargo, because what `mutate.py` reports is decided by reading libtest's
//! output — a mocked cargo would validate it against its author's idea of that
//! output rather than against the thing itself.

mod common;

use common::ScratchCrate;
use std::process::Command;

const LIB_TWO: &str = "pub fn answer() -> i32 { 2 }\n";

/// `.design/verification-that-can-fail.md` AC-9 — **a red baseline is reported,
/// and nothing is mutated.**
///
/// day#114. Without this, every mutation against a red suite reports `CAUGHT`,
/// including ones that assert nothing, because a test failing for an unrelated
/// reason is indistinguishable from a test catching the mutation. That happened:
/// two telos tests were already broken, a CID-hash mutation reported `CAUGHT`,
/// and only the obvious irrelevance of the failing names gave it away.
///
/// It is the rule the harness exists to enforce, missing from the harness — a
/// run against a red baseline *could not check* and reported the strongest
/// possible result instead.
#[test]
fn mutate_reports_a_red_baseline_and_leaves_the_file_alone() {
    let c = ScratchCrate::new();
    c.write("src/lib.rs", LIB_TWO).write(
        "tests/a.rs",
        "#[test]\nfn already_broken() { assert_eq!(scratch::answer(), 3); }\n",
    );

    let (text, ok) = c.run_script("mutate.py", &["src/lib.rs", "{ 2 }", "{ 99 }", "probe"]);

    // Keyed on the outcome LINE, not on the absence of a word anywhere in the
    // output. The first version of this assertion forbade the substring
    // "CAUGHT" and failed against a correct run, because the message explains
    // that a red baseline is what makes every mutation report CAUGHT. That is
    // `CLAUDE.md`'s "never key a classifier on the absence of a phrase",
    // committed inside the milestone about checks that cannot fail honestly.
    let outcome = text.lines().next().unwrap_or_default();
    assert_eq!(
        outcome, "probe: BASELINE-RED (the suite was already failing; nothing was mutated)",
        "the reported outcome must be the red baseline itself: {text}"
    );
    assert!(
        text.contains("already_broken"),
        "the failing test must be named: {text}"
    );
    assert!(!ok, "a run that could not check must not exit 0: {text}");
    assert_eq!(c.read("src/lib.rs"), LIB_TWO, "the file must be untouched");
}

/// AC-26 — **`--no-fail-fast`, so the catcher list is not truncated at the first
/// failing test binary.**
///
/// Two integration targets both assert the mutated value. Without
/// `--no-fail-fast`, cargo stops after the first binary fails and the report
/// names one catcher, so the author concludes the coverage is thinner than it
/// is — and thinness is exactly what a mutation run is consulted about.
#[test]
fn mutate_names_every_catching_test_not_just_the_first_binary() {
    let c = ScratchCrate::new();
    c.write("src/lib.rs", LIB_TWO)
        .write(
            "tests/a.rs",
            "#[test]\nfn a_catches_it() { assert_eq!(scratch::answer(), 2); }\n",
        )
        .write(
            "tests/b.rs",
            "#[test]\nfn b_catches_it() { assert_eq!(scratch::answer(), 2); }\n",
        );

    let (text, _) = c.run_script("mutate.py", &["src/lib.rs", "{ 2 }", "{ 99 }", "probe"]);

    assert!(text.contains("CAUGHT"), "{text}");
    assert!(
        text.contains("a_catches_it") && text.contains("b_catches_it"),
        "both catching tests must be named, in different binaries: {text}"
    );
}

/// AC-10 — **`target/` is rebuilt after the restore.**
///
/// Restoring the source is not the same as restoring the artifacts (day#114).
/// A manual probe run immediately after a mutation otherwise drives a binary
/// compiled from the mutant, which is how a defect that had *already been
/// fixed* got "measured" as still present.
///
/// Asserted by running the binary `target/` holds **without rebuilding it**: if
/// the harness left the mutant compiled, the output is the mutant's.
#[test]
fn mutate_leaves_target_built_from_the_restored_source() {
    let c = ScratchCrate::new();
    c.write("src/lib.rs", LIB_TWO)
        .write(
            "src/main.rs",
            "fn main() { println!(\"{}\", scratch::answer()); }\n",
        )
        .write(
            "tests/a.rs",
            "#[test]\nfn a_catches_it() { assert_eq!(scratch::answer(), 2); }\n",
        );

    let (text, _) = c.run_script("mutate.py", &["src/lib.rs", "{ 2 }", "{ 99 }", "probe"]);
    assert!(text.contains("CAUGHT"), "{text}");

    let built = Command::new(c.root().join("target/debug/scratch"))
        .output()
        .expect("the mutation run should have built the binary");
    assert_eq!(
        String::from_utf8_lossy(&built.stdout).trim(),
        "2",
        "target/ still holds a binary built from the mutant; a probe run now \
         would measure the mutation rather than the restored source"
    );
}
