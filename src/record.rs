//! Recording a design pass and a review verdict into kan, and reporting what
//! the atom graph says comes next.
//!
//! The chain day appends is `observe` → `plan` → one `decide` per resolved
//! question, each citing the last. day assembles it from CIDs it captured
//! itself; nothing here takes a CID from a caller's prose.

use std::path::Path;

use crate::atoms::{self, Atom};
use crate::design::{self, Document, Report};
use crate::kan_client::{KanClient, Write};
use crate::schema::Schema;

/// day's own four adversarial-review verdicts — the **default** vocabulary, not
/// the only one (day#77).
///
/// A closed set, checked at the argument boundary: a verdict outside it is a
/// malformed argument, not a workflow gate. What day#77 changes is *which*
/// closed set, moving it from code to a claim on `schema/verdicts`; the
/// closedness itself is the property both vocabularies exist to preserve, since
/// free text is what forces adjudication to be optional.
pub const DEFAULT_VERDICTS: [&str; 4] = ["APPROVE", "APPROVE-WITH-FOLLOW-UPS", "REDIRECT", "BLOCK"];

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error(transparent)]
    Schema(#[from] crate::schema::Error),
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("verdict must be one of {permitted}, got `{got}`")]
    BadVerdict { got: String, permitted: String },
    #[error(transparent)]
    Blocks(#[from] crate::blocks::Error),
    #[error("a review verdict must cite the design claim it audits (--cites <cid>)")]
    UncitedVerdict,
    #[error("no atom named `{0}` is declared in this project")]
    NoSuchAtom(String),
}

pub fn read_document(path: &Path) -> Result<Document, Error> {
    Ok(read_document_with_source(path)?.0)
}

/// The parsed document **and the bytes it was parsed from**.
///
/// day#119's second round needs the source: the claim texts a design pass
/// writes are both *summaries*, so an edit that leaves the finding counts and
/// the Summary line alone is invisible to them — a requirement reversed in
/// meaning reported `(unchanged)` and recorded nothing.
pub fn read_document_with_source(path: &Path) -> Result<(Document, String), Error> {
    let text = std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.display().to_string(),
        source,
    })?;
    let doc = Document::parse(&text);
    Ok((doc, text))
}

/// A content fingerprint for a design document: byte length and an FNV-1a
/// hash, rendered as `<len>:<hash>`.
///
/// **Written out rather than pulled in.** day has no hash dependency and adding
/// one for this would not match kan's dependency set; `DefaultHasher` is the
/// obvious alternative and is explicitly *not* stable across Rust releases,
/// which for a value that lives in a durable claim means a toolchain upgrade
/// would silently make every document look edited. FNV-1a is ten lines and
/// stable because day defines it.
///
/// **Length is carried alongside the hash deliberately.** A 64-bit hash can
/// collide, and a collision here fails in the bad direction — reporting
/// `unchanged` for a document that changed, which is the defect this fixes. Two
/// revisions of one design doc colliding on both length and hash is not a risk
/// worth a dependency, and stating the bound is cheaper than implying there
/// isn't one.
fn fingerprint(source: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in source.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{}:{hash:016x}", source.len())
}

/// The opening of every observe claim `design` writes.
///
/// **A contract between the writer and the reader in this one function**, which
/// is why it is a constant rather than typed twice. day#158: `newest_of_kind`
/// took the newest `Observation` of any kind on the subject, and
/// `commands/adversarial-review.md` records every review finding as
/// `kan observe "<finding>" --subject <subject>` — on the design subject. So
/// after a review the "previous pair" was a review finding, and the next design
/// pass cited it as the claim it superseded.
pub const DESIGN_OBSERVE_OPENING: &str = "design doc ";

/// The opening of every plan claim `design` writes, for a given subject.
fn design_plan_opening(subject: &str) -> String {
    format!("{subject} design (")
}

/// Slug for a design doc's subject: the filename stem unless overridden.
pub fn slug_for(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "design".to_string())
}

pub struct Recorded {
    pub subject: String,
    pub observe: String,
    pub plan: String,
    /// Whether the observe/plan pair was **unchanged** since the last run, so
    /// nothing was appended and the reported CIDs are the existing claims'
    /// (day#119). Reported rather than silent for the same reason `skipped` is:
    /// "recorded" and "already said this" are different facts, and printing the
    /// second as the first is what made three passes leave six near-identical
    /// claims nobody could order.
    pub pair_unchanged: bool,
    /// Whether the previous pair predates the document fingerprint, so this
    /// pass re-recorded it for a FORMAT reason rather than a content one
    /// (day#119's second round). Reported because the new claim cites the old,
    /// and an uncommented citation asserts "this supersedes that" about two
    /// passes over an identical document.
    pub format_migration: bool,
    pub decisions: Vec<String>,
    /// Resolution ids already on the subject, so a re-record is incremental
    /// (day#36). Reported rather than silent: "recorded 2, skipped 8" is a
    /// different fact from "recorded 2".
    pub skipped: Vec<String>,
    /// Resolved-question bullets carrying no id. These are recorded every time,
    /// because nothing identifies them across runs — reported so the duplication
    /// is a known consequence rather than a surprise.
    pub unidentified: usize,
    pub report: Report,
}

impl Recorded {
    pub fn render(&self) -> String {
        let mut out = format!("recorded design pass on subject `{}`\n", self.subject);
        if self.pair_unchanged {
            out.push_str(&format!("  observe  {} (unchanged)\n", self.observe));
            out.push_str(&format!("  plan     {} (unchanged)\n", self.plan));
        } else {
            out.push_str(&format!("  observe  {}\n", self.observe));
            out.push_str(&format!("  plan     {}\n", self.plan));
        }
        if self.format_migration {
            out.push_str(
                "  note     the previous pair predates the document fingerprint, so this pass \
                 re-recorded it for a format reason rather than because the document \
                 changed. The next pass over an unchanged document will report \
                 `(unchanged)`.\n",
            );
        }
        for cid in &self.decisions {
            out.push_str(&format!("  decide   {cid}\n"));
        }
        // day#36: what was NOT recorded, and why, is as much of the answer as
        // what was. "recorded 2" and "recorded 2, skipped 8 already on the
        // subject" are different facts, and only the second lets a reader tell
        // an incremental re-record from a design that resolved two questions.
        if !self.skipped.is_empty() {
            out.push_str(&format!(
                "  skipped  {} already recorded on this subject: {}\n",
                self.skipped.len(),
                self.skipped.join(", ")
            ));
        }
        if self.unidentified > 0 {
            out.push_str(&format!(
                "  note     {} resolved question(s) carry no id, so re-recording will \
                 append them again — give them `{}n` ids to make a re-record incremental\n",
                self.unidentified,
                crate::schema::Schema::starter().resolution_prefix,
            ));
        }
        if !self.report.is_clean() {
            out.push_str(
                "\nThe document did not pass validation. It was recorded anyway, with the\n\
                 result embedded in the plan claim — an under-specified design is more\n\
                 useful visible in the graph than blocked at a gate.\n",
            );
            out.push_str(&self.report.render());
        }
        out
    }
}

/// Validates a design document and appends its claim chain **regardless of
/// the result**, embedding the validation summary in the plan claim.
///
/// Recording a failing document rather than refusing it is deliberate
/// (`telos/affordance-not-enforcement`): gating would mostly cause people to
/// skip recording, not to fix the doc, and an unrecorded design is worse for
/// the record than a visibly rough one.
pub fn design(
    client: &KanClient,
    path: &Path,
    base: &Path,
    subject: Option<&str>,
    schema: &Schema,
) -> Result<Recorded, Error> {
    let (doc, source) = read_document_with_source(path)?;
    let report = design::check(&doc, schema, base);
    let subject = subject
        .map(str::to_string)
        .unwrap_or_else(|| slug_for(path));
    let shown = path.display();

    // **Refuse before appending anything if the view of this subject is
    // partial.** A cold review's BLOCK-2, and a regression this branch caused.
    //
    // `newest_of_kind` and `existing_resolution_ids` both degrade a failed read
    // to "record everything", which is right when the subject is UNREADABLE — a
    // duplicate is noise, a skipped claim is a loss. It is wrong when the
    // subject is PARTIALLY readable, because then the claims day would be
    // deduplicating against are known to exist and known to be hidden, so the
    // duplicate is not a risk but a certainty. Measured: three runs over an
    // unchanged document produced three observes, three plans and three
    // identical decides, reported as if each were a first recording.
    //
    // day cannot retract, so that damage is permanent and grows per run. A
    // write verb may refuse — the never-blocking rule is about hooks, which
    // must always render — and refusing is the only option that does not
    // silently corrupt the record it exists to keep.
    if let Err(e @ crate::kan_client::Error::PartiallyWithheld { .. }) = client.show(&subject) {
        return Err(Error::Kan(e));
    }

    // The fingerprint is what makes "unchanged" mean unchanged. Without it both
    // texts this function compares are summaries, and the first version of
    // day#119 reported `(unchanged)` for a document whose REQ-1 had been
    // reversed in meaning — recording nothing, on an append-only log, while
    // telling the user it had nothing to record.
    let observe_text = format!(
        "{DESIGN_OBSERVE_OPENING}{shown} checked against the live {} schema: {} [doc {}]",
        crate::schema::DEFAULT_SLUG,
        report.summary(),
        fingerprint(&source)
    );
    // day#119: the observe/plan pair was appended on every run, so three passes
    // over one evolving document left three of each — while the resolution half
    // correctly recorded one `decide` per `RQ-` id. The command already knew how
    // to ask "is this on the subject already"; it asked for one of the three
    // things it writes.
    //
    // **Superseding is done by citing, never by retracting.** The issue's
    // preferred remedy was to retract the previous pair, and day may not: it
    // only ever appends, and kan exposes no destroy path for it to reach. So an
    // unchanged pair records nothing, and a changed one cites the pair it
    // supersedes — which is how kan expresses supersession anyway, and leaves
    // the ordering explicit rather than inferred from CID order.
    // **Selected by what this mechanism WRITES, not by kind alone** (day#158).
    //
    // A design subject accumulates claims from several sources: review findings
    // are `kan observe` on the same subject by this repo's own prescribed
    // workflow, and a `Result` may sit between two design passes — day's own
    // `witness-model` subject already looks like that. Taking the newest
    // `Observation` picked whichever arrived last, so a design pass could report
    // a false format migration AND cite an adversarial review's finding as the
    // claim it superseded, into a log day cannot retract from.
    //
    // The opening is the discriminator because day writes it and nothing else
    // does. It is shared with the `format!` above rather than restated, so the
    // writer and the reader cannot drift apart — which is the failure this is.
    let plan_opening = design_plan_opening(&subject);
    let previous_observe =
        newest_written_by_design(client, &subject, "Observation", DESIGN_OBSERVE_OPENING);
    let previous_plan = newest_written_by_design(client, &subject, "Plan", &plan_opening);

    let summary = doc
        .summary_line()
        .unwrap_or_else(|| "(no summary section)".to_string());
    let plan_text = format!(
        "{}{shown}): {summary} [{}]",
        design_plan_opening(&subject),
        report.summary()
    );
    // **BOTH halves must be unchanged, and this is not a formality.** The two
    // texts are derived from *different* things: the observe carries the
    // validation report's summary, the plan carries the document's own summary
    // line. So a document whose Summary section was rewritten while its counts
    // stayed put changes the plan text and not the observe text — deciding from
    // the observe half alone silently records neither, losing a real change.
    //
    // Caught by running it, after a comment here asserted the opposite: three
    // passes over an edited document all reported `(unchanged)`. The mechanism
    // the comment named was wrong about the code beside it, which is the exact
    // failure CLAUDE.md keeps a section for.
    //
    // They still move together once decided, because a plan citing an observe
    // from two passes ago is a worse record than either outcome.
    let pair_unchanged = previous_observe
        .as_ref()
        .is_some_and(|(_, text)| text == &observe_text)
        && previous_plan
            .as_ref()
            .is_some_and(|(_, text)| text == &plan_text);

    // **A pair recorded before the fingerprint existed will never compare
    // equal, so the first pass after upgrading re-records it and cites the old
    // one — asserting a supersession that did not happen** (a cold review's
    // MAJOR-3). Bounded and self-healing: the second pass is `(unchanged)`
    // again. Silent is the part that is wrong, and day's own log has 22 design
    // subjects, so every project with a history meets this once per document.
    //
    // Detected by the marker's absence in the PREVIOUS claim rather than by a
    // version stamp, because the claim is prose day wrote and there is nothing
    // else in it to key on. Reported, not worked around: pretending the pair is
    // unchanged would drop a pass that genuinely might have differed, and day
    // cannot tell which from a claim written before it could.
    let format_migration = !pair_unchanged
        && previous_observe
            .as_ref()
            .is_some_and(|(_, text)| !text.contains(" [doc "));

    let observe = match &previous_observe {
        Some((cid, _)) if pair_unchanged => cid.clone(),
        Some((cid, _)) => {
            let cites = [cid.clone()];
            client.append(Write::new("observe", &subject, &observe_text).cites(&cites))?
        }
        None => client.append(Write::new("observe", &subject, &observe_text))?,
    };

    let plan = match &previous_plan {
        Some((cid, _)) if pair_unchanged => cid.clone(),
        previous => {
            let mut cites = vec![observe.clone()];
            if let Some((cid, _)) = previous {
                cites.push(cid.clone());
            }
            let mut write = Write::new("plan", &subject, &plan_text).cites(&cites);
            let title = doc.title.clone();
            if let Some(title) = title.as_deref() {
                write = write.declaring(title, "idea");
            }
            client.append(write)?
        }
    };

    let plan_cites = [plan.clone()];
    let mut decisions = Vec::new();
    let mut skipped = Vec::new();
    let mut unidentified = 0usize;

    // day#36: re-recording a design must not re-append decisions already on the
    // subject. `/design` supports iterating, and every iteration that resolves a
    // question ADDS a bullet — so without this, the second run rewrote every
    // decide from the first.
    //
    // Keyed on a stable id, not on text. Text was the obvious key and breaks the
    // moment a bullet is reworded, which is precisely what iterating does: a
    // sharpened wording would record twice, and a rewording that changed the
    // MEANING would record once and be silently wrong.
    let already: std::collections::BTreeSet<String> =
        existing_resolution_ids(client, &subject, schema);

    for bullet in doc.bullets(&schema.resolved_section) {
        match resolution_id(&bullet, &schema.resolution_prefix) {
            Some(id) if already.contains(&id) => {
                skipped.push(id);
                continue;
            }
            Some(_) => {}
            // A bullet with no id cannot be deduplicated, so it is recorded —
            // the pre-day#36 behaviour, which is right for a document that has
            // not adopted ids. Counted so the report can say why re-recording
            // will duplicate it rather than leaving that to be discovered.
            None => unidentified += 1,
        }
        decisions.push(client.append(Write::new("decide", &subject, &bullet).cites(&plan_cites))?);
    }

    Ok(Recorded {
        subject,
        observe,
        plan,
        pair_unchanged,
        format_migration,
        decisions,
        skipped,
        unidentified,
        report,
    })
}

/// The resolution id a bullet declares, e.g. `RQ-1` from `- RQ-1: …`.
///
/// Read from the start of the bullet only. A `RQ-2` mentioned mid-sentence is a
/// reference to another decision, not this bullet's own id — the same anchoring
/// distinction day#70 drew between `starts_with` and `contains`, for the same
/// reason.
pub fn resolution_id(bullet: &str, prefix: &str) -> Option<String> {
    let rest = bullet.trim_start().strip_prefix(prefix)?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    (!digits.is_empty()).then(|| format!("{prefix}{digits}"))
}

/// The newest claim of `kind` on `subject` **that a design pass wrote**, as
/// `(cid, text)`, identified by the opening day itself emits.
///
/// day#119 needed "the previous pair"; day#158 is what it cost to answer that
/// with "the newest claim of that kind". A design subject collects claims from
/// several sources — review findings arrive as `kan observe` on the same
/// subject, which is what `commands/adversarial-review.md` prescribes — so the
/// newest `Observation` is frequently not a design pass at all. Selecting by
/// kind alone made a design pass cite a review's finding as superseded, and
/// report a format migration that had not happened, over an unchanged document
/// at exit 0.
///
/// Newest by position: `KanClient::show` returns a subject's claims in record
/// order, which is the same ordering [`existing_resolution_ids`] relies on by
/// not needing one.
///
/// **The second `kan-read-may-degrade` site in this module, not the first.**
/// [`existing_resolution_ids`] states the argument in full; both spend the hatch
/// for the same reason and it is stated once, there.
///
/// [`existing_resolution_ids`]: fn@existing_resolution_ids
fn newest_written_by_design(
    client: &KanClient,
    subject: &str,
    kind: &str,
    opening: &str,
) -> Option<(String, String)> {
    // fallback: unreadable-subject-records-the-pair
    // kan-read-may-degrade: a failed read here degrades to "append the pair",
    // which is exactly the pre-day#119 behaviour — a duplicate claim, never a
    // lost one. The opposite degradation (treat unreadable as unchanged) would
    // silently record NOTHING for a design pass that did happen, so the
    // direction is chosen rather than inherited.
    let claims = client.show(subject).ok()?;
    claims
        .iter()
        .rev()
        .filter(|c| c.kind == kind)
        .find(|c| c.text.as_deref().is_some_and(|t| t.starts_with(opening)))
        .and_then(|c| c.text.clone().map(|t| (c.cid.clone(), t)))
}

/// Resolution ids already recorded as `decide` claims on a subject.
///
/// A read failure yields an empty set, which means "record everything" — the
/// safe direction: a duplicate decision is noise in an append-only log, while
/// skipping one that was never recorded loses it. Stated because the opposite
/// default would be the silent-loss failure that rule is about.
///
/// [`newest_of_kind`] spends the hatch on the same argument. This comment used
/// to call itself the only site, and then drifted onto that function when it
/// was added — so the sentence asserting uniqueness ended up on the thing that
/// made it false. The argument lives here; the other site points at it.
///
/// [`newest_of_kind`]: fn@newest_of_kind
fn existing_resolution_ids(
    client: &KanClient,
    subject: &str,
    schema: &crate::schema::Schema,
) -> std::collections::BTreeSet<String> {
    // kan-read-may-degrade: failing toward "record everything" is the safe
    // direction — see this function's doc comment. "and only here" was removed
    // rather than reworded: `newest_of_kind` spends the hatch on the same
    // argument, so the claim was false from the moment that function landed.
    let Ok(claims) = client.show(subject) else {
        return std::collections::BTreeSet::new();
    };
    claims
        .iter()
        .filter(|c| c.kind == "Decision")
        .filter_map(|c| c.text.as_deref())
        .filter_map(|t| resolution_id(t, &schema.resolution_prefix))
        .collect()
}

/// Appends an adversarial-review verdict. The verdict must be in the project's
/// declared vocabulary — [`DEFAULT_VERDICTS`] when none is declared — and must
/// cite the claim it audits.
pub fn review(
    client: &KanClient,
    subject: &str,
    verdict: &str,
    rationale: &str,
    cites: &[String],
) -> Result<String, Error> {
    let vocabulary = crate::blocks::VerdictVocabulary::load(client)?;
    let normalized = crate::blocks::normalize(verdict);
    if !vocabulary.permits(&normalized) {
        return Err(Error::BadVerdict {
            got: verdict.to_string(),
            permitted: vocabulary.verdicts.join(", "),
        });
    }
    if cites.is_empty() {
        return Err(Error::UncitedVerdict);
    }
    let text = format!("adversarial review of {subject}: {normalized} — {rationale}");
    Ok(client.append(Write::new("decide", subject, &text).cites(cites))?)
}

/// What the atom graph says follows `name`, and what each successor needs.
///
/// This exists so neither command has to name the other: composition is read
/// from kan, so inserting an atom between two others is a claim, not a prompt
/// edit (`telos/composable-process`).
pub fn next(client: &KanClient, name: &str) -> Result<String, Error> {
    let (atoms_list, _) = atoms::load(client)?;
    let atom = atoms_list
        .iter()
        .find(|a| a.name == name)
        .ok_or_else(|| Error::NoSuchAtom(name.to_string()))?;

    // Successors come from the acyclic view, never the raw declaration. Before
    // day#113 `day next adversarial-review` listed `generative-build` and
    // `pull-request` as equal successors — the fix loop presented as the next
    // step. Feedback edges are still shown, below, under what they are.
    let forward = atoms::Forward::build(&atoms_list);
    let successors = forward.successors(&atom.name);
    let unordered: Vec<&atoms::Cycle> = forward
        .cycles()
        .iter()
        .filter(|c| c.atoms.contains(&atom.name))
        .collect();

    // **"Terminal" is a positive claim, so it may only be made when day could
    // actually look.** An atom whose only `next` edge was dropped as cyclic has
    // no *orderable* successor and is not a sink, and saying otherwise is
    // checked-and-clean standing in for could-not-check — which is the thing
    // `docs/CONVENTIONS.md` promises this verb does not do, naming it.
    //
    // This was an early return that fired before the cycle report below, so
    // `day next` on an unmigrated vocabulary said "this is a terminal step in
    // the current vocabulary" about an atom that plainly declares a successor.
    // Found by a cold review of this branch; AC-12's fixture is migrated and
    // acyclic, so it could not reach the mode at all.
    let mut out = String::new();
    if !successors.is_empty() {
        out.push_str(&format!("After {}:\n", atom.subject()));
    } else if unordered.is_empty() {
        out.push_str(&format!(
            "{} declares no successors — this is a terminal step in the current vocabulary.\n",
            atom.subject()
        ));
    } else {
        out.push_str(&format!(
            "{} has no successor day can order — this is not a terminal step.\n",
            atom.subject()
        ));
    }
    for successor in successors {
        match atoms_list.iter().find(|a| &a.name == successor) {
            Some(next_atom) => {
                out.push_str(&format!(
                    "  {}  needs [{}]\n",
                    next_atom.subject(),
                    next_atom.interface.inputs.join(", ")
                ));
                for input in &next_atom.interface.inputs {
                    let producers: Vec<String> = producers_of(&atoms_list, input)
                        .into_iter()
                        .filter(|p| p != &next_atom.name)
                        .map(|p| format!("{}{p}", atoms::ATOM_PREFIX))
                        .collect();
                    out.push_str(&format!(
                        "    {input}: {}\n",
                        if producers.is_empty() {
                            "not produced by any declared atom".to_string()
                        } else {
                            format!("from {}", producers.join(", "))
                        }
                    ));
                }
            }
            None => out.push_str(&format!(
                "  {}{successor}  (declared as a successor but not itself declared)\n",
                atoms::ATOM_PREFIX
            )),
        }
    }

    if !atom.interface.revisits.is_empty() {
        out.push_str(&format!(
            "\nA negative outcome at {} sends you back to:\n",
            atom.subject()
        ));
        for target in &atom.interface.revisits {
            out.push_str(&format!("  {}{target}\n", atoms::ATOM_PREFIX));
        }
        out.push_str("Not a next step: this is work this atom can invalidate.\n");
    }

    for cycle in unordered {
        out.push_str(&format!("\n  ? {}\n", cycle.message()));
    }

    Ok(out)
}

fn producers_of(atoms_list: &[Atom], output: &str) -> Vec<String> {
    atoms_list
        .iter()
        .filter(|a| a.interface.outputs.iter().any(|o| o == output))
        .map(|a| a.name.clone())
        .collect()
}

#[cfg(test)]
mod resolution_ids {
    use super::*;

    /// day#36: an id is read from the START of a bullet only.
    ///
    /// An `RQ-2` mentioned mid-sentence is a *reference* to another decision,
    /// not this bullet's own id — the same anchoring distinction day#70 drew
    /// between `starts_with` and `contains`, and for the same reason: the
    /// unanchored reading matches things that merely talk about the thing.
    #[test]
    fn an_id_is_anchored_at_the_start_of_the_bullet() {
        assert_eq!(
            resolution_id("RQ-1: we settled x", "RQ-"),
            Some("RQ-1".into())
        );
        assert_eq!(
            resolution_id("  RQ-12: padded", "RQ-"),
            Some("RQ-12".into())
        );

        // A reference, not an id.
        assert_eq!(resolution_id("this supersedes RQ-2 entirely", "RQ-"), None);
        // No id at all — the pre-day#36 shape, which stays valid.
        assert_eq!(resolution_id("we settled x", "RQ-"), None);
        // The prefix without digits is not an id.
        assert_eq!(resolution_id("RQ- something", "RQ-"), None);
    }

    /// The prefix is the project's, like `requirement_prefix` before it.
    #[test]
    fn the_prefix_is_declared_not_assumed() {
        assert_eq!(resolution_id("D-7: a decision", "D-"), Some("D-7".into()));
        assert_eq!(resolution_id("RQ-7: a decision", "D-"), None);
    }
}
