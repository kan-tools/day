//! Where in the atom graph the work currently sits — inferred from evidence,
//! never tracked.
//!
//! An atom is a **candidate** for "current" when its declared inputs are
//! materially present and its outputs are not: the work needed to run it
//! exists, and what it would produce does not yet. That is derived from the
//! same probes an assessment uses, so day records nothing and stores nothing
//! — the position is recomputed each time, and `docs/CONVENTIONS.md`'s refusal
//! to track "how far along are we" stays intact.
//!
//! **Ambiguity is reported, not resolved.** When several atoms fit the
//! evidence, all are named; guessing one would be a claim day cannot support.
//!
//! **Inference reads; it never executes.** `path`, `tag`, and `claim` are all
//! reads — of the working tree, of the tag list, of kan's own log — and all
//! run here. `command` is execution and does not: inference happens on every
//! session start, and running project-declared commands as a side effect of
//! *starting a session* would be a far larger widening than `--run` ever was.
//! [`materialized`] short-circuits a command probe and otherwise passes
//! [`Authorization::Report`], the authorization that cannot execute anything,
//! so the rule holds by construction rather than by discipline.
//!
//! **Position is relative to the current cycle.** On a repo with any history
//! every artifact type exists — there is always *some* `v*` tag, *some* past
//! verdict — so a question phrased "does one exist" can only ever answer yes,
//! and day's own log reported four candidate atoms forever (day#60). Each
//! probe is therefore resolved against a [`Boundary`], the last release: a
//! path counts if it *changed since*, a tag if it was *created since*, a
//! claim if it was *recorded since*. A repo with no release has no boundary
//! and falls back to the cumulative reading, which is conservative rather
//! than clever — the alternative, treating all of history as the current
//! cycle, is exactly the failure this fixes.
//!
//! None of that reaches assessment. `assess telos` and `assess atom` ask
//! whether a witness was *ever* produced and keep calling
//! [`probe::evaluate`], which has no boundary to pass.
//!
//! fallback: no-release-boundary

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::atoms::Atom;
use crate::git::{Boundary, Git};
use crate::probe::{self, Authorization, ClaimLog, Failures, Probe, ReadFailure, Verdict};

/// Whether an artifact type is materially present, and how sure day is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// A `path`, `tag`, or `claim` probe found it — in this cycle, when a
    /// boundary is in force.
    Present,
    /// A probe ran and found nothing.
    Absent,
    /// No probe is declared for this type, or its probe is a `command` —
    /// which inference will not run. Presence is unknown, not absent, and the
    /// difference matters: treating unknown as absent would make every atom
    /// with a command-probed input look ready.
    Unknown,
}

/// Resolves a probe **for position**: relative to the cycle boundary when
/// there is one, cumulative when there is not, and never executing a command.
///
/// Shared with [`crate::status`] so the `done` criteria `day status` displays
/// read the same cycle as the position above them. Two renderings of one
/// computation, which is the same reason the status line and the long form
/// share [`crate::status::compute`].
pub fn resolve(
    probe: &Probe,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
) -> Verdict {
    // The caller renders this verdict itself — see `Failures::AlreadyReported`.
    //
    // True of `day status`'s long form, which prints each criterion's verdict
    // WITH its detail. Not true of `render_line`, which reduces the same
    // verdicts to `met/total` and drops the reason — and `render_line` is both
    // the status bar and what session-start puts in the model's context. So the
    // reason reaches one of two consumers.
    //
    // Left as-is because it is pre-existing (the bare `None` had exactly this
    // reach) and widening it here would change what the bar reports, which is a
    // separate decision. Recorded rather than papered over: the variant's name
    // makes the claim checkable, and this is where it is only partly true.
    resolve_collecting(probe, git, log, boundary, Failures::AlreadyReported)
}

/// [`resolve`], with somewhere to put reads that could not happen.
///
/// Split rather than adding a parameter to `resolve` because `resolve` is also
/// called where the caller renders the verdict itself (`status.rs`), and there
/// the reason is already on screen.
pub fn resolve_collecting(
    probe: &Probe,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
    failures: Failures<'_>,
) -> Verdict {
    resolve_corresponding(
        probe,
        git,
        log,
        boundary,
        probe::Correspondence::Unavailable,
        failures,
    )
}

/// [`resolve_collecting`] with the material half of a pair in scope, for a
/// record shape that must refer to *that instance* rather than merely exist.
///
/// A separate entry point rather than a sixth parameter on the common one:
/// exactly one caller has a material instance to supply — the pair comparison
/// in [`infer`] — and widening the signature everyone else uses would invite
/// passing `Unavailable` by habit at a site where it is wrong.
pub fn resolve_corresponding(
    probe: &Probe,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
    correspondence: probe::Correspondence<'_>,
    failures: Failures<'_>,
) -> Verdict {
    match (probe, boundary) {
        // Reported as not-run, never executed, boundary or no boundary. A
        // cycle is a question about *when* evidence appeared; it does not
        // make executing something any more acceptable at session start.
        (Probe::Command(_), _) => probe::evaluate(probe, git, log, Authorization::Report),
        // Cumulative, boundary or not. A universal states something about the
        // record AS A WHOLE -- "every design was reviewed" -- and narrowing it
        // to a cycle would weaken it to "every design recorded since the last
        // release", which goes quiet exactly while a milestone is in progress.
        (Probe::Every(_), _) => probe::evaluate(probe, git, log, Authorization::Report),
        // Cumulative, and never executing: `evaluate` carries the same
        // `Authorization::Report` every other inference read does, so a
        // forbidden *command* reports NOT RUN here exactly as a positive one
        // would. Absence is a standing property rather than a cycle's work, so
        // narrowing it to a boundary would answer a different question.
        (Probe::Absent(_), _) => probe::evaluate(probe, git, log, Authorization::Report),
        // No boundary — the cumulative reading. A claim probe is handled here
        // rather than falling into `evaluate` so it can still report a read it
        // could not make: `evaluate` has nowhere to put one.
        //
        // **This arm is the default one.** No release means no boundary, so
        // every repo without a `v*` tag takes it — which is every fresh clone,
        // and was every fixture in this file. Wiring only the `Some(boundary)`
        // path left the collector dead on exactly the population it was for,
        // repeating the defect CLAUDE.md records about the position
        // fingerprint.
        (Probe::Claim(shape), None) => {
            probe::claims_matching(shape, log, None, correspondence, failures)
        }
        (_, None) => probe::evaluate(probe, git, log, Authorization::Report),
        (Probe::Path(pathspec), Some(boundary)) => {
            match git.changed_files_matching(&boundary.tag, pathspec) {
                Ok(files) if files.is_empty() => Verdict::Unsatisfied(format!(
                    "no file matching `{pathspec}` changed since {}",
                    boundary.tag
                )),
                Ok(files) => Verdict::Satisfied(format!(
                    "{} file(s) matching `{pathspec}` changed since {}",
                    files.len(),
                    boundary.tag
                )),
                Err(e) => Verdict::Error(format!("could not diff against {}: {e}", boundary.tag)),
            }
        }
        (Probe::Tag(pattern), Some(boundary)) => match git.tags_with_dates(pattern) {
            // Strictly after: the tag that *is* the boundary closed the last
            // cycle, so it is not evidence of this one. That is what lets
            // `release` stop looking finished the moment a new cycle opens.
            Ok(tags) => match tags.iter().find(|(_, at)| *at > boundary.at_unix) {
                Some((tag, _)) => {
                    Verdict::Satisfied(format!("git tag {tag}, created since {}", boundary.tag))
                }
                None => Verdict::Unsatisfied(format!(
                    "no tag matching `{pattern}` created since {}",
                    boundary.tag
                )),
            },
            Err(e) => Verdict::Error(format!("could not list tags: {e}")),
        },
        (Probe::Claim(shape), Some(boundary)) => probe::claims_matching(
            shape,
            log,
            Some(boundary.at_micros()),
            correspondence,
            failures,
        ),
    }
}

/// Resolves one artifact type against the witness probes, without ever
/// executing a command.
fn materialized(
    kind: &str,
    probes: &BTreeMap<String, Probe>,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
    failures: Failures<'_>,
) -> Presence {
    match probes.get(kind) {
        None => Presence::Unknown,
        // A command probe is deliberately not run here (REQ-6). Its evidence
        // is unknowable at inference time, which is honest — the alternative
        // is executing it on every session start.
        Some(Probe::Command(_)) => Presence::Unknown,
        Some(probe) => match resolve_collecting(probe, git, log, boundary, failures) {
            Verdict::Satisfied(_) => Presence::Present,
            Verdict::Unsatisfied(_) => Presence::Absent,
            // NotRun should be unreachable for path/tag/claim, but if it
            // arises it is unknown rather than absent.
            _ => Presence::Unknown,
        },
    }
}

/// Whether an atom's declared outputs, taken together, are present. Three
/// states rather than a bool, because "probed and absent" and "unknowable"
/// must not collapse: off-sequence detection treats a *definitely absent*
/// upstream as evidence of a skip, and an *unknowable* one as no evidence at
/// all. Conflating them flags every atom with an unprobed output as skipped,
/// which is exactly the false positive dogfooding surfaced on day's own log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outputs {
    /// Every declared output is materially present.
    Present,
    /// Every output was probed and at least one was not found; none are
    /// unknowable. The atom demonstrably has not produced its product.
    Absent,
    /// At least one output is unknowable (no probe, or a command probe), or
    /// the atom declares no outputs. Nothing can be concluded either way.
    Unknown,
}

/// Classifies an atom's declared outputs as a whole. `Present` needs every
/// output present; a single unknowable output makes the set `Unknown`, and
/// only a fully-probed set with something missing is `Absent`.
fn classify_outputs(outputs: &[String], presence: &impl Fn(&str) -> Presence) -> Outputs {
    if outputs.is_empty() {
        return Outputs::Unknown;
    }
    let mut all_present = true;
    let mut any_unknown = false;
    for output in outputs {
        match presence(output) {
            Presence::Present => {}
            Presence::Absent => all_present = false,
            Presence::Unknown => {
                all_present = false;
                any_unknown = true;
            }
        }
    }
    if all_present {
        Outputs::Present
    } else if any_unknown {
        Outputs::Unknown
    } else {
        Outputs::Absent
    }
}

/// One atom's standing against the current evidence.
#[derive(Debug, Clone)]
pub struct Standing {
    pub atom: String,
    /// Inputs that are materially present.
    pub inputs_present: Vec<String>,
    /// Inputs a probe looked for and did not find.
    pub inputs_missing: Vec<String>,
    /// Inputs whose presence is unknowable (no probe, or a command probe).
    pub inputs_unknown: Vec<String>,
    /// This atom's own outputs, as a three-way presence.
    pub outputs: Outputs,
    /// The artifact types this atom declares it produces, by name.
    ///
    /// Carried because [`Outputs`] collapses them to a presence verdict, and
    /// day#98 needs the names: whether an inputless atom is a true source or a
    /// *convergent root* depends on whether anything downstream consumes what
    /// it makes, which the verdict cannot answer.
    pub outputs_declared: Vec<String>,
}

impl Standing {
    /// A candidate for "current": everything a probe could check about its
    /// inputs is present, and its outputs are not already all present. Unknown
    /// inputs do not disqualify — they are reported, and pretending they are
    /// absent would hide a ready atom. Unknown *outputs* likewise keep an atom
    /// a candidate: an atom whose product cannot be detected has not been
    /// shown to be finished.
    pub fn is_current(&self) -> bool {
        self.inputs_missing.is_empty() && self.outputs != Outputs::Present
    }

    /// Source atoms have no declared inputs; their inputs come from outside
    /// the vocabulary and are not evidence of position.
    ///
    /// **A convergent root is not a source** (day#98). An atom can declare no
    /// inputs and still be gated *on*: if something downstream lists what it
    /// produces as an input, then the absence of its outputs is precisely what
    /// blocks that downstream work, and is exactly what a reader needs named.
    /// Excluding those made day structurally blind to them — on the vocabulary
    /// that found this, the two inputless atoms were the build-out's first and
    /// last, so day could not name A1 while A1 was the work and could not name
    /// A4 once A4 was.
    ///
    /// `consumed` is every artifact type any atom declares as an input. A true
    /// source — one whose outputs nobody consumes — is still excluded, which
    /// preserves what this rule was protecting: naming an atom nothing gates
    /// would be noise.
    ///
    /// This was met once before and routed around locally rather than fixed:
    /// `tests/declared_vocabulary.rs` records hitting it while building a
    /// fixture and changed the fixture. Encountered, not generalised, which is
    /// why it survived to a real vocabulary.
    fn is_source(&self, consumed: &BTreeSet<&str>) -> bool {
        self.inputs_present.is_empty()
            && self.inputs_missing.is_empty()
            && self.inputs_unknown.is_empty()
            && !self
                .outputs_declared
                .iter()
                .any(|out| consumed.contains(out.as_str()))
    }
}

#[derive(Debug)]
pub struct Report {
    pub standings: Vec<Standing>,
    /// Atoms whose inputs are all satisfied and outputs not yet produced.
    pub current: Vec<String>,
    /// Off-sequence findings: an atom's outputs are present while an upstream
    /// atom's outputs are not, so a step was skipped.
    pub off_sequence: Vec<String>,
    /// Pairs of atoms whose order day **could not establish**, because they
    /// are on a cycle through `next` and "upstream" is undefined there
    /// (day#113).
    ///
    /// A separate list rather than more `off_sequence` lines, for the reason
    /// [`Presence::Unknown`] is separate from [`Presence::Absent`] one level
    /// down: an order day could not check is not an order it checked and found
    /// clean, and rendering the two the same way is how a reader stops
    /// believing either. Empty for any vocabulary that has moved its feedback
    /// edges to `revisits`, which is every migrated one.
    pub unordered: Vec<String>,
    /// Reads that could not happen while inferring this position, so a caller
    /// can say the report is partial instead of presenting it as whole
    /// (`.design/declared-blocks.md` REQ-4).
    pub read_failures: Vec<ReadFailure>,
    /// Artifact types that **exist but are not written down**: a declared
    /// `material` witness is satisfied for this cycle and the declared `record`
    /// witness is not (day#103).
    ///
    /// A separate list rather than a fourth [`Presence`], because it is not a
    /// statement about whether the work happened — it plainly did — but about
    /// whether the log knows. Folding it into presence would make an
    /// unrecorded artifact look absent and put the atom back in `current`,
    /// which is the collapse this exists to undo.
    pub unrecorded: Vec<String>,
}

/// The orders day cannot establish over this atom set, phrased for the
/// off-sequence surface. See [`Report::unordered`].
///
/// Public and free-standing because it needs **no probes at all** — a cycle in
/// `next` is a fact about the declaration. `status::compute` short-circuits to
/// "uncheckable" when a project has declared no readable witness probe, and
/// that path must still say the graph could not be ordered: the two are
/// independent reasons day knows less than the report might imply, and letting
/// one swallow the other is how a could-not-check goes quiet.
pub fn unordered(atoms: &[Atom]) -> Vec<String> {
    let mut out: Vec<String> = crate::atoms::Forward::build(atoms)
        .cycles()
        .iter()
        .map(|cycle| {
            format!(
                "whether a step was skipped between {} is unchecked: {}",
                cycle.atoms.join(" and "),
                cycle.message()
            )
        })
        .collect();
    out.sort();
    out
}

/// Infers position from the atom set and the witness probes, resolving each
/// artifact type against git (`path`/`tag`) and kan (`claim`) relative to
/// `boundary`, without ever running a command.
///
/// Each artifact type is resolved **at most once**. An artifact appears in
/// several atoms' interfaces — `code-change` is an output of one atom and an
/// input to three — and a `claim` probe scans the whole log, so resolving per
/// mention would multiply a session-start read by the size of the vocabulary
/// for answers that cannot differ within a single inference.
/// Takes the whole [`WitnessSchema`] rather than just its material probes.
///
/// That is REQ-8, and it is deliberate: day#101's recurring defect is a
/// guarantee wired at a call site, and passing `&schema.probes` would let a
/// future channel compute a position while silently skipping the record half.
/// With the declaration itself as the parameter there is no half to pass.
pub fn infer(
    atoms: &[Atom],
    schema: &crate::telos::WitnessSchema,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
) -> Report {
    // RefCell because `memoized` takes an `Fn`, and each resolution may add to
    // this. Resolution happens at most once per artifact type, so a failure
    // cannot be recorded twice for one type.
    let failures: RefCell<Vec<ReadFailure>> = RefCell::new(Vec::new());

    // Hoisted rather than passed inline, so the record comparison below can ask
    // the SAME memoized resolver instead of reading the material half a second
    // time (F1). The old shape called `materialized` again directly, which had
    // two defects at once: it passed no failure collector, so a read it could
    // not make was silently dropped; and it bypassed the memo, so collecting
    // properly would have reported the same failure twice for any type an atom
    // also names. One resolution per type, one collection, both fixed by
    // structure rather than by remembering the right argument.
    let resolve_presence = memoized(|kind| {
        let mut collected = Vec::new();
        let presence = materialized(
            kind,
            &schema.probes,
            git,
            log,
            boundary,
            Failures::Collect(&mut collected),
        );
        failures.borrow_mut().extend(collected);
        presence
    });

    let mut report = infer_with(atoms, &resolve_presence);

    // The material/record comparison (REQ-2). Only types declaring both halves
    // can be asked, so a project that declared no pair gets an empty list and
    // no behaviour change whatsoever.
    //
    // Resolved through the same `resolve_collecting` every other probe goes
    // through, so a record witness is cycle-scoped exactly like a material one:
    // "was this recorded *in this cycle*", not "was it ever recorded". Without
    // that, a release claim from six milestones ago would satisfy today's.
    for (kind, record) in &schema.records {
        if !schema.probes.contains_key(kind) {
            continue;
        }
        // Memoized: free if an atom already named this type, and collected
        // exactly once either way.
        if resolve_presence(kind) != Presence::Present {
            continue;
        }
        // The material instances this record must refer to, when it says it
        // must. Resolved from the material half rather than from the verdict
        // summary: `Verdict::Satisfied` carries prose for a human ("git tag
        // v0.6.0-beta.1"), and parsing a name back out of a sentence is how a
        // correspondence check would quietly start matching the wrong thing.
        //
        // Unscoped by the boundary on purpose. The question is "which artifact
        // is this", not "when did it appear" — and for `published-artifact` the
        // boundary *is* the newest tag, which is precisely day#107's
        // observation that cycle-scoping makes the material half absent by
        // construction.
        let material = schema
            .probes
            .get(kind)
            .and_then(|probe| probe::instances(probe, git));
        let correspondence = match &material {
            Some(instances) => probe::Correspondence::Material(instances),
            None => probe::Correspondence::Unavailable,
        };

        let mut collected = Vec::new();
        let seen = resolve_corresponding(
            record,
            git,
            log,
            boundary,
            correspondence,
            Failures::Collect(&mut collected),
        );
        failures.borrow_mut().extend(collected);

        // Only a probe that ran and found nothing counts. An `Error` is a read
        // that did not happen and is already reported as a read failure;
        // calling it "unrecorded" would manufacture a finding out of a failure
        // to look, which is the exact inversion `telos/honest-reads` forbids.
        if matches!(seen, Verdict::Unsatisfied(_)) {
            report.unrecorded.push(kind.clone());
        }
        // **An unanswerable correspondence is reported, never silent.** A
        // record shape declaring `mentions_material` against a material half
        // that names no instance produces `Error`, and the branch above
        // correctly refuses to call that "unrecorded" — which would leave it
        // reported nowhere at all. day#107 states the constraint directly: an
        // unanswerable comparison is UNCHECKED, not silence, and it is the same
        // rule `design check` got in day#105. `resolve_corresponding`'s own read
        // failures are already collected above; this is the one an Error verdict
        // carries instead.
        if let Verdict::Error(why) = &seen {
            failures.borrow_mut().push(ReadFailure {
                message: format!("`{kind}`: {why}"),
                version_skew: false,
            });
        }
    }

    // The resolver borrows `failures`, so it has to go before the cell can be
    // consumed. Explicit rather than relying on scope order, since the drop is
    // load-bearing for the line below it.
    drop(resolve_presence);
    report.read_failures = failures.into_inner();
    report
}

/// Wraps a resolver so each artifact type is looked up once per inference.
///
/// Its own function rather than a closure inlined above so the property can
/// be tested against the shipped code instead of a copy of it — the caching
/// is not cosmetic, it is what keeps a whole-log `claim` read from happening
/// once per mention.
fn memoized(resolve: impl Fn(&str) -> Presence) -> impl Fn(&str) -> Presence {
    let memo: RefCell<BTreeMap<String, Presence>> = RefCell::new(BTreeMap::new());
    move |kind| {
        if let Some(known) = memo.borrow().get(kind) {
            return *known;
        }
        let presence = resolve(kind);
        memo.borrow_mut().insert(kind.to_string(), presence);
        presence
    }
}

/// The pure core of inference: it takes a function answering whether each
/// artifact type is present and computes standings, candidates, and
/// off-sequence findings. Split from [`infer`] so this logic is tested by
/// injecting presences directly — no git subprocess, so the position tests are
/// deterministic rather than depending on spawning a stub under load (day#64).
fn infer_with(atoms: &[Atom], presence: impl Fn(&str) -> Presence) -> Report {
    let standings: Vec<Standing> = atoms
        .iter()
        .map(|atom| {
            let mut inputs_present = Vec::new();
            let mut inputs_missing = Vec::new();
            let mut inputs_unknown = Vec::new();
            for input in &atom.interface.inputs {
                match presence(input) {
                    Presence::Present => inputs_present.push(input.clone()),
                    Presence::Absent => inputs_missing.push(input.clone()),
                    Presence::Unknown => inputs_unknown.push(input.clone()),
                }
            }
            let outputs = classify_outputs(&atom.interface.outputs, &presence);
            Standing {
                atom: atom.name.clone(),
                inputs_present,
                inputs_missing,
                inputs_unknown,
                outputs,
                outputs_declared: atom.interface.outputs.clone(),
            }
        })
        .collect();

    // day#98: every artifact type some atom declares as an input. An inputless
    // atom producing one of these is a convergent root, not a source — see
    // [`Standing::is_source`]. Computed from the atom set already in hand: no
    // probe, no read, no declaration.
    let consumed: BTreeSet<&str> = atoms
        .iter()
        .flat_map(|a| a.interface.inputs.iter().map(String::as_str))
        .collect();

    let current: Vec<String> = standings
        .iter()
        .filter(|s| !s.is_source(&consumed) && s.is_current())
        .map(|s| s.atom.clone())
        .collect();

    // Off-sequence: an atom produced its outputs, but an atom it lists as a
    // predecessor (via `next`) is *demonstrably* missing its own. Availability
    // accumulates along a path, so a downstream artifact existing while an
    // upstream one is definitely absent means a step was skipped.
    //
    // The edges come from [`atoms::Forward`], never the raw declaration
    // (day#113). "Upstream" is undefined inside a cycle — each atom is upstream
    // of the other — so before the split this fired on whichever half had not
    // produced yet, which during any build is the whole build phase of every
    // milestone. What `Forward` had to drop is reported as `unordered` rather
    // than skipped quietly: could-not-check, never checked-and-clean.
    //
    // "Definitely absent" is [`Outputs::Absent`], never [`Outputs::Unknown`].
    // An upstream whose output has no probe (or a command probe) is unknowable
    // — not evidence of a skip. Flagging it anyway was a false positive on
    // day's own log, where `verdict` and `merged-change` are unprobed and made
    // every probed downstream look skipped. Found by running the tool, not by
    // the test, which only ever used probed artifacts.
    let by_name: BTreeMap<&str, &Standing> =
        standings.iter().map(|s| (s.atom.as_str(), s)).collect();
    let forward = crate::atoms::Forward::build(atoms);
    let mut off_sequence = Vec::new();
    for atom in atoms {
        let successor_produced = |name: &str| {
            by_name
                .get(name)
                .is_some_and(|s| s.outputs == Outputs::Present)
        };
        let upstream_definitely_absent = by_name
            .get(atom.name.as_str())
            .is_some_and(|s| s.outputs == Outputs::Absent);
        for successor in forward.successors(&atom.name) {
            if successor_produced(successor) && upstream_definitely_absent {
                off_sequence.push(format!(
                    "{} produced its output but upstream {} did not — a step was skipped",
                    successor, atom.name
                ));
            }
        }
    }
    off_sequence.sort();
    off_sequence.dedup();

    Report {
        unordered: unordered(atoms),
        read_failures: Vec::new(),
        // Filled by [`infer`], which is the only caller with the schema needed
        // to answer it. `infer_with` is given a bare presence function and
        // cannot know what a record witness would be, so an empty list here is
        // "not asked", not "asked and clean" — and the only path that skips
        // asking is the test seam.
        unrecorded: Vec::new(),
        standings,
        current,
        off_sequence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atoms::Interface;
    use crate::kan_client::KanClient;

    fn atom(name: &str, inputs: &[&str], outputs: &[&str], next: &[&str]) -> Atom {
        Atom {
            name: name.to_string(),
            cid: format!("bafy-{name}"),
            interface: Interface {
                inputs: inputs.iter().map(|s| s.to_string()).collect(),
                outputs: outputs.iter().map(|s| s.to_string()).collect(),
                next: next.iter().map(|s| s.to_string()).collect(),
                revisits: vec![],
                done: vec![],
            },
        }
    }

    /// Resolves each artifact type from a fixed table — the presence a probe
    /// *would* report, injected directly. This is what lets the inference logic
    /// be tested without spawning a git stub: day#64 was a lib unit test
    /// flaking because it exec'd a freshly-written script under CI parallelism,
    /// and the logic under test never needed a real process. Anything not in
    /// the table is `Unknown`, matching a type with no probe.
    fn presences<'a>(pairs: &'a [(&'a str, Presence)]) -> impl Fn(&str) -> Presence + 'a {
        move |kind| {
            pairs
                .iter()
                .find(|(k, _)| *k == kind)
                .map(|(_, p)| *p)
                .unwrap_or(Presence::Unknown)
        }
    }

    #[test]
    fn a_design_doc_present_and_no_code_change_puts_you_in_build() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        // design-doc present, code-change absent; intent has no probe.
        let report = infer_with(
            &atoms,
            presences(&[
                ("design-doc", Presence::Present),
                ("code-change", Presence::Absent),
            ]),
        );
        assert_eq!(report.current, vec!["build"], "{:?}", report.standings);
    }

    /// day#98, AC-1 — a convergent root is a candidate; a true source is not.
    ///
    /// This cannot be asserted against day's own vocabulary: all seven of its
    /// atoms declare inputs, so neither branch is reachable here. That is the
    /// point of the issue — the defect was invisible in this repo and bit a
    /// vocabulary with twelve bespoke atoms, where the two inputless ones were
    /// the build-out's first and its last.
    ///
    /// Both branches in one test on purpose. Asserting only that the root is
    /// named would be satisfied by deleting the `is_source` filter outright,
    /// which would reintroduce the noise the filter exists to prevent.
    #[test]
    fn a_convergent_root_is_current_but_a_true_source_is_not() {
        let atoms = [
            // No inputs, and `schema` is consumed downstream -> convergent root.
            atom("declare", &[], &["schema"], &["store"]),
            atom("store", &["schema"], &["store-impl"], &[]),
            // No inputs, and nothing declares `scratch` as an input -> a true
            // source, which stays excluded.
            atom("doodle", &[], &["scratch"], &[]),
        ];

        let report = infer_with(
            &atoms,
            presences(&[
                ("schema", Presence::Absent),
                ("store-impl", Presence::Absent),
                ("scratch", Presence::Absent),
            ]),
        );

        assert!(
            report.current.contains(&"declare".to_string()),
            "an inputless atom whose output something consumes gates that work, \
             so it must be nameable as current: {:?}",
            report.current
        );
        assert!(
            !report.current.contains(&"doodle".to_string()),
            "an inputless atom nothing consumes is a true source and stays \
             excluded, or the filter's purpose is lost: {:?}",
            report.current
        );
    }

    /// The discriminator is *consumption*, not merely having outputs. A
    /// convergent root whose output is already present is finished, not
    /// current — so the day#98 fix must not make inputless atoms permanently
    /// current, which is the obvious wrong way to satisfy the test above.
    #[test]
    fn a_convergent_root_whose_output_exists_is_not_current() {
        let atoms = [
            atom("declare", &[], &["schema"], &["store"]),
            atom("store", &["schema"], &["store-impl"], &[]),
        ];
        let report = infer_with(
            &atoms,
            presences(&[
                ("schema", Presence::Present),
                ("store-impl", Presence::Absent),
            ]),
        );
        assert!(
            !report.current.contains(&"declare".to_string()),
            "its output exists, so it is done: {:?}",
            report.current
        );
        assert!(
            report.current.contains(&"store".to_string()),
            "and the work it gates is now current: {:?}",
            report.current
        );
    }

    #[test]
    fn both_present_means_build_is_no_longer_current() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        let report = infer_with(
            &atoms,
            presences(&[
                ("design-doc", Presence::Present),
                ("code-change", Presence::Present),
            ]),
        );
        assert!(
            !report.current.contains(&"build".to_string()),
            "build's output exists, so it is not current: {:?}",
            report.current
        );
    }

    #[test]
    fn an_unknown_input_leaves_an_atom_a_candidate() {
        let atoms = [atom("check", &["passing-tests"], &["verdict"], &[])];
        // passing-tests is unknowable (e.g. a command probe, never run here).
        let report = infer_with(&atoms, presences(&[("passing-tests", Presence::Unknown)]));
        let s = &report.standings[0];
        assert_eq!(s.inputs_unknown, vec!["passing-tests"]);
        assert!(
            s.inputs_missing.is_empty(),
            "an unknowable input is not missing, so the atom stays a candidate"
        );
    }

    /// The command-probe path of [`materialized`] specifically: it must return
    /// `Unknown` without ever running the command. Constructed so no process is
    /// spawned — `materialized` short-circuits a command probe before touching
    /// git — so this stays deterministic too.
    #[test]
    fn a_command_probe_is_unknown_and_never_run() {
        let probes: BTreeMap<String, Probe> =
            [("passing-tests".to_string(), Probe::Command("exit 1".into()))]
                .into_iter()
                .collect();
        // A Git and a kan that would error if invoked; the command arm calls
        // neither. Checked with and without a boundary, because REQ-6 is not
        // a property of the unbounded path — a cycle must not make execution
        // acceptable.
        let git = Git::with_bin(".", "definitely-not-a-real-git-binary".to_string());
        let client = KanClient::with_bin(".", "definitely-not-a-real-kan-binary".to_string());
        let boundary = Boundary {
            tag: "v0.6.0".into(),
            at_unix: 1_700_000_000,
        };
        for bound in [None, Some(&boundary)] {
            assert_eq!(
                materialized(
                    "passing-tests",
                    &probes,
                    &git,
                    &ClaimLog::new(&client),
                    bound,
                    // The assertion is about Presence, and the test renders
                    // nothing — no failure can arise from a command probe,
                    // which is never run here.
                    Failures::AlreadyReported
                ),
                Presence::Unknown
            );
        }
    }

    /// AC-6, one level up from [`materialized`]: a whole inference over a
    /// schema whose `verdict` is a command probe executes nothing and leaves
    /// `verdict` unknowable — so the atom producing it stays a candidate
    /// rather than looking finished or looking skipped.
    #[test]
    fn inference_over_a_command_probed_schema_executes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("pwned");
        let atoms = [
            atom("build", &["design-doc"], &["code-change"], &["review"]),
            atom("review", &["code-change"], &["verdict"], &[]),
        ];
        let schema = crate::telos::WitnessSchema {
            probes: [(
                "verdict".to_string(),
                Probe::Command(format!("touch {}", marker.display())),
            )]
            .into_iter()
            .collect(),
            ..Default::default()
        };
        let git = Git::with_bin(dir.path(), "definitely-not-a-real-git-binary".to_string());
        let client =
            KanClient::with_bin(dir.path(), "definitely-not-a-real-kan-binary".to_string());

        let report = infer(&atoms, &schema, &git, &ClaimLog::new(&client), None);
        assert!(
            !marker.exists(),
            "inference executed a command probe — REQ-6 is broken"
        );
        let review = report
            .standings
            .iter()
            .find(|s| s.atom == "review")
            .unwrap();
        assert_eq!(review.outputs, Outputs::Unknown);
    }

    /// The memo is not an optimization detail: a `claim` probe scans the
    /// whole log, and `code-change` is mentioned by four of day's seven
    /// atoms. Resolving per mention would multiply a session-start read by
    /// the vocabulary size. Counted directly, since the cost is invisible to
    /// every other assertion here.
    #[test]
    fn each_artifact_type_is_resolved_at_most_once() {
        use std::cell::Cell;
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &["review"]),
            atom("review", &["code-change"], &["verdict"], &[]),
        ];
        let calls = Cell::new(0usize);
        let report = infer_with(
            &atoms,
            memoized(|_kind| {
                calls.set(calls.get() + 1);
                Presence::Unknown
            }),
        );
        assert_eq!(report.standings.len(), 3);
        // intent, design-doc, code-change, verdict — four types, though
        // design-doc and code-change are each mentioned twice.
        assert_eq!(
            calls.get(),
            4,
            "each artifact type should be resolved once, not once per mention"
        );
    }

    #[test]
    fn a_downstream_output_without_its_upstream_is_off_sequence() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        // code-change present, design-doc probed and absent: build ran without
        // a design.
        let report = infer_with(
            &atoms,
            presences(&[
                ("design-doc", Presence::Absent),
                ("code-change", Presence::Present),
            ]),
        );
        assert_eq!(report.off_sequence.len(), 1, "{:?}", report.off_sequence);
        assert!(report.off_sequence[0].contains("design"));
    }

    /// The false positive dogfooding found on day's own log: an upstream atom
    /// whose output has **no probe** is unknowable, not absent, and must not
    /// read as a skipped step. `design`'s output `verdict` is unprobed while
    /// `build`'s `code-change` is present; the old code flagged a skip because
    /// it only asked "is the upstream output present", conflating unprobed with
    /// missing.
    #[test]
    fn an_unprobed_upstream_output_is_not_a_skipped_step() {
        let atoms = [
            atom("design", &["intent"], &["verdict"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        // verdict unknowable (no probe), code-change present.
        let report = infer_with(
            &atoms,
            presences(&[
                ("verdict", Presence::Unknown),
                ("code-change", Presence::Present),
            ]),
        );
        assert!(
            report.off_sequence.is_empty(),
            "an unknowable upstream output must not be reported as a skip: {:?}",
            report.off_sequence
        );
    }

    /// The counterpart that keeps the fix honest: when the upstream output IS
    /// probed and genuinely absent, the skip is still reported. Otherwise the
    /// fix above could be "never flag anything".
    #[test]
    fn a_probed_and_absent_upstream_output_is_still_a_skip() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        let report = infer_with(
            &atoms,
            presences(&[
                ("design-doc", Presence::Absent),
                ("code-change", Presence::Present),
            ]),
        );
        assert_eq!(report.off_sequence.len(), 1, "{:?}", report.off_sequence);
    }
}
