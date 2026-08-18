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
        "${{ github.run_id }}",
        "actions/upload-artifact@v4",
        "evidence/v0.13/askme-${{ github.run_id }}",
    ] {
        assert!(workflow.contains(required), "missing `{required}`");
    }
    assert!(
        workflow.find("export CODEX_HOME").unwrap()
            < workflow.find("codex login --with-api-key").unwrap(),
        "the isolated home must exist before Codex writes authentication"
    );
    let runner = text("scripts/run-v013-askme-trial.py");
    assert!(runner.contains("\"codex\", \"exec\""));
    assert!(runner.contains("\"codex\", \"exec\", \"resume\""));
    assert!(runner.contains("assistant_turns"));
    assert!(runner.contains("kan-before.json") && runner.contains("kan-after.json"));
    assert!(runner.contains("raw_events") && runner.contains("sha256(path)"));
    assert!(runner.contains("command_log") && runner.contains("sha256(command_log)"));
    assert!(runner.contains("github_run_id"));
    assert!(runner.contains("kan show --all --json exited"));
    assert!(!runner.contains("return {\"raw\": completed.stderr, \"claims\": 0"));
    let grader = text("xtask/src/release/v013.rs");
    assert!(grader.contains("raw Codex commands, wrapper log, and recording expectation differ"));
    assert!(grader.contains("did not append the exact expected claim set"));
    assert!(grader.contains("extract_fenced::<day::events::AcquiredInput>"));
}

#[test]
fn reconstruction_trial_grades_an_addressed_commit_not_a_pass_marker() {
    let workflow = text(".github/workflows/workflow-reconstruction-trial.yml");
    assert!(workflow.contains("evidence_ref:"));
    assert!(workflow.contains("model:"));
    assert!(workflow.contains("git worktree add --detach"));
    assert!(workflow.contains("test \"$resolved\" = \"$EVIDENCE_REF\""));
    assert!(workflow.contains("cargo install kan --version 0.13.0-beta.1 --locked"));
    assert!(workflow.contains("@openai/codex@0.147.0"));
    assert!(workflow.contains("run-v013-reconstruction-trial.py"));
    assert!(workflow.contains("Authenticate the fresh Codex harness"));
    assert!(workflow.contains("grade-reconstruction-v013"));
    assert!(
        workflow.contains("${{ inputs.candidate_sha }}") && workflow.contains("$EVIDENCE_COMMIT")
    );
    assert!(!workflow.contains("echo passed") && !workflow.contains("touch passed"));
    assert!(
        workflow.find("export CODEX_HOME").unwrap()
            < workflow.find("codex login --with-api-key").unwrap(),
        "reconstruction must authenticate the same isolated home it executes from"
    );

    let grader = text("xtask/src/release/v013.rs");
    assert!(grader.contains("evidence commit has no published signed kan claims"));
    assert!(grader.contains("V013_EVIDENCE_PRINCIPAL"));
    assert!(grader.contains("pinned review claim does not adjudicate"));
    assert!(grader.contains("exact kan-tools/day candidate GitHub workflow file"));
    assert!(grader.contains("origin.repository != \"kan-tools/day\""));
    assert!(grader.contains("GITHUB_WORKFLOW_REF"));
    assert!(grader.contains("workflow_dispatch"));
    assert!(
        grader.contains("addressed kan read differs from kan's authenticated signed-claim view")
    );
    assert!(
        grader.contains("fresh wakeup raw events do not prove the required bulk kan command ran")
    );
    assert!(
        grader.contains("independently recheck ordered candidate, suite, census, and CI scopes")
    );
    assert!(grader.contains("command.trim() == expected"));
    assert!(grader.contains("contains an item lifecycle without completion"));

    let runner = text("scripts/run-v013-reconstruction-trial.py");
    assert!(runner.contains("no prior conversation transcript"));
    assert!(runner.contains("wakeup-events.jsonl"));
    assert!(runner.contains("--trust"));
    assert!(runner.contains("candidate checkout HEAD differs"));
    assert!(runner.contains("git status --porcelain"));
    assert!(workflow.contains("$GITHUB_WORKSPACE"));
    assert!(workflow.contains("${{ github.run_id }}"));
    assert!(runner.contains("do not wrap or combine them"));
    assert!(runner.contains("/opt/day-v013-trusted-"));
    assert!(runner.contains("sudo\", \"install"));
    assert!(runner.contains("wrong candidate working directory"));
    assert!(runner.contains("wrong candidate HEAD"));
    assert!(runner.contains("candidate checkout is dirty"));
    assert!(runner.contains("shutil.copy2(candidate_day"));
    assert!(runner.contains("real_tools[\"codex\"]"));
    assert!(runner.contains("declared_suite(kan_path, source)"));
    assert!(runner.contains("shutil.rmtree(candidate_target)"));
    assert!(runner.contains("runner-suite.json"));
    assert!(grader.contains("independent runner suite evidence"));
    assert!(!workflow.contains("target/debug\" >> \"$GITHUB_PATH"));
    assert!(grader.contains("census_has_zero_unaccounted"));
    assert!(grader.contains("ci_output_matches"));
    assert!(grader.contains("status_index < kan_index"));
    assert!(grader.contains("GITHUB_WORKFLOW_SHA"));
    assert!(grader.contains("GITHUB_REF"));

    let protocol: serde_json::Value =
        serde_json::from_str(&text(".release/protocols/reconstruction-v1.json")).unwrap();
    assert_eq!(protocol["removal_controls"].as_array().unwrap().len(), 12);
}

#[test]
fn no_release_or_trial_policy_leaks_into_the_public_day_cli() {
    let cli = text("src/cli/mod.rs");
    for forbidden in ["ReleaseAction", "TrialAction", "V013Action", "verify-v013"] {
        assert!(!cli.contains(forbidden), "day CLI contains `{forbidden}`");
    }
}
