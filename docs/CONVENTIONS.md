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
{"in": ["design-doc"], "out": ["code-change"], "next": ["adversarial-review"]}
```
````

| Field  | Meaning |
| ------ | ------- |
| `in`   | Type names this atom requires to be applicable |
| `out`  | Type names this atom produces |
| `next` | Slugs of atoms this one composes into |

Type names are free-form strings. day checks that they *match*; it
deliberately does not check what they mean. The type vocabulary is the
project's to choose and evolve.

```bash
day atom declare generative-build \
  --in design-doc --out code-change --next adversarial-review \
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
  "passing-tests": {"command": "cargo test"}
}
```

| Probe | Satisfied when | Runs |
| ----- | -------------- | ---- |
| `path` | a git pathspec matches at least one **tracked** file | always |
| `tag` | a git tag glob matches at least one tag | always |
| `command` | the command exits zero | only with `--run` |

`path` uses `git ls-files`, so an untracked build output or a stray local
file cannot witness a telos — being committed is the stronger claim, and it
costs no new dependency.

**A `command` probe is day's third substrate**, after kan and git, and the
only one that executes anything. Four rules bound it:

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
  --witness published-artifact --scope published-artifact=v0.5*
```

**A scope narrows; it does not override.** The project map still decides
which *kind* of probe runs — the scope only replaces its pattern. Two
consequences, both deliberate:

- **Weak equivalence survives.** `v0.5*` still admits `v0.5.0-beta.1`,
  `v0.5.0`, and `v0.5.1`, so it names a narrower equivalence class rather
  than one artifact. A telos that named a single instance would have
  collapsed onto it, which is the thing witnesses exist to prevent.
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
`--subject`. This page previously documented `kan result` with `--subject`,
which does not run. Copy the form above rather than the sibling verbs'.

Assess against **material evidence** — builds, tests, diffs, deployed
behavior — not against an agent's own account of what it did. kan's log is
append-only, signed, and content-addressed precisely so that this kind of
check has something non-gameable to stand on; an assessment that cites only
another narrative claim is worth much less than one citing an artifact.

## Not yet conventionalized

**Frames.** A telos is always assessed from within some frame of reference —
an actor's own model, with its own internal logic, in which a certificate
that the telos is satisfied is or isn't valid. Frames overlap, nest, and
sometimes fail to reconcile; two agents can hold genuinely incompatible but
individually valid assessments (`docs/TELOS.md`). day does not yet have a
subject convention for frames, and deliberately ships without one rather
than guessing at it. Until it does, an assessment is implicitly in the frame
of whoever signed the claim — which kan already records.
