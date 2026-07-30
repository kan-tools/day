# Conventions (v0)

day keeps **no store of its own**. Every durable thing it knows is an
ordinary kan claim, written with kan's ordinary verbs, found again by kan's
ordinary fold. This file is the whole schema: a set of subject-naming
conventions plus one embedded block format.

That is a deliberate constraint, not an accident of being early. kan owns a
feature iff it needs a new `ClaimBody`/`ClaimKind`/`Anchor`/`RelationKind`
variant or is a pure read over the claim graph (kan's ADR-18). day is the
other side of that line: it is entirely a calling convention over kan's
existing primitives, which is exactly why it can be swapped out, forked, or
disagreed with without touching anyone's memory substrate.

**These conventions are v0 and expected to change.** They are versioned as
this document, in this repo, and superseded the same way anything else is —
by a later claim, never by a rewrite of history.

## Teloi — `telos/<slug>`

A telos is a desired state of the world, held only **up to weak
equivalence**: it names an invariant — some aspect of the shape of the world
that has a particular coherence — not a point-target. Several teloi normally
apply to one project at once, and they are usually in some tension with each
other. That tension is information; do not collapse it.

```bash
day telos declare interface-legibility \
  "The CLI vocabulary stays legible to an agent that has never seen it before." \
  --title "Interface legibility" --kind idea

# ...and when two teloi pull against each other, which is normal:
day telos tension interface-legibility feature-depth \
  "Every verb added for depth is one more thing to learn before the tool reads clearly."
```

**The conventions are the contract, not the verbs.** `day telos declare` is a
convenience over `kan decide --subject telos/<slug>`; a hand-written claim
following the conventions on this page is exactly as valid, and day reads it
identically. Nothing here requires day to be installed.

There is no `revise` verb, for teloi or anything else. kan is append-only, so
a revision *is* a later claim — `declare` again and day cites the previous
claim automatically.

- Use `decide` to declare or revise a telos (it is a choice), `observe` to
  record something noticed about how a telos is holding up, and `result` to
  record an assessment of whether work landed inside its equivalence class.
- Revise a telos by appending a new claim citing the old one. Never retract
  a telos merely because it changed — a superseded telos is the record of
  what was once wanted, and drift is only visible against it.
- When work trades one telos off against another, record that trade-off as a
  `decide` claim citing both subjects. An unstated trade-off is how a
  misaligned telos enters a project unnoticed.

**A telos subject carries its declaration and its edges — not commentary
about it.** That is a rule with a reason: everywhere day renders a telos it
shows the newest claim carrying text, so anything else recorded there
displaces the statement. Recording a tension's reason on a telos subject
used to do exactly that, in day's own log, for four of six teloi.

## Tensions — `tension/<a>--<b>`

Two teloi pulling against each other is normal and is information. The
*reason* is the part a reader needs — "these two conflict" is much less
useful than "these two conflict because compelling the records legibility
needs would make day the kind of tool people route around".

A kan relation carries no narrative body, so a tension is recorded as two
things: an `in-tension-with` **edge in each direction**, on the telos
subjects, and a **claim carrying the reason**, on its own
`tension/<a>--<b>` subject, which both edges cite.

```bash
day telos tension interface-legibility feature-depth \
  "Every verb added for depth is one more thing to learn before the tool reads clearly."
```

The subject carries a fenced `day-tension` block naming the pair:

```day-tension
{"between": ["feature-depth", "interface-legibility"]}
```

**The slug is the two slugs sorted**, so `day telos tension b a` and
`day telos tension a b` name the same subject — one relationship, one
subject, whichever order it was typed in. day finds tensions by reading the
block, **never by parsing the slug**: telos slugs contain hyphens
themselves, so `tension/foo-bar--baz` is not reliably decomposable. The slug
is a name; the block is the data.

**Two edges, not one.** kan's relations are directed and visible only from
the source — `kan show telos/b` does not surface an edge declared from
`telos/a`. Tension is symmetric, so representing it faithfully in a directed
model takes both directions; with one edge, "what is this in tension with"
would answer from whichever side happened to be typed first.

`day telos tension` writes all three claims. `session_context` and
`day assess telos` read them back, so moving the reason off the telos does
not make it harder to find.

## Working practice — `practice`

day injects its own process practice and operational-safety guidance into
every session. A project's own practice goes on a `practice` subject, and day
projects it into the same block.

```bash
kan observe "Run the migration check before tagging a release." --subject practice
```

**Each live claim is one item.** Appending adds one; retracting removes one.
That is different from every other convention on this page — a `day-*` block
is newest-claim-wins because a schema is one object, and injected practice is
a *list*, which is exactly what kan's live fold already gives for free.

**Only locally-signed claims are projected.** A claim on `practice` signed by
another identity is not injected, and day says how many it skipped and why.
This is an injection path: text here reaches a model's context, so it is
scoped to the key that signs this workspace. If day cannot establish that
identity, it projects **nothing** and says so — verification being
unavailable is not a reason to inject unverified text.

**day's own blocks are refusable.** A claim of the form:

```
day-replace: practice
day-replace: safety
```

replaces day's corresponding block with your items instead of adding to
them. Both are replaceable, including safety: an opinion a project cannot
refuse is the kind of opinion `docs/TELOS.md`'s affordance-not-enforcement
rules out, and *"this rule is too important to remove"* is what every
blocking tool says about itself. day states which of its blocks was replaced,
so the suppression is visible in the thing being suppressed.

The projection is bounded — a per-item excerpt and a cap on item count, both
reported when they bite. Session-start competes with your actual request for
attention, and this is the one part of it a project can grow without limit.

## Atoms — `atom/<slug>`

An atom is a composable unit of process work: generative design, generative
build, adversarial review, user testing, structured research, formal
verification, purpose/drift evaluation, meta-evaluation of the atom set
itself, external human coordination. The vocabulary is **per-project** and
co-evolves with the project's teloi.

The vocabulary is **per-atom additive**: each atom is its own subject, "the
current vocabulary" is just the live non-retracted fold, and the newest
interface-bearing claim on a subject is that atom's current interface. There
is no whole-document process spec to swap out — revising an atom appends a
claim, exactly as kan handles everything else.

An atom claim carries a fenced `day-atom` JSON block:

````markdown
```day-atom
{"in": ["design-doc"], "out": ["code-change"], "next": ["adversarial-review"], "done": ["passing-tests"]}
```
````

| Field  | Meaning |
| ------ | ------- |
| `in`   | Type names this atom requires to be applicable |
| `out`  | Type names this atom produces |
| `next` | Slugs of atoms this one composes into |
| `done` | Witness types that evidence this atom is finished |

Type names are free-form strings. day checks that they *match*; it
deliberately does not check what they mean. The type vocabulary is the
project's to choose and evolve.

`done` is the completion story `in`/`out`/`next` leave unstated: they say what
an atom consumes, produces, and leads to, but nothing about *how you know it
is finished*. Its entries are witness types resolved through the same
`schema/witness` probes teloi use (below), so a project declares what would
evidence completion once and both teloi and atoms draw on it. `done` is
**additive and optional**: a `day-atom` block written before it existed parses
and composes identically, and an atom with no `done` is reported as having no
completion criteria — never treated as met. `day assess atom <slug>` checks
these criteria and **exits non-zero when a declared one is unmet**, so CI can
gate on it; it runs `command` probes only under `--run`, matching
`assess telos`.

```bash
day atom declare generative-build \
  --in design-doc --out code-change --next adversarial-review \
  --done passing-tests \
  --note "An agent session turns an accepted design into code."
```

day generates the block; you never hand-write the JSON. As with teloi, a
hand-written claim carrying the same block is equally valid — and
`day atom declare` reports composition findings but records regardless,
because declaring a multi-atom chain necessarily passes through states where
it does not yet compose, whatever order you declare it in.

Not every atom needs a shipped command. `generative-build` — an agent
session turning an accepted design into code — is a real atom in day's own
vocabulary with no slash command behind it. The vocabulary describes the
process, not the tooling.

`day doctor` reads every `atom/*` subject and checks the set composes: each
`next` target must exist, and each atom's `in` must be covered by what its
upstream atoms produce. A failure is reported, never repaired — day has no
write path into the log.

Coverage is checked against the **transitive** upstream closure, not just the
immediate predecessor, because artifacts accumulate along a bridging path
rather than being consumed by the next step. day's own three atoms are the
worked example:

```
design  in[intent]                    out[design-doc]   next[generative-build]
generative-build  in[design-doc]      out[code-change]  next[adversarial-review]
adversarial-review  in[design-doc, code-change]  out[verdict]
```

The review needs the design doc as well as the code change, and the design
doc is still there when the review runs even though the build step didn't
re-emit it. An atom with no upstream atoms is a source: its inputs come from
outside the vocabulary and aren't checked.

## Design-doc schemas — `schema/<slug>`

A design document's shape is process vocabulary too, so it lives in kan the
same way atoms do: a `schema/<slug>` subject (day looks for
`schema/design-doc` by default) carrying a fenced `day-schema` JSON block,
newest claim wins.

```day-schema
{
  "sections": ["Summary", "Requirements", "Acceptance Criteria", "Architecture"],
  "requirement_prefix": "REQ-",
  "criterion_prefix": "AC-",
  "min_requirements": 2,
  "min_criteria": 2,
  "placeholders": ["TODO", "TBD"],
  "paths_section": "Architecture",
  "resolved_section": "Resolved Questions"
}
```

day ships **no** hard-coded document shape. If no schema is declared,
`day design check` says so and prints a runnable command recording the
starter schema above — it will not quietly validate against an opinion you
never chose. A project that wants different sections changes its claim, not
day.

`day design check <path>` reports one line per rule: required sections
present and non-empty, requirement and criterion counts against the declared
minimums, every declared requirement referenced by at least one acceptance
criterion, no placeholder tokens outside fenced code blocks, every
backtick-quoted path in `paths_section` existing on disk, and a count of
unresolved `<!-- OPEN` blocks. Open questions warn; they do not fail — an
explicitly-marked unknown is a feature of a design doc, not a defect.

The placeholder and open-question rules ignore **both** fenced blocks and
inline code spans, so a document that *quotes* a marker while explaining the
convention is not treated as carrying one. The path rule deliberately does
not: it looks for backtick-quoted paths, so stripping spans there would
leave it nothing to read.

`day design record <path>` appends the chain: an `observe` carrying the
validation result, a `plan` for the design citing it, and one `decide` per
bullet under `resolved_section` citing the plan. day assembles `--cites`
from CIDs it captured itself, which is what makes the "pass a file path to
`--cites`" error unreachable rather than merely warned against.

**A document that fails validation is still recorded**, with the result
embedded in the plan claim. Gating would mostly cause people to skip
recording rather than fix the document, and an unrecorded design serves the
record worse than a visibly rough one.

## Bridges — `bridge/<slug>`

A **bridge** is a planned arrangement of atoms aimed at a target telos: how
you get from here to there. Intermediate states are not a new kind of thing —
per `docs/TELOS.md` a bridging state is just a telos at a shorter horizon, so
they stay ordinary `telos/<slug>` subjects and the *plan* is what a bridge
adds.

For "does this plan reach that telos" to be checkable at all, the telos has to
say what would evidence it. A telos may declare **witnesses**: artifact
*types* that would count as evidence, in a fenced `day-telos` block.

```bash
day telos declare v03-shipped "day v0.3 is published." --witness published-artifact
```

Witnesses do not collapse a telos to a type. They name the *kind* of evidence
while leaving open which concrete instance provides it — many artifacts of a
declared type satisfy the telos equally, which is the weak equivalence being
preserved. A telos without witnesses is still valid; it simply cannot be
machine-checked as a bridge target, which day says rather than guessing.

```bash
day bridge declare v0.3 --telos v03-shipped --have intent   --plan "design > generative-build > adversarial-review > pull-request > release"
day bridge check v0.3
```

The plan grammar, in a fenced `day-bridge` block that day generates:

| Form | Meaning |
| ---- | ------- |
| `a > b` | in sequence — `b` may use what `a` produced |
| `a & b` | concurrently — both happen, but neither may rely on the other |
| `a \| b` | alternatives — either route suffices |
| `(...)` | grouping |

`|` binds tightest, then `&`, then `>`.

**Sequence and concurrency are not the same**, and the difference is the
point: in `a > b` the ordering guarantees `b` can use `a`'s output, while in
`a & b` there is no such guarantee, so `b` is checked against only what was
available before either began.

**An alternative offers downstream only what every branch produces.** A route
that might not be taken cannot be relied on to have produced anything. That
intersection is what makes `|` mean something rather than being `&` with
different spelling, and it is how de-risking through separable parallel paths
becomes structural rather than a comment.

Availability accumulates along a path and is never consumed — the same rule
the atom composition check uses, so a design doc is still there when a review
runs even though the build between them did not re-emit it.

**day does not track whether a plan's steps have happened.** It checks whether
an arrangement *could* reach a telos and stops. Whether a step happened is
already derivable from claims and artifacts existing, and answering "how far
along are we" is the first question of a task tracker.

Realizability as reported is **frame-internal only**. `docs/TELOS.md` defines
it as two-fold — frame-internal continuity plus temporal coherence across
frames — and the second is vacuous with one actor. day says so in its output
rather than letting a single-frame result read as a settled global one.

## Docs assessment — `schema/docs`

`day assess docs` asks whether what the docs assert still matches what
shipped. What it checks is declared per project on a `schema/docs` subject
in a fenced `day-docs` block — day assumes no layout, because not every
project it might serve is a Rust CLI.

```day-docs
{
  "version_source": "Cargo.toml",
  "version_key": "version",
  "version_files": ["README.md"],
  "doc_files": ["README.md", "docs/ROADMAP.md"],
  "release_subject": "release"
}
```

`version_key` is read format-agnostically: day finds the key and takes the
value after it, which covers `version = "1.0"`, `"version": "1.0"`, and
`version: 1.0` without knowing TOML, JSON, or YAML.

**Two tiers with different powers.** The *mechanical* tier can fail — a
declared version-carrying file with a stale version is wrong, full stop. The
*evidence* tier only prompts: it reports what changed since the last release
and whether any watched doc changed with it. Deciding whether a change
*needed* documenting takes reading both, and that judgment stays with the
reader.

**The release boundary is reconciled, not chosen.** day reads both the last
claim on the release subject and the last `v*` git tag. Disagreement is a
finding: a release tagged but never recorded, or recorded but never cut, is
itself drift. `--since <ref>` names the boundary outright and skips the
question.

This is where day reads **git**, its second substrate, and only ever reads:
`tag` and `diff`, behind one module, with a test whitelisting the permitted
subcommands.

## Telos assessment — `schema/witness`

`day bridge check` asks whether a plan *could* reach a telos. `day assess
telos` asks whether it **did**. The difference is that a witness is a
*type* — `published-artifact` names a kind of evidence, not a particular
artifact — so assessing means binding the type to an instance without
collapsing the telos onto it.

What would count is declared per project on a `schema/witness` subject
carrying a fenced `day-witness` block: a map from witness type to **probe**.

```day-witness
{
  "published-artifact": {"tag": "v*"},
  "design-doc": {"path": ".design/*.md"},
  "passing-tests": {"command": "cargo test"},
  "verdict": {"claim": {"kind": "Decision", "starts_with": "adversarial review of"}},
  "assessment": {"claim": {"kind": "Result", "subject": "atom/*"}}
}
```

| Probe | Satisfied when | Reads | Runs |
| ----- | -------------- | ----- | ---- |
| `path` | a git pathspec matches at least one **tracked** file | git | always |
| `tag` | a git tag glob matches at least one tag | git | always |
| `claim` | a live claim exists satisfying **every** declared predicate | kan | always |
| `command` | the command exits zero | — | only with `--run` |

`path` uses `git ls-files`, so an untracked build output or a stray local
file cannot witness a telos — being committed is the stronger claim, and it
costs no new dependency.

**A `claim` probe is how a record-shaped witness becomes checkable.** Some
artifacts are not files or tags: a `verdict` is what `day review record`
appends, an `assessment` is what `kan result` records. Until v0.7 neither was
probeable at all, and day's own position could never narrow (day#60).

A claim shape is a **conjunction of independent predicates**. `kind` is
required; the rest are optional, each narrows on a different dimension, and a
claim must satisfy **every** one that is declared. An omitted predicate
constrains nothing.

| Field | Required | Matches when the claim… |
| ----- | -------- | ----------------------- |
| `kind` | yes | is of that kan claim kind, exactly as `kan show --json` renders it — `Observation`, `Plan`, `Decision`, `Result` |
| `contains` | no | has text containing this substring **anywhere** |
| `starts_with` | no | has text **beginning** with this prefix |
| `subject` | no | lives on a subject this **glob-lite** pattern admits |

`kind` alone is almost always too broad — a `Decision` alone matches every
decision in the log — so narrowing is the normal case, and picking *which*
dimension to narrow on is the part worth thinking about:

- **`starts_with` is anchored; `contains` is not.** That is the whole reason
  both exist. `day review record` writes its marker at the *start* of a
  verdict's text, so `{"starts_with": "adversarial review of"}` matches real
  verdicts and not the decision that merely *defined* that marker mid-sentence
  — which a `contains` probe cannot tell apart, and did not (day#70).
- **`subject` is glob-lite, not a glob.** A value ending in `*` is a prefix
  match on the part before it (`"atom/*"` is any `atom/…` subject, bare `"*"`
  is any subject); a value without one is exact (`"release"` is only the
  `release` subject). A `*` anywhere else is a literal character. This is why
  `assessment` is `{"kind": "Result", "subject": "atom/*"}` rather than a bare
  `Result`: a release note or a session handoff recorded with `kan result` is
  a `Result` too, and used to count as an atom assessment.

Every predicate is a plain string operation rather than a pattern language,
for the reason a telos's `scope` is: a probe definition arrives from a claim,
and a regex engine reading claim-supplied input is a wider surface than this
needs. The conjunction is also the extension point — a further narrowing
dimension arrives as one more optional field and one more conjunct, without
changing how the existing ones are written or read.

**A `claim` probe is a read, and has none of `command`'s three constraints.**
It performs only kan's read verbs, so there is nothing to shell-escape,
nothing for `--run` to gate, and no reason to withhold it over MCP. The rules
below are about *executing what a claim names*, which a `claim` probe never
does. Its one cost is breadth: a claim witness is not tied to a subject, so
answering one reads the whole log. Every claim probe in a single command
shares one such read.

**A probe kind day does not recognize costs that witness, not the schema.**
An entry this version cannot parse is reported as `ERROR` — unchecked, not
absent, so it never counts against a telos — and the rest of the map still
resolves. This mirrors what day requires of kan's `--json` shape, and it was
added after a `claim` probe recorded on day's own log made the installed
older binary fail the entire witness map, taking the session hook and status
line with it.

**A `command` probe is day's third substrate**, after kan and git, and the
only one that executes anything — `path`, `tag`, and `claim` are all reads.
Four rules bound it, and they bound it alone:

- **No shell, ever.** The argv is split on whitespace and executed directly.
  A probe declared as `true; touch /tmp/x` runs `true` with the literal
  arguments `;`, `touch`, `/tmp/x` — metacharacters arriving from a claim
  cannot become operators. This costs pipelines and redirection in probe
  definitions, which is the right trade for a check whose value is being hard
  to game.
- **Opt in per invocation.** Without `--run`, a command probe reports
  `NOT RUN` along with the exact argv, so you see what you would be
  authorizing.
- **Never over MCP.** The `assess_telos` tool has no parameter that could
  authorize execution.
- **Bounded.** `--timeout` (default 120s); a probe that outlives it is killed
  and reported as `TIMEOUT`.

**Not-run and timed-out are not failures.** They are absence of evidence, not
evidence of absence; only a probe that ran and found nothing counts against
the telos, and only that sets a non-zero exit.

**Prose never counts as material.** If a claim on the telos subject mentions
a witness type, day reports that separately and never as evidence — a project
asserting its own success is precisely what an assessment is meant to be
checkable *against*.

A telos with no witnesses, or a witness with no declared probe, is named as
not mechanically assessable rather than passed silently.

### Scoping a witness to a telos

A project's probe map says what a witness type *means*. It cannot say which
instance belongs to *this* telos — and that gap produced a real false
positive: `telos/v05-shipped` ("day v0.5 is published") reported its
`published-artifact` witness satisfied by the **v0.4** tag, because the
project probe was `{"tag": "v*"}`.

So a telos may narrow which instances count, in its own `day-telos` block:

```day-telos
{"witnesses": ["published-artifact"],
 "scope": {"published-artifact": "v0.5*"}}
```

```bash
day telos declare v05-shipped "day v0.5 is published." \
  --witness published-artifact --scope 'published-artifact=v0.5*'
```

**Quote any argument containing a `*`.** This example was unquoted through
several releases and does not run in zsh — macOS's default shell, and this
repo's:

```
zsh:1: no matches found: published-artifact=v0.5*
```

The shell expands the glob before day sees the argument, and zsh errors on a
failed match rather than passing it through the way bash does. Found by copying
this page's own example and having it fail (day#83), which is the same class of
defect as the `kan result` invocation this page documented for several releases
without anyone running it.

**A scope narrows; it does not override.** The project map still decides
which *kind* of probe runs — the scope only replaces its pattern. Two
consequences, both deliberate:

- **Weak equivalence survives.** `v0.5*` still admits `v0.5.0-beta.1`,
  `v0.5.0`, and `v0.5.1`, so it names a narrower equivalence class rather
  than one artifact. A telos that named a single instance would have
  collapsed onto it, which is the thing witnesses exist to prevent.
- **A scope never applies to a `claim` probe** either, and day says so. A
  scope replaces *the* pattern argument, and a claim shape has no single one
  — it is a conjunction of several predicates, so there is nothing for one
  string to replace, and picking one for it would let a telos *widen* which
  claims count rather than narrow them, since a schema's predicate is usually
  the more specific.
- **A scope never applies to a `command` probe**, and day reports that it
  was ignored. Honouring it would let a telos claim decide what day
  executes; commands originate only from `schema/witness`, which is one
  subject to review rather than every `telos/*` in the log.

Scope is optional and additive: a `day-telos` block written before it
existed is still valid and assesses identically.

## Assessments

An assessment is the claim that some work did (or did not) land inside a
telos's equivalence class. Record it on the telos subject, citing the
evidence:

```bash
kan result telos/interface-legibility \
  "v0.2 shipped: interface-legibility holds -- \`day --help\` names all four verbs, \
tests/cli.rs::init_prints_both_install_paths covers the setup path." \
  --cites <cid of the claim being assessed>
```

Note the shape: `kan result` takes the **subject positionally**, as
`<SUBJECT> <TEXT>`, while `observe`, `plan`, and `decide` take `<TEXT>` with
`--subject`. This page once documented `kan result` with `--subject`, which
did not run at the time — the asymmetry was kan#78. kan has since resolved it
by accepting **both** spellings, so the `--subject` form now works too; the
positional form above is what day emits and what `tests/kan_conformance.rs`
exercises against the real binary. Copy the form above.

Assess against **material evidence** — builds, tests, diffs, deployed
behavior — not against an agent's own account of what it did. kan's log is
append-only, signed, and content-addressed precisely so that this kind of
check has something non-gameable to stand on; an assessment that cites only
another narrative claim is worth much less than one citing an artifact.

## Position and the render cache — `.day/`

day infers **where the work currently sits** in the atom graph, from
artifacts alone. An atom is a *candidate* for "current" when its declared
inputs are materially present and its outputs are not yet — the work to run it
exists, and what it would produce does not. This reads the same
`schema/witness` probes an assessment does, so **nothing is tracked and
nothing is recorded**: position is recomputed each time, and
[the refusal to track "how far along are we"](#assessments) stays intact.
Ambiguity is *reported, not resolved* — when several atoms fit the evidence,
all are named, because guessing one would be a claim day cannot support.

Three rules bound inference:

- It **reads; it never executes.** `path`, `tag`, and `claim` are all reads
  and all run. `command` does not: inference happens on every session start,
  and executing project-declared commands as a side effect of *starting a
  session* would be a far larger widening than `--run`. A type whose probe is
  a command is reported as unknowable, not silently absent. The line is
  read vs. execute, not probe vs. probe.
- It is **relative to the current cycle** (below).
- **Off-sequence** is reported when a downstream atom's outputs are present
  while an upstream atom's outputs are *demonstrably* absent (probed and not
  found) — a skipped step. An upstream whose output is merely unprobed is
  unknowable, not evidence of a skip.

### The cycle boundary

On a repo with any history, *every* artifact type exists: there is always some
`v*` tag, some past verdict, some old assessment. So "does one exist" can only
ever answer yes, and day's own log reported four candidate atoms permanently
(day#60). Position therefore resolves each probe against a **cycle boundary**
— the last release, taken as the newest `v*` tag:

| Probe | Present, for position, when it |
| ----- | ------------------------------ |
| `path` | **changed since** the boundary (`git diff --name-only <tag> -- <pathspec>`) |
| `tag` | was **created since** the boundary |
| `claim` | was **recorded since** the boundary (`recorded_at` strictly greater) |

The boundary is derived from git on every read and **never stored** — day owns
no state, and a stale cached boundary would be worse than none.

`v*` is the **default**, not a rule: a project that does not release on `v*`
tags declares its own pattern on `schema/cycle` (see *Other declarations day
reads*), and position and `day assess docs` both resolve against the declared
boundary — one boundary, or the two would answer "since when" differently.

Consequences worth stating, because each is easy to mistake for a bug:

- **No release means no boundary**, and inference falls back to the
  cumulative, tracked-ever reading. Treating an unbounded repo as "everything
  is the current cycle" would make a fresh clone report every atom as current.
- **The boundary tag does not witness its own cycle** — it closed the previous
  one. Since the boundary is always the *newest* `v*` tag, a `tag` witness is
  absent under every boundary. `release` stops being current not by observing
  its own output but because cutting the tag opens a new cycle in which its
  input has not changed yet. Releasing ends a cycle, so it cannot also be
  evidence within it.
- **An undated claim is not this cycle's.** `recorded_at` is optional in kan's
  shape; a claim without one cannot be placed in a cycle, and reads as not
  current rather than as current work.
- **Work committed before the tag is last cycle's**, even if it was meant for
  this one. Cutting a release after next-cycle work has already landed on the
  default branch puts that work behind the boundary. This is the tree talking,
  and it is accurate about the repository even when it is surprising.

**Cycle-relativity is confined to position** — `day status`, the status line,
and the session hooks, including the `done` criteria status displays.
`day assess telos` and `day assess atom` are **cumulative and unchanged**:
they ask whether a witness was *ever* produced, and a release or a review from
any time is real evidence that work landed inside a telos's equivalence class.
Bounding assessment would make last cycle's shipped telos start reporting as
unmet the moment a new tag was cut — a regression invented entirely by the
tool. The two paths are separate functions rather than one function with a
flag, so this holds by construction.

`day status` renders this for a human: the current atom or candidates, which
inputs are satisfied, which `done` criteria are met and unmet, what the graph
says comes next, and any off-sequence finding. It **always exits zero** — it
reports, it does not gate; `day assess atom` is the gate.

**Transitions.** `day status` also reports when the work has moved *past the
atom you last recorded assessing*. The baseline is **claims, not the cache**:
the most recent assessment (`kan result`) recorded on any `atom/<slug>`
subject names the atom you last checked, and if the inferred position no
longer includes it, the work has moved on. day **reads those claims and never
writes them** — recording position would make day a task tracker, and
auto-writing a baseline would let the tool manufacture its own evidence. The
mechanism inherits claim semantics for free: retract the assessment and the
baseline is gone; a newer one supersedes it; whose assessments count is the
same locally-signed rule `practice` uses. `day assess atom` prints the
runnable `kan result atom/<slug>` that records one.

**The render cache** lives under `.day/`, is **gitignored**, and holds **only
rendered display state** — the status line's text, nothing an input to any
decision. It is written by `day hook session-start` (which already reads kan
and has time) and read only by the status line, which cannot afford to shell
out: Claude Code cancels an in-flight status line at 300 ms, so a line that
read kan directly could be cancelled before rendering anything. Its **absence
is never an error** — delete it and it regenerates next session. It is not a
store: strictly derived from kan and git, never authoritative, it stands in
the same relation to kan's log as kan's own disposable `.kan/index.sqlite`
does. Exactly one module (`src/cache.rs`) touches it, and a source scan keeps
it that way — *if day ever read the cache to decide something rather than to
display something, the line would have been crossed*.

## Declared block types — `schema/blocks`

day owns seven fenced block types and, until v0.7, a project could invent
none. A research program instantiating day's process for a non-software domain
needed exactly one more — `research-claim`, carrying a claim's evidential
station — and had nowhere to put it (day#74).

A project declares a block type by recording a `day-blocks` block on a
`schema/blocks` subject: a map from block name to its field spec.

```
kan observe "Our block vocabulary" --subject schema/blocks
```

````
```day-blocks
{
  "research-claim": {
    "required": ["medium", "anchor_ref"],
    "optional": ["decay_note", "scope_coords", "situated_verdict"]
  }
}
```
````

A claim then carries an instance in a fence of that name:

````
```research-claim
{"medium": "anchor-verified", "anchor_ref": "lean:Thm1"}
```
````

**The spec is names and required/optional only** — no value types, no pattern
language. A schema arrives from a claim, and a richer language is a wider
surface than the declared need (day#34, and the same line `subject` holds in a
claim probe). day validates that every required field is present and that
nothing undeclared appears; it will **not** catch `medium: 7` where a station
name was meant. A project that needs values interpreted interprets them in its
own linter.

**A declared block is validated wherever day reads one, on the same terms as a
built-in.** A missing required field or an undeclared field is refused, and the
refusal names the field. Declared blocks carry the same `_version` gate, so a
project can version its own vocabulary and an older day says *"this day reads
`research-claim` v1, this block declares v2"* rather than reporting the
project's claim as malformed. An unreadable declaration is reported through
the same channels a built-in's is — a vocabulary day silently ignored would be
a decorative declaration.

**day reads a declared block because a witness asks it to.** The `claim` probe
takes a `block` predicate:

```json
{"claim": {"kind": "Observation", "subject": "claim/*", "block": "research-claim"}}
```

which is satisfied by a claim carrying a **valid** instance of that declared
type. The verdicts follow the rule the rest of day follows — day never reports
an absence it did not check:

| state | verdict |
|---|---|
| the declaration could not be read | `ERROR` |
| the witness names a block type nobody declared | `ERROR` |
| an instance declares a `_version` this day does not read | `ERROR` |
| an instance is present and violates the spec | `MISSING` |
| an instance is present and valid | `MATERIAL` |

**The seven built-in fences are reserved.** Declaring `day-atom`,
`day-telos`, `day-bridge`, `day-witness`, `day-schema`, `day-docs`, or
`day-tension` in `schema/blocks` is refused by name rather than silently
shadowing the built-in.

### Why day's own blocks are not declared this way

The obvious symmetry — make `day-atom` the first instance of the mechanism,
one less special case — is **deliberately declined**, and the reason is
concrete rather than conservative.

day's built-in blocks are struct-defined: the Rust struct *is* the schema,
`deny_unknown_fields` and the `Versioned` trait make it strict at compile
time, and `tests/block_corpus.rs` checks the current reader against every
block shape a released day ever wrote. Declaring them instead would put a
declaration beside the struct **with no compiler between them** — two sources
of truth for one fact, kept in agreement by nobody. That is not hypothetical:
it is exactly the defect v0.7.0-beta.2's adversarial review found in
`extract_fenced`, where a `fence` parameter and the type's own `FENCE`
constant could disagree, and the fix was to delete the parameter.

So: **one mechanism for what day writes, one for what a project invents, and
neither pretends to be the other.** The two are not symmetric because their
guarantees are not symmetric — day can compile-check its own vocabulary and
cannot compile-check yours, and a declaration is the best available answer for
the second case rather than a better answer for the first.

### Other declarations day reads

Three more subjects let a project override a day default. They are **not**
declared block types — their fences are day's own, struct-defined for the
reason above. They are listed here because they are the rest of what `day init`
offers, and each has a working default, so declaring none of them is a complete
configuration.

**`schema/verdicts`** — which verdicts `day review record` accepts. day ships a
default vocabulary; a project whose review process uses different words
declares them rather than being told its own verdict is invalid.

````
```day-verdicts
{"verdicts": ["SHIP", "BLOCK", "COLLISION"]}
```
````

**`schema/cycle`** — which tags bound a cycle, replacing the default `v*`. A
project that releases on `pass/*` or `sprint-*` would otherwise have every
probe measured against a boundary it does not use.

````
```day-cycle
{"tags": "pass/*"}
```
````

**`schema/injection`** — how many prompts pass before day re-shows a standing
condition mid-session. A standing condition is not an event, so it is rationed;
an always-present rule becomes background (day#30).

````
```day-injection
{"cadence": 5}
```
````

Each is read where it is used and falls back to the shipped default when
absent. An unreadable declaration is reported rather than silently replaced by
the default — a project that configured something and got the default anyway
has no way to notice.

## Not yet conventionalized

**Frames.** A telos is always assessed from within some frame of reference —
an actor's own model, with its own internal logic, in which a certificate
that the telos is satisfied is or isn't valid. Frames overlap, nest, and
sometimes fail to reconcile; two agents can hold genuinely incompatible but
individually valid assessments (`docs/TELOS.md`). day does not yet have a
subject convention for frames, and deliberately ships without one rather
than guessing at it. Until it does, an assessment is implicitly in the frame
of whoever signed the claim — which kan already records.
