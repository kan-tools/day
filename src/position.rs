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

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::atoms::Atom;
use crate::git::{Boundary, Git};
use crate::probe::{self, Authorization, ClaimLog, Probe, Verdict};

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
    match (probe, boundary) {
        // Reported as not-run, never executed, boundary or no boundary. A
        // cycle is a question about *when* evidence appeared; it does not
        // make executing something any more acceptable at session start.
        (Probe::Command(_), _) => probe::evaluate(probe, git, log, Authorization::Report),
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
        (Probe::Claim(shape), Some(boundary)) => {
            probe::claims_matching(shape, log, Some(boundary.at_micros()))
        }
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
) -> Presence {
    match probes.get(kind) {
        None => Presence::Unknown,
        // A command probe is deliberately not run here (REQ-6). Its evidence
        // is unknowable at inference time, which is honest — the alternative
        // is executing it on every session start.
        Some(Probe::Command(_)) => Presence::Unknown,
        Some(probe) => match resolve(probe, git, log, boundary) {
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
    fn is_source(&self) -> bool {
        self.inputs_present.is_empty()
            && self.inputs_missing.is_empty()
            && self.inputs_unknown.is_empty()
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
pub fn infer(
    atoms: &[Atom],
    probes: &BTreeMap<String, Probe>,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
) -> Report {
    infer_with(
        atoms,
        memoized(|kind| materialized(kind, probes, git, log, boundary)),
    )
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
            }
        })
        .collect();

    let current: Vec<String> = standings
        .iter()
        .filter(|s| !s.is_source() && s.is_current())
        .map(|s| s.atom.clone())
        .collect();

    // Off-sequence: an atom produced its outputs, but an atom it lists as a
    // predecessor (via `next`) is *demonstrably* missing its own. Availability
    // accumulates along a path, so a downstream artifact existing while an
    // upstream one is definitely absent means a step was skipped.
    //
    // "Definitely absent" is [`Outputs::Absent`], never [`Outputs::Unknown`].
    // An upstream whose output has no probe (or a command probe) is unknowable
    // — not evidence of a skip. Flagging it anyway was a false positive on
    // day's own log, where `verdict` and `merged-change` are unprobed and made
    // every probed downstream look skipped. Found by running the tool, not by
    // the test, which only ever used probed artifacts.
    let by_name: BTreeMap<&str, &Standing> =
        standings.iter().map(|s| (s.atom.as_str(), s)).collect();
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
        for successor in &atom.interface.next {
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
                materialized("passing-tests", &probes, &git, &ClaimLog::new(&client), bound),
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
        let probes: BTreeMap<String, Probe> = [(
            "verdict".to_string(),
            Probe::Command(format!("touch {}", marker.display())),
        )]
        .into_iter()
        .collect();
        let git = Git::with_bin(dir.path(), "definitely-not-a-real-git-binary".to_string());
        let client = KanClient::with_bin(dir.path(), "definitely-not-a-real-kan-binary".to_string());

        let report = infer(&atoms, &probes, &git, &ClaimLog::new(&client), None);
        assert!(
            !marker.exists(),
            "inference executed a command probe — REQ-6 is broken"
        );
        let review = report.standings.iter().find(|s| s.atom == "review").unwrap();
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
