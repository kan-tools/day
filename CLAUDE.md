# day — agent working notes

You are building `day`: the structured **process** layer that sits next to
`kan`'s structured **knowledge** layer. This file orients you; `docs/` is
authoritative.

## Read first, in order
1. `README.md` — what day is and what it does today.
2. `docs/TELOS.md` — the model: teloi as weak-equivalence invariants, frames
   as internal toposes, bridging states, atoms and composition. This is
   theory, and most of it is not implemented. Do not treat unimplemented
   theory as a backlog to burn down.
3. `docs/CONVENTIONS.md` — AUTHORITATIVE for what day actually reads and
   writes. If TELOS.md and CONVENTIONS.md disagree about the shipped tool,
   CONVENTIONS.md wins.
4. `docs/ROADMAP.md` — where this is going and why in that order. A draft,
   not a schedule; revise it by editing it and recording the change in kan
   (`--subject roadmap`), never by pretending it always said something else.

## The two non-negotiables

**day stores nothing of its own.** Every durable thing day knows is an
ordinary kan claim, read back through kan's public CLI. No config store, no
sidecar database, no state file. If a feature seems to need day-owned
persistent state, that is a signal the feature is wrong or belongs in kan —
stop and reconsider before adding a store.

**Advisory, never blocking.** Hooks inject context; they never gate, deny, or
reject an agent's action. `tests/plugin.rs` enforces this against the shipped
hook config and is not to be weakened. This is a direct lesson from
`crosslink`, whose blocking hooks caused the integration friction that
motivated splitting day out of kan in the first place.

## Boundary with kan

kan owns a feature iff it needs a new/existing `ClaimBody`/`ClaimKind`/
`Anchor`/`RelationKind` variant, or is a pure read/fold over the claim graph
(kan's ADR-18). day owns everything buildable as a calling convention over
kan's existing primitives — process, orchestration, multi-turn interaction.

If a day feature would require changing kan's data model, that is a kan
design question first, raised as a kan issue, not something to work around
here.

## House rules

- Rust, matching kan's dependency choices where they overlap (clap, rmcp,
  serde, thiserror, tokio) so the two crates stay easy to read together.
- day has **two substrates**: kan, and (since v0.4) **git, read-only**. All
  git access lives in `src/git.rs`, restricted to read subcommands, with a
  test whitelisting them — day never stages, commits, tags, or pushes. git
  was taken on reluctantly, because kan does not expose claim artifacts
  (kan-tools/kan#61); if that changes, prefer reading the record.
- day talks to kan by **shelling out to the `kan` binary**, never by linking
  it as a library. The boundary is the public CLI on purpose: it's the same
  contract any other consumer gets, so day can't quietly depend on kan
  internals.
- **day writes only through kan's public CLI.** As of v0.2 day appends
  claims (`day design record`, `day review record`), but always by invoking
  `kan observe`/`plan`/`decide` as a subprocess — never by touching kan's
  storage, its signing, or its log format. kan remains the only thing that
  decides what a claim *is*.
  - The guarantee that matters is unchanged: **day cannot alter or destroy a
    subject.** It only ever appends, and kan exposes no destroy path to
    reach. Earlier versions said "day is a reader", which was a proxy for
    this; the proxy stopped being true in v0.2 and the real invariant is
    stated directly rather than worked around.
  - What day must never do: write kan's files directly, bypass its signing,
    or keep a store of its own. If a feature seems to need any of those,
    it is wrong or it belongs in kan.
  - day never retracts or rejects. Superseding is done by appending, the
    same way kan does it.
  - (Caveat, stated precisely rather than papered over: kan initializes a
    `.kan/` workspace on first use, so even a day *read* in a repo kan has
    never seen creates an empty log there.)
- Correctness before features. The atom composition check should be boring
  and obviously right.
- Keep the CLI small. Four verbs today. A new verb needs a design doc.
- `DAY_KAN_BIN` exists so tests can stub kan. Every integration test uses it;
  none require a real kan install.
  - **One deliberate exception: `tests/kan_conformance.rs`.** A stub accepts
    whatever day sends it, so stub-based tests validate day against day's own
    idea of kan's CLI, never against kan's contract — which is how
    `docs/CONVENTIONS.md` documented a `kan result` invocation that does not
    run, through several releases (day#27, kan#78). That file talks to the
    real binary and **skips when kan is absent**, so the rule above still
    holds. Its hermetic half — that `Write::new` is only ever used with the
    verbs whose subject is a flag — always runs, and is the part that
    actually protects the invariant.

## Dogfood before you trust a test

**Every defect found in day so far came from running it, not from testing
it.** Not most — every one. The composition check's false positive on day's
own atoms, a telos rendering its tension instead of itself, `--title`
silently discarding a title, a session hook wired to an event that cannot
reach the model, `bridge check` reporting the wrong set, a retracted telos
still listed as in play. All seven were invisible to a green suite, because
tests assert day's *output* while the defects were in what that output means
or whether anything receives it.

So: before calling a feature done, use it on this repo or on kan against the
real log. A passing suite is necessary and has never been sufficient here.

Two corollaries worth keeping:
- A check that only inspects its own side of an interface will miss the
  interface. Verify what the *other* side does — read the harness docs, call
  the MCP tool, install the published crate.
- Probes against a real log leave real claims. Use a scratch repo, or retract
  in the same breath. An assessment that pollutes the record it assesses is
  measuring its own footprint.

## Working practice

- Design goes through `/design` and lands in `.design/<slug>.md` before
  implementation, recorded into kan.
- Post-implementation, run `/adversarial-review` against the design doc.
  Both commands are day's own atoms — dogfood them.
- One PR per milestone: branch off `main`, commit, push, `gh pr create`,
  wait for CI, then `gh pr merge --merge --delete-branch` (regular merge, so
  the milestone's internal commits stay visible).
- Record durable findings and decisions into kan as you go, citing the claims
  they build on. `--cites` takes **CIDs of prior claims, never file paths** —
  capture the CID a write verb prints and chain it.
