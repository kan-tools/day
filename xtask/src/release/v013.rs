//! Thin composition for the v0.13 release instance.
//!
//! Generic validation belongs outside this module. The real-harness trials are
//! observational artifacts retained for human assessment, not candidate gates.

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
];
pub const OBSERVATIONAL_WORKFLOWS: &[&str] = &[
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
    pub observational_workflows: Vec<String>,
    pub evidence_protocols: Vec<String>,
    pub publication_artifacts: Vec<String>,
}

pub fn contract() -> Contract {
    Contract {
        schema: 1,
        release: VERSION.into(),
        issues: ISSUES.to_vec(),
        workflows: strings(WORKFLOWS),
        observational_workflows: strings(OBSERVATIONAL_WORKFLOWS),
        evidence_protocols: strings(EVIDENCE_PROTOCOLS),
        publication_artifacts: strings(PUBLICATION_ARTIFACTS),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}

pub fn verify_manifest(root: &Path, path: &Path) -> Outcome<()> {
    let path = root.join(path);
    let actual: Contract = match read(&path).and_then(|bytes| json(&path, &bytes)) {
        Ok(actual) => actual,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    if actual != contract() {
        return Outcome::Finding(Finding::new(
            "v0.13 manifest does not equal the typed release contract",
        ));
    }
    for relative in WORKFLOWS
        .iter()
        .chain(OBSERVATIONAL_WORKFLOWS)
        .chain(EVIDENCE_PROTOCOLS)
    {
        if !root.join(relative).is_file() {
            return Outcome::Finding(Finding::new(format!(
                "typed v0.13 dependency `{relative}` is missing"
            )));
        }
    }
    println!(
        "v0.13 contract: {} candidate gates, {} observational workflows, {} protocols",
        WORKFLOWS.len(),
        OBSERVATIONAL_WORKFLOWS.len(),
        EVIDENCE_PROTOCOLS.len()
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
    let manifest: PlanManifest = match read(&manifest_path).and_then(|b| json(&manifest_path, &b)) {
        Ok(manifest) => manifest,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    let request = ProcessRequest::new("kan", ["show", &manifest.subject, "--json"], root);
    let output = match process.run(&request) {
        Ok(output) if output.status == 0 => output.stdout,
        Ok(output) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "could not resolve v0.13 Plan through kan (exit {}): {}",
                output.status, output.stderr
            )))
        }
        Err(error) => return Outcome::CouldNotCheck(CouldNotCheck::new(error)),
    };
    let show: KanShow = match serde_json::from_str(&output) {
        Ok(show) => show,
        Err(error) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "kan returned an unreadable Plan view: {error}"
            )))
        }
    };
    let Some(claim) = show.claims.iter().find(|claim| claim.cid == manifest.cid) else {
        return Outcome::Finding(Finding::new(format!(
            "published kan view does not contain selected Plan CID {}",
            manifest.cid
        )));
    };
    let artifact_spec = format!("{}:{}", manifest.artifact.commit, manifest.artifact.path);
    let artifact = match checked(
        process,
        root,
        "git",
        ["show", &artifact_spec],
        "Plan artifact",
    ) {
        Ok(value) => value.into_bytes(),
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    let current = match read(&root.join(&manifest.artifact.path)) {
        Ok(value) => value,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    let published = match read(&root.join(&manifest.published_file)) {
        Ok(value) => value,
        Err(error) => return Outcome::Finding(Finding::new(error)),
    };
    match validate_plan(&manifest, claim, &artifact, &current, &published) {
        Ok(()) => {
            println!(
                "v0.13 Plan resolved: cid={} artifact={artifact_spec}",
                manifest.cid
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
        return Err("selected claim does not match the Plan identity contract".into());
    }
    let commit = format!("Commit(\"{}\")", manifest.artifact.commit);
    let file = format!(
        "FileAt(\"{}\", \"{}\")",
        manifest.artifact.path, manifest.artifact.commit
    );
    if !claim.artifacts.contains(&commit) || !claim.artifacts.contains(&file) {
        return Err("selected Plan lacks its exact commit and FileAt artifacts".into());
    }
    if digest(artifact) != manifest.artifact.sha256 || artifact != current {
        return Err("resolved Plan artifact differs from its digest or current mirror".into());
    }
    if Some(artifact) != claim.text.as_deref().map(str::as_bytes) {
        return Err("Plan narrative is not byte-identical to its addressed artifact".into());
    }
    if !String::from_utf8_lossy(published).contains(&manifest.cid) {
        return Err("published claim file does not contain the selected Plan CID".into());
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
    let result = (|| {
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
    })();
    match result {
        Ok(runs) => {
            println!(
                "v0.13 candidate verified: {} deterministic workflows",
                runs.len()
            );
            Outcome::Passed(())
        }
        Err(error) => Outcome::Finding(Finding::new(error)),
    }
}

pub fn verify_publication(root: &Path, candidate: &str, process: &dyn Process) -> Outcome<()> {
    match verify_publication_inner(root, candidate, process) {
        Ok(run) => {
            println!("v0.13 publication verified at {candidate}; release run {run}");
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
    let tag = checked(
        process,
        root,
        "git",
        ["rev-list", "-n", "1", VERSION],
        "release tag",
    )?;
    if tag.trim() != candidate {
        return Err(format!(
            "release tag {VERSION} does not target candidate {candidate}"
        ));
    }
    let release_run =
        workflow_runs(root, candidate, &[".github/workflows/release.yml"], process)?[0].1;
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
    let url = format!("https://crates.io/api/v1/crates/day/{version}");
    let crate_json = checked(
        process,
        root,
        "curl",
        ["--fail", "--silent", "--show-error", &url],
        "crates.io package",
    )?;
    let package: CratesResponse = serde_json::from_str(&crate_json)
        .map_err(|error| format!("crates.io response is malformed: {error}"))?;
    if package.version.num != version {
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
    if !claims.claims.iter().any(|claim| {
        claim.kind == "Result"
            && claim
                .text
                .as_deref()
                .is_some_and(|text| text.contains(VERSION) && text.contains(candidate))
    }) {
        return Err("kan has no Result binding the release tag to the candidate".into());
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
        let Some(run) = runs.iter().find(|run| {
            run.database_id != 0
                && run.head_sha == candidate
                && run.status == "completed"
                && run.conclusion == "success"
        }) else {
            return Err(format!(
                "workflow `{workflow}` has no completed successful run at {candidate}"
            ));
        };
        verified.push((*workflow, run.database_id));
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
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(format!(
            "could not resolve {label} (exit {}): {}",
            output.status,
            output.stderr.trim()
        ))
    }
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
    fn observational_trials_are_not_candidate_gates() {
        assert!(WORKFLOWS
            .iter()
            .all(|workflow| !OBSERVATIONAL_WORKFLOWS.contains(workflow)));
        assert_eq!(WORKFLOWS.len(), 4);
        assert_eq!(OBSERVATIONAL_WORKFLOWS.len(), 2);
    }
}
