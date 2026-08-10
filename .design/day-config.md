# Feature: `day config`

## Summary

A read-only verb that prints day's **effective configuration**: for every key
day reads out of a `schema/*` claim, the value in force, the CID of the claim
that set it or `(default)`, and which layer it came from. It exists so that
prose can stop restating values day reads from kan — the mediation principle
recorded on `process-model` says a Markdown document may *point at the verb
that prints the current answer* but must never carry a second copy with no
fold behind it. `day config` is that verb, and F21 is blocked on it.

## Requirements

- REQ-1: `day config` reports every key day reads from a `schema/*` subject.
  The set is the seven enumerated in `.design/vocabulary-packs.md`:
  `schema/witness`, `schema/docs`, `schema/cycle`, `schema/injection`,
  `schema/verdicts`, `schema/blocks`, `schema/design-doc`. The enumeration is
  **derived from the loaders**, not written out a second time — a hand-written
  list of what day reads is the defect class this verb exists to retire.

- REQ-2: Each key carries three facts: the **value** in force, its
  **provenance** (the CID of the claim that set it, or `(default)`), and the
  **layer** it came from. This is `.design/vocabulary-packs.md`'s REQ-13, and
  its AC-20 and AC-21 are inherited rather than restated here.

- REQ-3: The verb is read-only. It makes **zero** `kan` write-verb
  invocations, asserted against a counting stub rather than by inspection.

- REQ-4: A subject day could not fully read is rendered as **unreadable** —
  never as absent, and never silently as the shipped default. One unreadable
  subject does not suppress the other six, and the exit code stays 0.

- REQ-5: `day config` consumes the three-state reader from
  `.design/read-visibility.md` (its REQ-1 and REQ-2). It introduces **no new
  hand-taught withheld-read guard**, because six such guards written by hand
  across three review rounds are what day#160 exists to retire.

- REQ-6: Per-key provenance requires a key to *be* a subject
  (`.design/vocabulary-packs.md` REQ-11). Until that lands, a whole-block
  claim sets every field in its block and there is exactly one CID to report
  for all of them, so `day config` is sequenced **after** it rather than
  printing a per-key column whose entries cannot differ.

- REQ-7: Exactly one code path reads a `schema/*` subject **with** provenance,
  and its value for a subject agrees with what that subject's existing loader
  returns. Two readers of one subject that can disagree is the shape day#101
  records three times.

- REQ-8: `--json` is the machine surface and the rendered form is for a
  person, per kan#199. The JSON shape is versioned, so a consumer that does
  not recognise it says so rather than reading a partial answer as a whole one.

- REQ-9: `day config` stores nothing. It holds no cache, writes no file, and
  every value it prints is derived from a kan claim on the read it just did.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) With no `schema/*` claim in the log, `day config` prints a
      row for every one of the seven subjects, each marked `(default)`. A test
      asserts the rendered subject set equals the set the loaders read, so a
      newly-added schema subject fails this test rather than going unreported.

- [ ] AC-2: (REQ-2) On a log with one `schema/injection` claim setting
      `cadence`, the `cadence` row prints that claim's CID and the untouched
      fields of the same block print `(default)`.

- [ ] AC-3: (REQ-2) Every row prints a layer, and a value taken from a
      project claim renders a different layer from one taken from day's
      shipped default.

- [ ] AC-4: (REQ-3) Driven against a counting stub, `day config` performs zero
      `kan observe`/`plan`/`decide`/`result` invocations. The count is
      asserted as zero, not as "no writes were observed".

- [ ] AC-5: (REQ-4) With `schema/verdicts` withheld from the view, its row
      renders as unreadable and names the withheld count; the other six rows
      still print their values; the process exits 0.

- [ ] AC-6: (REQ-4) The unreadable rendering is distinct from both `(default)`
      and from a declared value, asserted by three different fixtures rather
      than by the absence of a phrase.

- [ ] AC-7: (REQ-5) A source scan asserts `day config`'s module contains no
      comparison against a withheld count and no `AbsentUnderNarrowedTrust`
      match arm — it obtains the distinction from the reader's type instead.

- [ ] AC-8: (REQ-6) A test asserts that when two keys of one block are set by
      two different claims, their rows print two different CIDs. This test
      cannot pass before `.design/vocabulary-packs.md`'s REQ-11, which is the
      mechanical statement of the sequencing.

- [ ] AC-9: (REQ-7) For every one of the seven subjects, a test asserts the
      value reported by the provenance-carrying read equals the value returned
      by that subject's existing loader in `src/blocks.rs` and `src/schema.rs`.

- [ ] AC-10: (REQ-8) `day config --json` emits a versioned shape; a fixture
      pins it, and an unrecognised version is an error with a message rather
      than a partial read.

- [ ] AC-11: (REQ-9) `day config` creates no file. A test runs it in a
      workspace, snapshots the tree before and after, and asserts equality
      apart from `.kan/`, which kan itself initialises.

## Architecture

**Where the values live now.** Four loaders in `src/blocks.rs` and one in
`src/schema.rs` each read one `schema/*` subject through
`atoms::newest_fenced` in `src/atoms.rs`. All four have the same body shape,
and it discards precisely what this verb needs:

```rust
Ok(atoms::newest_fenced::<Self>(client, &subject)?
    .map(|(_cid, x)| x)          // the CID is thrown away here
    .unwrap_or_default())
```

So `day config` cannot be built by calling the existing loaders. It needs a
read that keeps the CID.

**The chosen shape is one new module** that reads the seven subjects with
provenance and owns the rendering. The existing loaders are left alone, which
keeps the diff small and keeps every current caller on the path it already
uses. The cost is real and named in REQ-7: two code paths then read the same
subjects, and nothing structural stops them drifting. AC-9 is the mitigation —
a test that pins their agreement per subject — and it is a test rather than a
type because unifying them would mean changing every loader's signature and
every one of their callers, which is a larger change than this verb justifies.

**Why this waits on `.design/read-visibility.md`.** `atoms::newest_fenced`
ends with a guard that turns *any* unparsed read into
`AbsentUnderNarrowedTrust` whenever the **log-wide** withheld count is
non-zero. day#160 records that guard as over-broad for exactly the four
loaders whose documented contract is "absent means use the shipped default",
and `day config` reads all four. Building against today's `Result` would mean
teaching a seventh call site the distinction by hand — six were taught that
way across three review rounds, and that is what produced day#160. The
migration replaces the error variant with a three-state return so the
compiler enumerates the sites; its AC-10 pins the migrated count. `day config`
consumes the result and never carries a guard of its own, which is what AC-7
asserts.

**Why it also waits on per-key subjects.** `.design/vocabulary-packs.md`'s
REQ-11 makes a configuration key its own subject. Until then a whole-block
claim sets every field in its block, so a per-key CID column would print the
same CID on every row of a block — a column that cannot vary is not
provenance, it is decoration. AC-8 is written so that it can only pass after
REQ-11, which makes the ordering checkable rather than asserted.

**Sequencing, stated plainly.** This verb is therefore downstream of two
pieces of work rather than being the next thing:
`.design/read-visibility.md` (8 requirements, 11 criteria) and
`.design/vocabulary-packs.md`'s REQ-11. F21 stays open across both. That is a
larger prerequisite stack than the handoff on `agents/handoff/main` assumed
when it named `day config` as the immediate next step, and it is recorded here
so the roadmap position is a decision rather than a surprise.

**Exit code.** `day config` always exits 0, like `day status` in
`src/status.rs`. This is in tension with the rule that could-not-check
outranks checked-and-clean, and the tension is resolved rather than ignored:
that rule governs *verification* tools, and `day config` is a display verb.
The incompleteness is carried in the output, per AC-5 and AC-6, and a script
that needs a gate reads `--json`. `telos/affordance-not-enforcement` governs
day's own verbs too.

**What it must not become.** `day config` reports; it never offers to declare.
Under a narrowed trust base the starter command that `src/blocks.rs` emits is
the harm `atoms::newest_fenced`'s guard was written to prevent — following it
appends a competing claim under a key the view does not admit, and the
vocabulary forks silently.

## Resolved Questions

- RQ-1: `day config` is built **after** `.design/read-visibility.md`'s
  three-state migration, not before it, so that it never carries a hand-taught
  withheld-read guard and never becomes a thirteenth call site to migrate.

- RQ-2: Provenance is **per-key, and therefore blocked on**
  `.design/vocabulary-packs.md` REQ-11. A per-key column over whole-block
  claims would print one CID on every row of a block, which cannot distinguish
  anything.

- RQ-3: Provenance is read by **one new module reading all seven subjects**,
  leaving the existing loaders in `src/blocks.rs` untouched. The resulting risk
  of two readers disagreeing is pinned by a per-subject agreement test rather
  than by a type.

- RQ-4: An unreadable subject **prints its row, marked, and the process exits
  0**. One withheld subject never suppresses the other six, and the exit code
  is not used to report incompleteness because this is a display verb rather
  than a verification tool.

## Out of Scope

- Writing configuration. `day config` never appends, and never prints a
  starter command that would append under a view that cannot see what is
  already there.
- Per-key *merging* semantics across layers. Which claim wins is
  `.design/vocabulary-packs.md`'s question; this verb reports the outcome of
  that fold and does not define it.
- `telos/*` and `bridge/*` declarations. They go through
  `atoms::newest_fenced` too, but they are a subject's own declaration rather
  than configuration, and redeclaring one must replace rather than merge.
- The rendered-output pass of day#172 and kan#199. `day config` should be
  legible, but the short-CID form it would use is kan#198's and is not
  day's to invent.
