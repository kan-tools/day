# Feature: Current-cycle position — a cycle boundary and a claim-shaped probe

## Summary

Position inference asks "does an artifact of this type exist" and, on a repo
with history, the answer is always yes — every `v*` tag, every past `verdict`
and `assessment` exists from prior cycles, so `day status` reports four
candidate atoms on day's own log and could never narrow. This gives position a
notion of the **current cycle**, bounded by the last release, and resolves each
probe *relative to that boundary*: a path is present if it **changed since** the
boundary, a tag if it was **created since**, and — via a new `claim` probe kind
that reads the log rather than the working tree — a claim-shaped witness like
`verdict` or `assessment` is present if one was **recorded since** the boundary.
The change is confined to inference; assessment stays cumulative, because
"was this ever produced" is the right question for a telos and the wrong one for
"where is the work now."

## Motivation

The `release` Result claim already named the gap: *"a witness type whose
evidence is 'a claim exists' cannot be probed by path, tag, or command, because
all three read the world and this one reads the log."* And day#60 found the
deeper half: a naive claim-probe would report a *stale* verdict as present,
because project-level presence cannot tell this cycle's review from one three
milestones ago. The cycle boundary is what separates them, and day already
computes one — `src/docs.rs::reconcile_boundary` and `git.changed_files` scope
docs assessment to "since the last release." Position needs the same lens.

## Requirements

- REQ-1: Position inference resolves an artifact type relative to a **cycle
  boundary** — the last release — rather than over all of history. A `path`
  witness is present when a file matching its pathspec is among those changed
  since the boundary; a `tag` witness when a matching tag was created since it.
- REQ-2: A new **`claim` probe kind** resolves a witness by reading the kan log:
  it is present when a live claim matching a declared shape (a `ClaimKind`, and
  optionally a text marker) exists. Because it is a read of day's own substrate,
  not execution of project-declared code, inference may run it — unlike a
  `command` probe.
- REQ-3: For inference, a `claim` witness is present only when a matching claim
  was **recorded since the boundary** (`recorded_at` greater than the boundary's
  time), so a stale verdict or assessment from a prior cycle does not read as
  the current one.
- REQ-4: The cycle boundary is derived from git — the last `v*` tag — and is
  **`Option`al**: a repo with no release has no boundary, and inference falls
  back to today's cumulative, tracked-ever behaviour rather than treating the
  whole history as the current cycle.
- REQ-5: Cycle-relativity is **confined to position inference** — `day status`,
  the status line, and the session hooks. `day assess telos` and `day assess
  atom` are unchanged: they ask whether a witness was *ever* produced, and a
  release or assessment from any time is real evidence for a telos.
- REQ-6: Inference still runs **no `command` probe**. The read/execute line
  REQ-5 of `rigor-as-artifact` drew is preserved and restated: `path`, `tag`,
  and the new `claim` are all reads and may run during inference; `command` is
  execution and may not. A command-witness stays `Unknown` at inference time.
- REQ-7: The `claim` probe is available to the witness schema the same way the
  other kinds are — declared on `schema/witness`, serialized in the same
  `day-witness` block, additive so every existing schema still parses. day's own
  `schema/witness` gains `verdict` and `assessment` claim-probes so this repo's
  position actually narrows.
- REQ-8: The `claim` probe is **never an execution path**. It performs only kan
  read verbs (`show`/`status`), it is reachable over MCP (it changes nothing and
  runs nothing), and it has no `--run` gate because there is nothing to gate —
  distinguishing it in `docs/CONVENTIONS.md` from the `command` probe, which has
  all three constraints.

## Acceptance Criteria

- [ ] AC-1: With `code-change` a `path` witness and a boundary set, a source
      file tracked but unchanged since the boundary reads as **absent** for
      inference, while the same file changed since the boundary reads as
      present. (REQ-1)
- [ ] AC-2: A `claim` probe for kind `Result` reports present when a `Result`
      claim exists and absent when none does; a `claim` probe naming a text
      marker matches only claims whose text contains it. (REQ-2)
- [ ] AC-3: A `Result` claim recorded before the boundary does not satisfy an
      inference `claim` witness, and one recorded after it does. (REQ-3)
- [ ] AC-4: With no `v*` tag, inference produces the same result it does today
      (tracked-ever, cumulative), and names no boundary. (REQ-4)
- [ ] AC-5: `day assess telos` and `day assess atom` return byte-identical
      output with and without a boundary present — assessment never becomes
      cycle-relative. (REQ-5)
- [ ] AC-6: Inference over a schema whose `verdict` witness is a `command` probe
      executes nothing and reports `verdict` unknowable, exactly as a
      command-probed input does today. (REQ-6)
- [ ] AC-7: A `schema/witness` block written before this feature parses and
      resolves unchanged; a block adding a `claim` probe round-trips through its
      serialized form. (REQ-7)
- [ ] AC-8: `src/mcp.rs`'s `session_context` resolves `claim` probes (they are
      reads) while a `command` probe there still reports not-run — asserted by a
      claim witness reporting present over the MCP path. (REQ-8)

## Architecture

**`src/probe.rs`** gains a `Probe::Claim { kind: String, contains: Option<String> }`
variant beside `Path`, `Tag`, and `Command`. Its evaluation reads the log, so —
unlike the others — it needs a `KanClient`, which is the one real ripple: the
witness-resolution path grows a kan handle. Crucially it is **not** routed
through the command guardrails; the module doc's five rules are about spawning a
process, and a `claim` probe spawns nothing. A test asserts `Probe::Claim` never
reaches `run_command`.

**`src/position.rs`** already separates the pure core (`infer_with`, from
day#64) from the git-backed resolver. That resolver grows: `infer` becomes
`infer(atoms, probes, git, client, boundary)`, and the presence closure it
builds resolves `path`/`tag` against `boundary` (using `git.changed_files` and a
since-filtered `tags_matching`) and `claim` against `client` filtered by
`recorded_at`. The pure `infer_with` is untouched, so its deterministic tests
stand. `materialized` splits into a cumulative form (still used nowhere in
inference) and the cycle-relative resolver.

**`src/git.rs`** already has `changed_files(since)` and `latest_version_tag()`;
the boundary is the latter, and no new git verb is required. Tags-since reuses
`tags_matching` plus a creation-date filter against the boundary tag.

**`src/kan_client.rs`** already exposes `Claim { kind, text, recorded_at }` and
`show`/`subjects`, which is everything the `claim` probe reads. The probe
enumerates subjects and scans their live claims — the same broad read
`atoms::load` and `last_assessed_atom` already do, noted as a cost paid at
session-start where there is time.

**`src/status.rs`** computes the boundary once (via `git.latest_version_tag()`)
and threads it and the client into `infer`. **`src/telos.rs`** (assessment) is
deliberately not touched: it keeps calling `probe::evaluate`, the cumulative
path, so REQ-5 holds by construction rather than by a flag.

**`docs/CONVENTIONS.md`** documents the `claim` probe, the cycle boundary, and
the inference-only scope, and states plainly that `claim` is a read with none of
`command`'s three constraints.

**Nothing here writes a claim or destroys a subject.** The `claim` probe only
reads, and the boundary is derived from git each time — no new stored state, the
`.day/` cache included, which continues to hold only rendered display.

## Resolved Questions

- **Cycle-relativity is inference-only; assessment stays cumulative.** A telos
  asks whether work ever landed in its equivalence class, and a release from any
  time is evidence; position asks where *this* cycle sits. Conflating them would
  make `assess telos` start failing on last cycle's shipped telos, which is
  wrong. `src/telos.rs` keeps the cumulative `probe::evaluate` path untouched.
- **The `claim` probe is in scope, not a later design.** It is the half of day#60
  that actually resolves the claim-shaped witnesses, and it is *safer* than the
  `command` probe already shipped: a kan read, no shell, no execution, nothing to
  gate, reachable over MCP. Doing the path/tag scoping without it would leave
  day's own ambiguity only half-resolved.
- **No release yet means no boundary, and inference falls back to today's
  behaviour.** Treating an unbounded repo as "everything is the current cycle"
  would make a fresh repo report every atom current; falling back to tracked-ever
  is the conservative, already-correct default.
- **A verdict is identified by kind plus a text marker, an assessment by kind
  alone.** `record::review` writes a verdict as a `decide` (kind `Decision`)
  whose text is `"adversarial review of <subject>: <VERDICT> — …"`; `Decision`
  alone is far too broad, so the `verdict` probe is `{claim: {kind: Decision,
  contains: "adversarial review of"}}`. An assessment is a `Result` claim, which
  is specific enough on its own: `{claim: {kind: Result}}`. The text match
  inherits day#34's "narrowing" precedent — it tightens which claims count, it
  does not invent a new match language.
- **The boundary is the last `v*` tag, not the reconciled release boundary
  `docs.rs` uses.** Reconciliation (tag vs the `release` subject) exists to catch
  a release recorded-but-not-tagged, which is a docs-assessment concern; for
  position, the last tag is the cycle marker and needs no claim read to compute.

## Out of Scope

- **`merged-change` as a witness.** A pull-request's output is a merge to the
  default branch — git-shaped, but neither a tracked file, a tag, nor a claim, so
  it would need a fourth, git-log-based probe kind. Left unprobed; it stays
  `Unknown`, which the off-sequence rule already tolerates.
- **A "current work subject" model.** Scoping by *time since the boundary* is
  enough to separate this cycle from prior ones; a per-work-item subject notion
  is a heavier model this does not need and does not add.
- **Making the boundary configurable** (a branch base, an explicit ref). The
  last release is the one cycle marker that matters here; a flag can follow if a
  real need appears.
- **Changing `assess telos`/`assess atom`.** By REQ-5 they are cumulative and
  untouched.
