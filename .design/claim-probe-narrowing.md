# Feature: Claim-probe narrowing — subject scope and anchored text match

## Summary

The `claim` probe (v0.7, `src/probe.rs`) matches a witness against the kan log by
`kind` and an optional `contains` substring. On a real log that is too coarse in
two ways day#70 found by dogfooding: `assessment` (`{kind: Result}`) matches
release notes and session handoffs, not just atom assessments; and `verdict`
(`{kind: Decision, contains: "adversarial review of"}`) matches the very decision
that *defined* that marker, because `contains` is substring-anywhere. This
generalizes `ClaimShape`'s narrowing with two additive, independently-optional
predicates — a glob-lite `subject` scope and an anchored `starts_with` text match
— and restructures `matches()` as a conjunction so a future author/enrichment
dimension (kan#117) is additive rather than a re-litigation of the probe language.

## Requirements

- REQ-1: `ClaimShape` (`src/probe.rs`) gains an optional `subject` predicate that
  scopes which subjects a matching claim may live on, with **glob-lite**
  semantics: a value ending in `*` is a prefix match on the part before it
  (`"atom/*"` ≡ subject starts with `atom/`; bare `"*"` ≡ any subject), and a
  value without `*` is an exact match (`"release"` ≡ only the `release` subject).
  It is matched in-process against the `(subject, claim)` pairs `ClaimLog` already
  holds, so it adds no kan read.
- REQ-2: `ClaimShape` gains an optional `starts_with` predicate: an **anchored**
  prefix the claim's text must begin with, distinct from `contains`
  (substring-anywhere). This is the mechanism that separates a real verdict
  (`"adversarial review of <subject>: <VERDICT> …"`, marker at the start) from a
  claim that merely quotes the marker mid-text.
- REQ-3: `ClaimShape::matches` becomes a **conjunction of independent predicates**
  — `kind`, and each of `contains`/`starts_with`/`subject` that is present — so a
  later narrowing dimension (author/enrichment, deferred to kan#117) is added as
  one more optional field and one more conjunct, without changing how the existing
  predicates are expressed or evaluated.
- REQ-4: The additions are **additive and backward compatible**: a `schema/witness`
  block written before this feature (`{kind}` or `{kind, contains}`) parses and
  resolves unchanged, and every field round-trips through its serialized form with
  `contains`/`starts_with`/`subject` each omitted when absent.
- REQ-5: day's own `schema/witness` starter (`WitnessSchema::starter` in
  `src/telos.rs`) is updated so `assessment` is `{kind: Result, subject: "atom/*"}`
  and `verdict` is `{kind: Decision, starts_with: "adversarial review of"}`,
  fixing the two false positives day#70 recorded on day's own log.
- REQ-6: The narrowing is applied at the single evaluation site
  (`probe::claims_matching` in `src/probe.rs`), so it is honored **identically**
  by the cumulative assessment path (`probe::evaluate` → `claims_matching`,
  `src/telos.rs`) and the cycle-relative inference path (`position::resolve` →
  `claims_matching`, `src/position.rs`). Neither path gets its own copy of the
  matching rule.
- REQ-7: `docs/CONVENTIONS.md` documents `subject` and `starts_with` in the claim
  probe section, and its existing day#70 caution (that `assessment` is broader
  than an atom assessment, and the `contains`-quoting note) is replaced with the
  resolved behavior.

## Acceptance Criteria

- [ ] AC-1: A `claim` probe with `subject: "atom/*"` counts a `Result` on
      `atom/design` and does **not** count one on `release` or `spine`; a probe
      with `subject: "release"` (no `*`) matches only the `release` subject.
      (REQ-1)
- [ ] AC-2: A probe with `starts_with: "adversarial review of"` matches a claim
      whose text begins with that phrase and does **not** match one that only
      contains it mid-text (the marker-defining decision), while the same value
      as `contains` matches both — proving anchored and substring differ. (REQ-2)
- [ ] AC-3: `matches` returns present only when every present predicate holds;
      a shape carrying `kind` + `subject` + `starts_with` together requires all
      three, and omitting a predicate imposes no constraint from it. (REQ-3)
- [ ] AC-4: A pre-feature block `{"kind":"Result"}` and
      `{"kind":"Decision","contains":"x"}` parse and resolve unchanged; a block
      adding `subject`/`starts_with` round-trips through `serde` with absent
      fields omitted. (REQ-4)
- [ ] AC-5: On a fixture modeling day's own log (a `Result` on `atom/build`, a
      `Result` on `release` and on `spine`, the marker-defining `Decision` on
      `current-cycle-position`, and a real verdict `Decision`), day's updated
      starter resolves `assessment` present only from the `atom/*` Result and
      `verdict` present only from the anchored real verdict — both false
      positives gone. (REQ-5)
- [ ] AC-6: The same claim witness yields identical present/absent decisions
      whether reached through `assess telos` (cumulative) or `day status`
      (cycle-relative), asserted for a `subject`- and a `starts_with`-narrowed
      probe. (REQ-6)
- [ ] AC-7: `docs/CONVENTIONS.md` contains the `subject` and `starts_with`
      documentation and no longer carries the day#70 caution text, asserted by a
      `tests/plugin.rs` doc check. (REQ-7)

## Architecture

**`src/probe.rs`** is the whole of the mechanism. `ClaimShape` gains two fields
beside `kind` and `contains`, each `#[serde(default, skip_serializing_if =
"Option::is_none")]` so the serialized form stays minimal and old blocks parse:

```rust
pub struct ClaimShape {
    pub kind: String,
    pub contains: Option<String>,     // substring anywhere (unchanged)
    pub starts_with: Option<String>,  // NEW: anchored prefix of the text
    pub subject: Option<String>,      // NEW: glob-lite subject scope
}
```

`matches` changes signature from `matches(&self, claim)` to
`matches(&self, subject: &str, claim)` — the subject is not on
`kan_client::Claim`, it comes from the `(subject, claim)` pair `claims_matching`
already iterates, so the call site in `claims_matching` passes it in. The body
becomes a conjunction: `kind` equality, then each of `contains` (substring),
`starts_with` (anchored, `text.starts_with`), and `subject` (glob-lite) that is
`Some` must hold; a `None` predicate is vacuously satisfied. Glob-lite subject
matching is a few lines — strip a trailing `*` and prefix-match, else compare
exact — deliberately **not** a glob crate, matching day#34's rule that a
probe value is claim-supplied input and a pattern engine is a wider surface than
needed (the same reason `contains` is a substring, noted in the existing doc
comment). `describe()` gains cases for the new predicates so a verdict reads
e.g. `` `Decision` claim starting with `adversarial review of` on `atom/*` ``.

**`src/telos.rs`** — `WitnessSchema::starter` is where day's own `verdict` and
`assessment` probes live; they migrate to the narrowed forms (REQ-5). No other
change: assessment stays cumulative via `probe::evaluate`, which calls the same
`claims_matching`, so it inherits the narrowing for free (REQ-6).

**`src/position.rs`** — `position::resolve` reaches `claims_matching` for the
`Probe::Claim` arm, so cycle-relative inference inherits the narrowing with no
change here either (REQ-6). This single-evaluation-site property is what AC-6
verifies rather than assumes.

**`src/status.rs`** is untouched — it threads the boundary and client into
inference, which is unaffected by how a claim is matched.

**`docs/CONVENTIONS.md`** — the claim-probe subsection gains `subject` and
`starts_with`; the day#70 caution paragraphs (the `assessment`-too-broad note
and the marker-quoting note) are replaced with the resolved semantics.

**Tests** extend `tests/cycle_position.rs` (which already has the `claim`-probe
fixtures and the `write_kan_stub` harness with `decision_claim`/`result_claim`
carrying distinct subjects and timestamps) for AC-1 through AC-6, and a
`tests/plugin.rs` doc assertion for AC-7. The serde round-trip (AC-4) sits in
`src/probe.rs`'s unit tests beside the existing `a_claim_probe_round_trips_…`
test.

**Nothing here writes a claim or reads the log differently** — the two new
predicates are filters over `(subject, claim)` pairs `ClaimLog` already reads,
so day still stores nothing and adds no kan round-trip. `telos/no-store-of-its-own`
is untouched.

## Resolved Questions

- **The seam stays at `subject`; the author/enrichment dimension is deferred to
  kan#117.** An `author` predicate is technically buildable now (day already
  reads `author` on every claim) and is on the day-ownable side of ADR-18 (a flat
  filter, not kan's trust-weighted fold). It is deferred anyway, for two reasons:
  (1) representational pre-commitment — shipping a flat `author` filter into the
  stable probe language would make it day's way of saying "whose evidence counts,"
  which kan#117's enrichment records may supersede, leaving two ways to express
  one thing; (2) correctness — a flat `author == did` filter is not even a correct
  `Solo{did}`, because it would miss the same actor's other keys that kan's fold
  merges via `SameAs`. So the frame-relative dimension belongs against the kan#117
  contract, and this design's job toward it is only to make `matches` a
  conjunction so adding it later is additive (REQ-3).
- **`subject` is glob-lite, not a full glob.** The real need is namespace scoping
  (`atom/*`), which a trailing-`*` prefix covers. A full glob engine reintroduces
  the pattern-language surface day#34 rejected for `contains` on the same grounds
  (claim-supplied input). Expandable later if a concrete need for mid-string
  globbing appears.
- **The `contains`-quoting fix is folded in, as an anchored `starts_with` mode.**
  Neither `subject` nor a future `author` predicate separates a real verdict from
  the decision that defined its marker — they can share both subject and author.
  What separates them is structure: `record::review` anchors the marker at the
  start of the text. `starts_with` is that separator, and it is cheap (a distinct
  optional field, `text.starts_with`), so it is included here rather than tracked
  as a follow-up.

## Out of Scope

- **The `author`/enrichment/frame dimension.** Deferred to kan#117 and the frames
  design written against its exposed contract (see the `frames` subject in kan).
  This design only keeps `matches` shaped to receive it additively.
- **Full glob or regex for `subject` or the text predicates.** Glob-lite prefix
  and substring/anchored are enough for the declared needs; a pattern language is
  the surface day#34 deliberately avoided.
- **Subject-scoping `verdict`.** A verdict lands on whatever subject was reviewed,
  so there is no clean subject prefix for it; `verdict` is narrowed by
  `starts_with` alone, and `assessment` by `subject` alone.
