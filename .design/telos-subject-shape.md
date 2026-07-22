# Feature: What a telos subject carries

## Summary

Two v0.5 blockers — day#32 and day#34 — are the same question asked from
opposite sides: **what belongs on a `telos/<slug>` subject?** day#32 says a
tension's reason should not be there, because it displaces the telos statement
everywhere day renders one. day#34 says a witness *scope* should be there,
because a project-level probe map cannot tell v0.5's `published-artifact` from
v0.4's. Designing them separately means the second pass revises the first's
decisions about the same fenced block, so this is one pass.

## Motivation

**day#32.** `src/hooks.rs` renders each telos as its newest claim carrying
text. For any telos with a recorded tension, that claim is the tension. Four
of six teloi are affected today, so session-start injects tension prose where
the telos statement belongs, and `day assess telos` shows the same thing. The
statement — the actual invariant the work is meant to serve — is two claims
older and never displayed.

This was expected to fix itself when tension became a `RelationKind` edge, on
the grounds that relations carry no narrative body. That reasoning was wrong
and self-contradictory: the edge was added *alongside* the prose claim,
because the reason has to live somewhere, and `hooks.rs` selects the newest
claim *with* text. Verified still broken after all four edges were back-filled.

**day#34.** `telos/v05-shipped` says "day v0.5 is published". Its witness
`published-artifact` resolves through the project map `{"tag": "v*"}`, which
matches `v0.4.0-beta.1`. The assessment reports `[MATERIAL]` — a false
positive in the headline case of the feature v0.5 exists to ship. day names
the instance it matched, so the evidence shown is honest where the label is
not: useful, and not right.

Both are cheap to live with and corrosive to leave. v0.5's thesis is that
shipped capabilities get *used*, and a tool caught reporting something false
once stops being used.

## Requirements

- REQ-1: A tension's reason lives on a `tension/<a>--<b>` subject, not on
  either telos subject. `day telos tension` writes the reason there and emits
  the two `in-tension-with` edges, which cite it. The verb's argument surface
  does not change.
- REQ-2: The tension subject's slug is the two telos slugs in **sorted**
  order, so `day telos tension b a` and `day telos tension a b` name the same
  subject. Recording a tension twice in opposite orders must not produce two
  subjects describing one relationship.
- REQ-3: The tension subject carries a fenced `day-tension` block naming the
  two teloi it relates. day finds tensions by reading that block, never by
  parsing the slug — a slug is an rkey, and telos slugs contain `-`
  themselves, so `tension/foo-bar--baz` is not reliably decomposable.
- REQ-4: With the reason no longer on the telos subject, day surfaces it where
  a reader needs it: `session_context` and `day assess telos`'s record tier
  read `tension/*` and report which teloi a telos pulls against, and why.
  Moving information must not make it unfindable.
- REQ-5: After REQ-1, the newest text claim on a telos subject is its
  declaration again, so `src/hooks.rs` and `src/telos.rs` render the statement
  rather than commentary about it. Neither module needs a heuristic to tell
  them apart.
- REQ-6: A telos may declare a **scope** per witness type in its `day-telos`
  block: `{"witnesses": ["published-artifact"], "scope": {"published-artifact":
  "v0.5*"}}`. The scope narrows *which instances count*; the project map on
  `schema/witness` still decides *which kind of probe runs*.
- REQ-7: A scope replaces the pattern argument of a `path` or `tag` probe,
  keeping its kind. `{"tag": "v*"}` scoped by `v0.5*` evaluates as
  `{"tag": "v0.5*"}`.
- REQ-8: A scope on a witness whose project probe is a `command` is **ignored
  and reported**, never applied. Honouring it would let a telos claim
  determine what executes, which is precisely the widening the day#34 decision
  rejected — a command must originate only from `schema/witness`.
- REQ-9: A witness with no scope behaves exactly as it does today, so every
  existing `day-telos` block stays valid. This is an additive convention
  change.
- REQ-10: `day telos declare` accepts `--scope <witness>=<pattern>`
  (repeatable) and generates the block. As with every other block, it is never
  hand-written.
- REQ-11: `docs/CONVENTIONS.md` documents the `tension/<a>--<b>` subject, the
  `day-tension` block, and witness scoping; the sections it supersedes are
  rewritten rather than left describing the old shape.
- REQ-12: The four tensions already recorded as prose on telos subjects are
  **not rewritten or retracted**. They are real claims. See Q1 for what
  happens to them.

## Acceptance Criteria

- [ ] AC-1: `day telos tension a b "why"` creates `tension/a--b` carrying the
      reason, emits two edges citing it, and leaves no reason-bearing claim on
      either telos subject. (REQ-1)
- [ ] AC-2: `day telos tension b a "why"` writes to `tension/a--b`, the same
      subject the forward order produces. (REQ-2)
- [ ] AC-3: The tension subject carries a `day-tension` block naming both
      teloi, and day locates the tension by reading that block from a subject
      whose slug it never parses. (REQ-3)
- [ ] AC-4: `session_context` and `day assess telos` both name the teloi a
      given telos is in tension with, and the reason, sourced from the tension
      subject. (REQ-4)
- [ ] AC-5: For a telos with a declaration and a tension, `day hook
      session-start` renders the declaration, not the tension — asserted
      against the exact failure day#32 records. (REQ-5)
- [ ] AC-6: A telos declaring `scope` for a witness produces an assessment
      that probes the scoped pattern; the same telos without the scope probes
      the project pattern. (REQ-6, REQ-7)
- [ ] AC-7: Given `schema/witness` mapping a witness to a `tag` probe and a
      telos scoping it to a pattern matching no tag, the witness reports
      `MISSING` where the unscoped probe reported `MATERIAL` — the day#34
      false positive, inverted into a regression test. (REQ-7)
- [ ] AC-8: A scope on a `command`-probed witness leaves the executed argv
      byte-identical to the unscoped one, and the report says the scope was
      ignored and why. Asserted with a stub whose argv is recorded. (REQ-8)
- [ ] AC-9: Every `day-telos` block written before this change parses and
      assesses identically after it. (REQ-9)
- [ ] AC-10: `day telos declare v --scope w=p` generates a block that
      round-trips through day's own parser. (REQ-10)
- [ ] AC-11: `docs/CONVENTIONS.md` contains the tension prefix and the
      `day-tension` fence token, checked against the code's own constants, and
      no longer instructs recording a tension's reason on a telos subject.
      (REQ-11)
- [ ] AC-12: The four existing prose tension claims remain live with their
      original CIDs after the change and any migration. (REQ-12)

## Architecture

**`src/tension.rs` (new)** holds the `TENSION_PREFIX`, the `day-tension`
block, the canonical slug rule from REQ-2, and the read that answers "what is
this telos in tension with, and why". It is the same shape as
`src/telos.rs` and `src/docs.rs`: a fenced block, a loader over
`atoms::newest_fenced`, and a render. `src/cli/mod.rs`'s `TelosAction::Tension`
arm becomes a caller of it rather than carrying the logic inline, which it
does today.

**`src/bridge.rs`** owns the `Witnesses` type and the `day-telos` fence, so
`scope` is added there rather than in a second definition. It is deserialized
with `#[serde(default)]`, which is what makes REQ-9 hold without a version
field: an old block simply has no scope.

**`src/telos.rs`** gains the narrowing from REQ-7 — a function taking a
`Probe` and an optional scope and returning the effective probe. Keeping it
there rather than in `src/probe.rs` matters: `probe.rs` is the module the
no-shell guardrail greps, and scoping is a policy decision about which
instances count, not a change to how a probe executes. Its record tier also
gains the tension report from REQ-4.

**`src/hooks.rs`** stops needing the comment at `render_teloi` explaining that
the newest claim is often commentary rather than the telos. After REQ-1 it is
not, and the comment should go rather than be left describing a hazard that no
longer exists.

**Nothing retracts.** The migration in Q1 is append-only whatever is chosen,
because kan exposes no destroy path and day would not reach for one if it did.

## Resolved Questions

- **One design pass, not two.** day#32 and day#34 both change what a telos
  subject carries and both touch the `day-telos` block. Sequencing them means
  the second revises the first's decisions about the same structure.
- **Scoping narrows, it does not override.** The project map keeps deciding
  which probe kind runs, so a telos claim can never introduce a command probe.
  Recorded on `assess-telos` when day#34's shape was chosen; restated here
  because REQ-8 is where it becomes code.
- **The tension subject's slug is canonical and sorted**, so one relationship
  has one subject regardless of the order the arguments were typed in.
- **day finds tensions by their fenced block, not by parsing slugs.** Telos
  slugs contain hyphens, so a `<a>--<b>` slug is not reliably decomposable;
  the block makes the slug a name rather than a data structure.
- **Additive, not versioned.** `scope` is `#[serde(default)]`, so every block
  written before this change keeps working and no migration is needed for the
  witness half.
- **The four existing prose tensions are migrated, not left and not
  re-declared.** Their reasons move to `tension/<a>--<b>` subjects cited by
  the edges that already exist, and the original claims stay live and
  unmodified as superseded context. Chosen over re-declaring the four teloi —
  which would fix day#32's rendering symptom while leaving the reasons in the
  old shape — because a repo that documents a convention it does not itself
  follow is the drift day exists to surface. This makes day's own log the
  worked example of the shape.
- **Scope lives on the telos, not on the bridge.** The objection considered
  was that `v0.5*` smuggles an instance into something defined up to weak
  equivalence. It does not: `v0.5*` still admits `v0.5.0-beta.1`, `v0.5.0`,
  `v0.5.1` and more, so it names a **narrower equivalence class, not a
  point** — and `telos/v05-shipped` genuinely is about a narrower class than
  "day is published". Putting it on the bridge would also leave a
  bridge-less telos nowhere to declare a scope, and would couple assessment
  to planning by making `assess telos` find a bridge before it could assess a
  telos.
- **Noted but not acted on:** the question surfaced that milestone-shaped
  teloi and invariant-shaped teloi behave differently — `v05-shipped` wants a
  scope, `legible-process` never will — and `docs/TELOS.md` does not
  distinguish them. Recorded rather than resolved here; blocking day#34
  behind a theory question is the trade v0.5 exists to stop making.

## Out of Scope

- **Drift detection.** Still deferred, still more model.
- **Making `kan show` expose cites**, which would let a reader find a tension's
  reason from the edge alone without day's help. That is kan's (kan#61 is the
  neighbouring gap) and REQ-4 makes day's own surfaces sufficient meanwhile.
- **day#36's incremental recording.** This design doc will hit it: resolving
  Q1 and Q2 adds bullets to Resolved Questions, and re-running `day design
  record` would duplicate the chain. The workaround stays hand-appending the
  delta.
- **Any change to how a probe executes.** The four guardrails in
  `src/probe.rs` are untouched; scoping happens before a probe is evaluated.
