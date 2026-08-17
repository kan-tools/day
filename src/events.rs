//! Explicit recording conventions for acquired input and interventions.
//!
//! Both are ordinary kan Observations. day validates and packages the payload;
//! kan remains responsible for authorship, signing, CIDs, and durable storage.

use serde::{Deserialize, Serialize};

use crate::kan_client::{KanClient, Write};

pub const ACQUIRED_INPUT_FENCE: &str = "day-acquired-input";
pub const INTERVENTION_FENCE: &str = "day-intervention";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error("the active recording author could not be established")]
    UnknownRecorder,
    #[error("{field} must not be empty")]
    Empty { field: &'static str },
    #[error("at least one fact, decision, or unresolved item is required")]
    EmptyAcquiredInput,
    #[error("source claim `{0}` is not visible in the current kan view")]
    MissingSourceClaim(String),
    #[error("source claim `{0}` has no author, so it cannot authenticate a provider")]
    UnattributedSourceClaim(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "provenance", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Source {
    /// The recorder reports who supplied the material; this is not first-hand
    /// authentication of that provider.
    Reported { description: String },
    /// The cited claim authenticates speech by its own signing principal. This
    /// says nothing about repository admission or view trust.
    AuthenticatedClaim { principal: String, claim: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcquiredInput {
    pub work_subject: String,
    pub topic: String,
    pub provider: Source,
    pub recorded_by: String,
    pub facts: Vec<String>,
    pub decisions: Vec<String>,
    pub unresolved: Vec<String>,
    pub material_effect: String,
    pub basis: Vec<String>,
}

impl crate::atoms::Versioned for AcquiredInput {
    const SUPPORTED_VERSION: u64 = 1;
    const FENCE: &'static str = ACQUIRED_INPUT_FENCE;

    fn validate(&self) -> Result<(), String> {
        validate_source(&self.provider)?;
        for (field, value) in [
            ("work subject", self.work_subject.as_str()),
            ("topic", self.topic.as_str()),
            ("recording author", self.recorded_by.as_str()),
            ("material effect", self.material_effect.as_str()),
        ] {
            nonempty(field, value).map_err(|error| error.to_string())?;
        }
        if self.facts.is_empty() && self.decisions.is_empty() && self.unresolved.is_empty() {
            return Err("at least one fact, decision, or unresolved item is required".into());
        }
        if self.basis.is_empty() {
            return Err("at least one basis CID is required".into());
        }
        Ok(())
    }
}

impl AcquiredInput {
    pub fn to_claim_text(&self) -> String {
        fenced(
            "Acquired input recorded explicitly.",
            ACQUIRED_INPUT_FENCE,
            self,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum InterventionKind {
    DirectionCorrection,
    MissingContext,
    AnsweredQuestion,
    StoppedWork,
    Approval,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Intervention {
    pub work_subject: String,
    pub kind: InterventionKind,
    pub summary: String,
    pub material_effect: String,
    pub source: Source,
    pub recorded_by: String,
    pub basis: Vec<String>,
}

impl crate::atoms::Versioned for Intervention {
    const SUPPORTED_VERSION: u64 = 1;
    const FENCE: &'static str = INTERVENTION_FENCE;

    fn validate(&self) -> Result<(), String> {
        validate_source(&self.source)?;
        for (field, value) in [
            ("work subject", self.work_subject.as_str()),
            ("summary", self.summary.as_str()),
            ("material effect", self.material_effect.as_str()),
            ("recording author", self.recorded_by.as_str()),
        ] {
            nonempty(field, value).map_err(|error| error.to_string())?;
        }
        if self.basis.is_empty() {
            return Err("at least one basis CID is required".into());
        }
        Ok(())
    }
}

impl Intervention {
    pub fn to_claim_text(&self) -> String {
        fenced(
            "Intervention classified and recorded explicitly.",
            INTERVENTION_FENCE,
            self,
        )
    }
}

#[derive(Debug)]
pub struct AcquiredInputRequest {
    pub subject: String,
    pub topic: String,
    pub reported_provider: Option<String>,
    pub provider_claim: Option<String>,
    pub facts: Vec<String>,
    pub decisions: Vec<String>,
    pub unresolved: Vec<String>,
    pub material_effect: String,
    pub basis: Vec<String>,
}

#[derive(Debug)]
pub struct InterventionRequest {
    pub subject: String,
    pub kind: InterventionKind,
    pub summary: String,
    pub material_effect: String,
    pub reported_source: Option<String>,
    pub source_claim: Option<String>,
    pub basis: Vec<String>,
}

pub fn record_acquired_input(
    client: &KanClient,
    mut request: AcquiredInputRequest,
) -> Result<String, Error> {
    nonempty("work subject", &request.subject)?;
    nonempty("topic", &request.topic)?;
    nonempty("material effect", &request.material_effect)?;
    if request.facts.is_empty() && request.decisions.is_empty() && request.unresolved.is_empty() {
        return Err(Error::EmptyAcquiredInput);
    }
    let recorded_by = client.identity().ok_or(Error::UnknownRecorder)?;
    let provider = source(
        client,
        request.reported_provider.take(),
        request.provider_claim.take(),
        &mut request.basis,
    )?;
    if request.basis.is_empty() {
        return Err(Error::Empty { field: "basis CID" });
    }
    let payload = AcquiredInput {
        work_subject: request.subject.clone(),
        topic: request.topic,
        provider,
        recorded_by,
        facts: cleaned("fact", request.facts)?,
        decisions: cleaned("decision", request.decisions)?,
        unresolved: cleaned("unresolved item", request.unresolved)?,
        material_effect: request.material_effect,
        basis: cleaned("basis CID", request.basis)?,
    };
    let text = payload.to_claim_text();
    Ok(client.append(Write::new("observe", &request.subject, &text).cites(&payload.basis))?)
}

pub fn record_intervention(
    client: &KanClient,
    mut request: InterventionRequest,
) -> Result<String, Error> {
    nonempty("work subject", &request.subject)?;
    nonempty("summary", &request.summary)?;
    nonempty("material effect", &request.material_effect)?;
    let recorded_by = client.identity().ok_or(Error::UnknownRecorder)?;
    let source = source(
        client,
        request.reported_source.take(),
        request.source_claim.take(),
        &mut request.basis,
    )?;
    if request.basis.is_empty() {
        return Err(Error::Empty { field: "basis CID" });
    }
    let payload = Intervention {
        work_subject: request.subject.clone(),
        kind: request.kind,
        summary: request.summary,
        material_effect: request.material_effect,
        source,
        recorded_by,
        basis: cleaned("basis CID", request.basis)?,
    };
    let text = payload.to_claim_text();
    Ok(client.append(Write::new("observe", &request.subject, &text).cites(&payload.basis))?)
}

fn source(
    client: &KanClient,
    reported: Option<String>,
    claim: Option<String>,
    basis: &mut Vec<String>,
) -> Result<Source, Error> {
    if let Some(cid) = claim {
        let author = client
            .show_all()?
            .into_iter()
            .map(|(_, claim)| claim)
            .find(|candidate| candidate.cid == cid)
            .ok_or_else(|| Error::MissingSourceClaim(cid.clone()))?
            .author
            .ok_or_else(|| Error::UnattributedSourceClaim(cid.clone()))?;
        if !basis.contains(&cid) {
            basis.push(cid.clone());
        }
        return Ok(Source::AuthenticatedClaim {
            principal: author,
            claim: cid,
        });
    }
    let description = reported.unwrap_or_default();
    nonempty("reported provider/source", &description)?;
    Ok(Source::Reported { description })
}

fn validate_source(source: &Source) -> Result<(), String> {
    let fields: Vec<(&str, &str)> = match source {
        Source::Reported { description } => {
            vec![("reported provider/source", description.as_str())]
        }
        Source::AuthenticatedClaim { principal, claim } => vec![
            ("authenticated principal", principal.as_str()),
            ("source claim", claim.as_str()),
        ],
    };
    for (field, value) in fields {
        if value.trim().is_empty() {
            return Err(format!("{field} must not be empty"));
        }
    }
    Ok(())
}

fn nonempty(field: &'static str, value: &str) -> Result<(), Error> {
    if value.trim().is_empty() {
        Err(Error::Empty { field })
    } else {
        Ok(())
    }
}

fn cleaned(field: &'static str, values: Vec<String>) -> Result<Vec<String>, Error> {
    for value in &values {
        nonempty(field, value)?;
    }
    Ok(values)
}

fn fenced<T: Serialize>(opening: &str, fence: &str, value: &T) -> String {
    let json = serde_json::to_string(value).expect("event payload structs always serialize");
    let json = format!("{{\"_version\":1,{}", &json[1..]);
    format!("{opening}\n\n```{fence}\n{json}\n```\n")
}
