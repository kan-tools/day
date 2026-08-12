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
- REQ-11: A configuration key is **its own subject** — `schema/injection/cadence`,
  `schema/witness/verdict` — read with the newest-claim-wins fold day already
  has. Today a whole block lives in one claim, so a claim setting one field
  resets the others (`src/blocks.rs:275`, `:375`) and a sixteen-entry map must
  be restated to add one. Verified: kan accepts three-segment rkeys, and
  `KanClient::show` is served from the memoised bulk read, so N subjects cost
  no extra kan invocation.
- REQ-12: The effective value of a key is `Default`, overlaid by a legacy
  whole-block claim on the parent subject if one exists, overlaid by the newest
  live claim on the key's own subject. One rule, three layers, and **no
  migration**: a project that never adopts per-key subjects keeps exactly
  today's behaviour.
- REQ-16: A key is removed by **retracting the claim that set it**. This is the
  requirement per-key subjects exist for: `kan retract` operates on a claim, one
  claim carries a whole block today, so a single witness probe cannot be
  retired without rewriting the set. day writes no retraction — it reads live
  claims (`src/kan_client.rs:75`) and a human runs the verb.
- REQ-18: A tension the pack work makes concrete is recorded by citing the
  existing claim on its subject, never by re-declaring it. `Tension::new` sorts
  its pair, so a second declaration silently supersedes the first in injected
  context.
- REQ-19: **Every file a pack writes is bounded by a source-scanned invariant.**
  Writing outside kan is the genuinely new capability, and today
  `ac9_the_render_cache_is_touched_in_exactly_one_module` matches the `.day/`
  literal only — so after packs land, a `src/pack.rs` writing an arbitrary path
  passes every scan in the repo. REQ-6 bounds the set in prose; prose in the
  right place is not a constraint.
- REQ-20: Granular retraction (REQ-16) is **not available for a key inherited
  from the legacy whole-block layer**, because retracting that claim removes
  every key it carried. day says so when it reports such a key, rather than
  letting the capability appear uniform. Every adopting project passes through
  this hybrid state.
- REQ-21: A per-key claim's shape is declared, not implied: which fence carries
  the value, and how the assembler enumerates the per-key subjects that exist.
  After `kan retract` the subject **remains**, carrying only a `Retraction`, so
  "subject exists, no block" must read as *key absent* rather than as a read
  failure.
- REQ-17: The three layers are assembled by exactly one function, which every
  `schema/*` loader calls. Seven loaders reimplementing an overlay is the shape
  day#101 records three instances of.
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
- REQ-8: Every atom a pack declares is checked through `telos::cautions_for`,
  **not** `telos::cautions` — `every_declaration_reports_its_cautions` in
  `tests/plugin.rs` fails the build on a direct call, so a `src/pack.rs` written
  to the first version of this requirement would not compile. Any
  already-satisfied, structurally monotone, or unreadable `done` criterion is
  reported. Reported, never refused — `telos/affordance-not-enforcement`
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
- [ ] AC-14: A claim on `schema/injection/cadence` setting 25 and a claim on
  `schema/injection/max_practice_items` setting 30 resolve to `cadence == 25`
  **and** `max_practice_items == 30`. Under today's fold, two claims on the
  parent subject lose the first; this is the test that fails before the change.
  (REQ-11)
- [ ] AC-15: With no claim anywhere, every field equals its `Default`. With a
  legacy whole-block claim on `schema/injection` and no per-key claims, every
  field equals what that block declared — byte-identical to today's behaviour,
  which is what "no migration" has to mean. (REQ-12)
- [ ] AC-16: A per-key claim overrides the legacy block for its own key and
  leaves the block's other fields intact. (REQ-12)
- [ ] AC-17: Retracting the claim on `schema/witness/verdict` removes exactly
  that probe and leaves the other fifteen — driven through a stubbed kan whose
  folded view omits the retracted claim, since that is what day reads. Under
  today's shape this is not expressible at all. (REQ-16)
- [ ] AC-18: A source scan asserts no `schema/*` loader calls `newest_fenced`
  directly; all of them go through the one assembler. Same shape as
  `the_claim_shape_predicate_has_one_evaluator`, and for the same reason.
  (REQ-17)
- [ ] AC-19: `schema/verdicts` — a `Vec<String>`, which per-key *merging* could
  not express — resolves as one subject per permitted verdict, and retracting
  one removes it while `VerdictVocabulary::validate`'s non-empty rule still
  fires when the last goes. (REQ-11, REQ-16)
- [ ] AC-20: `day config` on a log with one `schema/injection` claim prints
  that claim's CID beside the key it set and `(default)` beside a key it did
  not. (REQ-13)
- [ ] AC-21: `day config` makes zero `kan` write-verb invocations, asserted
  against a counting stub. (REQ-13)
- [ ] AC-22: Refusing one declaration in the walkthrough applies the others and
  writes nothing for the refused one, asserted as write-verb invocation counts
  per subject. (REQ-14)
- [ ] AC-23: `day pack apply` with stdin not a TTY and no `--yes` exits
  non-zero, writes nothing, and says which flag would proceed. Driven by
  running the binary with piped stdin, which is the mode this repo is never
  interactively in — day#91's failure shape. (REQ-15)
- [ ] AC-24: `--yes` with piped stdin applies the pack and prints the same
  per-subject report the walkthrough shows. (REQ-15)
- [ ] AC-25: `pack` is absent from the MCP tool list, asserted the way
  `tests/assess.rs` asserts it for `--run`. (REQ-15)
- [ ] AC-26: A source scan asserts every `fs::write`/`File::create`/`OpenOptions`
  in `src/` is in `src/cache.rs` or `src/pack.rs`, and that `src/pack.rs`'s
  writes resolve under a declared allowlist. Verified by adding a write to an
  arbitrary path and watching it fail. (REQ-19)
- [ ] AC-27: A pack declaring a path outside the allowlist is refused whole,
  with the path named, and makes zero writes. (REQ-19, REQ-9)
- [ ] AC-28: Recording the pack tension leaves the existing claim on
  `tension/composable-process--no-store-of-its-own` live and cited, and
  `day hook session-start` still renders the original rationale. (REQ-18)
- [ ] AC-29: A key set only by a legacy whole-block claim reports that it cannot
  be retracted alone, naming the other keys that claim carries. (REQ-20)
- [ ] AC-30: A subject carrying only a `Retraction` resolves as *key absent* and
  not as a read failure — driven through a stubbed kan whose folded view is that
  shape, which is what `kan retract` actually leaves behind. (REQ-21)
- [ ] AC-31: A per-key claim whose fence is unrecognised is refused with the
  fence named, matching every other declared vocabulary. (REQ-21)

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

### One subject per key, and why it is not the merge this design first proposed

`atoms::newest_fenced::<T>()` returns the newest claim carrying the fence and
nothing else; `unwrap_or_default()` supplies a whole `T` when there is none. The
first draft of this design called for folding those subjects **per key** — walk
the claims, take the newest that mentions each field. Reviewing the twelve call
sites before implementing showed why that is wrong, and the review is worth
recording because the sentence read fine.

**They are not one kind of thing.** Four shapes go through `newest_fenced`:

| subject | shape | what "per key" would mean |
|---|---|---|
| `schema/injection`, `schema/cycle`, `schema/docs`, `schema/design-doc` | config struct | field-wise |
| `schema/witness`, `schema/blocks` | `BTreeMap<String, _>` | entry-wise |
| `schema/verdicts` | `Vec<String>` | **undefined — a list has no keys** |
| `telos/*` `Witnesses`, `bridge/*` `Plan` | a subject's own declaration | *wrong* — redeclaring must replace |

The last row is correctly out of scope by the `schema/*` prefix, and by
accident rather than by design. The third row is not expressible at all. And the
shape that actually hurt — restating sixteen probes to add one — is the map,
while the requirement had been written from the mild field-wise case.

**Worse, merging costs removability exactly where it matters.** Today, restating
a map without an entry deletes it; that is the only way to retire a bad probe,
and day never retracts. Under entry-wise merge a mis-declared probe is immortal.

**One subject per key gets all of it for less.** `schema/injection/cadence` is
its own subject, so newest-wins per subject *is* per-key resolution with no fold
change. Three facts make it cheap, each checked rather than assumed:

- kan already accepts three-segment rkeys — written to a scratch log and read
  back.
- `KanClient::show` is served from the memoised `kan show --all --json`
  (`src/kan_client.rs:464`), so sixteen subjects cost **no extra invocation**.
- `kan retract` exists and day reads *live* claims, so retracting one key's
  claim removes that key. Granular retraction is the capability the whole-block
  shape structurally cannot offer, and it is the real argument here — not
  ergonomics.

Compatibility is a third layer rather than a migration: `Default`, overlaid by a
legacy whole-block claim on the parent subject, overlaid by per-key claims. A
project that adopts nothing sees today's behaviour byte-for-byte.

The cost is subject count — day's own log goes 64 → roughly 100, and roughly a third
becomes configuration, which `kan status` and `kan issues` then list alongside
work. That is not fixable from day, and papering over it by avoiding the design
would trade granular retraction for presentation. Filed as kan#186: kan grew a
config system without noticing, and subject visibility is the design question it
raises.

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

**The tension is already declared, and the obvious command would erase it.**
`tension/composable-process--no-store-of-its-own` carries a live Decision today
("Richer process structure pulls toward day-owned schema and state; the fenced
`day-atom` JSON block is already a schema smuggled inside claim text…"), and
`day hook session-start` renders it. `Tension::new` sorts its pair
(`src/tension.rs:68`), so `day telos tension no-store-of-its-own
composable-process "…"` lands on **that same subject**, and `src/hooks.rs` takes
the newest — so running it as written replaces the existing rationale in
injected context rather than adding to it.

So REQ-18: the pack work extends that claim by citing it, and does not mint a
second statement of the same tension. Held rather than resolved, and queryable,
per day's own model.

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
  individually refusable (REQ-14, AC-22). `declare`'s revision semantics are
  unchanged underneath; what changes is that nothing is applied unseen.
- RQ-2: **The walkthrough is a full TUI, and `ratatui` is day's first TUI
  dependency.** Neither day nor kan carries one today, so this departs from
  "match kan's dependency choices where they overlap" deliberately: a
  thirty-subject pack reviewed through sequential stdin prompts is a worse
  artifact than the vocabulary it installs. The cost is a second code path for
  the non-TTY case, which is day#91's exact failure shape — a mode this repo is
  never in and therefore never tests — so REQ-15 makes that path a refusal
  rather than a fallback, and AC-23 drives it with piped stdin.
- RQ-3: **`day config` already exists; it is seven subjects and has no name.**
  `schema/witness`, `schema/docs`, `schema/cycle`, `schema/injection`,
  `schema/verdicts`, `schema/blocks` and `schema/design-doc` are each read by
  `atoms::newest_fenced` with `unwrap_or_default()` — defaults, overlaid by the
  latest claim, excluding retractions, which is the shipped answer to "where is
  day's config". It is not a store and does not become one: the config *is* a
  claim, which is `telos/vocabulary-substrate` as written.
- RQ-4: **SUPERSEDED BY RQ-7 — the defect is real, the remedy was wrong.** The overlay is per claim, not per key, and that is a defect for packs.
  A pack setting one field resets the rest of its block (`src/blocks.rs:275`, `:375`). REQ-11 and REQ-12 make the fold per-key; REQ-13 adds a read-only
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
- RQ-7: **Per-key merge is rejected; a configuration key becomes its own
  subject.** RQ-4 named a real defect and prescribed the wrong fix, found by
  reading all twelve `newest_fenced` call sites before implementing rather than
  after. Three reasons, in increasing order of importance. (a) "Per key" is
  under-specified for a `BTreeMap` and **undefined for a `Vec`** —
  `schema/verdicts` is a list and a list has no keys. (b) The shape that
  actually hurt is the map, not the config struct RQ-4 was written from:
  restating sixteen probes to add one. (c) Merging makes a mis-declared probe
  **immortal**, because restating a map without an entry is currently the only
  way to remove one and day never retracts. One subject per key resolves all
  three and needs no fold change — newest-wins per subject *is* per-key
  resolution — and it makes `kan retract` granular enough to remove a key,
  which is the capability the whole-block shape structurally cannot offer.
  Cheap because `KanClient::show` is served from one memoised bulk read, so N
  subjects cost no extra kan invocation. Compatibility is a third overlay
  layer, not a migration. The cost is subject count (64 → roughly 100 in day's
  own log), filed as kan#186 rather than designed around.

- RQ-8: **Five design findings from the cold review, fixed rather than argued
  with.** REQ-8 named `telos::cautions`, which this branch's own guard forbids
  calling directly — a `src/pack.rs` written to it would not compile, and the
  requirement was written *after* the guard landed. The `no-store` tension is
  already declared on a subject `Tension::new` sorts into, so the command this
  document prescribed would have replaced its rationale in injected context
  rather than extending it (REQ-18). Nothing bounded the file writes that are
  the pack's genuinely new capability, so a `src/pack.rs` writing an arbitrary
  path would pass every scan in the repo (REQ-19, AC-26). Granular retraction —
  RQ-7's stated "real argument" — is unavailable for a key inherited from the
  legacy layer, which is the state every adopting project passes through, and
  the design claimed the capability uniformly (REQ-20). And the per-key claim
  shape, the mechanism RQ-7 turns on, was never specified: which fence, how
  subjects are enumerated, and that a retracted subject *remains* carrying only
  a `Retraction` and must read as key-absent (REQ-21).

- RQ-9: **RQ-7 scopes declarations out by the wrong property, and two `schema/*`
  subjects fall through the gap.** RQ-7's table puts "a subject's own
  declaration" out of scope and says it is "correctly out of scope by the
  `schema/*` prefix, and by accident rather than by design" — which was a
  prediction that the accident would hold. It does not. `schema/docs` and
  `schema/design-doc` carry the prefix and are declarations: neither type has a
  shipped default, absence is `NotDeclared` rather than a value, and redeclaring
  must replace. Under a prefix rule they are configuration, and per-key
  resolution would give them a layer 1 that does not exist.

  The property that actually decides it is **whether the type has a default**,
  and the type system already carries it: `layers::config` requires
  `T: Default`, so a declaration cannot be routed there even deliberately. That
  turns the scoping rule from a naming convention into a compile-time one, which
  is what the prefix was standing in for.

  Found while routing the second shape, by reading the types rather than the
  requirement — the requirement reads fine, which is the same way RQ-4's remedy
  read fine before all twelve call sites were looked at. REQ-17's scan is
  written accordingly: it asks every direct fenced read to *state* why it is not
  per-key, rather than asserting that none exists. The four categories that
  legitimately remain — map, list, no-default declaration, subject's own
  declaration — are named at their call sites, so a new loader on the direct
  path is an offender until someone writes the reason down.

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
