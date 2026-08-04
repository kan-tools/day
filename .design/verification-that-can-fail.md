# Feature: verification that can fail

## Summary

Six issues, one thesis: **every check day owns is able to report clean without
having checked anything, and in a different mode for each one.** day#116's test
cannot observe its guarantee, day#101's guarantee is not reached from every
path, day#114's harness reports `CAUGHT` against a red baseline, day#118's
release step cannot fail at the moment its omission is created, day#91's
fallback is never the mode this repo is in, and day#89's documented invocations
have no check at all. This milestone gives each of those a way to fail, and adds
one general instrument — a **revert harness** that applies the inverse of a named
change, asserts named tests fail, restores, and asserts they pass — plus the rule
that a commit closing a finding carries a demonstration the build re-derives.

The harness is built first and the rule is written last, deliberately: a rule
that costs something on every fix commit with no tooling behind it is ceremony,
and ceremony is routed around. If the demonstration is not nearly free by the
end of this milestone, the rule does not ship and this document records that
instead.

## Requirements

### The revert harness (day#116)

- REQ-1: `scripts/revert-demo.py` reports one of seven outcomes and never
  conflates them: `DEMONSTRATED` (baseline green, reverted, the named tests
  failed, restored, they pass again), `VACUOUS` (the named tests still passed
  with the fix reverted — the headline finding), `BASELINE-RED`,
  `DID-NOT-COMPILE`, `REVERT-FAILED`, `NO-SUCH-TEST`, `NOT-RESTORED`. The last
  five are could-not-checks and are named as such in their own words, per
  `CLAUDE.md`'s rule that a verification tool distinguishes "could not check"
  from "checked and found nothing".
- REQ-2: The change to invert may be the **uncommitted working-tree diff against
  `HEAD`** (the default, so a demonstration can be run *before* the commit
  message is written) or a **committed revision** (`--rev <ref>`).
- REQ-3: **The test half of the change is not reverted.** A fix commit normally
  carries the fix and the test that closes the finding; reverting both deletes
  the test, and a deleted test cannot fail. Hunks under `tests/` are excluded,
  and within a `src/*.rs` file, hunks at or after the file's `#[cfg(test)]` line
  are excluded. `--include`/`--exclude` pathspecs override. The harness prints
  which files and how many hunks it reverted and kept, so a mismatch with intent
  is visible rather than inferred.
- REQ-4: **The named tests are asserted to have run.** A `cargo test` filter
  matching zero tests reports `NO-SUCH-TEST`, never a pass. This is the harness
  holding itself to day#116's premise rule: the thing the demonstration turns on
  is that the named test *executed*, and a filter typo otherwise reads as the
  strongest possible result.
- REQ-5: **A green baseline is a precondition.** The named tests are run before
  anything is reverted; if any fails, the outcome is `BASELINE-RED`, the tree is
  not touched, and the failing test is named. (day#114's rule, applied to the new
  harness at the point it is written rather than retrofitted.)
- REQ-6: **The restore is verified, not assumed.** Every touched file is
  snapshotted before the revert and compared byte-for-byte after; files are
  written back with a fresh mtime so cargo's change detection sees the restore;
  and the named tests are re-run and must pass. Anything else is `NOT-RESTORED`
  at non-zero exit. Restoration happens in a `finally` block, so an interrupt
  cannot leave a reverted tree.
- REQ-7: On `DEMONSTRATED`, the harness prints the exact commit trailer to paste:

  ```
  Demonstrated-by: revert=<ref|worktree> tests=<a>,<b> outcome=DEMONSTRATED
  ```

  Verbatim and copy-pasteable, because the cost of the rule is the cost of this
  step.
- REQ-8: `scripts/revert-demo.py --verify <rev>` re-derives the demonstration a
  commit claims. It reads that commit's `Demonstrated-by:` trailer, checks the
  commit out into a **detached worktree** (so the caller's tree is never
  touched), reverts the non-test half there, and asserts the named tests fail and
  then pass on restore. A commit whose trailer cannot be re-derived is a failure,
  not a warning.
- REQ-9: A CI job re-verifies every commit on the branch that carries a
  `Demonstrated-by:` trailer. It fails on `VACUOUS`, and it reports a
  could-not-check outcome distinctly from a verified one — a job that cannot
  reach the commits must say so rather than pass.
- REQ-10: A hermetic test asserts the trailer grammar and that `--verify`
  **rejects** a trailer naming a test that does not exist, a malformed trailer,
  and a trailer whose claimed outcome is not the one re-derivation produces. The
  trailer is a claim about the work; a claim nothing can refute is not
  verification.

### The mutation harness (day#114)

- REQ-11: `scripts/mutate.py` runs the suite once before mutating and reports
  `BASELINE-RED` — a seventh outcome in its own vocabulary, neither `CAUGHT` nor
  `SURVIVED` — naming a failing test, without mutating.
- REQ-12: After restoring, `mutate.py` rebuilds so `target/` is not left holding
  artifacts compiled from the mutant. This cost real time twice: a manual probe
  against a stale binary "measured" a defect that had already been fixed.
- REQ-13: `mutate.py` passes `--no-fail-fast`, so the reported list of catching
  tests is not truncated at the first failing binary.

### The release row (day#118)

- REQ-14: `scripts/cut-release.sh` measures the release it is cutting with
  `scripts/run-migration-cell.sh` and appends `<tag>\t<outcome>` to
  `tests/fixtures/migration-expectations.tsv`, **committing it before the tag is
  created**, so the tagged tree contains its own row. Same order, and the same
  reason, as the release claim: a step that happens after the tag is a step that
  gets dropped when the cadence compresses.
- REQ-15: `.github/workflows/migration-matrix.yml` stops excluding the tag being
  released. Every released tag has a measured row with no exceptions and no
  window. The current tag's cell is a well-defined measurement — that binary
  against this commit's block shapes — and the next release re-measures it
  against the corpus of *that* commit, which either confirms the row or reports
  that the blast radius moved. That second reading is the failure the matrix
  exists to produce, and it is the one the exclusion was deferring.
- REQ-16: `cut-release.sh` refuses to cut a release while any *earlier* released
  tag lacks a row, naming the tag and the command that measures it. Belt to
  REQ-14's braces: if a row is ever lost, the refusal lands on the maintainer's
  machine at the start of the next cut rather than in someone's CI a release
  later.
- REQ-17: The origin/main guard's meaning narrows — it now proves the *parent* of
  the tagged commit is on the remote, because the row commit is made by the
  script and cannot be. That narrowing is documented at the guard, and the
  closing instruction becomes `git push origin main "$tag"`.

### Fallback modes (day#91)

- REQ-18: A source scan over `src/` finds doc comments and comments that
  **describe a fallback** — the phrases an author naturally writes ("falls back",
  "degrades to", "no … means …") — and fails the build unless the site names a
  fallback state covered by a test in `tests/fallbacks.rs`, or carries a per-site
  `fallback-untested: <why>` hatch. Detection-first and hatch-to-exempt, matching
  `a_failed_kan_read_is_never_swallowed` and `an_ordering_is_never_read_off_the_raw_next`
  rather than inverting them: a registration marker would report clean by having
  found nothing, which is keying a check on the absence of a phrase.
- REQ-19: The scan's doc comment states its blind spot — **a degrade path whose
  comment uses none of the watched phrases is invisible to it** — rather than
  reporting "clean" while meaning "clean of the shapes I happen to know".
- REQ-20: Every test in `tests/fallbacks.rs` opens with an explicit **premise
  assertion**: that the fixture is in the un-favourable state, named as a state
  and not as a feature. `current-cycle-position`'s AC-4 is the model. A test in
  that file asserts every fallback test has one, so the file cannot grow a
  vacuous entry.
- REQ-21: day#91's listed candidates are back-filled, starting with the two the
  issue evidences: no `v*` tag (position reads cumulative) and
  `Git::position_fingerprint` with no boundary.

### Documented invocations (day#89)

- REQ-22: A test extracts fenced shell blocks from `README.md`, `docs/*.md` and
  `commands/*.md` and runs every `day …` invocation in a scratch directory
  against a stub kan and a stub git, asserting it does not fail on **argument
  parsing**. The bar is that every documented invocation parses and runs — not
  that it produces the right output, which would need per-example fixtures and
  would rot.
- REQ-23: Blocks run under `zsh` where available, falling back to `sh`. The
  zsh/bash difference *is* day#83 — `--scope published-artifact=v0.5*` unquoted
  fails with `no matches found` in zsh, macOS's default shell and this repo's —
  and running through `sh` would have missed it. The fallback is itself a
  fallback mode and gets a `tests/fallbacks.rs` entry under REQ-20.
- REQ-24: Exclusion is by a **rule stated in the test**, not a hand-maintained
  skip list: a block is skipped if it contains a `<placeholder>` or its command
  is not `day`. Every skip is printed with its reason, so a silently shrinking
  corpus is visible — the failure `capture-block-corpus.sh` had twice.
- REQ-25: No documented example may reach a real log. Every invocation runs with
  `DAY_KAN_BIN` pointed at a stub in a scratch directory.

### The call-site guarantee (day#101)

- REQ-26: A source scan fails the build on a `pub fn` in `src/` whose only
  callers are inside `#[cfg(test)]` regions or under `tests/`, with a per-site
  `test-only-caller-ok: <why>` hatch. `BlockSchemas::extract` and
  `Compat::is_notable` were both exactly this, both `pub`, and clippy was silent
  for both because `pub` suppresses dead-code detection. A `pub fn` whose only
  callers are tests is either dead or a requirement about to go nominal.
- REQ-27: If REQ-26's scan produces more than a handful of pre-existing
  offenders, the scan ships with them hatched and each hatch states why —
  never with the rule weakened to make the count small. The count itself is
  recorded in this document.

### The rule (day#116, and the milestone's own test)

- REQ-28: Every fix commit in this milestone that closes a finding carries a
  `Demonstrated-by:` trailer produced by the harness. The milestone is the
  harness's first user.
- REQ-29: `CLAUDE.md` gains the rule **only if** REQ-28 showed the demonstration
  to be nearly free. The measurement — wall-clock per demonstration and how many
  came back `VACUOUS` — is recorded in this document either way, and if the rule
  does not ship, the reason is recorded in kan rather than the requirement being
  quietly dropped.
- REQ-30: `commands/adversarial-review.md` states kan's ADR-52 — a round of fixes
  to a `BLOCK` gets its own review — which has now held eight times in this repo
  and is still something a person has to remember.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) Each of the seven outcomes is produced by a test that puts
      the harness in that state, and no two states produce the same token.
- [ ] AC-2: (REQ-3) Given a diff touching `src/foo.rs` above and below a
      `#[cfg(test)]` line, the harness reverts only the hunks above it, and
      prints the kept/reverted hunk counts.
- [ ] AC-3: (REQ-4) A demonstration naming a test that does not exist reports
      `NO-SUCH-TEST` at non-zero exit, and does not report `DEMONSTRATED` or
      `VACUOUS`.
- [ ] AC-4: (REQ-5) With a named test already failing, the harness reports
      `BASELINE-RED`, names it, and the working tree is byte-identical
      afterwards.
- [ ] AC-5: (REQ-6) Killing the harness mid-run leaves the tree byte-identical;
      asserted by driving the restore path directly and comparing hashes.
- [ ] AC-6: (REQ-1, REQ-7) A fix whose test does *not* assert the fix reports
      `VACUOUS` and prints no trailer. Driven by a real vacuous pair, not by a
      mock.
- [ ] AC-7: (REQ-8) `--verify <rev>` leaves the caller's working tree and index
      untouched, asserted by comparing `git status --porcelain` and `git
      rev-parse HEAD` before and after.
- [ ] AC-8: (REQ-9, REQ-10) The CI job fails on a commit whose trailer claims
      `DEMONSTRATED` for a test that in fact passes under revert; asserted by a
      fixture repository, not by reading the workflow.
- [ ] AC-9: (REQ-11) With a test forced red, `mutate.py` prints `BASELINE-RED`,
      names the failing test, and the target file is unmodified.
- [ ] AC-10: (REQ-12) After a `mutate.py` run, a binary built from `target/`
      behaves as the restored source does; asserted by a marker the mutation
      changes and the restored source does not.
- [ ] AC-11: (REQ-14) After `cut-release.sh` on a scratch repository, the tagged
      commit contains a row for its own tag whose outcome equals what
      `run-migration-cell.sh` reports for that binary.
- [ ] AC-12: (REQ-15) The matrix workflow contains no exclusion of
      `GITHUB_REF_NAME`, and `tests/` asserts every `v*.*.*` tag visible to the
      test has a row — reporting *could not check* rather than passing when tags
      are not fetched.
- [ ] AC-13: (REQ-16) With a row deleted, `cut-release.sh` refuses before
      building, names the tag, and prints the measuring command.
- [ ] AC-14: (REQ-18) A new `src/` comment saying "falls back" with no
      corresponding `tests/fallbacks.rs` entry fails the build; adding the hatch
      or the test makes it pass. Both directions asserted.
- [ ] AC-15: (REQ-20) A `tests/fallbacks.rs` test with no premise assertion
      fails the file's own check.
- [ ] AC-16: (REQ-21) `fallback_no_release_boundary` asserts, as its premise,
      that its fixture repository has no `v*` tag — and fails when the fallback
      is removed, demonstrated with the revert harness.
- [ ] AC-17: (REQ-22, REQ-23) The unquoted `--scope published-artifact=v0.5*`
      form fails the docs test under zsh; the quoted form passes. This is
      day#83 reproduced as a test, and it is what shows the harness can fail.
- [ ] AC-18: (REQ-24) Every skipped block is printed with its reason, and the
      test asserts the number of blocks it *ran* against an explicit list of
      files, not a count — a generator whose failure mode is less output needs an
      exhaustive expectation.
- [ ] AC-19: (REQ-25) The docs test's scratch directory contains no `.kan/`
      belonging to this repo, and the stub records every invocation it received.
- [ ] AC-20: (REQ-26) Adding a `pub fn` to `src/` called only from a
      `#[cfg(test)]` module fails the build; adding the hatch makes it pass.
- [ ] AC-21: (REQ-27) The offender count at the time the scan lands is recorded
      in this document, and every hatch in the initial landing states a reason
      specific to that site.
- [ ] AC-22: (REQ-28) Every commit in the milestone's PR that closes a finding
      carries a trailer, and CI re-verifies each one.
- [ ] AC-23: (REQ-29) This document records the measured per-demonstration cost
      and the `VACUOUS` count, and `CLAUDE.md` either gains the rule or does not,
      matching what the measurement showed.
- [ ] AC-24: (REQ-30) `commands/adversarial-review.md` names ADR-52, and
      `tests/plugin.rs` asserts it does.
- [ ] AC-25: (REQ-2) Both modes are driven end to end against the same fix: the
      default mode with it uncommitted, `--rev HEAD` with it committed, and both
      report `DEMONSTRATED` for the same named test.
- [ ] AC-26: (REQ-13) With two tests catching one mutation, `mutate.py` names
      both. Asserted against a mutation with two known catchers, so a truncated
      list is observable.
- [ ] AC-27: (REQ-17) `cut-release.sh`'s origin/main guard carries a comment
      stating that it proves the *parent* of the tagged commit is pushed, and the
      script's closing instruction pushes `main` and the tag together. Asserted
      by a test over the script text, which is where every other guarantee about
      this script is asserted.
- [ ] AC-28: (REQ-19) The fallback scan's stated blind spot is **demonstrated,
      not claimed**: a probe file containing an undocumented degrade path is not
      flagged, and the test says so. A scan whose limits are asserted only in
      prose is the thing this milestone is about.

## Architecture

**Where the harness lives.** `scripts/revert-demo.py`, beside `mutate.py`, in
Python for the same reason: the logic is snapshot/apply/run/compare/restore, and
a shell version of that is where the pipeline-status defects in `cut-release.sh`
came from. It shells `git` and `cargo` and touches nothing else.

**Two modes, two mechanisms.** The default mode inverts the uncommitted diff in
place, because that is the moment the author has the fix and not yet the commit
message. `--verify` mode operates in a detached `git worktree`, so re-derivation
in CI can never touch the caller's tree — and worktrees are already the idiom
this repo uses for building historical readers in `migration-matrix.yml`.

**Reverting the fix but not the test** is the design's one real difficulty, and
it is why `--include`/`--exclude` exist. The default rule is mechanical: drop
hunks under `tests/`, and drop hunks whose new-file start line is at or after the
enclosing file's `#[cfg(test)]` line. Nineteen files in `src/` carry a trailing
test module, so this is the shape that matters here. The rule's failure mode is
**visible rather than silent**: if it excludes too much, the fix is not fully
reverted and the outcome is `VACUOUS`, which is loud; if it excludes too little,
the named test is deleted and the outcome is `NO-SUCH-TEST`, which is also loud.
Neither degrades to `DEMONSTRATED`. That property is what makes a heuristic
acceptable here.

**Outcome precedence** follows `mutate.py`'s: could-not-check outranks
checked-and-clean. `DID-NOT-COMPILE` is checked before "tests failed", because a
reverted fix that no longer compiles says nothing about whether the test asserts
anything — and reverting a fix that changed a signature is a *normal* way to
reach that state, not an exotic one.

**The trailer is a claim, and `--verify` is what makes it refutable.** This is
the same move `docs/CONVENTIONS.md` makes for evidence generally: a statement
about the work that nothing can contradict is not evidence. The CI job re-derives
rather than trusts, which is also why AC-8 is asserted against a fixture
repository — a test that reads the workflow YAML checks day's own side of the
interface and would miss the interface, which `CLAUDE.md` records as this repo's
most-repeated failure.

**The fallback scan is prose-detected on purpose.** Every other option inverts
the hatch direction and thereby keys on absence. Detecting the words an author
writes has an obvious blind spot, and REQ-19 requires the scan to say so in its
own doc comment — the standard both existing scans were held to last milestone.
The scan's value is not that it is complete; it is that adding a documented
fallback now costs a test, and that the existing fifteen-odd sites get triaged
once.

**`cut-release.sh`'s ordering** becomes: verify the tree and the remote → build,
test, clippy, fmt → `day assess docs` → **measure and commit the migration row**
→ record the release claim → tag. The row commit sits between the last read-only
check and the two write steps, and it is the reason the origin/main guard's
meaning narrows (REQ-17). That narrowing is a real cost of the unification and is
written at the guard rather than in a design document nobody re-reads.

**Nothing here touches day's shipped behaviour**, with one exception: REQ-21's
back-fill may find a fallback that is wrong, in which case the fix is day's
behaviour and gets its own demonstration. That is the expected outcome rather
than a surprise — day#91's evidence is that these paths are the ones a fresh
clone runs.

## What the milestone measured about itself

REQ-29 made the rule conditional on the demonstration being nearly free, and
required the measurement to be recorded either way. It is:

| | measured |
| --- | --- |
| one demonstration, cold | **11.9 s** |
| one demonstration, warm | **2.0 s** |
| the same demonstration, test target unqualified | **3 m 54 s** |
| demonstrations run over the milestone's own commits | 4 `DEMONSTRATED`, 2 `REVERT-FAILED` (test-only), 0 `VACUOUS` |
| defects found by *using* the tooling, not by testing it | 7 |

So the rule ships. The load-bearing number is the third row: an unqualified
`cargo test` filter builds every integration target three times over, and
qualifying it (`plugin::some_test`) is the difference between a rule that
describes what already happens and one that gets routed around.

The two `REVERT-FAILED` outcomes are day#101 and day#89, whose commits add a
guard rather than fix behaviour — there is nothing to invert, the harness says so
in those words, and the guard is instead shown to fire. That case is written into
`CLAUDE.md` beside the rule rather than left to judgement.

**The seven defects are the stronger argument, and none was found by a test.**
Five were in `revert-demo.py` itself: `--quiet` suppressing the very lines that
prove a test ran; the test half of a change being reverted with it; default diff
context merging a fix and its adjacent test module into one hunk; `--verify
HEAD~1` re-resolving against the worktree's own HEAD and verifying the wrong
commit; and `--verify` perturbing the tree it was checking through
`CARGO_TARGET_DIR`. The sixth was in `tests/common`, found by running `--verify`
on the commit that had just added the tests. Every one of them would have shipped
a harness that reported confidently and wrongly.

**The seventh arrived after this table first said six, and is the sharpest of
them.** `.github/workflows/revert-demo.yml` also triggered on pushes to `main`,
where `merge-base(origin/main, HEAD)` *is* `HEAD` — so the commit range is empty
by construction and the job was permanently green for having found nothing. A
check that reports clean by finding nothing, in the job built to enforce the rule
against exactly that, written one commit after the rule. It was found by pushing
the branch and noticing the run that never appeared, which is dogfooding and not
review or testing. Recorded by correcting the count rather than by leaving the
table at the number that was true when it was written.

## Resolved Questions

### Q1: Does REQ-26's scan produce a tolerable number of pre-existing offenders?

**Measured, then decided.** Over `src/` at the head of this milestone: **163
`pub fn` definitions outside `#[cfg(test)]`, and 0 offenders.** So REQ-26 ships
as specified, with no hatches in its initial landing — which resolves AC-21 by
making it vacuous, and that is worth saying rather than leaving to be inferred.

A scan that has never been observed to fire is a scan nobody has reason to
believe, and "0 offenders" is exactly the reading that would be produced by a
scan that does not work. So it was validated against the instance day#101 named:
run over the tree at `1e02220^` — the commit that dropped the dead code — it
reports **exactly one**, `src/compat.rs: is_notable`, and nothing else. The other
named instance, `BlockSchemas::extract`, has since gained a production caller,
which is why it is correctly silent today.

`BlockSchemas::extract` and `Compat::is_notable` were the two the issue lists,
and both were `pub` with only `#[cfg(test)]` callers, which is why clippy was
silent for both. The fear that motivated this question — that day being a library
means many `pub fn`s legitimately have no in-crate caller — did not materialise,
because a library item still appears in the production corpus at its definition
*and* wherever the crate re-exports or calls it.

## Out of Scope

- **day#123, day#120, day#84, day#119** — the `design check` cluster. day#89 is
  arguably day#123's general case, but day#123 is a defect in a shipped verb and
  belongs with the verb's own design pass, not with the harness milestone.
- **Vocabulary packs (day#73)** and everything else in v0.12.
- **Making `/adversarial-review` a declared fix-round step in the atom
  vocabulary**, which day#116 §6 also asks for. REQ-30 lands the documentation
  half; the vocabulary half is an atom change and wants its own pass.
- **Retrofitting demonstrations onto past fix commits.** The rule applies going
  forward; back-filling would produce trailers nobody derived, which is the
  fabricated-evidence failure this milestone exists to prevent.
