# day

**Structured process for AI-assisted development.** day is the process layer
that sits next to [kan](https://github.com/kan-tools/kan)'s memory layer.

kan is a generalizable layer for structured *knowledge* that happens to work
well for software development. day is a generalizable layer for structured
*process* that happens to work well for software development. They are
separate on purpose — you can hold onto your memory substrate while throwing
away every opinion in this repo.

## The idea

When you write software you hold several plausible futures in mind at once.
Each satisfies some purpose, each pulls the work forward, and they are
usually in tension with each other: a clean interface, a thing that actually
works in the world, something people will use. Those purposes are **teloi**,
and a telos is a state of the world defined only *up to weak equivalence* —
it names an invariant, a shape the world should have, not a point-target.
Many different concrete outcomes satisfy the same telos equally well, which
is exactly why several can apply at once without any of them being wrong.

day's job is to keep track of those teloi as they drift, help decompose the
gap between here and there into composable units of work, and make the
assessment of where you actually are auditable rather than vibes-based.

The model, including the parts not yet implemented (frames as internal
toposes, cross-frame reconciliation, realizability as a sheaf condition, the
polynomial-functor treatment of composition), is written up in
[`docs/TELOS.md`](docs/TELOS.md). The grounding reference is David Spivak's
[Plausible Fiction](https://topos.institute/blog/2024-08-27-plausible-fiction/).

## What it actually does today

Deliberately small. The theory is ambitious; the tool is not.

**Declaring the vocabulary**
- **`day telos declare`** — declare a telos, or revise it by declaring again.
  There is no `revise` verb: kan is append-only, so a revision *is* a later
  claim, and day cites the previous one for you.
- **`day telos tension`** — record that two teloi pull against each other,
  and why. The relation that makes teloi more than a values list. It writes a
  real `in-tension-with` edge so the tension is *queryable*, plus a claim
  carrying the reason, since a kan relation has no narrative body.
- **`day atom declare`** — declare a process unit with its input, output, and
  successor types. day generates the interface block; you never hand-write it.
- **`day init`** — records the baseline design-doc schema, and *prints* the
  harness wiring. It writes claims, never config.

**Planning**
- **`day bridge declare`** — a planned arrangement of atoms aimed at a telos:
  `a > b` in sequence, `a & b` concurrently, `a | b` as alternatives.
- **`day bridge check`** — could this plan actually get there? Walks the
  arrangement, verifies each step's inputs are available where it sits, and
  reports whether the target's declared witnesses are produced.

**Assessing**
- **`day assess docs`** — do the docs still match what shipped? Checks
  declared version-carrying files, reconciles the last recorded release
  against the last git tag, and prompts when code changed and no watched doc
  did.
- **`day assess telos`** — did work land inside a telos's equivalence class?
  A telos declares *witnesses* (types of evidence); a project declares
  *probes* saying what would count — a tracked file, a git tag, or a command
  that exits zero. Two tiers: material evidence can fail the run, what the
  log says only prompts, and prose is never counted as evidence. Command
  probes execute only with `--run`, never over MCP, never through a shell.

**Reading and checking**
- **`day doctor`** — verifies the live atom vocabulary still composes: every
  declared successor exists, and each atom's inputs are covered by what its
  upstream atoms produce.
- **`day next <atom>`** — what the graph says follows this step, and what it
  needs. Ask this instead of assuming a fixed pipeline.
- **`day design check`** — validates a design document against the schema
  your project declared in kan.
- **`day design record`** / **`day review record`** — append the claim chain
  for a design pass, or a four-value review verdict citing what it audits.
- **`day hook session-start`** — the harness integration. Injects the teloi in
  play, the declared atoms, what's still open, and any drift warnings.
- **`day mcp`** — the checks over MCP, for agents without a shell.

**Commands**
- **`/design`** — interactive, codebase-grounded design authoring that records
  into kan and validates against your schema.
- **`/adversarial-review`** — a hostile-by-default post-implementation audit
  against a named north star, ending in one of four hard verdicts. It has been
  run on day itself, and it returned BLOCK.

## Two rules that are load-bearing

**day stores nothing of its own.** Every durable thing it knows is an ordinary
kan claim under the conventions in [`docs/CONVENTIONS.md`](docs/CONVENTIONS.md)
— teloi on `telos/<slug>` subjects, process atoms on `atom/<slug>` subjects
carrying a fenced `day-atom` interface block. day reads them back through
kan's public CLI and never writes. This is kan's own ADR-18 boundary rule
observed from the other side: kan owns durable claims and pure reads over
them; day is entirely a calling convention over those primitives.

**Advisory, never blocking.** day's hooks inject context. They do not gate,
deny, or reject an agent's action, and a test enforces that the shipped hook
config contains no blocking construct. Agents act; the record is made
legible; drift surfaces in the graph as data. This is the specific friction
day exists to avoid repeating.

## Install

```bash
cargo install kan --version 0.7.0-beta.1   # the memory layer day reads
cargo install day --version 0.6.0-beta.1
```

**The versions are not decoration.** Everything day has published is a
pre-release, and cargo will not select one without `--version` — a plain
`cargo install day` errors. kan does have one stable version, `0.1.0`, so a
plain `cargo install kan` "succeeds" and hands you a kan six minor versions
old that day cannot talk to, which is the worse failure because it looks like
it worked. Tracked as day#50.

Then, in a repo:

```bash
day init            # prints the wiring steps; writes nothing
```

Or install the whole thing — commands, session hook, and MCP server — as a
Claude Code plugin:

```
/plugin install <path to this repo>
```

## Status

Early. **v0.6.0-beta.1** is *"rigor as artifact"*: it leads with checkable
artifacts rather than injected rules, because this project has direct evidence
that a rule in an agent's own context did not survive the moment it applied to.

Atoms now carry `done` criteria — witness types checked through the same probes
teloi use — and `day assess atom` exits non-zero when one is unmet, so CI or a
human can gate on it. day **infers where the work sits** from artifacts alone,
reports it in `day status` and a **status line**, and marks a transition when
the work moves past its last recorded assessment — the first time day's process
has been visible to the *human* it is meant to be legible to, on a channel
verified by watching it render rather than by trusting the docs. Enforcement
stays at the artifact level, never the action: day makes the state of the work
measurable and reports it; it never gates an agent mid-action.

Frames moved to v0.7 — its second deferral, said plainly in the roadmap
rather than renumbered quietly. The conventions are v0 and expected to change.

Where it's going, and why in that order: [`docs/ROADMAP.md`](docs/ROADMAP.md).

Every real bug so far has been found by using day on day rather than by
testing it. Not most — every one. Building v0.5 with day found faults in day
itself: a design-doc checker that miscounted the marker its own template tells
you to quote, a documented `kan` invocation that does not run, and an
assessment that reported a telos's witness satisfied by the *previous*
release. None were visible to a green suite. That is the strongest signal
available about how to sequence what comes next, and the roadmap is ordered
on it.

## License

MIT
