# Roadmap

Draft, and deliberately a *plausible fiction* in Spivak's sense rather than a
schedule: it starts where day actually is, ends somewhere worth going, and the
path between is meant to be plausible enough to draw work forward. Later
versions are sketchier than earlier ones on purpose. Revise it by editing this
file and recording the change in kan (`--subject roadmap`), never by pretending
it always said something else.

## Where we are

**Shipped: `v0.9.0-beta.1`. Merged and unreleased: v0.10.** day declares, reads,
assesses and records. Eleven
verbs, two commands (`/design`, `/adversarial-review`), three hooks, a status
line, and three substrates (kan; git read-only; project-declared commands). The
v0.7 line delivered honest reads (`beta.2`) and declared vocabularies
(`beta.3`); `v0.8.0-beta.1` is the bulk read (day#71); `v0.9.0-beta.1` is
position reports that can represent their own failure (day#98/#97/#103/#105/#111)
plus the material/record witness pair.

**The version number and the milestone name below have come apart, and this is
the honest record of it.** "v0.8 — transportable" in the table further down is a
*milestone*: vocabulary packs, day#73. The **release** `v0.8.0-beta.1` is not
that milestone. It took the 0.8.0 number because day#71 raised day's minimum kan
to 0.9.1 — a breaking requirement change that `0.7.0-beta.4` would have
understated — and semver answers to what breaks, not to what a roadmap section
is called. Packs remain unbuilt and will ship under whatever number is current
when they do. Recorded rather than reconciled by renaming, because renaming
would lose the reason.

Every defect found in day so far came from using day on day, not from testing
it. That is still the strongest signal available about how to sequence what
follows: **prefer the version that gets day used daily over the version that
adds the most model.**

### The next three, and why in that order

Tracked as GitHub milestones; the issue numbers are the contract, this is the
reasoning.

**v0.10 — the graph tells the truth. *Merged (#122); not yet released.*** `next`
is forward-only and a **guaranteed DAG**; feedback lives in `revisits`. The
constant false "a step was skipped" warning is gone, and reachability,
topological ordering and partial-order reporting went from unavailable to
trivial. day#113/#115/#112/#108/#95 all closed.

Two things it turned out to be about, neither of which this section predicted:

- **day#115 was not the generalisation it was filed as.** Two live defects —
  an assessment rendering as the reason two teloi pull apart, and another
  injected into every session as working practice. Filed as "the concrete
  instances are fixed, the rule remains"; the rule was missing from two modules
  the earlier fix never reached.
- **The milestone's real yield was in the verifiers.** A cold review found the
  DAG source scan could be evaded by the multi-line chain `cargo fmt` *itself
  produces* — and `cargo fmt --check` gates CI, so an offending consumer would
  have been formatted into the evading shape automatically, while the scan
  reported clean. Separately, the kan-read scan knew four swallow shapes and
  there were six, with a live unmarked instance. Both scans' escape hatches were
  per-file and are now per-site.

That is the third consecutive milestone where an independent review found
defects the author could not, and the severity has moved from code, to
fixes-that-introduced-bugs, to tests, to **the scans that check the tests**.
v0.11 was already the answer to that trajectory; it is now overdue rather than
speculative.

**v0.11 — verification that can fail. *Built; not yet released.*** day#101 (a
guarantee wired at a call site) and day#116 (a guarantee asserted by a test that
cannot observe it) are one defect from two sides; day#114 and day#118 are the
tooling that would catch either. All six issues closed —
day#116/#114/#118/#101/#91/#89.

The deliverable is `scripts/revert-demo.py`: apply the inverse of a named change,
assert named tests fail, restore, assert they pass. Seven outcomes, never
conflated, could-not-check outranking checked-and-clean. A commit closing a
finding now carries a `Demonstrated-by:` trailer, and CI **re-derives** it rather
than reading it — the trailer is a claim about the work, and a claim nothing can
contradict is not evidence.

**The harness was built before the rule, and the condition was met.** Measured
over the milestone's own commits: **11.9 s cold, 2.0 s warm**, one command, the
trailer pasted rather than written. The load-bearing detail is that qualifying
the test target is what buys it — unqualified, the same demonstration takes
**3 m 54 s**, and at that price the rule would have been ceremony.

Three things it turned out to be about, none of which this section predicted:

- **Using the tooling found defect after defect in the tooling, and almost none
  of them was findable by a test.** `--quiet` suppressing the very lines that
  prove a named test ran; the test half of a change being reverted along with the
  fix; `cargo fmt`-shaped diff context merging a fix and its test module into one
  hunk; `--verify HEAD~1` re-resolving the rev against the *worktree's* HEAD;
  `--verify` perturbing the tree it was checking through `CARGO_TARGET_DIR`; the
  CI job that enforces the rule triggering on `main`, where the range is empty by
  construction, so it was permanently green for having found nothing; a trailer
  naming tests that had not failed; and the commit census reporting a git failure
  as a finding about a commit that does not exist. The full list, and how each was
  found, is in `.design/verification-that-can-fail.md` — **kept in one place
  deliberately**, because it was carried in two and the two disagreed about both
  the number and the membership, three rounds running.

- **A guard is not a fix, and the rule had to say so.** day#101 and day#89 add
  checks rather than change behaviour, so there is nothing executable to invert:
  day#89's run reports `REVERT-FAILED` and day#101's reports `VACUOUS`, because
  its only non-test change is a design document and reverting prose cannot fail a
  test. Each guard demonstrates by being shown to *fire* instead — day#101's scan
  against the tree at `1e02220^`, where it finds exactly the instance the issue
  named.
- **day#118's fix was to stop deferring a reading, not to add a step.** A version
  was excluded from the migration matrix at its own tag push, so its missing row
  could not fail until the *next* release — an omission invisible while anyone
  was looking at it, and dropped three releases running. `cut-release.sh` now
  measures and commits the row *before* tagging, and the workflow's exclusion is
  gone. One invariant, no window.

Scope was every harness in `scripts/` and `.github/workflows/`, not just
`mutate.py`: in the single session that produced this section, four separate
measurement tools asserted more than they verified, and none of it was in day's
shipped behaviour.

**v0.12 — transportable.** Vocabulary packs (day#73), with day#109, day#86,
day#50, day#107. This is the milestone the table below still calls "v0.8", and it
has now been deferred twice for the same stated reason: a pack transports a
vocabulary, and transporting one whose reporting is known-wrong exports the
defect to exactly the population `telos/v1.0`'s bar names. v0.10 and v0.11 are
what make it safe to ship.

Bracketed, not scheduled: day#38, day#75, day#82, day#84, day#85, day#88,
day#93, day#106.

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

## v0.6 — Rigor as artifact *(shipped: v0.6.0-beta.1)*

**Shipped and verified against the published artifact.** v0.6.0-beta.1 is live
on crates.io; the cargo-installed binary reads this repo's log (`doctor`
composes) and drives the v0.6 surface (`day status`, the status-line/cache
round-trip). The one open item, day#60, is designed
(`.design/current-cycle-position.md`) and deliberately not built here.

**Model half shipped.** Atoms carry `done` criteria, `day assess atom` gates
on them, and position is inferred from artifacts (`src/position.rs`).

**Surface half largely shipped.** `src/cache.rs` (the `.day/` render cache,
display-only, one-module source-scanned), `src/status.rs` (`day status` and
the status line, two renderings of one computation), inference wired into
`day hook session-start` (which makes AC-5 real coverage rather than vacuous,
mutation-checked), and **claims-based transitions** (`day status` reports when
the work has moved past the atom you last recorded assessing; baseline read
from `kan result` claims on `atom/*`, never written; `assess atom` prints the
runnable command that records one). Three things surfaced by *doing* it, not
testing it: (1) off-sequence detection false-positived on unprobed upstream
outputs on day's own log — fixed with a three-way `Outputs` presence; (2)
`recorded_at` is an **integer** in `kan show --json`, not the RFC-3339 string
first assumed — caught by `tests/kan_conformance.rs` against real kan before
it could break every read; (3) a Claude Code plugin **cannot declare the
top-level `statusLine`** (only `agent`/`subagentStatusLine`), so the design's
"the plugin gains a statusLine entry" is not achievable — the line is set up
in the user's own settings, and `day init` now says so.

**Transition notice shipped, on a live-verified channel.** The step-5 gate —
which human channel a transition can print to — was closed by *observation*,
not docs: in a real Claude Code session, a hook's `systemMessage` field
displayed to the user, and the status line rendered from the cache. So the
transition/off-sequence notice lands as a `systemMessage` from a **second,
separate** SessionStart hook (`day hook session-notice`), leaving the context
hook's output byte-for-byte unchanged — adding the human notice cannot regress
the model-context injection. `displayContent` was ruled out (it *replaces* the
assistant's on-screen text; wrong tool). The status line's persistent `day ⤳`
marker is the visibility floor if `systemMessage` ever renders differently on
SessionStart than where it was verified.

**What remains for v0.6** is the schema-legibility gap (day#60): day's own
witness schema does not probe intermediate atom outputs (`verdict`,
`assessment`), so position stays ambiguous and transitions stay quiet on this
repo until the schema is richer — likely needing a claim-existence probe kind,
which is its own design pass. Full design in `.design/rigor-as-artifact.md`.

**day is squishy.** It describes a process well and does very little to make
one happen. A session using day still produces test gaps, semantic drift
between a design and its implementation, and a lot of doubling back — this
repo's own v0.5 session produced all three, repeatedly, while day was running.

The intuitive fix is to inject better rules, in the style of `crosslink`'s
`rules/rigor.md`. **The evidence says that is the weak lever**, and it comes
from this project:

> day#30 shipped an injected rule — *"do not chain commit and push; chaining
> removes the last checkpoint where a bad commit is still cheap."* About an
> hour later, the agent that wrote it chained an inspection to an
> irreversible `rm -rf` and destroyed kan's log. The rule was in its context
> at the time.

A rule in context did not survive contact with the moment it applied to. So
v0.6 does not lead with injection.

**What did work in that session, every time:** mutation testing (each test
mutated and watched to fail was real; each one where that step was skipped
turned out hollow — the kan stub, the open-question counter, a git stub
returning all tags regardless of pattern), dogfooding against the real log,
and guardrail tests that grep the source. None of those are rules. All of
them are **artifacts that fail loudly when the thing they guard breaks**.

### The structural gap

Teloi have **witnesses** and **probes** — an evidence story, shipped in v0.5.
Atoms have `in`, `out`, `next`: types only, and **nothing that says how you
know the atom is done**. `generative-build` emits a `code-change`; what makes
a code-change finished is currently vibes.

So atoms get what teloi already have:

```day-atom
{"in": ["design-doc"], "out": ["code-change"], "next": ["adversarial-review"],
 "done": ["tests-fail-when-mutated", "clippy-clean", "every-req-named-by-a-test"]}
```

`done` entries resolve through the same `schema/witness` probes v0.5 shipped.
Atom completion becomes mechanically checkable **and per-project** — derived
from what this project declared, not from an opinion day ships. That is also
the difference from a rules file: `crosslink` ships general standards; day
makes *your* declared process checkable.

### Situated injection

Session-start currently injects `Process atoms (7): <names>`, which tells an
agent nothing it can act on. With `done` criteria and **position inferred from
evidence** — a design doc with no code change means you are in
`generative-build` — injection becomes: what this atom needs, what it
produces, and how you will know it is finished.

Inferred, never tracked. day still stores nothing, and the task-tracker line
`docs/CONVENTIONS.md` draws stays uncrossed.

### The human has never seen any of this

The observation that reframes the milestone, from the person who has used day
across several repos:

> the atom sequencing has never visibly played out in any repo I've used day
> in.

Not *unclear* — **never observed**. And it follows directly from where day's
output goes: hook stdout is injected into the **model's** context, never
displayed, and everything else requires running a verb. The human driving the
session sees nothing at all.

So day's value proposition is legible process, and the process is invisible to
the person it is meant to be legible to. That is also a better explanation of
v0.5's adoption finding than v0.5 gave: bare verbs are not merely
unmemorable — **nothing ever indicates there is anything to look at**.

v0.6 adds a surface aimed at the human, not the agent:

- **A status line.** Current atom, which inputs are satisfied, which `done`
  criteria are unmet, what comes next. Continuous, and it costs **no context
  budget** because statusline output never enters the model's context.
- **`day status`** — the detail one line cannot carry, for when the line says
  something is wrong.
- **Off-sequence detection.** With position inferred, out-of-order work is
  derivable: a `code-change` with no `design-doc`, against a bridge plan that
  says `design > generative-build > adversarial-review`. Shown in the line,
  and injected to the agent through `UserPromptSubmit` **only when the state
  changes** — not every turn.

That last one needs distinguishing from the day#30 evidence above, or it looks
like a contradiction. What failed there was a **general standing rule**,
present always and therefore background. *"You are off-sequence right now"* is
**specific, situated, and state-triggered**. One data point against ambient
rules does not condemn timely ones, and treating them as the same thing would
be over-learning from a single failure.

### Transitions and end-state

A status line shows continuous state. **Transitions are events**, and an event
deserves marking rather than being something you catch by watching a line
change. So v0.6 also reports when an atom transition happens, and prints an
atom's end-state — its `done` criteria, evaluated — when one completes.

Two constraints fall out, and both must be settled *before* anything is built.

**A transition is a difference, and a difference needs a baseline.** Position
is inferred from evidence, which is stateless by design and is exactly what
keeps day off the task-tracker line. But *"the position changed"* cannot be
computed from current evidence alone. Candidates: remembered state in a file
(day-owned, disposable, and still the thing `telos/no-store-of-its-own`
forbids); the git diff against `HEAD` (stateless and evidence-derived, but
approximate); or a recorded claim (task-tracking under another name).

This is **the same question as the status line's latency cache**. Both want a
small piece of ephemeral derived state on disk.

**Decided, then refined.** `SessionStart` does the kan reads — it already
runs and has time — and writes a small rendered snapshot. The status line
reads that snapshot, so it can never be cancelled mid-flight.

**But the two problems turned out to be separable, and merging them was the
error.** "The line must render before it is cancelled" and "a transition
needs a baseline" are different questions. **The baseline is claims**, not
the cache: an assessment recorded in kan is durable, attributable,
retractable, contradictable, and scoped by the same locally-signed projection
rule `practice` uses. A file is none of those.

This is not the task-tracking `docs/CONVENTIONS.md` refuses — that refusal
says whether a step happened *"is already derivable from claims and artifacts
existing"*, which is exactly this. What it refuses is day-owned tracking
state. And day **reads** those claims and never writes them, because
`src/telos.rs` already holds that conflating *"I checked"* with *"I recorded
that I checked"* would let the tool manufacture its own evidence.

So a transition means something better than originally designed: not *"the
cache file changed"* but *"position has changed since you last recorded an
assessment"* — self-reinforcing, since the baseline exists only if
assessments are actually recorded, and `assess telos` already prints the
`kan result` that writes one.

The cache is left holding **only rendered display state**, which tightens its
guardrail structurally rather than by rule: there is no longer anything in it
to misuse.

Why that is not a store, stated so it can be argued with. This telos exists
so *"a project can discard day entirely and lose nothing but opinions"*. A
cache that is strictly derived, gitignored, regenerated next session, and
never read as a source of truth for anything but display satisfies that
exactly — delete it and nothing is lost.

**The precedent that makes it defensible is kan's own.** `.kan/log/` is the
durable signed record; `.kan/index.sqlite` is a disposable derived index that
rebuilds from it — verified during this session's recovery work, where
deleting the index recovered all 164 claims from the log. A day render cache
stands in the same relation to kan's log as kan's index does to it. If the
pattern is acceptable for the memory layer it is acceptable for the process
layer.

**Guardrails the design pass must build in**, because this is a step toward
the seam `tension/composable-process--no-store-of-its-own` already names:
gitignored; regenerable from kan and git alone; **never read by any check,
assessment or verb whose answer matters**; contains nothing not derivable;
and its absence is never an error. *If day ever reads the cache to decide
something rather than to display something, that is the line being crossed.*

**Which hook events reach the *human* is unknown and must be verified first.**
`hooks/hooks.json` records which events deliver stdout to the **model**;
nothing records what reaches a person mid-session. A status line does. Whether
any hook does is unverified.

This repo has already shipped a hook wired to an event whose output reached
nobody — the v0.2 session-end hook, which `/adversarial-review` named the most
serious finding it ever produced, and whose lesson is in `CLAUDE.md` as *"a
check that only inspects its own side of an interface will miss the
interface."* Designing transition reporting without first reading the harness
docs would repeat that failure in the same place, having written the lesson
down twice.

**So the first task of the v0.6 design pass was not design.** It was
establishing, from the harness documentation rather than by inference, which
channels can deliver to a human mid-session.

### What that verification found *(done)*

**The status line is the right primary channel, and better than assumed.** It
receives rich session JSON on stdin (~40 fields including
`workspace.current_dir`, context-window usage, and cost), supports **multiple
lines**, ANSI colour, and clickable OSC 8 links — and explicitly *"runs
locally and does not consume API tokens"*, so it never enters the model's
context. Information for the human at zero context cost, which is exactly the
property wanted.

**The latency constraint is worse than estimated, and now concrete.** The line
re-runs on session start, every new assistant message, `/compact`,
permission-mode changes, vim-mode toggles, and an optional `refreshInterval`.
Claude Code debounces at 300 ms **and cancels an in-flight script when a new
update arrives.** So a status line that shells kan does not merely lag — it
can be cancelled before rendering anything, repeatedly, and show nothing at
all. That raises the stakes on the baseline/cache decision rather than
settling it. `refreshInterval` is a knob worth knowing about.

**stderr reaches the human on several events, but framed as an error.**
`SessionStart`, `SessionEnd`, `Notification`, `Setup`, `MessageDisplay`,
`PermissionRequest` and `PermissionDenied` show stderr to the user. But a
non-zero exit makes the transcript show `<hook name> hook error` followed by
**only the first line** of stderr. Unsuitable for routine transition
reporting; possibly right for a genuine warning, which off-sequence detection
might be.

**A `MessageDisplay` hook can return `displayContent`**, documented as
affecting screen display only rather than model context. A second
human-facing channel, and not error-framed. Worth investigating before the
design settles.

### And it corrects something day has believed since v0.2

`hooks/hooks.json` and `src/hooks.rs` both state that every end-of-session
event writes to the debug log, so a prompt registered there *"would silently
reach nobody."* That is true of the **model** and **false of the human** —
`SessionEnd` stderr is shown to the user.

The v0.2 reasoning was sound about the audience it considered and never asked
about the other one. Which is the *"check the other side of the interface"*
pattern appearing inside the very decision that first recorded that lesson.

**Documented gaps, so they are not mistaken for settled:** the destination of
the universal `systemMessage` field is unspecified; stderr behaviour on a
clean exit 0 is not stated for most events; and no documentation addresses
whether any of this differs between the CLI, VS Code, and JetBrains surfaces.
Each would need an empirical check before being relied on.

### Still advisory

A hard gate was considered and rejected: a `Stop` hook or pre-commit refusing
when criteria are unmet would be the strongest standardizing force and would
directly contradict `telos/affordance-not-enforcement`, which `tests/plugin.rs`
enforces and which exists because crosslink's blocking hooks caused the
friction day was split out to avoid.

The line that holds instead: **enforcement at the artifact level, not the
action level.** day never gates an agent mid-action. It makes the state of the
work measurable and exits non-zero. CI and humans gate; day reports.

## v0.7 — Frames, and current-cycle position

Two threads. **Frames is not slipping again** — it stays in v0.7, now beside
the position work that v0.6 designed but did not build. Adding a second thread
here rather than deferring Frames a third time is the deliberate answer to the
tripwire below.

**v0.7 ships across betas rather than in one cut.** `v0.7.0-beta.1` is the
position work; Frames closes the milestone. This is the mechanism that lets the
position work reach a real log — where every defect in day so far has been
found — without renumbering Frames out of the milestone. Shipping a beta is not
the same act as closing a milestone, and only the second one would be a third
deferral.

**Two more betas landed between them, and naming why matters more than naming
them.** `beta.2` and `beta.3` below are not new scope competing with Frames —
they are Frames' day-side precondition, arrived at from the opposite direction.
Frames factors into *whose attestations count* (kan's enrichment) × *what counts
as a satisfying certificate* (day's witness criteria); day already ships the
second half, globally, on `schema/witness`. Making it frame-indexable means the
declared-vocabulary layer underneath it has to be worth indexing, and running
day against a real log showed it was not: **day silently drops declarations it
cannot account for, and four of the things a project reasonably differs on are
still Rust constants rather than claims.**

So there is one through-line under `beta.2`, `beta.3`, Frames and v0.8, and it
is worth stating once. Every durable thing day does is driven by
**project-declared vocabulary read back from kan claims**. That read path needs
four properties, and they can only be built in this order:

| | property | milestone | status |
| --- | --- | --- | --- |
| 1 | **honest** — what it cannot account for is reported, never dropped | `beta.2` | **shipped** |
| 2 | **complete** — everything a project may differ on is declarable | `beta.3` | **shipped** |
| 3 | **transportable** — a vocabulary can be handed to another repo | packs (day#73) | not started |
| 4 | **frame-relative** — the same vocabulary read under several frames | Frames | blocked on kan |

Property 1 has been tested twice more since it shipped, and both times by
something other than a test. `v0.7.0-beta.3`'s review found an unreadable
*instance* reaching neither hook channel; day#71's review found `assess telos`
reporting `[MISSING]` for evidence day had never received. Honest reads is not
a milestone that closes — it is the property every subsequent read has to keep,
and the two mechanisms that keep it are the source scans and
`/adversarial-review`. day#101 is about that second one being the only thing
catching a specific recurring shape.

Honest strictly precedes complete: every surface made declarable before the
degradation contract exists inherits the silent-dropping bug, which is day#78
against day#74 exactly. And it must *ship* before, not merely land before — the
fix only helps a binary that has it, so `beta.2` is released before `beta.3`
introduces fields older binaries cannot read.

**None of this is Frames losing again.** Frames is fourth in that table only
because its kan half — enrichment selection at the read surface — is kan's v0.8
REQ-3, specced and unbuilt at the time of writing. day is not choosing to
deprioritise it; day is building the half it owns while the other half is built,
which is the opposite of a deferral. Stated here rather than left inferable from
a beta number, because that is what the tripwire below is for.

### Current-cycle position (day#60)

Designed in v0.6, recorded on subject `current-cycle-position`
(`.design/current-cycle-position.md`): give position a **cycle boundary** — the
last release — and a **`claim` probe kind** that reads the log rather than the
working tree, so a witness like `verdict` or `assessment` is present when one
was *recorded since the boundary*. It resolves the day#60 legibility gap the
v0.6 release assessment ran into head-on: on a repo with history every artifact
type exists from prior cycles, so `day status` reports four candidates and could
never narrow, and a naive claim-probe would report a stale verdict as current.
Cycle-relativity is **inference-only**; assessment stays cumulative. It is *use*,
not model — the same axis v0.6 was ordered on — which is why it sits ahead of or
beside Frames rather than after it. The design is validated and its next atom is
`generative-build`; the build is the v0.7 opener.

**Shipped in `v0.7.0-beta.1`.** Built, adversarially reviewed
(APPROVE-WITH-FOLLOW-UPS), and merged in day#69. Position on day's own log went
from four candidates to two, and recording the milestone's own review verdict
then made `pull-request` current with `inputs satisfied: code-change, verdict` —
the claim probe closing the loop on a real log rather than in a fixture.

Two defects came from running it rather than testing it, which is now the
expected rather than the surprising outcome: the installed v0.6 binary failed
the *whole* witness schema on meeting a `claim` probe (day now tolerates a probe
kind it cannot read, costing that witness and nothing else), and resolving a
witness per mention made `day status` take 5.7s on a 38-subject log.

Follow-ups: day#70 (the `assessment` probe is broader than an atom assessment)
is **built** — `.design/claim-probe-narrowing.md`; `ClaimShape` is now a
conjunction of independent predicates, narrowable by an anchored `starts_with`
and a glob-lite `subject`, and it doubles as the day-side seam for Frames
below. Running it on day's own log is what sized the defect: 14 live `Result`
claims here and **zero** on an `atom/*` subject, so the old `{kind: Result}`
probe matched 14 claims none of which was an atom assessment.

Still open: day#71 (a claim probe reads the whole log; the right fix is a bulk
read upstream, since kan owns folds over the claim graph) and day#78, which
day#70's review surfaced — a narrowed `ClaimShape` read by a *pre*-day#70
binary silently widens rather than reporting itself unreadable, the opposite of
what day does for an unknown probe *kind*. The question it raises is broader
than one struct: which fenced-block schemas are descriptive (ignore unknown
fields, as day requires of kan) and which are restrictive (an unknown field is
a narrowing predicate, so ignoring it changes the answer).

### Honest reads (`v0.7.0-beta.2`)

Found by running day, not by reading it. Two demonstrations, both against the
shipped binary:

- An atom declaring `{"in":…,"out":…,"next":…,"requires":["approval"]}` — a
  field this day has never heard of — loads as though `requires` were absent,
  prints without it, and reports `composition: ok` at exit 0.
- A project declaring `forbidden_sections: ["Scratch Notes"]` in its
  `schema/design-doc` gets nine `[PASS]` lines and exit 0 on a document that
  **contains** a "Scratch Notes" section. day certified conformance to a
  declaration it had silently truncated.

Seven fenced vocabularies, none with `deny_unknown_fields`, and every one of
them exists to *constrain or narrow* something day then reports on — which is
what makes ignoring an unknown field unsafe here in a way it is not for kan's
claim JSON. An extra fact is extra information; an extra constraint dropped is a
false certification. day's tolerance of kan is underwritten by a version
contract (`SCHEMA_VERSION`, additive-only, and day pins `SHAPE_VERSION` against
it); day's own blocks carry no version at all, so their tolerance is unbacked.

So: `deny_unknown_fields` on day's built-in blocks, **plus** a block version, so
the message is *"this day reads `day-atom` v1, this block declares v2"* rather
than a parse error that reads as the project's mistake. Versioning is the
load-bearing half — the v0.6 binary that took the session hook down failed
loudly and misdirected the reader, and detection without an actionable message
repeats that.

Degradation granularity needed less work than expected: atoms already degrade
per-atom and name the cascade, a `--all` telos sweep already reports the bad one
and assesses the rest at exit 2, and a single-object schema failing the whole
command *is* its smallest unit. What was missing is the **hook**, which lists an
unreadable telos beside the readable ones and says nothing — the channel that
reaches the model, silent, which is day#60's pattern precisely. Reported to both
audiences per day's existing allocation: the model gets the caveat attached to
the item it undermines, the human gets a `systemMessage` differentiated by cause
(upgrade day, or fix the claim).

Also here: `UserPromptSubmit` wired at last — designed in v0.6, allowlisted in
`tests/plugin.rs`, never registered. Measured, `day status` is 2.76 s of which
1.99 s is 41 `kan` invocations, while the git half is 0.03 s — so `path`- and
`tag`-driven transitions are affordable per prompt today and the `claim`-probe
half waits for day#71. State-triggered display and a bounded periodic
re-display are kept as *separate* triggers: a recurring reminder is much closer
to the ambient standing rule that failed in day#30 than an event notice is, so
the periodic channel carries only standing conditions that affect the
correctness of what day already said.

### The vocabulary substrate (`v0.7.0-beta.3`)

Six hand-rolled fenced vocabularies collapse onto one declared-block mechanism
(day#74), with `day-atom` as its first instance and one less special case. Then
everything day hardcodes that a project may reasonably differ on becomes a
declaration on it: verdict vocabularies (day#77), the cycle boundary (day#76),
resolved-question ids (day#36), practice items (day#43/#44), the injection
cadence above, and design-check-against-recorded-decisions (day#41).

Net *simplifying* in code while adding capability, and it strengthens
`telos/no-store-of-its-own` rather than straining it — four things move out of
Rust and into claims.

It also resolves a conflict rather than splitting it. day#74 asks for
"generated blocks only, never hand-written," which contradicts the conventions'
standing rule that a hand-written claim following them is exactly as valid. The
resolution is that generation-only is **unenforceable by construction** in a
multi-actor log: day cannot control what another actor's binary wrote, so the
moment claims arrive over sync from a second key, writer-side guarantees
evaporate. The guarantee that survives is reader-side validation — which is
`beta.2`. The ask is right; its mechanism was the wrong one.

### Frames

Multi-actor. The shape is now understood well enough to say precisely what it
depends on, which is narrower than "kan's sync epic" (recorded on the `frames`
subject in kan; kan issues #114/#115/#117).

**A frame factors along the kan/day seam.** Reading kan's source, a day-frame —
an actor's internal logic in which a certificate for "telos satisfied" is or
isn't valid — *is* a kan **enrichment / `TrustBase`**: the same construction in
two vocabularies. And it factors into two halves that land on opposite sides of
the boundary: (i) *whose attestations count, and how much* = kan's enrichment;
(ii) *what counts as a satisfying certificate* = day's witness criteria. **day
already ships half a frame** — `schema/witness` is (ii), currently global /
single-frame. So Frames is not a new model to invent; it is: kan generalizes the
enrichment half and exposes it, day indexes the witness half by frame and
orchestrates the comparison. Per ADR-18, day never re-implements the fold — a
trust-weighted fold over the claim graph is kan's.

**The dependency is `kan#117`, not the whole transport layer.** kan's fold is
already `fold(claims, enrichment)`, pure in the enrichment; `PeerContested` is
built and tested — only the read surface hardcodes `solo_trust()`. So the
reconciliation *semantics* are ungated, not unbuilt. kan#117 (enrichment
selection at the read surface + enrichment-as-signed-record) is the substrate
day folds under; it rides on kan#114 (consume another actor's published claims)
and kan#115 (per-role signing identities). A **real dogfood vehicle now exists**:
kan#114's agentic research loop (PI + director + prover, machine-checked in Lean)
states "the trust parameterization *is* the role hierarchy" — day's frames
arrived at from a live multi-actor workload, not day's single-actor log.

**The day-side seam is `day#70`** (`.design/claim-probe-narrowing.md`, **built**).
Its `claim`-probe generalization makes `ClaimShape`'s matching a conjunction of
independent predicates, so the frame-relative *author/enrichment* dimension
slots in additively later — against kan#117's contract — rather than
re-litigating the probe language. A day-side `author` predicate was
deliberately *not* built with it: a flat `author == did` filter is not even a
correct `Solo`, since it misses the keys kan's fold merges via `SameAs`, and it
would pre-commit a frame representation kan owns. The heavy day-Frames design is
written once that contract exists, not guessed ahead of it. **Nothing further
toward Frames is buildable here until kan#117 lands.**

**Frames has been deferred twice** — v0.5 moved it to v0.6, v0.6's rigor work
moved it to v0.7. It is **not moving again**: v0.7 carries it alongside the
position work rather than bumping it. `v0.7.0-beta.1` shipping without it is a
beta, not a milestone close — Frames is still v0.7, and the tripwire below
fires on it moving to v0.8, not on it landing in a later beta. The deferral reason was consistent both
times — the v0.5 meta-evaluation found day's model outrunning its use, frames is
more model, rigor-as-artifact was use — and honouring the tripwire means holding
Frames here rather than finding a third reason to slip it.

Said plainly so it can be checked: **if frames slips a third time, that is no
longer a sequencing argument.** It would mean either the item should be cut or
there is a reason it keeps losing that has not been named.

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

## v0.8 — Plugins: adopting a style instead of hand-declaring one

Getting day useful in a new repo currently means hand-declaring a vocabulary:
atoms, teloi, schemas, witness probes, and — after v0.6 — practice items and
`done` criteria. That is the onboarding cost the **v1.0 bar measures
directly**, since "a person who is not the author uses day on a project that
is neither kan nor day" is exactly the person facing an empty log.

So: bundles, installable from a git repo, following the plugin-repo format
Claude Code already established.

```
day plugin install maxinelevesque/day-flow
```

**The insight that makes this fit rather than fight the model:** a plugin is a
bundle of **declarations**, and installing one means **recording them as your
own claims**. Not files day keeps, not a config directory, not a second store
— the repo is a *source*, and afterwards the only durable artifact is claims
in your log, signed by you. `telos/no-store-of-its-own` is preserved rather
than strained.

**This section used to end "and it falls out for free because everything a
plugin would carry is already a kan claim." That sentence was false, and it was
this milestone's load-bearing assumption.** Four of the things a real pack needs
to carry — review verdicts, the cycle boundary, the block schemas themselves,
and `schema/design-doc`'s own conventions — were Rust constants, not claims. A
pack can only transport what is declarable, so v0.8 was not buildable as
written.

It was not found by re-reading the roadmap. A research program distilled its
process into a spec, tried to compile it to day vocabulary, and filed four
issues from that one attempt — day#73/#74/#76/#77 — each one a part that would
not move into claims. `v0.7.0-beta.3` is that precondition, and it is why the
substrate is named as a milestone rather than left implicit here: **v0.8 is the
payoff of beta.3, not a feature standing on its own.** Corrected in place rather
than quietly rewritten, because a roadmap that silently stops having been wrong
is worth less than one that records the correction.

It also gets a property monolithic plugin systems lack. Because the contents
become ordinary claims, **adoption is per-item and revisable**: retract one
atom, revise another, keep the rest. A plugin is not a unit you install and
uninstall. Provenance survives by citing an `adopted <plugin>@<version>`
claim, so the record shows both that you declared these and where they came
from.

**The risk, which is easy to understate.** A plugin is a vector for prompt
injection *and* for command execution — its practice items reach an agent's
context, and its witness probes can carry `command` probes. day#25 requires
injected practice to be locally-signed, but plugin content becomes *your*
claims signed by *you*, so it passes that check **by construction**. The trust
boundary therefore moves to **install time**.

**And the most dangerous payload a pack can carry is not a command probe.** Once
Frames lands, a pack can reasonably declare a **frame** — which roles exist and
whose claims count — and a research pack arguably should: kan#114's field report
puts it plainly, *"the trust parameterization IS the role hierarchy."* But a
frame declaration changes what you can **see**, not what runs. A pack that
weights your own key to zero makes your own claims vanish from your own reads,
at exit 0, in a view that looks complete — verified as reachable behaviour while
wiring the day/kan bridge, on a workspace with two local identities. A command
probe is loud and reviewable in one line of argv; this is silent and its effect
is an absence. So whatever install-time review guards `command` probes must
guard trust declarations *more*, and a frame must never be applied without being
shown.

### Trust is a claim, not a setting

The answer is internal to the model day already has. **Trust toward an author
is itself a kan claim**, and a plugin install is evaluated against it:

- No recorded trust toward the plugin's author → install **warns and asks**,
  showing what is being adopted: which atoms, which practice items, and — most
  importantly — any `command` probes.
- Recorded trust toward that author → the install proceeds smoothly.

This is day#25's deferred `trust/injection` list, generalised: it stops being
injection-specific and becomes the primitive for *whose claims do I accept*.
Being a claim rather than a config file means trust is attributable,
retractable, syncs with everything else, and can itself be cited — you can see
**when** you decided to trust someone and **why**.

**Why a prompt here does not violate affordance-not-enforcement.** That telos
constrains what day does to an **agent mid-flow**: no gates, no blocked
actions, no refusals. `day plugin install` is a **human** deliberately
adopting third-party content, and asking for confirmation before adopting
executable content from a stranger is not the same category of act as
refusing an agent's edit. The distinction is worth stating because it will
come up again: day never gates the work, and may absolutely ask before it
adopts something on your behalf.

**Why here and not sooner:** the bundle format cannot stabilize before the
vocabulary it packages does. v0.6 adds `done` criteria, situated injection,
and transition reporting; a format designed now gets redesigned then.

## v0.9 — Atom library and meta-evaluation

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
