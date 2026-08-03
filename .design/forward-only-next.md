# Feature: `next` becomes forward-only, and feedback moves to `revisits`

## Summary

`next` currently carries two different relations — sequence ("review follows
build") and feedback ("a review sends you back to fix") — and every consumer
that treats it as an ordering assumes a DAG it does not have. This splits them:
`next` becomes forward-only and **guaranteed acyclic by construction**, a new
optional `revisits` field carries feedback edges, and every consumer that needs
an ordering obtains it from one view that cannot be built without also handing
back the cycles it had to drop. The constant false "a step was skipped" warning
(day#113) disappears as a consequence; the DAG guarantee is the deliverable.

## Requirements

- REQ-1: `atoms::Interface` gains `revisits: Vec<String>`, defaulting to empty
  and skipped when empty, exactly as `done` is. A `day-atom` block written
  before this existed parses, composes, and round-trips byte-identically.
- REQ-2: A `day-atom` block whose `revisits` is non-empty declares
  `_version: 2`, and `Interface::SUPPORTED_VERSION` becomes 2. A block with no
  `revisits` emits no `_version` and stays a v1 block. This follows the stated
  meaning of `atoms::VERSION_KEY` — "the reader version a block **requires**" —
  so an older day reading a migrated vocabulary reports `BlockError::TooNew`
  ("upgrade day to read it") rather than `Malformed`, which would blame the
  claim for the reader being behind.
- REQ-3: The forward relation over `next` is obtained only through
  `atoms::Forward`, which excludes every edge participating in a cycle and
  carries the cycles it excluded on the same value. There is no way to get the
  ordering without the could-not-order list.
- REQ-4: `day doctor` reports a cycle in `next` as a finding that names the
  atoms on the cycle, names the edges it dropped, and says the feedback edge
  probably belongs in `revisits`. It **does not** change `Report::is_healthy`
  or the exit code: a cycle is a legal declaration in a vocabulary written
  before `revisits` existed, so an existing project is told, not broken.
- REQ-5: The off-sequence check in `src/position.rs` walks `Forward` only, and
  when `Forward` excluded any edge it reports that it could not check the order
  — never silence, and never a clean report.
- REQ-6: Input coverage in `atoms::check` is computed over `Forward`'s
  transitive ancestors. When an atom's inputs are covered by the raw declared
  closure but not by the acyclic one, the resulting finding is marked
  could-not-check and does not fail `is_healthy` — the exclusion, not the
  vocabulary, is what made it unanswerable.
- REQ-7: `revisits` targets are checked for existence, exactly as `next`
  targets are, and a `revisits` target that does not reach the declaring atom
  through `next` is reported as a finding: a revisit that is not a return has
  no defined meaning. `revisits` never contributes to input coverage and never
  appears in any ordering.
- REQ-8: A slug appearing in both `next` and `revisits` on one atom is refused
  by `Interface::validate` as `BlockError::Invalid`. One block cannot say an
  edge is both forward and backward, and no block written before this feature
  can trip it.
- REQ-9: `day next <atom>` renders successors and revisits as distinct sections,
  so `day next adversarial-review` stops presenting `generative-build` and
  `pull-request` as equal successors.
- REQ-10: `day atom declare` accepts `--revisits <slug>` (repeatable).
- REQ-11: Reading `atom.interface.next` anywhere outside `src/atoms.rs` fails
  the build unless the site carries an adjacent `// dag-not-required: <why>`
  comment, enforced by a source scan in `tests/plugin.rs`.
- REQ-12: day's own vocabulary is migrated, declaring four `revisits` edges:
  `adversarial-review`, `assess-telos`, `pull-request` and `assess-docs` each
  revisit `generative-build`. Only the first is currently in `next`; the other
  three are true relations day has never been able to state.
- REQ-13: `tests/fixtures/migration-blocks.json`'s from-the-future block
  declares a `_version` strictly greater than `Interface::SUPPORTED_VERSION`,
  asserted by a test rather than by a hardcoded number, so bumping the
  supported version can never quietly turn that fixture into a readable block.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) A `day-atom` block with no `revisits` key parses to
  `revisits == []`, and `Interface::to_claim_text` for an interface with empty
  `revisits` produces text byte-identical to what this commit's parent produced
  for the same interface — asserted against the committed block corpus in
  `tests/fixtures/block-corpus/`.
- [ ] AC-2: (REQ-2) `Interface::to_claim_text` emits `"_version":2` when `revisits` is
  non-empty and emits no `_version` key when it is empty; `parse_block` accepts
  a `day-atom` body declaring `_version: 2` and rejects `_version: 3` as
  `BlockError::TooNew`.
- [ ] AC-3: (REQ-3) `Forward::build` on `[a next b, b next a]` yields no successors for
  either atom and exactly one cycle listing both atoms and both dropped edges;
  on an acyclic set it yields every declared edge and no cycles.
- [ ] AC-4: (REQ-3) `Forward::build` on an atom whose `next` names itself yields one
  cycle of length one and drops that edge.
- [ ] AC-5: (REQ-4) `day doctor` against a vocabulary containing a `next` cycle prints a
  finding naming both atoms and the word `revisits`, prints `composition: ok`,
  and **exits zero**.
- [ ] AC-6: (REQ-5) `day status` against the cyclic fixture prints no
  "a step was skipped" line for either direction of the cycle, and prints a
  could-not-check line naming the atoms whose order it could not establish.
- [ ] AC-7: (REQ-5) `day status` against a fixture where an acyclic upstream genuinely
  did not produce still prints its "a step was skipped" line — the true positive
  survives.
- [ ] AC-8: (REQ-12) With day's own vocabulary migrated, a working tree with a modified
  `src/*.rs` and no recorded verdict produces **no** off-sequence finding
  mentioning `adversarial-review`, while the same tree with no design doc still
  reports `generative-build produced its output but upstream design did not`.
- [ ] AC-9: (REQ-6) On the fixture `design next a`, `a next b`, `b next a`, `a in[x]`,
  `b out[x]`, `atoms::check` reports `a`'s uncovered input as could-not-check,
  and `Report::is_healthy` is true. The test asserts the fixture reaches that
  state — that the raw closure covers `x` and the acyclic one does not — rather
  than only that a finding was produced.
- [ ] AC-10: (REQ-7) `atoms::check` reports a finding for a `revisits` target that does
  not exist, and a distinct finding for one that exists but does not reach the
  declaring atom through `next`; it reports neither for
  `adversarial-review revisits generative-build` in day's own vocabulary.
- [ ] AC-11: (REQ-8) An interface with `next: ["x"]` and `revisits: ["x"]` fails
  `parse_block` with `BlockError::Invalid`.
- [ ] AC-12: (REQ-9) `day next adversarial-review` output contains `pull-request` under a
  successors heading and `generative-build` under a distinct revisits heading,
  and does not list `generative-build` as a successor.
- [ ] AC-13: (REQ-10) `day atom declare x --revisits y` records a block containing
  `"revisits":["y"]`.
- [ ] AC-14: (REQ-11) The source scan in `tests/plugin.rs` fails when a
  `.interface.next` read is added to a file other than `src/atoms.rs` without a
  `dag-not-required:` comment, verified by reintroducing such a read.
- [ ] AC-15: (REQ-13) A test reads `tests/fixtures/migration-blocks.json` and asserts its
  `_version` exceeds `Interface::SUPPORTED_VERSION`.

## Architecture

**The block.** `src/atoms.rs` holds `Interface`. `revisits` is added with
`#[serde(default, skip_serializing_if = "Vec::is_empty")]`, the mechanism
`done` already uses and which `Interface`'s doc comment names as the reason old
blocks stay byte-identical. `Interface::validate` (the `Versioned::validate`
hook, added for day#20's structurally-empty bridge node) gains the both-fields
check in REQ-8: it is the right home because it is decidable from one block,
where the return check in REQ-7 needs the whole atom set and therefore belongs
in `atoms::check`.

`Interface::to_claim_text` currently does `serde_json::to_string(self)`. It
becomes: serialize to a `serde_json::Value`, and insert `_version: 2` at the
front when `revisits` is non-empty. `atoms::version_gate` already strips
`VERSION_KEY` before the typed parse — its doc comment records that this is
precisely so no block type needs a `_version` field — so the read path needs no
change beyond `SUPPORTED_VERSION`.

**The ordering.** A new `atoms::Forward<'a>` is built from `&'a [Atom]`:

```rust
pub struct Cycle {
    pub atoms: Vec<String>,
    pub dropped: Vec<(String, String)>,
}

pub struct Forward<'a> { /* successors, cycles */ }

impl<'a> Forward<'a> {
    pub fn build(atoms: &'a [Atom]) -> Self;
    pub fn successors(&self, name: &str) -> &[&'a str];
    pub fn predecessors(&self, name: &str) -> Vec<&'a str>;
    pub fn ancestors(&self, atoms: &'a [Atom], name: &str) -> Vec<&'a Atom>;
    pub fn cycles(&self) -> &[Cycle];
}
```

An edge `u -> v` is dropped iff `v` reaches `u` through declared `next` edges,
which covers self-loops without a special case. Cycles are reported as the
non-trivial strongly connected components, computed as the mutual-reachability
partition rather than by Tarjan: the vocabularies are single digits of atoms,
and `docs/CONVENTIONS.md`'s composition check is meant to be boring and
obviously right. `Forward::ancestors` replaces the private `ancestors` function
in `src/atoms.rs`, which today walks the raw `next` and is only terminating
because of its visited set — after this it is terminating because the graph is
acyclic, and the visited set is an optimisation.

The guarantee is that `cycles()` sits on the same value as `successors()`. This
is the day#101 shape and deliberately the same one `position::infer` already
uses in its REQ-8 note: take the whole declaration rather than the half a caller
happened to want, so there is no half to forget.

**Findings.** `atoms::Finding` gains `unchecked: bool` beside `unreadable` and
`version_skew`, and `doctor::Report::is_healthy` becomes "every finding is
`unchecked`". `unreadable` deliberately keeps failing while `unchecked` does
not: an unreadable block is not legal at any version, whereas a `next` cycle is
a legal declaration in a vocabulary written before `revisits` existed. Both
`src/status.rs`'s `unreadable_from` and the render in `src/doctor.rs` already
filter on typed flags rather than on message prose, which is what makes adding
a third flag safe here.

**Off-sequence.** `src/position.rs`'s loop at the `off_sequence` block moves to
`forward.predecessors`, and `position::Report` gains `unordered: Vec<String>`
beside the existing `read_failures` and `unrecorded` — the module already
distinguishes "checked and absent" from "unknowable" through `Presence`, and
this is the same distinction one level up, at the graph rather than the probe.
`src/status.rs` renders it under the existing `Off-sequence:` heading with a
`?` marker to separate could-not-check from the `!` of a finding.

**Why this is not the fix day#113 rejected.** The rejected proposal was to
exclude cyclic edges *as the fix*, silently and with no vocabulary change: that
loses the true positive `generative-build -> adversarial-review` (a verdict
recorded with no code change — a rubber stamp) along with the noise. Here,
after REQ-12, day's own `next` has no cycle at all: the forward edge
`generative-build -> adversarial-review` stays in `next` and keeps reporting,
and only the feedback edge moves. Exclusion applies solely to vocabularies that
have not migrated, and it is reported rather than silent. The distinction is
that exclusion is now the fallback for an ambiguous declaration, not the answer
to the question.

**Consumers, and the source scan.** After this change `atom.interface.next` is
read raw in exactly one place outside `src/atoms.rs`: the declaration dump in
`src/doctor.rs`'s `render`, which prints what the claim says and must not
silently omit a dropped edge. It carries a `dag-not-required:` comment.
`src/position.rs`, `src/record.rs`'s `next` verb and `src/status.rs`'s `here`
all move to `Forward`. `tests/plugin.rs` gains the scan, in the shape CLAUDE.md
records as the one that held — an explicit escape hatch, because a test with no
way out gets deleted the first time it is wrong.

**Docs and fixtures.** `docs/CONVENTIONS.md`'s atom table gains the `revisits`
row and the DAG guarantee, stated as a guarantee rather than a convention.
`scripts/capture-block-corpus.sh` is re-run because the block shape changed, and
`tests/fixtures/migration-blocks.json` is bumped above the new supported
version per REQ-13.

## Resolved Questions

- RQ-1: A `revisits` target that does not reach the declaring atom through
  `next` is an advisory finding, not a refused block and not silence. It catches
  a forward edge filed in the wrong field — the exact confusion this milestone
  removes — without refusing a declaration day read and understood.
- RQ-2: day's own vocabulary declares all four feedback edges, not only the one
  currently forming a 2-cycle. The other three are true and have never been
  expressible; declaring them is what dogfooding the field means.
- RQ-3: The DAG guarantee is enforced by a source scan with a
  `dag-not-required:` escape hatch, not by type discipline alone. CLAUDE.md
  records that a rule stated in one module's doc comment failed to propagate
  five times, and that the source-scan shape is the one that held.
- RQ-4: A `day-atom` block declares `_version: 2` only when it actually uses
  `revisits`, rather than on every block a new day writes. `VERSION_KEY` is
  documented as the reader version a block *requires*, and a block with no
  `revisits` requires nothing new.

## Out of Scope

- Per-edge annotation inside `next`, weighed in day#113 and rejected as a
  larger change to a block many projects have already written.
- Inferring the split from a bridge or a heuristic. day guessing at intent the
  project could state is what produced day#113.
- Topological ordering, longest-path and partial-order *reporting*. This
  milestone makes them available; day#108 is where a surface uses them.
- Vocabulary packs (day#73), which transport this graph and are deferred to
  v0.12 for exactly the reason this milestone exists.
