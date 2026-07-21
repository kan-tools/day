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

Deliberately small. The theory is ambitious; the tool is a walking skeleton.

- **`day doctor`** — checks kan is reachable, reads the live atom vocabulary,
  and verifies it still composes: every declared successor exists, and each
  upstream atom's outputs cover its downstream atom's inputs.
- **`day hook session-start`** — the harness integration. Injects the teloi
  in play, the declared atoms, and any drift warnings as advisory context at
  the start of a session.
- **`day mcp`** — the same reads over MCP, for agents without a shell.
- **`day init`** — prints how to wire day into a repo. Prints; never mutates.
- **`/design`** — interactive, codebase-grounded design authoring that records
  into kan.
- **`/adversarial-review`** — a hostile-by-default post-implementation audit
  against a named north star, ending in one of four hard verdicts.

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
cargo install kan   # the memory layer day reads
cargo install day
```

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

Early. v0.1.2-beta.1 is the scaffold: four verbs, two commands, one hook, and
the conventions doc. Declaring a telos, planning a bridge between teloi, and
recording an assessment are all still calling conventions you apply by hand
with kan — not yet verbs day offers. The conventions are v0 and expected to
change.

Where it's going, and why in that order: [`docs/ROADMAP.md`](docs/ROADMAP.md).
Next up is v0.2, the declaring surface — chosen over the more interesting
bridging and assessment work because every real bug so far was found by using
day on day, so the version that gets it used daily beats the version that adds
the most model.

## License

MIT
