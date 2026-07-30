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

- REQ-0: A block that deserializes but **violates an invariant its type cannot
  encode** is refused in the same place, and with the same diagnostics, as one
  that will not deserialize. `deny_unknown_fields` (REQ-1) catches a block saying
  *more* than its type allows; nothing caught a block saying *less than it needs
  to mean anything*. day#20 is the instance: `{"any": []}` in a bridge plan is
  valid JSON and a valid `Vec<Node>`, and an empty alternative contributed
  nothing to reachability while reporting nothing — so `bridge check` could call
  a plan reachable on the strength of a branch that says nothing.
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
- REQ-9: **CI carries a version-migration matrix**, in both directions, because
  this milestone's entire subject is what one version does with another
  version's declarations and nothing currently tests that. The two directions
  are asymmetric and belong in different places:
  - **Forward (a current reader, historical blocks)** is a *guarantee* and must
    hold, so it is hermetic and runs on every push: day resolves every block
    shape any released version wrote, unchanged.
  - **Backward (a historical reader, current blocks)** is a *characterization*
    and runs on release, over a matrix of released tags. An older binary cannot
    be fixed, so the assertion is not that it behaves well but that it behaves
    **as recorded** — which turns "what will the deployed population do with the
    log we are about to write" from a thing discovered afterwards, as day#78 was,
    into a thing the release reports.

## Acceptance Criteria

- [ ] AC-0: An empty `seq`, `all`, or `any` node in a `day-bridge` plan is
      refused, naming the path to the offending node, and reported as the
      *claim's* problem rather than as version skew. Nested empty nodes are
      caught too, since a check reading only the root would pass a plan whose
      branch says nothing. **Negative control:** every shape day's own plan
      grammar produces still parses, and a plan day writes passes its own
      validation — so the invariant cannot be tightened past what day emits.
      (REQ-0)
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
- [ ] AC-10: A committed corpus holds the block shapes every released version
      wrote, one set per released tag, and a hermetic test asserts this build
      resolves each of them — no network, no old binaries, running on every
      push. **Negative control:** the corpus is generated from each tag's own
      `starter()` output rather than hand-written, so a shape nobody ever wrote
      cannot pass for history. (REQ-9)
- [ ] AC-11: A release-triggered matrix builds each released tag and runs it,
      through a stubbed kan, against **current-shape** blocks — using
      `day doctor` over a `day-atom` block, the one verb and block type every
      released version has. Each version's outcome is compared against a
      committed expectation, so a change in what the deployed population does
      fails the release rather than being learned later. Versions carrying the
      `_version` gate must refuse honestly; versions predating it are recorded as
      silently widening, which is day#78's residue stated as data. (REQ-9)

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

**The first implementation did not do this, and the adversarial review blocked
it.** It called `status::compute` unconditionally — measured 3.03 s on every
prompt — while the function's own doc comment and `hooks/hooks.json` both claimed
it read what session-start had already computed. Three-second-per-turn
degradation, under a 10 s hook timeout so it never failed, described by its own
documentation as its opposite: the precise defect this milestone exists to stop
day committing, committed by the milestone.

Recorded here rather than silently corrected, and worth being exact about who was
wrong: **this paragraph was right**. The design specified the gate; the
implementation ignored it and its comments asserted compliance. That is a
different failure from a bad design, and the useful lesson is narrower — a
comment claiming a performance property is worth nothing without a measurement
beside it, and there was none until the review took one.

The rule that keeps the cache honest for this: **a missing fingerprint means
recompute, never all-clear.** Then deleting `.day/` costs one redundant read and
never changes an answer, which is REQ-7's test applied to the second thing stored
there. Verified: cold 3.2 s, warm 0.02 s, slow again after a tracked file changes,
slow again after `rm -rf .day/`.

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

**REQ-9's two directions need different machinery, and conflating them is the
trap.** The forward direction needs no old binaries at all: the shapes each
version wrote are exactly what that version's `starter()` emitted, so the corpus
is generated by building each tag once and capturing its starter output, then
committing the result. That makes the guarantee hermetic and cheap enough to run
on every push, and it means the fixtures are provably shapes day actually wrote
rather than a maintainer's recollection of them.

The backward direction needs the old binaries, and every released tag is in git
(`v0.1.1-beta.1` through `v0.7.0-beta.1`), so the matrix builds from tags rather
than depending on crates.io remaining intact. `DAY_KAN_BIN` landed in day's first
feature commit, so every released version can be driven against a stub kan —
which is what makes this testable at all. `day doctor` over a `day-atom` block is
the probe, chosen because it is the one verb and the one block type every
released version has; `day status` would restrict the matrix to v0.6 and later.

The assertion shape matters more than the mechanism. An older binary cannot be
fixed, so asserting it behaves *correctly* would be asserting something false for
every version before this one. The matrix asserts it behaves **as recorded**, in
a committed table of version-to-outcome. That converts the question day#78
answered too late — what does the deployed population do when it meets our new
log — into a release-time report, and it makes the day#78 residue visible as data
rather than as a paragraph in a roadmap.

**Nothing here writes a claim, and no new substrate is added.** No new process is
spawned, so `src/probe.rs`'s `run_command` remains day's only spawn site for a
probe; `telos/no-store-of-its-own` is untouched. The matrix runs `cargo` and
`git` in CI, which is not day executing anything at a user's request and so does
not touch `src/probe.rs`'s guardrails.

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
