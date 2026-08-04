//! `.design/verification-that-can-fail.md` AC-1..AC-8, AC-25 — the revert
//! harness reports each of its outcomes for the state that actually produces it.
//!
//! **Every scenario here is driven end to end against a real scratch crate**,
//! never against a mock of cargo. The harness's whole value is that it observes
//! what a test binary does when a fix is taken away, and a mocked cargo would
//! validate the harness against the harness author's idea of libtest's output —
//! which is the stub-shaped blind spot `tests/kan_conformance.rs` exists as the
//! deliberate exception to.
//!
//! Each scratch project is a two-file crate with no dependencies, so a scenario
//! costs one trivial compile rather than a dependency graph.

mod common;

use common::ScratchCrate;
use std::path::Path;

/// A scratch git repo holding a crate: `src/lib.rs` at HEAD, plus whatever the
/// working tree adds on top. The working-tree state is the "fix"; the harness's
/// job is to take it away again.
struct Scratch {
    crate_: ScratchCrate,
}

impl Scratch {
    /// `head_lib` is committed; `work_lib` is left uncommitted (the fix), and
    /// `test_src` is written as an untracked integration test.
    fn new(head_lib: &str, work_lib: &str, test_src: &str) -> Self {
        let crate_ = ScratchCrate::new();
        crate_.write("src/lib.rs", head_lib);
        crate_.commit_all("before the fix");
        crate_.write("src/lib.rs", work_lib);
        crate_.write("tests/t.rs", test_src);
        Self { crate_ }
    }

    fn root(&self) -> &Path {
        self.crate_.root()
    }

    fn run(&self, args: &[&str]) -> (String, bool) {
        self.crate_.run_script("revert-demo.py", args)
    }

    fn git(&self, args: &[&str]) -> String {
        self.crate_.git(args)
    }

    fn lib(&self) -> String {
        self.crate_.read("src/lib.rs")
    }
}

const FIXED: &str = "pub fn answer() -> i32 { 2 }\n";
const BUGGY: &str = "pub fn answer() -> i32 { 1 }\n";
const ASSERTS_THE_FIX: &str = "#[test]\nfn demo_test() { assert_eq!(scratch::answer(), 2); }\n";

/// AC-1, AC-25 — **DEMONSTRATED**, in the mode an author uses before writing a
/// commit message: the fix is still uncommitted.
#[test]
fn a_fix_whose_test_observes_it_is_demonstrated() {
    let s = Scratch::new(BUGGY, FIXED, ASSERTS_THE_FIX);
    let (text, ok) = s.run(&["--tests", "t::demo_test"]);

    assert!(text.contains("DEMONSTRATED"), "{text}");
    assert!(ok, "DEMONSTRATED must exit 0: {text}");
    assert!(
        text.contains("Demonstrated-by: revert=HEAD tests=t::demo_test outcome=DEMONSTRATED"),
        "the trailer must be printed verbatim and copy-pasteable: {text}"
    );
    // The premise the whole harness turns on: the fix is back.
    assert_eq!(s.lib(), FIXED, "the working tree was not restored");
}

/// AC-1, AC-6 — **VACUOUS**: the headline finding. The fix is reverted and the
/// test written to close the finding passes anyway.
///
/// day#116's first instance was exactly this and it took a third review round to
/// see: *"reverting the entire fix left 337/337 green."* Driven with a real
/// vacuous pair rather than a mock, because a mock would assert that the harness
/// prints a word.
#[test]
fn a_test_that_does_not_observe_the_fix_is_vacuous() {
    let s = Scratch::new(
        BUGGY,
        FIXED,
        "#[test]\nfn demo_test() { assert!(scratch::answer() > 0); }\n",
    );
    let (text, ok) = s.run(&["--tests", "t::demo_test"]);

    assert!(text.contains("VACUOUS"), "{text}");
    assert!(!ok, "VACUOUS is a finding, not a pass: {text}");
    assert!(
        !text.contains("Demonstrated-by:"),
        "a vacuous run must print no trailer to paste: {text}"
    );
    assert_eq!(s.lib(), FIXED);
}

/// AC-1, AC-4 — **BASELINE-RED**, and *nothing is touched*.
///
/// day#114's rule, held by the harness that was written after it: a run against
/// a red baseline could not check, and the tree must come out exactly as it went
/// in — a harness that mutates a tree it then refuses to reason about is worse
/// than one that refuses earlier.
#[test]
fn a_failing_baseline_is_reported_and_nothing_is_reverted() {
    let s = Scratch::new(
        BUGGY,
        FIXED,
        "#[test]\nfn demo_test() { assert_eq!(scratch::answer(), 3); }\n",
    );
    let (text, ok) = s.run(&["--tests", "t::demo_test"]);

    assert!(text.contains("BASELINE-RED"), "{text}");
    assert!(
        text.contains("demo_test"),
        "the failing test must be named: {text}"
    );
    assert!(!ok, "{text}");
    assert_eq!(
        s.lib(),
        FIXED,
        "a red baseline must leave the tree untouched"
    );
}

/// AC-3 — **NO-SUCH-TEST**, never a pass.
///
/// A `cargo test` filter that matches nothing exits 0, so without this the
/// strongest possible result is one typo away. It is the same shape as the
/// defect the harness exists to find, one level up — and it fired for real on
/// this harness's first run, when `--quiet` turned out to suppress the per-test
/// lines the check reads.
#[test]
fn a_filter_that_matches_nothing_is_not_a_demonstration() {
    let s = Scratch::new(BUGGY, FIXED, ASSERTS_THE_FIX);
    let (text, ok) = s.run(&["--tests", "t::no_such_test_name"]);

    assert!(text.contains("NO-SUCH-TEST"), "{text}");
    assert!(!ok, "{text}");
    assert!(
        !text.contains("DEMONSTRATED") && !text.contains("VACUOUS"),
        "a name that matched nothing says nothing about coverage: {text}"
    );
}

/// AC-1, AC-5 — **DID-NOT-COMPILE**, and the tree is restored anyway.
///
/// Reverting a fix that changed a signature is an ordinary way to reach this,
/// not an exotic one, so it must be a distinct outcome rather than folded into
/// "the tests failed" — a build error is not evidence that a test asserts
/// something. This also drives the restore path *through an exception*, which is
/// AC-5's real subject: the `finally` block, not the happy path.
#[test]
fn a_revert_that_does_not_build_is_inconclusive_and_still_restores() {
    let s = Scratch::new(
        "pub fn unrelated() {}\n",
        "pub fn unrelated() {}\npub fn answer() -> i32 { 2 }\n",
        ASSERTS_THE_FIX,
    );
    let before = s.lib();
    let (text, ok) = s.run(&["--tests", "t::demo_test"]);

    assert!(text.contains("DID-NOT-COMPILE"), "{text}");
    assert!(!ok, "{text}");
    assert_eq!(s.lib(), before, "the finally-block restore did not run");
}

/// AC-1 — **REVERT-FAILED** when the change is test-only.
///
/// There is nothing to demonstrate: taking away a test does not reintroduce a
/// finding. Reporting `VACUOUS` here would blame the test for a change that
/// contained no fix.
#[test]
fn a_test_only_change_has_nothing_to_revert() {
    let s = Scratch::new(FIXED, FIXED, ASSERTS_THE_FIX);
    // Track the test file, so it is in the diff and gets filtered out rather
    // than being invisible for the unrelated reason that it is untracked.
    s.git(&["add", "tests/t.rs"]);
    s.git(&["commit", "-qm", "add the test"]);
    std::fs::write(
        s.root().join("tests/t.rs"),
        "#[test]\nfn demo_test() { assert_eq!(scratch::answer(), 2); }\n// edited\n",
    )
    .unwrap();

    let (text, ok) = s.run(&["--tests", "t::demo_test"]);
    assert!(text.contains("REVERT-FAILED"), "{text}");
    assert!(!ok, "{text}");
}

/// AC-1 — **NOT-RESTORED**: the named tests do not pass again afterwards.
///
/// Induced with a test that leaves state behind, which is a real reason a
/// demonstration cannot be trusted: if the second green depends on the first
/// run's side effects, "restored, and it passes" is not something the harness
/// observed. Loud rather than swallowed, because the alternative is a harness
/// that hands back a trailer for a run it could not complete.
#[test]
fn a_test_that_leaves_state_behind_cannot_support_a_demonstration() {
    let s = Scratch::new(
        BUGGY,
        FIXED,
        "#[test]\nfn demo_test() {\n\
         \x20   let marker = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"marker\");\n\
         \x20   let existed = marker.exists();\n\
         \x20   std::fs::write(&marker, \"x\").unwrap();\n\
         \x20   assert!(!existed, \"this test left state behind\");\n\
         \x20   assert_eq!(scratch::answer(), 2);\n\
         }\n",
    );
    let (text, ok) = s.run(&["--tests", "t::demo_test"]);

    assert!(text.contains("NOT-RESTORED"), "{text}");
    assert!(!ok, "{text}");
    assert_eq!(s.lib(), FIXED, "the source must still come back");
}

/// AC-2 — **the test half of a change is not reverted**, including when it
/// shares a file with the fix.
///
/// Nineteen files in day's `src/` carry a trailing `#[cfg(test)]` module, so
/// this is the common case rather than a corner. Reverting both halves deletes
/// the test, and a deleted test cannot fail.
#[test]
fn hunks_at_or_after_cfg_test_are_kept_in_place() {
    let s = Scratch::new(
        "pub fn answer() -> i32 { 1 }\n\n#[cfg(test)]\nmod tests {\n    // nothing yet\n}\n",
        "pub fn answer() -> i32 { 2 }\n\n#[cfg(test)]\nmod tests {\n\
         \x20   #[test]\n    fn unit_asserts_the_fix() { assert_eq!(super::answer(), 2); }\n}\n",
        "",
    );
    let (text, ok) = s.run(&["--tests", "lib::unit_asserts_the_fix"]);

    assert!(
        text.contains("1/2 hunk(s)") && text.contains("test-side hunk"),
        "the report must show what was reverted and what was kept: {text}"
    );
    assert!(
        text.contains("DEMONSTRATED") && ok,
        "the unit test survives the revert and observes the fix: {text}"
    );
}

/// AC-7, AC-8 — **`--verify` re-derives the claim, in a worktree, and refutes a
/// trailer that is not true.**
///
/// The trailer is a claim about the work. A claim nothing can contradict is not
/// verification, which is `docs/CONVENTIONS.md`'s rule about evidence applied to
/// the commit message. So the assertion here is the *refutation*: a commit whose
/// trailer says DEMONSTRATED for a test that in fact passes under revert must
/// fail, and the caller's tree must be exactly where it was.
#[test]
fn verify_refutes_a_trailer_that_does_not_hold_and_touches_nothing() {
    let s = Scratch::new(
        BUGGY,
        FIXED,
        "#[test]\nfn demo_test() { assert!(scratch::answer() > 0); }\n",
    );
    s.git(&["add", "-A"]);
    s.git(&[
        "commit",
        "-qm",
        "fix the answer\n\nDemonstrated-by: revert=HEAD tests=t::demo_test \
             outcome=DEMONSTRATED",
    ]);

    let head_before = s.git(&["rev-parse", "HEAD"]);
    // Tracked files only, and deliberately so rather than to make this pass:
    // `--verify` builds in `<repo>/target/revert-demo`, sharing the caller's
    // artifact cache the way every other cargo invocation does. `target/` is
    // gitignored in any Rust repo; what must not move is the source being
    // checked, which is what this compares.
    let tracked = ["status", "--porcelain", "--untracked-files=no"];
    let status_before = s.git(&tracked);

    let (text, ok) = s.run(&["--verify", "HEAD"]);

    assert!(
        !ok,
        "the trailer claims DEMONSTRATED for a test that passes under revert: {text}"
    );
    assert!(
        text.contains("VACUOUS") && text.contains("claims DEMONSTRATED"),
        "the failure must name what was re-derived and what was claimed: {text}"
    );
    assert_eq!(head_before, s.git(&["rev-parse", "HEAD"]));
    assert_eq!(
        status_before,
        s.git(&tracked),
        "--verify must work in a worktree and leave the caller's tracked files alone"
    );
}

/// AC-10 — **a trailer that does not parse is refused**, rather than being read
/// as an absent one.
///
/// The two are different: no trailer means nothing was claimed, and a malformed
/// trailer means something was claimed and cannot be checked. Collapsing them
/// would let a typo silently opt a commit out of verification.
#[test]
fn a_malformed_trailer_is_refused_rather_than_ignored() {
    let s = Scratch::new(BUGGY, FIXED, ASSERTS_THE_FIX);
    s.git(&["add", "-A"]);
    s.git(&[
        "commit",
        "-qm",
        "fix\n\nDemonstrated-by: I reverted it and it failed",
    ]);

    let (text, ok) = s.run(&["--verify", "HEAD"]);
    assert!(!ok, "{text}");
    assert!(text.contains("does not parse"), "{text}");
}

/// AC-25 — **both modes, one fix.** The uncommitted mode is what an author runs
/// before writing the message; `--rev` is what re-derives it afterwards. They
/// must agree, or the trailer an author pastes describes a different run from
/// the one CI checks.
#[test]
fn the_uncommitted_and_committed_modes_agree() {
    let s = Scratch::new(BUGGY, FIXED, ASSERTS_THE_FIX);
    let (uncommitted, _) = s.run(&["--tests", "t::demo_test"]);
    assert!(uncommitted.contains("DEMONSTRATED"), "{uncommitted}");

    s.git(&["add", "-A"]);
    s.git(&["commit", "-qm", "fix the answer"]);
    let (committed, _) = s.run(&["--rev", "HEAD", "--tests", "t::demo_test"]);
    assert!(committed.contains("DEMONSTRATED"), "{committed}");
}

/// AC-7 — **a symbolic rev is resolved in the caller's repo, once.**
///
/// `HEAD~1` means one thing in the caller's repo and something else entirely
/// inside the detached worktree, where `HEAD` *is* the commit being verified. So
/// `--verify HEAD~1` read the right commit's trailer and then reverted the diff
/// of the commit before it — a confident answer about the wrong change.
///
/// Found by running `--verify` on this branch's own history, which is the third
/// defect in this milestone that a test could not have found and one command
/// did.
#[test]
fn a_symbolic_rev_resolves_against_the_callers_repo_not_the_worktree() {
    let s = Scratch::new(BUGGY, FIXED, ASSERTS_THE_FIX);
    s.git(&["add", "-A"]);
    s.git(&[
        "commit",
        "-qm",
        "fix the answer\n\nDemonstrated-by: revert=HEAD tests=t::demo_test \
         outcome=DEMONSTRATED",
    ]);
    // One more commit, so `HEAD~1` is the commit carrying the trailer.
    std::fs::write(s.root().join("README.md"), "unrelated\n").unwrap();
    s.git(&["add", "-A"]);
    s.git(&["commit", "-qm", "unrelated"]);

    let (text, ok) = s.run(&["--verify", "HEAD~1"]);
    assert!(
        ok,
        "HEAD~1 names the commit whose trailer holds; it must re-derive: {text}"
    );
    assert!(text.contains("DEMONSTRATED"), "{text}");
    assert!(
        text.contains("reverting src/lib.rs"),
        "the diff reverted must be the trailer-carrying commit's own: {text}"
    );
}
