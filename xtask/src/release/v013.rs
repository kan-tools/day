use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

pub const VERSION: &str = "v0.13.0-beta.1";
pub const ISSUES: &[u64] = &[93, 143, 152, 193, 195, 204];
pub const WORKFLOWS: &[&str] = &[
    ".github/workflows/ci.yml",
    ".github/workflows/agent-plugins.yml",
    ".github/workflows/kan-compat.yml",
    ".github/workflows/migration-matrix.yml",
    ".github/workflows/askme-behavioral-trial.yml",
    ".github/workflows/workflow-reconstruction-trial.yml",
];
pub const EVIDENCE_PROTOCOLS: &[&str] = &[
    ".release/protocols/askme-v1.json",
    ".release/protocols/reconstruction-v1.json",
];
pub const PUBLICATION_ARTIFACTS: &[&str] = &[
    ".release/v0.13-plan.json",
    ".github/workflows/release.yml",
    "git-tag",
    "crates-io-package",
    "github-release",
    "kan-release-claim",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    pub schema: u64,
    pub release: String,
    pub issues: Vec<u64>,
    pub workflows: Vec<String>,
    pub evidence_protocols: Vec<String>,
    pub publication_artifacts: Vec<String>,
}

pub fn contract() -> Contract {
    Contract {
        schema: 1,
        release: VERSION.into(),
        issues: ISSUES.to_vec(),
        workflows: WORKFLOWS.iter().map(|value| (*value).into()).collect(),
        evidence_protocols: EVIDENCE_PROTOCOLS
            .iter()
            .map(|value| (*value).into())
            .collect(),
        publication_artifacts: PUBLICATION_ARTIFACTS
            .iter()
            .map(|value| (*value).into())
            .collect(),
    }
}

pub fn verify_manifest(root: &Path, path: &Path) -> Outcome<()> {
    let path = root.join(path);
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "could not read v0.13 manifest `{}`: {error}",
                path.display()
            )))
        }
    };
    let actual: Contract = match serde_json::from_slice(&bytes) {
        Ok(actual) => actual,
        Err(error) => {
            return Outcome::Finding(Finding::new(format!(
                "v0.13 manifest `{}` is malformed: {error}",
                path.display()
            )))
        }
    };
    let expected = contract();
    if actual != expected {
        let expected = serde_json::to_string_pretty(&expected).expect("contract serializes");
        let actual = serde_json::to_string_pretty(&actual).expect("manifest serializes");
        return Outcome::Finding(Finding::new(format!(
            "v0.13 manifest does not equal the typed contract\nexpected:\n{expected}\nactual:\n{actual}"
        )));
    }
    for relative in WORKFLOWS.iter().chain(EVIDENCE_PROTOCOLS) {
        if !root.join(relative).is_file() {
            return Outcome::Finding(Finding::new(format!(
                "typed v0.13 dependency `{relative}` is missing"
            )));
        }
    }
    println!(
        "v0.13 manifest equals typed contract: {} issues, {} workflows, {} protocols, {} publication artifacts",
        ISSUES.len(),
        WORKFLOWS.len(),
        EVIDENCE_PROTOCOLS.len(),
        PUBLICATION_ARTIFACTS.len()
    );
    Outcome::Passed(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanManifest {
    schema: u64,
    cid: String,
    subject: String,
    rfc_result: String,
    normative_source: String,
    artifact: PlanArtifact,
    published_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanArtifact {
    commit: String,
    path: String,
    sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct KanShow {
    claims: Vec<KanPlanClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct KanPlanClaim {
    cid: String,
    kind: String,
    subject: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    cites: Vec<String>,
    #[serde(default)]
    artifacts: Vec<String>,
}

pub fn verify_plan(root: &Path, manifest_path: &Path, process: &dyn Process) -> Outcome<()> {
    let manifest_path = root.join(manifest_path);
    let manifest: PlanManifest =
        match read(&manifest_path).and_then(|bytes| json(&manifest_path, &bytes)) {
            Ok(manifest) => manifest,
            Err(error) => return Outcome::Finding(Finding::new(error)),
        };
    let kan = ProcessRequest::new("kan", ["show", manifest.subject.as_str(), "--json"], root);
    let kan = match process.run(&kan) {
        Ok(output) if output.status == 0 => output.stdout,
        Ok(output) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "could not resolve v0.13 Plan through kan (exit {}): {}",
                output.status, output.stderr
            )))
        }
        Err(error) => return Outcome::CouldNotCheck(CouldNotCheck::new(error)),
    };
    let show: KanShow = match serde_json::from_str(&kan) {
        Ok(show) => show,
        Err(error) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "kan returned an unreadable Plan view: {error}"
            )))
        }
    };
    let claim = match show.claims.iter().find(|claim| claim.cid == manifest.cid) {
        Some(claim) => claim,
        None => {
            return Outcome::Finding(Finding::new(format!(
                "published kan view does not contain selected Plan CID {}",
                manifest.cid
            )))
        }
    };
    let artifact_spec = format!("{}:{}", manifest.artifact.commit, manifest.artifact.path);
    let git = ProcessRequest::new("git", ["show", artifact_spec.as_str()], root);
    let artifact = match process.run(&git) {
        Ok(output) if output.status == 0 => output.stdout.into_bytes(),
        Ok(output) => {
            return Outcome::Finding(Finding::new(format!(
                "Plan artifact `{artifact_spec}` does not resolve (exit {}): {}",
                output.status, output.stderr
            )))
        }
        Err(error) => return Outcome::CouldNotCheck(CouldNotCheck::new(error)),
    };
    let current = match read(&root.join(&manifest.artifact.path)) {
        Ok(current) => current,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    let published = match read(&root.join(&manifest.published_file)) {
        Ok(published) => published,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    match validate_plan(&manifest, claim, &artifact, &current, &published) {
        Ok(()) => {
            println!(
                "v0.13 Plan resolved: cid={} subject={} artifact={artifact_spec} sha256={}",
                manifest.cid, manifest.subject, manifest.artifact.sha256
            );
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn validate_plan(
    manifest: &PlanManifest,
    claim: &KanPlanClaim,
    artifact: &[u8],
    current: &[u8],
    published: &[u8],
) -> Result<(), String> {
    if manifest.schema != 1 {
        return Err("Plan resolver manifest has unsupported schema".into());
    }
    full_sha("Plan artifact commit", &manifest.artifact.commit)?;
    if manifest.subject != "v0.13-workflow-ergonomics"
        || manifest.rfc_result != "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua"
        || manifest.normative_source
            != "35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md"
        || manifest.artifact.path != ".design/v0.13-workflow-ergonomics.md"
    {
        return Err("Plan resolver manifest changed a closed v0.13 identity coordinate".into());
    }
    if claim.cid != manifest.cid
        || claim.kind != "Plan"
        || claim.subject != manifest.subject
        || !claim.cites.contains(&manifest.rfc_result)
        || !claim
            .text
            .as_deref()
            .is_some_and(|text| text.contains(&manifest.normative_source))
    {
        return Err(
            "selected claim does not match the Plan CID/subject/RFC/source contract".into(),
        );
    }
    let commit = format!("Commit(\"{}\")", manifest.artifact.commit);
    let file = format!(
        "FileAt(\"{}\", \"{}\")",
        manifest.artifact.path, manifest.artifact.commit
    );
    if !claim.artifacts.contains(&commit) || !claim.artifacts.contains(&file) {
        return Err("selected Plan does not carry the exact commit and FileAt artifacts".into());
    }
    if digest(artifact) != manifest.artifact.sha256 {
        return Err("resolved Plan artifact digest differs from the manifest".into());
    }
    if artifact != current {
        return Err(
            "current compatibility mirror is not byte-identical to the Plan artifact".into(),
        );
    }
    if Some(artifact) != claim.text.as_deref().map(str::as_bytes) {
        return Err("Plan narrative is not byte-identical to its addressed artifact".into());
    }
    if !String::from_utf8_lossy(published).contains(&manifest.cid) {
        return Err("tracked published claim file does not contain the selected Plan CID".into());
    }
    Ok(())
}

const ASKME_PROTOCOL: &str = ".release/protocols/askme-v1.json";
const RECONSTRUCTION_PROTOCOL: &str = ".release/protocols/reconstruction-v1.json";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AskmeProtocol {
    schema: u64,
    id: String,
    rubric_version: String,
    skill: String,
    scenarios: Vec<AskmeProtocolScenario>,
    global_checks: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AskmeProtocolScenario {
    id: String,
    topic: String,
    turns: Vec<String>,
    adaptive_signals: Vec<Vec<String>>,
    expect: AskmeExpectation,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AskmeExpectation {
    adaptive_follow_up: bool,
    record: bool,
    classifications: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AskmeManifest {
    schema: u64,
    candidate_sha: String,
    protocol_sha256: String,
    harness: String,
    harness_version: String,
    model: String,
    scenarios: Vec<EvidenceFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceFile {
    id: String,
    path: String,
    sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AskmeEvidence {
    schema: u64,
    id: String,
    candidate_sha: String,
    user_turns: Vec<String>,
    assistant_turns: Vec<String>,
    commands: Vec<Vec<String>>,
    claims_before: u64,
    claims_after: u64,
    durable_claim_texts: Vec<String>,
}

pub fn grade_askme(root: &Path, bundle: &Path) -> Outcome<()> {
    match grade_askme_inner(root, bundle) {
        Ok(count) => {
            println!(
                "ASKME MATERIAL: {count} scenario(s) derived cleanly from addressed raw evidence"
            );
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn grade_askme_inner(root: &Path, bundle: &Path) -> Result<usize, String> {
    let protocol_path = root.join(ASKME_PROTOCOL);
    let protocol_bytes = read(&protocol_path)?;
    let protocol: AskmeProtocol = json(&protocol_path, &protocol_bytes)?;
    if protocol.schema != 1
        || protocol.id != "day-v0.13-askme-v1"
        || protocol.rubric_version != "askme-rubric-v1"
        || protocol.skill != "skills/askme/SKILL.md"
        || protocol.global_checks.len() != 8
    {
        return Err(
            "askme protocol identity or closed rubric is not the registered v1 shape".into(),
        );
    }
    let bundle = root.join(bundle);
    let manifest_path = bundle.join("manifest.json");
    let manifest: AskmeManifest = json(&manifest_path, &read(&manifest_path)?)?;
    full_sha("candidate SHA", &manifest.candidate_sha)?;
    if manifest.schema != 1
        || manifest.harness.trim().is_empty()
        || manifest.harness_version.trim().is_empty()
        || manifest.model.trim().is_empty()
    {
        return Err("askme manifest metadata is incomplete".into());
    }
    if manifest.protocol_sha256 != digest(&protocol_bytes) {
        return Err("askme protocol digest does not address the committed protocol".into());
    }
    if manifest.scenarios.len() != protocol.scenarios.len() {
        return Err("askme evidence does not cover the exact preregistered scenario set".into());
    }

    for expected in &protocol.scenarios {
        if expected.topic.trim().is_empty() != (expected.id == "unknown-topic") {
            return Err(format!(
                "askme scenario `{}` has the wrong topic-presence contract",
                expected.id
            ));
        }
        let addressed = manifest
            .scenarios
            .iter()
            .find(|entry| entry.id == expected.id)
            .ok_or_else(|| format!("missing askme scenario `{}`", expected.id))?;
        safe_relative(&addressed.path)?;
        let path = bundle.join(&addressed.path);
        let bytes = read(&path)?;
        if digest(&bytes) != addressed.sha256 {
            return Err(format!(
                "digest mismatch for askme scenario `{}`",
                expected.id
            ));
        }
        let evidence: AskmeEvidence = json(&path, &bytes)?;
        grade_askme_scenario(expected, &manifest, &evidence)?;
    }
    Ok(protocol.scenarios.len())
}

fn grade_askme_scenario(
    expected: &AskmeProtocolScenario,
    manifest: &AskmeManifest,
    evidence: &AskmeEvidence,
) -> Result<(), String> {
    let fail = |reason: &str| Err(format!("askme scenario `{}`: {reason}", expected.id));
    if evidence.schema != 1
        || evidence.id != expected.id
        || evidence.candidate_sha != manifest.candidate_sha
        || evidence.user_turns != expected.turns
    {
        return fail("identity, candidate, or scripted user turns differ from the protocol");
    }
    if evidence.assistant_turns.len() != evidence.user_turns.len() + 1 {
        return fail("does not contain one observed assistant response per interaction step");
    }
    if evidence
        .assistant_turns
        .iter()
        .any(|turn| turn.matches('?').count() > 1)
    {
        return fail("an assistant turn asked more than one question");
    }
    for (index, signals) in expected.adaptive_signals.iter().enumerate() {
        if signals.is_empty() {
            continue;
        }
        let next = evidence.assistant_turns[index + 1].to_ascii_lowercase();
        if !signals
            .iter()
            .any(|signal| next.contains(&signal.to_ascii_lowercase()))
        {
            return fail("a preregistered follow-up did not address the preceding answer");
        }
    }
    if expected.expect.adaptive_follow_up && expected.adaptive_signals.is_empty() {
        return fail("claims adaptive behavior without a preregistered observable signal");
    }
    if expected
        .turns
        .iter()
        .any(|turn| turn.eq_ignore_ascii_case("stop"))
    {
        if evidence
            .assistant_turns
            .last()
            .is_some_and(|turn| turn.contains('?'))
        {
            return fail("continued questioning after stop");
        }
    } else {
        let consent = &evidence.assistant_turns[evidence.assistant_turns.len() - 2];
        if !consent.to_ascii_lowercase().contains("record") || !consent.contains('?') {
            return fail("did not ask explicit recording consent after the summary");
        }
        for bucket in &expected.expect.classifications {
            if !consent.to_ascii_lowercase().contains(bucket) {
                return fail("the consent summary omitted a required classification bucket");
            }
        }
    }
    let record_commands = evidence
        .commands
        .iter()
        .filter(|argv| {
            argv.windows(2)
                .any(|pair| pair == ["acquired-input", "record"])
        })
        .count();
    if expected.expect.record {
        if record_commands != 1 || evidence.claims_after <= evidence.claims_before {
            return fail(
                "explicit-yes branch did not produce exactly one recording command and append",
            );
        }
    } else if record_commands != 0 || evidence.claims_after != evidence.claims_before {
        return fail("non-consent branch mutated the claim log");
    }
    for text in &evidence.durable_claim_texts {
        let lowered = text.to_ascii_lowercase();
        let labels = ["human:", "user:", "assistant:", "agent:"]
            .iter()
            .filter(|label| lowered.contains(**label))
            .count();
        if labels >= 2 {
            return fail("durable claim content contains a raw multi-speaker transcript");
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconstructionEvidence {
    schema: u64,
    candidate_sha: String,
    behavioral_candidate_sha: String,
    publication_candidate_sha: String,
    stream_subject: String,
    acquired_input_cid: String,
    intervention_cid: String,
    handoff_cid: String,
    wakeup_evidence_path: String,
    kan_read_path: String,
    suite_commit: String,
    census_base: String,
    census_head: String,
    ci_run_id: u64,
    ci_head_sha: String,
    fresh_wakeup_had_transcript: bool,
}

pub fn grade_reconstruction(root: &Path, bundle: &Path) -> Outcome<()> {
    let protocol_path = root.join(RECONSTRUCTION_PROTOCOL);
    let protocol: serde_json::Value =
        match read(&protocol_path).and_then(|bytes| json(&protocol_path, &bytes)) {
            Ok(protocol) => protocol,
            Err(error) => return Outcome::Finding(Finding::new(error)),
        };
    let registered_controls = [
        "remove-stream",
        "remove-acquired-input",
        "remove-intervention",
        "remove-handoff",
        "remove-wakeup",
        "remove-suite-commit",
        "remove-census-base",
        "remove-census-head",
        "remove-ci-run-id",
        "remove-ci-head-sha",
        "change-candidate-sha",
    ];
    let declared_controls = protocol
        .get("removal_controls")
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
        });
    if protocol.get("schema").and_then(serde_json::Value::as_u64) != Some(1)
        || protocol.get("id").and_then(serde_json::Value::as_str)
            != Some("day-v0.13-reconstruction-v1")
        || declared_controls.as_deref() != Some(registered_controls.as_slice())
    {
        return Outcome::Finding(Finding::new(
            "reconstruction protocol identity or removal-control set differs from the registered v1 contract",
        ));
    }
    let bundle = root.join(bundle);
    let manifest_path = bundle.join("manifest.json");
    let evidence: ReconstructionEvidence =
        match read(&manifest_path).and_then(|bytes| json(&manifest_path, &bytes)) {
            Ok(evidence) => evidence,
            Err(error) => return Outcome::Finding(Finding::new(error)),
        };
    if let Err(error) = validate_reconstruction(&bundle, &evidence) {
        return Outcome::Finding(Finding::new(error));
    }

    let controls = reconstruction_controls(&bundle, &evidence);
    if let Some(survived) = controls.iter().find(|(_, rejected)| !rejected) {
        return Outcome::Finding(Finding::new(format!(
            "reconstruction removal control `{}` survived",
            survived.0
        )));
    }
    println!(
        "RECONSTRUCTION MATERIAL: exact candidate bound; {} removal control(s) rejected",
        controls.len()
    );
    Outcome::Passed(())
}

fn validate_reconstruction(bundle: &Path, evidence: &ReconstructionEvidence) -> Result<(), String> {
    if evidence.schema != 1 {
        return Err("reconstruction manifest has unsupported schema".into());
    }
    for (field, sha) in [
        ("candidate SHA", &evidence.candidate_sha),
        (
            "behavioral candidate SHA",
            &evidence.behavioral_candidate_sha,
        ),
        (
            "publication candidate SHA",
            &evidence.publication_candidate_sha,
        ),
        ("suite commit", &evidence.suite_commit),
        ("census base", &evidence.census_base),
        ("census head", &evidence.census_head),
        ("CI head SHA", &evidence.ci_head_sha),
    ] {
        full_sha(field, sha)?;
    }
    if evidence.candidate_sha != evidence.behavioral_candidate_sha
        || evidence.candidate_sha != evidence.publication_candidate_sha
    {
        return Err(
            "behavioral, reconstruction, and publication candidate coordinates differ".into(),
        );
    }
    for (field, value) in [
        ("stream subject", &evidence.stream_subject),
        ("acquired-input CID", &evidence.acquired_input_cid),
        ("intervention CID", &evidence.intervention_cid),
        ("handoff CID", &evidence.handoff_cid),
        ("wakeup evidence path", &evidence.wakeup_evidence_path),
        ("kan read path", &evidence.kan_read_path),
    ] {
        if value.trim().is_empty() {
            return Err(format!("{field} is missing"));
        }
    }
    if evidence.ci_run_id == 0 {
        return Err("CI run ID is missing".into());
    }
    if evidence.fresh_wakeup_had_transcript {
        return Err("fresh wakeup received the raw transcript".into());
    }
    safe_relative(&evidence.wakeup_evidence_path)?;
    safe_relative(&evidence.kan_read_path)?;
    let wakeup = String::from_utf8(read(&bundle.join(&evidence.wakeup_evidence_path))?)
        .map_err(|error| format!("wakeup evidence is not UTF-8: {error}"))?;
    for required in [
        evidence.stream_subject.as_str(),
        evidence.acquired_input_cid.as_str(),
        evidence.intervention_cid.as_str(),
        evidence.handoff_cid.as_str(),
        evidence.suite_commit.as_str(),
        evidence.census_base.as_str(),
        evidence.census_head.as_str(),
        &evidence.ci_run_id.to_string(),
        evidence.ci_head_sha.as_str(),
    ] {
        if !wakeup.contains(required) {
            return Err(format!(
                "fresh wakeup evidence does not reconstruct `{required}`"
            ));
        }
    }
    let kan_read = String::from_utf8(read(&bundle.join(&evidence.kan_read_path))?)
        .map_err(|error| format!("kan evidence is not UTF-8: {error}"))?;
    for cid in [
        &evidence.acquired_input_cid,
        &evidence.intervention_cid,
        &evidence.handoff_cid,
    ] {
        if !kan_read.contains(cid) {
            return Err(format!("bulk kan evidence does not contain `{cid}`"));
        }
    }
    Ok(())
}

fn reconstruction_controls(
    bundle: &Path,
    evidence: &ReconstructionEvidence,
) -> Vec<(&'static str, bool)> {
    type ReconstructionMutation = Box<dyn Fn(&mut ReconstructionEvidence)>;
    let mutations: Vec<(&str, ReconstructionMutation)> = vec![
        (
            "remove-stream",
            Box::new(|value| value.stream_subject.clear()),
        ),
        (
            "remove-acquired-input",
            Box::new(|value| value.acquired_input_cid.clear()),
        ),
        (
            "remove-intervention",
            Box::new(|value| value.intervention_cid.clear()),
        ),
        (
            "remove-handoff",
            Box::new(|value| value.handoff_cid.clear()),
        ),
        (
            "remove-wakeup",
            Box::new(|value| value.wakeup_evidence_path.clear()),
        ),
        (
            "remove-suite-commit",
            Box::new(|value| value.suite_commit.clear()),
        ),
        (
            "remove-census-base",
            Box::new(|value| value.census_base.clear()),
        ),
        (
            "remove-census-head",
            Box::new(|value| value.census_head.clear()),
        ),
        ("remove-ci-run-id", Box::new(|value| value.ci_run_id = 0)),
        (
            "remove-ci-head-sha",
            Box::new(|value| value.ci_head_sha.clear()),
        ),
        (
            "change-candidate-sha",
            Box::new(|value| value.candidate_sha = "f".repeat(40)),
        ),
    ];
    mutations
        .into_iter()
        .map(|(name, mutate)| {
            let mut changed = evidence.clone();
            mutate(&mut changed);
            (name, validate_reconstruction(bundle, &changed).is_err())
        })
        .collect()
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|error| format!("could not read `{}`: {error}", path.display()))
}

fn json<T: serde::de::DeserializeOwned>(path: &Path, bytes: &[u8]) -> Result<T, String> {
    serde_json::from_slice(bytes)
        .map_err(|error| format!("`{}` is malformed: {error}", path.display()))
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn safe_relative(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        Err(format!(
            "evidence path `{}` escapes its bundle",
            path.display()
        ))
    } else {
        Ok(())
    }
}

fn full_sha(field: &str, value: &str) -> Result<(), String> {
    if value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(format!("{field} must be a full 40-hex commit SHA"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_or_adding_any_contract_member_is_a_mismatch() {
        let expected = contract();

        for index in 0..expected.issues.len() {
            let mut changed = expected.clone();
            changed.issues.remove(index);
            assert_ne!(changed, expected);
        }
        for index in 0..expected.workflows.len() {
            let mut changed = expected.clone();
            changed.workflows.remove(index);
            assert_ne!(changed, expected);
        }
        for index in 0..expected.evidence_protocols.len() {
            let mut changed = expected.clone();
            changed.evidence_protocols.remove(index);
            assert_ne!(changed, expected);
        }
        for index in 0..expected.publication_artifacts.len() {
            let mut changed = expected.clone();
            changed.publication_artifacts.remove(index);
            assert_ne!(changed, expected);
        }
        let mut changed = expected.clone();
        changed.issues.push(999);
        assert_ne!(changed, expected);
        let mut changed = expected.clone();
        changed.workflows.push("extra.yml".into());
        assert_ne!(changed, expected);
        let mut changed = expected.clone();
        changed.evidence_protocols.push("extra.json".into());
        assert_ne!(changed, expected);
        let mut changed = expected.clone();
        changed.publication_artifacts.push("extra".into());
        assert_ne!(changed, expected);
    }

    #[test]
    fn askme_grader_derives_outcomes_and_rejects_a_mutated_transcript() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let protocol_path = root.join(ASKME_PROTOCOL);
        let protocol_bytes = std::fs::read(&protocol_path).unwrap();
        let protocol: AskmeProtocol = serde_json::from_slice(&protocol_bytes).unwrap();
        let bundle = tempfile::tempdir().unwrap();
        let candidate = "a".repeat(40);
        let mut files = Vec::new();

        for scenario in &protocol.scenarios {
            let mut assistant_turns = vec!["What outcome would make this useful?".to_string()];
            for signals in &scenario.adaptive_signals {
                let signal = signals.first().map(String::as_str).unwrap_or("skipped");
                assistant_turns.push(format!("You said {signal}. What should we consider next?"));
            }
            while assistant_turns.len() < scenario.turns.len() + 1 {
                assistant_turns.push("Summary complete.".into());
            }
            if scenario
                .turns
                .iter()
                .any(|turn| turn.eq_ignore_ascii_case("stop"))
            {
                *assistant_turns.last_mut().unwrap() = "Stopped without recording.".into();
            } else {
                let buckets = scenario.expect.classifications.join(" ");
                let index = assistant_turns.len() - 2;
                let signals = scenario
                    .adaptive_signals
                    .last()
                    .and_then(|values| values.first())
                    .cloned()
                    .unwrap_or_default();
                assistant_turns[index] =
                    format!("{signals} fact decision unresolved material-effect {buckets}. Record this now?");
                *assistant_turns.last_mut().unwrap() = "Finished.".into();
            }
            let evidence = serde_json::json!({
                "schema": 1,
                "id": scenario.id,
                "candidate_sha": candidate,
                "user_turns": scenario.turns,
                "assistant_turns": assistant_turns,
                "commands": if scenario.expect.record { serde_json::json!([["day", "acquired-input", "record"]]) } else { serde_json::json!([]) },
                "claims_before": 2,
                "claims_after": if scenario.expect.record { 3 } else { 2 },
                "durable_claim_texts": ["structured summary only"]
            });
            let path = bundle.path().join(format!("{}.json", scenario.id));
            std::fs::write(&path, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
            files.push(serde_json::json!({
                "id": scenario.id,
                "path": path.file_name().unwrap().to_str().unwrap(),
                "sha256": digest(&std::fs::read(&path).unwrap())
            }));
        }
        let manifest = serde_json::json!({
            "schema": 1,
            "candidate_sha": candidate,
            "protocol_sha256": digest(&protocol_bytes),
            "harness": "fixture",
            "harness_version": "1",
            "model": "fixture-model",
            "scenarios": files
        });
        std::fs::write(
            bundle.path().join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(grade_askme_inner(root, bundle.path()).is_ok());

        let first = &protocol.scenarios[0];
        let path = bundle.path().join(format!("{}.json", first.id));
        let mut evidence: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        evidence["assistant_turns"][1] = "What should we consider next?".into();
        std::fs::write(&path, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
        let mut manifest: serde_json::Value =
            serde_json::from_slice(&std::fs::read(bundle.path().join("manifest.json")).unwrap())
                .unwrap();
        manifest["scenarios"][0]["sha256"] = digest(&std::fs::read(&path).unwrap()).into();
        std::fs::write(
            bundle.path().join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).unwrap(),
        )
        .unwrap();
        assert!(grade_askme_inner(root, bundle.path())
            .unwrap_err()
            .contains("did not address"));
    }

    #[test]
    fn reconstruction_bundle_passes_only_with_every_coordinate_and_claim() {
        let bundle = tempfile::tempdir().unwrap();
        let candidate = "a".repeat(40);
        let base = "b".repeat(40);
        let head = "c".repeat(40);
        let ci = "d".repeat(40);
        let wakeup = format!(
            "agents/handoff/main cid-input cid-intervention cid-handoff {candidate} {base} {head} 42 {ci}"
        );
        std::fs::write(bundle.path().join("wakeup.txt"), wakeup).unwrap();
        std::fs::write(
            bundle.path().join("kan.json"),
            "cid-input cid-intervention cid-handoff",
        )
        .unwrap();
        let evidence = ReconstructionEvidence {
            schema: 1,
            candidate_sha: candidate.clone(),
            behavioral_candidate_sha: candidate.clone(),
            publication_candidate_sha: candidate.clone(),
            stream_subject: "agents/handoff/main".into(),
            acquired_input_cid: "cid-input".into(),
            intervention_cid: "cid-intervention".into(),
            handoff_cid: "cid-handoff".into(),
            wakeup_evidence_path: "wakeup.txt".into(),
            kan_read_path: "kan.json".into(),
            suite_commit: candidate,
            census_base: base,
            census_head: head,
            ci_run_id: 42,
            ci_head_sha: ci,
            fresh_wakeup_had_transcript: false,
        };
        assert!(validate_reconstruction(bundle.path(), &evidence).is_ok());
        assert!(reconstruction_controls(bundle.path(), &evidence)
            .iter()
            .all(|(_, rejected)| *rejected));
    }
}
