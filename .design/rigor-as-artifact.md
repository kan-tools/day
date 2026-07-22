# Feature: Rigor as artifact — done criteria, inferred position, and a human surface

## Summary

day describes a process well and does little to make one happen. This gives
atoms the evidence story teloi already have — a `done` field of witness types,
checked through the probes v0.5 shipped — infers which atom the work is
currently in from artifacts alone, and puts that where the **human** can see
it, which is somewhere day has never put anything.

It leads with checkable artifacts rather than injected rules, because this
project has direct evidence that injected rules are the weaker lever.

## Motivation

**The rule that did not hold.** day#30 shipped injected guidance saying *"do
not chain commit and push; chaining removes the last checkpoint where a bad
commit is still cheap."* About an hour later the agent that wrote it chained
an inspection to an irreversible `rm -rf` and destroyed kan's log, with that
rule in its context. A standing rule did not survive contact with the moment
it applied to.

**What did hold, every time:** mutation testing (each test mutated and watched
to fail was real; each one where that step was skipped turned out hollow —
the kan stub, the open-question counter, a git stub returning all tags
regardless of pattern), dogfooding the real log, and guardrail tests that grep
the source. None are rules. All are **artifacts that fail loudly**.

**The structural gap.** Teloi declare witnesses and resolve them through
probes — an evidence story, shipped in v0.5. `atoms::Interface` has `in`,
`out`, `next`: types only, and **nothing that says how you know the atom is
done**. `generative-build` emits a `code-change`; what makes one finished is
currently vibes.

**And nobody has ever seen any of it.** From the person who has used day
across several repos: *the atom sequencing has never visibly played out in
any repo I've used day in.* Not unclear — never observed. Hook stdout goes to
the model and is never displayed; everything else needs a verb run. day's
value proposition is legible process, and the process is invisible to the
person it is meant to be legible to.

## Requirements

- REQ-1: `atoms::Interface` gains an optional `done` field: witness type names
  resolved through the same `schema/witness` probes teloi use. Absent means
  "no completion criteria declared", which is reported, never treated as met.
- REQ-2: `done` is additive. Every `day-atom` block written before this parses
  and composes identically, and `day doctor` is unchanged by its absence.
- REQ-3: day infers the **current atom** from artifacts alone: an atom is a
  candidate when its declared inputs are materially present and its outputs
  are not. Nothing is tracked and nothing is recorded — inference reads the
  same probes an assessment does.
- REQ-4: Ambiguity is reported, not resolved. When several atoms are
  candidates, all are named. Guessing one would be a claim day cannot support.
- REQ-5: Position inference runs **only** `path` and `tag` probes, never
  `command` probes. Inference happens on every session start and must not
  execute project-declared commands as a side effect; a type whose probe is a
  command is reported as unknowable rather than silently treated as absent.
- REQ-6: **Off-sequence detection.** If an atom's outputs are present while an
  upstream atom's outputs are not, the work skipped a step. Reported, with
  both atoms named.
- REQ-7: `day status` reports, for a human: the current atom or candidates,
  which inputs are satisfied, which `done` criteria are met and unmet, what
  the graph says comes next, and any off-sequence finding.
- REQ-8: A **status line** renders the same state in one to three lines, and
  reads only the render cache — never kan, never git. Claude Code cancels an
  in-flight status line when a new update arrives, so a line that shells out
  does not lag, it shows nothing.
- REQ-9: The render cache is written by `day hook session-start`, which
  already reads kan and has time. It is gitignored, contains nothing not
  derivable from kan and git, its absence is never an error, and holds
  **only rendered display state** — no input to any decision lives in it.
  It solves latency and nothing else.
- REQ-10: The **transition baseline is claims**, not the cache. A transition
  is the current inferred position differing from the position implied by the
  **last recorded assessment** on the relevant subject. day *reads* those
  claims and never writes them.
- REQ-14: Because the baseline is claims, it inherits their semantics:
  retracting an assessment removes it as a baseline, a contradicting claim is
  visible as a contradiction rather than an overwrite, and whose assessments
  count is answered by the same locally-signed projection rule `practice`
  uses.
- REQ-13: `day assess atom <slug>` reports an atom's `done` criteria against
  the project's probes and exits non-zero when a declared criterion is unmet,
  so CI can gate on it. It runs command probes only under `--run`, matching
  `assess telos`.
- REQ-11: Everything here is advisory. `day status` and the status line report;
  no hook emits a blocking decision; nothing gates an agent's action.
  `tests/plugin.rs` continues to enforce this.
- REQ-12: `docs/CONVENTIONS.md` documents the `done` field, the inference
  rule, and the cache's guarantees; `CLAUDE.md` records that day now writes a
  derived cache and why that is not a store.

## Acceptance Criteria

- [ ] AC-1: An atom declaring `done` with two witness types reports each as
      met or unmet against the project's probes. (REQ-1)
- [ ] AC-2: Every `day-atom` block in this repo's log parses unchanged and
      `day doctor` output is byte-identical before and after. (REQ-2)
- [ ] AC-3: Given a design doc present and no code change, inference names
      `generative-build`; given both present, it does not. (REQ-3)
- [ ] AC-4: With two atoms equally consistent with the evidence, both are
      named and neither is chosen. (REQ-4)
- [ ] AC-5: Position inference executes no command probe — asserted with a
      stub probe whose sentinel file must not exist afterwards. (REQ-5)
- [ ] AC-6: With a `code-change` present and no `design-doc`, the output
      names the skipped upstream atom. (REQ-6)
- [ ] AC-7: `day status` names the current atom, its satisfied and missing
      inputs, its met and unmet `done` criteria, and what follows. (REQ-7)
- [ ] AC-8: The status-line command produces output with the kan binary
      pointed at a path that does not exist, proving it reads only the cache.
      (REQ-8)
- [ ] AC-9: `day hook session-start` writes the cache; deleting it and
      re-running regenerates it; with the cache absent, `day status` still
      works and no command errors. A source scan asserts the cache is read in
      exactly one module and only for rendering. (REQ-9)
- [ ] AC-10: With the last recorded assessment implying a different position
      from the inferred one, the transition is named; with them agreeing,
      nothing is said. With no assessment ever recorded, no transition is
      claimed — absence of a baseline is not a change. (REQ-10)
- [ ] AC-14: Retracting the assessment that formed the baseline changes what
      day reports, with no code path and no file touched. (REQ-14)
- [ ] AC-13: `day assess atom` exits non-zero with a declared criterion
      unmet and zero when all are met or none are declared, and executes no
      command probe without `--run`. (REQ-13)
- [ ] AC-11: The shipped hook config declares no blocking construct, and
      `day status` exits zero when it merely has findings to report. (REQ-11)
- [ ] AC-12: `docs/CONVENTIONS.md` contains the `done` key and the cache path,
      checked against the code's constants; `CLAUDE.md` states the cache is
      derived and never authoritative. (REQ-12)

## Architecture

**`src/atoms.rs`** gains `done: Vec<String>` on `Interface`, with
`#[serde(default, skip_serializing_if = "Vec::is_empty")]` so blocks written
before this serialize byte-identically — the property AC-2 asserts and the
same mechanism `Witnesses::scope` already uses.

**`src/position.rs` (new)** holds inference and off-sequence detection. It
takes the atom set, the witness schema, and a `Git`, and answers which atoms
are candidates. It reuses `probe::evaluate` with `Authorization::Report`,
which is what makes REQ-5 hold by construction rather than by discipline:
`Report` is the authorization that cannot execute anything.

**`src/status.rs` (new)** assembles the human report from position, `done`
criteria, and the atom graph, and renders both the long form (`day status`)
and the one-to-three-line form (the status line). Two renderings of one
computation, so they cannot drift.

**`src/cache.rs` (new)** writes and reads the render cache under `.day/`. It
is the only module that touches it. The source scan in `tests/plugin.rs`
asserts nothing else does — the guardrail that keeps *"display only"* from
decaying into *"and also decides things"*.

**`src/hooks.rs`** writes the cache at the end of `session_start`, after it
has already done the kan reads. **`src/cli/mod.rs`** gains `status` and a
hidden `status-line` leaf. The plugin gains a `statusLine` entry and a
`UserPromptSubmit` hook that emits only on transition.

**Nothing here writes a claim.** Position is inferred and displayed; it is
never recorded, because recording it would make day a task tracker, which
`docs/CONVENTIONS.md` refuses on purpose.

## Resolved Questions

- **Checkable artifacts before injected rules.** The evidence is day#30: a
  standing rule in the agent's own context did not survive the moment it
  applied to. What worked was mutation testing, dogfooding, and guardrail
  tests — artifacts that fail loudly. Injection is demoted to a pointer at a
  check.
- **`done` reuses `schema/witness` rather than inventing a second vocabulary.**
  Atoms and teloi both need "what would evidence this", and one probe map
  serving both keeps a project's evidence story in one place.
- **Position is inferred, never tracked.** Inference reads artifacts, so day
  still stores nothing durable and the task-tracker line stays uncrossed.
  Ambiguity is reported rather than resolved.
- **Inference never runs command probes.** It happens every session start, and
  executing project-declared commands as a side effect of *starting a session*
  would be a far larger widening than `--run` ever was.
- **The cache is written by the session-start hook, not the status line.**
  Claude Code cancels an in-flight status line when a new update arrives, so
  the expensive reads have to happen somewhere with time. The hook already
  reads kan.
- **The transition baseline is claims, not the cache** — revising an earlier
  decision that merged two problems. "The status line must render before it
  is cancelled" and "a transition needs a baseline" are separable, and claims
  solve exactly one: they are the right baseline and no help at all for
  latency, since reading a claim *is* the slow read the line must avoid.
- **This is not the task-tracking `docs/CONVENTIONS.md` refuses.** That
  refusal says whether a step happened *"is already derivable from claims and
  artifacts existing"* — deriving position from claims is what that sentence
  points at. What it refuses is day-owned tracking state.
- **day reads those claims and never writes them.** `src/telos.rs` already
  holds that recording an assessment is a separate act, and that conflating
  *"I checked"* with *"I recorded that I checked"* would let the tool
  manufacture its own evidence. Auto-writing a baseline would be exactly that.
- **So a transition means something better:** not *"position differs from the
  last time a cache file was written"* but *"position has changed since you
  last recorded an assessment."* Self-reinforcing, since the baseline exists
  only if assessments are actually recorded — and `assess telos` already
  prints the runnable `kan result` that writes one.
- **The cache is display-only, and that is enforced by a source scan** rather
  than by intent. If day ever reads it to decide something rather than to
  display something, the line has been crossed — so a test watches the line.
- **Atom completion is checkable as `day assess atom <slug>`**, beside
  `assess docs` and `assess telos`, exiting non-zero on unmet criteria. The
  milestone's thesis is enforcement at the *artifact* level — day makes the
  state of the work measurable and CI or a human gates on it — and a display
  command that always exits zero gives nothing to gate on. It costs one more
  verb in a CLI whose smallness is advertised, which is the real price and is
  paid knowingly. Rejected: `day status --check`, because a flag that
  silently changes exit semantics is the kind of thing CI configs copy
  without noticing.
- **Transition printing is not designed until an empirical check settles the
  channel.** The harness verification established what is documented; it also
  established that `MessageDisplay`'s `displayContent` and the `systemMessage`
  field are *not* documented in enough detail to build on. Designing on that
  gap is exactly what gate one existed to prevent, and exactly what the v0.2
  session-end hook got wrong. So the check comes first and the design follows
  it. Until then transitions surface in `day status` and the status line, and
  inject to the model on change — REQ-10 as written.

## Out of Scope

- **Drift detection.** Still v0.4's leftover, still more model.
- **Plugins.** v0.8, and the bundle format cannot stabilise until this
  milestone's vocabulary does.
- **Any blocking construct.** A `Stop` hook or pre-commit gate would be the
  strongest standardising force and would recreate the crosslink friction day
  was split out to avoid. Enforcement stays at the artifact level.
- **Recording position as a claim.** That is a task tracker, and
  `docs/CONVENTIONS.md` refuses it deliberately.
