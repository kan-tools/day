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
    let text = std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.display().to_string(),
        source,
    })?;
    Ok(Document::parse(&text))
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
        out.push_str(&format!("  observe  {}\n", self.observe));
        out.push_str(&format!("  plan     {}\n", self.plan));
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
    let doc = read_document(path)?;
    let report = design::check(&doc, schema, base);
    let subject = subject
        .map(str::to_string)
        .unwrap_or_else(|| slug_for(path));
    let shown = path.display();

    let observe_text = format!(
        "design doc {shown} checked against the live {} schema: {}",
        crate::schema::DEFAULT_SLUG,
        report.summary()
    );
    let observe = client.append(Write::new("observe", &subject, &observe_text))?;

    let summary = doc
        .summary_line()
        .unwrap_or_else(|| "(no summary section)".to_string());
    let plan_text = format!(
        "{subject} design ({shown}): {summary} [{}]",
        report.summary()
    );
    let cites = [observe.clone()];
    let mut write = Write::new("plan", &subject, &plan_text).cites(&cites);
    let title = doc.title.clone();
    if let Some(title) = title.as_deref() {
        write = write.declaring(title, "idea");
    }
    let plan = client.append(write)?;

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

/// Resolution ids already recorded as `decide` claims on a subject.
///
/// A read failure yields an empty set, which means "record everything" — the
/// safe direction: a duplicate decision is noise in an append-only log, while
/// skipping one that was never recorded loses it. Stated because the opposite
/// default would be the silent-loss failure this milestone is about.
fn existing_resolution_ids(
    client: &KanClient,
    subject: &str,
    schema: &crate::schema::Schema,
) -> std::collections::BTreeSet<String> {
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

    if atom.interface.next.is_empty() {
        return Ok(format!(
            "{} declares no successors — this is a terminal step in the current vocabulary.\n",
            atom.subject()
        ));
    }

    let mut out = format!("After {}:\n", atom.subject());
    for successor in &atom.interface.next {
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
