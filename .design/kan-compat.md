# Feature: kan/day version compatibility — measured, reported, never enforced

## Summary

day and kan are separately released crates with a versioned contract between
them, and nothing recorded which pairings work. `day doctor` reported
`kan: reachable`, and reachable is not compatible: a user running day against a
kan that predates a read day depends on found out by getting a wrong answer.

Worse, the one test that could have known was inert. `tests/kan_conformance.rs`
exists because "a stub accepts whatever day sends it, so those tests validate
day against day's own idea of kan's CLI rather than against kan's contract" —
the blind spot that shipped a documented `kan result` invocation that does not
run, through several releases (day#27, kan#78). It skips when kan is absent, and
`ci.yml` never installed kan. **It had never run in CI.**

This measures every released kan against this day, commits the result, derives
the range day reports from that table, and says something to the user when the
pairing is outside it.

**Written after the implementation rather than before it**, contrary to
`CLAUDE.md`'s working practice. Recorded that way rather than backdated: the ask
arrived mid-milestone and the first thing built was the measurement harness,
because the design question ("what range does day support?") is not answerable
from the armchair. The requirements below are what the implementation must
satisfy, and the adversarial review runs against them the same as any other.

## Requirements

- REQ-1: `ci.yml` **installs kan**, so `tests/kan_conformance.rs` stops
  skipping. The kan is **pinned to a tag**, not tracking kan's default branch —
  CI must go red because day changed, and an unpinned dependency turns an
  unrelated kan commit into a failure here.
- REQ-2: A **matrix measures every released kan** against this day commit, one
  cell per kan tag, running the conformance suite. Cells are cached on
  `(kan tag, day's read surface, toolchain)` — all three immutable inputs to a
  deterministic outcome, so a hit means the answer cannot have changed. This is
  `migration-matrix.yml`'s design on the other axis: that one varies *day* and
  holds the block shapes fixed; this varies *kan* and holds day fixed.
- REQ-3: Outcomes are compared against a **committed expectation** in
  `tests/fixtures/kan-compat.tsv`. A pairing with no row fails the build,
  because whether a pairing is supported is a decision rather than a default —
  the rule `migration-expectations.tsv` already holds.
- REQ-4: day **reads kan's version** (`kan --version`) and classifies it. An
  unreadable version is `Unknown`, **never a mismatch**: claiming
  incompatibility from a failed read would break day against any kan whose
  output format shifts, which is a self-inflicted outage rather than a warning.
- REQ-5: **The two skew directions are distinguished.** kan older than day
  supports is a real problem the user fixes by upgrading kan. kan newer than day
  was measured against is normally fine, because kan's read surface is
  additive-only, and must not be as loud — a warning that fires on every kan
  release is a warning nobody reads. This is the same distinction `_version`
  skew already draws for blocks ("upgrade day" vs "the claim needs fixing").
- REQ-6: **The reported range is derived from the measurements, not asserted
  ahead of them.** A CI job fails if `OLDEST_SUPPORTED` / `NEWEST_MEASURED` in
  `src/compat.rs` disagree with the `ok` rows in the table. Without it the
  constant and the measurements drift apart silently and the range day prints
  to users stops meaning anything. Writing the bound first and calling it a
  requirement is the nominal-requirement failure that blocked
  `.design/declared-blocks.md`'s first implementation — the same mistake, one
  milestone later, would be inexcusable.
- REQ-7: **Advisory, never blocking.** A mismatch is reported text. day does not
  refuse to run, and the pairing deliberately does **not** enter
  `doctor::Report::is_healthy`, which is documented as the composition check and
  may already be scripted. A day that refused to run against an unrecognised kan
  would break on every kan release that outpaces it.
- REQ-8: **No store.** The supported range is a compiled-in constant — code,
  exactly like `Versioned::SUPPORTED_VERSION` — and day asks kan its version
  rather than remembering it. Nothing about the pairing is written to disk.
  "day bookkeeps versions" reads like a store and must not become one.

## Acceptance Criteria

- [ ] AC-1: With kan installed, `cargo test --test kan_conformance` runs four
      tests that previously skipped; CI asserts `kan --version` succeeds rather
      than trusting the install step's exit code, because a silent skip is the
      failure this exists to end. (REQ-1)
- [ ] AC-2: The matrix enumerates kan's tags from kan's repo rather than a
      hardcoded list, so a kan release appears without anyone remembering to add
      it; a second run with no change to day's read surface is a full cache hit.
      (REQ-2)
- [ ] AC-3: A kan tag absent from `kan-compat.tsv` fails with a message naming
      the file and what to do; an outcome differing from its row fails naming
      both values. (REQ-3)
- [ ] AC-4: `Version::parse` accepts what kan actually prints
      (`kan 0.8.0-beta.1`) and returns `None` — classified `Unknown`, not
      `TooOld` — for empty output, a missing version, and two- or
      four-component strings. Asserted on the real binary's output shape, not an
      invented one. (REQ-4)
- [ ] AC-5: A kan below the range renders "OLDER than this day supports" and
      "Upgrade kan"; one above renders "newer than this day was measured
      against" and says it is normally fine. A pre-release of a supported
      version (`0.8.0-beta.1`) is **supported** — every kan release so far is a
      `-beta.N`, so a rule excluding them would reject the whole of kan.
      **Negative control:** a supported pairing is not notable and prints no
      warning. (REQ-5)
- [ ] AC-6: Moving `OLDEST_SUPPORTED` away from the oldest `ok` row in the table
      fails the `range-matches-measurements` job. (REQ-6)
- [ ] AC-7: `day doctor` against an unsupported kan still exits zero and still
      performs every read. `tests/plugin.rs`'s advisory scan is unchanged and
      still passes. (REQ-7)
- [ ] AC-8: No new file is written under `.day/` or anywhere else by the version
      check; the existing source scan proving only `src/cache.rs` touches
      `.day/` still passes. (REQ-8)

## Architecture

**`src/compat.rs` holds the range and the verdict; `kan_client::version()` does
the read.** The split matters: `version()` returns `Option<Version>` and never
errors, for the same reason `identity()` returns `Option<String>` — a caller
deciding whether to warn needs a value it can branch on, not an error that
aborts. `classify` then maps `None` to `Unknown`, which is what keeps a failed
read from becoming a false mismatch.

**`release_order` is a named method, deliberately not `Ord`.** Semver says a
pre-release precedes its release; day has no reason to encode that subtlety for
a range check, and if the comparison were an operator a future caller would
reach for `<` and silently get a rule nobody chose.

**The cell script selects its kan by prepending to `PATH`.**
`kan_conformance::real_kan()` resolves `kan` through `PATH` and deliberately
ignores `DAY_KAN_BIN`, because a stub is exactly what it must not talk to. A
shim directory containing only that one binary is therefore the mechanism, and
`DAY_KAN_BIN` is unset in the cell so a stray value cannot redirect the suite.

**`unbuildable` is a distinct outcome from `incompatible`.** An old kan tag may
not build with a current toolchain, and a version nobody can build is a version
nobody is running — recording that as an incompatibility would be a claim about
the pairing that the run never tested. The script separates them by checking for
a compile failure rather than assuming a non-zero exit means a contract break.

**No new substrate.** `kan --version` is one more read verb through the existing
`KanClient::run`; `src/probe.rs`'s `run_command` remains day's only spawn site.

## Resolved Questions

- **Offer a warning, not a refusal (REQ-7).** A hard failure on an unrecognised
  kan is the enforcement posture day exists to avoid, and it fails in the
  direction that hurts most: kan will ship a version day predates on every kan
  release, so the common case for "unrecognised" is *benign*.
- **The range is measured, not declared.** The alternative — decide day requires
  kan ≥ X from reading the read-surface contract — was rejected because
  `.design/kan-read-contract.md` describes what day *asked kan for*, and day
  does not yet consume kan v0.8's trust surface at all. The contract doc would
  have produced a minimum of v0.8, which is wrong by several releases. Only
  running the suite knows.
- **A conformance cell may only run tests of day's own dependencies.** The first
  measurement put the floor at v0.7.1 and was wrong, because
  `conformance_the_documented_kan_result_form_runs` carried two assertions: that
  the positional `kan result` form runs — which day depends on — and that
  `result --subject` also runs, which asserts **kan#78 was resolved** and is a
  property of *kan*. day emits only the positional form, so a kan predating
  kan#78 serves it fine. One test mixing "what we depend on" with "what they
  promised" moved a user-visible floor by a release, in the direction that turns
  working setups away. The assertion is now its own test, named for what it
  actually checks, and the cell skips it — while normal CI still runs it against
  the pinned kan, where it does its real job of catching a revocation.
- **The floor is hard below v0.7.0, and that is worth stating so nobody tries to
  lower it.** kan v0.6.0 and earlier have no `kan show --json`, and day reads the
  structured form for everything precisely because it parsed the rendered form
  once, kan changed it, and day read a full log as empty while reporting success.
  They also lack `in-tension-with` (kan#60). A fallback to rendered output would
  not widen support; it would reintroduce the defect day migrated away from.
- **`is_healthy` is left alone (REQ-7).** Folding the pairing in would change
  the documented meaning of `day doctor`'s exit code, and the usual reason it
  would fire is `Newer`, which is not a fault.
- **The matrix reads kan's tags from kan's repo.** A hardcoded list is a second
  place to remember, and the thing it would be a copy of is authoritative and
  one `git ls-remote` away.

## Out of Scope

- **Surfacing the pairing on the hook channels.** `doctor` is where a user asks
  after correctness; putting a version warning into every session's injected
  context is a second, louder channel that should be a decision of its own once
  there is evidence anyone hit a mismatch.
- **A day-side view in kan.** Which kan versions work with which day is a fact
  about the pair, and day is the consumer that can act on it. If kan wants the
  reverse view that is a kan issue citing day#94.
- **Per-feature capability detection** — asking kan what it supports rather than
  inferring from a version. A larger design, and it needs kan to expose such a
  surface; the version is what exists today.
