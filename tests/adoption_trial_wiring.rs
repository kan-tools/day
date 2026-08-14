use std::path::PathBuf;

fn workflow() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(".github/workflows/adoption-reliability-trial.yml"))
        .expect("the preregistered trial runner must ship")
}

#[test]
fn the_actions_runner_covers_every_preregistered_row_and_retains_evidence() {
    let workflow = workflow();
    for required in [
        "agent-skills-unix:",
        "ubuntu-latest",
        "macos-latest",
        "agent-skills-windows:",
        "windows-latest",
        "claude-node-free-adapter:",
        "node absent from adapter PATH",
        "controls:",
        "HEAD-DID-NOT-BUILD",
        "BASE-DID-NOT-BUILD",
        "DAY_TRIAL_EVIDENCE_DIR",
        "evidence-complete:",
    ] {
        assert!(
            workflow.contains(required),
            "trial runner omits `{required}`"
        );
    }
    assert_eq!(
        workflow.matches("actions/upload-artifact@v4").count(),
        7,
        "package metadata, three consumer rows, Claude, controls, and the final summary must all be retained"
    );
    assert!(
        workflow.contains("ref: ${{ inputs.rc_sha }}")
            && workflow.contains("git rev-parse HEAD")
            && workflow.contains("cargo package --locked"),
        "the trial must package and exercise the caller's exact RC, not a source-tree binary"
    );
}

#[test]
fn a_green_workflow_is_not_written_as_an_attestation() {
    let workflow = workflow();
    assert!(workflow.contains("workflow_dispatch:"));
    assert!(
        !workflow.contains("kan result") && !workflow.contains("adoption reliability trial PASSED"),
        "Actions has no kan workspace and must only produce reviewable evidence"
    );
}
