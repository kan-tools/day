# Roadmap

Draft, and deliberately a *plausible fiction* in Spivak's sense rather than a
schedule: it starts where day actually is, ends somewhere worth going, and the
path between is meant to be plausible enough to draw work forward. Later
versions are sketchier than earlier ones on purpose. Revise it by editing this
file and recording the change in kan (`--subject roadmap`), never by pretending
it always said something else.

## Where we are

**v0.1.x — the reader.** day folds and reports; it does not write. Teloi and
process atoms are hand-typed kan claims, atom interfaces are fenced JSON blocks
written by hand. Four verbs (`init`, `doctor`, `hook`, `mcp`), two commands
(`/design`, `/adversarial-review`), one hook (session-start), and the v0
conventions in `CONVENTIONS.md`.

Both real bugs found so far came from using day on day, not from testing it.
That is the strongest signal available about how to sequence what follows:
**prefer the version that gets day used daily over the version that adds the
most model.**

## v0.2 — Declaring *(complete)*

Two passes: `.design/design-atom-backing.md` (`day design check|record`, `day
review record`, `day next`, the `schema/<slug>` convention, and day's first
write path) and `.design/vocabulary-verbs.md` (`day telos declare|tension`,
`day atom declare`, baseline setup in `day init`, and the session-end hook).

Three narrowings against what this section originally anticipated, each
recorded as a claim rather than quietly dropped: **no `revise` verb** (kan is
append-only, so a revision is just a later claim), **no read verbs** (kan's
`show`/`status` plus `doctor` and `session_context` cover reading), and the
session-end hook reports **what is open** rather than what changed this
session — day has no store and so no session state, and acquiring one would
trade `telos/no-store-of-its-own` for a reminder.

The friction blocking daily use is that recording a telos or an atom means
hand-writing a fenced JSON block inside a claim. That friction suppresses
recording, and suppressed recording undermines `telos/legible-process`
directly.

- `day telos` — declare, list, show, revise. A revision appends a claim citing
  the one it supersedes; nothing is rewritten.
- `day atom` — declare, list, revise, with the interface as flags rather than
  hand-written JSON. `day doctor`'s composition check moves from
  after-the-fact to at-declaration-time.
- **day becomes a writer, through kan's CLI only.** It shells `kan
  decide`/`observe`/`result`; kan still signs, content-addresses, and owns the
  log format. day never touches storage directly.
- Harness: a SessionEnd or equivalent hook prompting for what should be
  recorded before context is lost.

**The invariant, restated honestly.** "day is a reader" was always a proxy for
the guarantee that actually matters: *day cannot alter or destroy a subject.*
Appending through kan's own append-only, signed verbs preserves that guarantee
exactly — there is no destroy path in kan to reach. What day must never do is
write to kan's storage directly, bypass its signing, or acquire a store of its
own. `CLAUDE.md` gets updated to say that, rather than keeping a rule that
sounds stronger than it is while being quietly worked around.

This is also the first real test of the conventions. Expect them to change
under use; that is the point of doing this before anything is built on top.

## v0.3 — Bridging

Decomposition: the part of the model with the most substance and the least
implementation.

- Bridging states as intermediate teloi in time — telescoping shorter horizons
  into longer ones.
- Planning a path from here to a telos as an atom diagram: sequential and
  parallel composition, not a fixed linear pipeline.
- De-risking through separable parallel paths — several independent routes to
  the same invariant, so one failing does not sink the bridge.
- Realizability, checked the two ways `TELOS.md` describes: frame-internal
  continuity, and temporal coherence across frames at the orchestration layer.
  With one actor the second is trivial, which is exactly why this ships before
  frames.

Highest risk of rework in the whole roadmap. Ship it against real work
(day's own, and kan's) rather than against a fixture.

## v0.4 — Assessing

Where "auditable, non-gameable" stops being an aspiration.

- `day assess` — did work land inside a telos's equivalence class? Judged
  against material evidence (builds, tests, diffs, artifacts), never against an
  agent's own account of what it did.
- Drift detection: teloi that shifted without the shift being recorded;
  unstated teloi inferred from what the work actually optimized; trade-offs
  made silently between teloi in tension.
- The adversarial-review atom grows into this rather than staying a standalone
  command — review becomes one way of producing an assessment.

## v0.5 — Frames

Multi-actor, and paced by kan's own sync work (a frame only bites once there is
more than one actor with more than one log).

- Frames as internal toposes: an assessment is a certificate valid inside some
  actor's own logic, not an absolute fact.
- Cross-frame reconciliation relative to a base frame, with the shared
  reality-regularities substrate (code compiles, tests pass, the artifact
  exists) as the floor every frame factors through.
- Genuine incommensurability held as irreconcilable rather than force-merged.
  Rare in practice, but the honest failure mode has to exist.
- Blocked on a modeling gap in kan, not day: `RelationKind` has no edge for
  "in tension with", so tension between teloi is currently prose rather than
  something queryable. That needs a new `RelationKind` variant, which by
  ADR-18's rule is kan's to own. Recorded on the `process-model` subject.

## v0.6 — Atom library and meta-evaluation

- The remaining atoms from `TELOS.md`: user testing, structured research and
  data extraction, formal verification before build-out, external comms and
  human operator assignment.
- Meta-evaluation as a first-class flow: revising the atom vocabulary is itself
  an atom, and the process framework co-evolves with the project's teloi.
- Non-Claude-Code harnesses. The CLI core is already harness-agnostic, so this
  is packaging rather than redesign.

## v1.0 — Someone else ships with it

**The bar: a person who is not the author uses day on a project that is neither
kan nor day, and the record is legible to a third party afterward.**

Chosen over model-completeness or the formalism landing, because those can both
be satisfied by a tool only its author can operate. Conventions invented by one
person survive contact with that person indefinitely; the useful signal is what
happens when they meet someone who did not invent them.

Implies, without needing separate milestones: install and onboarding that work
without the author present, conventions documented well enough to follow
without reading the theory, and error messages that teach the model rather than
assume it.

## Cross-cutting, every version

**Harness surface grows with the features that need it** rather than as its own
milestone: each release adds the hook events its own capabilities require, and
the injected working-practice text gets refined as evidence accumulates about
what actually changes behavior versus what just consumes context budget.
Advisory always — a hook that can block is a different tool.

**Dogfood first.** Every feature gets used on day or kan before it is called
done. The track record so far is that this finds more than testing does.

## Bracketed, not scheduled

The polynomial-functor formalization of atom composition — positions and
directions, systems-as-interfaces, composition as lens composition. It is a
good telos for this project in its own right, and it should inform v0.3's
design rather than block it. If the conventions end up approximating the math
badly, that is the signal to promote this from bracketed to scheduled.
