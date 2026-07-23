//! `day status` and the status line — **two renderings of one computation**,
//! so they cannot drift.
//!
//! [`compute`] assembles where the work sits from three reads day already
//! has: the atom graph (`atom/<slug>` subjects), the witness probe map
//! (`schema/witness`), and git. It runs [`position::infer`], then for each
//! current atom evaluates its `done` criteria. [`Status::render_long`] is
//! `day status`; [`Status::render_line`] is the one-to-three-line form the
//! status line shows.
//!
//! **Nothing here executes a command probe.** Status is a *display*, and a
//! display that runs project-declared commands would be the same widening
//! position inference refuses — [`position::resolve`] holds the line by
//! construction, exactly as it does for inference. The gateable check that
//! *does* run commands (under `--run`) is `day assess atom`, which exists
//! precisely so status can stay safe to run on every keystroke.
//!
//! **Everything here is relative to the current cycle.** The boundary — the
//! last release — is computed once in [`compute`] and threaded through both
//! the position and the `done` criteria shown under it, so the whole display
//! answers one question ("where is *this* cycle") rather than mixing it with
//! "what has this repo ever produced". Assessment answers the other one.
//!
//! **Nothing here writes a claim.** Position is inferred and displayed, never
//! recorded — recording it would make day the task tracker
//! `docs/CONVENTIONS.md` refuses on purpose.

use std::collections::BTreeMap;

use crate::atoms::{self, Atom};
use crate::git::{Boundary, Git};
use crate::kan_client::KanClient;
use crate::position::{self, Standing};
use crate::probe::{ClaimLog, Verdict};
use crate::telos::WitnessSchema;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Telos(#[from] crate::telos::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
}

/// One `done` criterion of a current atom, resolved without running commands.
#[derive(Debug)]
pub struct Criterion {
    pub witness: String,
    /// `None` when no probe is declared for this witness type. A command
    /// probe resolves to [`Verdict::NotRun`] rather than executing.
    pub verdict: Option<Verdict>,
}

impl Criterion {
    /// Met only when a probe ran and found its evidence. Not-run (a command
    /// probe, un-executed here by design), no-probe, and error are all
    /// "unknown, go look" — never counted as met, the same rule assessment
    /// uses.
    pub fn is_met(&self) -> bool {
        matches!(self.verdict, Some(Verdict::Satisfied(_)))
    }
}

/// A current (or candidate) atom, with the detail `day status` reports.
#[derive(Debug)]
pub struct Here {
    pub atom: String,
    pub inputs_present: Vec<String>,
    pub inputs_unknown: Vec<String>,
    pub done: Vec<Criterion>,
    /// What the atom graph says composes after this one.
    pub next: Vec<String>,
}

impl Here {
    /// Criteria that ran and were satisfied, over criteria declared.
    fn done_counts(&self) -> (usize, usize) {
        (
            self.done.iter().filter(|c| c.is_met()).count(),
            self.done.len(),
        )
    }
}

/// The work has moved past the atom you last recorded assessing.
///
/// The baseline is **claims, not the cache** (REQ-10): the most recent
/// assessment (`kan result`) recorded on any `atom/<slug>` subject names the
/// atom you last checked. If the current inferred position no longer includes
/// it, the work has moved on since that assessment. day *reads* those claims
/// and never writes them — recording position itself would make day a task
/// tracker, and auto-writing the baseline would let the tool manufacture its
/// own evidence. The whole mechanism inherits claim semantics for free:
/// retract the assessment and the baseline is gone; a newer one supersedes it.
#[derive(Debug)]
pub struct Transition {
    /// The atom named by the last recorded assessment.
    pub from: String,
    /// Where the evidence now says the work sits.
    pub to: Vec<String>,
}

#[derive(Debug)]
pub struct Status {
    /// The atoms consistent with the current evidence. Empty, one, or many —
    /// day names them all rather than choosing, because guessing one would be
    /// a claim it cannot support.
    pub here: Vec<Here>,
    /// Off-sequence findings from [`position::infer`]: a downstream output is
    /// present while an upstream one is not, so a step was skipped.
    pub off_sequence: Vec<String>,
    /// Set when position has moved past the last recorded assessment. `None`
    /// when no atom assessment exists, or when the assessed atom is still
    /// current — absence of a baseline is not a change (REQ-10, AC-10).
    pub transition: Option<Transition>,
    /// True when no witness probes are declared, so position cannot be
    /// inferred at all — reported plainly rather than as "no current atom".
    pub uncheckable: bool,
}

impl Status {
    fn ambiguous(&self) -> bool {
        self.here.len() > 1
    }

    /// A one-line, human-facing notice of the *events* worth marking — a
    /// transition past the last-assessed atom, or a skipped step — or `None`
    /// when there is nothing to mark.
    ///
    /// This is what a `systemMessage` hook shows the human once per session: a
    /// transition is an event, and an event deserves marking rather than being
    /// something you catch by watching the status line change. Persistent state
    /// (the current atom) stays in the status line; this is only the delta.
    pub fn notice(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(t) = &self.transition {
            let to = if t.to.is_empty() {
                "no atom currently in play".to_string()
            } else {
                t.to.join(", ")
            };
            parts.push(format!(
                "day: since your last recorded assessment of `{}`, the work has moved to {to} \
                 — consider `day assess atom <slug>`",
                t.from
            ));
        }
        if let Some(first) = self.off_sequence.first() {
            parts.push(format!("day: possible skipped step — {first}"));
        }
        (!parts.is_empty()).then(|| parts.join("\n"))
    }

    /// `day status`: the full human report — current atom(s), satisfied and
    /// unknown inputs, met and unmet `done` criteria, what follows, and any
    /// off-sequence finding.
    pub fn render_long(&self) -> String {
        let mut out = String::from("day — process position\n\n");

        if self.uncheckable {
            out.push_str(
                "No witness probes are declared (`schema/witness`), so day cannot infer\n\
                 where the work sits from artifacts. Declare what would evidence each\n\
                 type and position becomes checkable:\n  \
                 kan observe \"...\" --subject schema/witness   (see docs/CONVENTIONS.md)\n",
            );
            return out;
        }

        match self.here.len() {
            0 => out.push_str(
                "No atom is currently in play: every declared atom either already has its\n\
                 outputs, or is still missing an input. Nothing to do inside the known\n\
                 vocabulary — which usually means the work is between milestones.\n",
            ),
            1 => out.push_str(&format!("Current atom: {}\n\n", self.here[0].atom)),
            n => out.push_str(&format!(
                "{n} atoms are consistent with the evidence — day names them all rather\n\
                 than guessing which one you are in:\n\n"
            )),
        }

        for here in &self.here {
            if self.ambiguous() {
                out.push_str(&format!("- {}\n", here.atom));
            }
            let indent = if self.ambiguous() { "  " } else { "" };
            if !here.inputs_present.is_empty() {
                out.push_str(&format!(
                    "{indent}inputs satisfied: {}\n",
                    here.inputs_present.join(", ")
                ));
            }
            if !here.inputs_unknown.is_empty() {
                out.push_str(&format!(
                    "{indent}inputs unknowable (no probe, or a command probe): {}\n",
                    here.inputs_unknown.join(", ")
                ));
            }
            if here.done.is_empty() {
                out.push_str(&format!(
                    "{indent}done criteria: none declared — completion cannot be checked\n"
                ));
            } else {
                let (met, total) = here.done_counts();
                out.push_str(&format!("{indent}done criteria ({met}/{total} met):\n"));
                for c in &here.done {
                    let (mark, detail) = match &c.verdict {
                        Some(Verdict::Satisfied(d)) => ("[met]", d.clone()),
                        Some(v) => (unmet_mark(v), v.detail().to_string()),
                        None => ("[no probe]", "no probe declared for this type".to_string()),
                    };
                    out.push_str(&format!("{indent}  {mark} {}: {detail}\n", c.witness));
                }
            }
            if here.next.is_empty() {
                out.push_str(&format!("{indent}next: nothing — this atom is a sink\n"));
            } else {
                out.push_str(&format!("{indent}next: {}\n", here.next.join(", ")));
            }
            out.push('\n');
        }

        if let Some(t) = &self.transition {
            out.push_str(&format!(
                "Since your last recorded assessment of `{}`, the work has moved on.\n  \
                 now: {}\n  \
                 Consider assessing where it sits now: day assess atom <slug>\n\n",
                t.from,
                if t.to.is_empty() {
                    "no atom currently in play".to_string()
                } else {
                    t.to.join(", ")
                }
            ));
        }

        if !self.off_sequence.is_empty() {
            out.push_str("Off-sequence:\n");
            for finding in &self.off_sequence {
                out.push_str(&format!("  ! {finding}\n"));
            }
            out.push('\n');
        }

        out.push_str(
            "Position is inferred from artifacts, not tracked — it is recomputed each\n\
             time and nothing is recorded. To gate an atom's completion in CI:\n  \
             day assess atom <slug>\n",
        );
        out
    }

    /// The status line: the same state in one to three lines, terse enough to
    /// sit in a status bar. Rendered at session start and cached; the status
    /// line reads the cache, never this.
    pub fn render_line(&self) -> String {
        let mut lines = Vec::new();

        if self.uncheckable {
            lines.push("day · no witness probes declared".to_string());
        } else {
            match self.here.as_slice() {
                [] => lines.push("day · no current atom".to_string()),
                [here] => {
                    let mut parts = vec![format!("day · {}", here.atom)];
                    let (met, total) = here.done_counts();
                    if total > 0 {
                        parts.push(format!("{met}/{total} done"));
                    }
                    if let Some(next) = here.next.first() {
                        let more = if here.next.len() > 1 { "…" } else { "" };
                        parts.push(format!("next: {next}{more}"));
                    }
                    lines.push(parts.join(" · "));
                }
                many => {
                    let names: Vec<&str> = many.iter().map(|h| h.atom.as_str()).collect();
                    lines.push(format!("day · candidates: {}", names.join(", ")));
                }
            }
        }

        // A transition is an event, and an event earns its own line: "you've
        // moved past what you last assessed" is the nudge the whole baseline
        // machinery exists to deliver.
        if let Some(t) = &self.transition {
            lines.push(format!("day ⤳ moved past assessed `{}`", t.from));
        }

        // Off-sequence is a warning, and worth its own line even in the terse
        // form — a skipped step is exactly what a person scanning a status bar
        // should catch.
        if let Some(first) = self.off_sequence.first() {
            lines.push(format!("day ! {first}"));
        }

        lines.join("\n")
    }
}

/// The mark for a `done` criterion whose probe did not confirm it. A
/// genuinely-missing artifact reads differently from one that was merely not
/// checked, so the two do not collapse into a single "unmet".
fn unmet_mark(verdict: &Verdict) -> &'static str {
    match verdict {
        Verdict::Unsatisfied(_) => "[unmet]",
        Verdict::NotRun(_) => "[not run]",
        Verdict::TimedOut(_) => "[timeout]",
        Verdict::Error(_) => "[error]",
        Verdict::Satisfied(_) => "[met]",
    }
}

/// Assembles the status from kan and git. Reads only — appends nothing, runs
/// no command probe.
pub fn compute(client: &KanClient, git: &Git) -> Result<Status, Error> {
    let (atoms, _findings) = atoms::load(client)?;
    // A missing witness schema is not an error here: it means position is
    // uncheckable, which the report says plainly. `assess` needs the schema
    // and errors without it; `status` degrades to "cannot infer".
    let schema = match WitnessSchema::load(client) {
        Ok(schema) => schema,
        Err(crate::telos::Error::NotDeclared { .. }) => WitnessSchema::default(),
        Err(e) => return Err(e.into()),
    };

    if schema.probes.is_empty() && schema.unsupported.is_empty() {
        return Ok(Status {
            here: Vec::new(),
            off_sequence: Vec::new(),
            transition: None,
            uncheckable: true,
        });
    }

    // The cycle boundary, computed once and threaded through. A git read that
    // fails leaves it `None`, which is the same state a repo with no release
    // is in — position falls back to its cumulative reading rather than
    // failing, because "where am I" degrading is better than not answering.
    let boundary = git.cycle_boundary().unwrap_or(None);

    // One read of the log, shared by every claim probe below.
    let log = ClaimLog::new(client);

    let report = position::infer(&atoms, &schema.probes, git, &log, boundary.as_ref());
    let by_name: BTreeMap<&str, &Atom> = atoms.iter().map(|a| (a.name.as_str(), a)).collect();

    let here: Vec<Here> = report
        .current
        .iter()
        .filter_map(|name| {
            let atom = by_name.get(name.as_str())?;
            let standing = report.standings.iter().find(|s| &s.atom == name)?;
            Some(here_for(atom, standing, &schema, git, &log, boundary.as_ref()))
        })
        .collect();

    // Transition: the last recorded assessment names an atom the work has
    // since moved past. Read from claims, never written.
    let transition = match last_assessed_atom(client, &atoms)? {
        Some(from) if !report.current.contains(&from) => Some(Transition {
            from,
            to: report.current.clone(),
        }),
        _ => None,
    };

    Ok(Status {
        here,
        off_sequence: report.off_sequence,
        transition,
        uncheckable: false,
    })
}

/// The atom named by the most recent assessment (`kan result`) recorded on any
/// `atom/<slug>` subject, or `None` if none has ever been assessed.
///
/// "Most recent" is a global order across atom subjects, which is why this
/// reads `recorded_at`: `show` returns each subject oldest-first, but the
/// baseline is *the* last assessment regardless of which atom it was on.
/// Claims kan returns are already live (retracted ones are gone), so a
/// retracted assessment simply stops being the baseline — AC-14 for free.
fn last_assessed_atom(client: &KanClient, atoms: &[Atom]) -> Result<Option<String>, Error> {
    let mut best: Option<(i64, String)> = None; // (recorded_at µs, atom slug)
    for atom in atoms {
        for claim in client.show(&atom.subject())? {
            if claim.kind != "Result" {
                continue;
            }
            // A claim with no timestamp sorts as 0 — oldest — so a real,
            // timestamped assessment always outranks it rather than an undated
            // one winning by accident.
            let at = claim.recorded_at.unwrap_or(0);
            if best.as_ref().is_none_or(|(best_at, _)| at >= *best_at) {
                best = Some((at, atom.name.clone()));
            }
        }
    }
    Ok(best.map(|(_, slug)| slug))
}

fn here_for(
    atom: &Atom,
    standing: &Standing,
    schema: &WitnessSchema,
    git: &Git,
    log: &ClaimLog<'_>,
    boundary: Option<&Boundary>,
) -> Here {
    let done = atom
        .interface
        .done
        .iter()
        .map(|witness| Criterion {
            witness: witness.clone(),
            // Resolved the same way the position above it was: cycle-relative
            // and never executing a command. A `done` criterion met by last
            // cycle's artifact is exactly the stale reading day#60 is about,
            // and it would be incoherent for the criteria under an atom to
            // answer a different question than the atom's own standing.
            // Status displays; it does not run the build.
            verdict: schema
                .probes
                .get(witness)
                .map(|p| position::resolve(p, git, log, boundary))
                .or_else(|| schema.unreadable(witness)),
        })
        .collect();

    Here {
        atom: atom.name.clone(),
        inputs_present: standing.inputs_present.clone(),
        inputs_unknown: standing.inputs_unknown.clone(),
        done,
        next: atom.interface.next.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn here(atom: &str, done: Vec<Criterion>, next: &[&str]) -> Here {
        Here {
            atom: atom.to_string(),
            inputs_present: vec![],
            inputs_unknown: vec![],
            done,
            next: next.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn met(witness: &str) -> Criterion {
        Criterion {
            witness: witness.to_string(),
            verdict: Some(Verdict::Satisfied("found".into())),
        }
    }

    fn unmet(witness: &str) -> Criterion {
        Criterion {
            witness: witness.to_string(),
            verdict: Some(Verdict::Unsatisfied("missing".into())),
        }
    }

    #[test]
    fn a_single_current_atom_names_it_and_what_follows() {
        let status = Status {
            here: vec![here(
                "build",
                vec![met("tests"), unmet("docs")],
                &["review"],
            )],
            off_sequence: vec![],
            transition: None,
            uncheckable: false,
        };
        let long = status.render_long();
        assert!(long.contains("Current atom: build"), "{long}");
        assert!(long.contains("[met] tests"), "{long}");
        assert!(long.contains("[unmet] docs"), "{long}");
        assert!(long.contains("next: review"), "{long}");

        let line = status.render_line();
        assert_eq!(line, "day · build · 1/2 done · next: review");
    }

    #[test]
    fn several_candidates_are_all_named_and_none_chosen() {
        let status = Status {
            here: vec![here("design", vec![], &[]), here("build", vec![], &[])],
            off_sequence: vec![],
            transition: None,
            uncheckable: false,
        };
        let long = status.render_long();
        assert!(long.contains("2 atoms are consistent"), "{long}");
        assert!(long.contains("- design"), "{long}");
        assert!(long.contains("- build"), "{long}");

        let line = status.render_line();
        assert_eq!(line, "day · candidates: design, build");
    }

    #[test]
    fn no_current_atom_says_so_in_both_forms() {
        let status = Status {
            here: vec![],
            off_sequence: vec![],
            transition: None,
            uncheckable: false,
        };
        assert!(status
            .render_long()
            .contains("No atom is currently in play"));
        assert_eq!(status.render_line(), "day · no current atom");
    }

    #[test]
    fn no_probes_is_reported_as_uncheckable_not_as_no_atom() {
        let status = Status {
            here: vec![],
            off_sequence: vec![],
            transition: None,
            uncheckable: true,
        };
        assert!(status
            .render_long()
            .contains("No witness probes are declared"));
        assert_eq!(status.render_line(), "day · no witness probes declared");
    }

    /// Off-sequence is a warning and gets its own line even in the terse form:
    /// a skipped step is exactly what a status-bar glance should catch.
    #[test]
    fn off_sequence_surfaces_in_both_forms() {
        let status = Status {
            here: vec![here("build", vec![], &["review"])],
            off_sequence: vec!["review produced its output but upstream build did not".into()],
            transition: None,
            uncheckable: false,
        };
        assert!(status.render_long().contains("Off-sequence:"));
        let line = status.render_line();
        assert!(line.lines().count() == 2, "{line}");
        assert!(line.lines().nth(1).unwrap().starts_with("day ! "), "{line}");
    }

    /// A transition renders in both forms: the long report nudges toward a
    /// fresh assessment, the terse line marks the event.
    #[test]
    fn a_transition_surfaces_in_both_forms() {
        let status = Status {
            here: vec![here("review", vec![], &[])],
            off_sequence: vec![],
            transition: Some(Transition {
                from: "build".into(),
                to: vec!["review".into()],
            }),
            uncheckable: false,
        };
        let long = status.render_long();
        assert!(
            long.contains("last recorded assessment of `build`"),
            "{long}"
        );
        assert!(long.contains("now: review"), "{long}");

        let line = status.render_line();
        assert!(
            line.lines()
                .any(|l| l.contains("moved past assessed `build`")),
            "{line}"
        );
    }

    /// The human notice marks events — a transition, a skipped step — and is
    /// silent when there is nothing to mark.
    #[test]
    fn notice_marks_events_and_is_silent_otherwise() {
        // Nothing to mark.
        let quiet = Status {
            here: vec![here("build", vec![], &["review"])],
            off_sequence: vec![],
            transition: None,
            uncheckable: false,
        };
        assert_eq!(quiet.notice(), None);

        // A transition and an off-sequence both surface.
        let loud = Status {
            here: vec![here("review", vec![], &[])],
            off_sequence: vec!["build produced its output but upstream design did not".into()],
            transition: Some(Transition {
                from: "build".into(),
                to: vec!["review".into()],
            }),
            uncheckable: false,
        };
        let notice = loud.notice().expect("there is something to mark");
        assert!(notice.contains("`build`"), "{notice}");
        assert!(notice.contains("moved to review"), "{notice}");
        assert!(notice.contains("skipped step"), "{notice}");
    }

    /// A not-run command probe is neither met nor "unmet": the evidence is
    /// unknown, and collapsing it into a failure would misreport a criterion
    /// that status deliberately never checks.
    #[test]
    fn a_not_run_command_probe_is_not_counted_as_met_nor_shown_as_unmet() {
        let c = Criterion {
            witness: "passing-tests".into(),
            verdict: Some(Verdict::NotRun("would run `cargo test`".into())),
        };
        assert!(!c.is_met());
        let status = Status {
            here: vec![here("build", vec![c], &[])],
            off_sequence: vec![],
            transition: None,
            uncheckable: false,
        };
        let long = status.render_long();
        assert!(long.contains("[not run] passing-tests"), "{long}");
        assert!(long.contains("0/1 met"), "{long}");
    }
}
