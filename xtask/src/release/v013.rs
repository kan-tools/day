use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

pub const VERSION: &str = "v0.13.0-beta.1";
const V013_EVIDENCE_PRINCIPAL: &str = "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn";
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
    github_run_id: u64,
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
    command_log: AddressedArtifact,
    kan_before: AddressedArtifact,
    kan_after: AddressedArtifact,
}

#[derive(Debug, Clone, Deserialize)]
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

pub fn grade_askme(
    root: &Path,
    bundle: &Path,
    candidate_sha: &str,
    github_run_id: u64,
) -> Outcome<()> {
    if let Err(error) = github_actions_origin(
        "v0.13 askme behavioral trial",
        candidate_sha,
        Some(github_run_id),
    ) {
        return Outcome::Finding(Finding::new(error));
    }
    match grade_askme_inner(root, bundle, Some((candidate_sha, github_run_id))) {
        Ok(count) => {
            println!(
                "ASKME MATERIAL: {count} scenario(s) derived cleanly from addressed raw evidence"
            );
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

fn grade_askme_inner(
    root: &Path,
    bundle: &Path,
    workflow: Option<(&str, u64)>,
) -> Result<usize, String> {
    let protocol_path = root.join(ASKME_PROTOCOL);
    let protocol_bytes = read(&protocol_path)?;
    let protocol: AskmeProtocol = json(&protocol_path, &protocol_bytes)?;
    if protocol.schema != 1
        || protocol.id != "day-v0.13-askme-v1"
        || protocol.rubric_version != "askme-rubric-v1"
        || protocol.skill != "skills/askme/SKILL.md"
        || protocol.global_checks.len() != 12
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
        || manifest.harness != "codex-cli"
        || manifest.harness_version != "codex-cli 0.147.0"
        || manifest.model.trim().is_empty()
        || manifest.github_run_id == 0
    {
        return Err("askme manifest is not from the pinned Codex workflow harness".into());
    }
    if let Some((candidate_sha, github_run_id)) = workflow {
        full_sha("workflow candidate SHA", candidate_sha)?;
        if manifest.candidate_sha != candidate_sha || manifest.github_run_id != github_run_id {
            return Err(
                "askme bundle coordinates differ from the candidate and run supplied by the workflow"
                    .into(),
            );
        }
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
        grade_raw_askme_artifacts(&bundle, addressed, &evidence, expected.expect.record)?;
    }
    Ok(protocol.scenarios.len())
}

fn grade_raw_askme_artifacts(
    bundle: &Path,
    addressed: &EvidenceFile,
    evidence: &AskmeEvidence,
    expected_record: bool,
) -> Result<(), String> {
    if addressed.raw_events.len() != evidence.assistant_turns.len() {
        return Err(format!(
            "askme scenario `{}` does not address one raw event log per assistant turn",
            evidence.id
        ));
    }
    let mut thread_id: Option<String> = None;
    let mut raw_record_commands = 0;
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
        let observed_thread = validate_codex_turn(&values, turn.trim()).map_err(|reason| {
            format!(
                "askme scenario `{}` raw Codex event log {index}: {reason}",
                evidence.id
            )
        })?;
        raw_record_commands += values
            .iter()
            .filter(|event| {
                event.get("type").and_then(serde_json::Value::as_str) == Some("item.completed")
                    && event
                        .pointer("/item/type")
                        .and_then(serde_json::Value::as_str)
                        == Some("command_execution")
                    && event
                        .pointer("/item/status")
                        .and_then(serde_json::Value::as_str)
                        == Some("completed")
                    && event
                        .pointer("/item/exit_code")
                        .and_then(serde_json::Value::as_i64)
                        == Some(0)
                    && event
                        .pointer("/item/command")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|command| {
                            command
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .windows(2)
                                .any(|pair| pair == ["acquired-input", "record"])
                        })
            })
            .count();
        match &thread_id {
            Some(expected) if expected != observed_thread => {
                return Err(format!(
                    "askme scenario `{}` changed Codex thread between turns",
                    evidence.id
                ));
            }
            None => thread_id = Some(observed_thread.to_owned()),
            _ => {}
        }
    }
    let command_bytes = addressed_bytes(bundle, &addressed.command_log, "day command log")?;
    let command_text = std::str::from_utf8(&command_bytes)
        .map_err(|error| format!("day command log is not UTF-8: {error}"))?;
    let observed_commands = command_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if observed_commands != evidence.commands {
        return Err(format!(
            "askme scenario `{}` synthesized commands differ from the addressed wrapper log",
            evidence.id
        ));
    }
    let logged_record_commands = observed_commands
        .iter()
        .filter(|argv| {
            argv.windows(2)
                .any(|pair| pair == ["acquired-input", "record"])
        })
        .count();
    if raw_record_commands != logged_record_commands
        || logged_record_commands != usize::from(expected_record)
    {
        return Err(format!(
            "askme scenario `{}` raw Codex commands, wrapper log, and recording expectation differ",
            evidence.id
        ));
    }
    let before = addressed_bytes(bundle, &addressed.kan_before, "kan-before snapshot")?;
    let after = addressed_bytes(bundle, &addressed.kan_after, "kan-after snapshot")?;
    let before = kan_snapshot_counts(&before)?;
    let after = kan_snapshot_counts(&after)?;
    if before.claims.len() as u64 != evidence.claims_before
        || after.claims.len() as u64 != evidence.claims_after
        || after.texts != evidence.durable_claim_texts
    {
        return Err(format!(
            "askme scenario `{}` synthesized claim counts/texts differ from addressed kan snapshots",
            evidence.id
        ));
    }
    if before
        .claims
        .keys()
        .any(|cid| !after.claims.contains_key(cid))
    {
        return Err(format!(
            "askme scenario `{}` removed a pre-existing durable claim",
            evidence.id
        ));
    }
    let appended = after
        .claims
        .iter()
        .filter(|(cid, _)| !before.claims.contains_key(*cid))
        .collect::<Vec<_>>();
    if appended.len() != usize::from(expected_record) {
        return Err(format!(
            "askme scenario `{}` did not append the exact expected claim set",
            evidence.id
        ));
    }
    if let Some((_, Some(text))) = appended.first() {
        match day::atoms::extract_fenced::<day::events::AcquiredInput>(text) {
            Some(Ok(_)) => {}
            _ => {
                return Err(format!(
                    "askme scenario `{}` new claim is not a valid acquired-input event",
                    evidence.id
                ));
            }
        }
    } else if expected_record {
        return Err(format!(
            "askme scenario `{}` new acquired-input claim has no text",
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

fn validate_codex_turn<'a>(
    events: &'a [serde_json::Value],
    expected_message: &str,
) -> Result<&'a str, String> {
    let allowed_events = [
        "thread.started",
        "turn.started",
        "turn.completed",
        "item.started",
        "item.updated",
        "item.completed",
    ];
    let allowed_items = [
        "agent_message",
        "reasoning",
        "command_execution",
        "file_change",
        "mcp_tool_call",
        "collab_tool_call",
        "web_search",
        "todo_list",
    ];
    for event in events {
        let event_type = event
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "contains an event without a top-level type".to_string())?;
        if !allowed_events.contains(&event_type) {
            return Err(format!("contains unsupported Codex event `{event_type}`"));
        }
        if event_type.starts_with("item.") {
            let item_type = event
                .pointer("/item/type")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("`{event_type}` has no typed item"))?;
            if !allowed_items.contains(&item_type) {
                return Err(format!("contains unsupported Codex item `{item_type}`"));
            }
        }
    }
    if events
        .first()
        .and_then(|event| event.get("type"))
        .and_then(serde_json::Value::as_str)
        != Some("thread.started")
        || events
            .get(1)
            .and_then(|event| event.get("type"))
            .and_then(serde_json::Value::as_str)
            != Some("turn.started")
        || events
            .last()
            .and_then(|event| event.get("type"))
            .and_then(serde_json::Value::as_str)
            != Some("turn.completed")
    {
        return Err("does not have the pinned Codex event ordering".into());
    }
    let starts = events
        .iter()
        .filter(|event| {
            event.get("type").and_then(serde_json::Value::as_str) == Some("thread.started")
        })
        .collect::<Vec<_>>();
    if starts.len() != 1 {
        return Err("must contain exactly one `thread.started` event".into());
    }
    let thread_id = starts[0]
        .get("thread_id")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "`thread.started` has no thread identifier".to_string())?;
    let turn_starts = events
        .iter()
        .filter(|event| {
            event.get("type").and_then(serde_json::Value::as_str) == Some("turn.started")
        })
        .count();
    let turn_completions = events
        .iter()
        .filter(|event| {
            event.get("type").and_then(serde_json::Value::as_str) == Some("turn.completed")
        })
        .count();
    if turn_starts != 1 || turn_completions != 1 {
        return Err("does not contain a complete Codex turn".into());
    }
    let mut items = std::collections::BTreeMap::<&str, (&str, bool)>::new();
    for event in &events[2..events.len() - 1] {
        let event_type = event
            .get("type")
            .and_then(serde_json::Value::as_str)
            .expect("top-level event type checked above");
        if !event_type.starts_with("item.") {
            return Err("contains a thread/turn event inside the active turn".into());
        }
        let id = event
            .pointer("/item/id")
            .and_then(serde_json::Value::as_str)
            .filter(|id| !id.trim().is_empty())
            .ok_or_else(|| format!("`{event_type}` has no item identifier"))?;
        let item_type = event
            .pointer("/item/type")
            .and_then(serde_json::Value::as_str)
            .expect("typed item checked above");
        match event_type {
            "item.started" => {
                if items.insert(id, (item_type, false)).is_some() {
                    return Err(format!("item `{id}` starts more than once"));
                }
            }
            "item.updated" => {
                if !items.get(id).is_some_and(|(started_type, completed)| {
                    *started_type == item_type && !completed
                }) {
                    return Err(format!(
                        "item `{id}` is updated outside one type-stable active lifecycle"
                    ));
                }
            }
            "item.completed" => match items.get_mut(id) {
                Some((started_type, completed)) if *started_type == item_type && !*completed => {
                    *completed = true
                }
                Some(_) => {
                    return Err(format!(
                        "item `{id}` changes type or completes more than once"
                    ))
                }
                None => {
                    // Codex 0.147 emits atomic items (including messages and
                    // reasoning) as completion-only events. They are a closed
                    // lifecycle, unlike an update or duplicate completion.
                    items.insert(id, (item_type, true));
                }
            },
            _ => unreachable!("allowed item event checked above"),
        }
    }
    if items.values().any(|(_, completed)| !completed) {
        return Err("contains an item lifecycle without completion".into());
    }
    let final_message = events
        .iter()
        .filter_map(|event| {
            if event.get("type").and_then(serde_json::Value::as_str) != Some("item.completed")
                || event
                    .pointer("/item/type")
                    .and_then(serde_json::Value::as_str)
                    != Some("agent_message")
            {
                return None;
            }
            event
                .pointer("/item/text")
                .and_then(serde_json::Value::as_str)
        })
        .next_back();
    if final_message.map(str::trim) != Some(expected_message) {
        return Err(
            "final typed `agent_message` does not equal the captured assistant turn".into(),
        );
    }
    Ok(thread_id)
}

fn github_actions_origin(
    workflow: &str,
    candidate_sha: &str,
    run_id: Option<u64>,
) -> Result<(), String> {
    let origin = GithubActionsOrigin {
        actions: std::env::var("GITHUB_ACTIONS").unwrap_or_default(),
        workflow: std::env::var("GITHUB_WORKFLOW").unwrap_or_default(),
        sha: std::env::var("GITHUB_SHA").unwrap_or_default(),
        run_id: std::env::var("GITHUB_RUN_ID").unwrap_or_default(),
        repository: std::env::var("GITHUB_REPOSITORY").unwrap_or_default(),
        server: std::env::var("GITHUB_SERVER_URL").unwrap_or_default(),
        event: std::env::var("GITHUB_EVENT_NAME").unwrap_or_default(),
        workflow_ref: std::env::var("GITHUB_WORKFLOW_REF").unwrap_or_default(),
        ref_name: std::env::var("GITHUB_REF").unwrap_or_default(),
        workflow_sha: std::env::var("GITHUB_WORKFLOW_SHA").unwrap_or_default(),
    };
    validate_github_actions_origin(workflow, candidate_sha, run_id, &origin)
}

#[derive(Debug)]
struct GithubActionsOrigin {
    actions: String,
    workflow: String,
    sha: String,
    run_id: String,
    repository: String,
    server: String,
    event: String,
    workflow_ref: String,
    ref_name: String,
    workflow_sha: String,
}

fn validate_github_actions_origin(
    workflow: &str,
    candidate_sha: &str,
    run_id: Option<u64>,
    origin: &GithubActionsOrigin,
) -> Result<(), String> {
    let workflow_file = match workflow {
        "v0.13 askme behavioral trial" => "askme-behavioral-trial.yml",
        "v0.13 workflow reconstruction trial" => "workflow-reconstruction-trial.yml",
        _ => return Err("evidence grader named an unregistered workflow".into()),
    };
    let expected_ref = format!("kan-tools/day/.github/workflows/{workflow_file}@");
    if origin.actions != "true"
        || origin.workflow != workflow
        || origin.sha != candidate_sha
        || origin.repository != "kan-tools/day"
        || origin.server != "https://github.com"
        || origin.event != "workflow_dispatch"
        || origin.ref_name.is_empty()
        || origin.workflow_ref != format!("{expected_ref}{}", origin.ref_name)
        || origin.workflow_sha != candidate_sha
    {
        return Err(
            "evidence grading is authoritative only inside the exact kan-tools/day candidate GitHub workflow file".into(),
        );
    }
    let observed_run_id = origin.run_id.parse::<u64>().ok().filter(|id| *id != 0);
    if observed_run_id.is_none() {
        return Err("GitHub workflow run ID is absent or invalid".into());
    }
    if let Some(run_id) = run_id {
        if observed_run_id != Some(run_id) {
            return Err("evidence bundle is not bound to the executing GitHub run".into());
        }
    }
    Ok(())
}

struct KanSnapshot {
    claims: std::collections::BTreeMap<String, Option<String>>,
    texts: Vec<String>,
}

fn kan_snapshot_counts(bytes: &[u8]) -> Result<KanSnapshot, String> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("kan snapshot is malformed: {error}"))?;
    let subjects = value
        .get("subjects")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "kan snapshot has no subjects array".to_string())?;
    let mut claims_by_cid = std::collections::BTreeMap::new();
    let mut texts = Vec::new();
    for subject in subjects {
        let claims = subject
            .get("claims")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "kan snapshot subject has no claims array".to_string())?;
        for claim in claims {
            let text = claim
                .get("text")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            if let Some(cid) = claim.get("cid").and_then(serde_json::Value::as_str) {
                if claims_by_cid.insert(cid.to_owned(), text.clone()).is_some() {
                    return Err(format!("kan snapshot repeats CID `{cid}`"));
                }
            }
            if let Some(text) = text {
                texts.push(text.to_owned());
            }
        }
    }
    Ok(KanSnapshot {
        claims: claims_by_cid,
        texts,
    })
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
        if day::events::contains_transcript_shape(text) {
            return fail("durable claim content contains a raw multi-speaker transcript");
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconstructionEvidence {
    schema: u64,
    github_run_id: u64,
    candidate_sha: String,
    behavioral_candidate_sha: String,
    publication_candidate_sha: String,
    stream_subject: String,
    acquired_input_cid: String,
    intervention_cid: String,
    handoff_cid: String,
    review_claim_cid: String,
    wakeup_evidence_path: String,
    wakeup_evidence_sha256: String,
    wakeup_raw_events: AddressedArtifact,
    runner_suite: AddressedArtifact,
    kan_read_path: String,
    kan_read_sha256: String,
    kan_authors: Vec<String>,
    suite_commit: String,
    census_base: String,
    census_head: String,
    ci_run_id: u64,
    ci_head_sha: String,
    fresh_wakeup_had_transcript: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconstructionSource {
    schema: u64,
    candidate_sha: String,
    behavioral_candidate_sha: String,
    publication_candidate_sha: String,
    stream_subject: String,
    acquired_input_cid: String,
    intervention_cid: String,
    handoff_cid: String,
    review_claim_cid: String,
    kan_authors: Vec<String>,
    suite_commit: String,
    census_base: String,
    census_head: String,
    ci_run_id: u64,
    ci_head_sha: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RunnerSuiteEvidence {
    schema: u64,
    argv: Vec<String>,
    candidate_sha: String,
    head_sha: String,
    tree_clean: bool,
    exit_code: i64,
    #[serde(rename = "stdout")]
    _stdout: String,
    #[serde(rename = "stderr")]
    _stderr: String,
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
    claims: Vec<ReconstructionClaim>,
}

#[derive(Debug, Deserialize)]
struct ReconstructionClaim {
    cid: String,
    kind: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    cites: Vec<String>,
}

pub fn grade_reconstruction(
    root: &Path,
    bundle: &Path,
    source: &Path,
    candidate_sha: &str,
    evidence_commit: &str,
) -> Outcome<()> {
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
        "remove-review",
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
    let source = root.join(source);
    let manifest_path = bundle.join("manifest.json");
    let evidence: ReconstructionEvidence =
        match read(&manifest_path).and_then(|bytes| json(&manifest_path, &bytes)) {
            Ok(evidence) => evidence,
            Err(error) => return Outcome::Finding(Finding::new(error)),
        };
    if let Err(error) = github_actions_origin(
        "v0.13 workflow reconstruction trial",
        candidate_sha,
        Some(evidence.github_run_id),
    ) {
        return Outcome::Finding(Finding::new(error));
    }
    if let Err(error) =
        authenticate_reconstruction(&source, &bundle, &evidence, candidate_sha, evidence_commit)
    {
        return Outcome::Finding(Finding::new(error));
    }
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

fn authenticate_reconstruction(
    source: &Path,
    bundle: &Path,
    evidence: &ReconstructionEvidence,
    candidate_sha: &str,
    evidence_commit: &str,
) -> Result<(), String> {
    full_sha("workflow candidate SHA", candidate_sha)?;
    full_sha("workflow evidence commit", evidence_commit)?;
    if evidence.candidate_sha != candidate_sha {
        return Err(
            "bundle coordinates differ from the candidate and evidence commit supplied by the workflow"
                .into(),
        );
    }
    let source_manifest_path = source.join("manifest.json");
    let declared: ReconstructionSource =
        json(&source_manifest_path, &read(&source_manifest_path)?)?;
    if declared.schema != evidence.schema
        || declared.candidate_sha != evidence.candidate_sha
        || declared.behavioral_candidate_sha != evidence.behavioral_candidate_sha
        || declared.publication_candidate_sha != evidence.publication_candidate_sha
        || declared.stream_subject != evidence.stream_subject
        || declared.acquired_input_cid != evidence.acquired_input_cid
        || declared.intervention_cid != evidence.intervention_cid
        || declared.handoff_cid != evidence.handoff_cid
        || declared.review_claim_cid != evidence.review_claim_cid
        || declared.kan_authors != evidence.kan_authors
        || declared.suite_commit != evidence.suite_commit
        || declared.census_base != evidence.census_base
        || declared.census_head != evidence.census_head
        || declared.ci_run_id != evidence.ci_run_id
        || declared.ci_head_sha != evidence.ci_head_sha
    {
        return Err(
            "generated bundle rewrote coordinates from the immutable source manifest".into(),
        );
    }
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(source)
        .output()
        .map_err(|error| format!("could not inspect evidence checkout: {error}"))?;
    if !head.status.success() || String::from_utf8_lossy(&head.stdout).trim() != evidence_commit {
        return Err("bundle is not the exact immutable evidence commit checkout".into());
    }
    if !source.join(".claims").is_dir() {
        return Err("evidence commit has no published signed kan claims".into());
    }
    if evidence.kan_authors != [V013_EVIDENCE_PRINCIPAL] {
        return Err("evidence manifest does not use the pinned v0.13 review principal".into());
    }
    let mut command = std::process::Command::new("kan");
    command.args(["show", "--all", "--json"]);
    for author in &evidence.kan_authors {
        command.args(["--trust", author]);
    }
    let live = command
        .current_dir(source)
        .output()
        .map_err(|error| format!("could not authenticate evidence through kan: {error}"))?;
    if !live.status.success() {
        return Err(format!(
            "kan rejected the published evidence claims: {}",
            String::from_utf8_lossy(&live.stderr).trim()
        ));
    }
    let live: serde_json::Value = serde_json::from_slice(&live.stdout)
        .map_err(|error| format!("authenticated kan read was malformed: {error}"))?;
    let addressed = read(&bundle.join(&evidence.kan_read_path))?;
    if digest(&addressed) != evidence.kan_read_sha256 {
        return Err("kan read digest differs from its manifest address".into());
    }
    let addressed: serde_json::Value = serde_json::from_slice(&addressed)
        .map_err(|error| format!("addressed kan read was malformed: {error}"))?;
    if live != addressed {
        return Err("addressed kan read differs from kan's authenticated signed-claim view".into());
    }
    Ok(())
}

fn validate_reconstruction(bundle: &Path, evidence: &ReconstructionEvidence) -> Result<(), String> {
    if evidence.schema != 1 || evidence.github_run_id == 0 {
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
        ("review claim CID", &evidence.review_claim_cid),
        ("wakeup evidence path", &evidence.wakeup_evidence_path),
        ("kan read path", &evidence.kan_read_path),
    ] {
        if value.trim().is_empty() {
            return Err(format!("{field} is missing"));
        }
    }
    for (field, cid) in [
        ("acquired-input CID", &evidence.acquired_input_cid),
        ("intervention CID", &evidence.intervention_cid),
        ("handoff CID", &evidence.handoff_cid),
        ("review claim CID", &evidence.review_claim_cid),
    ] {
        if !cid.starts_with("bafy") || cid.len() < 50 {
            return Err(format!("{field} is not a content-addressed CID"));
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
    let wakeup_bytes = read(&wakeup_path)?;
    if digest(&wakeup_bytes) != evidence.wakeup_evidence_sha256 {
        return Err("fresh wakeup evidence digest differs from its manifest address".into());
    }
    let wakeup: FreshWakeupEvidence = json(&wakeup_path, &wakeup_bytes)?;
    validate_fresh_wakeup(&wakeup, evidence)?;
    let raw = addressed_bytes(
        bundle,
        &evidence.wakeup_raw_events,
        "fresh wakeup Codex event log",
    )?;
    let raw = std::str::from_utf8(&raw)
        .map_err(|error| format!("fresh wakeup Codex event log is not UTF-8: {error}"))?;
    let events = raw
        .lines()
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_str::<serde_json::Value>(line).map_err(|error| {
                format!(
                    "fresh wakeup event line {} is malformed: {error}",
                    index + 1
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    validate_codex_turn(&events, wakeup.rendered_context.trim())?;
    if completed_command_event(&events, "git rev-parse HEAD").map(|(_, output)| output.trim())
        != Some(evidence.candidate_sha.as_str())
        || !completed_command_event(&events, "git status --porcelain")
            .map(|(_, output)| output)
            .is_some_and(|output| output.trim().is_empty())
    {
        return Err(
            "fresh wakeup raw events do not prove a clean checkout at the candidate SHA".into(),
        );
    }
    if completed_command_event(&events, "kan show --all --json").is_none() {
        return Err(
            "fresh wakeup raw events do not prove the required bulk kan command ran".into(),
        );
    }

    let kan_path = bundle.join(&evidence.kan_read_path);
    let kan_bytes = read(&kan_path)?;
    if digest(&kan_bytes) != evidence.kan_read_sha256 {
        return Err("kan read digest differs from its manifest address".into());
    }
    let kan_read: BulkKanRead = json(&kan_path, &kan_bytes)?;
    validate_bulk_reconstruction(bundle, &kan_read, evidence, &events)?;
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
    bundle: &Path,
    read: &BulkKanRead,
    evidence: &ReconstructionEvidence,
    events: &[serde_json::Value],
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
    let review = unique_claim(read, &evidence.review_claim_cid)?;
    for (label, claim) in [
        ("acquired-input", acquired.1),
        ("intervention", intervention.1),
        ("handoff", handoff.1),
        ("review", review.1),
    ] {
        if !claim
            .author
            .as_ref()
            .is_some_and(|author| evidence.kan_authors.contains(author))
        {
            return Err(format!(
                "{label} claim is not signed by an explicitly admitted evidence principal"
            ));
        }
    }
    let acquired_payload =
        validate_event_claim::<day::events::AcquiredInput>(acquired.1, "day-acquired-input")?;
    let intervention_payload =
        validate_event_claim::<day::events::Intervention>(intervention.1, "day-intervention")?;
    if acquired.1.author.as_deref() != Some(acquired_payload.recorded_by.as_str())
        || intervention.1.author.as_deref() != Some(intervention_payload.recorded_by.as_str())
    {
        return Err(
            "event block recording principal differs from its signed claim envelope".into(),
        );
    }
    if acquired.0 != acquired_payload.work_subject
        || intervention.0 != intervention_payload.work_subject
        || acquired_payload.work_subject != intervention_payload.work_subject
    {
        return Err(
            "acquired-input and intervention blocks are not bound to one real work subject".into(),
        );
    }
    validate_event_source(read, &acquired_payload.provider, &acquired_payload.basis)?;
    validate_event_source(
        read,
        &intervention_payload.source,
        &intervention_payload.basis,
    )?;
    if handoff.0 != evidence.stream_subject || handoff.1.kind != "Observation" {
        return Err("handoff CID is not an Observation on the declared stream subject".into());
    }
    if review.0 != "v0.13-workflow-ergonomics"
        || !matches!(review.1.kind.as_str(), "Decision" | "Result")
        || !review.1.text.as_deref().is_some_and(|text| {
            text.contains("v0.13 reconstruction evidence reviewed as genuine work")
        })
        || ![
            evidence.acquired_input_cid.as_str(),
            evidence.intervention_cid.as_str(),
            evidence.handoff_cid.as_str(),
        ]
        .iter()
        .all(|cid| review.1.cites.iter().any(|cited| cited == cid))
    {
        return Err(
            "pinned review claim does not adjudicate the exact real-work reconstruction claims"
                .into(),
        );
    }
    let text = handoff
        .1
        .text
        .as_deref()
        .ok_or_else(|| "handoff claim has no narrative".to_string())?;
    let scopes = day::atoms::extract_fenced::<day::stream::HandoffScopes>(text)
        .ok_or_else(|| "handoff claim has no day-handoff-scopes block".to_string())?
        .map_err(|error| format!("handoff scope block is invalid: {error}"))?;
    let suite = scopes
        .suites
        .iter()
        .find(|scope| scope.commit == evidence.suite_commit && scope.tree_clean);
    let census = scopes.censuses.iter().find(|scope| {
        scope.base == evidence.census_base
            && scope.head == evidence.census_head
            && scope.unaccounted == 0
    });
    let ci = scopes.ci.iter().find(|scope| {
        scope.run_id == evidence.ci_run_id
            && scope.head_sha == evidence.ci_head_sha
            && scope.conclusion == "success"
    });
    let (Some(suite), Some(_census), Some(_ci)) = (suite, census, ci) else {
        return Err("handoff scopes do not bind the declared suite/census/CI coordinates".into());
    };
    let suite_command = suite.argv.join(" ");
    let suite_bytes = addressed_bytes(
        bundle,
        &evidence.runner_suite,
        "independent runner suite evidence",
    )?;
    let runner_suite: RunnerSuiteEvidence = serde_json::from_slice(&suite_bytes)
        .map_err(|error| format!("independent runner suite evidence is malformed: {error}"))?;
    if runner_suite.schema != 1
        || runner_suite.argv != suite.argv
        || runner_suite.candidate_sha != evidence.candidate_sha
        || runner_suite.head_sha != evidence.candidate_sha
        || !runner_suite.tree_clean
        || runner_suite.exit_code != 0
    {
        return Err(
            "independent runner suite did not execute the declared suite from the clean candidate"
                .into(),
        );
    }
    let census_command = format!(
        "just census-demonstrations {}..{}",
        evidence.census_base, evidence.census_head
    );
    let ci_command = format!(
        "gh run view {} --json headSha,conclusion",
        evidence.ci_run_id
    );
    let Some((head_index, head_output)) = completed_command_event(events, "git rev-parse HEAD")
    else {
        return Err("fresh wakeup did not execute the candidate HEAD check".into());
    };
    let Some((status_index, status_output)) =
        completed_command_event(events, "git status --porcelain")
    else {
        return Err("fresh wakeup did not execute the clean-tree check".into());
    };
    let Some((kan_index, _)) = completed_command_event(events, "kan show --all --json") else {
        return Err("fresh wakeup did not execute the authenticated kan read".into());
    };
    let Some((suite_index, _)) = completed_command_event(events, &suite_command) else {
        return Err("fresh wakeup did not execute the declared suite".into());
    };
    let Some((census_index, census_output)) = completed_command_event(events, &census_command)
    else {
        return Err("fresh wakeup did not execute the declared census".into());
    };
    let Some((ci_index, ci_output)) = completed_command_event(events, &ci_command) else {
        return Err("fresh wakeup did not execute the declared CI check".into());
    };
    if head_output.trim() != evidence.candidate_sha
        || !status_output.trim().is_empty()
        || !(head_index < status_index
            && status_index < kan_index
            && kan_index < suite_index
            && suite_index < census_index
            && census_index < ci_index)
        || !census_has_zero_unaccounted(census_output)
        || !ci_output_matches(ci_output, &evidence.ci_head_sha)
    {
        return Err(
            "fresh wakeup raw events do not independently recheck ordered candidate, suite, census, and CI scopes"
                .into(),
        );
    }
    Ok(())
}

fn completed_command_event<'a>(
    events: &'a [serde_json::Value],
    expected: &str,
) -> Option<(usize, &'a str)> {
    events.iter().enumerate().find_map(|(index, event)| {
        (event.get("type").and_then(serde_json::Value::as_str) == Some("item.completed")
            && event
                .pointer("/item/type")
                .and_then(serde_json::Value::as_str)
                == Some("command_execution")
            && event
                .pointer("/item/status")
                .and_then(serde_json::Value::as_str)
                == Some("completed")
            && event
                .pointer("/item/exit_code")
                .and_then(serde_json::Value::as_i64)
                == Some(0)
            && event
                .pointer("/item/command")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|command| command.trim() == expected))
        .then(|| {
            (
                index,
                event
                    .pointer("/item/aggregated_output")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or(""),
            )
        })
    })
}

fn census_has_zero_unaccounted(output: &str) -> bool {
    output
        .lines()
        .any(|line| line.split('|').map(str::trim).collect::<Vec<_>>() == ["unaccounted", "0"])
}

fn ci_output_matches(output: &str, expected_head: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(output.trim())
        .ok()
        .is_some_and(|value| {
            value.get("headSha").and_then(serde_json::Value::as_str) == Some(expected_head)
                && value.get("conclusion").and_then(serde_json::Value::as_str) == Some("success")
        })
}

fn unique_claim<'a>(
    read: &'a BulkKanRead,
    cid: &str,
) -> Result<(&'a str, &'a ReconstructionClaim), String> {
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
    claim: &ReconstructionClaim,
    fence: &str,
) -> Result<T, String> {
    if claim.kind != "Observation" {
        return Err(format!("{fence} CID is not an Observation"));
    }
    let text = claim
        .text
        .as_deref()
        .ok_or_else(|| format!("{fence} claim has no narrative"))?;
    day::atoms::extract_fenced::<T>(text)
        .ok_or_else(|| format!("claim has no `{fence}` block"))?
        .map_err(|error| format!("`{fence}` block is invalid: {error}"))
}

fn validate_event_source(
    read: &BulkKanRead,
    source: &day::events::Source,
    basis: &[String],
) -> Result<(), String> {
    let day::events::Source::AuthenticatedClaim { principal, claim } = source else {
        return Ok(());
    };
    if !basis.contains(claim) {
        return Err("authenticated event source is not retained in the event basis".into());
    }
    let (_, source_claim) = unique_claim(read, claim)?;
    if source_claim.author.as_deref() != Some(principal) {
        return Err("authenticated event source principal differs from its signed claim".into());
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
            "remove-review",
            Box::new(|value| value.review_claim_cid.clear()),
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
    fn authoritative_origin_is_the_exact_upstream_workflow_not_a_named_fork() {
        let candidate = "a".repeat(40);
        let mut origin = GithubActionsOrigin {
            actions: "true".into(),
            workflow: "v0.13 askme behavioral trial".into(),
            sha: candidate.clone(),
            run_id: "42".into(),
            repository: "kan-tools/day".into(),
            server: "https://github.com".into(),
            event: "workflow_dispatch".into(),
            workflow_ref:
                "kan-tools/day/.github/workflows/askme-behavioral-trial.yml@refs/heads/candidate"
                    .into(),
            ref_name: "refs/heads/candidate".into(),
            workflow_sha: candidate.clone(),
        };
        assert!(validate_github_actions_origin(
            "v0.13 askme behavioral trial",
            &candidate,
            Some(42),
            &origin
        )
        .is_ok());
        origin.workflow_ref =
            "kan-tools/day/.github/workflows/askme-behavioral-trial.yml@refs/heads/other".into();
        assert!(validate_github_actions_origin(
            "v0.13 askme behavioral trial",
            &candidate,
            Some(42),
            &origin
        )
        .is_err());
        origin.workflow_ref =
            "kan-tools/day/.github/workflows/askme-behavioral-trial.yml@refs/heads/candidate"
                .into();
        origin.workflow_sha = "b".repeat(40);
        assert!(validate_github_actions_origin(
            "v0.13 askme behavioral trial",
            &candidate,
            Some(42),
            &origin
        )
        .is_err());
        origin.workflow_sha = candidate.clone();
        origin.run_id = "41".into();
        assert!(validate_github_actions_origin(
            "v0.13 askme behavioral trial",
            &candidate,
            Some(42),
            &origin
        )
        .is_err());
        origin.run_id = "42".into();
        origin.repository = "attacker/fork".into();
        assert!(validate_github_actions_origin(
            "v0.13 askme behavioral trial",
            &candidate,
            Some(42),
            &origin
        )
        .is_err());
    }

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
            let acquired_text = day::events::AcquiredInput {
                work_subject: "work/askme".into(),
                topic: "release".into(),
                provider: day::events::Source::Recorder {
                    principal: "did:key:recorder".into(),
                },
                recorded_by: "did:key:recorder".into(),
                facts: vec!["structured summary only".into()],
                decisions: vec![],
                unresolved: vec![],
                material_effect: "records the explicit answer".into(),
                basis: vec!["bafy-basis".into()],
            }
            .to_claim_text();
            let durable_texts = if scenario.expect.record {
                vec!["structured summary only".to_string(), acquired_text.clone()]
            } else {
                vec!["structured summary only".to_string()]
            };
            let evidence = serde_json::json!({
                "schema": 1,
                "id": scenario.id,
                "candidate_sha": candidate,
                "user_turns": scenario.turns,
                "assistant_turns": assistant_turns,
                "commands": if scenario.expect.record { serde_json::json!([["day", "acquired-input", "record"]]) } else { serde_json::json!([]) },
                "claims_before": 2,
                "claims_after": claims_after,
                "durable_claim_texts": durable_texts
            });
            let path = bundle.path().join(format!("{}.json", scenario.id));
            std::fs::write(&path, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
            let mut raw_events = Vec::new();
            let thread_id = format!("thread-{}", scenario.id);
            for (index, turn) in turns_for_raw.iter().enumerate() {
                let raw = bundle
                    .path()
                    .join(format!("{}-events-{index}.jsonl", scenario.id));
                let command_event = if scenario.expect.record && index + 1 == turns_for_raw.len() {
                    "{\"type\":\"item.completed\",\"item\":{\"id\":\"record-command\",\"type\":\"command_execution\",\"command\":\"day acquired-input record\",\"aggregated_output\":\"recorded\",\"exit_code\":0,\"status\":\"completed\"}}\n"
                } else {
                    ""
                };
                std::fs::write(
                    &raw,
                    format!(
                        "{{\"type\":\"thread.started\",\"thread_id\":{thread}}}\n{{\"type\":\"turn.started\"}}\n{command_event}{{\"type\":\"item.completed\",\"item\":{{\"id\":\"item-{index}\",\"type\":\"agent_message\",\"text\":{turn}}}}}\n{{\"type\":\"turn.completed\",\"usage\":{{}}}}\n",
                        thread = serde_json::to_string(&thread_id).unwrap(),
                        turn = serde_json::to_string(turn).unwrap(),
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
                after_claims.push(serde_json::json!({"cid": "cid-after-3", "text": acquired_text}));
            }
            std::fs::write(&before_path, serde_json::to_vec(&before).unwrap()).unwrap();
            std::fs::write(
                &after_path,
                serde_json::to_vec(&serde_json::json!({"subjects": [{"claims": after_claims}]}))
                    .unwrap(),
            )
            .unwrap();
            let command_path = bundle.path().join(format!("{}-commands.log", scenario.id));
            let command_text = if scenario.expect.record {
                "day acquired-input record\n"
            } else {
                ""
            };
            std::fs::write(&command_path, command_text).unwrap();
            files.push(serde_json::json!({
                "id": scenario.id,
                "path": path.file_name().unwrap().to_str().unwrap(),
                "sha256": digest(&std::fs::read(&path).unwrap()),
                "raw_events": raw_events,
                "command_log": {
                    "path": command_path.file_name().unwrap().to_str().unwrap(),
                    "sha256": digest(&std::fs::read(&command_path).unwrap())
                },
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
            "github_run_id": 1,
            "protocol_sha256": digest(&protocol_bytes),
            "harness": "codex-cli",
            "harness_version": "codex-cli 0.147.0",
            "model": "fixture-model",
            "scenarios": files
        });
        std::fs::write(
            bundle.path().join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).unwrap(),
        )
        .unwrap();
        grade_askme_inner(root, bundle.path(), None)
            .expect("the structurally valid, non-authoritative fixture must grade internally");
        assert!(
            !grade_askme(root, bundle.path(), &candidate, 1).is_passed(),
            "synthetic structural fixtures are never authoritative workflow evidence"
        );
        let record_id = protocol
            .scenarios
            .iter()
            .find(|scenario| scenario.expect.record)
            .unwrap()
            .id
            .clone();
        let evidence_path = bundle.path().join(format!("{record_id}.json"));
        let after_path = bundle.path().join(format!("{record_id}-kan-after.json"));
        let evidence_original = std::fs::read(&evidence_path).unwrap();
        let after_original = std::fs::read(&after_path).unwrap();
        let manifest_original = std::fs::read(bundle.path().join("manifest.json")).unwrap();
        let serialized_transcript = day::events::AcquiredInput {
            work_subject: "work/askme".into(),
            topic: "release".into(),
            provider: day::events::Source::Recorder {
                principal: "did:key:recorder".into(),
            },
            recorded_by: "did:key:recorder".into(),
            facts: vec!["**Alice Smith**\nchoose A\n**Bob Jones**\nchoose B".into()],
            decisions: vec![],
            unresolved: vec![],
            material_effect: "records the explicit answer".into(),
            basis: vec!["bafy-basis".into()],
        }
        .to_claim_text();
        assert!(day::events::contains_transcript_shape(
            "**Alice Smith**\nchoose A\n**Bob Jones**\nchoose B"
        ));
        assert!(!day::events::contains_transcript_shape(
            &serialized_transcript
        ));
        let mut hostile_evidence: serde_json::Value =
            serde_json::from_slice(&evidence_original).unwrap();
        hostile_evidence["durable_claim_texts"][1] = serialized_transcript.clone().into();
        std::fs::write(
            &evidence_path,
            serde_json::to_vec_pretty(&hostile_evidence).unwrap(),
        )
        .unwrap();
        let mut hostile_after: serde_json::Value = serde_json::from_slice(&after_original).unwrap();
        hostile_after["subjects"][0]["claims"][2]["text"] = serialized_transcript.into();
        std::fs::write(&after_path, serde_json::to_vec(&hostile_after).unwrap()).unwrap();
        let mut hostile_manifest: serde_json::Value =
            serde_json::from_slice(&manifest_original).unwrap();
        let entry = hostile_manifest["scenarios"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["id"] == record_id)
            .unwrap();
        entry["sha256"] = digest(&std::fs::read(&evidence_path).unwrap()).into();
        entry["kan_after"]["sha256"] = digest(&std::fs::read(&after_path).unwrap()).into();
        std::fs::write(
            bundle.path().join("manifest.json"),
            serde_json::to_vec_pretty(&hostile_manifest).unwrap(),
        )
        .unwrap();
        assert!(grade_askme_inner(root, bundle.path(), None)
            .unwrap_err()
            .contains("not a valid acquired-input event"));
        std::fs::write(&evidence_path, evidence_original).unwrap();
        std::fs::write(&after_path, after_original).unwrap();
        std::fs::write(bundle.path().join("manifest.json"), manifest_original).unwrap();
        assert!(validate_codex_turn(
            &[serde_json::json!({"event": {"message": "invented"}})],
            "invented"
        )
        .is_err());
        let misordered = [
            serde_json::json!({"type": "thread.started", "thread_id": "thread"}),
            serde_json::json!({"type": "item.completed", "item": {"id": "message", "type": "agent_message", "text": "invented"}}),
            serde_json::json!({"type": "turn.started"}),
            serde_json::json!({"type": "turn.completed", "usage": {}}),
        ];
        assert!(validate_codex_turn(&misordered, "invented").is_err());
        let unpaired = [
            serde_json::json!({"type": "thread.started", "thread_id": "thread"}),
            serde_json::json!({"type": "turn.started"}),
            serde_json::json!({"type": "item.started", "item": {"id": "message", "type": "agent_message", "text": "invented"}}),
            serde_json::json!({"type": "turn.completed", "usage": {}}),
        ];
        assert!(validate_codex_turn(&unpaired, "invented").is_err());
        let type_changing = [
            serde_json::json!({"type": "thread.started", "thread_id": "thread"}),
            serde_json::json!({"type": "turn.started"}),
            serde_json::json!({"type": "item.started", "item": {"id": "same", "type": "reasoning", "text": "thinking"}}),
            serde_json::json!({"type": "item.updated", "item": {"id": "same", "type": "command_execution", "command": "cargo test"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "same", "type": "agent_message", "text": "invented"}}),
            serde_json::json!({"type": "turn.completed", "usage": {}}),
        ];
        assert!(validate_codex_turn(&type_changing, "invented").is_err());

        let manifest_value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(bundle.path().join("manifest.json")).unwrap())
                .unwrap();
        let raw_relative = manifest_value["scenarios"][0]["raw_events"][0]["path"]
            .as_str()
            .unwrap();
        let raw_path = bundle.path().join(raw_relative);
        let raw_original = std::fs::read(&raw_path).unwrap();
        std::fs::write(&raw_path, b"{}\n").unwrap();
        assert!(grade_askme_inner(root, bundle.path(), None)
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
        assert!(grade_askme_inner(root, bundle.path(), None)
            .unwrap_err()
            .contains("did not address"));
    }

    #[test]
    fn reconstruction_bundle_passes_only_with_every_coordinate_and_claim() {
        let bundle = tempfile::tempdir().unwrap();
        let candidate = "a".repeat(40);
        let base = "b".repeat(40);
        let input_cid = format!("bafy{}", "i".repeat(55));
        let intervention_cid = format!("bafy{}", "j".repeat(55));
        let handoff_cid = format!("bafy{}", "k".repeat(55));
        let review_claim_cid = format!("bafy{}", "r".repeat(55));
        let rendered = format!(
            "agents/handoff/main {input_cid} {intervention_cid} {handoff_cid} {candidate} {base} {candidate} 42 {candidate}"
        );
        let wakeup = serde_json::json!({
            "schema": 1,
            "session_kind": "fresh",
            "raw_transcript_supplied": false,
            "kan_command": ["kan", "show", "--all", "--json"],
            "stream_subject": "agents/handoff/main",
            "claims_read": [&input_cid, &intervention_cid, &handoff_cid],
            "candidate_sha": candidate,
            "rendered_context": rendered,
        });
        std::fs::write(
            bundle.path().join("wakeup.json"),
            serde_json::to_vec_pretty(&wakeup).unwrap(),
        )
        .unwrap();
        let wakeup_bytes = std::fs::read(bundle.path().join("wakeup.json")).unwrap();
        let raw = [
            serde_json::json!({"type": "thread.started", "thread_id": "thread-wakeup"}),
            serde_json::json!({"type": "turn.started"}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-head", "type": "command_execution", "command": "git rev-parse HEAD", "aggregated_output": format!("{candidate}\n"), "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-clean", "type": "command_execution", "command": "git status --porcelain", "aggregated_output": "", "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-1", "type": "command_execution", "command": "kan show --all --json", "aggregated_output": "signed claims", "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-2", "type": "command_execution", "command": "cargo test", "aggregated_output": "ok", "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-3", "type": "command_execution", "command": format!("just census-demonstrations {base}..{candidate}"), "aggregated_output": "unaccounted | 0", "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "command-4", "type": "command_execution", "command": "gh run view 42 --json headSha,conclusion", "aggregated_output": format!("{{\"headSha\":\"{candidate}\",\"conclusion\":\"success\"}}"), "exit_code": 0, "status": "completed"}}),
            serde_json::json!({"type": "item.completed", "item": {"id": "message-1", "type": "agent_message", "text": rendered}}),
            serde_json::json!({"type": "turn.completed", "usage": {}}),
        ]
        .into_iter()
        .map(|event| serde_json::to_string(&event).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
            + "\n";
        std::fs::write(bundle.path().join("wakeup-events.jsonl"), &raw).unwrap();
        let acquired = day::events::AcquiredInput {
            work_subject: "work/main".into(),
            topic: "release".into(),
            provider: day::events::Source::Recorder {
                principal: V013_EVIDENCE_PRINCIPAL.into(),
            },
            recorded_by: V013_EVIDENCE_PRINCIPAL.into(),
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
                principal: V013_EVIDENCE_PRINCIPAL.into(),
            },
            recorded_by: V013_EVIDENCE_PRINCIPAL.into(),
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
                    {"cid": &input_cid, "kind": "Observation", "author": V013_EVIDENCE_PRINCIPAL, "text": acquired.to_claim_text()},
                    {"cid": &intervention_cid, "kind": "Observation", "author": V013_EVIDENCE_PRINCIPAL, "text": intervention.to_claim_text()}
                ]},
                {"subject": "agents/handoff/main", "excluded_by_trust": 0, "claims": [
                    {"cid": &handoff_cid, "kind": "Observation", "author": V013_EVIDENCE_PRINCIPAL, "text": handoff_text}
                ]},
                {"subject": "v0.13-workflow-ergonomics", "excluded_by_trust": 0, "claims": [
                    {"cid": &review_claim_cid, "kind": "Decision", "author": V013_EVIDENCE_PRINCIPAL,
                     "text": "v0.13 reconstruction evidence reviewed as genuine work",
                     "cites": [&input_cid, &intervention_cid, &handoff_cid]}
                ]}
            ]
        });
        std::fs::write(
            bundle.path().join("kan.json"),
            serde_json::to_vec_pretty(&kan).unwrap(),
        )
        .unwrap();
        let kan_bytes = std::fs::read(bundle.path().join("kan.json")).unwrap();
        let runner_suite = serde_json::json!({
            "schema": 1,
            "argv": ["cargo", "test"],
            "candidate_sha": candidate,
            "head_sha": candidate,
            "tree_clean": true,
            "exit_code": 0,
            "stdout": "tests passed",
            "stderr": ""
        });
        std::fs::write(
            bundle.path().join("runner-suite.json"),
            serde_json::to_vec_pretty(&runner_suite).unwrap(),
        )
        .unwrap();
        let runner_suite_bytes = std::fs::read(bundle.path().join("runner-suite.json")).unwrap();
        let evidence = ReconstructionEvidence {
            schema: 1,
            github_run_id: 73,
            candidate_sha: candidate.clone(),
            behavioral_candidate_sha: candidate.clone(),
            publication_candidate_sha: candidate.clone(),
            stream_subject: "agents/handoff/main".into(),
            acquired_input_cid: input_cid,
            intervention_cid,
            handoff_cid,
            review_claim_cid,
            wakeup_evidence_path: "wakeup.json".into(),
            wakeup_evidence_sha256: digest(&wakeup_bytes),
            wakeup_raw_events: AddressedArtifact {
                path: "wakeup-events.jsonl".into(),
                sha256: digest(raw.as_bytes()),
            },
            runner_suite: AddressedArtifact {
                path: "runner-suite.json".into(),
                sha256: digest(&runner_suite_bytes),
            },
            kan_read_path: "kan.json".into(),
            kan_read_sha256: digest(&kan_bytes),
            kan_authors: vec![V013_EVIDENCE_PRINCIPAL.into()],
            suite_commit: candidate,
            census_base: base,
            census_head: "a".repeat(40),
            ci_run_id: 42,
            ci_head_sha: "a".repeat(40),
            fresh_wakeup_had_transcript: false,
        };
        assert!(validate_reconstruction(bundle.path(), &evidence).is_ok());
        let raw_values = raw
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect::<Vec<serde_json::Value>>();
        let head = completed_command_event(&raw_values, "git rev-parse HEAD")
            .unwrap()
            .0;
        let suite = completed_command_event(&raw_values, "cargo test")
            .unwrap()
            .0;
        assert!(head < suite, "candidate checks must precede the suite");
        let mut out_of_order = raw_values.clone();
        out_of_order.swap(head, suite);
        assert!(
            completed_command_event(&out_of_order, "git rev-parse HEAD")
                .unwrap()
                .0
                > completed_command_event(&out_of_order, "cargo test")
                    .unwrap()
                    .0,
            "the hostile ordering must be observable"
        );
        let out_of_order_bytes = out_of_order
            .iter()
            .map(|event| serde_json::to_string(event).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        std::fs::write(
            bundle.path().join("out-of-order-events.jsonl"),
            &out_of_order_bytes,
        )
        .unwrap();
        let mut out_of_order_evidence = evidence.clone();
        out_of_order_evidence.wakeup_raw_events = AddressedArtifact {
            path: "out-of-order-events.jsonl".into(),
            sha256: digest(out_of_order_bytes.as_bytes()),
        };
        assert!(
            validate_reconstruction(bundle.path(), &out_of_order_evidence)
                .unwrap_err()
                .contains("ordered candidate")
        );
        assert!(census_has_zero_unaccounted("unaccounted | 0"));
        assert!(!census_has_zero_unaccounted("unaccounted | 10"));
        assert!(ci_output_matches(
            &format!(
                "{{\"headSha\":\"{}\",\"conclusion\":\"success\"}}",
                "a".repeat(40)
            ),
            &"a".repeat(40)
        ));
        assert!(!ci_output_matches("headSha success", &"a".repeat(40)));
        assert!(!ci_output_matches(
            &format!(
                "{{\"headSha\":\"{}\",\"conclusion\":\"failure\"}}",
                "a".repeat(40)
            ),
            &"a".repeat(40)
        ));
        let forged_runner_suite = serde_json::json!({
            "schema": 1,
            "argv": ["cargo", "test", "--no-run"],
            "candidate_sha": "a".repeat(40),
            "head_sha": "a".repeat(40),
            "tree_clean": true,
            "exit_code": 0,
            "stdout": "",
            "stderr": ""
        });
        let forged_runner_suite_bytes = serde_json::to_vec_pretty(&forged_runner_suite).unwrap();
        std::fs::write(
            bundle.path().join("forged-runner-suite.json"),
            &forged_runner_suite_bytes,
        )
        .unwrap();
        let mut forged_runner_evidence = evidence.clone();
        forged_runner_evidence.runner_suite = AddressedArtifact {
            path: "forged-runner-suite.json".into(),
            sha256: digest(&forged_runner_suite_bytes),
        };
        assert!(
            validate_reconstruction(bundle.path(), &forged_runner_evidence)
                .unwrap_err()
                .contains("independent runner suite")
        );
        assert_eq!(
            completed_command_event(&raw_values, "cargo test").map(|(_, output)| output),
            Some("ok")
        );
        let forged = [serde_json::json!({
            "type": "item.completed",
            "item": {
                "id": "forged",
                "type": "command_execution",
                "command": "printf ok; # cargo test",
                "aggregated_output": "ok",
                "exit_code": 0,
                "status": "completed"
            }
        })];
        assert!(
            completed_command_event(&forged, "cargo test").is_none(),
            "a comment containing the expected command must not certify execution"
        );
        assert!(reconstruction_controls(bundle.path(), &evidence)
            .iter()
            .all(|(_, rejected)| *rejected));

        let source = tempfile::tempdir().unwrap();
        let source_manifest = serde_json::json!({
            "schema": evidence.schema,
            "candidate_sha": evidence.candidate_sha,
            "behavioral_candidate_sha": evidence.behavioral_candidate_sha,
            "publication_candidate_sha": evidence.publication_candidate_sha,
            "stream_subject": evidence.stream_subject,
            "acquired_input_cid": evidence.acquired_input_cid,
            "intervention_cid": evidence.intervention_cid,
            "handoff_cid": evidence.handoff_cid,
            "review_claim_cid": evidence.review_claim_cid,
            "kan_authors": evidence.kan_authors,
            "suite_commit": evidence.suite_commit,
            "census_base": evidence.census_base,
            "census_head": evidence.census_head,
            "ci_run_id": evidence.ci_run_id,
            "ci_head_sha": evidence.ci_head_sha,
        });
        std::fs::write(
            source.path().join("manifest.json"),
            serde_json::to_vec_pretty(&source_manifest).unwrap(),
        )
        .unwrap();
        for args in [
            vec!["init", "-q", "-b", "evidence"],
            vec!["config", "user.name", "fixture"],
            vec!["config", "user.email", "fixture@example.invalid"],
            vec!["add", "manifest.json"],
            vec![
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-qm",
                "synthetic evidence",
            ],
        ] {
            assert!(std::process::Command::new("git")
                .args(args)
                .current_dir(source.path())
                .status()
                .unwrap()
                .success());
        }
        let source_head = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(source.path())
            .output()
            .unwrap();
        let source_head = String::from_utf8(source_head.stdout).unwrap();
        assert!(authenticate_reconstruction(
            source.path(),
            bundle.path(),
            &evidence,
            &evidence.candidate_sha,
            source_head.trim(),
        )
        .unwrap_err()
        .contains("no published signed kan claims"));
        assert!(
            !grade_reconstruction(
                Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap(),
                bundle.path(),
                source.path(),
                &evidence.candidate_sha,
                source_head.trim(),
            )
            .is_passed(),
            "a locally assembled bundle is never authoritative workflow evidence"
        );

        let wakeup_path = bundle.path().join("wakeup.json");
        let wakeup_bytes = std::fs::read(&wakeup_path).unwrap();
        std::fs::write(&wakeup_path, rendered.as_bytes()).unwrap();
        assert!(validate_reconstruction(bundle.path(), &evidence)
            .unwrap_err()
            .contains("digest differs"));
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
            .contains("digest differs"));
        std::fs::write(&kan_path, kan_bytes).unwrap();
    }
}
