# day — agent working notes

You are building `day`: the structured **process** layer that sits next to
`kan`'s structured **knowledge** layer. This file orients you; `docs/` is
authoritative.

## Where the rules are, and why they are not here

**The imperative form of every process rule lives on the `practice` subject in
kan, not in this file.** `day hook session-start` projects them, so a session
inherits them at session start rather than depending on someone having read
this. `src/practice.rs` is the mechanism; one claim per item; the cap is
declared on `schema/injection` and is 30 here because there are more than twelve
of them.

That is not organisation. day exists to inject process, and for most of its life
its own process learnings were written into a Markdown file day does not read —
the self-application gap, one level up from every instance recorded below. A
rule nobody is injected with is a rule that depends on attention.

**So what is this file for.** Three things a claim cannot carry:

1. **Orientation** — what day is, what to read, where the boundary with kan sits.
2. **Architectural invariants** — properties of the *code*, enforced by tests
   that fail the build: the two non-negotiables, the three substrates, the three
   spawn sites. These are not process; they constrain what may be written.
3. **The narrative** — what happened, and why each rule says what it says. A
   rule injected without its history reads as arbitrary, and the histories are
   long. They stay here.

A rule that ends up in both will drift, which this file records as a defect
class twice. If you are about to write an imperative sentence below, it belongs
on `practice`; write the story here and record the rule there.

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
- Keep the CLI small, and **a new verb needs a design doc**. This said "Four
  verbs today" while `day --help` listed twelve, which is the hand-written count
  this repo has now been wrong about in four separate places. The count is not
  the rule and was never load-bearing; the design doc is. Run `day --help` if
  you want the number.
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

A passing suite is necessary and has never been sufficient here. The rule that
follows from it is a `practice` item, injected each session; the history is what
this section is for.

Two corollaries worth keeping:
- A check that only inspects its own side of an interface will miss the
  interface. Verify what the *other* side does — read the harness docs, call
  the MCP tool, install the published crate.
- Probes against a real log leave real claims. Use a scratch repo, or retract
  in the same breath. An assessment that pollutes the record it assesses is
  measuring its own footprint.

## A justification that names a mechanism is a claim about the code

Three decisions in the v0.12 witness work were made by reasoning, survived
review, and died on contact with the code. None was caught by reading it; each
was caught by running it. What they have in common is sharper than "dogfood it",
because in all three the wrong belief was about a mechanism **whose own source
says otherwise, in a comment**:

- **`assess telos` is cumulative, never cycle-scoped**, and the comment at the
  probe call says so at length — scoping to the current cycle "would make last
  cycle's shipped telos start reporting as unmet". A design doc asserted
  cycle-scoping anyway, to argue a witness was falsifiable. It was not, and
  `telos/legible-process` shipped reporting met forever (day#138) — day#86's own
  objection, inside the declaration written to close day#86.
- **`effective_probe` ignores `--scope` for claim probes**, and says why: it
  could *widen* what counts. A resolved question (RQ-10) proposed a witness that
  needed exactly that narrowing.
- **`cfg_test_module_line` documents the `#[cfg(test)]`-cut defect** where one
  `#[cfg(test)] use` exempts a whole file. A new scan hand-rolled the cut and had
  precisely that defect.

This is the kan-conformance floor's lesson one level out. There, the measurement
was real, the failure was real, the diagnosis was right, and the *inference* was
wrong — and "the check that mattered was reading day's own source". Here the
inference was wrong in the same way, three times, in a codebase whose comments
already held the answer.

The rule this produced is on `practice`. What belongs here is why it is worth a
rule at all: in all three cases the wrong belief was about a mechanism whose own
source says otherwise, in a comment. The comments in this repo are unusually
load-bearing — they were written by the people who hit the defect — and they are
the cheapest oracle available.

**The part that should not depend on discipline.** Falsifiability is mechanically
checkable and was left to judgement. A witness over a *monotone* set — a
cumulative claim probe against an append-only log, a path probe over committed
files — can never stop matching, and detecting that needs no counterfactual. It
is the vacuity guard from the other side: a negated probe that can never fire is
vacuous, a positive probe that can never stop firing is equally uninformative.
Both are "this witness cannot distinguish", and one mechanism reports both.

## Then verify the verifier

Every rule above is about not trusting a test. These are about not trusting the
thing that checks the test. All of them are from one milestone
(`v0.7.0-beta.2`), which makes the pattern hard to dismiss as bad luck.

- **Could-not-check reported as checked-and-clean.** The mutation harness here
  reported `SURVIVED` for a mutation
  that *did not compile* — it grepped for `FAILED`, which a build error never
  prints — and on an earlier run a timeout left a mutation in the tree, caught
  only because a separate check noticed a red suite. Both fail toward false
  confidence. This is the *same rule the milestone was about*, violated in the
  tooling that verified the milestone: exit-code precedence, could-not-check
  outranks checked-and-clean, applied to day and not to day's own scripts. A
  mutation harness needs a per-mutation restore, a distinct compile-error
  outcome, and an assertion that the file actually changed.
- **A property claimed in a comment, asserted by nothing.** `user_prompt`'s doc comment,
  `hooks/hooks.json`, and the design all said it did not recompute; it
  recomputed on every prompt, 3.03s, for as long as nobody measured. The fix
  pins it as *zero kan invocations*, not as a duration — a timing assertion
  measures the machine and flakes; an invocation count measures the design.
- **A classifier keyed on the absence of a phrase.** The migration cell
  looked for `composition: ok` to mean "loaded it anyway", so an unrelated
  finding — a dangling `next` edge in the fixture — suppressed the phrase and
  filed a reader that silently widened as `errored`. Key on the positive signal:
  did the thing get rendered.
- **An expectation table written from reasoning.** The
  migration expectations were eight rows, five wrong. The
  matrix said so the first time it ran, which is the argument for building it,
  and also the argument for never hand-writing what it produces.
- **A generator whose failure mode is "less output", checked by a count.** The
  block corpus silently omitted three of seven block types,
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

## And one about measuring the wrong thing accurately

- **A conformance test must separate "what we depend on" from "what they
  promised."** day's first kan-compatibility measurement put the floor at kan
  v0.7.1. It was wrong by a release, and not because the harness was sloppy — it
  built, ran, and reported honestly. One test carried two assertions: that
  `kan result <subject> <text>` runs, which day depends on, and that
  `kan result --subject` *also* runs, which asserts kan#78 was resolved and is a
  fact about **kan**. day emits only the positional form. So a property of the
  dependency silently decided a fact about day, and the fact was user-visible:
  a floor that turns working setups away. **The rule is that a cell measuring
  "does X work against Y" may only run assertions about X's own requirements** —
  every other assertion belongs in a test named for what it actually checks.
  This is `docs/CONVENTIONS.md`'s descriptive-vs-restrictive distinction one
  level up: the same suite was doing both jobs, and only one of them sets a
  floor.

  Note what did *not* catch it: the measurement was real, the failure was real,
  and the diagnosis ("`--subject` is rejected") was correct. What was wrong was
  the inference from it. Running the thing is necessary and was not sufficient —
  the check that mattered was reading day's own source to ask whether day emits
  the form at all, and `src/telos.rs` says in a comment that it does not.

## The one that keeps coming back: a guarantee wired at a call site

Three milestones, three instances, each caught by `/adversarial-review` and none
by the build. Filed as **day#101**; repeated here because this is where a future
session will look.

- `BlockSchemas::extract` validated a declared block and **nothing called it**.
- `unaccounted_subjects()` detected a dropped subject and was wired into
  `status::compute` — so the hook channels were protected and `assess telos`,
  where day publishes *evidentiary verdicts*, reported `[MISSING]` for evidence
  day had never received.
- The fix for that was right about **where** and wrong about **when**: it took
  the subject list *after* the bulk read, so a concurrent append by another
  agent looked like a missing subject.

Both times the real fix was to push the guarantee down into `KanClient`, and
both times a check added at a call site looked complete because the author's
test drove the call site they were thinking about. The rule is on `practice`;
what is worth keeping here is that it took three milestones to see, because a
call-site check passes its author's test every time.

Two corollaries with teeth:
- **`pub` suppresses dead-code detection.** `BlockSchemas::extract` and
  `Compat::is_notable` were both `pub`, both called only by their own tests, and
  clippy was silent for both. A `pub fn` whose only callers are `#[cfg(test)]`
  is either dead or a requirement about to go nominal.
- **A test written to close a finding can assert the wrong side of it.** The
  first test for the `show()` guard drove `session-start`, which reports the
  same string from a *different* mechanism — so deleting the guard **SURVIVED**
  mutation. If a test covers a finding, mutate the exact line the finding was
  about, not the feature around it.

## A fix that closes a finding demonstrates that it fixes something

**Mutation and reversion are different questions**, and conflating them is what
let day#116's first instance survive three review rounds:

- *Mutation* asks: does **any** test assert this line?
- *Reversion* asks: does **the test written for this finding** fail when the
  finding is reintroduced?

Mutating an adjacent line, or the feature around the finding, answers the first
and looks like the second. The rule above — "mutate the exact line the finding
was about" — is not specific enough, because the exact line is usually ambiguous
after a restructure. Reverting the change is not.

Hence the `Demonstrated-by:` trailer, produced by `scripts/revert-demo.py` and
re-derived in CI by `.github/workflows/revert-demo.yml` — a property of the
commit rather than of a reviewer's attention. The rule and its exemption
discipline are on `practice`; the outcomes and their meanings are below, because
that is reference material rather than instruction.

```
python3 scripts/revert-demo.py --tests harness_honesty::the_matrix_does_not_exclude_the_tag
```

**This rule shipped only because the tooling made it nearly free**, which is the
condition `docs/ROADMAP.md` set and the reason the harness was built first: a
rule that costs something on every fix commit with no tooling behind it is
ceremony, and ceremony is what people route around. Measured over v0.11's own
commits: **11.9 s cold, 2.0 s warm**, one command, and the trailer is pasted
rather than written. Qualifying the test target (`plugin::some_test`, not
`some_test`) is what buys that — the unqualified form builds every integration
target three times and took **3m54s**.

**`VACUOUS` is a finding, not a nuisance.** It means the fix was taken away and
the test written to close the finding passed anyway — day#116 itself, and the
commit is not ready.

**Where the rule does not apply.** Each case below names the outcome the harness
actually reports for it, so an exemption is checked against what the tool prints
rather than against this list. That distinction is not decorative: an exemption
claimed from the list rather than from the tool survived two review rounds, and
another named an outcome the tool does not produce.

- **A guard rather than a fix** has nothing executable to invert, and what the
  harness says depends on what else the commit touched. day#89's guard reports
  `REVERT-FAILED`, whose message is a *disjunction* — "Either the change is
  test-only, or `--include`/`--exclude` excluded the fix" — so it narrows rather
  than decides. day#101's reports `VACUOUS`, because its only non-test change is
  a design document and reverting prose cannot fail a test; that is the harness
  having nothing to work with, not a test failing to observe its finding.
  A guard demonstrates by being shown to **fire** — both directions, and against
  the instance it was written for where one exists. day#101's scan checks out the
  tree at `1e02220^` and asserts it finds exactly `Compat::is_notable`
  (`the_test_only_caller_scan_finds_the_instance_it_was_written_for`).
- **A fix and its test in one file under `tests/`**, with no `#[cfg(test)]`
  boundary between them: `--include` reverts the whole file, the test goes with
  it, and the harness reports `NO-SUCH-TEST` — correctly, and unhelpfully. This
  is where a scan's mechanism lives, so it recurs whenever a source scan is
  fixed.
- **A bootstrap commit** — the one introducing the harness — can only show that
  deleting the instrument breaks the instrument's tests. The harness reports
  `DEMONSTRATED` and the claim is worth nothing, which is the one case here the
  tool's own output does *not* flag. State the reason instead.

**Check the exemption against the commit, not against the list.** v0.11's fix
round claimed the same-file exemption for a commit whose fix half was
`CLAUDE.md` and two workflows and whose test half was `tests/harness_honesty.rs`
— the case the *default* rule handles. A cold review ran the harness on it and
had a trailer in ninety seconds. An exemption reached for rather than checked is
the rule being routed around by the person who wrote it, which is the failure
mode the roadmap predicted for a rule with no tooling behind it, arriving even
though the tooling exists.

## The requirements that get skipped are the ones nothing fails over

Two cold reviews of v0.12's witness work both returned BLOCK, and one of them
named a pattern that held across the design doc it audited:

> **Every requirement whose artifact was Rust was met. Every requirement whose
> artifact was a kan claim or a hand-maintained list was skipped.**

**Stated as holding "without exception" across the branch, it is false**, and a
later review refuted it using the other BLOCK: `every_subject` bypassed
`claims_matching`, which is REQ-9 — a Rust requirement, unmet. The sentence
asserting the rule was written into this file in `0009e02`, one commit after
`664188c` fixed the counterexample — near enough to be the same fix round, and
not, as this paragraph used to say, the same commit. `0009e02` touches no
`src/` at all, which is checkable in four seconds and was not checked, in a
paragraph about a claim widened without re-checking. Recorded
because the failure is instructive on its own: a pattern observed in one place
was widened to a law without re-checking it where it would break, which is the
same move as a hand-written count nobody re-derives. The scoped version is
true and is worth keeping; the universal one is not.

The milestone shipped a design document promising a `witness-interview` atom, a
slash command for it, and a renderer pointing at it — and never ran `day atom
declare`. `day doctor` said 7. `day next witness-interview` errored. Ninety
seconds of running day would have found it, and nobody ran day. `telos/v1.0`
landed in neither of two passes, each pointing at the other. A citation count
was hand-written as five while the tree held seven, on the branch that existed
to fix that. `tests/plugin.rs` enumerated two command files while three shipped,
so the third's preambles went unchecked and its exhaustive `checked == 13` still
matched.

The mechanism is not carelessness. **A Rust requirement fails the build when it
is missing; a kan claim and a list entry fail nothing.** Compilation is a
verifier that runs on every save, and the other two have no verifier at all, so
attention flows to the one that pushes back. Naming it as a discipline problem
is the wrong diagnosis and produces the wrong fix.

So:

- Every one of the kan-side misses above announces itself in `day doctor`, `day
  status`, or `day next <the atom you just wrote>`, and none of the three was
  run. The imperative is a `practice` item; what it costs to skip is here.
- **A list that can be derived must be derived.** `commands/` read from the
  directory cannot fail to grow; a literal pair can. When the enumeration was
  replaced by a directory read it went red immediately — four preambles
  unchecked, and the count that was supposed to catch exactly that still said
  13, because the count and the list fail differently and only one of them was
  derived.
- **A count and a list are different guarantees.** Keep the count exact — it
  catches a parser that silently stopped matching — and derive the list, which
  catches a member that was never added. Neither substitutes for the other.
- **A supersession is only as findable as the read that surfaces it.** RQ-4's
  supersession *was* in kan — a claim on `witness-model` said so and cited RQ-4
  directly — and a reader of `kan show witness-interview` still saw only the
  superseded decision, because that view does not surface **inbound** citations.
  So the claim "it was superseded only in the design document" was itself false,
  and got recorded; the real defect is narrower and worse, because it looks
  fine from the writing side. Record the supersession on **the subject a reader
  will look at**, not only on the one you happen to be writing.

## A fix round verifies itself worse than the work it corrects

Four cold reviews across v0.12's witness work, three rounds, and **every round's
fix introduced the next round's finding**. The reviews found real, demonstrated
defects in four of four passes, so the loop works; what follows is why it
converges so slowly. Recorded as a `kan result` on `process-model`.

**The cause is one thing.** The original work gets tests. The fix gets a hand-run
demonstration in a terminal, written into a commit message where it is
unrepeatable. `every_subject`'s routing fix was verified by A/B in a shell and
shipped with **no behavioural test**; the next edit to that function — one commit
later, by its author — broke it in two new ways with the whole suite green. The
effort is inverted exactly where the code is most fragile: immediately after
being told you were wrong, under pull toward a minimal local edit whose blast
radius nobody has re-derived.

One mechanism under it is worth stating here because it is about *this file's*
tooling: **the demonstration rule has a blind spot.** Mutation asks "does any
test assert this line". Reversion asks "does the test written for this finding
fail when it returns". Neither asks whether the fix changed behaviour it was not
meant to change — which is the question both reviewers answered by building the
pre-fix binary and diffing it. `Demonstrated-by:` was entirely honest on the
commit that carried the regression.

**And the self-application gap that let it all through: not one of day's atoms
declared `done` criteria.** `day status` reported "completion cannot be checked"
for every atom, in every milestone, while day shipped and documented the
mechanism for exactly that. Nothing said "this fix round is not finished" when
the fix had no test. All eight declare them now, which is the cheapest of these
by a wide margin and the one that closes the gap the others fell through.

**The rules this produced are NOT here.** They are claims on the `practice`
subject, because that is the one day projects into injected context — so a
session inherits them at session start instead of depending on someone having
read this file. Putting process rules in `CLAUDE.md` was the same self-application
gap one level up: day exists to inject process, and its own learnings were going
somewhere day does not read. `src/practice.rs` is the mechanism, one claim per
item, and `day hook session-start` is where to check they arrive.

This section keeps the *narrative* — what happened and why — because a rule
injected without its history reads as arbitrary. The imperative form lives in
`practice`, and a rule that ends up in both will drift, which is a lesson this
file already carries twice.

## The tools in `scripts/`, already written — use them rather than reinventing them

No count in that heading, deliberately. It said "Two tools" while heading four,
which is the same stale-hand-written-number this milestone fixed in three other
places before noticing it here.

- **`scripts/mutate.py`** — one mutation, honestly reported. A green suite says
  nothing about whether a test *asserts* anything, so a claim of coverage wants a
  mutation. Use this rather than an inline loop: the inline version was
  reinvented at least three times in one session and had a different defect each
  time, all failing toward false confidence. It reports **CAUGHT / SURVIVED /
  DID-NOT-COMPILE / ANCHOR-MISSING** as distinct outcomes — a build error is not
  a survived mutation, and a stale anchor is not a passing test — and restores the
  file visibly (`copy`, not `copy2`, then `touch`, because preserving mtime hides
  the restore from cargo and corrupts the *next* run).
- **`scripts/revert-demo.py`** — one demonstration, honestly reported, per the
  rule above. Seven outcomes, never conflated, could-not-check outranking
  checked-and-clean: **DEMONSTRATED / VACUOUS / BASELINE-RED / NO-SUCH-TEST /
  DID-NOT-COMPILE / REVERT-FAILED / NOT-RESTORED**. It does **not** revert the
  test half of a change — a deleted test cannot fail — and both failure modes of
  that heuristic are loud: excluding too much reports `VACUOUS`, excluding too
  little reports `NO-SUCH-TEST`, and neither degrades to a pass.
- **`scripts/demonstration-census.py`** — accounts for every commit on the branch
  under the demonstration rule: `demonstrated`, `exempt` with a stated reason, or
  **`unaccounted`**, which is the only verdict. Exit codes are the contract — 0
  accounted, 1 unaccounted, 2 the range is unknowable, 3 the range is empty — and
  they are four rather than two because a caller that cannot tell "git failed"
  from "a commit is missing a trailer" accused a nonexistent commit and turned CI
  red. It replaced a hand-written table that was wrong in three consecutive
  review rounds; the table is what to reach for only if you want to be wrong a
  fourth time.
- **`scripts/capture-block-corpus.sh`** — regenerates the backward-compatibility
  corpus by building every released tag and driving that tag's own binary. Run it
  by hand after changing a block shape; `tests/block_corpus.rs` consumes the
  committed output on every push.

## Working practice — why the injected version says what it says

The sequence itself is a `practice` item: `/design` into `.design/<slug>.md`,
recorded with `day design record`; `/adversarial-review` against that document
afterwards; one PR per milestone off `main`; `scripts/cut-release.sh` for
releases; `--cites` takes CIDs. Both commands are day's own atoms — dogfooding
them is the point, and it is how most of this file's findings were found.

Two pieces of it have histories that do not fit in an injected item.

**Why releases go through the script.** It verifies, records the `release`
claim, and *then* tags — one step, in that order. Two consecutive releases
shipped with no claim because recording was a separate step beside the tag, and
a separate step is the one that gets dropped when the cadence compresses.
Recording *before* tagging also inverts the failure mode: a claim with no tag is
loud (`assess docs` reports "a boundary nobody cut") where a tag with no claim
was silent until somebody looked.

**Why that is not enforced in CI**, which is the more interesting half. `.kan/`
is gitignored and this repo publishes no `.claims/`, so a workflow cannot see
the log. A CI step asserting the claim exists would be green forever for the
wrong reason — better no gate than a gate that cannot fail, which is the same
judgement `.github/workflows/revert-demo.yml` records for why it runs on
`pull_request` only.

**A merge is a regular merge** (`--merge`, not squash) so a milestone's internal
commits stay visible. The demonstration census walks that history; squashing it
would leave `scripts/demonstration-census.py` nothing to account for.
