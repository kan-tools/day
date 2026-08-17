# day

**A process layer for AI-assisted development.** day gives a coding agent a
shared, inspectable account of what the project is trying to achieve, what
step the work is in, and what evidence would count as done. It uses
[kan](https://github.com/kan-tools/kan) as its memory layer, so the durable
record remains signed, append-only, and usable outside any one agent harness.

day is an early prerelease. It is useful today for design, handoff, adversarial
review, process graphs, and evidence-based assessment; its vocabulary and
installation experience are still evolving.

## Quick start

Install the currently tested pair explicitly—Cargo does not select prereleases
unless a version is given:

```bash
cargo install kan --version 0.13.0-beta.1
cargo install day --version 0.13.0-beta.1
```

Then, from a git repository:

```bash
day init       # records the baseline schema and prints integration steps
day doctor     # checks that the declared process graph composes
day config     # shows every effective setting and where it came from
```

For Claude Code, install this repository as a plugin to add the skills, hooks,
and MCP server:

```text
/plugin install <path to this repository>
```

The fastest useful loop is:

1. Run `/design` to turn intent into a checked design and record its decisions.
2. Implement the design.
3. Run `/adversarial-review` against the design's north star.
4. Use `/handoff` before leaving and `/wakeup` when resuming.

See [CONVENTIONS](docs/CONVENTIONS.md) for record shapes, [TELOS](docs/TELOS.md)
for the model, and the [roadmap](docs/ROADMAP.md) for planned work.

## Why day and kan are separate

kan is a generalizable layer for structured *knowledge* that happens to work
well for software development. day is a generalizable layer for structured
*process* that happens to work well for software development. They are
separate on purpose — you can hold onto your memory substrate while throwing
away every opinion in this repo.

## The model, briefly

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

The fuller model, including the parts not yet implemented (frames as internal
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
- **`day stream list`** — one bulk-read inventory of visible live handoff
  streams, with claim counts, bounded previews, and timestamps when known.
  Withheld, unaccounted, or failed published reads make completeness explicit;
  it never guesses another stream's branch, worktree, position, or staleness.
- **`day design check`** — validates a design document against the schema
  your project declared in kan.
- **`day design record`** / **`day review record`** — append the claim chain
  for a design pass, or a four-value review verdict citing what it audits.
- **`day acquired-input record`** / **`day intervention record`** — explicit,
  validated Observation conventions for useful human input and material
  interventions. They preserve the active signer, citations, and reported
  versus separately signed source provenance; neither is automatic.
- **`day hook session-start`** — the harness integration. Injects the teloi in
  play, the declared atoms, what's still open, and any drift warnings — and
  after a Claude Code compaction, explicitly redirects the session to re-read
  and verify the durable record before continuing — and
  renders the **footer** the status line shows: position (`☀️ atom: build`),
  repo/branch/sync/checkout context, the signing role, and one message tray.
  Emoji unless a knowable negative (non-UTF-8 locale, `TERM=dumb`, `NO_COLOR`)
  says otherwise; `DAY_FOOTER=plain|emoji` overrides either way, and the
  footer fits itself to `COLUMNS`, eliding visibly rather than silently.
- **`day mcp`** — the checks over MCP, for agents without a shell.
- **`day config [--json]`** — a read-only inventory of every configuration
  value day consumes, including its effective layer, claim provenance, and any
  declaration day could not interpret.

**Commands**
- **`/askme`** — gather semi-structured human input one question at a time,
  separating facts, decisions, unresolved items, and material effect. It writes
  nothing unless the person explicitly asks to record the final summary.
- **`/design`** — interactive, codebase-grounded design authoring that records
  into kan and validates against your schema.
- **`/adversarial-review`** — a hostile-by-default post-implementation audit
  against a named north star, ending in one of four hard verdicts. It has been
  run on day itself, and it returned BLOCK.

## Two rules that are load-bearing

**day stores nothing of its own.** Every durable thing it knows is an ordinary
kan claim under the conventions in [`docs/CONVENTIONS.md`](docs/CONVENTIONS.md)
— teloi on `telos/<slug>` subjects, process atoms on `atom/<slug>` subjects
carrying a fenced `day-atom` interface block. day reads and appends them
through kan's public CLI only — never touching kan's storage, its signing, or
its log format. The guarantee that matters is that **day cannot alter or
destroy a subject**: it only ever appends, and kan exposes no destroy path to
reach. ("day never writes" was the earlier phrasing and was a proxy for this;
it stopped being true in v0.2, when `day design record` and `day review
record` landed.) This is kan's own ADR-18 boundary rule observed from the
other side: kan owns durable claims and pure reads over them; day is entirely
a calling convention over those primitives.

**Advisory, never blocking.** day's hooks inject context. They do not gate,
deny, or reject an agent's action, and a test enforces that the shipped hook
config contains no blocking construct. Agents act; the record is made
legible; drift surfaces in the graph as data. This is the specific friction
day exists to avoid repeating.

## Installation details

See [Getting started](docs/GETTING_STARTED.md) for direct and plugin setup,
verification, version compatibility, worktrees, and common recovery paths.

## Status

Early. **v0.13.0-beta.1 is a candidate.** It makes active work more economical
to recover: visible handoff streams have one honest bulk-read inventory,
handoff measurements retain executable immutable scopes, and compaction gets a
distinct reorientation path. `/askme`, acquired-input records, and intervention
records preserve explicit consent and provenance without retaining transcripts.
The candidate contract, real-harness protocol, reconstruction controls, and
publication dependencies are repository-owned under `.release/`; passing
feature tests alone is not presented as a release verdict.

This boundary also fixes newest-practice projection and lets a recorded Plan
directly cite its normative claims. RFC 1 remains the semantic constraint, but
kan's not-yet-shipped RFC 1 authorship surface and the deeper retrieval/profile
work are deliberately bumped rather than approximated with legacy roles.

Each fix moved the guarantee into a mechanism rather than into attention: one
fenced-block scanner with four states instead of two readers that disagreed, the
same validation on the assembled value as on the claim, and an outcome
vocabulary in which a cell cannot express a verdict about a program it never
ran. Four adversarial reviews across two branches found them, and each fix round
introduced the next round's finding until the duplicate reader was deleted
rather than repaired.

**v0.12.0-beta.2** was a bugfix boundary, and what it fixed is mostly
day's own advice. `day design check` told an author to name the requirement each
criterion covers, and following that suggestion parsed every criterion to zero
with nothing naming the cause; citing a kan subject warned it was a missing file;
resolutions written as headings recorded nothing at all. The harness that
answers "did this change alter behaviour it was not meant to alter" reported
agreement for fixtures that compared nothing. And a read could not tell *nothing
is recorded here* from *this view cannot see what is recorded here*, so under a
narrowed trust base day reported an empty project over a log it could not read.
Three cold reviews found those, in that order; each returned BLOCK and each
finding was reproduced against a running binary before being fixed.

**v0.12.0-beta.1** extends verification to teloi. A witness could only
ask *does one exist*, and over an append-only log that question starts
answering yes and never stops — so `every` (wherever the anchor holds, the
requirements hold on the same subject), `absent` with a vacuity guard,
correspondence, authorship scoping, and a declare-time check that reports a
witness already satisfied or structurally unable to stop matching. `day atom
declare` runs that check too, which it never did. Alongside it: `/handoff` and
`/wakeup`, a pair of commands that carry a working thread across a session
boundary by writing claims the other side verifies rather than believes.

**v0.11.0-beta.2** is where verification stopped being something day
asserted about itself. A green suite says nothing about whether a test asserts
anything, so a claim of coverage now carries a mutation; a fix that closes a
review finding carries a `Demonstrated-by:` trailer produced by reverting the
fix and watching the finding's own test fail; and a script accounts for every
commit on a branch as demonstrated, exempt-with-a-reason, or **unaccounted**,
which is the only verdict. Each replaced a check that could not fail — a
mutation harness that reported `SURVIVED` for a mutation that did not compile,
a `pub fn` whose only callers were its own tests, a hand-written table that was
wrong in three consecutive review rounds.

**v0.7.0-beta.2** made day honest about the declarations it cannot
read — which sounds small and was not: day was certifying conformance to
declarations it had silently truncated.

Two runs against the previous release show it. An atom declaring a field this
day had never heard of loaded as though the field were absent and reported
`composition: ok` at exit 0. A project declaring `forbidden_sections` got nine
`[PASS]` lines and exit 0 on a document containing that section. Seven fenced
vocabularies, none rejecting unknown fields — and every one of them exists to
*constrain* something day then reports on, so a dropped field was never lost
information, it was a false certification.

Blocks now refuse what they cannot account for, and carry a `_version` so the
refusal can say **why**: *this day reads `day-atom` up to v2, this block
declares v3 — upgrade day* rather than a parse error that reads as the
project's mistake. The version is the reader version a block *requires*, so it
appears only when a block actually uses something older readers lack — `next`
and `revisits` are the worked example: an atom with no feedback edge is still
byte-identical to what v0.2 wrote. A
failed kan read reports as `[UNCHECKED]` rather than as an absent artifact. The
session hooks say when their own lists are partial, on both the model's channel
and the human's. And a **version-migration matrix** in CI records what every
released version does with the block shapes the current commit writes — because
the answer turned out to be "silently widens", for every version that can read
them at all, and nobody knew.

**v0.7.0-beta.1** remains the foundation: it made day's own position *legible*
rather than merely computed.

Position inference used to ask "does an artifact of this type exist", and on
any repo with history the answer is always yes — some `v*` tag, some past
verdict. day reported four candidate atoms on its own log and could never
narrow. Position now resolves each probe against a **cycle boundary** (the last
release): a path counts if it *changed since*, a tag if it was *created since*,
and — via a new **`claim` probe**, the first that reads the kan log rather than
the working tree — a witness like `verdict` or `assessment` counts if it was
*recorded since*. Assessment stays cumulative, because "was this ever produced"
is the right question for a telos and the wrong one for "where is the work now".

v0.6 remains the foundation this stands on: atoms carry `done` criteria,
`day assess atom` exits non-zero when one is unmet, and `day status` plus a
**status line** put the inferred position in front of the human. Enforcement
stays at the artifact level, never the action.

**Frames is still in v0.7**, targeted for a later beta rather than deferred a
third time. The roadmap says why in the open. The conventions are v0 and
expected to change.

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
