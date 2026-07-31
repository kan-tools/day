# Feature: one read of the log, not one per subject (day#71)

## Summary

A `claim` witness is not tied to a subject, so answering one means reading the
whole log. day does that with `kan status --json` plus a `kan show` per subject,
and the cost is paid at session start.

Measured on day's own 52-subject log, with a wrapper counting every invocation:

| | wall | kan calls |
|---|---|---|
| `day hook session-start` | 4.75s | 98 |
| `day hook session-notice` | 2.95s | 69 |
| **a session start, both hooks** | **7.70s** | **167** |

Per-call cost is ~48ms and is almost entirely **fixed process startup** — kan#123
established that an empty log costs the same as a full one, that a one-claim
subject costs the same as the largest, and that `kan identity did`, which reads
no log at all, costs the same again. So no optimisation inside a read helps and
**only the invocation count matters.**

kan#123 shipped the fix as `kan show --all --json` (ADR-71) in kan **v0.9.1**-beta.1 — not v0.9.0, whose `--all` is on `publish`.
This adopts it: `ClaimLog` makes one call instead of N+1.

## Requirements

- REQ-1: `ClaimLog` loads the whole log in **one** `kan show --all --json`
  invocation. It is the single site that reads every subject, so this is the
  single place that changes.
- REQ-2: **day requires kan ≥ 0.9.1**, and says so rather than failing obscurely.
  `OLDEST_SUPPORTED` moves, `tests/fixtures/kan-compat.tsv` records
  v0.7.0/v0.7.1/v0.8.0 as `incompatible`, and `day doctor` renders the existing
  `TooOld` message, which already says "Upgrade kan".

  The floor moves because a **fallback would be a two-mode mechanism**, and this
  repo's own record says that is where defects hide: `CLAUDE.md` names it, day#91
  is open about exactly this ("fallback modes are untested because this repo is
  never in them"), and the `v0.7.0-beta.3` milestone hit it twice — once in the
  position fingerprint, once in the read-failure collector written to fix a
  review finding. An untested fallback is worse than an honest floor. kan is
  pre-1.0 and shipped v0.7.0 through v0.9.1 inside a single day, so "run a recent
  kan" is a fair ask, and the whole point of this change is to **delete** the
  slow path rather than keep it alive untested.
- REQ-3: **The compatibility table must fail the pairings that now break.** The
  matrix runs `tests/kan_conformance.rs`, which today does not touch
  `--all`, so without a new assertion the table would keep reporting v0.8.0 as
  `ok` while day is in fact broken against it. A table that says `ok` for a kan
  day cannot use is precisely the false-completeness `telos/honest-reads`
  forbids, in the artifact this milestone built to prevent it.
- REQ-4: A kan that rejects `--all` is an **error naming the cause**, never a
  silently empty log. `src/probe.rs` already states that a subject day cannot
  read is an error and never a silently empty result, and
  `a_failed_kan_read_is_never_swallowed` in `tests/plugin.rs` enforces it.
- REQ-5: **`ClaimLog` stays stack-local and lazy.** day#71 says so explicitly:
  it lives for one invocation and dies with it, so `telos/no-store-of-its-own` is
  untouched. Laziness matters more after this change, not less — a command that
  needs no claim probe must still pay nothing.
- REQ-6: The bulk envelope is parsed with the **existing `Claim` parser**. ADR-71
  made each entry a full `ShowJson`, repeated `trust` field and all, specifically
  so day would not write a second parser; taking that deal is the point.
  `SHAPE_VERSION` is checked as it is for `show`.

## Acceptance Criteria

- [ ] AC-1: With a counting wrapper, a command makes **exactly one**
      `show --all` call **however many claim probes are declared** — one read,
      shared by every probe, which is what REQ-1 actually claims. Asserted as an
      invocation count, not a duration: `v0.7.0-beta.2` established that a
      timing assertion measures the machine and flakes while an invocation count
      measures the design. (REQ-1)

      **This AC originally read "and zero per-subject `show` calls", which
      contradicted this document's own Architecture section** — the specialised
      loaders (`atoms::load`, `telos.rs`, `tension.rs`) are explicitly *not*
      routed through `ClaimLog` here, so per-subject reads remain by design.
      Written that way the AC would have been unmeetable without silently
      widening the change. The per-subject reads that remain are pinned as a
      characterization instead, so their number cannot drift unnoticed.
- [ ] AC-2: `ClaimLog` returns the same `(subject, claim)` set through the bulk
      read as the per-subject loop returned, compared **CID for CID** over a log
      containing several subjects and kinds. A faster path that returns a
      different answer is a different answer wearing the same name. (REQ-1, REQ-6)
- [ ] AC-3: Against a kan that rejects `--all`, day reports an error naming the
      version requirement, and **no** command reports an empty or partial log as
      a complete one. **Negative control:** the same fixture against a kan that
      accepts `--all` succeeds. (REQ-2, REQ-4)
- [ ] AC-4: `tests/kan_conformance.rs` exercises the bulk read against the real
      binary, so the compat matrix's six pre-0.9.0 cells turn `incompatible` on
      their own rather than because a human edited the table. Re-measured, not
      hand-written. (REQ-3)
- [ ] AC-5: `day doctor` against kan 0.8.0 renders `OLDER than this day supports`
      and `Upgrade kan`; `OLDEST_SUPPORTED` and the table agree, enforced by the
      existing `tests/kan_compat.rs`. (REQ-2)
- [ ] AC-6: A command declaring no claim probe makes **no** `show --all` call at
      all — laziness preserved. `day doctor` in particular must not regress from
      its current 10 invocations to a whole-log read. (REQ-5)

## Architecture

**One site changes.** `ClaimLog::claims()` in `src/probe.rs` is a `get_or_init`
containing the `subjects()` + N × `show()` loop. It becomes one `show_all()`.
Everything downstream — `claims_matching`, position inference, the block
predicate — consumes the same `&[(String, Claim)]` and is untouched.

**What this does *not* fix, deliberately.** Profiling found ~45% of
`session-start`'s calls are duplicate reads *within* one command: `atom/*` read
4×, `telos/*` and `tension/*` 2×, `kan status --json` 5×, because eight
independent read sites each run their own loop instead of sharing `ClaimLog`.
This change does not route them through it. It does not need to: once the whole
log costs one call, reading it once and serving everything from memory beats any
targeted strategy, so the remaining duplication is worth ~7 calls rather than
~45. Routing the specialised loaders through `ClaimLog` naively would also
*regress* `day doctor`, which reads only `atom/*` today (10 calls) and would be
forced into a whole-log read. Separate change, separate design.

**Also not fixed here:** `hooks.json` registers two commands on `SessionStart`
and each reads the log in its own process, so a session start pays for two full
reads. Merging them is orthogonal to this and trades a stated safety property
(the hooks were split so the notice could not regress the model-context
injection) for speed. Its own decision.

**`show --all` requires `--json`**, which suits day: `kan_client` reads the
structured form everywhere because it parsed the rendered form once, kan changed
it, and day read a full log as empty while reporting success.

## Resolved Questions

- **Floor moves rather than falling back** — see REQ-2. The deciding argument is
  not ergonomics but testability: this repo has a documented, repeated failure
  mode for two-mode mechanisms, and the alternative was to add a third instance
  of it in the same week it was recorded twice.
- **The conformance suite must grow an assertion (REQ-3).** Without it the compat
  matrix keeps reporting `ok` for kans day can no longer use, and the table
  becomes a specific false claim about another program. This is also the lesson
  from `v0.7.0-beta.3`: a cell measuring "does day work against this kan" is only
  worth its answer if it asserts the things day actually depends on.
- **The remaining intra-command duplication is left alone.** It is real and
  measured, but it is ~7 calls after this lands, and fixing it properly means
  deciding what `ClaimLog` should do for commands that want one subject.

## Out of Scope

- **Merging the two `SessionStart` hooks** — orthogonal, and a safety trade.
- **Routing the specialised loaders through `ClaimLog`** — regresses `doctor`
  without a subset-aware design.
- **Filtering the bulk read by kind.** day#71 floats it; the whole log in one
  call is already the win, and a filter is a second contract to agree with kan.
