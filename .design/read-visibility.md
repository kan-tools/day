# Feature: a read that cannot be mistaken for an absence

## Summary

day has one representation for two facts — *nothing is recorded here* and *this
view cannot see what is recorded here* — and it has been separating them by hand,
one call site at a time, for three cold review rounds. `KanClient::show` returns
`Ok(vec![])` for both. `atoms::newest_fenced` returns `Ok(None)` for both. Every
consumer already handles `Result` and `Option`, so each new distinction added as
an *error variant* compiled everywhere and changed meaning silently.

This proposes replacing the two return types with ones that make the distinction
**unrepresentable-by-accident**: a consumer must say what it does about a
withheld read, or it does not compile. The value is not a better message. It is
that the compiler, rather than a reviewer, enumerates the sites — which is the
only thing that has been missing from a loop that is otherwise well instrumented.

Scope is the read contract only. No new verb, no vocabulary change, no
user-visible feature. The observable effect is that surfaces which currently
assert an absence they did not establish stop doing so, in one change rather
than in six.

## Requirements

- REQ-1: `KanClient::show` returns a value that distinguishes **present**,
  **absent**, and **withheld** (with the count), so a caller cannot obtain a
  claim list without having decided what a withheld read means.
- REQ-2: `atoms::newest_fenced` does the same for the fenced-vocabulary loaders:
  a caller cannot treat "no block parsed" as "none is declared" without saying so.
- REQ-3: Every existing call site is migrated deliberately, and each one's
  decision is recorded in the diff — not defaulted by a blanket `match … => {}`.
- REQ-4: A loader whose documented contract is *"absent means use the shipped
  default"* keeps that behaviour for a genuine absence and reports the withheld
  case. `BlockSchemas`, `InjectionSchema`, `CycleSchema` and `VerdictVocabulary`
  all document that contract, and `docs/CONVENTIONS.md` states that declaring
  none of them is a complete configuration.
- REQ-5: The remedy day prints for a withheld read must be executable. It is not
  today: day accepts no `--trust` flag on any verb, and the advice it gives
  (`--trust me`) narrows rather than widens, because kan's default base is
  `local` and `me` is a subset of it.
- REQ-6: `day hook session-start` never fails and never blocks, whatever the
  read returns. Advisory-never-blocking is enforced by `tests/plugin.rs` and is
  not weakened by this change.
- REQ-7: A write verb refuses rather than appending into a view it cannot fully
  read, and refuses on **both** withheld shapes — fully withheld as well as
  partial. day cannot retract, so a duplicate is permanent.
- REQ-8: The migration is checked by a mechanism, not by review. The set of
  call sites is derived by the compiler and the count is asserted, so a site
  added later cannot silently inherit a default.

## Acceptance Criteria

- [ ] AC-1: `show` returns a three-state value; a caller that ignores the
  withheld case does not compile. Demonstrated by adding a call site that
  ignores it and observing the build fail. (REQ-1)
- [ ] AC-2: `newest_fenced` returns a three-state value with the same property. (REQ-2)
- [ ] AC-3: `day assess telos <slug>` reports that the telos is unreadable, not
  that no such telos is declared, when its subject is fully withheld. (REQ-3)
- [ ] AC-4: `day next <atom>` does the same rather than reporting no such atom.
- [ ] AC-5: `practice::project` reports a withheld subject on **both** shapes;
  today only the `Err` arm is handled and the empty-claims arm is silent. (REQ-3)
- [ ] AC-6: `day review record` succeeds with a withheld `schema/verdicts`,
  using `DEFAULT_VERDICTS` — a subject day ships defaults for and never asks
  anyone to declare (REQ-4).
- [ ] AC-7: `day design record` refuses on a fully withheld subject as it
  already does on a partial one, and appends nothing (REQ-7).
- [ ] AC-8: The remedy text names a flag day accepts, and following it widens
  the view. If no such flag exists, the text says what to do instead and does
  not name one (REQ-5).
- [ ] AC-9: A `fixtures/behaviour` fixture covers each shape, and the harness
  reports the migration as `CHANGED-AS-DECLARED` rather than unexplained.
- [ ] AC-10: A test asserts the number of migrated call sites, so a new one
  cannot be added without the count moving. (REQ-8)
- [ ] AC-11: `day hook session-start` exits zero and renders on every read
  shape — present, absent and withheld — with no shape able to make it fail.
  `tests/plugin.rs` already forbids a blocking hook; this extends the same
  guarantee to the new states. (REQ-6)

## Architecture

The two functions are `src/kan_client.rs`'s `show` and `src/atoms.rs`'s
`newest_fenced`. Their consumers are the set this design exists to enumerate:
**15 `show` sites and 12 `newest_fenced` sites**, spread across `src/practice.rs`,
`src/docs.rs`, `src/hooks.rs`, `src/doctor.rs`, `src/atoms.rs`, `src/record.rs`,
`src/status.rs`, `src/telos.rs`, `src/tension.rs`, `src/blocks.rs`,
`src/bridge.rs` and `src/schema.rs`.

Six of those files were taught the distinction by hand across three review
rounds. The remaining sites are where rounds 2 and 3 kept finding siblings, and
`docs/CONVENTIONS.md` is the record of which contract each loader promises.

The withheld signal itself is already parsed and does not change:
`ShowAllEnvelope` and `SubjectsEnvelope` both carry a log-wide
`excluded_by_trust`, and `ShowAllEntry` carries a per-subject one. Their shapes
are pinned against the real binary by `tests/kan_conformance.rs`. What changes is
only how a read hands that fact to its caller.

`tests/plugin.rs` holds the source scans that would enforce REQ-8, and
`tests/fallbacks.rs` holds the premise-asserting fixtures for modes this repo is
never in — which is every mode in this document, since day's own log has
`excluded_by_trust: 0`.

## Resolved Questions

- RQ-1: Three states or two? Two (`Readable` / `Unreadable`) would be smaller,
  and it collapses the distinction REQ-4 depends on: a loader with a shipped
  default must treat a genuine absence differently from a withheld one, and with
  two states it cannot. Three.
- RQ-2: A new type, or an `Option<Result<…>>`-shaped composition of existing
  ones? A named enum. The point is that the *name* appears at every call site and
  a reader of the diff can see what each one decided; a nested generic reads as
  plumbing and gets pattern-matched away without thought.
- RQ-3: Migrate `show` and `newest_fenced` together, or in two passes? Together.
  They are one class, and the whole argument of this document is that splitting a
  class into instances is what produced three review rounds. Two passes would
  make the second pass an instance-fix by construction.
- RQ-4: Does a write verb refuse or warn on a withheld view? Refuse (REQ-7). day
  cannot retract, so an unwanted append is permanent and grows on every run; a
  warning that is ignored costs the record something it cannot get back. This is
  the one place in day where refusing beats reporting, and it is bounded to write
  verbs — hooks must always render (REQ-6).
- RQ-5: Should this block v0.12.0-beta.2? No. The branch's shipped behaviour is
  strictly better than `main` on every measure, and every open finding concerns a
  narrowed-trust workspace — a mode day did not handle at all before this branch.
  Shipping the improvement and doing the type change as its own milestone is the
  smaller risk. Recorded here so the sequencing is a decision rather than a
  drift.
- RQ-6: Is the remedy fixable inside this change, or does it need a `--trust`
  flag on day's verbs? Unresolved as a *mechanism*, resolved as a *rule*: until
  day accepts such a flag, the text must not name one. Whether to add the flag is
  its own design question, because it means day starts selecting a trust base
  rather than inheriting kan's — which touches `telos/no-store-of-its-own` and
  the frame model in `docs/TELOS.md`.
