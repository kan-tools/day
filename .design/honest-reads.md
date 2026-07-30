# Feature: Honest reads — day reports the declarations it cannot account for

## Summary

day's durable behaviour is driven by project-declared vocabulary read back from
kan claims, and that read path currently **drops what it does not recognise,
silently**. Seven fenced block types, none rejecting unknown fields, and each one
exists to *constrain or narrow* something day then reports on — so a dropped
field is not lost information, it is a false certification. This milestone makes
the read path honest: unknown fields are refused, blocks carry a version so the
refusal says *why*, the failure is reported on the surfaces that matter
(including the one that reaches the model, which is silent today), and no read
error is ever reported as an absent artifact.

It is `v0.7.0-beta.3`'s hard precondition. Every surface made declarable before
this contract exists inherits the silent-widening bug, which is day#78 against
day#74 exactly — and it must *ship*, not merely merge, because the fix only helps
a binary that has it.

## Requirements

- REQ-1: day's built-in fenced blocks are **restrictive**: an unrecognised field
  is refused rather than ignored. Applies to all seven (`day-atom`,
  `day-telos`, `day-bridge`, `day-witness`, `day-schema`, `day-docs`,
  `day-tension`), and closes day#78 for `ClaimShape` specifically. The rule this
  encodes, stated once so day#74 inherits it: a vocabulary day *reads as facts*
  is descriptive and tolerates unknown fields — which is the contract day
  requires of kan — while a vocabulary day *reads as constraints* is restrictive,
  because ignoring a constraint reports something as verified that was not.
- REQ-2: Each built-in block may carry a `_version` integer, **versioned per
  block type** rather than one version for the vocabulary, so a reader fails only
  on the block that actually changed. An absent `_version` means 1, so every
  block written before this milestone stays valid unchanged. The name is
  underscore-prefixed to mark it as metadata distinct from declared content, and
  deliberately not `$`-prefixed, since day#74 makes block schemas
  project-declared and `$` is reserved in JSON Schema.
- REQ-3: A refused block produces a **diagnostic that names the cause and the
  fix**, distinguishing two states that are indistinguishable today: a block
  declaring a `_version` this day does not read ("upgrade day") from a block
  malformed at a version it does read ("fix the claim"). Includes the subject
  being printed twice in the telos path (`telos/bad: telos/bad: fenced block…`).
- REQ-4: An unreadable declaration costs the **smallest unit that contains it**,
  never the whole read, and is always reported. This is largely how day already
  behaves — atoms degrade per-atom and name the cascade, an `--all` telos sweep
  reports the bad one and assesses the rest at exit 2, and a single-object schema
  failing its command *is* its smallest unit — so the requirement is to make it a
  **stated invariant with a test**, since REQ-1 turns a rare path into a common
  one.
- REQ-5: The **session-start hook reports unreadable declarations**, on both of
  day's audiences and per its existing allocation: the model's context carries
  the caveat *attached to the item it undermines* rather than in a footer, and
  the human gets a `systemMessage` differentiated by REQ-3's cause. Today the
  hook lists an unreadable telos beside the readable ones and says nothing —
  the channel that reaches the model, silent.
- REQ-6: `UserPromptSubmit` is **registered**, giving day a mid-session channel
  to the model for the first time. Two distinct triggers: a **state-transition**
  display gated on a cheap git check, and a **bounded periodic re-display** of
  standing conditions defaulting to every 10 user turns. `path`- and `tag`-driven
  transitions resolve live; `claim`-driven ones remain session-start-only until
  day#71, and day says which rather than implying live coverage.
- REQ-7: REQ-6's "have I already said this" memory uses the `.day/` cache, and
  the carve-out stays bounded by a **stated test**: if `.day/` were deleted,
  day's answer must not change — it may only re-display sooner. Nothing day
  *decides* may read the cache; the invariant remains source-scanned.
- REQ-8: **No read failure is reported as an absent artifact.** Fixes day#81
  (`src/docs.rs` folding a failed `kan show` into "no release recorded") and
  makes it non-recurring, honouring the rule `src/probe.rs` already states: a
  subject day cannot read is an error, never a silently empty result.

## Acceptance Criteria

- [ ] AC-1: For each of the seven built-in blocks, a block carrying an
      unrecognised field is refused rather than parsed with the field dropped.
      **Negative control:** the same block without that field parses, and its
      declared content resolves identically to before this milestone — proving
      the refusal is the new behaviour and not a broken parser. (REQ-1)
- [ ] AC-2: An `atom/*` declaring `{"in":…,"out":…,"next":…,"requires":[…]}` no
      longer loads as though `requires` were absent, and `day doctor` no longer
      reports `composition: ok` over it. A `schema/design-doc` declaring
      `forbidden_sections` no longer yields all-`[PASS]` and exit 0 on a document
      that violates it. Both are the reproductions that scoped this milestone.
      (REQ-1)
- [ ] AC-3: A block with `_version: 1` and a block with no `_version` resolve
      identically. A block declaring a `_version` above what this day reads is
      refused, while a *different* block type at a version this day does read
      still resolves in the same run — so the version is per type, not global.
      (REQ-2)
- [ ] AC-4: The diagnostic for a too-new `_version` names the version read and
      the version declared and directs the reader to upgrade day; the diagnostic
      for a malformed block at a known version directs them to the claim. The two
      messages are distinguishable by assertion, and neither prints its subject
      twice. (REQ-3)
- [ ] AC-5: With one unreadable declaration among several, every readable one
      still resolves and the unreadable one is named, for each of: the atom
      vocabulary, an `--all` telos sweep, and the witness map. **Negative
      control:** removing the reporting makes a test fail rather than merely
      changing wording — a check that passes with the report stubbed out is not
      testing it. (REQ-4)
- [ ] AC-6: `day hook session-start` output naming an unreadable telos carries
      the caveat on that telos's own line, and a grep of the full output for
      error language finds it — the assertion that fails today. Separately,
      `day hook session-notice` emits a `systemMessage` whose text differs
      between the version-skew and malformed causes, and emits **nothing** when
      every declaration is readable. (REQ-5)
- [ ] AC-7: `hooks/hooks.json` registers a `UserPromptSubmit` hook, it emits no
      blocking construct (`tests/plugin.rs`'s existing enforcement stays green),
      and it emits nothing when there is neither a transition nor a standing
      condition. A `path`-witness transition produced mid-session is displayed;
      a `claim`-witness one is not, and the output says position is
      session-start-only for claim-probed types rather than staying silent.
      (REQ-6)
- [ ] AC-8: Deleting `.day/` between two prompts changes only *when* the periodic
      re-display next fires, never what day reports about position, atoms, teloi,
      or witnesses. Asserted by comparing day's reports across the deletion.
      `tests/plugin.rs`'s source scan still finds exactly one module touching
      `.day/`. (REQ-7)
- [ ] AC-9: With kan unreadable for one subject, `day assess docs` reports a
      could-not-check finding naming that subject rather than reporting no
      release recorded, and its exit code is the could-not-check code rather than
      the findings code. **Negative control:** with kan readable and genuinely no
      release claim, the absent case is still reported as absent. (REQ-8)

## Architecture

**The contract lives in the shared read primitive.** `atoms::newest_fenced` in
`src/atoms.rs` is the one function every block type is located and parsed
through, and `extract_fenced` beside it does the deserialization. `_version` is
read there — before the typed parse, since the whole point is to answer "can this
day read this block at all" without first requiring the block to fit a struct
this day defines. A too-new version short-circuits to a typed refusal carrying
the declared and supported versions; a supported version proceeds to the typed
parse, where `deny_unknown_fields` now applies.

`deny_unknown_fields` goes on the seven structs themselves —
`atoms::Interface`, `bridge::Plan` and `bridge::Witnesses`, `probe::ClaimShape`,
`schema::Schema`, `docs::DocsSchema`, `tension::Tension` — with `_version` as an
explicitly allowed field. `probe::ClaimShape` is day#78's case and needs no
special handling once the general rule is in place, which is the point of
deriving the rule rather than patching the instance.

**Degradation granularity mostly exists and needs pinning, not building.**
`atoms::load` already collects per-atom failures as findings and names the
cascade; `telos.rs`'s sweep already reports a bad telos and assesses the rest;
`telos::WitnessSchema`'s per-entry deserializer already sets an unreadable probe
aside in `unsupported` and renders it as `ERROR`. That last one is the model to
generalise from — it exists because a `claim` probe recorded on this repo made
the installed v0.6 binary fail the *whole* witness map. REQ-1 makes that path
common rather than rare, which is why AC-5 pins it with a negative control.

**The hook work is two audiences already separated by design.**
`hooks/hooks.json` registers two `SessionStart` hooks on purpose: `session-start`
prints markdown into the model's context, and `session-notice` emits only
`{"systemMessage": …}` for the human, split so a human notice cannot regress
context injection. `src/status.rs`'s `notice()` already carries transition and
off-sequence events, and unreadable-declarations becomes a third event type on
it — inheriting its infallibility (any failure degrades to empty output) rather
than establishing a new contract. The model-facing half is in `src/hooks.rs`,
attaching the caveat to the rendered item; where an unreadable declaration causes
the item to vanish entirely — an unreadable atom drops out of the vocabulary, and
the hook currently says "No process atoms are declared yet" — there is nothing to
attach to, so it is named instead.

**`UserPromptSubmit` is designed and unwired.** `docs/ROADMAP.md`'s situated
injection section specifies it, `tests/plugin.rs` already allowlists it as one of
the three events whose stdout reaches the model, and `hooks/hooks.json` registers
none. The cost split decides the scope: `day status` measures 2.76 s of which
1.99 s is 41 `kan` invocations, while the git half of the same computation is
0.03 s — so a git gate is affordable per prompt and a kan read is not. Cheap
gate, expensive confirm: when the changed-file set moves, do the one kan read and
emit only if the atom actually changed. The two triggers stay separate mechanisms
because a recurring reminder is much closer to the ambient standing rule that
failed in day#30 than an event notice is; the periodic channel therefore carries
only standing conditions affecting the correctness of what day already said.

**`src/cache.rs` remains the only module touching `.day/`**, and the turn counter
and last-displayed marker go there. They are display state by REQ-7's test:
delete them and day re-displays sooner, never differently. The boundary that
matters — nothing day *decides* reads the cache — is unchanged and still scanned
from source by `tests/plugin.rs`.

**REQ-8 is a small fix and a scan.** `src/docs.rs` folds a failed
`client.show` into an empty claim list, so an unreadable release subject reads as
no release recorded. The error propagates instead and surfaces as a
could-not-check finding. The `unwrap_or_default()` calls in `src/telos.rs` and
`src/bridge.rs` were checked and are correct — they are `Option` after a `?`, so
read errors already propagate and `None` genuinely means no declaration block.

**Nothing here writes a claim, and no new substrate is added.** No new process is
spawned, so `src/probe.rs`'s `run_command` remains day's only spawn site for a
probe; `telos/no-store-of-its-own` is untouched.

## Resolved Questions

- **`_version` is a minimum reader version, not a shape generation.** A block
  declaring `_version: 2` asserts "a day that does not understand v2 cannot read
  this correctly" — it does not assert that the v1 shape is gone. So a v2 day
  reads v1 blocks natively, because each version is a superset of the last and
  the bump exists to warn *older* readers. The alternative — a version per
  incompatible shape, with day carrying a reader per generation — buys nothing
  here, since day controls these seven schemas and would have no reason to remove
  a field rather than stop requiring it.
- **`_version`, not `$version` or `v`.** Underscore marks metadata as distinct
  from declared content, which a bare `v` does not — a project's block could
  legitimately want a field called `v`. `$` is avoided because day#74 makes block
  schemas project-declared, and if that validation is ever expressed as JSON
  Schema, `$`-prefixed names are reserved. day's blocks are user-authored content,
  where a collision with a real field name is a live hazard; kan's `"v": 1` sits
  on an envelope kan generates, where it is not.
- **Per block type rather than one vocabulary version.** A single version bumped
  for one block invalidates all seven for older readers, which is the whole-read
  blast radius REQ-4 exists to avoid. Seven numbers to track is the cost, and it
  is the same cost as seven fence strings day already tracks.
- **day#81 belongs in this milestone.** It is the identical defect class reached
  by a different mechanism — a swallowed `Result` rather than a dropped field —
  and shipping a release whose thesis is "day reports what it cannot account
  for" while leaving a known instance in place would undercut the thesis.
- **The periodic cadence defaults to 10 turns and is explicitly a guess.**
  Filed as day#82 to tune against measured recall rather than intuition, and made
  declarable in `v0.7.0-beta.3`. If measurement shows recall does not decay at
  any testable N, the honest response is to drop the periodic channel and keep
  only state-triggered display.

## Out of Scope

- **Making anything new declarable.** The six-vocabularies-to-one-mechanism work
  is day#74 in `v0.7.0-beta.3`; this milestone only makes the existing read path
  honest so that work does not inherit a silent bug. The cadence from REQ-6 is a
  fixed constant here and becomes a declaration there.
- **Live `claim`-driven position mid-session.** Gated on day#71's bulk read at
  2.0 s per recomputation. `path` and `tag` resolve live; day says which rather
  than implying coverage it does not have.
- **Frames, and anything reading under a non-default trust base.** Needs kan's
  v0.8 REQ-3; day's consumer contract for it is `.design/kan-read-contract.md`.
- **Changing what an older day already does.** Nothing here can retro-fix
  `v0.7.0-beta.1`, which drops day#70's narrowed predicates silently. That is
  day#78's residue and the reason this milestone ships before beta.3 adds fields
  rather than alongside it.
