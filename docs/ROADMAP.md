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

### What is left before v0.5 releases

`day assess telos`, the tension edge (day#18), the kan-conformance test
(day#27), and the open-question miscount (day#9) have all landed. What remains
is ordered by one rule: **a capability that reports something false is worse
than one that does not exist**, because v0.5's whole thesis is getting shipped
capabilities *used*, and a tool caught lying once stops being used.

1. **day#34 — witness probes are per-project, satisfaction is often
   per-telos.** `telos/v05-shipped` says "day v0.5 is published" and the
   assessment reports its witness satisfied by **v0.4's tag**. A false positive
   in the headline case of the feature v0.5 exists to ship. day names the
   instance it matched, so the evidence shown is honest where the label is not
   — useful, not right. Blocking: releasing v0.5 with this is releasing a
   capability whose first real use misleads.
2. **day#32 — tension prose displaces the telos statement** in injected
   session context. Four of six teloi affected. It degrades the record at
   exactly the moment the record is read into a session, which is the moment
   `telos/legible-process` is about. The fix the tension edge now makes
   available is to move the *why* onto a `tension/<a>--<b>` subject.
3. **day#30 — operational-safety practice in the injected context.** A real
   incident: credentials swept into a commit by a blanket `git add -A` chained
   with a push, and a force-push that GitHub still served by SHA afterwards.
   The injected practice is entirely about process discipline and says nothing
   about this. Cheap, because day authors the text itself — it needs none of
   day#25's trust machinery, and should not wait behind it.
4. **day#25 — repo-defined prompt injection.** Still gated on the trust
   question: an injection path sourced from claims is inert with one signer
   and is not once kan sync lands. That decision comes before the code.

Both day#34 and day#32 are conventions changes — they alter the `day-telos`
block's shape and where a tension's reason lives — so each wants a design pass
rather than a patch, and each revises a REQ already recorded in kan.

### Not in v0.5, and why

- **day#36** — `day design record` has no incremental mode, so iterating on a
  design doc re-appends every decide already recorded. Needs a stable id per
  resolved question (`RQ-`, matching how `REQ-` and `AC-` already work), which
  is a `schema/design-doc` change and its own design pass. The workaround —
  appending the delta by hand — is legitimate under the conventions.
- **day#20** — empty `any`/`seq` nodes in a hand-written bridge plan are
  silently permissive. Real, narrow, and no one has hit it.
- **day#19** — the repo is private, so `/plugin install` works for nobody
  else. A decision rather than code, and it blocks the **v1.0** bar directly:
  "a person who is not the author uses day on a project that is neither kan
  nor day" is unreachable while the repo cannot be installed from.

### Upstream, in kan

- **kan#78** — `kan result` takes its subject positionally while
  `observe`/`plan`/`decide` take `--subject`. The asymmetry that put a
  non-running command into `docs/CONVENTIONS.md` for several releases. Filed
  as a question with four options; day is not blocked either way, because
  day#27's conformance test now catches the class.
- **kan#61** — `kan show` exposes no artifacts or anchors. Still the reason
  day took git as a second substrate, and it would materially improve day#34:
  with anchors, day could tell that a tag predates a telos and downgrade a
  false positive to a warning.

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
done. The track record is not that this finds *more* than testing — it is that
it finds a **different class**, and that class has contained every real defect
so far.

The v0.5 session is the sharpest evidence yet, because day was used to build
day throughout and the tool found faults in itself:

- Using `/design` end to end exposed that the open-question counter counted
  the marker the template tells you to quote, so every design doc with
  unresolved questions was miscounted (day#9). Four releases of design docs
  had never carried an unresolved block through the checker, so no test could
  have hit it.
- Running `kan result` by hand exposed that `docs/CONVENTIONS.md` documented a
  command that does not run — and that a design doc had already inherited the
  wrong form into a requirement to *print it for the reader* (day#27, kan#78).
- Running `day assess telos` on day's own log, before the feature was
  committed, exposed two bugs (a raw JSON block rendered as the telos
  statement; a witness declaration read as prose asserting its own success)
  and one design limitation that is now blocking the release (day#34).
- Reading day's own injected session context exposed that tension prose had
  displaced four of six teloi (day#32).
- Folding a bridge and a target telos into the milestone exposed that neither
  had existed for four consecutive milestones, which produced the REDIRECT
  that reordered this roadmap.

Two patterns worth naming, because they recur:

**A check that only inspects its own side of an interface will miss the
interface.** It has now happened between day and the harness (a hook on an
event that reaches nobody), between day and kan (a stub that accepts any
argument shape), and between day and *itself* (a test constructing an open
block the command never emits). The last is the most instructive: both sides
were in this repo.

**A finding recorded where it is convenient does not reach the thing that
would act on it.** kan#78 sat as a sentence inside another issue's body;
day#36 sat in the kan log with no issue. Both surfaced only when someone
asked. Record the finding *and* file it where the work happens.

## Bracketed, not scheduled

The polynomial-functor formalization of atom composition — positions and
directions, systems-as-interfaces, composition as lens composition. It is a
good telos for this project in its own right, and it should inform v0.3's
design rather than block it. If the conventions end up approximating the math
badly, that is the signal to promote this from bracketed to scheduled.
