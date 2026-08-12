//! `day status` and the status line — **two renderings of one computation**,
//! so they cannot drift.
//!
//! [`compute`] assembles where the work sits from three reads day already
//! has: the atom graph (`atom/<slug>` subjects), the witness probe map
//! (`schema/witness`), and git. It runs [`position::infer`], then for each
//! current atom evaluates its `done` criteria. [`Status::render_long`] is
//! `day status`; [`Status::render_line`] is the terse form injected into the
//! **model's** session-start context.
//!
//! **`render_line` is no longer what the status line shows** — `src/footer.rs`
//! is, since day#179. The two renderings were forked rather than swapped, and
//! this comment claimed the old arrangement for a while afterwards, which
//! matters here more than usual: the tests under `render_line` pin strings a
//! human never sees now, and a reader trusting this sentence would have
//! changed the wrong one. Their audiences differ (a model reading injected
//! context, a person glancing at a bar), which is why both survive.
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

/// What `day status` prints when it could not read enough to report a position
/// at all (day#95).
///
/// Its whole reason for existing is that the caller **exits zero anyway**. The
/// verb documents "always exits zero", and a report that can fail a step is not
/// advisory — so the failure has to be renderable rather than propagated. It is
/// worded like `day hook session-start`'s degradation for the same state,
/// because it *is* the same state and a reader meeting it in two places should
/// not have to work out that they match.
///
/// Not a silent empty report: the error is printed, and the reader is pointed
/// at the verb whose job is to diagnose. "day could not read this" and "there
/// is nothing here" must not be spelled the same way.
pub fn render_unreadable(error: &Error) -> String {
    format!(
        "day — process position\n\n\
         kan is installed but its log could not be read here ({error}).\n\
         If this repo isn't tracked by kan yet, that's expected.\n\n\
         No position is reported, which is not the same as no work in progress —\n\
         day could not look. To diagnose:\n  day doctor\n"
    )
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
    /// Orders [`position::infer`] could not establish, because the atoms are on
    /// a cycle through `next` (day#113). See [`position::Report::unordered`].
    pub unordered: Vec<String>,
    /// Artifact types that exist but are not written down (day#103): a declared
    /// `material` witness is satisfied for this cycle and the declared `record`
    /// witness is not.
    ///
    /// Rendered separately from everything else here because it is a distinct
    /// claim: not "this is next", not "a step was skipped", but "this happened
    /// and the log does not know". Collapsing it into either of those is the
    /// defect it exists to fix.
    pub unrecorded: Vec<String>,
    /// day#103 — the cycle-closing tag exists and the log does not record it.
    ///
    /// Computed here rather than only in `day assess docs`, so every channel
    /// that reports a position inherits it. Two consecutive releases shipped
    /// unrecorded because the only detector was a manual verb downstream of the
    /// step that was skipped.
    pub unrecorded_boundary: Option<String>,
    /// Set when position has moved past the last recorded assessment. `None`
    /// when no atom assessment exists, or when the assessed atom is still
    /// current — absence of a baseline is not a change (REQ-10, AC-10).
    pub transition: Option<Transition>,
    /// True when no witness probes are declared, so position cannot be
    /// inferred at all — reported plainly rather than as "no current atom".
    pub uncheckable: bool,
    /// The declared injection cadence (`schema/injection`), resolved here
    /// because this is where day already reads declarations and already reports
    /// the ones it could not read. Resolving it in the hook instead meant an
    /// unreadable declaration silently became the default — the same defect as
    /// day#81, on a value nobody would have noticed was wrong.
    pub cadence: u32,
    /// Declarations this build could not read, so every field above is
    /// **partial** and a reader must be told rather than left to assume.
    ///
    /// `compute` used to discard these: it called `atoms::load` and threw the
    /// findings away, so an unreadable atom reached neither the status line nor
    /// the human notice. Position was computed over a vocabulary day knew was
    /// incomplete, and said so nowhere.
    pub unreadable: Vec<Unreadable>,
}

/// Why day could not read something, and therefore **who has to do what**.
///
/// A bool (`version_skew`) held this until a third cause arrived: a subject kan
/// listed but did not return, which is neither a stale reader nor a bad claim.
/// It rendered as "the blocks are malformed — the claims need fixing", and
/// fixing the claim would have done nothing. A two-valued classifier silently
/// absorbing a third case is how a reader gets pointed the wrong way; an enum
/// makes the next cause a compile error instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// This day is behind the log. The reader upgrades day.
    VersionSkew,
    /// The claim is malformed. Someone fixes the claim.
    Malformed,
    /// kan listed the subject and did not return it. Neither of the above will
    /// help, and day cannot tell which it is (kan#143).
    Unaccounted,
}

impl Cause {
    fn is_skew(self) -> bool {
        matches!(self, Cause::VersionSkew)
    }
}

/// One declaration this build could not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreadable {
    pub message: String,
    /// Whether the *reader* is behind the log, rather than the log being wrong.
    /// Decides which of two different actions, for two different people, the
    /// human notice asks for.
    pub cause: Cause,
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
        // The third event type (`.design/honest-reads.md` REQ-5), and the one
        // whose *cause* decides the message. Version skew is the reader's
        // problem and is fixed by upgrading; a malformed block is the claim's
        // and is fixed by editing it. Telling someone to upgrade over a typo,
        // or to edit a claim that is fine, are both worse than saying nothing —
        // which is day#60's lesson, where the v0.6 binary failed loudly and
        // pointed the reader the wrong way.
        if !self.unreadable.is_empty() {
            let n = self.unreadable.len();
            let plural = if n == 1 {
                "declaration"
            } else {
                "declarations"
            };
            // Ordered so the honest answer wins: an unaccounted subject is
            // something NEITHER the reader nor the claim's author can fix, so
            // it must not be folded into either remedy.
            let fix = if self
                .unreadable
                .iter()
                .any(|u| u.cause == Cause::Unaccounted)
            {
                "kan did not return everything it listed — day cannot tell what is \
                 in those subjects, so treat this report as incomplete"
            } else if self.unreadable.iter().all(|u| u.cause.is_skew()) {
                "this day is older than the log — upgrading day should read them"
            } else if self.unreadable.iter().any(|u| u.cause.is_skew()) {
                "some need a newer day, others need the claim fixed"
            } else {
                "the blocks are malformed — the claims need fixing"
            };
            parts.push(format!(
                "day: {n} {plural} could not be read, so what day reported is partial: \
                 {fix}. `day doctor` for detail."
            ));
        }
        (!parts.is_empty()).then(|| parts.join("\n"))
    }

    /// The **event** half of [`Self::notice`], for the model's mid-session
    /// channel rather than the human's.
    ///
    /// Split from `notice` because the two audiences need different things from
    /// the same state: the human gets an actionable instruction (upgrade day, fix
    /// the claim), the model gets an epistemic qualifier on what day told it.
    /// Sharing one string would have meant telling the model to run a command it
    /// cannot run, and telling the human something only a reasoner needs.
    pub fn notice_for_model(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(t) = &self.transition {
            let to = if t.to.is_empty() {
                "no atom currently in play".to_string()
            } else {
                t.to.join(", ")
            };
            parts.push(format!(
                "day: the work has moved past `{}`, the last atom whose assessment was \
                 recorded — it now sits at {to}.",
                t.from
            ));
        }
        if let Some(first) = self.off_sequence.first() {
            parts.push(format!(
                "day: a step may have been skipped — {first}. Advisory; nothing is blocked."
            ));
        }
        (!parts.is_empty()).then(|| parts.join("\n"))
    }

    /// The **standing** half of what the model should hear: things that are
    /// true until somebody records something, as opposed to events.
    ///
    /// Separate from [`Self::notice_for_model`] because the two are rationed
    /// differently, and `hooks.rs` is where that distinction lives. A transition
    /// is an event and fires once; "this exists and the log does not mention it"
    /// stays true for as long as nobody acts, so repeating it every prompt is
    /// day#30's failure — an always-present rule becomes background, and then
    /// the real one is invisible too.
    ///
    /// day#103 wanted this on the model channel at all, and that part is right:
    /// the record is cheapest to repair in the session that broke it, and an
    /// hour later the boundary check is archaeology. Rationed, not silent.
    pub fn standing_notice(&self) -> Option<String> {
        let mut parts = Vec::new();
        if let Some(finding) = &self.unrecorded_boundary {
            parts.push(format!(
                "day: {finding}. Recording it is an append, not a correction. \
                 Advisory; nothing is blocked."
            ));
        }
        for kind in &self.unrecorded {
            parts.push(format!(
                "day: `{kind}` exists in this cycle, but its declared record witness \
                 finds nothing — the work happened and the log does not say so. \
                 Advisory; nothing is blocked."
            ));
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

        // One section, whichever findings fired. Two blocks each printing the
        // same header rendered it twice when both were live (F8).
        if self.unrecorded_boundary.is_some() || !self.unrecorded.is_empty() {
            out.push_str("Done but unrecorded:\n");
            if let Some(finding) = &self.unrecorded_boundary {
                out.push_str(&format!("  ! {finding}\n"));
            }
            for kind in &self.unrecorded {
                out.push_str(&format!(
                    "  ! {kind} exists in this cycle, but its declared record witness \
                     finds nothing —\n    the work happened and the log does not say so\n"
                ));
            }
            out.push('\n');
        }

        if !self.off_sequence.is_empty() || !self.unordered.is_empty() {
            out.push_str("Off-sequence:\n");
            for finding in &self.off_sequence {
                out.push_str(&format!("  ! {finding}\n"));
            }
            // `?`, not `!`: a check day could not run is not a finding it made.
            // Rendered even when `off_sequence` is empty, because "nothing to
            // report" and "could not look" are the two things this section
            // exists to keep apart.
            for finding in &self.unordered {
                out.push_str(&format!("  ? {finding}\n"));
            }
            out.push('\n');
        }

        // **Could-not-read, in the channel a human actually runs.**
        //
        // This section renders `off_sequence` findings and `unordered`
        // could-not-checks, and rendered `unrecorded` above — but `unreadable`
        // reached only `render_notice`. So `day status` showed everything day
        // *found* and nothing day *could not look at*, which is the inversion
        // `telos/honest-reads` forbids, at the rendering layer rather than in a
        // computation. Found by an unanswerable correspondence having nowhere
        // to appear.
        //
        // `?` rather than `!`, matching `unordered` directly above: a check day
        // could not run is not a finding it made.
        if !self.unreadable.is_empty() {
            out.push_str("Could not be read, so this report is partial:\n");
            for u in &self.unreadable {
                out.push_str(&format!("  ? {}\n", u.message));
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

        // day#108: the bar is read hundreds of times a session, always
        // peripherally, never with `--help` at hand. It gets ~40 characters and
        // used to spend them on `candidates:` — a word whose referent it never
        // stated. Read cold, `candidates: generative-build, release` looks like
        // a list of failures, or a menu.
        //
        // So the domain is named (`atom`), and the separator carries the state:
        // `atom:` is one day inferred, `atom?` is several the evidence does not
        // distinguish. That plurality is a deliberate property — day names them
        // all rather than guessing — and it now reads as one instead of as a
        // complaint.
        if self.uncheckable {
            // A setup step, not a diagnostic. This is what a fresh repo shows,
            // and "no witness probes declared" reads as an error about the
            // *work*. `day doctor` gets this right for the empty-vocabulary
            // case — "a valid starting state, not an error" — and the bar was
            // the one surface that did not.
            //
            // It names `schema/witness`, NOT `day init`. day#108 suggested
            // `not set up (day init)`, and that would send a reader to a verb
            // which records a `schema/design-doc` starter and no witnesses at
            // all — a remedy that does not remedy this. The long form carries
            // the full `kan observe` invocation.
            lines.push("day · setup: declare schema/witness".to_string());
        } else {
            match self.here.as_slice() {
                [] => lines.push("day · no atom in play".to_string()),
                [here] => {
                    let mut parts = vec![format!("day · atom: {}", here.atom)];
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
                    // Named, not counted. `atom? 2` would be shorter and the
                    // names are the actionable part — knowing you are in
                    // *either* build or release tells you what to do next;
                    // knowing there are two does not. `|` reads as "or", which
                    // is what ambiguity means here.
                    let names: Vec<&str> = many.iter().map(|h| h.atom.as_str()).collect();
                    lines.push(format!("day · atom? {}", names.join(" | ")));
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
        // F2: the done-but-unrecorded findings reach the bar. It is persistent,
        // it already carries `day ! ` findings, and a STANDING condition — one
        // that stays true until someone records something — is exactly what a
        // persistent surface is for. The model channel rations these instead
        // (see `notice_for_model`), because repeating a standing condition every
        // prompt is day#30's failure.
        if let Some(finding) = &self.unrecorded_boundary {
            lines.push(format!("day ! {finding}"));
        }
        if let Some(kind) = self.unrecorded.first() {
            let more = match self.unrecorded.len() {
                1 => String::new(),
                n => format!(" (+{} more)", n - 1),
            };
            lines.push(format!("day ! {kind} exists but is not recorded{more}"));
        }

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
        // Answered, and the answer establishes nothing — a universal with
        // nothing to quantify over. It must not read as `[unmet]`, which would
        // be a finding day did not make, nor as `[met]`, which is the whole
        // point of having the verdict at all.
        Verdict::Vacuous(_) => "[vacuous]",
        Verdict::Satisfied(_) => "[met]",
    }
}

/// Assembles the status from kan and git. Reads only — appends nothing, runs
/// no command probe.
pub fn compute(client: &KanClient, git: &Git) -> Result<Status, Error> {
    let (atoms, findings) = atoms::load(client)?;
    // Declared block schemas. An ABSENT declaration is the common case and not
    // an error; a declaration day could not READ is, and the difference is the
    // whole subject of `v0.7.0-beta.2`.
    //
    // This was `.unwrap_or_default()` for about an hour, which turned "day could
    // not read your block schemas" into "you have none" — the fourth instance of
    // that pattern in this codebase (day#81 in docs.rs, `render_teloi`, and
    // `compute` discarding `atoms::load`'s findings were the others), written
    // *after* CLAUDE.md gained a rule naming it. A rule in prose is not a
    // constraint.
    // The declared cadence, resolved with the other declarations. An ABSENT
    // declaration is day's default; one that could not be READ is reported.
    let (cadence, cadence_unreadable) = match crate::blocks::InjectionSchema::load(client) {
        Ok(i) => (i.cadence, None),
        Err(e) => (
            crate::cache::DEFAULT_CADENCE,
            Some(Unreadable {
                message: format!("injection settings could not be read: {e}"),
                cause: Cause::Malformed,
            }),
        ),
    };

    // What ends a cycle is declared (day#76); absent, it is a release. An
    // unreadable declaration is reported rather than silently falling back —
    // silently reverting to release semantics on a repo whose cycles are passes
    // would report position confidently and wrongly.
    let (cycle, cycle_unreadable) = match crate::blocks::CycleSchema::load(client) {
        Ok(c) => (c, None),
        Err(e) => (
            crate::blocks::CycleSchema::default(),
            Some(Unreadable {
                message: format!("cycle declaration could not be read: {e}"),
                cause: Cause::Malformed,
            }),
        ),
    };

    // day#103 — the boundary check, asked HERE rather than only in `assess
    // docs`. `status::compute` is the one place position is computed for every
    // channel, so wiring it here means the hooks, the status line and the long
    // form all inherit it and no channel can be added later that omits it. That
    // is day#101's rule: a guarantee belongs at the mechanism, not at a caller.
    //
    // An error is reported as unreadable, never dropped. A log day could not
    // read is not a boundary that is fine.
    let (unrecorded_boundary, boundary_unreadable) =
        match crate::docs::unrecorded_boundary(client, git) {
            Ok(finding) => (finding, None),
            Err(e) => (
                None,
                Some(Unreadable {
                    message: format!("the release boundary could not be reconciled: {e}"),
                    cause: Cause::Malformed,
                }),
            ),
        };

    let (blocks, blocks_unreadable) = match crate::blocks::BlockSchemas::load(client) {
        Ok(blocks) => (blocks, None),
        Err(e) => (
            crate::blocks::BlockSchemas::default(),
            Some(Unreadable {
                message: format!("block schemas could not be read: {e}"),
                // A schema day cannot parse at all is the claim's problem unless
                // it says otherwise; `BlockSchemas::validate` reports a reserved
                // name this way, and that is the project's to fix.
                cause: Cause::Malformed,
            }),
        ),
    };
    // A missing witness schema is not an error here: it means position is
    // uncheckable, which the report says plainly. `assess` needs the schema
    // and errors without it; `status` degrades to "cannot infer".
    //
    // fallback: uncheckable-without-witness-schema
    let schema = match WitnessSchema::load(client) {
        Ok(schema) => schema,
        Err(crate::telos::Error::NotDeclared { .. }) => WitnessSchema::default(),
        Err(e) => return Err(e.into()),
    };

    // Readable probes, not declared ones. A schema whose every entry is a
    // kind this version cannot read leaves inference with nothing to resolve,
    // and proceeding would report every atom as a candidate — noise dressed
    // as an answer. "Uncheckable" is the honest reading, and it is the same
    // one an empty schema gets.
    if schema.probes.is_empty() {
        return Ok(Status {
            here: Vec::new(),
            off_sequence: Vec::new(),
            // Reported even here. A cycle in `next` is a fact about the
            // declaration, not about the evidence, so "day cannot infer a
            // position" is no reason to also go quiet about an order day
            // could not establish — two independent gaps, and one must not
            // swallow the other.
            unordered: crate::position::unordered(&atoms),
            unrecorded: Vec::new(),
            unrecorded_boundary: unrecorded_boundary.clone(),
            transition: None,
            uncheckable: true,
            cadence,
            unreadable: unreadable_from(
                &findings,
                &schema,
                &blocks,
                [
                    blocks_unreadable.clone(),
                    cadence_unreadable.clone(),
                    cycle_unreadable.clone(),
                    boundary_unreadable.clone(),
                ],
                // Inference has not run, so it cannot have failed a read.
                &[],
                &client.unaccounted_subjects(),
            ),
        });
    }

    // The cycle boundary, computed once and threaded through. A git read that
    // fails leaves it `None`, which is the same state a repo with no release
    // is in — position falls back to its cumulative reading rather than
    // failing, because "where am I" degrading is better than not answering.
    // A git read that fails leaves it `None`, which is the same state a repo
    // with no boundary is in — position falls back to its cumulative reading
    // rather than failing.
    //
    // fallback: no-release-boundary
    let boundary = git.cycle_boundary_matching(&cycle.tags).unwrap_or(None);

    // One read of the log, shared by every claim probe below.
    let log = ClaimLog::new(client);

    let report = position::infer(&atoms, &schema, git, &log, boundary.as_ref());
    let forward = crate::atoms::Forward::build(&atoms);
    let by_name: BTreeMap<&str, &Atom> = atoms.iter().map(|a| (a.name.as_str(), a)).collect();

    let here: Vec<Here> = report
        .current
        .iter()
        .filter_map(|name| {
            let atom = by_name.get(name.as_str())?;
            let standing = report.standings.iter().find(|s| &s.atom == name)?;
            Some(here_for(
                atom,
                standing,
                &schema,
                git,
                &log,
                boundary.as_ref(),
                &forward,
            ))
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
        unordered: report.unordered,
        unrecorded: report.unrecorded,
        unrecorded_boundary,
        transition,
        uncheckable: false,
        cadence,
        unreadable: unreadable_from(
            &findings,
            &schema,
            &blocks,
            [
                blocks_unreadable.clone(),
                cadence_unreadable.clone(),
                cycle_unreadable.clone(),
                boundary_unreadable.clone(),
            ],
            &report.read_failures,
            &client.unaccounted_subjects(),
        ),
    })
}

/// Collects the declarations this build could not read, from both places they
/// surface: an atom whose block would not parse, and a witness probe of a kind
/// this build does not know.
///
/// Deliberately **not** every composition finding. A dangling `next` edge is
/// day reporting something it *did* read and found wrong; only a declaration it
/// could not read at all makes the rest of the report partial, and conflating
/// them would make the "treat this as partial" caveat fire on states that are
/// fully known.
fn unreadable_from(
    findings: &[atoms::Finding],
    schema: &WitnessSchema,
    blocks: &crate::blocks::BlockSchemas,
    // Failures to READ a declaration, as opposed to findings within one that was
    // read. They lead the list because "day could not read your declaration"
    // outranks anything day found inside the declarations it could.
    declaration_errors: [Option<Unreadable>; 4],
    // Instances position inference read and could not check
    // (`.design/declared-blocks.md` REQ-4). Distinct from the declaration
    // errors above: the project's *declaration* was fine and a *claim carrying
    // one* is from a newer day.
    read_failures: &[crate::probe::ReadFailure],
    // Subjects kan listed that the bulk read did not return (day#71).
    unaccounted: &[String],
) -> Vec<Unreadable> {
    let mut out: Vec<Unreadable> = declaration_errors.into_iter().flatten().collect();
    out.extend(
        findings
            .iter()
            // `f.unreadable`, not a substring of `f.message`. This filtered on
            // `contains("could not be read")` and broke the moment day#20 added a
            // second unreadable wording: `BlockError::Invalid` renders "is not a
            // valid …", so a structurally-empty plan node passed the filter and
            // reached neither hook channel. The typed flag exists precisely so a
            // caller never decides this by matching prose — a rule this function was
            // violating two definitions after the flag that states it.
            .filter(|f| f.unreadable)
            .map(|f| Unreadable {
                message: f.message.clone(),
                cause: if f.version_skew {
                    Cause::VersionSkew
                } else {
                    Cause::Malformed
                },
            }),
    );
    // A project-declared block schema this build could not read (day#74). Same
    // treatment as an unreadable witness probe, because it is the same
    // situation: the project declared vocabulary and day is only partly able to
    // act on it. Leaving the declarable path unreported while day's own seven
    // are reported would be the inconsistency day#78 was about.
    // A subject kan listed but the bulk read did not return. day cannot tell
    // whether it was unreadable or dropped, and must not treat it as absent —
    // one bulk read is day's entire view of the log, so an unaccounted subject
    // makes every answer built on it partial.
    out.extend(unaccounted.iter().map(|subject| Unreadable {
        message: format!(
            "kan lists `{subject}` but did not return it in the bulk read, so \
             anything day concluded about it is unverified"
        ),
        cause: Cause::Unaccounted,
    }));
    // Position inference reduces a verdict to a `Presence`, so an instance it
    // could not check became `Presence::Unknown` and the reason was dropped on
    // the floor. day then reported a position built on a partial read without
    // saying so — which is exactly what `telos/honest-reads` forbids, on the
    // path a project actually hits at session start.
    out.extend(read_failures.iter().map(|f| Unreadable {
        message: f.message.clone(),
        cause: if f.version_skew {
            Cause::VersionSkew
        } else {
            Cause::Malformed
        },
    }));
    for (name, reason) in &blocks.unsupported {
        out.push(Unreadable {
            message: format!("block schema `{name}`: {reason}"),
            cause: Cause::VersionSkew,
        });
    }
    for (witness, reason) in &schema.unsupported {
        out.push(Unreadable {
            message: format!("witness `{witness}`: {reason}"),
            // An unreadable probe *kind* is the same situation as a too-new
            // block — this build is behind what the project declared — so it
            // asks for the same action.
            cause: Cause::VersionSkew,
        });
    }
    out
}

/// The atom named by the most recent assessment (`kan result`) recorded on any
/// `atom/<slug>` subject, or `None` if none has ever been assessed.
///
/// "Most recent" is a global order across atom subjects, which is why this
/// reads `recorded_at`: `show` returns each subject oldest-first, but the
/// baseline is *the* last assessment regardless of which atom it was on.
/// Claims kan returns are already live (retracted ones are gone), so a
/// retracted assessment simply stops being the baseline — AC-14 for free.
///
/// The per-atom `show` calls here are **memo-served** (day#71): every one
/// answers from the single bulk read `KanClient` already holds, so this loop
/// costs zero kan subprocesses however many atoms are declared. Stated
/// because the 2026-08 review read this loop as one invocation per atom —
/// the pre-day#71 cost — and `tests/status.rs` now pins the property the
/// comment claims, so the next reader gets an assertion rather than an
/// argument.
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
    forward: &crate::atoms::Forward<'_>,
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
        // From the acyclic view, not the declaration. `next:` here means "what
        // comes next" on a surface a person reads every session, so an edge day
        // could not order must not appear as one it did. `doctor` renders the
        // raw declaration instead, because there the job is to show what the
        // claim says.
        next: forward
            .successors(&atom.name)
            .iter()
            .map(|s| s.to_string())
            .collect(),
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

    /// A `Status` with nothing set, so a test can name only the field it is
    /// about. The struct has eight fields and a test that spells all of them
    /// buries its own subject.
    fn blank() -> Status {
        Status {
            here: vec![],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
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
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        let long = status.render_long();
        assert!(long.contains("Current atom: build"), "{long}");
        assert!(long.contains("[met] tests"), "{long}");
        assert!(long.contains("[unmet] docs"), "{long}");
        assert!(long.contains("next: review"), "{long}");

        let line = status.render_line();
        assert_eq!(line, "day · atom: build · 1/2 done · next: review");
    }

    #[test]
    fn several_candidates_are_all_named_and_none_chosen() {
        let status = Status {
            here: vec![here("design", vec![], &[]), here("build", vec![], &[])],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        let long = status.render_long();
        assert!(long.contains("2 atoms are consistent"), "{long}");
        assert!(long.contains("- design"), "{long}");
        assert!(long.contains("- build"), "{long}");

        let line = status.render_line();
        assert_eq!(line, "day · atom? design | build");
    }

    #[test]
    fn no_current_atom_says_so_in_both_forms() {
        let status = Status {
            here: vec![],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        assert!(status
            .render_long()
            .contains("No atom is currently in play"));
        assert_eq!(status.render_line(), "day · no atom in play");
    }

    #[test]
    fn no_probes_is_reported_as_uncheckable_not_as_no_atom() {
        let status = Status {
            here: vec![],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: true,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        assert!(status
            .render_long()
            .contains("No witness probes are declared"));
        assert_eq!(status.render_line(), "day · setup: declare schema/witness");

        // day#108, and the part that is a *property* rather than a literal:
        // the bar must name the thing that actually fixes this. The issue
        // suggested `not set up (day init)`, and `day init` records a
        // `schema/design-doc` starter and no witnesses at all — so it would
        // send a first-time reader to a verb that leaves the bar saying the
        // same thing. The literal above can be rewritten freely; this cannot.
        let line = status.render_line();
        assert!(
            line.contains(crate::telos::WITNESS_SLUG),
            "the setup line must name the subject that resolves it: {line}"
        );
        assert!(
            !line.contains("day init"),
            "`day init` does not declare witnesses, so pointing at it here would \
             be a remedy that does not remedy: {line}"
        );
    }

    /// day#108 — the bar distinguishes "day inferred one atom" from "several
    /// fit the evidence", and says which domain it is talking about.
    ///
    /// `candidates: generative-build, release` read cold looks like a list of
    /// failures, or a menu, or a queue. The plurality is a deliberate design
    /// property — day names them all rather than guessing which one you are in
    /// — and none of that survived into forty characters.
    ///
    /// Asserted as the distinction rather than as two literals: what must hold
    /// is that a reader can tell the two states apart and knows what is being
    /// named, not that either renders any particular way.
    #[test]
    fn the_bar_names_its_domain_and_marks_ambiguity() {
        let one = Status {
            here: vec![here("build", vec![], &[])],
            ..blank()
        };
        let several = Status {
            here: vec![here("build", vec![], &[]), here("release", vec![], &[])],
            ..blank()
        };

        let (one, several) = (one.render_line(), several.render_line());
        assert!(one.contains("atom"), "the domain must be named: {one}");
        assert!(several.contains("atom"), "in both states: {several}");
        assert_ne!(
            one.contains("atom?"),
            several.contains("atom?"),
            "one inferred atom and several candidates must not render alike — \
             that ambiguity is information:\n  {one}\n  {several}"
        );
        assert!(
            several.contains("build") && several.contains("release"),
            "the names are the actionable part, so ambiguity is named and not \
             merely counted: {several}"
        );
    }

    /// Off-sequence is a warning and gets its own line even in the terse form:
    /// a skipped step is exactly what a status-bar glance should catch.
    /// F2 — the done-but-unrecorded findings reach the model, not only
    /// `day status`.
    ///
    /// The review mutated the boundary line out of the model channel and it
    /// **SURVIVED**: the guarantee was computed at the mechanism and then
    /// dropped at three separate call sites, and the AC-8 scan could not see it
    /// because it asserts `status::compute` *calls* the check, not that the
    /// answer is delivered. day#101 one layer out.
    #[test]
    fn the_unrecorded_findings_reach_the_model_channel() {
        let mut status = Status {
            here: vec![here("build", vec![], &[])],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: Some("v1.0.0 is tagged but no `release` claim records it".into()),
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };

        let notice = status
            .standing_notice()
            .expect("a boundary finding must reach the model channel");
        assert!(
            notice.contains("v1.0.0"),
            "the notice must name the tag, or it cannot be acted on: {notice}"
        );

        // The generalised finding too, not only the release special case.
        status.unrecorded_boundary = None;
        status.unrecorded = vec!["code-change".into()];
        let notice = status
            .standing_notice()
            .expect("a paired-witness finding must reach the model channel too");
        assert!(
            notice.contains("code-change"),
            "the notice must name the artifact type: {notice}"
        );

        // And stays quiet when there is nothing to say.
        status.unrecorded = vec![];
        assert!(
            status.standing_notice().is_none(),
            "a healthy repo must produce no standing notice at all"
        );
    }

    /// F5 — the GENERAL finding reaches the status line, not only the release
    /// special case it was meant to generalise.
    ///
    /// The end-to-end test declared a single `tag` witness plus a `schema/docs`,
    /// so all four of its passing mutations ran through
    /// `docs::unrecorded_boundary` — the pre-existing release path. Deleting the
    /// `unrecorded` (paired-witness) half of `render_line` SURVIVED: the test
    /// named for the generalisation exercised only the instance.
    ///
    /// A unit test rather than another end-to-end one, because reaching this
    /// through the hooks needs a git stub reporting a changed file, and the
    /// property here is about rendering, not about probes.
    #[test]
    fn the_paired_witness_finding_reaches_the_status_line() {
        let status = Status {
            here: vec![here("build", vec![], &[])],
            off_sequence: vec![],
            unordered: vec![],
            unrecorded: vec!["code-change".into(), "design-doc".into()],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };

        let line = status.render_line();
        assert!(
            line.contains("code-change"),
            "the paired-witness finding must reach the bar, not only `day status` \
             — the release case is the instance, this is the rule: {line}"
        );
        assert!(
            line.contains("+1 more"),
            "with several types unrecorded the line must say so rather than \
             silently naming one: {line}"
        );
    }

    #[test]
    fn off_sequence_surfaces_in_both_forms() {
        let status = Status {
            here: vec![here("build", vec![], &["review"])],
            off_sequence: vec!["review produced its output but upstream build did not".into()],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
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
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: Some(Transition {
                from: "build".into(),
                to: vec!["review".into()],
            }),
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
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
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        assert_eq!(quiet.notice(), None);

        // A transition and an off-sequence both surface.
        let loud = Status {
            here: vec![here("review", vec![], &[])],
            off_sequence: vec!["build produced its output but upstream design did not".into()],
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: Some(Transition {
                from: "build".into(),
                to: vec!["review".into()],
            }),
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
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
            unordered: vec![],
            unrecorded: vec![],
            unrecorded_boundary: None,
            transition: None,
            uncheckable: false,
            unreadable: Vec::new(),
            cadence: crate::cache::DEFAULT_CADENCE,
        };
        let long = status.render_long();
        assert!(long.contains("[not run] passing-tests"), "{long}");
        assert!(long.contains("0/1 met"), "{long}");
    }
}
