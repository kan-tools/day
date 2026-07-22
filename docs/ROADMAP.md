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
`day atom declare`, and baseline setup in `day init`).

Narrowings against what this section originally anticipated, each recorded as
a claim rather than quietly dropped:

- **No `revise` verb.** kan is append-only, so a revision is just a later
  claim; `declare` cites the prior one.
- **No read verbs.** kan's `show`/`status` plus `doctor` and
  `session_context` already cover reading.
- **No end-of-session hook.** This section originally promised one. It is not
  achievable: only `UserPromptSubmit`, `UserPromptExpansion`, and
  `SessionStart` add hook stdout to the model's context, and every
  end-of-session event writes to the debug log instead. The one mechanism
  that would deliver text at that moment is `Stop`'s blocking decision —
  which `telos/affordance-not-enforcement` forbids, making this a concrete
  instance of a tension previously recorded only in the abstract. The useful
  content (what is still open) moved into the **session-start** hook, where
  injection works and the agent can still act on it; `day hook session-end`
  remains as a command to run by hand.

Found by running `/adversarial-review` on the v0.2 diff — day's own atom, on
its own work. The verdict was **BLOCK**, and this is one of the two findings
that earned it.

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

## v0.3 — Bridging *(shipped)*

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

**Shipped in v0.3.0-beta.1**, with two amendments the build forced. The design
treated sequence and concurrency as equivalent for realizability; they are
not — `a > b` lets `b` use what `a` produced, `a & b` does not, and collapsing
them would accept plans whose steps depend on work that has not happened. And
a bridge needs to declare what is already available where it starts (`--have`),
or every source atom reports its inputs missing, since a source's inputs come
from outside the vocabulary by definition. Cross-frame realizability remains
out of scope, deferred to v0.5 with frames.

## v0.4 — Assessing *(in progress)*

Where "auditable, non-gameable" stops being an aspiration.

**Shipped: `day assess docs`** — the first assessment, built at the release
boundary because a tag and a published artifact are the most crystallized
substrate available. It gave day its second substrate (git, read-only) and
caught a real gap on its first run: releases were being recorded on `spine`
rather than on a `release` subject, so the tool reported a release nobody had
written down. One amendment the build forced: the evidence tier compares
*code changed* against *watched docs changed* rather than correlating claims
to the boundary, because `kan show` exposes no timestamps (kan-tools/kan#61) —
and the simpler comparison is a closer match to the failure it exists to
catch anyway.

**Next up — `day assess telos`.** The machinery it needs already exists:
teloi declare **witnesses** (v0.3), so assessing one means asking whether
those witnesses were actually produced and by what evidence. `bridge check`
already computes whether a plan *could* reach a telos; assessment asks
whether it *did*, and the gap between those two is the design question.

Also newly unblocked: kan v0.6 shipped `RelationKind::InTensionWith`
(kan#60), so `day telos tension` can emit a real edge instead of a prose
claim (day#18). This repo's own log holds several prose tensions already, so
it is its own migration case — decide deliberately what happens to them
rather than rewriting; they are real claims.

**What was still to come in v0.4 now lands in v0.5**, alongside the affordance
work that section explains — `day assess telos` and the tension edge move
there rather than stretching this milestone further.

Two items are deferred past v0.5 outright:

- **Drift detection** — teloi that shifted without the shift being recorded;
  unstated teloi inferred from what the work actually optimized; trade-offs
  made silently between teloi in tension. Deferred because it is more model,
  and the v0.5 meta-evaluation found that model is already outrunning use. It
  waits for evidence from `assess telos` about what drift here actually looks
  like.
- **The adversarial-review atom growing into assessment** rather than staying a
  standalone command. Worth doing, but it should follow a version in which
  assessment has actually been run more than once.

## v0.5 — Used, not just built

**This section replaced "Frames", which moved to v0.6.** The change came out of
a meta-evaluation of the first five milestones, recorded on `process-model`
with a **REDIRECT** verdict. The evidence, from git and the log rather than
recollection:

**Adoption tracks affordance.** Every capability with a slash command is used
every milestone — `/design` produced six design docs, one per milestone with no
exceptions; `/adversarial-review` produced three verdicts, one of which
(**BLOCK** on `vocabulary-verbs`) actually stopped a release. Every capability
that is a bare CLI verb was used **once**, by its author, in the milestone that
built it: `day bridge check` ran for `bridge/v0.3` and has not run since;
exactly one milestone telos has ever been declared. The `release` subject holds
two claims against five git tags.

The consequence is that day's own log cannot reconstruct what v0.4 was aiming
at — no target telos, no bridge. `telos/legible-process` is failing on the
repo that defines it, traded silently against `telos/composable-process`, which
is exactly the unrecorded trade-off `/adversarial-review` names as its most
common real finding.

So this release ships **affordances for what already exists** rather than more
model:

- **Finish v0.4** — `day assess telos`, the tension edge (day#18), and the two
  assessment atoms. The loop closes here: assessing `telos/legible-process`
  against witnesses is the check that asks whether a milestone had a declared
  target at all.
- **Hang planning off the atom that already has adoption.** `/design` runs
  every milestone without exception. If a design pass *ends* by declaring the
  milestone's target telos and its bridge, bridges get declared because designs
  get written — adoption is inherited rather than depending on someone
  remembering a bare verb. The highest-leverage item here, and the cheapest.
- **Repo-defined prompt injection** (day#25). Load-bearing rather than a
  nicety: `PRACTICE` asks for exactly one behavior — "name which telos it
  serves" — and that is the one declared thing that recurs. Injected text
  determines what gets used. Gated on the trust question in day#25, which must
  be settled before an injection path sourced from claims can ship.
- **Record the gaps; do not backfill them.** v0.1 through v0.4 had no declared
  targets. That absence is recorded as an observation and left visible.
  Fabricating teloi and bridges after the fact would corrupt the substrate the
  legibility telos depends on, to make a chart look better.

**Deliberately excluded: drift detection**, though it is v0.4's remaining
roadmap item. It is more model, which is the trap under review. It waits until
`assess telos` has run against real milestones and there is evidence about what
drift here actually looks like.

**The honest limit on the evidence:** n is small. `bridge` is 0-for-2 and
`assess docs` has had no release since it shipped. This is a consistent
direction, not an established law, and it should be re-checked after v0.5
rather than treated as settled.

## v0.6 — Frames

Multi-actor, and paced by kan's own sync work (a frame only bites once there is
more than one actor with more than one log). **Moved here from v0.5** by the
meta-evaluation above: with one actor, shipping frames would build a model
nobody exercises, which is the pattern that earned the REDIRECT. The roadmap's
own pacing argument already said as much.

- Frames as internal toposes: an assessment is a certificate valid inside some
  actor's own logic, not an absolute fact.
- Cross-frame reconciliation relative to a base frame, with the shared
  reality-regularities substrate (code compiles, tests pass, the artifact
  exists) as the floor every frame factors through.
- Genuine incommensurability held as irreconcilable rather than force-merged.
  Rare in practice, but the honest failure mode has to exist.
- **No longer blocked.** This section previously recorded a modeling gap in
  kan: `RelationKind` had no edge for "in tension with", leaving tension
  between teloi as prose rather than something queryable. kan#60 shipped that
  variant, and day#18 lands the edge in v0.5, so frames arrives with tension
  already queryable rather than needing it first.

## v0.7 — Atom library and meta-evaluation

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
