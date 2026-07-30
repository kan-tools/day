# day — agent working notes

You are building `day`: the structured **process** layer that sits next to
`kan`'s structured **knowledge** layer. This file orients you; `docs/` is
authoritative.

## Read first, in order
1. `README.md` — what day is and what it does today.
2. `docs/TELOS.md` — the model: teloi as weak-equivalence invariants, frames
   as internal toposes, bridging states, atoms and composition. This is
   theory, and most of it is not implemented. Do not treat unimplemented
   theory as a backlog to burn down.
3. `docs/CONVENTIONS.md` — AUTHORITATIVE for what day actually reads and
   writes. If TELOS.md and CONVENTIONS.md disagree about the shipped tool,
   CONVENTIONS.md wins.
4. `docs/ROADMAP.md` — where this is going and why in that order. A draft,
   not a schedule; revise it by editing it and recording the change in kan
   (`--subject roadmap`), never by pretending it always said something else.

## The two non-negotiables

**day stores nothing of its own.** Every durable thing day knows is an
ordinary kan claim, read back through kan's public CLI. No config store, no
sidecar database, no state file. If a feature seems to need day-owned
persistent state, that is a signal the feature is wrong or belongs in kan —
stop and reconsider before adding a store.

- **One carve-out, since v0.6: the `.day/` render cache** (`src/cache.rs`). It
  is not a store and does not weaken the rule, because nothing *durable* lives
  in it: it holds only the rendered status line, is gitignored, is strictly
  derived from kan and git, regenerates on the next session start, and is never
  read to *decide* anything — only to *display*. It exists solely because
  Claude Code cancels an in-flight status line at 300 ms, so the kan reads must
  happen in the session-start hook and the result be cached for the line. It
  stands in the same relation to kan's log as kan's own disposable
  `.kan/index.sqlite` does to it. The line that keeps it honest: exactly one
  module touches `.day/`, and `tests/plugin.rs` greps the rest of `src/` to
  prove it. **If day ever reads the cache to decide something rather than to
  display it, the carve-out has been abused** — that is the boundary, and it is
  a source-scanned invariant, not a matter of intent.

**Advisory, never blocking.** Hooks inject context; they never gate, deny, or
reject an agent's action. `tests/plugin.rs` enforces this against the shipped
hook config and is not to be weakened. This is a direct lesson from
`crosslink`, whose blocking hooks caused the integration friction that
motivated splitting day out of kan in the first place.

## Boundary with kan

kan owns a feature iff it needs a new/existing `ClaimBody`/`ClaimKind`/
`Anchor`/`RelationKind` variant, or is a pure read/fold over the claim graph
(kan's ADR-18). day owns everything buildable as a calling convention over
kan's existing primitives — process, orchestration, multi-turn interaction.

If a day feature would require changing kan's data model, that is a kan
design question first, raised as a kan issue, not something to work around
here.

## House rules

- Rust, matching kan's dependency choices where they overlap (clap, rmcp,
  serde, thiserror, tokio) so the two crates stay easy to read together.
- day has **three substrates**: kan; **git, read-only** (since v0.4); and
  **project-declared commands** (since v0.5), which is the only one that
  executes anything.
  - All git access lives in `src/git.rs`, restricted to read subcommands,
    with a test whitelisting them — day never stages, commits, tags, or
    pushes. git was taken on reluctantly, because kan does not expose claim
    artifacts (kan-tools/kan#61); if that changes, prefer reading the record.
  - All command execution lives in `src/probe.rs`. It exists because
    `docs/CONVENTIONS.md` defines material evidence as *builds, tests,
    diffs*, and a witness like `passing-tests` is not expressible without
    running something. It is bounded by four rules, each with a test in
    `tests/assess_telos.rs` and `tests/assess.rs`: **no shell ever** (argv is
    split and exec'd directly, so metacharacters from a claim stay literal),
    **`--run` opt-in per invocation**, **never reachable over MCP**, and a
    **timeout that kills**. A probe's command comes from a kan claim, so
    these are what keep the log from being an execution path — do not relax
    them, and do not add a fourth spawn site outside these three modules.
  - **Not every probe is a command, and the `claim` probe (since v0.7) is
    bound by none of those four rules.** It reads the kan log through the
    same public read verbs `atoms::load` uses, so there is nothing to
    shell-escape, nothing for `--run` to gate, and no reason to withhold it
    over MCP. The guardrails above are about *executing what a claim names*,
    which is narrower and sharper than "probes are dangerous" — folding
    `claim` into them would be a category error, and a test asserts it never
    reaches `run_command`. **The line is read vs. execute**, and position
    inference draws it in exactly that place: `path`, `tag`, and `claim` all
    run at session start; `command` never does.
- day talks to kan by **shelling out to the `kan` binary**, never by linking
  it as a library. The boundary is the public CLI on purpose: it's the same
  contract any other consumer gets, so day can't quietly depend on kan
  internals.
- **day writes only through kan's public CLI.** As of v0.2 day appends
  claims (`day design record`, `day review record`), but always by invoking
  `kan observe`/`plan`/`decide` as a subprocess — never by touching kan's
  storage, its signing, or its log format. kan remains the only thing that
  decides what a claim *is*.
  - The guarantee that matters is unchanged: **day cannot alter or destroy a
    subject.** It only ever appends, and kan exposes no destroy path to
    reach. Earlier versions said "day is a reader", which was a proxy for
    this; the proxy stopped being true in v0.2 and the real invariant is
    stated directly rather than worked around.
  - What day must never do: write kan's files directly, bypass its signing,
    or keep a store of its own. If a feature seems to need any of those,
    it is wrong or it belongs in kan.
  - day never retracts or rejects. Superseding is done by appending, the
    same way kan does it.
  - (Caveat, stated precisely rather than papered over: kan initializes a
    `.kan/` workspace on first use, so even a day *read* in a repo kan has
    never seen creates an empty log there.)
- Correctness before features. The atom composition check should be boring
  and obviously right.
- Keep the CLI small. Four verbs today. A new verb needs a design doc.
- `DAY_KAN_BIN` exists so tests can stub kan. Every integration test uses it;
  none require a real kan install.
  - **One deliberate exception: `tests/kan_conformance.rs`.** A stub accepts
    whatever day sends it, so stub-based tests validate day against day's own
    idea of kan's CLI, never against kan's contract — which is how
    `docs/CONVENTIONS.md` documented a `kan result` invocation that does not
    run, through several releases (day#27, kan#78). That file talks to the
    real binary and **skips when kan is absent**, so the rule above still
    holds. Its hermetic half — that `Write::new` is only ever used with the
    verbs whose subject is a flag — always runs, and is the part that
    actually protects the invariant.

## Dogfood before you trust a test

**Every defect found in day so far came from running it, not from testing
it.** Not most — every one. The composition check's false positive on day's
own atoms, a telos rendering its tension instead of itself, `--title`
silently discarding a title, a session hook wired to an event that cannot
reach the model, `bridge check` reporting the wrong set, a retracted telos
still listed as in play. All seven were invisible to a green suite, because
tests assert day's *output* while the defects were in what that output means
or whether anything receives it.

So: before calling a feature done, use it on this repo or on kan against the
real log. A passing suite is necessary and has never been sufficient here.

Two corollaries worth keeping:
- A check that only inspects its own side of an interface will miss the
  interface. Verify what the *other* side does — read the harness docs, call
  the MCP tool, install the published crate.
- Probes against a real log leave real claims. Use a scratch repo, or retract
  in the same breath. An assessment that pollutes the record it assesses is
  measuring its own footprint.

## Then verify the verifier

Every rule above is about not trusting a test. These are about not trusting the
thing that checks the test. All of them are from one milestone
(`v0.7.0-beta.2`), which makes the pattern hard to dismiss as bad luck.

- **A verification tool must distinguish "could not check" from "checked and
  found nothing."** The mutation harness here reported `SURVIVED` for a mutation
  that *did not compile* — it grepped for `FAILED`, which a build error never
  prints — and on an earlier run a timeout left a mutation in the tree, caught
  only because a separate check noticed a red suite. Both fail toward false
  confidence. This is the *same rule the milestone was about*, violated in the
  tooling that verified the milestone: exit-code precedence, could-not-check
  outranks checked-and-clean, applied to day and not to day's own scripts. A
  mutation harness needs a per-mutation restore, a distinct compile-error
  outcome, and an assertion that the file actually changed.
- **A property claimed in a comment needs a test named after it, and the test
  must assert the property rather than a proxy.** `user_prompt`'s doc comment,
  `hooks/hooks.json`, and the design all said it did not recompute; it
  recomputed on every prompt, 3.03s, for as long as nobody measured. The fix
  pins it as *zero kan invocations*, not as a duration — a timing assertion
  measures the machine and flakes; an invocation count measures the design.
- **Never key a classifier on the absence of a phrase.** The migration cell
  looked for `composition: ok` to mean "loaded it anyway", so an unrelated
  finding — a dangling `next` edge in the fixture — suppressed the phrase and
  filed a reader that silently widened as `errored`. Key on the positive signal:
  did the thing get rendered.
- **Generate expectation tables from a measurement run, then review them.** The
  migration expectations were written from reasoning: eight rows, five wrong. The
  matrix said so the first time it ran, which is the argument for building it,
  and also the argument for never hand-writing what it produces.
- **A generator whose failure mode is "less output" needs an exhaustive
  expectation.** The block corpus silently omitted three of seven block types,
  twice — a verb was refused, nothing was appended, and the coverage was quietly
  smaller with no error. It is now a list of seven fences, not a count.

## Two more, about where a defect hides

- **A mechanism with two modes gets tested in whichever mode this repo is in.**
  The position fingerprint covered files-changed-since-the-boundary and not the
  tracked set, so on any repo with no `v*` tag it was a constant and the
  mid-session hook was inert. day has release tags, so every check passed. The
  broken path was the *default* one — every fresh clone, i.e. exactly the
  population `telos/v1.0`'s bar names. Worse, day had already learned this once:
  `current-cycle-position`'s AC-4 is "no release means no boundary and the
  cumulative reading." The lesson was recorded for position and not carried to
  the thing that gates position.
- **A rule written in one module's doc comment does not propagate to the
  others.** `src/probe.rs` states plainly that a subject day cannot read is an
  error and never a silently empty result. It then happened **five times**:
  `docs.rs` (day#81), `hooks.rs`'s `render_teloi`, `status::compute` discarding
  `atoms::load`'s findings, `BlockSchemas::load(…).unwrap_or_default()`, and
  `InjectionSchema::load(…).unwrap_or(DEFAULT_CADENCE)` — the last written *after*
  this file gained a rule naming it. Prose in the right place is not a
  constraint.

  **It is now a source scan that fails the build**
  (`a_failed_kan_read_is_never_swallowed` in `tests/plugin.rs`), with an explicit
  `kan-read-may-degrade: <why>` escape hatch, because a test with no way out gets
  deleted the first time it is wrong. Verified by reintroducing all five defects.
  If a rule matters, this is the shape it wants — the `.day/` carve-out has the
  same treatment for the same reason.

## Two tools, already written — use them rather than reinventing them

- **`scripts/mutate.py`** — one mutation, honestly reported. A green suite says
  nothing about whether a test *asserts* anything, so a claim of coverage wants a
  mutation. Use this rather than an inline loop: the inline version was
  reinvented at least three times in one session and had a different defect each
  time, all failing toward false confidence. It reports **CAUGHT / SURVIVED /
  DID-NOT-COMPILE / ANCHOR-MISSING** as distinct outcomes — a build error is not
  a survived mutation, and a stale anchor is not a passing test — and restores the
  file visibly (`copy`, not `copy2`, then `touch`, because preserving mtime hides
  the restore from cargo and corrupts the *next* run).
- **`scripts/capture-block-corpus.sh`** — regenerates the backward-compatibility
  corpus by building every released tag and driving that tag's own binary. Run it
  by hand after changing a block shape; `tests/block_corpus.rs` consumes the
  committed output on every push.

## Working practice

- Design goes through `/design` and lands in `.design/<slug>.md` before
  implementation, recorded into kan.
- Post-implementation, run `/adversarial-review` against the design doc.
  Both commands are day's own atoms — dogfood them.
- One PR per milestone: branch off `main`, commit, push, `gh pr create`,
  wait for CI, then `gh pr merge --merge --delete-branch` (regular merge, so
  the milestone's internal commits stay visible).
- Record durable findings and decisions into kan as you go, citing the claims
  they build on. `--cites` takes **CIDs of prior claims, never file paths** —
  capture the CID a write verb prints and chain it.
