# Changelog

All notable changes to `day`, newest first.

**Every release so far is a prerelease.** There is no stable line, the
conventions are v0, and `cargo install day` will not select a version without
an explicit `--version`. Because all 17 releases are marked as prereleases on
GitHub, `/releases/latest` returns 404 — GitHub excludes prereleases from it.
Nothing depends on that endpoint; the release workflow triggers on tag push.

**These entries were reconstructed after the fact**, from the tags, the commit
history, and the issues closed in each release window. They were written when
the repository had 17 tags and no GitHub Releases at all. Where a release's
own commits name an issue, that issue is cited; where they do not, the entry
describes what the commits did. Milestone names are the ones the repository
actually used, because they say what each release was *about* better than a
version number does.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
versioning is [SemVer](https://semver.org/spec/v2.0.0.html), pre-1.0.

## [Unreleased]

Work merged since `v0.12.0-beta.1`.

### Fixed
- `/plugin install kan-tools/day` failed: the repo shipped a plugin manifest
  but no marketplace manifest ([#156]).
- `telos/v1.0` was cited as a live telos in four places and had no claims in
  kan ([#131]).

---

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
[Unreleased]: https://github.com/kan-tools/day/compare/v0.12.0-beta.1...HEAD
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
[#131]: https://github.com/kan-tools/day/issues/131
[#137]: https://github.com/kan-tools/day/issues/137
[#138]: https://github.com/kan-tools/day/issues/138
[#144]: https://github.com/kan-tools/day/issues/144
[#145]: https://github.com/kan-tools/day/issues/145
[#146]: https://github.com/kan-tools/day/issues/146
[#156]: https://github.com/kan-tools/day/issues/156
