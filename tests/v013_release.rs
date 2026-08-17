use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(root().join(path)).unwrap()
}

#[test]
fn the_candidate_contract_is_closed_and_repository_owned() {
    let manifest: serde_json::Value = serde_json::from_str(&text(".release/v0.13.json")).unwrap();
    assert_eq!(
        manifest["issues"],
        serde_json::json!([93, 143, 152, 193, 195, 204])
    );
    assert_eq!(manifest["workflows"].as_array().unwrap().len(), 6);
    assert_eq!(manifest["evidence_protocols"].as_array().unwrap().len(), 2);
    let xtask = text("xtask/src/release/v013.rs");
    assert!(xtask.contains("pub const ISSUES"));
    assert!(xtask.contains("actual != expected"));
}

#[test]
fn askme_trial_is_real_multiturn_and_retains_addressed_raw_evidence() {
    let workflow = text(".github/workflows/askme-behavioral-trial.yml");
    for required in [
        "candidate_sha:",
        "model:",
        "grade-askme-v013",
        "actions/upload-artifact@v4",
        "evidence/v0.13/askme-${{ github.run_id }}",
    ] {
        assert!(workflow.contains(required), "missing `{required}`");
    }
    let runner = text("scripts/run-v013-askme-trial.py");
    assert!(runner.contains("\"codex\", \"exec\""));
    assert!(runner.contains("\"codex\", \"exec\", \"resume\""));
    assert!(runner.contains("assistant_turns"));
    assert!(runner.contains("kan-before.json") && runner.contains("kan-after.json"));
    assert!(runner.contains("raw_events") && runner.contains("sha256(path)"));
    assert!(runner.contains("kan show --all --json exited"));
    assert!(!runner.contains("return {\"raw\": completed.stderr, \"claims\": 0"));
}

#[test]
fn reconstruction_trial_grades_an_addressed_commit_not_a_pass_marker() {
    let workflow = text(".github/workflows/workflow-reconstruction-trial.yml");
    assert!(workflow.contains("evidence_ref:"));
    assert!(workflow.contains("git archive --format=tar"));
    assert!(workflow.contains("grade-reconstruction-v013"));
    assert!(!workflow.contains("echo passed") && !workflow.contains("touch passed"));

    let protocol: serde_json::Value =
        serde_json::from_str(&text(".release/protocols/reconstruction-v1.json")).unwrap();
    assert_eq!(protocol["removal_controls"].as_array().unwrap().len(), 11);
}

#[test]
fn no_release_or_trial_policy_leaks_into_the_public_day_cli() {
    let cli = text("src/cli/mod.rs");
    for forbidden in ["ReleaseAction", "TrialAction", "V013Action", "verify-v013"] {
        assert!(!cli.contains(forbidden), "day CLI contains `{forbidden}`");
    }
}
