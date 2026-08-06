# Feature: What a witness is allowed to express

## Summary

day's witness model was built one probe kind at a time, each addition answering
the case in front of it. Nine gaps have accumulated, every one found by running
day rather than by reasoning about it, and seven are already filed. This is the
re-grounding: what a witness set can *say* (disjunction, negation, authorship),
what it can *compare* (correspondence, scope precision), and what day *tells you*
about one (declare-time silence, unrun commands, a starter shape, a hardcoded
prefix). It is scoped to all nine deliberately, because the gaps interact —
`telos/v1.0` alone needs three of them.

## Requirements

### What a witness set can say

- REQ-1: A telos can declare that **any one of several witness types** suffices.
  Today `src/bridge.rs` treats an uncovered witness as unreachable and
  `src/telos.rs`'s `is_clean` fails on any unsatisfied one, so a witness list is
  a conjunction in both consumers and a disjunction is not expressible.
- REQ-2: The disjunctive form is backward compatible: every `day-telos` block
  already written keeps its current meaning, and a plain witness list stays a
  conjunction.
- REQ-3: `day bridge check` reports a disjunctive group as covered when the plan
  produces **at least one** member, and names which member it counted.
- REQ-4: A probe can be **negated**, so a telos satisfied by the *absence* of an
  artifact is expressible. Every probe kind in `src/probe.rs` is an existence
  check today, and day#125's guest-tree telos — "our tooling leaves no trace on
  repositories we are guests in" — is unprobeable in principle as a result.
- REQ-5: A negated probe reports **`VACUOUS`** unless a **companion positive
  witness** resolves, establishing that the forbidden thing could have occurred.
  Absence on its own establishes nothing, and `src/probe.rs`'s existing
  precedence — only a probe that ran and found nothing counts against the telos —
  must extend to vacuity without letting a vacuous pass read as evidence.
- REQ-6: A claim probe can require evidence **not authored by a given identity**,
  so a telos cannot be satisfied by the person who declared it. This is the abuse
  case: without it, an adoption witness is satisfiable by its own author.

### What a witness can compare

- REQ-7: A record witness can require **correspondence** — that the record refers
  to the resolved material instance, rather than merely existing. `ClaimShape`
  carries only constants today, which is why day#107's release case still needs
  `docs::reconcile_boundary` and its `text.contains(tag)`.
- REQ-8: A scope can express "the final release but not its pre-releases".
  day#85 shows the two available readings are "satisfied by a beta" and
  "collapsed onto one artifact", and `docs/CONVENTIONS.md` argues against the
  second while the first is the defect.
- REQ-9: No addition here reintroduces a quiet check. A comparison that cannot be
  answered for structural reasons is reported as unanswerable, per day#107's
  stated constraint and the rule `design check` got in day#105.

### What day tells you about a witness

- REQ-10: `day telos declare --witness X` reports when `X` has no probe in
  `schema/witness`. day#125 declared four teloi with witnesses, felt done, and
  found out much later at `day status` that none were checkable.
- REQ-11: The failure that reports a missing witness schema prints a **starter
  `day-witness` block**, the way `day init` prints a starter `day-blocks`. Today
  it points at `docs/CONVENTIONS.md`, which is not installed locally, so the path
  from "day told me what is missing" to "I know the shape" runs through GitHub.
- REQ-12: A telos whose witnesses are all `command` probes is not silent by
  default. `src/probe.rs` correctly treats an unauthorized command as absence of
  evidence rather than failure, and the consequence is that three of day's four
  foundational teloi show nothing material unless someone passes `--run`. Per
  RQ-11 this is answered by *naming the invocation*, not by making the telos
  checkable some other way: the material section states the exact `--run` command
  that would resolve each unrun witness. Legibility, not a second route to green.
- REQ-19: No witness type is satisfied by the *existence* of a verdict. A probe
  whose evidence is "someone recorded that this was met" consumes a flattened
  assessment and makes the flattening durable, which is RQ-11's rule stated as a
  requirement on `schema/witness` rather than as a principle.
- REQ-20: A witness can express a **universal over co-located claims** —
  wherever one shape holds on a subject, others must hold on that same subject.
  Every existing probe asks "does one exist", which over an append-only log can
  only start answering yes and never stop, so a witness built from them reports
  its telos met forever (day#138). Co-location is what day#86 asked for and three
  independent witnesses discard; the universal is what stops the result being
  monotone.
- REQ-21: A universal with nothing in scope reports **`VACUOUS`**, never
  satisfied. A universal over an empty set is true and establishes nothing, which
  is REQ-9's quiet check arriving through the logic rather than through the data.
- REQ-22: `day telos declare` reports a witness that cannot distinguish — one
  already satisfied, or one whose probe can never stop matching. Falsifiability
  was left to judgement and was got wrong twice in one session; it is mechanically
  checkable and belongs in the tool.
- REQ-13: The `verdict` witness's anchor is **declared, not compiled in**.
  `day review record` hardcodes the `adversarial review of` prefix, so a second
  review atom's verdict is unrecordable and is mislabelled into the first atom's
  evidence (day#106).
- REQ-14: `docs/CONVENTIONS.md` documents every form added here, since it is
  authoritative for what day reads and writes.

### Holding the line

- REQ-15: No addition here creates a fourth command-spawn site, relaxes any of
  the four rules bounding `src/probe.rs`, or requires a shell. day#125's own
  workaround for absence — `test -z "$(...)"` — is correctly unexpressible, and
  negation exists so that it stays that way.
- REQ-16: Every new form is exercised by a fallbacks-style test that asserts its
  premise, per day#91: the fixture must be shown to be in the degraded state
  before the assertion about it means anything.
- REQ-17: A witness form that day can parse but not evaluate is reported as
  unreadable rather than skipped, extending `WitnessSchema::unreadable`'s
  existing behaviour to every form added here.
- REQ-18: A negated `command` probe must declare which non-zero exit means "ran
  and found nothing"; any other non-zero code is `Error`, never satisfied.
  `run_command` collapses every non-zero exit into `Unsatisfied`, which is
  conservative un-negated and a **false clean** negated — `grep -r SECRET srcc/`
  exits 2 for a bad path and would report the secret absent. This is the one
  place negation-as-a-uniform-modifier does not hold, and it lands in the feature
  whose whole point is a vacuity guard.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A `day-telos` block declaring a disjunctive group
  parses, and a block written before this change parses to the same witness set
  and the same verdict as it does today.
- [ ] AC-2: (REQ-1) `day assess telos` on a telos with a disjunctive group
  reports the telos clean when exactly one member is satisfied.
- [ ] AC-3: (REQ-3) `day bridge check` reports a disjunctive group as covered
  when the plan produces one member, and its output names that member.
- [ ] AC-4: (REQ-3) `day bridge check` reports the group uncovered when the plan
  produces none of its members, and names the whole group rather than one entry.
- [ ] AC-5: (REQ-4) A negated `path` probe reports satisfied when no tracked file
  matches, and unsatisfied when one does.
- [ ] AC-6: (REQ-5) A negated probe with no companion positive witness, or whose
  companion does not resolve, reports `VACUOUS`, and `VACUOUS` does not make the
  telos report clean.
- [ ] AC-7: (REQ-5) `VACUOUS` is distinct from satisfied **in the render**, and
  in the exit code behaves like the other could-not-check verdicts — `ERROR`,
  `NOT RUN`, `TIMEOUT` — rather than like a probe that ran and found nothing.

  *Amended after a cold review read the original wording as requiring a non-zero
  exit.* That would make `VACUOUS` stricter than `ERROR`, which is incoherent:
  an error is the stronger could-not-check, and `Verdict::is_failure` counts only
  `Unsatisfied` for all of them. The original text was wrong, and the test
  written against it asserted a bare `Some(0)` — right answer, no stated reason,
  so it read as pinning the side the AC forbade. Whether could-not-check should
  affect the exit code at all is a real question about *every* such verdict and
  is filed separately, not settled by giving one of them special treatment.
- [ ] AC-24: (REQ-5) All three states of day#125's guest-tree case are driven by
  fixtures and report distinctly: no companion resolving is `VACUOUS`, companion
  present with the forbidden path tracked is unsatisfied, and companion present
  with it absent is satisfied.
- [ ] AC-25: (REQ-18) A negated `command` probe declaring an expected exit code
  reports satisfied on that code and `Error` on any other non-zero code; a
  fixture drives the bad-path case that would otherwise read as a false clean.
- [ ] AC-8: (REQ-6) A claim probe excluding an author DID reports unsatisfied
  when the only matching claims carry that DID, and satisfied when one does not.
- [ ] AC-9: (REQ-6) The exclusion resolves the active identity without a second
  substrate — through the same read path `day hook session-start` already uses.
- [ ] AC-10: (REQ-7) A record witness requiring correspondence reports satisfied
  only when the record refers to the resolved material instance, and a fixture
  with a record referring to a *different* instance reports unsatisfied.
- [ ] AC-11: (REQ-7, REQ-9) A correspondence that cannot be evaluated because the
  material witness did not resolve reports unanswerable, never silence.
- [ ] AC-12: (REQ-8) A scope excluding pre-releases reports a telos unsatisfied
  when only a beta tag exists and satisfied when the final tag does.
- [ ] AC-13: (REQ-8) The excluding scope still names a class rather than one
  artifact: a fixture with two qualifying final tags satisfies it.
- [ ] AC-14: (REQ-10) `day telos declare --witness X` where `X` has no entry in
  `schema/witness` emits a note naming `X`, and the declaration still succeeds.
- [ ] AC-15: (REQ-11) The missing-witness-schema failure prints a starter block
  that can be pasted into a `kan observe` without editing, and a test asserts the
  printed block parses as a `day-witness` map.
- [ ] AC-16: (REQ-12) `day assess telos` with no `--run` prints the exact `--run`
  invocation for each unrun command witness, and a telos whose witnesses are all
  command probes never renders an empty material section.
- [ ] AC-26: (REQ-19) A witness type whose probe matches a claim asserting a
  telos was met is refused or reported, and a test drives the `{kind: Result}`
  shape that RQ-10 would have declared.
- [ ] AC-27: (REQ-19, RQ-11) `day assess telos` states that its exit code is a
  reading derived from the witness state rather than a stored verdict, so a
  reader cannot take a clean exit as a durable property of the telos.
- [ ] AC-28: (REQ-20) An `every` probe reports unsatisfied when a subject
  matching the anchor lacks a required shape, satisfied when all of them carry
  it, and **names the incomplete subjects** rather than counting them.
- [ ] AC-29: (REQ-20) A required claim on a *different* subject does not complete
  an anchored one — the co-location is asserted directly, since a predicate that
  never rejects is indistinguishable from one never applied.
- [ ] AC-30: (REQ-21) An `every` probe whose anchor matches no subject reports
  `VACUOUS`, and `VACUOUS` renders distinctly from both met and unmet.
- [ ] AC-31: (REQ-22) `day telos declare` names a witness that is already
  satisfied, one whose probe can never stop matching, and one with no probe at
  all — and declares it anyway.
- [ ] AC-32: (REQ-20, REQ-9) `telos/legible-process` is declared with a witness
  that reports absent against day's own log at declaration time, closing day#138
  by being falsifiable rather than by being re-argued.
- [ ] AC-17: (REQ-13) The review anchor is read from a declaration; a second
  review atom declaring its own anchor records a verdict that resolves to that
  atom and not to the first.
- [ ] AC-18: (REQ-13) With no anchor declared, the shipped behaviour is
  unchanged, so existing logs keep resolving.
- [ ] AC-19: (REQ-14) `tests/plugin.rs` asserts `docs/CONVENTIONS.md` names every
  form added here, extending the content check it already performs.
- [ ] AC-20: (REQ-15) The command-spawn-site test still finds exactly three
  sites, and a test asserts no negated probe reaches `run_command`.
- [ ] AC-21: (REQ-16) Each new form has a test in `tests/fallbacks.rs` that
  asserts its premise before asserting its behaviour.
- [ ] AC-22: (REQ-17) A witness form this build cannot evaluate is reported with
  the reason, and a test drives a block using a form from a later version.
- [ ] AC-23: (REQ-9) No form added here can produce a witness that is satisfied
  without a probe having run and found something.

## Architecture

The witness model lives in three files and the split is already clean:
`src/telos.rs` holds `WitnessSchema`, `PairedWitness` and the `day-witness`
block; `src/probe.rs` holds `Probe`, `ClaimShape`, `Verdict` and every
evaluation; `src/bridge.rs` holds the reachability fold. Nothing here moves that
boundary — the additions are to the vocabulary each already owns.

**Disjunction** is a change to the witness *list*, not to any probe. The natural
encoding keeps the existing array and lets an element be either a type or a list
of types, so the list reads as a conjunction of groups and a bare string is a
one-member group. That is backward compatible by construction, which is REQ-2,
and it is why the nested form is preferred over a parallel `witnesses_any` key
that would leave two lists to keep in agreement. `src/bridge.rs`'s `uncovered`
becomes "groups with no member in `available`" and `src/telos.rs`'s `is_clean`
becomes "no group with every member unsatisfied".

**Negation** is a modifier on a probe rather than a new kind, per day#125, and
the vacuity guard is what makes that safe. A negated probe is satisfied by
everything that does not exist, which is the inverse of the cannot-fail problem
day#86 names — so it needs a companion positive witness before its satisfaction
means anything, and reports `VACUOUS` without one.

The rule this *replaces* is worth recording, because it was the intuitive one and
it is wrong. Deciding vacuity from git history — has this pathspec ever been
added — is computable from the read-only substrate in `src/git.rs`, and it fails
the exact case that motivated negation: if day left no trace in a guest tree,
there is no history of a trace either, so the probe reports vacuous forever,
precisely when the telos is genuinely held. Absence of the artifact is also
absence of the evidence that anything could have produced it. The companion rule
has no such circularity and is not git-shaped, so it answers `claim` and
`command` negation on the same terms.

`command` is where negation stops being uniform, and REQ-18 is the exception
stated rather than papered over. `run_command` maps every non-zero exit to
`Unsatisfied` — correct and conservative for an existence check, and a false
clean once inverted, since a mistyped pathspec exits non-zero exactly as
"searched and found nothing" does. A spawn failure is already `Verdict::Error`,
so the hazard is narrower than it first looks and lives entirely in the exit
code, which is why declaring the expected one closes it.

The precedent for the outcome name is deliberate — `scripts/revert-demo.py`
already reports `VACUOUS` as one of seven outcomes, and CLAUDE.md records that it
is a finding rather than a nuisance. Reusing the word means one concept rather
than two.

**Authorship** extends `ClaimShape`, which today carries `kind`, `contains`,
`starts_with`, `subject` and `block` — all constants. An author exclusion is
also a constant once the active identity is resolved, and day already resolves it
through the read path `src/hooks.rs` uses at session start, so this needs no new
substrate. It is the narrowest of the three additions and the one that makes an
adoption witness honest.

**Correspondence** is the one that is not a constant, which is exactly day#107's
diagnosis: `ClaimShape` cannot reference the material witness's resolved value.
The smallest form that closes it makes `docs::reconcile_boundary`'s
`text.contains(tag)` a special case of the general rule rather than a parallel
mechanism, which was day#103's original goal. This is the requirement most likely
to want its own pass if the milestone runs long, and REQ-9 is what keeps a
half-built version from shipping as a quiet check.

**Scope precision** is the smallest change of the five: the scope value grows an
exclusion alongside its pattern. day#85's constraint is the one to hold — the
result must still name a class, not collapse onto a single artifact, which is
what `docs/CONVENTIONS.md` argues prefix scopes exist for.

The four reporting requirements share a property worth naming: each is a place
where day knows something and does not say it. That is the same defect class the
companion pass is about, and it is why they are here rather than filed as
polish — a witness nobody can see declared is not a witness.

## Resolved Questions

- RQ-1: **All nine gaps are in scope for this design**, rather than the
  expressiveness core alone. They interact: the v1.0 telos needs disjunction,
  authorship scoping and a scope exclusion together, and splitting them would
  produce three passes that each cannot demonstrate their own case.
- RQ-2: **Negation is a modifier on existing probes**, per day#125's own
  proposal, and not a distinct probe kind. It reuses the evaluation and keeps the
  surface small.
- RQ-3: **A vacuity guard is what makes negation safe**, mirroring the rule this
  milestone established for witness-absence. A negated probe forbidding something
  that has never existed is non-functional in the same way an unwitnessed telos
  is: it reports clean without establishing anything. Both must be findings.
- RQ-4: **`VACUOUS` is the outcome name**, reusing `scripts/revert-demo.py`'s
  vocabulary rather than minting a synonym, so the repository has one concept for
  "the check was taken away and nothing noticed".
- RQ-5: **The disjunctive form nests inside the existing witness array** rather
  than adding a parallel key, so backward compatibility is structural rather than
  maintained.
- RQ-6: **`src/status.rs`'s no-probes-declared message stays separate** from the
  unwitnessed-telos renderer, carried over from the companion pass: it reports a
  project-level fact, and day#108 already rejected routing it to a remedy that
  does not remedy it.
- RQ-7: **Vacuity is decided by a companion positive witness**, not by git
  history. History-wide matching was the intuitive rule and is provably wrong for
  the case that motivated negation: if day left no trace in a guest tree, there is
  no history of a trace either, so the probe would report `VACUOUS` forever —
  exactly when the telos is genuinely held. The companion rule is correct on all
  three states, is uniform across probe kinds rather than git-shaped, and
  generalizes `PairedWitness` instead of adding a parallel mechanism.
- RQ-8: **A negated `command` probe declares its expected exit code.** Negation
  was to be a uniform modifier, and `run_command` is where that does not hold:
  every non-zero exit becomes `Unsatisfied`, which is conservative un-negated and
  a false clean negated. Requiring the author to state which code means "found
  nothing" makes the failure loud instead of silent, and keeps the expressiveness
  that excluding `command` from negation would have cost.
- RQ-9: **Correspondence stays in this pass, sequenced last.** It is the only
  requirement with real machinery risk, so it is implemented after the other
  eight and can be dropped without unpicking them. It closes day#103's own case,
  open across two milestones and the reason the pair mechanism exists.
- RQ-10: ~~The unrun-command problem is served by disjunction~~ — **superseded
  by RQ-11.** It proposed a group whose members were the command probe and a
  recorded assessment. Kept here rather than rewritten, because the wrong turn
  is the useful part: it read as obviously right and did not survive being
  built.
- RQ-11: **A witness must never be a flattened verdict, and a telos has no
  permanent green.** RQ-10 fails three ways, in increasing depth. `assessment`
  is `{kind: Result, subject: "atom/*"}` — an *atom* assessment, `[MATERIAL]` on
  this repo today on the strength of one claim on `atom/assess-docs`, so it
  would have reported three foundational teloi green on unrelated evidence. It
  cannot be narrowed, because `effective_probe` ignores `--scope` for a claim
  probe on the grounds that replacing its marker could *widen* what counts. And
  the general reason: an assessment has fine-grained structure — which witnesses
  resolved, how, against what — while "a `Result` exists" keeps one bit and
  discards the rest. A witness consuming that bit makes the flattening durable,
  and the telos is green forever because somebody once said so.

  So the binary is a **lens**: a filter over the fine-grained witness state to
  get an up/down readout for an exit code or a status bar, derived per
  invocation and never stored. day is already partly this shape — `kan result`
  on a telos is prose rather than a boolean, `is_clean` counts only material
  `Unsatisfied` so absence of evidence cannot fail a telos, and the render
  refuses a cross-frame reading. What is missing is that nothing says so, and
  nothing stops a witness type from being a flattened verdict.

  REQ-12 is therefore re-answered with the option RQ-10 rejected: render the
  exact `--run` invocation where the material section would otherwise be empty.
  The four foundational teloi carry their command probes alone and report
  `NOT RUN` by default, which is honest — day#86 predicted that day's
  foundational properties are evidenced by its own tests, and a test's evidence
  is running it.

## Out of Scope

- The interview mechanism itself — the atom, its command, the single renderer and
  its scan. That is the companion pass; this one decides what "witnessed" may
  mean, that one decides what day does when nothing is.
- Declaring the teloi. The four foundational witness sets are settled in the
  companion pass and land there; the v1.0 declarations land after this.
- The pack mechanism (day#73), which transports a vocabulary and does not define
  one.
- Any change to how `schema/witness` is discovered or to the subject it lives on.
- New probe *kinds*. Every requirement here is a modifier, a comparison or a
  report over the four kinds that exist.
