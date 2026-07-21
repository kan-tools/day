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

/// The four permitted adversarial-review verdicts. A closed set, checked at
/// the argument boundary — a verdict outside it is a malformed argument, not
/// a workflow gate.
pub const VERDICTS: [&str; 4] = ["APPROVE", "APPROVE-WITH-FOLLOW-UPS", "REDIRECT", "BLOCK"];

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
    #[error(
        "verdict must be one of {}, got `{got}`",
        VERDICTS.join(", ")
    )]
    BadVerdict { got: String },
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
    for bullet in doc.bullets(&schema.resolved_section) {
        decisions.push(client.append(Write::new("decide", &subject, &bullet).cites(&plan_cites))?);
    }

    Ok(Recorded {
        subject,
        observe,
        plan,
        decisions,
        report,
    })
}

/// Appends an adversarial-review verdict. The verdict must be one of
/// [`VERDICTS`] and must cite the claim it audits.
pub fn review(
    client: &KanClient,
    subject: &str,
    verdict: &str,
    rationale: &str,
    cites: &[String],
) -> Result<String, Error> {
    let normalized = verdict.trim().to_uppercase().replace(' ', "-");
    if !VERDICTS.contains(&normalized.as_str()) {
        return Err(Error::BadVerdict {
            got: verdict.to_string(),
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
