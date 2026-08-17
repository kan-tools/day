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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowRun {
    database_id: u64,
    head_sha: String,
    status: String,
    conclusion: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GithubRelease {
    tag_name: String,
    is_draft: bool,
}

#[derive(Debug, Deserialize)]
struct CratesResponse {
    version: CratesVersion,
}

#[derive(Debug, Deserialize)]
struct CratesVersion {
    num: String,
}

pub fn verify_candidate(root: &Path, candidate: &str, process: &dyn Process) -> Outcome<()> {
    match verify_candidate_inner(root, candidate, process) {
        Ok(runs) => {
            println!(
                "v0.13 candidate verified: {candidate}; {} required workflow run(s) succeeded",
                runs.len()
            );
            for (workflow, run_id) in runs {
                println!("  {workflow}: run {run_id}");
            }
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn verify_candidate_inner(
    root: &Path,
    candidate: &str,
    process: &dyn Process,
) -> Result<Vec<(&'static str, u64)>, String> {
    full_sha("candidate SHA", candidate)?;
    let head = checked(process, root, "git", ["rev-parse", "HEAD"], "local HEAD")?;
    if head.trim() != candidate {
        return Err(format!(
            "local HEAD {} differs from candidate {candidate}",
            head.trim()
        ));
    }
    let status = checked(
        process,
        root,
        "git",
        ["status", "--porcelain"],
        "working-tree status",
    )?;
    if !status.trim().is_empty() {
        return Err("candidate working tree is dirty".into());
    }
    workflow_runs(root, candidate, WORKFLOWS, process)
}

pub fn verify_publication(root: &Path, candidate: &str, process: &dyn Process) -> Outcome<()> {
    match verify_publication_inner(root, candidate, process) {
        Ok(release_run) => {
            println!(
                "v0.13 publication verified: tag={VERSION} candidate={candidate} release-run={release_run} crate={} GitHub-Release=present kan-claim=present",
                VERSION.trim_start_matches('v')
            );
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn verify_publication_inner(
    root: &Path,
    candidate: &str,
    process: &dyn Process,
) -> Result<u64, String> {
    full_sha("candidate SHA", candidate)?;
    let tag_target = checked(
        process,
        root,
        "git",
        ["rev-list", "-n", "1", VERSION],
        "release tag",
    )?;
    if tag_target.trim() != candidate {
        return Err(format!(
            "release tag {VERSION} targets {}, not candidate {candidate}",
            tag_target.trim()
        ));
    }
    let release_runs = workflow_runs(root, candidate, &[".github/workflows/release.yml"], process)?;
    let release_run = release_runs[0].1;

    let release_json = checked(
        process,
        root,
        "gh",
        [
            "release",
            "view",
            VERSION,
            "--repo",
            "kan-tools/day",
            "--json",
            "tagName,isDraft",
        ],
        "GitHub Release",
    )?;
    let release: GithubRelease = serde_json::from_str(&release_json)
        .map_err(|error| format!("GitHub Release response is malformed: {error}"))?;
    if release.tag_name != VERSION || release.is_draft {
        return Err("GitHub Release is absent, draft, or names a different tag".into());
    }

    let version = VERSION.trim_start_matches('v');
    let crate_url = format!("https://crates.io/api/v1/crates/day/{version}");
    let crate_json = checked(
        process,
        root,
        "curl",
        ["--fail", "--silent", "--show-error", crate_url.as_str()],
        "crates.io package",
    )?;
    let published: CratesResponse = serde_json::from_str(&crate_json)
        .map_err(|error| format!("crates.io response is malformed: {error}"))?;
    if published.version.num != version {
        return Err("crates.io returned a different package version".into());
    }

    let claim_json = checked(
        process,
        root,
        "kan",
        ["show", "release", "--json"],
        "kan release claim",
    )?;
    let claims: KanShow = serde_json::from_str(&claim_json)
        .map_err(|error| format!("kan release response is malformed: {error}"))?;
    let claim_matches = claims.claims.iter().any(|claim| {
        claim.kind == "Result"
            && claim
                .text
                .as_deref()
                .is_some_and(|text| text.contains(VERSION) && text.contains(candidate))
    });
    if !claim_matches {
        return Err("kan has no release Result binding the tag to the candidate SHA".into());
    }
    Ok(release_run)
}

fn workflow_runs(
    root: &Path,
    candidate: &str,
    workflows: &[&'static str],
    process: &dyn Process,
) -> Result<Vec<(&'static str, u64)>, String> {
    let mut verified = Vec::with_capacity(workflows.len());
    for workflow in workflows {
        let output = checked(
            process,
            root,
            "gh",
            [
                "run",
                "list",
                "--repo",
                "kan-tools/day",
                "--workflow",
                workflow,
                "--commit",
                candidate,
                "--limit",
                "100",
                "--json",
                "databaseId,headSha,status,conclusion",
            ],
            &format!("workflow {workflow}"),
        )?;
        let runs: Vec<WorkflowRun> = serde_json::from_str(&output)
            .map_err(|error| format!("workflow `{workflow}` response is malformed: {error}"))?;
        let run = runs.iter().find(|run| {
            run.database_id != 0
                && run.head_sha == candidate
                && run.status == "completed"
                && run.conclusion == "success"
        });
        match run {
            Some(run) => verified.push((*workflow, run.database_id)),
            None => {
                return Err(format!(
                    "workflow `{workflow}` has no completed successful run at candidate {candidate}"
                ))
            }
        }
    }
    Ok(verified)
}

fn checked<I, S>(
    process: &dyn Process,
    root: &Path,
    program: &str,
    args: I,
    label: &str,
) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let output = process
        .run(&ProcessRequest::new(program, args, root))
        .map_err(|error| format!("could not resolve {label}: {error}"))?;
    if output.status != 0 {
        Err(format!(
            "could not resolve {label} (exit {}): {}",
            output.status,
            output.stderr.trim()
        ))
    } else {
        Ok(output.stdout)
    }
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
    raw_events: Vec<AddressedArtifact>,
    kan_before: AddressedArtifact,
    kan_after: AddressedArtifact,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AddressedArtifact {
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
        || protocol.global_checks.len() != 10
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
        grade_raw_askme_artifacts(&bundle, addressed, &evidence)?;
    }
    Ok(protocol.scenarios.len())
}

fn grade_raw_askme_artifacts(
    bundle: &Path,
    addressed: &EvidenceFile,
    evidence: &AskmeEvidence,
) -> Result<(), String> {
    if addressed.raw_events.len() != evidence.assistant_turns.len() {
        return Err(format!(
            "askme scenario `{}` does not address one raw event log per assistant turn",
            evidence.id
        ));
    }
    for (index, (artifact, turn)) in addressed
        .raw_events
        .iter()
        .zip(&evidence.assistant_turns)
        .enumerate()
    {
        let bytes = addressed_bytes(bundle, artifact, "raw Codex event log")?;
        let raw = std::str::from_utf8(&bytes).map_err(|error| {
            format!(
                "askme scenario `{}` raw event log {index} is not UTF-8: {error}",
                evidence.id
            )
        })?;
        let mut values = Vec::new();
        for (line_index, line) in raw.lines().enumerate() {
            let value: serde_json::Value = serde_json::from_str(line).map_err(|error| {
                format!(
                    "askme scenario `{}` raw event log {index} line {} is malformed: {error}",
                    evidence.id,
                    line_index + 1
                )
            })?;
            values.push(value);
        }
        if values.is_empty()
            || !values
                .iter()
                .any(|value| json_contains_text(value, turn.trim()))
        {
            return Err(format!(
                "askme scenario `{}` assistant turn {index} is not present in its addressed raw event log",
                evidence.id
            ));
        }
    }
    let before = addressed_bytes(bundle, &addressed.kan_before, "kan-before snapshot")?;
    let after = addressed_bytes(bundle, &addressed.kan_after, "kan-after snapshot")?;
    let before = kan_snapshot_counts(&before)?;
    let after = kan_snapshot_counts(&after)?;
    if before.0 != evidence.claims_before
        || after.0 != evidence.claims_after
        || after.1 != evidence.durable_claim_texts
    {
        return Err(format!(
            "askme scenario `{}` synthesized claim counts/texts differ from addressed kan snapshots",
            evidence.id
        ));
    }
    Ok(())
}

fn addressed_bytes(
    bundle: &Path,
    artifact: &AddressedArtifact,
    label: &str,
) -> Result<Vec<u8>, String> {
    safe_relative(&artifact.path)?;
    let bytes = read(&bundle.join(&artifact.path))?;
    if digest(&bytes) != artifact.sha256 {
        return Err(format!("{label} digest differs from its manifest address"));
    }
    Ok(bytes)
}

fn json_contains_text(value: &serde_json::Value, expected: &str) -> bool {
    match value {
        serde_json::Value::String(value) => value.trim() == expected,
        serde_json::Value::Array(values) => values
            .iter()
            .any(|value| json_contains_text(value, expected)),
        serde_json::Value::Object(values) => values
            .values()
            .any(|value| json_contains_text(value, expected)),
        _ => false,
    }
}

fn kan_snapshot_counts(bytes: &[u8]) -> Result<(u64, Vec<String>), String> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("kan snapshot is malformed: {error}"))?;
    let subjects = value
        .get("subjects")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "kan snapshot has no subjects array".to_string())?;
    let mut cids = std::collections::BTreeSet::new();
    let mut texts = Vec::new();
    for subject in subjects {
        let claims = subject
            .get("claims")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "kan snapshot subject has no claims array".to_string())?;
        for claim in claims {
            if let Some(cid) = claim.get("cid").and_then(serde_json::Value::as_str) {
                cids.insert(cid.to_owned());
            }
            if let Some(text) = claim.get("text").and_then(serde_json::Value::as_str) {
                texts.push(text.to_owned());
            }
        }
    }
    Ok((cids.len() as u64, texts))
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FreshWakeupEvidence {
    schema: u64,
    session_kind: String,
    raw_transcript_supplied: bool,
    kan_command: Vec<String>,
    stream_subject: String,
    claims_read: Vec<String>,
    candidate_sha: String,
    rendered_context: String,
}

#[derive(Debug, Deserialize)]
struct BulkKanRead {
    v: u64,
    excluded_by_trust: u64,
    subjects: Vec<BulkKanSubject>,
}

#[derive(Debug, Deserialize)]
struct BulkKanSubject {
    subject: String,
    excluded_by_trust: u64,
    claims: Vec<day::kan_client::Claim>,
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
        || evidence.candidate_sha != evidence.suite_commit
        || evidence.candidate_sha != evidence.census_head
        || evidence.candidate_sha != evidence.ci_head_sha
    {
        return Err(
            "behavioral, reconstruction, publication, suite, census, and CI candidate coordinates differ".into(),
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
    let wakeup_path = bundle.join(&evidence.wakeup_evidence_path);
    let wakeup: FreshWakeupEvidence = json(&wakeup_path, &read(&wakeup_path)?)?;
    validate_fresh_wakeup(&wakeup, evidence)?;

    let kan_path = bundle.join(&evidence.kan_read_path);
    let kan_read: BulkKanRead = json(&kan_path, &read(&kan_path)?)?;
    validate_bulk_reconstruction(&kan_read, evidence)?;
    Ok(())
}

fn validate_fresh_wakeup(
    wakeup: &FreshWakeupEvidence,
    evidence: &ReconstructionEvidence,
) -> Result<(), String> {
    if wakeup.schema != 1
        || wakeup.session_kind != "fresh"
        || wakeup.raw_transcript_supplied
        || wakeup.kan_command != ["kan", "show", "--all", "--json"]
        || wakeup.stream_subject != evidence.stream_subject
        || wakeup.candidate_sha != evidence.candidate_sha
    {
        return Err("fresh wakeup evidence has the wrong typed invocation or identity".into());
    }
    let expected = [
        evidence.acquired_input_cid.as_str(),
        evidence.intervention_cid.as_str(),
        evidence.handoff_cid.as_str(),
    ]
    .into_iter()
    .collect::<std::collections::BTreeSet<_>>();
    let actual = wakeup
        .claims_read
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    if actual != expected {
        return Err("fresh wakeup did not report the exact required claim set".into());
    }
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
        if !wakeup.rendered_context.contains(required) {
            return Err(format!(
                "fresh wakeup rendered context does not reconstruct `{required}`"
            ));
        }
    }
    Ok(())
}

fn validate_bulk_reconstruction(
    read: &BulkKanRead,
    evidence: &ReconstructionEvidence,
) -> Result<(), String> {
    if read.v != 1 || read.excluded_by_trust != 0 {
        return Err("bulk kan evidence is unsupported or narrowed by trust".into());
    }
    if read
        .subjects
        .iter()
        .any(|subject| subject.excluded_by_trust != 0)
    {
        return Err("bulk kan evidence contains a partially withheld subject".into());
    }
    let acquired = unique_claim(read, &evidence.acquired_input_cid)?;
    let intervention = unique_claim(read, &evidence.intervention_cid)?;
    let handoff = unique_claim(read, &evidence.handoff_cid)?;
    validate_event_claim::<day::events::AcquiredInput>(acquired.1, "day-acquired-input")?;
    validate_event_claim::<day::events::Intervention>(intervention.1, "day-intervention")?;
    if handoff.0 != evidence.stream_subject || handoff.1.kind != "Observation" {
        return Err("handoff CID is not an Observation on the declared stream subject".into());
    }
    let text = handoff
        .1
        .text
        .as_deref()
        .ok_or_else(|| "handoff claim has no narrative".to_string())?;
    let scopes = day::atoms::extract_fenced::<day::stream::HandoffScopes>(text)
        .ok_or_else(|| "handoff claim has no day-handoff-scopes block".to_string())?
        .map_err(|error| format!("handoff scope block is invalid: {error}"))?;
    if !scopes
        .suites
        .iter()
        .any(|scope| scope.commit == evidence.suite_commit && scope.tree_clean)
        || !scopes.censuses.iter().any(|scope| {
            scope.base == evidence.census_base
                && scope.head == evidence.census_head
                && scope.unaccounted == 0
        })
        || !scopes.ci.iter().any(|scope| {
            scope.run_id == evidence.ci_run_id
                && scope.head_sha == evidence.ci_head_sha
                && scope.conclusion == "success"
        })
    {
        return Err("handoff scopes do not bind the declared suite/census/CI coordinates".into());
    }
    Ok(())
}

fn unique_claim<'a>(
    read: &'a BulkKanRead,
    cid: &str,
) -> Result<(&'a str, &'a day::kan_client::Claim), String> {
    let mut found = read.subjects.iter().flat_map(|subject| {
        subject
            .claims
            .iter()
            .filter(move |claim| claim.cid == cid)
            .map(move |claim| (subject.subject.as_str(), claim))
    });
    let claim = found
        .next()
        .ok_or_else(|| format!("bulk kan evidence does not contain claim `{cid}`"))?;
    if found.next().is_some() {
        return Err(format!(
            "bulk kan evidence contains claim `{cid}` more than once"
        ));
    }
    Ok(claim)
}

fn validate_event_claim<T: day::atoms::Versioned + serde::de::DeserializeOwned>(
    claim: &day::kan_client::Claim,
    fence: &str,
) -> Result<(), String> {
    if claim.kind != "Observation" {
        return Err(format!("{fence} CID is not an Observation"));
    }
    let text = claim
        .text
        .as_deref()
        .ok_or_else(|| format!("{fence} claim has no narrative"))?;
    day::atoms::extract_fenced::<T>(text)
        .ok_or_else(|| format!("claim has no `{fence}` block"))?
        .map_err(|error| format!("`{fence}` block is invalid: {error}"))?;
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
            let turns_for_raw = assistant_turns.clone();
            let claims_after = if scenario.expect.record { 3 } else { 2 };
            let evidence = serde_json::json!({
                "schema": 1,
                "id": scenario.id,
                "candidate_sha": candidate,
                "user_turns": scenario.turns,
                "assistant_turns": assistant_turns,
                "commands": if scenario.expect.record { serde_json::json!([["day", "acquired-input", "record"]]) } else { serde_json::json!([]) },
                "claims_before": 2,
                "claims_after": claims_after,
                "durable_claim_texts": ["structured summary only"]
            });
            let path = bundle.path().join(format!("{}.json", scenario.id));
            std::fs::write(&path, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
            let mut raw_events = Vec::new();
            for (index, turn) in turns_for_raw.iter().enumerate() {
                let raw = bundle
                    .path()
                    .join(format!("{}-events-{index}.jsonl", scenario.id));
                std::fs::write(
                    &raw,
                    format!(
                        "{{\"event\":{{\"message\":{}}}}}\n",
                        serde_json::to_string(turn).unwrap()
                    ),
                )
                .unwrap();
                raw_events.push(serde_json::json!({
                    "path": raw.file_name().unwrap().to_str().unwrap(),
                    "sha256": digest(&std::fs::read(&raw).unwrap())
                }));
            }
            let before_path = bundle
                .path()
                .join(format!("{}-kan-before.json", scenario.id));
            let after_path = bundle
                .path()
                .join(format!("{}-kan-after.json", scenario.id));
            let before = serde_json::json!({"subjects": [{"claims": [
                {"cid": "cid-before-1"}, {"cid": "cid-before-2"}
            ]}]});
            let mut after_claims = vec![
                serde_json::json!({"cid": "cid-before-1", "text": "structured summary only"}),
                serde_json::json!({"cid": "cid-before-2"}),
            ];
            if claims_after == 3 {
                after_claims.push(serde_json::json!({"cid": "cid-after-3"}));
            }
            std::fs::write(&before_path, serde_json::to_vec(&before).unwrap()).unwrap();
            std::fs::write(
                &after_path,
                serde_json::to_vec(&serde_json::json!({"subjects": [{"claims": after_claims}]}))
                    .unwrap(),
            )
            .unwrap();
            files.push(serde_json::json!({
                "id": scenario.id,
                "path": path.file_name().unwrap().to_str().unwrap(),
                "sha256": digest(&std::fs::read(&path).unwrap()),
                "raw_events": raw_events,
                "kan_before": {
                    "path": before_path.file_name().unwrap().to_str().unwrap(),
                    "sha256": digest(&std::fs::read(&before_path).unwrap())
                },
                "kan_after": {
                    "path": after_path.file_name().unwrap().to_str().unwrap(),
                    "sha256": digest(&std::fs::read(&after_path).unwrap())
                }
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

        let manifest_value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(bundle.path().join("manifest.json")).unwrap())
                .unwrap();
        let raw_relative = manifest_value["scenarios"][0]["raw_events"][0]["path"]
            .as_str()
            .unwrap();
        let raw_path = bundle.path().join(raw_relative);
        let raw_original = std::fs::read(&raw_path).unwrap();
        std::fs::write(&raw_path, b"{}\n").unwrap();
        assert!(grade_askme_inner(root, bundle.path())
            .unwrap_err()
            .contains("raw Codex event log digest differs"));
        std::fs::write(&raw_path, raw_original).unwrap();

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
        let rendered = format!(
            "agents/handoff/main cid-input cid-intervention cid-handoff {candidate} {base} {candidate} 42 {candidate}"
        );
        let wakeup = serde_json::json!({
            "schema": 1,
            "session_kind": "fresh",
            "raw_transcript_supplied": false,
            "kan_command": ["kan", "show", "--all", "--json"],
            "stream_subject": "agents/handoff/main",
            "claims_read": ["cid-input", "cid-intervention", "cid-handoff"],
            "candidate_sha": candidate,
            "rendered_context": rendered,
        });
        std::fs::write(
            bundle.path().join("wakeup.json"),
            serde_json::to_vec_pretty(&wakeup).unwrap(),
        )
        .unwrap();
        let acquired = day::events::AcquiredInput {
            work_subject: "work/main".into(),
            topic: "release".into(),
            provider: day::events::Source::Recorder {
                principal: "did:key:zFixture".into(),
            },
            recorded_by: "did:key:zFixture".into(),
            facts: vec!["candidate exists".into()],
            decisions: vec![],
            unresolved: vec![],
            material_effect: "qualifies reconstruction".into(),
            basis: vec!["cid-basis".into()],
        };
        let intervention = day::events::Intervention {
            work_subject: "work/main".into(),
            kind: day::events::InterventionKind::Approval,
            summary: "operator approved the direction".into(),
            material_effect: "continued the release work".into(),
            source: day::events::Source::Recorder {
                principal: "did:key:zFixture".into(),
            },
            recorded_by: "did:key:zFixture".into(),
            basis: vec!["cid-basis".into()],
        };
        let scopes = serde_json::json!({
            "_version": 1,
            "suites": [{"argv": ["cargo", "test"], "commit": candidate, "tree_clean": true}],
            "censuses": [{"base": base, "head": candidate, "unaccounted": 0}],
            "ci": [{"provider": "github-actions", "workflow": "CI", "run_id": 42, "head_sha": candidate, "conclusion": "success"}],
        });
        let handoff_text = format!(
            "Scoped handoff.\n\n```day-handoff-scopes\n{}\n```\n",
            serde_json::to_string(&scopes).unwrap()
        );
        let kan = serde_json::json!({
            "v": 1,
            "excluded_by_trust": 0,
            "subjects": [
                {"subject": "work/main", "excluded_by_trust": 0, "claims": [
                    {"cid": "cid-input", "kind": "Observation", "text": acquired.to_claim_text()},
                    {"cid": "cid-intervention", "kind": "Observation", "text": intervention.to_claim_text()}
                ]},
                {"subject": "agents/handoff/main", "excluded_by_trust": 0, "claims": [
                    {"cid": "cid-handoff", "kind": "Observation", "text": handoff_text}
                ]}
            ]
        });
        std::fs::write(
            bundle.path().join("kan.json"),
            serde_json::to_vec_pretty(&kan).unwrap(),
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
            wakeup_evidence_path: "wakeup.json".into(),
            kan_read_path: "kan.json".into(),
            suite_commit: candidate,
            census_base: base,
            census_head: "a".repeat(40),
            ci_run_id: 42,
            ci_head_sha: "a".repeat(40),
            fresh_wakeup_had_transcript: false,
        };
        assert!(validate_reconstruction(bundle.path(), &evidence).is_ok());
        assert!(reconstruction_controls(bundle.path(), &evidence)
            .iter()
            .all(|(_, rejected)| *rejected));

        let wakeup_path = bundle.path().join("wakeup.json");
        let wakeup_bytes = std::fs::read(&wakeup_path).unwrap();
        std::fs::write(&wakeup_path, rendered.as_bytes()).unwrap();
        assert!(validate_reconstruction(bundle.path(), &evidence)
            .unwrap_err()
            .contains("malformed"));
        std::fs::write(&wakeup_path, wakeup_bytes).unwrap();

        let kan_path = bundle.path().join("kan.json");
        let kan_bytes = std::fs::read(&kan_path).unwrap();
        std::fs::write(
            &kan_path,
            b"cid-input cid-intervention cid-handoff agents/handoff/main",
        )
        .unwrap();
        assert!(validate_reconstruction(bundle.path(), &evidence)
            .unwrap_err()
            .contains("malformed"));
        std::fs::write(&kan_path, kan_bytes).unwrap();
    }
}
