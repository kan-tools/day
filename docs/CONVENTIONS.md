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
kan decide "The CLI vocabulary stays legible to an agent that has never seen it before." \
  --subject telos/interface-legibility --title "Interface legibility" --kind Goal
```

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
kan observe "$(cat <<'EOF'
The generative build atom: an agent session turns an accepted design into code.

```day-atom
{"in": ["design-doc"], "out": ["code-change"], "next": ["adversarial-review"]}
```
EOF
)" --subject atom/generative-build
```

`day doctor` reads every `atom/*` subject and checks the set composes: each
`next` target must exist, and an upstream atom's `out` must cover its
downstream atom's `in`. A failure is reported, never repaired — day has no
write path into the log.

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
