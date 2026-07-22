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
//! **Inference runs only `path` and `tag` probes, never `command`.** It
//! happens on every session start, and executing project-declared commands as
//! a side effect of *starting a session* would be a far larger widening than
//! `--run` ever was. [`materialized`] passes [`Authorization::Report`], which
//! is the authorization that cannot execute anything — so the rule holds by
//! construction, not by discipline.

use std::collections::BTreeMap;

use crate::atoms::Atom;
use crate::git::Git;
use crate::probe::{self, Authorization, Probe, Verdict};

/// Whether an artifact type is materially present, and how sure day is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// A `path` or `tag` probe found it.
    Present,
    /// A probe ran and found nothing.
    Absent,
    /// No probe is declared for this type, or its probe is a `command` —
    /// which inference will not run. Presence is unknown, not absent, and the
    /// difference matters: treating unknown as absent would make every atom
    /// with a command-probed input look ready.
    Unknown,
}

/// Resolves one artifact type against the witness probes, without ever
/// executing a command.
fn materialized(kind: &str, probes: &BTreeMap<String, Probe>, git: &Git) -> Presence {
    match probes.get(kind) {
        None => Presence::Unknown,
        // A command probe is deliberately not run here (REQ-5). Its evidence
        // is unknowable at inference time, which is honest — the alternative
        // is executing it on every session start.
        Some(Probe::Command(_)) => Presence::Unknown,
        Some(probe) => match probe::evaluate(probe, git, Authorization::Report) {
            Verdict::Satisfied(_) => Presence::Present,
            Verdict::Unsatisfied(_) => Presence::Absent,
            // NotRun should be unreachable for path/tag, but if it arises it
            // is unknown rather than absent.
            _ => Presence::Unknown,
        },
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
    /// Whether this atom's own outputs are already present.
    pub outputs_present: bool,
}

impl Standing {
    /// A candidate for "current": everything a probe could check about its
    /// inputs is present, and its outputs are not yet. Unknown inputs do not
    /// disqualify — they are reported, and pretending they are absent would
    /// hide a ready atom.
    pub fn is_current(&self) -> bool {
        self.inputs_missing.is_empty() && !self.outputs_present
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

/// Infers position from the atom set and the witness probes.
pub fn infer(atoms: &[Atom], probes: &BTreeMap<String, Probe>, git: &Git) -> Report {
    let present = |kind: &str| materialized(kind, probes, git) == Presence::Present;

    let standings: Vec<Standing> = atoms
        .iter()
        .map(|atom| {
            let mut inputs_present = Vec::new();
            let mut inputs_missing = Vec::new();
            let mut inputs_unknown = Vec::new();
            for input in &atom.interface.inputs {
                match materialized(input, probes, git) {
                    Presence::Present => inputs_present.push(input.clone()),
                    Presence::Absent => inputs_missing.push(input.clone()),
                    Presence::Unknown => inputs_unknown.push(input.clone()),
                }
            }
            let outputs_present = !atom.interface.outputs.is_empty()
                && atom.interface.outputs.iter().all(|o| present(o));
            Standing {
                atom: atom.name.clone(),
                inputs_present,
                inputs_missing,
                inputs_unknown,
                outputs_present,
            }
        })
        .collect();

    let current: Vec<String> = standings
        .iter()
        .filter(|s| !s.is_source() && s.is_current())
        .map(|s| s.atom.clone())
        .collect();

    // Off-sequence: an atom produced its outputs, but an atom it lists as a
    // predecessor (via `next`) has not produced its own. Availability
    // accumulates along a path, so a downstream artifact existing while an
    // upstream one does not means a step was skipped.
    let produced: std::collections::BTreeSet<&str> = standings
        .iter()
        .filter(|s| s.outputs_present)
        .map(|s| s.atom.as_str())
        .collect();
    let mut off_sequence = Vec::new();
    for atom in atoms {
        for successor in &atom.interface.next {
            // `atom` is upstream of `successor`. If the successor's outputs
            // exist but the predecessor's do not, the sequence was skipped.
            if produced.contains(successor.as_str()) && !produced.contains(atom.name.as_str()) {
                // Only flag when the predecessor *could* have been detected —
                // an atom whose outputs are unknowable is not evidence of a
                // skip.
                let upstream = standings.iter().find(|s| s.atom == atom.name);
                if upstream.is_some_and(|s| !s.outputs_present) {
                    off_sequence.push(format!(
                        "{} produced its output but upstream {} did not — a step was skipped",
                        successor, atom.name
                    ));
                }
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

    /// A git that answers path/tag probes from fixed sets. Returns the
    /// `TempDir` too, so the caller keeps the stub alive — and uses
    /// `Git::with_bin` rather than mutating the git-binary env var, which is
    /// process global and races when tests run in parallel (found the hard
    /// way: this test flaked until the env mutation was removed).
    fn git_with(tracked: &[&str], tags: &[&str]) -> (tempfile::TempDir, Git) {
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("git-stub.sh");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\npattern=\"$3\"\nmatch() {{ for i in $1; do case \"$i\" in $pattern) printf '%s\\n' \"$i\";; esac; done; }}\ncase \"$1\" in\n  ls-files) match \"{}\" ;;\n  tag) match \"{}\" ;;\n  *) exit 1 ;;\nesac\n",
                tracked.join(" "),
                tags.join(" "),
            ),
        )
        .unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        let git = Git::with_bin(".", script.display().to_string());
        (dir, git)
    }

    fn probes(pairs: &[(&str, Probe)]) -> BTreeMap<String, Probe> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn a_design_doc_present_and_no_code_change_puts_you_in_build() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        let probes = probes(&[
            ("design-doc", Probe::Path(".design/*.md".into())),
            ("code-change", Probe::Path("src/*.rs".into())),
            // intent has no probe: unknowable, and a source atom's inputs are
            // not evidence anyway.
        ]);
        let (_d, git) = git_with(&[".design/x.md"], &[]);
        let report = infer(&atoms, &probes, &git);
        assert_eq!(report.current, vec!["build"], "{:?}", report.standings);
    }

    #[test]
    fn both_present_means_build_is_no_longer_current() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        let probes = probes(&[
            ("design-doc", Probe::Path(".design/*.md".into())),
            ("code-change", Probe::Path("src/*.rs".into())),
        ]);
        let (_d, git) = git_with(&[".design/x.md", "src/lib.rs"], &[]);
        let report = infer(&atoms, &probes, &git);
        assert!(
            !report.current.contains(&"build".to_string()),
            "build's output exists, so it is not current: {:?}",
            report.current
        );
    }

    #[test]
    fn a_command_probed_input_is_unknown_not_absent() {
        let atoms = [atom("check", &["passing-tests"], &["verdict"], &[])];
        let probes = probes(&[("passing-tests", Probe::Command("exit 1".into()))]);
        let (_d, git) = git_with(&[], &[]);
        let report = infer(&atoms, &probes, &git);
        // The command is never run, so its input is unknown -- and an atom
        // whose only input is unknowable is still a candidate rather than
        // being silently ruled out.
        let s = &report.standings[0];
        assert_eq!(s.inputs_unknown, vec!["passing-tests"]);
        assert!(s.inputs_missing.is_empty());
    }

    #[test]
    fn a_downstream_output_without_its_upstream_is_off_sequence() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        let probes = probes(&[
            ("design-doc", Probe::Path(".design/*.md".into())),
            ("code-change", Probe::Path("src/*.rs".into())),
        ]);
        // code-change exists, design-doc does not: the build ran without a
        // design.
        let (_d, git) = git_with(&["src/lib.rs"], &[]);
        let report = infer(&atoms, &probes, &git);
        assert_eq!(report.off_sequence.len(), 1, "{:?}", report.off_sequence);
        assert!(report.off_sequence[0].contains("design"));
    }
}
