# Feature: Vocabulary packs — a declared process, transported as data

## Summary

A pack is a declarative manifest that records a project's process vocabulary —
teloi, atoms, bridges, tensions, and the `schema/*` blocks day reads — in one
reviewable file, applied by one command. It exists because day's README claims
to be "a generalizable layer for structured process", and generality is only
real when a process can be *transported*; today the only transport is a shell
script of `day telos declare` invocations that each project rewrites. The same
mechanism carries the config a plugin structurally cannot deliver, which makes
applying a pack the consent boundary for anything day would change in a user's
own files.

Serves `telos/vocabulary-substrate` directly — "a process day cannot transport
is a process only day's own repo has" — and `telos/composable-process`. It
trades against `telos/no-store-of-its-own`, which is recorded as a tension
rather than resolved in prose (see Architecture).

## Requirements

- REQ-1: A pack is a **declarative manifest that day walks**, never a script
  day executes. day has exactly three process-spawn sites (`src/git.rs`,
  `src/kan_client.rs`, `src/probe.rs`) and `CLAUDE.md` forbids a fourth; a
  pack's content originates outside the repo applying it, which is the precise
  situation `src/probe.rs`'s four rules exist for.
- REQ-2: Every durable thing a pack records is an ordinary kan claim, written
  by invoking `kan` as a subprocess through `src/kan_client.rs`. A pack adds no
  day-owned store, and its own text is not retained after application.
- REQ-3: A pack may declare any subject day already reads: `telos/*`, `atom/*`,
  `bridge/*`, `practice`, and the `schema/*` blocks enumerated in
  `src/blocks.rs` — `schema/witness`, `schema/docs`, `schema/cycle`,
  `schema/injection`, `schema/verdicts`, `schema/blocks`, `schema/design-doc`.
- REQ-4: `MAX_ITEMS` in `src/practice.rs` becomes a declared field on
  `InjectionSchema` (`src/blocks.rs:327`, fence `day-injection`) alongside
  `cadence`, with the current value 12 as its serde default. A pack sets it by
  declaring `schema/injection` like any other block.
- REQ-11: The `schema/*` subjects are folded **per key, not per claim**. Today
  `atoms::newest_fenced` (`src/blocks.rs:275`, `:357`) takes the newest claim
  wholesale and `unwrap_or_default()` fills the rest, so a claim setting one
  field silently resets every other field in that block to its default. A pack
  touching one setting must not clobber the others.
- REQ-12: A field never mentioned by any live claim resolves to its `Default`,
  and a field mentioned by several resolves to the newest claim that mentions
  it. Retracted claims are excluded, which they already are — the fold reads
  kan's live view through `kan show --all --json`.
- REQ-13: `day config` prints the effective configuration and, per key, the CID
  of the claim that set it or `(default)`. Read-only: it writes nothing and
  declares nothing, so the one way to change a setting stays `kan observe` on
  the schema subject, directly or via a pack.
- REQ-14: Pack application is an interactive walkthrough presenting each
  declaration with its before/after, individually acceptable or refusable.
- REQ-15: The walkthrough requires a TTY. Without one — CI, a pipe, MCP — day
  refuses rather than degrading to an unattended apply, unless `--yes` is given,
  which applies the whole pack after printing the same report the walkthrough
  would have shown.
- REQ-5: Applying a pack **shows every change before making it** and requires
  confirmation, because a pack mutates subjects in the applying repo's log and
  may mutate files the user owns.
- REQ-6: A pack may declare config a Claude Code plugin structurally cannot —
  `statusLine` in the user's settings and `.gitignore` entries — and these are
  the only files outside kan a pack writes. Each is shown individually under
  REQ-5 and is independently refusable.
- REQ-7: Applying a pack ends by running the `doctor` composition check and
  reporting its result, so a pack that leaves the vocabulary non-composing says
  so rather than exiting clean (day#73 item 2).
- REQ-8: Every atom a pack declares is checked with `telos::cautions`
  (`src/telos.rs:126`) and any already-satisfied or structurally monotone `done`
  criterion is reported. Reported, never refused — `telos/affordance-not-enforcement`
  governs day's own verbs.
- REQ-9: A pack manifest is refused whole if any part of it fails to parse.
  Partial application would leave a vocabulary neither the author nor the
  adopter designed, and day cannot retract what it wrote.
- REQ-10: A pack file is a fenced block in an ordinary Markdown document, so
  the pack ships beside the prose that explains it and both are reviewed
  together (day#73 item 3).

## Acceptance Criteria

- [ ] AC-1: A source scan in `tests/plugin.rs` asserts pack application reaches
  no `Command::new` outside `src/kan_client.rs`, extending the existing
  three-spawn-site invariant. (REQ-1)
- [ ] AC-2: Applying a manifest declaring one telos and one atom produces
  exactly the claims `day telos declare` and `day atom declare` would, byte-for-byte
  in claim text, verified against a stubbed kan via `DAY_KAN_BIN`. (REQ-2, REQ-3)
- [ ] AC-3: `grep -c "day-pack" src/` finds the fence declared once, and
  `BlockSchemas` refuses a pack block carrying an unknown field, matching every
  other fenced vocabulary. (REQ-3, REQ-9)
- [ ] AC-4: `InjectionSchema` parses `{"cadence":25,"max_practice_items":30}`;
  `{}` yields `cadence` at `DEFAULT_CADENCE` and `max_practice_items` at 12; an
  unknown field is refused. (REQ-4)
- [ ] AC-5: `day hook session-start` in a repo declaring
  `max_practice_items: 30` projects more than 12 practice items, and the
  "further item(s) not shown" notice reflects the declared cap rather than 12.
  (REQ-4)
- [ ] AC-6: Applying a pack without confirmation makes no `kan` write —
  asserted as **zero write-verb invocations** against a counting stub, not as
  absence of output. (REQ-5)
- [ ] AC-7: The pre-application report names every subject the pack would
  write and every file it would touch; a test drives a manifest with one of
  each and asserts both appear. (REQ-5, REQ-6)
- [ ] AC-8: Declining the `statusLine` change applies the vocabulary and leaves
  the user's settings file byte-identical. (REQ-6)
- [ ] AC-9: A manifest whose atoms do not compose produces a non-composing
  `doctor` report on stdout and a non-zero exit, with the vocabulary still
  written — the report is the finding, not a rollback. (REQ-7)
- [ ] AC-10: A manifest declaring an atom whose sole `done` criterion is a
  `claim` probe prints the monotone caution and still applies. (REQ-8)
- [ ] AC-11: A manifest with a valid first declaration and a malformed second
  makes zero `kan` write invocations. (REQ-9)
- [ ] AC-12: A pack applied from a Markdown file with prose around the fence
  applies identically to the same fence alone. (REQ-10)
- [ ] AC-13: `tests/documented_invocations.rs` covers the pack invocation in
  `README.md`, so the documented form is known to parse and run. (REQ-10)
- [ ] AC-14: Two claims on `schema/injection`, the first `{"cadence":25}` and
  the second `{"max_practice_items":30}`, resolve to `cadence == 25` **and**
  `max_practice_items == 30`. Under today's fold the first is lost; this is the
  test that fails before the change and passes after. (REQ-11)
- [ ] AC-15: With no claim on `schema/injection`, every field equals its
  `Default`; with one claim mentioning one field, the others still equal their
  `Default`. (REQ-12)
- [ ] AC-16: A retracted claim setting a key does not contribute it — driven
  through a stubbed kan whose folded view omits the retracted claim, since that
  is what day actually reads. (REQ-12)
- [ ] AC-17: The per-key fold is applied by exactly one function, and a source
  scan asserts no `schema/*` loader calls `newest_fenced` directly — the same
  shape as `the_claim_shape_predicate_has_one_evaluator`, and for the same
  reason: a second folder would diverge silently. (REQ-11, REQ-12)
- [ ] AC-18: `day config` on a log with one `schema/injection` claim prints
  that claim's CID beside the key it set and `(default)` beside a key it did
  not. (REQ-13)
- [ ] AC-19: `day config` makes zero `kan` write-verb invocations, asserted
  against a counting stub. (REQ-13)
- [ ] AC-20: Refusing one declaration in the walkthrough applies the others and
  writes nothing for the refused one, asserted as write-verb invocation counts
  per subject. (REQ-14)
- [ ] AC-21: `day pack apply` with stdin not a TTY and no `--yes` exits
  non-zero, writes nothing, and says which flag would proceed. Driven by
  running the binary with piped stdin, which is the mode this repo is never
  interactively in — day#91's failure shape. (REQ-15)
- [ ] AC-22: `--yes` with piped stdin applies the pack and prints the same
  per-subject report the walkthrough shows. (REQ-15)
- [ ] AC-23: `pack` is absent from the MCP tool list, asserted the way
  `tests/assess.rs` asserts it for `--run`. (REQ-15)

## Architecture

### The verb

`src/cli/mod.rs:118` declares `Command::Init`, whose contract is that it
**writes nothing** — `init_instructions` (`src/cli/mod.rs:902`) prints wiring
steps for a human to perform. day#73 proposes `day init --pack <file>`, which
would make a flag invert the verb's defining property; day#109 proposes
`day pack <source>`. This design takes the second, as `day pack apply <file>`.

`CLAUDE.md` says "Keep the CLI small. Four verbs today. A new verb needs a
design doc." The count is stale — `day --help` lists twelve — and that staleness
is itself evidence for the rule rather than against it. This document is the
design doc the rule asks for.

### Manifest shape

A fenced `day-pack` block in Markdown, parsed through the existing
`Versioned`/`parse_block` machinery in `src/blocks.rs`, so a pack inherits what
every other fenced vocabulary already has: refusal of unknown fields, a
`_version` the refusal can cite, and the "smallest unit containing the problem"
error shape `telos/honest-reads` requires. A pack is a list of declarations,
each naming a subject and carrying the same block content that subject's own
schema already defines — so the pack format adds no second grammar for teloi or
atoms, it transports the one that exists.

### Where the config lives, and why this is not a store

The practice cap is the worked example. `src/practice.rs:46` hardcodes
`MAX_ITEMS: usize = 12`, and `spine` records that a pack shipping more than the
remaining budget delivers a fold-order-dependent subset. The fix is not a config
file: `InjectionSchema` (`src/blocks.rs:327`) is already a project-declarable
block governing the injection channel, read from `schema/injection` as a kan
claim, with `cadence` as its one field. `max_practice_items` becomes its second.

That is `telos/vocabulary-substrate` applied as written — "everything day
hardcodes that a project may reasonably differ on is declarable as a kan claim"
— and it means "day config" exists in exactly the form the telos permits. There
is no config store, because the config is a claim.

### The fold that has to change, and the compatibility question under it

`atoms::newest_fenced::<T>()` returns the newest claim carrying the fence and
nothing else; `unwrap_or_default()` then supplies a whole `T` when there is no
claim at all. The consequence nobody had needed until now is that
`{"max_practice_items": 30}` appended to `schema/injection` resets `cadence`,
because serde fills it from `default_cadence()` and the older claim that set it
is never consulted.

Per-key folding replaces that for the seven `schema/*` subjects: walk the live
claims oldest to newest, and for each key take the newest claim that *mentions*
it — which requires parsing to `serde_json::Value` before deserialising to `T`,
since "mentions" is a property of the JSON, not of the struct.

**This changes behaviour for logs that already exist**, in one direction: a
project whose newest claim deliberately omitted a field to reset it now inherits
the older value instead. That is a real regression for anyone who used omission
as a reset, and the honest mitigation is that it be visible — `day config` (REQ-13)
names the claim behind every key, so an inherited value has a CID next to it
rather than looking like a default. `tests/block_corpus.rs` and the migration
matrix are where the shipped-block shapes are pinned, and the corpus gains a
two-claim case.

### The tension that is real

A pack writes `statusLine` into the user's settings and entries into
`.gitignore`. Those are not kan claims, and `telos/no-store-of-its-own` says
"No config store, no sidecar database, no state file."

The honest reading is that this does not violate the rule as stated — day keeps
no state of its own, retains nothing, and never reads these files back to decide
anything, which is the same boundary that keeps the `.day/` cache carve-out
honest. But it is adjacent enough that arguing it away in prose is the failure
mode day exists to surface. It is recorded as a tension against
`telos/composable-process` instead:

```
day telos tension no-store-of-its-own composable-process \
  "Transporting a process requires writing the config that makes it run, and
   config is the one durable thing that is not a claim."
```

Held rather than resolved, and queryable, per day's own model.

### Why day#146 comes first

A pack transports atoms, and `day atom declare` never runs the falsifiability
check `day telos declare` runs (`src/cli/mod.rs:473` is its only call site). Six
of day's own nine atoms carry `done` criteria that can never report unmet.
Shipping packs before that is fixed gives the defect a distribution channel:
every adopter installs criteria that are green on arrival and stay green. REQ-8
is the pack-time half; day#146 is the declare-time half, and it lands first.

### Trust

day#109 proposes evaluating a pack against recorded trust toward its author.
kan already models this — `--trust`, and the identity work in kan#121/#136 — and
a pack from a remote source is claim-shaped data arriving from a stranger.
Remote sources are out of scope here (below); the local-file case still shows
every change before making it, which is the property that matters and does not
depend on the trust model landing.

## Resolved Questions

- RQ-1: **A pack is a walkthrough, not a batch.** Overwriting an adopter's tuned
  declaration is settled by showing it rather than by a policy — pack
  application is infrequent and high-ceremony, like `init`, so the answer is to
  lean into the interaction instead of choosing between silent revision and a
  `--force` flag. Each declaration is presented with its before/after and is
  individually refusable (REQ-14, AC-20). `declare`'s revision semantics are
  unchanged underneath; what changes is that nothing is applied unseen.
- RQ-2: **The walkthrough is a full TUI, and `ratatui` is day's first TUI
  dependency.** Neither day nor kan carries one today, so this departs from
  "match kan's dependency choices where they overlap" deliberately: a
  thirty-subject pack reviewed through sequential stdin prompts is a worse
  artifact than the vocabulary it installs. The cost is a second code path for
  the non-TTY case, which is day#91's exact failure shape — a mode this repo is
  never in and therefore never tests — so REQ-15 makes that path a refusal
  rather than a fallback, and AC-21 drives it with piped stdin.
- RQ-3: **`day config` already exists; it is seven subjects and has no name.**
  `schema/witness`, `schema/docs`, `schema/cycle`, `schema/injection`,
  `schema/verdicts`, `schema/blocks` and `schema/design-doc` are each read by
  `atoms::newest_fenced` with `unwrap_or_default()` — defaults, overlaid by the
  latest claim, excluding retractions, which is the shipped answer to "where is
  day's config". It is not a store and does not become one: the config *is* a
  claim, which is `telos/vocabulary-substrate` as written.
- RQ-4: **But the overlay is per claim, not per key, and that is a defect for packs.**
  A pack setting one field resets the rest of its block (`src/blocks.rs:275`,
  `:357`). REQ-11 and REQ-12 make the fold per-key; REQ-13 adds a read-only
  `day config` so an effective value can be traced to the claim that set it.
  This is the substantive change the practice-cap question turned out to be
  about — the cap itself (REQ-4) is one field of it.
- RQ-5: **Two new verbs, not one.** `pack` and `config`. `CLAUDE.md`'s "Keep the CLI
  small. Four verbs today. A new verb needs a design doc" is stale by eight —
  `day --help` lists twelve — and the rule survives its own count being wrong:
  this document is what it asks for. `config` earns its place by being what
  makes a pack's effect on configuration inspectable, and it is read-only.
- RQ-6: **day#146 lands first.** A pack transports atoms, and `day atom declare` never
  runs the falsifiability check `day telos declare` runs — six of day's own nine
  atoms carry `done` criteria that can never report unmet. Shipping packs before
  that is fixed gives the defect a distribution channel. REQ-8 is the pack-time
  half.

## Out of Scope

- **Remote pack sources.** day#109 sketches `day pack kan-tools/day`. Fetching
  a pack introduces a network substrate, a trust evaluation, and a caching
  question, none of which the local-file case needs. `day pack apply <file>`
  first; a fetcher is a separate design.
- **Uninstalling a pack.** day never retracts, and unwinding a vocabulary is a
  question about kan's model rather than a calling convention over it.
- **Packs that carry probe commands day would run.** A `command` probe declared
  by a pack is a command from a stranger; `--run` already gates execution, and
  whether a pack may supply one at all is a separate decision.
- **Migrating day's own vocabulary into a pack.** Dogfooding this by exporting
  day's own teloi and atoms is the obvious next step and is not this design.
- **`day pack export`.** Producing a manifest from a live log is the inverse
  operation and is not needed to transport a process someone has already
  written down.
