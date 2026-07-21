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
- day talks to kan by **shelling out to the `kan` binary**, never by linking
  it as a library. The boundary is the public CLI on purpose: it's the same
  contract any other consumer gets, so day can't quietly depend on kan
  internals.
- day is a **reader**. It runs kan's read verbs only; it never appends,
  retracts, or rejects a claim. Claims get recorded by *instructing* an agent
  to call kan's write verbs (that's what the commands do), never by day
  calling them itself. This is what makes it structurally impossible for day
  to alter or destroy a subject. (Caveat worth stating precisely rather than
  papering over: kan initializes a `.kan/` workspace on first use, so a day
  read in a repo kan has never seen creates an empty log there. No claim is
  touched, but it isn't literally zero side effects.)
- Correctness before features. The atom composition check should be boring
  and obviously right.
- Keep the CLI small. Four verbs today. A new verb needs a design doc.
- `DAY_KAN_BIN` exists so tests can stub kan. Every integration test uses it;
  none require a real kan install.

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
