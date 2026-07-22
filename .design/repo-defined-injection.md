# Feature: Repo-defined injection, projected from a kan subject

## Summary

day injects two prescriptive blocks — `PRACTICE` and `SAFETY` — and both are
hard-coded `const`s. Every other thing day knows about a project is declared
per project in kan and read back; the one thing day actually *says* to the
model each session is the one thing a project cannot change without forking
the binary. This projects a `practice` subject into the injected context, so a
project's own working practice reaches the agent the same way its teloi and
atoms do.

It ships **locally-signed only**: an injection path sourced from claims is
inert while one key signs everything, and stops being inert the moment kan
sync lands. The per-author trust list is designed here and not built, so the
extension point exists before it is needed rather than being retrofitted onto
a live injection path.

## Motivation

`src/hooks.rs` holds day's opinions as constants. The comment on `PRACTICE`
defends day *having* opinions — "process opinions are this tool's whole job,
which is exactly why they live here and not in kan (ADR-18)" — and that
defence is sound. It does not defend those opinions being **unextendable by
the project they are injected into**.

The consequence is concrete. A project with a house rule that matters — a
release checklist, a domain constraint, a pattern that burned them once — has
nowhere to put it where an agent will see it. `CLAUDE.md` is the usual answer
and it is harness-specific, unsigned, unattributable, and not part of the
record. The whole argument for kan is that durable things belong in the log.

**Why the fold shape matters here.** Every other `day-*` block is
newest-claim-wins, because a schema is one object. Injected practice is a
*list*, and a list in an append-only log is the thing kan's live fold already
gives for free: **append adds an item, retract removes one.** No revision
mechanism to design, no document to swap.

## Requirements

- REQ-1: day projects the live claims on a `practice` subject into the
  session-start context. Each live claim is one item. Appending a claim adds
  an item; retracting one removes it.
- REQ-2: The projection is **locally-signed only**. A claim on `practice`
  whose `author` is not this workspace's identity is not injected.
- REQ-3: Ignored claims are **reported, never silently dropped**. If a claim
  on `practice` is skipped for authorship, the injected block says how many
  and why. A projection that quietly omits things is indistinguishable from
  one that found nothing, which is the failure shape this repo has now met
  three times.
- REQ-4: If day cannot establish the local identity, it **injects no
  projected practice and says so**. Trust failure fails closed. Injecting
  claims whose authorship could not be checked, because checking was
  unavailable, is the exact inversion of the property REQ-2 exists for.
- REQ-5: day's own `PRACTICE` and `SAFETY` still inject by default, with
  project items added after them. A project may **replace** either, but only
  by an explicit claim carrying that instruction — so discarding day's
  opinions is possible (the README promises exactly that) and is a recorded,
  attributable act rather than a config toggle.
- REQ-6: The projection is bounded: a per-item excerpt limit and a cap on
  total projected items, both reported when they bite. Session-start competes
  with the user's actual request for attention, and a projection is the one
  part of that block a project can grow without limit.
- REQ-7: A `practice` subject with no live claims changes nothing. Absence is
  not an error and produces no note.
- REQ-8: `session_context` over MCP returns the same projected text as the
  hook, because both call `hooks::session_start`.
- REQ-9: **Designed, not built:** a `trust/injection` subject listing DIDs
  whose `practice` claims day will also project. It is itself a kan claim,
  signed locally, so the trust root is the local key and the list stays
  inside `telos/no-store-of-its-own`. `src/hooks.rs` resolves authorship
  through one function so that adding the list later is a change to that
  function and to nothing else.
- REQ-10: `docs/CONVENTIONS.md` documents the `practice` subject, the
  locally-signed rule, and the replace instruction.

## Acceptance Criteria

- [ ] AC-1: Two live claims on `practice` produce two injected items;
      retracting one produces one. (REQ-1)
- [ ] AC-2: A claim authored by a DID other than the workspace identity is
      absent from the injected text. (REQ-2)
- [ ] AC-3: With such a claim present, the injected block states that a claim
      was skipped and why — asserted on the rendered text, not on internals.
      (REQ-3)
- [ ] AC-4: With the local identity unavailable, no projected item appears
      and the block says the projection was skipped. Asserted with a stub
      whose identity call fails. (REQ-4)
- [ ] AC-5: By default both day blocks and the project's items are present,
      in that order; with a replace instruction recorded, day's corresponding
      block is absent and the project's items remain. (REQ-5)
- [ ] AC-6: An item longer than the excerpt limit is truncated, and a
      projection exceeding the item cap is trimmed with a note saying so.
      (REQ-6)
- [ ] AC-7: With no `practice` subject, the injected text is byte-identical
      to today's. (REQ-7)
- [ ] AC-8: The MCP `session_context` tool and `day hook session-start`
      return the same text for the same log. (REQ-8)
- [ ] AC-9: Authorship resolution is reached through exactly one function,
      asserted by a source scan — the property that keeps REQ-9's extension
      point from becoming a rewrite. (REQ-9)
- [ ] AC-10: `docs/CONVENTIONS.md` contains the `practice` subject name and
      the replace instruction token, checked against the code's constants.
      (REQ-10)

## Architecture

**`src/practice.rs` (new)** holds the subject name, the replace instruction,
the projection, and the bounding. It is the same shape as `src/tension.rs`:
a read over a subject, a small type, and a render. `src/hooks.rs` calls it and
appends the result after `PRACTICE` and `SAFETY`.

**`src/kan_client.rs`** gains `identity()`, shelling `kan identity did` — the
public, safe-to-share identifier, never `kan identity phrase`, which prints a
secret. It returns `Option`: kan's identity access can block on a macOS
keychain prompt that never arrives non-interactively (kan's own `src/sign.rs`
documents this, and it silently emptied day's reads once already), so this
must be a value day can branch on rather than an error that aborts a hook.
`Claim.author` already exists, from the `--json` migration.

**Authorship resolution lives in one function** in `src/practice.rs`, taking
the local identity and a claim and answering whether to project it. REQ-9's
trust list becomes a change to that function's body. The source scan in
`tests/plugin.rs` asserts nothing else decides it.

**Bounding reuses `src/hooks.rs`'s existing shape** — `TELOS_EXCERPT` already
caps a single telos line for exactly this reason, and the projection cap is
the same idea applied to a list a project controls.

**Nothing is written.** The projection reads `practice` and renders. day
acquires no new write path, and the hook stays infallible: every failure here
degrades to a note in the block, because a broken projection must not be able
to derail a session.

## Resolved Questions

- **Locally-signed as the shipped default, trust list designed alongside.**
  An injection path sourced from claims is inert with one signer and is not
  once sync lands. Scoping to the local key makes it inert *by construction*
  rather than by vigilance, and retrofitting a trust boundary onto a live
  injection path is much worse than designing one in.
- **The fold is all-live-claims, not newest-wins.** Injected practice is a
  list, and append-adds/retract-removes is what kan's live fold already
  provides. Every other `day-*` block is newest-wins because a schema is one
  object; this is not one object.
- **A single `practice` subject, not `practice/<slug>`.** One claim per item
  on one subject is lighter than one subject per item, and retraction already
  operates per claim.
- **Not under `schema/`.** `schema/*` subjects carry a fenced block describing
  a shape day validates against. This is content, not a shape.
- **Trust failure fails closed.** If the local identity cannot be established,
  nothing is projected. The alternative — projecting unverified claims because
  verification was unavailable — inverts the property REQ-2 exists to provide,
  and would do so exactly when something is already wrong.
- **Ignored claims are reported.** Silent omission is the failure shape this
  repo has hit three times now (a deleted log, a format change, an identity
  swap), each time presenting as a plausible empty result. A projection that
  drops claims quietly would be the fourth.
- **The safety block is replaceable, and day says so when it is replaced.**
  Treating it as unremovable would mean day holds an opinion a project cannot
  refuse, which is exactly what `telos/affordance-not-enforcement` forbids —
  and "this rule is too important to let you remove" is the argument every
  blocking tool makes about itself. There is also direct evidence against its
  efficacy: the block was in the agent's context and did not prevent the
  incident that motivated recording it. So it is replaceable like anything
  else, and the injected text states that safety guidance was replaced.
  Transparency rather than enforcement: the suppression is visible in the
  thing being suppressed, which is the strongest move available that is still
  advisory.

## Out of Scope

- **Building the trust list.** Designed under REQ-9, deferred until sync makes
  a second signer real.
- **`UserPromptSubmit` projection.** This is session-start only. Per-turn
  injection is v0.6's, alongside off-sequence detection, and wants the
  situated-vs-ambient distinction settled first.
- **Any new write path.** day projects what a project recorded; it does not
  offer a verb for recording practice. `kan observe --subject practice` is
  the whole interface, and the conventions already hold that a hand-written
  claim is exactly as valid as one of day's verbs.
