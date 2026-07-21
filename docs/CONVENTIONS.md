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

`day design record <path>` appends the chain: an `observe` carrying the
validation result, a `plan` for the design citing it, and one `decide` per
bullet under `resolved_section` citing the plan. day assembles `--cites`
from CIDs it captured itself, which is what makes the "pass a file path to
`--cites`" error unreachable rather than merely warned against.

**A document that fails validation is still recorded**, with the result
embedded in the plan claim. Gating would mostly cause people to skip
recording rather than fix the document, and an unrecorded design serves the
record worse than a visibly rough one.

## Assessments

An assessment is the claim that some work did (or did not) land inside a
telos's equivalence class. Record it on the telos subject, citing the
evidence:

```bash
kan result "v0.2 shipped: interface-legibility holds -- \`day --help\` names all four verbs, \
tests/cli.rs::init_prints_both_install_paths covers the setup path." \
  --subject telos/interface-legibility --cites <cid of the claim being assessed>
```

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
