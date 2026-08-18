//! Thin composition for the v0.13 release instance.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::capability::process::Process;
use crate::outcome::{Finding, Outcome};
use crate::validate::release::{PlanIdentity, PublicationIdentity};

pub const VERSION: &str = "v0.13.0-beta.1";
const REPOSITORY: &str = "kan-tools/day";
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
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Outcome::Finding(Finding::new(format!(
                "could not read `{}`: {error}",
                path.display()
            )))
        }
    };
    let actual: Contract = match serde_json::from_slice(&bytes) {
        Ok(actual) => actual,
        Err(error) => {
            return Outcome::Finding(Finding::new(format!(
                "`{}` is malformed: {error}",
                path.display()
            )))
        }
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

pub fn verify_plan(root: &Path, manifest: &Path, process: &dyn Process) -> Outcome<()> {
    crate::validate::release::verify_plan(
        root,
        manifest,
        PlanIdentity {
            subject: "v0.13-workflow-ergonomics",
            rfc_result: "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
            normative_source:
                "35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md",
            artifact_path: ".design/v0.13-workflow-ergonomics.md",
        },
        process,
    )
}

pub fn verify_candidate(root: &Path, candidate: &str, process: &dyn Process) -> Outcome<()> {
    crate::validate::release::verify_candidate(root, candidate, REPOSITORY, WORKFLOWS, process)
}

pub fn verify_publication(root: &Path, candidate: &str, process: &dyn Process) -> Outcome<()> {
    crate::validate::release::verify_publication(
        root,
        candidate,
        PublicationIdentity {
            version: VERSION,
            repository: REPOSITORY,
            crate_name: "day",
            release_workflow: ".github/workflows/release.yml",
            release_subject: "release",
        },
        process,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observational_trials_are_not_candidate_gates() {
        assert!(WORKFLOWS
            .iter()
            .all(|workflow| !OBSERVATIONAL_WORKFLOWS.contains(workflow)));
    }
}
