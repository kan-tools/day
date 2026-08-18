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
    #[error("{field} looks like a raw conversation transcript; record a summary instead")]
    Transcript { field: &'static str },
    #[error("event payload is invalid: {0}")]
    InvalidPayload(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "provenance", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Source {
    /// The recording principal is also the source. The claim envelope
    /// authenticates this principal directly; no second claim is needed.
    Recorder { principal: String },
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
        validate_recorder_source(&self.provider, &self.recorded_by)?;
        for (field, value) in [
            ("work subject", self.work_subject.as_str()),
            ("topic", self.topic.as_str()),
            ("recording author", self.recorded_by.as_str()),
            ("material effect", self.material_effect.as_str()),
        ] {
            nonempty(field, value).map_err(|error| error.to_string())?;
            reject_transcript(field, value).map_err(|error| error.to_string())?;
        }
        if self.facts.is_empty() && self.decisions.is_empty() && self.unresolved.is_empty() {
            return Err("at least one fact, decision, or unresolved item is required".into());
        }
        validate_basis(&self.basis)?;
        validate_summaries("fact", &self.facts)?;
        validate_summaries("decision", &self.decisions)?;
        validate_summaries("unresolved item", &self.unresolved)?;
        let mut narrative = vec![
            self.work_subject.as_str(),
            self.topic.as_str(),
            self.material_effect.as_str(),
        ];
        if let Source::Reported { description } = &self.provider {
            narrative.push(description);
        }
        narrative.extend(self.facts.iter().map(String::as_str));
        narrative.extend(self.decisions.iter().map(String::as_str));
        narrative.extend(self.unresolved.iter().map(String::as_str));
        reject_transcript("acquired input", &narrative.join("\n"))
            .map_err(|error| error.to_string())?;
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
        validate_recorder_source(&self.source, &self.recorded_by)?;
        for (field, value) in [
            ("work subject", self.work_subject.as_str()),
            ("summary", self.summary.as_str()),
            ("material effect", self.material_effect.as_str()),
            ("recording author", self.recorded_by.as_str()),
        ] {
            nonempty(field, value).map_err(|error| error.to_string())?;
            reject_transcript(field, value).map_err(|error| error.to_string())?;
        }
        let mut narrative = vec![
            self.work_subject.as_str(),
            self.summary.as_str(),
            self.material_effect.as_str(),
        ];
        if let Source::Reported { description } = &self.source {
            narrative.push(description);
        }
        reject_transcript("intervention", &narrative.join("\n"))
            .map_err(|error| error.to_string())?;
        validate_basis(&self.basis)?;
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
    pub recorder_provider: bool,
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
    pub recorder_source: bool,
    pub basis: Vec<String>,
}

pub fn record_acquired_input(
    client: &KanClient,
    mut request: AcquiredInputRequest,
) -> Result<String, Error> {
    nonempty("work subject", &request.subject)?;
    nonempty("topic", &request.topic)?;
    nonempty("material effect", &request.material_effect)?;
    reject_transcript("topic", &request.topic)?;
    reject_transcript("material effect", &request.material_effect)?;
    if request.facts.is_empty() && request.decisions.is_empty() && request.unresolved.is_empty() {
        return Err(Error::EmptyAcquiredInput);
    }
    let recorded_by = client.identity().ok_or(Error::UnknownRecorder)?;
    let provider = source(
        client,
        request.reported_provider.take(),
        request.provider_claim.take(),
        request.recorder_provider,
        &recorded_by,
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
    crate::atoms::Versioned::validate(&payload).map_err(Error::InvalidPayload)?;
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
    reject_transcript("summary", &request.summary)?;
    reject_transcript("material effect", &request.material_effect)?;
    let recorded_by = client.identity().ok_or(Error::UnknownRecorder)?;
    let source = source(
        client,
        request.reported_source.take(),
        request.source_claim.take(),
        request.recorder_source,
        &recorded_by,
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
    crate::atoms::Versioned::validate(&payload).map_err(Error::InvalidPayload)?;
    let text = payload.to_claim_text();
    Ok(client.append(Write::new("observe", &request.subject, &text).cites(&payload.basis))?)
}

fn source(
    client: &KanClient,
    reported: Option<String>,
    claim: Option<String>,
    recorder: bool,
    recorded_by: &str,
    basis: &mut Vec<String>,
) -> Result<Source, Error> {
    if recorder {
        return Ok(Source::Recorder {
            principal: recorded_by.to_owned(),
        });
    }
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
    reject_transcript("reported provider/source", &description)?;
    Ok(Source::Reported { description })
}

fn validate_source(source: &Source) -> Result<(), String> {
    let fields: Vec<(&str, &str)> = match source {
        Source::Recorder { principal } => vec![("recording source principal", principal.as_str())],
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
        reject_transcript(field, value).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn validate_recorder_source(source: &Source, recorded_by: &str) -> Result<(), String> {
    if let Source::Recorder { principal } = source {
        if principal != recorded_by {
            return Err("recorder source principal must equal the recording author".into());
        }
    }
    Ok(())
}

fn validate_basis(basis: &[String]) -> Result<(), String> {
    if basis.is_empty() {
        return Err("at least one basis CID is required".into());
    }
    let unique = basis.iter().collect::<std::collections::BTreeSet<_>>();
    if unique.len() != basis.len() {
        return Err("basis CIDs must be unique".into());
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
        reject_transcript(field, value)?;
    }
    reject_transcript(field, &values.join("\n"))?;
    Ok(values)
}

fn validate_summaries(field: &'static str, values: &[String]) -> Result<(), String> {
    for value in values {
        nonempty(field, value).map_err(|error| error.to_string())?;
        reject_transcript(field, value).map_err(|error| error.to_string())?;
    }
    reject_transcript(field, &values.join("\n")).map_err(|error| error.to_string())?;
    Ok(())
}

/// Rejects common durable-transcript shapes without pretending day can infer
/// whether arbitrary prose originated in a conversation. Two or more
/// speaker-like labels are enough to identify turn-by-turn content. Labels are
/// intentionally not an allowlist: names such as `Alice:` and `[Bob]` are the
/// ordinary way a transcript bypasses role-name matching. Whitespace after a
/// colon is not required, and dash-delimited dialogue is covered too. One
/// label remains valid quoted evidence or ordinary prose.
fn reject_transcript(field: &'static str, value: &str) -> Result<(), Error> {
    let heading_turns = value
        .lines()
        .filter(|line| markdown_speaker_heading(line))
        .take(2)
        .count();
    let words = value.split_whitespace().collect::<Vec<_>>();
    let inline_turns = words
        .iter()
        .enumerate()
        .filter(|(index, word)| {
            let word = word.trim_matches(|character: char| matches!(character, '*' | '_' | '`'));
            let colon = word
                .split_once(':')
                .is_some_and(|(label, _)| speaker_label(label));
            let bracketed = word
                .strip_prefix('[')
                .and_then(|word| word.split_once(']'))
                .is_some_and(|(label, _)| speaker_label(label));
            let compact_dash = word
                .find(['–', '—'])
                .is_some_and(|split| speaker_label(&word[..split]));
            let dashed = words.get(index + 1).is_some_and(|next| {
                matches!(
                    next.trim_matches(|character: char| {
                        matches!(character, '*' | '_' | '`' | ',' | ';')
                    }),
                    "-" | "–" | "—"
                ) && speaker_label(
                    word.trim_matches(|character: char| !character.is_alphanumeric()),
                )
            });
            colon || bracketed || compact_dash || dashed
        })
        .take(2)
        .count();
    if heading_turns >= 2 || inline_turns >= 2 {
        Err(Error::Transcript { field })
    } else {
        Ok(())
    }
}

/// Returns whether text has the same multi-speaker shape rejected by event
/// validation. Release evidence uses this to avoid maintaining a weaker copy
/// of the durable-record boundary.
pub fn contains_transcript_shape(value: &str) -> bool {
    reject_transcript("text", value).is_err()
}

fn markdown_speaker_heading(line: &str) -> bool {
    let line = line.trim();
    let marked = line.starts_with('#')
        || (line.starts_with("**") && line.ends_with("**") && line.len() > 4)
        || (line.starts_with('_') && line.ends_with('_') && line.len() > 2)
        || (line.starts_with('`') && line.ends_with('`') && line.len() > 2);
    if !marked {
        return false;
    }
    let label = line
        .trim_start_matches('#')
        .trim()
        .trim_matches(|character| matches!(character, '*' | '_' | '`'));
    speaker_label(label) || multiword_speaker_name(label)
}

fn multiword_speaker_name(label: &str) -> bool {
    let parts = label.split_whitespace().collect::<Vec<_>>();
    (2..=4).contains(&parts.len())
        && parts.iter().all(|part| {
            part.chars().all(char::is_alphabetic)
                && part.chars().next().is_some_and(char::is_uppercase)
                && part
                    .chars()
                    .skip(1)
                    .all(|character| !character.is_alphabetic() || character.is_lowercase())
                && !semantic_label(&part.to_lowercase())
        })
}

fn speaker_label(label: &str) -> bool {
    let label = label.trim_matches(|character: char| !character.is_alphanumeric());
    if label.is_empty() || label.chars().count() > 40 || !label.chars().all(char::is_alphabetic) {
        return false;
    }
    let lower = label.to_lowercase();
    if matches!(
        lower.as_str(),
        "user" | "human" | "assistant" | "agent" | "system" | "speaker"
    ) {
        return true;
    }
    if semantic_label(&lower) {
        return false;
    }
    let mut characters = label.chars();
    characters.next().is_some_and(char::is_uppercase)
        && characters.all(|character| !character.is_alphabetic() || character.is_lowercase())
}

fn semantic_label(lower: &str) -> bool {
    matches!(
        lower,
        "acceptance"
            | "candidate"
            | "criteria"
            | "decision"
            | "effect"
            | "risk"
            | "mitigation"
            | "fact"
            | "facts"
            | "source"
            | "scope"
            | "summary"
            | "topic"
            | "provider"
            | "recorder"
            | "release"
            | "requirement"
            | "requirements"
            | "unresolved"
    )
}

fn fenced<T: Serialize>(opening: &str, fence: &str, value: &T) -> String {
    let json = serde_json::to_string(value).expect("event payload structs always serialize");
    let json = format!("{{\"_version\":1,{}", &json[1..]);
    format!("{opening}\n\n```{fence}\n{json}\n```\n")
}
