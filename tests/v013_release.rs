use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(root().join(path)).unwrap()
}

#[test]
fn candidate_gates_and_observational_trials_are_distinct() {
    let manifest: serde_json::Value = serde_json::from_str(&text(".release/v0.13.json")).unwrap();
    assert_eq!(
        manifest["issues"],
        serde_json::json!([93, 143, 152, 193, 195, 204])
    );
    assert_eq!(manifest["workflows"].as_array().unwrap().len(), 4);
    assert_eq!(
        manifest["observational_workflows"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(manifest["evidence_protocols"].as_array().unwrap().len(), 2);

    let adapter = text("xtask/src/release/v013.rs");
    assert!(adapter.contains("OBSERVATIONAL_WORKFLOWS"));
    for forbidden in [
        "grade_askme",
        "grade_reconstruction",
        "removal_controls",
        "GithubActionsOrigin",
        "RawEvent",
        "Receipt",
        "struct PlanManifest",
        "struct WorkflowRun",
        "serde_json::from_str",
        "ProcessRequest",
    ] {
        assert!(
            !adapter.contains(forbidden),
            "version adapter contains `{forbidden}`"
        );
    }
}

#[test]
fn askme_trial_retains_raw_observations_without_claiming_a_grade() {
    let workflow = text(".github/workflows/askme-behavioral-trial.yml");
    let runner = text("scripts/run-v013-askme-trial.py");
    let protocol = text(".release/protocols/askme-v1.json");
    for source in [&workflow, &runner, &protocol] {
        assert!(source.contains("observational"));
    }
    assert!(runner.contains("raw_events"));
    assert!(runner.contains("kan-before.json") && runner.contains("kan-after.json"));
    assert!(runner.contains("producer_assumption"));
    assert!(!workflow.contains("grade-askme-v013"));
    assert!(!runner.contains("/opt/") && !runner.contains("sudo"));
}

#[test]
fn reconstruction_is_a_post_merge_observation_not_an_attestation() {
    let workflow = text(".github/workflows/workflow-reconstruction-trial.yml");
    let runner = text("scripts/run-v013-reconstruction-trial.py");
    let protocol: serde_json::Value =
        serde_json::from_str(&text(".release/protocols/reconstruction-v1.json")).unwrap();
    assert!(workflow.contains("post-merge"));
    assert!(workflow.contains("no conversation transcript"));
    assert!(!workflow.contains("grade-reconstruction-v013"));
    assert!(runner.contains("observational-trial"));
    assert!(runner.contains("honest-producer"));
    assert!(!runner.contains("/opt/") && !runner.contains("removal"));
    assert_eq!(protocol["authoritative"], false);
    assert_eq!(protocol["lifecycle"], "post-merge");
}

#[test]
fn instrumentation_policy_is_checked_in_ci() {
    let profile = text("xtask/src/profile.rs");
    let inventory = text(".release/instrumentation.json");
    assert!(profile.contains("instrumentation"));
    assert!(inventory.contains("does_not_establish"));
    assert!(inventory.contains("default_threat_model"));
}

#[test]
fn release_policy_does_not_leak_into_the_public_day_cli() {
    let cli = text("src/cli/mod.rs");
    for forbidden in ["ReleaseAction", "TrialAction", "V013Action", "verify-v013"] {
        assert!(!cli.contains(forbidden), "day CLI contains `{forbidden}`");
    }
}
