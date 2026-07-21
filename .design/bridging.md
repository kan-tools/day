# Feature: bridging — planning a path from here to a telos

## Summary

day can declare where a project is trying to get to, and what units of work
exist, but nothing connects them: there is no way to say *this arrangement of
atoms is how we get from here to that telos*, and no way to check whether it
would. This adds bridges — a planned arrangement of atoms aimed at a target
telos, checked for realizability the same way atom vocabularies are already
checked for composition.

It is the part of the model the rest of day exists to serve, and the part
with the least implementation. It has now been deferred three times in favour
of cheaper work, each time defensibly; this is the pass that stops deferring
it.

## The enabling move

Atom composition is checkable because atoms declare typed inputs and outputs.
Telos satisfaction is *not* checkable the same way, because a telos is a
weak-equivalence invariant rather than a type — many different concrete
world-states satisfy one equally, which is the whole point of holding it up to
weak equivalence.

The bridge: a telos may declare **witnesses** — artifact *types* that would
evidence it. Witnesses do not define the telos and do not collapse it to a
type. They say what kind of evidence would count, leaving open which concrete
instance provides it, which is exactly the weak-equivalence structure
`docs/TELOS.md` describes. With witnesses declared, realizability computes by
the same rule composition already does: **a plan reaches a telos iff the
outputs available at its terminal nodes cover that telos's declared
witnesses.**

## Requirements

- REQ-1: A telos may declare witnesses — a list of artifact type names — via
  an optional fenced `day-telos` JSON block on its `telos/<slug>` subject,
  written by `day telos declare --witness <type>` and read the same way atom
  interfaces are (`atoms::newest_fenced`). A telos without witnesses stays
  valid; it is simply not machine-checkable as a bridge target, which day
  says plainly rather than treating as an error.
- REQ-2: A `bridge/<slug>` subject holds a plan: a target telos and an
  arrangement of atom references, in a fenced `day-bridge` JSON block.
  Intermediate states remain ordinary `telos/<slug>` subjects — per
  `docs/TELOS.md`, a bridging state is just a telos at a shorter horizon, so
  the plan is the new object, not a new kind of state.
- REQ-3: A plan is a tree of nodes. A node is either an **atom reference**, an
  **all-of** node (every child must be traversed — sequential or parallel work,
  which are the same thing for realizability), or an **any-of** node (any one
  child suffices). `any-of` is how de-risking through separable parallel paths
  is expressed structurally rather than as a comment.
- REQ-4: `day bridge check <slug>` computes realizability: walking the plan,
  accumulating the artifact types available at each point, and reporting
  whether the target telos's witnesses are covered at the end. An atom whose
  declared inputs are not available where it sits is reported with the missing
  types named, the same shape `day doctor`'s findings already take.
- REQ-5: Artifact availability accumulates along a path and is never consumed
  — the same rule the atom-composition check already uses, and for the same
  reason: a design doc is still there when a review runs even though the build
  in between did not re-emit it.
- REQ-6: For an `any-of` node, only the types produced by **every** branch are
  available downstream. A route that might not be taken cannot be relied on to
  have produced anything. This is what makes alternatives genuinely different
  from parallel work rather than a relabelling of it.
- REQ-7: `day bridge declare <slug> --telos <slug> --plan <expr>` records a
  bridge, with the plan given in a small textual grammar (`a > b` for
  sequence, `a & b` for all-of, `a | b` for any-of, parentheses for grouping)
  rather than hand-written JSON — the same principle as `day atom declare`
  generating its own interface block.
- REQ-8: day does **not** track whether a plan's steps have happened. It
  checks whether an arrangement *could* reach a telos and stops. Progress
  tracking is where a process layer becomes a task tracker, and the smell
  test in `CLAUDE.md` warns hardest about exactly that.
- REQ-9: Realizability is reported as **frame-internal only**, and says so.
  `docs/TELOS.md` defines realizability as two-fold — frame-internal
  continuity, plus temporal coherence across frames at the orchestration
  layer — and the second is vacuous with one actor. day must not present a
  single-frame check as if it settled the global question; the output names
  the limit.
- REQ-10: `docs/CONVENTIONS.md` documents the `bridge/<slug>` convention, the
  `day-bridge` and `day-telos` blocks, and the plan grammar. `docs/TELOS.md`
  gains a pointer from its bridging section to what is now implemented.

## Acceptance Criteria

- [ ] AC-1: `day telos declare x "..." --witness published-artifact` records a
      claim whose `day-telos` block parses back to exactly that witness list;
      a telos declared without witnesses still records and still reads.
      (REQ-1)
- [ ] AC-2: `day bridge declare b --telos x --plan "design > build"` records a
      `bridge/b` claim whose `day-bridge` block parses back to a sequential
      plan of those two atom references naming target telos `x`. (REQ-2,
      REQ-7)
- [ ] AC-3: The grammar parses `a > b`, `a & b`, `a | b`, and
      `a > (b | c) > d` into the expected node trees, and a plan referencing
      an undeclared atom is refused with that atom named. (REQ-3, REQ-7)
- [ ] AC-4: `day bridge check` on a plan whose terminal outputs cover the
      target's witnesses reports it reachable and exits zero; one that does
      not names the uncovered witnesses and exits non-zero. (REQ-4)
- [ ] AC-5: An atom placed where its declared inputs are unavailable is
      reported with the missing input types named. (REQ-4)
- [ ] AC-6: A three-step plan where the middle atom does not re-emit an
      artifact the third needs still checks clean, proving availability
      accumulates rather than being consumed. (REQ-5)
- [ ] AC-7: For `a | b` where only `a` produces type `t`, a downstream atom
      requiring `t` is reported unsatisfied; where both branches produce `t`,
      it is satisfied. (REQ-6)
- [ ] AC-8: Checking a bridge whose target telos declares no witnesses
      reports that it cannot be machine-checked, names the telos, and exits
      zero — an undeclared witness list is a gap, not a failure. (REQ-1)
- [ ] AC-9: No day verb reports, stores, or infers whether a plan step has
      been completed — asserted by a test grepping the CLI surface for
      progress vocabulary, the same guardrail style used for blocking hooks.
      (REQ-8)
- [ ] AC-10: `day bridge check`'s output states that realizability is assessed
      within a single frame and that cross-frame coherence is not checked.
      (REQ-9)
- [ ] AC-11: `docs/CONVENTIONS.md` contains the `bridge/` prefix and both
      fence tokens, checked against the code's own constants the way existing
      prefixes already are. (REQ-10)

## Architecture

**`src/bridge.rs` (new)** holds the plan model (`Node`, `Plan`), the grammar
parser, and the realizability walk. The walk is a fold over the node tree
carrying a set of available artifact types: an atom reference checks its
inputs against the set and adds its outputs; an all-of node unions its
children's results; an any-of node **intersects** them (REQ-6). That
intersection is the only genuinely new rule in the whole feature, and it is
the one that makes alternatives mean something.

**`src/atoms.rs`** already provides `newest_fenced` and the fenced-block
convention; `Interface`'s serialization pattern extends directly to the two
new block types, so bridges and telos-witnesses reuse it rather than
introducing a third way to embed structure in a claim.

**`src/vocabulary.rs`** already handles declare-and-supersede for every
kan-backed subject, so `bridge declare` and the extended `telos declare` are
new callers of it, not new mechanisms.

**`src/cli/mod.rs`** gains a `bridge` group (`declare`, `check`), and
`telos declare` gains `--witness`. **`src/mcp.rs`** exposes `bridge_check`,
following `doctor` and `design_check`.

**The relationship to `day doctor`** is worth stating: `doctor` asks whether
the *vocabulary* composes — is this set of atoms internally coherent.
`bridge check` asks whether a *particular arrangement* reaches a *particular
target*. Same underlying rule about artifact availability, different question,
which is why the availability logic moves into `bridge.rs` and `doctor` keeps
its own narrower check rather than the two being forced into one function.

**Nothing here writes except the declare verbs**, which append through kan's
CLI like every other declaration. `bridge check` reads and prints.

## Resolved Questions

- **Teloi gain declared witnesses**, as artifact types, which is what makes
  realizability computable at all. Witnesses are evidence *types*, so they do
  not collapse a telos to a type and weak equivalence survives — many concrete
  artifacts of a declared type satisfy the telos equally.
- **day does not track plan progress.** It checks whether an arrangement could
  reach a telos and stops. Whether a step happened is already derivable from
  claims and artifacts existing, and answering "how far along are we" is the
  first question of a task tracker.
- **Alternatives are `any-of` nodes inside one plan**, not competing separate
  plans, so that de-risking is structural and the relationship between routes
  is explicit. The intersection rule for what an any-of node makes available
  downstream is what gives the distinction teeth.
- **`bridge/<slug>` holds the plan; intermediate states stay `telos/<slug>`.**
  `docs/TELOS.md`'s claim that a bridging state is just a shorter-horizon
  telos is preserved exactly: the plan is the new object, not the state.

## Out of Scope

- Cross-frame realizability — the sheaf-gluing half of `docs/TELOS.md`'s
  definition. Vacuous with one actor, and it belongs with frames in v0.5.
- Executing a plan. day describes and checks arrangements; running them is
  the agent's and the human's job.
- Progress, status, assignment, scheduling, or estimation of any kind.
- Inferring a plan automatically from a telos. Proposing an arrangement is a
  judgment, and it belongs in the `/design` command's interview, not in the
  binary.
- The polynomial-functor formalization of composition. Still bracketed; this
  pass should inform it rather than pre-empt it.
- Any change to kan.
