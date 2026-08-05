# Feature: An unwitnessed telos is a trigger for a human pass, not a state day tolerates

## Summary

day currently meets a telos with no declared witnesses by telling the reading
agent to declare one itself. That remedy invites a solo guess at the one thing a
solo guess is worst at, and a trivially-satisfiable witness reports met forever —
the failure `telos/v05-shipped` taught. This replaces the remedy with a new
`witness-interview` atom: an unwitnessed telos routes to a human pass that
establishes what would evidence it. It also closes the two instances day's own
log carries — `telos/v1.0` declared nowhere (day#131) and four foundational teloi
declaring no witnesses (day#86).

## Requirements

- REQ-1: A telos declaring no witnesses is reported as *actionable by a human
  pass*, never as a state to resolve by declaring a witness alone. The three
  render sites — `src/telos.rs`, `src/bridge.rs`, `src/status.rs` — stop
  emitting `day telos declare <slug> "..." --witness <type>` as the remedy.
- REQ-2: The unwitnessed remedy has exactly one renderer. Per CLAUDE.md's
  standing rule, a guarantee about what day prints belongs in the mechanism and
  not at three call sites that each hand-rolled the same prose.
- REQ-3: A source scan fails the build when a module outside that renderer emits
  the unwitnessed remedy itself, with a stated escape hatch — the shape
  `a_failed_kan_read_is_never_swallowed` already uses in `tests/plugin.rs`.
- REQ-4: A `witness-interview` atom is declared in day's own vocabulary,
  consuming an unwitnessed telos and producing a revised telos declaration. It
  composes so that `day next` names it from where an unwitnessed telos is met.
- REQ-5: The interview is carried by a slash command, not a bare CLI verb.
  `commands/design.md` Phase 6 records day's own measurement: every capability
  with a slash command was used every time, every bare verb once by its author.
- REQ-6: `telos/v1.0` exists as a declared standing invariant with witnesses,
  and the five files citing it reason about a telos the log can check.
- REQ-7: A separate milestone telos carries the v1.0 *bar*, so the standing
  invariant is not conflated with the release that demonstrates it.
- REQ-8: Each of `telos/legible-process`, `telos/composable-process`,
  `telos/affordance-not-enforcement` and `telos/no-store-of-its-own` declares
  witnesses, so each can be a bridge target.
- REQ-9: No witness declared here is satisfiable by an artifact that would exist
  whether or not the telos held. A witness that cannot fail is the defect day#86
  names, and it is worse than none.
- REQ-10: The citation count for `telos/v1.0` is derived from the tree rather
  than hand-written. day#131's title says four, `docs/ROADMAP.md` says five, and
  the tree says five files — day#133's class inside day#131.
- REQ-11: The default `day assess telos` does not report nothing material for a
  telos whose witnesses are all `command` probes. Three of day's four
  foundational teloi are in that state under Q2's resolution, and a telos that
  is only checkable under a flag nobody passes is checkable in the same sense an
  unwitnessed one is.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) Exactly one function in `src/` produces the
  unwitnessed-telos remedy text; `src/bridge.rs` and `src/status.rs` call it
  rather than formatting their own.
- [ ] AC-2: (REQ-1) `day assess telos legible-process` names the interview pass
  and does not print a `--witness <type>` declare command.
- [ ] AC-3: (REQ-1) `day bridge check` against an unwitnessed target names the
  interview pass and does not print a `--witness <type>` declare command.
- [ ] AC-4: (REQ-3) A test in `tests/plugin.rs` greps `src/` and fails when a
  module other than the renderer emits the remedy, and passes when the emitting
  line carries the declared escape-hatch marker.
- [ ] AC-5: (REQ-3) The scan is demonstrated to fire: reintroducing the
  hand-rolled remedy in `src/bridge.rs` turns the scan red.
- [ ] AC-6: (REQ-4) `day doctor` reports eight declared atoms with
  `composition: ok`, including `atom/witness-interview`.
- [ ] AC-7: (REQ-4) `day next witness-interview` reports what follows it, and
  the atom's `--in`/`--out` types resolve through `schema/witness`.
- [ ] AC-8: (REQ-5) `commands/witness-interview.md` exists and is listed
  alongside `commands/design.md` and `commands/adversarial-review.md`.
- [ ] AC-9: (REQ-6) `kan show telos/v1.0` returns at least one live claim
  carrying a `day-telos` block with a non-empty witness list.
- [ ] AC-10: (REQ-6) `day assess telos v1.0` renders material evidence rather
  than the unwitnessed path.
- [ ] AC-11: (REQ-7) A milestone telos for the v1.0 bar exists on its own
  subject, and `day assess telos` distinguishes it from the standing invariant.
- [ ] AC-12: (REQ-8) `day assess telos <slug>` renders material evidence for
  each of the four foundational teloi.
- [ ] AC-13: (REQ-9) For each witness declared here, a named condition is
  recorded under which it would report absent, and at least one is demonstrated
  to report absent against a fixture that violates its telos.
- [ ] AC-14: (REQ-10) The count of files citing `telos/v1.0` is produced by a
  command, and both `docs/ROADMAP.md` and day#131 agree with what it prints.
- [ ] AC-15: (REQ-1, REQ-4) A fallbacks-style test asserts its premise — that
  the fixture telos really declares no witnesses — before asserting the
  interview remedy is what gets rendered.
- [ ] AC-16: (REQ-11) `day assess telos` with no `--run` reports at least one
  material verdict for each of the four foundational teloi. This is satisfied by
  the disjunctive group the companion pass adds, so it is the one criterion here
  that lands only once that pass does — stated as a dependency rather than left
  to be discovered.

## Architecture

The unwitnessed state is computed identically in two places today:
`src/telos.rs` sets `checkable: !witnesses.is_empty()` on its `Report`, and
`src/bridge.rs` does the same on its own. Each then formats its own prose,
and `src/status.rs` carries a third variant for the adjacent case where
`schema/witness` declares no probes at all. The three strings differ in wording
and agree in advice, which is how a fourth would arrive unnoticed.

REQ-2 collapses the advice into one renderer. This is deliberately the same move
CLAUDE.md records for `KanClient`: the reason `unaccounted_subjects()` had to
move was that a check at a call site looks complete because the author's test
drives the call site they were thinking about. Three call sites that already
disagree about wording is that condition, observed rather than predicted.

`src/status.rs`'s case is *adjacent, not identical* — no probes declared for the
project is a different fact from no witnesses declared on a telos — so it was
put as a question rather than folded in by assumption, and RQ-1 keeps it out.

The atom is data, not a verb. `day atom declare` already takes `--in`, `--out`,
`--next`, `--revisits`, `--done` and `--note`, so `witness-interview` is
declarable with the shipped CLI and needs no new surface — which keeps
CLAUDE.md's "a new verb needs a design doc" from being triggered at all. The
command that drives it is a plugin command beside `commands/design.md`, and the
interview it runs is modelled on that file's Phase 1: questions grounded in what
was found, then wait.

Ordering note for implementation, not a requirement: the four foundational
witnesses and `telos/v1.0`'s are established by *running* the interview by hand
with the author first, and the atom is declared from what that pass actually
needed. Declaring the atom first and then inventing an interview to fit it is
the failure mode `.design/position-honesty.md` names — writing the expectation
table from reasoning rather than from a measurement run.

## Resolved Questions

Stated as `RQ-` bullets before the narrative, and that form is the point rather
than a style choice. This document's first recording produced an observe, a plan
and **zero decides**: `design::check_against_record` reads resolutions from
*bullets* carrying `RQ-` ids, so the `### Qn:` headings below yielded nothing and
`day design check` said nothing about it. `.design/verification-that-can-fail.md`
has the same shape, so v0.11's design resolutions never reached the log as
decisions either. The finding is recorded on this subject; the bullets are the
fix applied to the document that found it.

- RQ-1: `src/status.rs`'s no-probes-declared message **stays separate** from the
  unwitnessed-telos renderer. It reports a project-level fact — no readable probe
  in `schema/witness` — which is upstream of any telos, and day#108 already
  rejected routing that reader to a remedy that does not remedy it. REQ-3's scan
  is therefore keyed on the presence of the literal `--witness <type>`, a
  positive signal that `status.rs` does not contain.
- RQ-2: The four foundational teloi take the witnesses in the table below —
  `legible-process` conjunctively from the three types already declared, and the
  other three from `command` probes over day's own guard tests, confirming
  day#86's prediction that day's foundational properties are evidenced by its own
  tests rather than by artifacts.
- RQ-3: The interview atom declares **both edges** — `--next` from `design` for
  the prospective case and `--revisits` from `assess-telos` for the retrospective
  one. Both are real occasions, and declaring only one leaves the other with no
  route to the atom.
- RQ-4: The unrun-command problem (REQ-11) is served by the **disjunction** the
  companion pass adds, not by new machinery here: a group whose members are the
  command probe and a recorded assessment is satisfied either by running the
  check or by someone having recorded that they did. Running guard probes at
  assessment time was rejected — `--run` is one of the four rules bounding
  `src/probe.rs`.
- RQ-5: A **disjunctive witness set is not expressible**, found by running this
  interview, and it is a separate design pass rather than a decision to take
  here. `.design/witness-model.md` carries it with eight further gaps.

### Q1: `src/status.rs`'s no-probes-declared message stays separate

The two telos sites and `status.rs` report **different facts at different
layers**. `schema/witness` maps a witness *type* to a probe, so
`status.rs`'s trigger — `schema.probes.is_empty()` at `src/status.rs`, the
project declares no readable probe at all — is upstream of any telos. A telos can
be fully witnessed while that fires, and `src/position.rs` already states the
independence: *"the two are independent reasons day knows less than the report
might imply, and letting one swallow the other is how a could-not-check goes
quiet."*

Decisive evidence: `status.rs` has already applied this milestone's own rule
once, and recorded it. Its comment rejects day#108's suggestion to route the
reader to `day init` because that verb *"records a `schema/design-doc` starter
and no witnesses at all — a remedy that does not remedy this."* Routing it to
the telos interview would reintroduce exactly that, since no telos is in
question. `render_short` also notes the case is *"a setup step, not a
diagnostic"* — the fresh-repo state, which is the state a pack arrives into and
must stay a setup step.

So REQ-3's scan is keyed on the **presence** of the literal `--witness <type>`
in an emitted string anywhere in `src/` outside the renderer. That is a positive
signal, per CLAUDE.md's rule against keying a classifier on a phrase's absence,
and the two cases separate cleanly on text that already differs: `status.rs`
emits `kan observe "..." --subject schema/witness` and contains no such literal.

### Q2: The four foundational teloi take these witnesses

Established by running the interview by hand rather than by reasoning, which is
the ordering the Architecture section requires.

| telos | witness type(s) | probe | reports absent when |
|---|---|---|---|
| `legible-process` | `design-doc`, `verdict`, `assessment` | already declared in `schema/witness` | a milestone ships without one of the three, in this cycle |
| `composable-process` | `composing-vocabulary` | `command: day doctor` | the declared atoms stop composing |
| `affordance-not-enforcement` | `no-blocking-hook` | `command: cargo test --test plugin ac5_shipped_hooks_declare_no_blocking_decisions` | a shipped hook gains a blocking decision |
| `no-store-of-its-own` | `single-cache-writer` | `command: cargo test --test plugin ac9_the_render_cache_is_touched_in_exactly_one_module` | a second module touches `.day/` |

`legible-process` needs no new probe kinds and its conjunction is *correct*: the
record reconstructs the process only if design, verdict and assessment are all
present. Cycle-scoping is what keeps it falsifiable — `.design/*.md` always
exists in this repo, but a design doc created *since the boundary* does not.

The other three are `command` probes, confirming day#86's observation that day's
foundational properties are evidenced by its own tests rather than by artifacts.
The consequence is named rather than buried: without `--run` they report
`NOT RUN`, which `src/probe.rs` correctly treats as absence of evidence and not
failure, so the default `day assess telos` shows nothing material for three of
day's four foundational teloi. That is honest and it is also nearly useless, and
it is carried into this milestone's scope as REQ-11.

### Q3: The interview atom declares both edges

`--next` from `design` for the prospective case — `commands/design.md` Phase 6
declares a telos, and an unwitnessed one should be interviewed before the work
proceeds — and `--revisits` from `assess-telos` for the retrospective case, where
an assessment meets a telos it cannot check. Both are real occasions and
declaring only one leaves the other with no route to the atom, which is the gap
that motivated it.

### Q5: A disjunctive witness set is not expressible, and that is a separate question

Found by running the interview, and the clearest argument this pass earns its
place: reasoning about witnesses did not surface it.

The author named three independently valid evidences for `telos/v1.0` — a claim
signed by a foreign DID, a third-party contribution on this repo, and an adoption
record — which "can separately or jointly hold." day treats a witness list as a
**conjunction in both consumers**: `src/bridge.rs` computes `uncovered` as
declared witnesses the plan does not produce and `is_reachable` requires it
empty, and `src/probe.rs`'s `is_failure` plus `src/telos.rs`'s `is_clean` mean
one unsatisfied witness makes the whole report unclean. The render lists
witnesses independently; the verdict ands them.

So there is no way to say *any one of these suffices*. Filed separately rather
than decided here, because it is a change to day's model that a pack plausibly
needs — a telos transported into a foreign repo meets exactly this shape — and
settling it as a side effect of declaring one telos is how a model change
arrives unreviewed.

## Out of Scope

- **What a witness can express.** Q5's finding opens a separate design pass on
  the witness model — disjunctive sets, day#107's correspondence gap, and
  authorship scoping. `telos/v1.0`'s own declaration (REQ-6, REQ-7) lands there
  rather than here, because its witness set is the case the current model cannot
  hold. This doc is the mechanism day runs when a telos is unwitnessed; that one
  is what "witnessed" is allowed to mean.
- The pack mechanism itself (day#73). This establishes the vocabulary a dev pack
  will later carry; it does not build the transport.
- day#120's absent-versus-unreadable distinction for schema reads. Same family
  and next in sequence, but it is a different read path and a different fix.
- Retracting or rewriting the existing prose in the five files citing
  `telos/v1.0`. Once the telos is declared they become correct as written.
- Any change to `schema/witness`'s block format or to probe kinds.
