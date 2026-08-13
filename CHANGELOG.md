# Changelog

All notable changes to `day`, newest first.

**Every release so far is a prerelease.** There is no stable line, the
conventions are v0, and `cargo install day` will not select a version without
an explicit `--version`. Because every release is marked as a prerelease on
GitHub, `/releases/latest` returns 404 — GitHub excludes prereleases from it.
Nothing depends on that endpoint; the release workflow triggers on tag push.

**The entries through `v0.12.0-beta.1` were reconstructed after the fact**,
from the tags, the commit history, and the issues closed in each release
window; they were written when the repository had 17 tags and no GitHub
Releases at all. From `v0.12.0-beta.2` onward they are written at release
time. Where a release's own commits name an issue, that issue is cited; where
they do not, the entry describes what the commits did. Milestone names are the
ones the repository actually used, because they say what each release was
*about* better than a version number does.

**Which release contains an item is decided by ancestry, never by close date.**
`git tag --contains <commit>` is the question; an issue's `closedAt` is not.
day#17 closed three minutes after the `v0.4.0-beta.1` tag and shipped in v0.4;
day#131 closed six hours after the `v0.12.0-beta.1` tag and shipped in
v0.12.0-beta.1. Both would be filed a release late by a window-based pairing,
and one of them was.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
versioning is [SemVer](https://semver.org/spec/v2.0.0.html), pre-1.0.

## [Unreleased]

### Changed

- Measured kan `v0.12.0-beta.5` and `v0.13.0-beta.1`, adopting the latter;
  README and bootstrap install guidance now point to the newest verified pair.
- Reworked the README around installation and a first useful workflow, with
  theory and detailed contracts left to the supplemental documentation.
- Refreshed the roadmap around adoption, workflow ergonomics, transportable
  vocabularies, and the v1.0 legibility bar.

## [v0.12.1-beta.3] — 2026-08-13

### Fixed

- The Windows CI bootstrap assertion now reaches the command it measures;
  PowerShell previously rejected an undelimited variable followed by a colon
  while parsing the workflow itself ([#209]).

[#209]: https://github.com/kan-tools/day/issues/209

## [v0.12.1-beta.2] — 2026-08-13

*Publication recovery.* The beta.1 tag passed the local release ceremony but
its publication workflow caught a changelog/tag-date mismatch after midnight.
beta.2 carries the same approved audit fixes with coherent release metadata;
beta.1 was never published to crates.io or as a GitHub Release.

## [v0.12.1-beta.1] — 2026-08-13

*Bugfix audit release.* A full-repository adversarial review of
`v0.12.0-beta.4` found that linked worktrees could be reported as empty,
narrowed reads were still represented as absence at typed boundaries, and
positive command witnesses could mistake a broken check for missing evidence.

### Added

- A four-state read contract (`present`, `absent`, subject-specific `withheld`,
  and log-wide `indeterminate`) for direct and fenced kan reads. Every
  production consumer must decide both narrowed-visibility cases.
- Structured command witnesses with `argv` and `found_nothing_exit`, while
  retaining the released string form.
- `day config` and `day config --json`, a read-only inventory of effective
  configuration values, per-key provenance, unsupported declarations, and
  narrowed-read caveats, derived through the production loaders.

### Fixed

- A linked Git worktree no longer gets a false empty process record when the
  main checkout owns `.kan/`. day diagnoses the unsupported split and does not
  redirect kan to a different Git anchor.
- Write paths refuse both partial and wholly withheld histories; advisory hooks
  continue to render and exit successfully.
- Withheld-read guidance no longer recommends a trust flag day does not accept.
- Review verdict recording continues to use day's shipped vocabulary when a
  project override cannot be established under narrowed trust.
- Structured command signal termination and unexpected exit statuses now
  report could-not-check rather than missing evidence; invalid or duplicated
  `found_nothing_exit` declarations are refused before execution.
- Revert and mutation harnesses classify compilation with dedicated build
  results. The Windows bootstrap uses a shell-free registered Node command,
  and release verification now gates on the full design corpus, process census,
  doctor, and the live configuration telos witnesses.

## [v0.12.0-beta.4] — 2026-08-12

*Milestone: v0.12 — transportable.* A configuration release, and a round of
honest-reads fixes found by four adversarial reviews across two branches. Its
headline pattern: **every defect this cycle was a reader reporting something it
could not read as something that was not there**, and each fix round introduced
the next round's finding until the duplicate reader was deleted rather than
repaired.

### Added

- **A configuration key can be its own subject.** `schema/injection/cadence` and
  `schema/injection/max_practice_items` resolve independently, where two claims
  on the parent subject previously left only the newer one — the older field
  silently reverting to day's default. Resolution is `Default` ← a legacy
  whole-block claim ← per-key claims, so a project that adopts nothing sees
  exactly the previous behaviour. `schema/injection` and `schema/cycle` are
  routed; the map (`schema/blocks`) and list (`schema/verdicts`) shapes are not
  built, and each direct reader now states why it is not per-key.
- **kan 0.12 is measured and adopted** — `v0.12.0-beta.1` through `beta.4`, all
  `ok`.

### Fixed

- **An unterminated `day-` fence is a block day could not read, not an absence.**
  A claim that opened a fence and never closed it read as "nothing declared",
  so day resolved its own default while the project believed it had declared
  something. This reverses a decision the source stated outright and a test
  pinned; the premise ("day never writes one") was true and did not support the
  conclusion, because people and agents do.
- **An assembled configuration value is validated, not merely deserialized.**
  `CycleSchema` refuses an empty tag pattern because "the failure would look
  exactly like working" — and the per-key path reached one anyway.
- **A per-key claim carrying an unrecognised `day-` fence is refused** rather
  than read as a retracted key.
- **The compatibility cell no longer publishes facts about the wrong program.**
  It cannot say `unbuildable` at all — that is a claim about whether the kan tag
  builds, which the workflow decides before the cell runs — and a day build
  failure, a broken environment, or a hung compiler are reported as
  `could-not-run` rather than as a pairing outcome. The matrix no longer caches
  an infrastructure failure as a durable fact about a kan release.
- **The measured ceiling names the artifact that was run.** `day doctor` reports
  `0.9.1..=0.12.0-beta.4`, and an unreleased stable `0.12.0` classifies as
  `Newer` rather than `Supported`.
- **The `Demonstrated-by:` trailer carries its revert scope**, so a
  demonstration valid only under `--include` can be replayed by its own
  verifier instead of reporting could-not-check.

## [v0.12.0-beta.3] — 2026-08-10

*Milestone: v0.12 — transportable.* A bugfix and verification round produced
by a full external review of the repo (design, implementation, testing,
UX/DX/AX, process), which returned APPROVE-WITH-FOLLOW-UPS; this release is
the follow-ups that should not wait. Its headline pattern: every high finding
sat where the typed honesty architecture does not reach — a shipped shell
script, a stale measured artifact, a harness exit code, a `format!` that
flattened could-not-check into checked-and-negative.

### Added
- The standard OSS files, so day is contributable by someone who is not its
  author: `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, issue forms,
  a PR template, and this changelog.
- A declared MSRV (`rust-version = "1.88"`, set transitively by `darling`) and
  a CI job that keeps the declaration true by reading it from `Cargo.toml`
  rather than repeating it.
- crates.io metadata: description, keywords, categories, repository.
- Not-found errors teach: `day next`, `day assess telos` and `day assess atom`
  on an unknown slug now list what is declared instead of only refusing.

### Fixed
- `LICENSE` named a copyright holder that was not a legal person ([#121]).
- The bootstrap hook told a broken install to run the two unpinned
  `cargo install` commands the README documents as broken; its pins are now
  derived at runtime from `Cargo.toml` and the kan-compat table, the same
  derivations `tests/install_docs.rs` holds the README to ([#50] class).
- A `bridge check` that errored rendered in `assess telos` as "its plan could
  not reach it" — a could-not-check reported as checked-and-negative. It now
  says the check could not be made, and why ([#141]).
- "newest on `X`" in claim-probe verdicts was iteration order, not time; it is
  now the maximum `recorded_at`, and an undated match set says
  "e.g. on `X` (one of N)" rather than claiming a recency nothing computed.
- `assess telos --all` swept fully-retracted telos subjects, advising witness
  interviews for them and citing the retraction's own CID as the thing to
  record against. The sweep now folds first, excludes them, and counts the
  exclusion — so the hook and the sweep stop disagreeing about how many teloi
  exist.
- The "asserted in prose — not material evidence" note rendered under
  satisfied `[MATERIAL]` verdicts, where it read as day disputing itself —
  including on the very assessment the previous run's output said to record.
  It now attaches only to unsatisfied findings.
- The run-constant assessment coda (performed-vs-recorded, the exit-code
  reading, the single-frame caveat) prints once per `--all` sweep instead of
  once per telos; the per-telos `kan result … --cites` commands stay.
- The fenced-block scanner was prefix-matched and hand-rolled three times
  with divergent bugs: a project fence named `day-atom-ext` was misread as a
  malformed `day-atom`, a block quoted inside a four-backtick fence leaked
  into prose, and a triple-backtick inside a JSON string truncated the body.
  One line-anchored scanner (CommonMark fence rules) now serves all three
  consumers.
- A version-skewed block instance on a subject a probe's own `subject`
  predicate excludes no longer poisons that witness to `ERROR`.
- `telos declare --title` without `--kind` was silently discarded at the
  mechanism level (clap guarded one call surface); it is now refused.
- Both git fingerprints hash their file lists without a joinable-separator
  ambiguity, and both fingerprints share one FNV-1a instead of one of them
  using the std hasher whose stability the other's comment disclaims.
- `day assess docs`' version-key match is anchored to the start of a line
  with a separator, so `versions_tested = "12"` or prose containing the word
  cannot answer for the version.
- `--scope`'s help now quotes its example and states the zsh glob trap
  ([#83]), on the surface where the trap actually fires.

### Verification
- The block-compat corpus was stale by eleven releases, and regenerating it
  surfaced worse: the capture stub predated `kan show --all`, so `day-bridge`,
  `day-witness` and `day-tension` silently vanished from the capture for
  every tag since v0.8.0-beta.1. The corpus is regenerated in full; its
  expected tags derive from the migration-expectations table; a
  monotone-coverage test asserts a fence a release captured never disappears
  from a later one; the versioned-shape assertion is scoped per-tag instead
  of resting on a premise v0.10 falsified; the corpus resolves through
  `parse_block`, the production entry point, rather than raw serde; and
  `cut-release.sh` captures the release's own corpus row before tagging, the
  same no-window treatment [#118] gave the migration row.
- `mutate.py` printed its outcomes honestly and exited 0 for all of CAUGHT,
  SURVIVED, DID-NOT-COMPILE and ANCHOR-MISSING. Its exit code now carries the
  taxonomy (0/1/2/3, matching the census's shape), and its baseline compile
  check gains the clause its mutation-side twin had.
- The kan-read swallow scan's call shapes are now derived from
  `kan_client.rs`'s own method list plus the `::load(` convention, with a
  floor assertion against parser rot — three live read shapes had already
  drifted out of its hand-written vocabulary.
- CI probes that the pinned kan can run the [#120] trust-withholding
  conformance cell, so a future pin predating identity roles fails loudly
  instead of the cell silently skipping on every push.
- The release recovery instruction stays one runnable command now that the
  release commit stages two files, and the recovery test drives the new
  state.
- `KanClient::bin()` (zero callers) removed; a tautological assertion in the
  injection-boundary scan replaced with one that has teeth; a dead no-op
  line in `behaviour-diff.py` deleted; an invocation-count test pins that
  the status baseline read costs no extra kan calls (the review's claim that
  it cost ~0.5s of session start was false — the calls were memo-served —
  and the test is what keeps that true).

---

## [v0.12.0-beta.2] — 2026-08-09

*Milestone: v0.12 — transportable.* The fix rounds the milestone's cold
reviews produced, and the first release installable as a plugin.

### Added
- The plugin says what to install when its binaries are absent, and the compat
  notice reports a too-old or newer kan at session start ([#165]). With the
  companion `kan-tools/plugins` marketplace, this closed `/plugin install`
  never having worked from a URL ([#156]).
- `.design/read-visibility.md` — a design for a read that cannot be mistaken
  for an absence ([#160]).

### Changed
- `kan/` is declared an external root in day's own workspace, dogfooding
  [#84] and restoring the citation that declaring it cost.
- `docs/ROADMAP.md`: `v0.12.0-beta.1` is a cut, and it is not the v0.12
  milestone.

### Fixed
- A subject withheld by trust was reported as an undeclared subject — and the
  first fix for it keyed on a shape kan never emits ([#120]).
- A path under a declared external root is unchecked, not missing; and an
  unused external-root field must not break every earlier day ([#84]).
- An unchanged design pass recorded nothing, and a changed one now cites what
  it supersedes ([#119]). A design pass no longer cites a review's finding as
  superseded ([#158]).
- An id followed by a qualifier is named, not silently dropped ([#123]).
- A resolved section that records nothing now says so ([#135]).
- A subject in day's own namespace is not a missing file ([#136]).
- Two ways a `behaviour-diff` fixture compared nothing and reported agreement
  ([#144], [#145]).
- The behaviour harness now runs on the modes day is never in — a mechanism
  with two modes had only ever been exercised in the one this repo is in.
- `.claude/worktrees/` is gitignored, after a stray `git add -A` committed one.

## [v0.12.0-beta.1] — 2026-08-08

*Milestone: v0.12 — transportable.* Verification extended to teloi.

A witness could previously only ask *does one exist*, and over an append-only
log that question starts answering yes and never stops.

### Added
- `every` — the first witness in day that can actually fail: wherever the
  anchor holds, the requirements hold on the same subject.
- `absent` — witnessing something that is not there, with a vacuity guard, so
  a negated probe that could never fire is reported rather than counted.
- Correspondence: a record witness may require naming the material instance.
- Authorship scoping for witnesses, and witness lists that may express a
  disjunction rather than only a conjunction.
- A declare-time falsifiability check that reports a witness already satisfied,
  or structurally unable to stop matching, when it is written rather than when
  it is discovered later.
- `/handoff` and `/wakeup` — a pair of commands carrying a working thread
  across a session boundary by writing claims the other side verifies rather
  than believes.
- Vocabulary packs: a declared process, transported as data (design recorded;
  a config key becomes its own subject).
- `scripts/behaviour-diff.py`, asking the question the other two harnesses do
  not.

### Changed
- day's own process rules moved out of `CLAUDE.md` and onto the `practice`
  subject, which day injects — a rule in a file day never reads is a rule that
  depends on attention. The practice cap became declarable.
- day now dogfoods its own commands, because the plugin that ships them is not
  installed in its own repo.

### Fixed
- `day atom declare` never ran the falsifiability check `day telos declare`
  does; 6 of day's own 9 atoms could not report unmet ([#146]).
- The README install instructions, and the two ways they were rotting ([#50]).
- `run_command` collapsed every non-zero exit into `Unsatisfied` — safe then,
  a false clean the moment a probe is negated ([#137]).
- `telos/legible-process` was witnessed by three monotone probes, so it
  reported met forever ([#138]).
- `revert-demo`'s test-half boundary fired on any `#[cfg(test)]` rather than
  just a module.
- The suite depended on a clean global git config and never said so.
- An `every` verdict described a wider set than the one it quantified over.
- `behaviour-diff` reported IDENTICAL by four paths; two fixed, two filed
  ([#144], [#145]).
- `telos/v1.0` was cited as a live telos in four places and had no claims in
  kan ([#131]). Closed six hours after this tag, and shipped in it.

## [v0.11.0-beta.2] — 2026-08-04

### Fixed
- `release.yml` did not fetch what the suite needs, so `v0.11.0-beta.1` never
  published to crates.io. The census correctly reported could-not-check at a
  tag checkout with no `origin/main`, and correctly refused to call that a
  pass — the census was right and the environment was wrong.

## [v0.11.0-beta.1] — 2026-08-04

*Milestone: v0.11 — verification that can fail.* Where verification stopped
being something day asserted about itself.

### Added
- `scripts/revert-demo.py` and the `Demonstrated-by:` trailer, so a fix
  answering a review finding demonstrates that it fixes something — by being
  reverted and watching the finding's own test fail ([#116]).
- `.github/workflows/revert-demo.yml`, re-deriving every trailer on a branch.
- `scripts/demonstration-census.py`, accounting for every commit as
  demonstrated, exempt-with-a-reason, or **unaccounted**.
- `tests/documented_invocations.rs` — nothing had checked that day's documented
  invocations actually run ([#89]).
- Coverage of the fallback modes this repo is never in ([#91]).

### Fixed
- `scripts/mutate.py` did not verify the baseline was green, so a red suite
  reported every mutation as CAUGHT ([#114]).
- A `pub fn` whose only callers are its own tests now fails the build ([#101]).
- The migration row lands in the tagged tree, closing an ungated release step
  ([#118]).
- The verification job could not fail on `main`: after a merge the commit range
  is always empty, so it was permanently green for having found nothing.

## [v0.10.0-beta.2] — 2026-08-03

### Changed
- Adopted kan 0.9.2 and 0.10.0 into the measured compatibility matrix.

## [v0.10.0-beta.1] — 2026-08-03

*Milestone: v0.10 — the graph tells the truth.*

### Changed
- `next` became forward-only; feedback edges moved to `revisits`, because
  off-sequence detection assumed the atom graph was a DAG and so reported a
  skipped step through the whole build phase for every feedback edge ([#113]).
- Vocabulary subjects fold by role: current state is an aggregation over
  claims, not whatever text arrived last ([#115]).

### Fixed
- `day status` exited 2 despite documenting "always exits zero", and `day init`
  reported kan reachable when it was not ([#95]).
- The status line was the least legible thing day printed ([#108]).
- A satisfied unbounded witness reported an arbitrary instance, which read as
  "the latest" ([#112]).
- The kan-read scan knew four shapes and there were six.

## [v0.9.0-beta.1] — 2026-08-02

*Milestone: v0.9 — position honesty.*

### Added
- An artifact type may declare a material **and** a record witness.
- Recording the release claim became part of tagging rather than a step beside
  it — two consecutive releases had shipped with no claim ([#103]).

### Fixed
- `UserPromptSubmit` recomputed the position but never re-rendered the status
  line, so the bar showed session-start state all session ([#97]).
- `Standing::is_source` excluded convergent roots, so an inputless prerequisite
  atom could never be named current ([#98]).
- `design check` reported 0 acceptance criteria and "every requirement is
  referenced by an acceptance criterion" in the same run ([#105]).
- The status bar went stale whenever work was recorded in kan rather than in
  files: the position fingerprint was git-only ([#111]).
- `/adversarial-review` aborted before loading — an unguarded `ls` of four
  orientation files exits non-zero unless all exist ([#99]) — and now defaults
  to dispatching into a fresh subagent, since invoked in-session its "you did
  not write this code" premise is false ([#100]).

## [v0.8.0-beta.1] — 2026-07-31

*Milestone: v0.8 — one bulk read.*

### Changed
- Every `show` is served from one whole-log `kan show --all --json` rather than
  one call per subject — roughly 10x ([#71]). This is what sets day's kan floor
  at 0.9.1: an older kan cannot answer a claim probe at all.

### Fixed
- Detection of a subject kan lists but does not return, pushed down into
  `KanClient` rather than wired at one caller — and the subject list is taken
  *before* the bulk read, so a concurrent append by another agent no longer
  looks like a missing subject.

## [v0.7.0-beta.3] — 2026-07-30

*Milestone: v0.7.0-beta.3 — the vocabulary substrate.*

### Added
- A project can invent a fenced vocabulary that day validates under the same
  contract it holds its own to ([#74]).
- Project-declared review verdict vocabularies ([#77]) and configurable cycle
  boundaries, for cycles that are not releases ([#76]).
- Incremental design records ([#36]) and record-coverage validation against the
  `decide` claims already on a subject ([#41]).
- `tests/fixtures/kan-compat.tsv` — a measured record of which kan versions
  work with this day, printed by `day doctor` ([#94]).

### Fixed
- The kan floor was corrected to v0.7.0: a fact about **kan** had been setting
  day's floor. A cell measuring "does X work against Y" may only run assertions
  about X's own requirements.
- An unmeasured kan warns instead of failing day's release.

## [v0.7.0-beta.2] — 2026-07-30

*Milestone: v0.7.0-beta.2 — honest reads.* day stopped certifying conformance
to declarations it had silently truncated.

### Added
- Blocks refuse what they cannot account for, and carry a `_version` so the
  refusal can say **why** — *this day reads `day-atom` up to v2, this block
  declares v3* — rather than a parse error that reads as the project's mistake
  ([#78]).
- A version-migration matrix in CI, recording what every released version does
  with the block shapes the current commit writes. The answer turned out to be
  "silently widens", for every version that could read them at all.
- The session hooks say when their own lists are partial, on both the model's
  channel and the human's.

### Fixed
- A failed kan read reports as `[UNCHECKED]` rather than as an absent artifact
  ([#81]).
- The position fingerprint was dead on every repo without a release tag — the
  default path, i.e. every fresh clone. day has tags, so every check passed.
- Empty any-of / seq nodes in a bridge were silently permissive ([#20]).
- The `assessment` claim probe was broader than "an atom assessment" ([#70]).
- The documented `--scope` example did not run in zsh ([#83]).
- The block corpus could not capture `day-bridge` or `day-witness` shapes
  ([#87]).

## [v0.7.0-beta.1] — 2026-07-23

*Milestone: v0.7.0-beta.1 — current-cycle position.* day's position became
*legible* rather than merely computed.

### Added
- Position resolves each probe against a **cycle boundary** (the last release):
  a path counts if it changed since, a tag if it was created since.
- The **`claim` probe** — the first probe that reads the kan log rather than
  the working tree, so a witness like `verdict` counts if it was *recorded*
  since ([#60]).

Assessment stays cumulative, deliberately: "was this ever produced" is the
right question for a telos and the wrong one for "where is the work now".

## [v0.6.0-beta.1] — 2026-07-23

*Milestone: v0.6 — rigor as artifact.*

### Added
- Atoms carry `done` criteria, and `day assess atom` exits non-zero when one is
  unmet.
- `day status` and a status line, putting the inferred position in front of the
  human.
- The `.day/` render cache — the one carve-out from "day stores nothing of its
  own", holding nothing durable and never read to decide anything.

### Fixed
- `day design check` reports malformed requirement ids instead of dropping them
  ([#55]).
- The README install instructions ([#50]).
- Flaky stub-based git/kan tests under CI parallelism ([#64]).

## [v0.5.0-beta.1] — 2026-07-22

*Milestone: v0.5 — structured reads.*

### Added
- `day assess telos` — witnesses against material evidence, in two tiers:
  material evidence can fail the run, what the log says only prompts, and prose
  is never counted as evidence.
- Repo-defined prompt injection, projected from a kan subject ([#25]), and
  operational-safety practice injected into session context ([#30]).
- `day telos tension` emits a real `in-tension-with` edge ([#18]).
- `tests/kan_conformance.rs`, verifying day's kan argument shapes against the
  real binary ([#27]).

### Changed
- day reads kan through `--json` instead of parsing its rendered output
  ([#42]).

### Fixed
- Tension prose displaced the telos statement in injected session context
  ([#32]).
- Witness probes are per-project but witness satisfaction is often per-telos
  ([#34]).
- An unassessable telos no longer exits zero.
- A quoted `<!-- OPEN -->` marker was counted as an open question — the checker
  miscounted the marker its own template tells you to quote.
- The documented `kan result` form was corrected *before* day emitted it.

## [v0.4.0-beta.1] — 2026-07-21

*Milestone: v0.4 — the second substrate.*

### Added
- `day assess docs` — the first assessment atom ([#17]).
- git as day's second substrate, **read-only**: answering "which files changed
  since the last release" needs a commit-level view kan does not expose
  (kan-tools/kan#61). All git access lives in one module with no method that
  stages, commits, tags, checks out, or pushes.

## [v0.3.0-beta.1] — 2026-07-21

### Added
- Bridging — `day bridge declare` and `day bridge check`: plan a path from here
  to a telos as an arrangement of atoms (`a > b` sequential, `a & b`
  concurrent, `a | b` alternative), then check whether the plan could actually
  get there.

## [v0.2.0-beta.1] — 2026-07-21

### Added
- Vocabulary declaration verbs — `day telos declare`, `day atom declare`.
- CLI backing for the design and review atoms: `day design record` and
  `day review record`. This is where day stopped being read-only and began
  appending claims through kan's public CLI.
- A drafted version roadmap.

### Fixed
- Both blocking findings from the v0.2 adversarial review.

## [v0.1.2-beta.1] — 2026-07-20

### Fixed
- Show a telos's declared title, not just its newest claim.

## [v0.1.1-beta.1] — 2026-07-20

The first published release.

### Added
- The day scaffold: CLI, Claude Code plugin, and the first two process atoms.
- The release workflow and repository skeleton.

### Fixed
- Atom inputs are checked against the transitive upstream closure.

<!-- Releases -->
[Unreleased]: https://github.com/kan-tools/day/compare/v0.12.0-beta.4...HEAD
[v0.12.0-beta.4]: https://github.com/kan-tools/day/compare/v0.12.0-beta.3...v0.12.0-beta.4
[v0.12.0-beta.3]: https://github.com/kan-tools/day/compare/v0.12.0-beta.2...v0.12.0-beta.3
[v0.12.0-beta.2]: https://github.com/kan-tools/day/compare/v0.12.0-beta.1...v0.12.0-beta.2
[v0.12.0-beta.1]: https://github.com/kan-tools/day/compare/v0.11.0-beta.2...v0.12.0-beta.1
[v0.11.0-beta.2]: https://github.com/kan-tools/day/compare/v0.11.0-beta.1...v0.11.0-beta.2
[v0.11.0-beta.1]: https://github.com/kan-tools/day/compare/v0.10.0-beta.2...v0.11.0-beta.1
[v0.10.0-beta.2]: https://github.com/kan-tools/day/compare/v0.10.0-beta.1...v0.10.0-beta.2
[v0.10.0-beta.1]: https://github.com/kan-tools/day/compare/v0.9.0-beta.1...v0.10.0-beta.1
[v0.9.0-beta.1]: https://github.com/kan-tools/day/compare/v0.8.0-beta.1...v0.9.0-beta.1
[v0.8.0-beta.1]: https://github.com/kan-tools/day/compare/v0.7.0-beta.3...v0.8.0-beta.1
[v0.7.0-beta.3]: https://github.com/kan-tools/day/compare/v0.7.0-beta.2...v0.7.0-beta.3
[v0.7.0-beta.2]: https://github.com/kan-tools/day/compare/v0.7.0-beta.1...v0.7.0-beta.2
[v0.7.0-beta.1]: https://github.com/kan-tools/day/compare/v0.6.0-beta.1...v0.7.0-beta.1
[v0.6.0-beta.1]: https://github.com/kan-tools/day/compare/v0.5.0-beta.1...v0.6.0-beta.1
[v0.5.0-beta.1]: https://github.com/kan-tools/day/compare/v0.4.0-beta.1...v0.5.0-beta.1
[v0.4.0-beta.1]: https://github.com/kan-tools/day/compare/v0.3.0-beta.1...v0.4.0-beta.1
[v0.3.0-beta.1]: https://github.com/kan-tools/day/compare/v0.2.0-beta.1...v0.3.0-beta.1
[v0.2.0-beta.1]: https://github.com/kan-tools/day/compare/v0.1.2-beta.1...v0.2.0-beta.1
[v0.1.2-beta.1]: https://github.com/kan-tools/day/compare/v0.1.1-beta.1...v0.1.2-beta.1
[v0.1.1-beta.1]: https://github.com/kan-tools/day/releases/tag/v0.1.1-beta.1

<!-- Issues -->
[#17]: https://github.com/kan-tools/day/issues/17
[#18]: https://github.com/kan-tools/day/issues/18
[#20]: https://github.com/kan-tools/day/issues/20
[#25]: https://github.com/kan-tools/day/issues/25
[#27]: https://github.com/kan-tools/day/issues/27
[#30]: https://github.com/kan-tools/day/issues/30
[#32]: https://github.com/kan-tools/day/issues/32
[#34]: https://github.com/kan-tools/day/issues/34
[#36]: https://github.com/kan-tools/day/issues/36
[#41]: https://github.com/kan-tools/day/issues/41
[#42]: https://github.com/kan-tools/day/issues/42
[#50]: https://github.com/kan-tools/day/issues/50
[#55]: https://github.com/kan-tools/day/issues/55
[#60]: https://github.com/kan-tools/day/issues/60
[#64]: https://github.com/kan-tools/day/issues/64
[#70]: https://github.com/kan-tools/day/issues/70
[#71]: https://github.com/kan-tools/day/issues/71
[#74]: https://github.com/kan-tools/day/issues/74
[#76]: https://github.com/kan-tools/day/issues/76
[#77]: https://github.com/kan-tools/day/issues/77
[#78]: https://github.com/kan-tools/day/issues/78
[#81]: https://github.com/kan-tools/day/issues/81
[#83]: https://github.com/kan-tools/day/issues/83
[#84]: https://github.com/kan-tools/day/issues/84
[#87]: https://github.com/kan-tools/day/issues/87
[#89]: https://github.com/kan-tools/day/issues/89
[#91]: https://github.com/kan-tools/day/issues/91
[#94]: https://github.com/kan-tools/day/issues/94
[#95]: https://github.com/kan-tools/day/issues/95
[#97]: https://github.com/kan-tools/day/issues/97
[#98]: https://github.com/kan-tools/day/issues/98
[#99]: https://github.com/kan-tools/day/issues/99
[#100]: https://github.com/kan-tools/day/issues/100
[#101]: https://github.com/kan-tools/day/issues/101
[#103]: https://github.com/kan-tools/day/issues/103
[#105]: https://github.com/kan-tools/day/issues/105
[#108]: https://github.com/kan-tools/day/issues/108
[#111]: https://github.com/kan-tools/day/issues/111
[#112]: https://github.com/kan-tools/day/issues/112
[#113]: https://github.com/kan-tools/day/issues/113
[#114]: https://github.com/kan-tools/day/issues/114
[#115]: https://github.com/kan-tools/day/issues/115
[#116]: https://github.com/kan-tools/day/issues/116
[#118]: https://github.com/kan-tools/day/issues/118
[#119]: https://github.com/kan-tools/day/issues/119
[#120]: https://github.com/kan-tools/day/issues/120
[#121]: https://github.com/kan-tools/day/issues/121
[#123]: https://github.com/kan-tools/day/issues/123
[#131]: https://github.com/kan-tools/day/issues/131
[#135]: https://github.com/kan-tools/day/issues/135
[#136]: https://github.com/kan-tools/day/issues/136
[#137]: https://github.com/kan-tools/day/issues/137
[#138]: https://github.com/kan-tools/day/issues/138
[#141]: https://github.com/kan-tools/day/issues/141
[#144]: https://github.com/kan-tools/day/issues/144
[#145]: https://github.com/kan-tools/day/issues/145
[#146]: https://github.com/kan-tools/day/issues/146
[#156]: https://github.com/kan-tools/day/issues/156
[#158]: https://github.com/kan-tools/day/issues/158
[#160]: https://github.com/kan-tools/day/issues/160
[#165]: https://github.com/kan-tools/day/issues/165
