//! Version-neutral release-coordinate validation.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanManifest {
    pub schema: u64,
    pub cid: String,
    pub subject: String,
    pub rfc_result: String,
    pub normative_source: String,
    pub artifact: PlanArtifact,
    pub published_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanArtifact {
    pub commit: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy)]
pub struct PlanIdentity<'a> {
    pub subject: &'a str,
    pub rfc_result: &'a str,
    pub normative_source: &'a str,
    pub artifact_path: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct PublicationIdentity<'a> {
    pub version: &'a str,
    pub repository: &'a str,
    pub crate_name: &'a str,
    pub release_workflow: &'a str,
    pub release_subject: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct KanShow {
    claims: Vec<KanClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct KanClaim {
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

#[derive(Debug, Deserialize)]
struct CargoVcsInfo {
    git: CargoGit,
}

#[derive(Debug, Deserialize)]
struct CargoGit {
    sha1: String,
    dirty: bool,
}

enum CheckError {
    Finding(String),
    CouldNotCheck(String),
}

#[derive(Clone, Copy)]
enum Nonzero {
    Finding,
    CouldNotCheck,
}

impl CheckError {
    fn finding(message: impl Into<String>) -> Self {
        Self::Finding(message.into())
    }

    fn unavailable(message: impl Into<String>) -> Self {
        Self::CouldNotCheck(message.into())
    }

    fn outcome(self) -> Outcome<()> {
        match self {
            Self::Finding(message) => Outcome::Finding(Finding::new(message)),
            Self::CouldNotCheck(message) => Outcome::CouldNotCheck(CouldNotCheck::new(message)),
        }
    }
}

pub fn verify_plan(
    root: &Path,
    manifest_path: &Path,
    identity: PlanIdentity<'_>,
    process: &dyn Process,
) -> Outcome<()> {
    match verify_plan_inner(root, manifest_path, identity, process) {
        Ok(manifest) => {
            println!(
                "Plan resolved: cid={} artifact={}:{}",
                manifest.cid, manifest.artifact.commit, manifest.artifact.path
            );
            Outcome::Passed(())
        }
        Err(error) => error.outcome(),
    }
}

fn verify_plan_inner(
    root: &Path,
    manifest_path: &Path,
    identity: PlanIdentity<'_>,
    process: &dyn Process,
) -> Result<PlanManifest, CheckError> {
    let path = root.join(manifest_path);
    let bytes = std::fs::read(&path).map_err(|error| {
        CheckError::unavailable(format!("could not read `{}`: {error}", path.display()))
    })?;
    let manifest: PlanManifest = serde_json::from_slice(&bytes).map_err(|error| {
        CheckError::finding(format!("`{}` is malformed: {error}", path.display()))
    })?;
    if manifest.schema != 1
        || manifest.subject != identity.subject
        || manifest.rfc_result != identity.rfc_result
        || manifest.normative_source != identity.normative_source
        || manifest.artifact.path != identity.artifact_path
    {
        return Err(CheckError::finding(
            "Plan manifest changed a selected identity coordinate",
        ));
    }
    full_sha("Plan artifact commit", &manifest.artifact.commit).map_err(CheckError::finding)?;

    let kan = run_checked(
        process,
        root,
        "kan",
        ["show", manifest.subject.as_str(), "--json"],
        "Plan kan view",
        Nonzero::CouldNotCheck,
    )?;
    let show: KanShow = parse_external("Plan kan view", &kan)?;
    let claim = show
        .claims
        .iter()
        .find(|claim| claim.cid == manifest.cid)
        .ok_or_else(|| CheckError::finding("published kan view lacks the selected Plan CID"))?;
    if claim.kind != "Plan"
        || claim.subject != manifest.subject
        || !claim.cites.contains(&manifest.rfc_result)
        || !claim
            .text
            .as_deref()
            .is_some_and(|text| text.contains(&manifest.normative_source))
    {
        return Err(CheckError::finding(
            "selected claim does not match the Plan identity",
        ));
    }

    let artifact_spec = format!("{}:{}", manifest.artifact.commit, manifest.artifact.path);
    let artifact = run_checked(
        process,
        root,
        "git",
        ["show", &artifact_spec],
        "Plan artifact",
        Nonzero::Finding,
    )?
    .into_bytes();
    let current = std::fs::read(root.join(&manifest.artifact.path)).map_err(|error| {
        CheckError::unavailable(format!("could not read current Plan: {error}"))
    })?;
    let published = std::fs::read(root.join(&manifest.published_file)).map_err(|error| {
        CheckError::unavailable(format!("could not read published Plan claims: {error}"))
    })?;
    let commit = format!("Commit(\"{}\")", manifest.artifact.commit);
    let file = format!(
        "FileAt(\"{}\", \"{}\")",
        manifest.artifact.path, manifest.artifact.commit
    );
    if !claim.artifacts.contains(&commit) || !claim.artifacts.contains(&file) {
        return Err(CheckError::finding(
            "selected Plan lacks exact artifact coordinates",
        ));
    }
    if digest(&artifact) != manifest.artifact.sha256 || artifact != current {
        return Err(CheckError::finding(
            "resolved Plan differs from its digest or mirror",
        ));
    }
    if Some(artifact.as_slice()) != claim.text.as_deref().map(str::as_bytes) {
        return Err(CheckError::finding(
            "Plan narrative differs from its addressed artifact",
        ));
    }
    if !String::from_utf8_lossy(&published).contains(&manifest.cid) {
        return Err(CheckError::finding(
            "published claim file lacks the selected Plan CID",
        ));
    }
    Ok(manifest)
}

pub fn verify_candidate(
    root: &Path,
    candidate: &str,
    repository: &str,
    workflows: &[&'static str],
    process: &dyn Process,
) -> Outcome<()> {
    let result = (|| {
        full_sha("candidate SHA", candidate).map_err(CheckError::finding)?;
        let head = run_checked(
            process,
            root,
            "git",
            ["rev-parse", "HEAD"],
            "local HEAD",
            Nonzero::CouldNotCheck,
        )?;
        if head.trim() != candidate {
            return Err(CheckError::finding(format!(
                "local HEAD {} differs from candidate {candidate}",
                head.trim()
            )));
        }
        let status = run_checked(
            process,
            root,
            "git",
            ["status", "--porcelain"],
            "working-tree status",
            Nonzero::CouldNotCheck,
        )?;
        if !status.trim().is_empty() {
            return Err(CheckError::finding("candidate working tree is dirty"));
        }
        workflow_runs(root, candidate, repository, workflows, process)
    })();
    match result {
        Ok(runs) => {
            println!("candidate verified: {} deterministic workflows", runs.len());
            Outcome::Passed(())
        }
        Err(error) => error.outcome(),
    }
}

pub fn verify_publication(
    root: &Path,
    candidate: &str,
    identity: PublicationIdentity<'_>,
    process: &dyn Process,
) -> Outcome<()> {
    match verify_publication_inner(root, candidate, identity, process) {
        Ok(run) => {
            println!("publication verified at {candidate}; release run {run}");
            Outcome::Passed(())
        }
        Err(error) => error.outcome(),
    }
}

fn verify_publication_inner(
    root: &Path,
    candidate: &str,
    identity: PublicationIdentity<'_>,
    process: &dyn Process,
) -> Result<u64, CheckError> {
    full_sha("candidate SHA", candidate).map_err(CheckError::finding)?;
    let tag = run_checked(
        process,
        root,
        "git",
        ["rev-list", "-n", "1", identity.version],
        "release tag",
        Nonzero::Finding,
    )?;
    if tag.trim() != candidate {
        return Err(CheckError::finding(
            "release tag does not target the candidate",
        ));
    }
    let run = workflow_runs(
        root,
        candidate,
        identity.repository,
        &[identity.release_workflow],
        process,
    )?[0]
        .1;
    let release_json = run_checked(
        process,
        root,
        "gh",
        [
            "release",
            "view",
            identity.version,
            "--repo",
            identity.repository,
            "--json",
            "tagName,isDraft",
        ],
        "GitHub Release",
        Nonzero::CouldNotCheck,
    )?;
    let release: GithubRelease = parse_external("GitHub Release", &release_json)?;
    if release.tag_name != identity.version || release.is_draft {
        return Err(CheckError::finding(
            "GitHub Release is absent, draft, or names another tag",
        ));
    }
    let version = identity.version.trim_start_matches('v');
    let url = format!(
        "https://crates.io/api/v1/crates/{}/{version}",
        identity.crate_name
    );
    let crate_json = run_checked(
        process,
        root,
        "curl",
        ["--fail", "--silent", "--show-error", &url],
        "crates.io package",
        Nonzero::CouldNotCheck,
    )?;
    let package: CratesResponse = parse_external("crates.io package", &crate_json)?;
    if package.version.num != version {
        return Err(CheckError::finding(
            "crates.io returned another package version",
        ));
    }
    let archive_dir = tempfile::tempdir().map_err(|error| {
        CheckError::unavailable(format!(
            "could not create crate verification directory: {error}"
        ))
    })?;
    let archive = archive_dir
        .path()
        .join(format!("{}-{version}.crate", identity.crate_name));
    let download = format!(
        "https://crates.io/api/v1/crates/{}/{version}/download",
        identity.crate_name
    );
    run_checked(
        process,
        root,
        "curl",
        [
            "--fail",
            "--silent",
            "--show-error",
            "--location",
            "--output",
            archive
                .to_str()
                .ok_or_else(|| CheckError::unavailable("crate verification path is not UTF-8"))?,
            &download,
        ],
        "crates.io package download",
        Nonzero::CouldNotCheck,
    )?;
    let vcs_path = format!("{}-{version}/.cargo_vcs_info.json", identity.crate_name);
    let vcs_json = run_checked(
        process,
        root,
        "tar",
        ["-xOf", archive.to_str().unwrap(), &vcs_path],
        "published crate VCS metadata",
        Nonzero::Finding,
    )?;
    let vcs: CargoVcsInfo = serde_json::from_str(&vcs_json).map_err(|error| {
        CheckError::finding(format!(
            "published crate VCS metadata is malformed: {error}"
        ))
    })?;
    if vcs.git.sha1 != candidate || vcs.git.dirty {
        return Err(CheckError::finding(
            "published crate VCS metadata does not bind the clean candidate SHA",
        ));
    }
    let claim_json = run_checked(
        process,
        root,
        "kan",
        ["show", identity.release_subject, "--json"],
        "kan release claim",
        Nonzero::CouldNotCheck,
    )?;
    let claims: KanShow = parse_external("kan release claim", &claim_json)?;
    let binding = format!("{} candidate {} — ", identity.version, candidate);
    if !claims.claims.iter().any(|claim| {
        claim.kind == "Result"
            && claim
                .text
                .as_deref()
                .is_some_and(|text| text.starts_with(&binding))
    }) {
        return Err(CheckError::finding(
            "kan has no canonical Result binding tag and candidate",
        ));
    }
    Ok(run)
}

fn workflow_runs<'a>(
    root: &Path,
    candidate: &str,
    repository: &str,
    workflows: &[&'a str],
    process: &dyn Process,
) -> Result<Vec<(&'a str, u64)>, CheckError> {
    let mut verified = Vec::new();
    for workflow in workflows {
        let output = run_checked(
            process,
            root,
            "gh",
            [
                "run",
                "list",
                "--repo",
                repository,
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
            Nonzero::CouldNotCheck,
        )?;
        let runs: Vec<WorkflowRun> = parse_external(&format!("workflow {workflow}"), &output)?;
        let run = runs.iter().find(|run| {
            run.database_id != 0
                && run.head_sha == candidate
                && run.status == "completed"
                && run.conclusion == "success"
        });
        match run {
            Some(run) => verified.push((*workflow, run.database_id)),
            None => {
                return Err(CheckError::finding(format!(
                    "workflow `{workflow}` has no completed successful run at {candidate}"
                )))
            }
        }
    }
    Ok(verified)
}

fn run_checked<I, S>(
    process: &dyn Process,
    root: &Path,
    program: &str,
    args: I,
    label: &str,
    nonzero: Nonzero,
) -> Result<String, CheckError>
where
    I: IntoIterator<Item = S>,
    S: Into<std::ffi::OsString>,
{
    let output = process
        .run(&ProcessRequest::new(program, args, root))
        .map_err(|error| CheckError::unavailable(format!("could not resolve {label}: {error}")))?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        let message = format!(
            "could not resolve {label} (exit {}): {}",
            output.status,
            output.stderr.trim()
        );
        match nonzero {
            Nonzero::Finding => Err(CheckError::finding(message)),
            Nonzero::CouldNotCheck => Err(CheckError::unavailable(message)),
        }
    }
}

fn parse_external<T: serde::de::DeserializeOwned>(
    label: &str,
    text: &str,
) -> Result<T, CheckError> {
    serde_json::from_str(text).map_err(|error| {
        CheckError::unavailable(format!("{label} returned unreadable JSON: {error}"))
    })
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
