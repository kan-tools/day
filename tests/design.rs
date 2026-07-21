//! `.design/design-atom-backing.md` AC-1..AC-8 — the design/review/next
//! surface, driven through the real `day` binary against a stub kan.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{appends, atom_claim, claim, schema_claim, write_kan_stub};

const DOC: &str = "# Feature: a thing\n\n\
    ## Summary\nIt does the thing.\n\n\
    ## Requirements\n- REQ-1: first\n- REQ-2: second\n\n\
    ## Acceptance Criteria\n- [ ] AC-1: covers (REQ-1)\n- [ ] AC-2: covers (REQ-2)\n\n\
    ## Architecture\nTouches `src/design.rs`.\n\n\
    ## Resolved Questions\n- **Q1**: chose the first thing\n- **Q2**: chose the second thing\n";

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("failed to run day")
}

/// A workspace containing a design doc plus the `src/design.rs` path it
/// references, so path-existence checks have something real to resolve.
fn workspace(doc: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/design.rs"), "// fixture\n").unwrap();
    std::fs::create_dir_all(dir.path().join(".design")).unwrap();
    std::fs::write(dir.path().join(".design/thing.md"), doc).unwrap();
    dir
}

#[test]
fn ac1_check_passes_a_clean_doc_and_names_a_missing_section() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    assert!(
        out.status.success(),
        "clean doc should pass: {}",
        String::from_utf8_lossy(&out.stdout)
    );

    std::fs::write(
        dir.path().join(".design/thing.md"),
        DOC.replace("## Architecture", "## Notes"),
    )
    .unwrap();
    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("section missing: Architecture"), "{stdout}");
}

#[test]
fn ac2_revising_the_schema_claim_changes_the_result_with_no_file_edited() {
    let dir = workspace(DOC);
    let mut schema = day::schema::Schema::starter();
    schema.sections.push("Threat Model".to_string());
    let revised = claim(
        "schema/design-doc",
        "bafyreirevised",
        &format!(
            "Revised schema.\n\n```day-schema\n{}\n```\n",
            serde_json::to_string(&schema).unwrap()
        ),
    );
    let kan = write_kan_stub(
        dir.path(),
        &[schema_claim("design-doc", "bafyreischema"), revised],
    );

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "the newest schema claim should win");
    assert!(stdout.contains("section missing: Threat Model"), "{stdout}");
}

/// AC-3, amended during implementation: an uncovered requirement **warns**
/// rather than fails. Running this check over kan's own design docs showed
/// several map criteria to requirements positionally (AC-1 covers REQ-1)
/// instead of naming them — a convention the check cannot detect, so failing
/// on it would be failing on a false signal. Recorded as a decide claim on
/// the `design-atom-backing` subject rather than changed silently.
#[test]
fn ac3_an_uncovered_requirement_is_named() {
    let dir = workspace(&DOC.replace("- [ ] AC-2: covers (REQ-2)", "- [ ] AC-2: covers something"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("REQ-2 is not named by any acceptance criterion"),
        "{stdout}"
    );
    assert!(
        out.status.success(),
        "an unnamed requirement warns; it does not fail the document"
    );
}

#[test]
fn ac4_a_referenced_path_that_does_not_exist_is_named() {
    let dir = workspace(&DOC.replace("`src/design.rs`", "`src/nope.rs`"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("src/nope.rs"), "{stdout}");
}

#[test]
fn check_without_a_declared_schema_explains_and_offers_a_starter() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(
        stderr.contains("no design-doc schema is declared"),
        "{stderr}"
    );
    assert!(
        stderr.contains("kan observe") && stderr.contains("day-schema"),
        "the error should hand over a runnable starter: {stderr}"
    );
}

#[test]
fn ac5_record_appends_observe_then_plan_then_one_decide_per_resolved_question() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "record", ".design/thing.md"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let log = appends(dir.path());
    assert_eq!(
        log.len(),
        4,
        "expected observe + plan + 2 decides, got {log:#?}"
    );
    assert!(log[0].starts_with("observe "), "{}", log[0]);
    assert!(log[1].starts_with("plan "), "{}", log[1]);
    assert!(log[2].starts_with("decide "), "{}", log[2]);
    assert!(log[3].starts_with("decide "), "{}", log[3]);

    // The chain is real: plan cites the observe's CID, decides cite the
    // plan's -- and every --cites value is a CID, never a path.
    assert!(log[1].contains("--cites bafyreistub00000001"), "{}", log[1]);
    assert!(log[2].contains("--cites bafyreistub00000002"), "{}", log[2]);
    assert!(log[3].contains("--cites bafyreistub00000002"), "{}", log[3]);
    for line in &log {
        assert!(
            !line.contains("--cites .design"),
            "a file path must never be passed to --cites: {line}"
        );
    }
    assert!(log.iter().all(|l| l.contains("--subject thing")));
}

#[test]
fn ac6_a_failing_doc_is_still_recorded_with_the_result_embedded() {
    let dir = workspace(&DOC.replace("## Architecture", "## Notes"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "record", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "a failing doc must still record -- gating would just mean it goes unrecorded"
    );
    assert!(stdout.contains("recorded anyway"), "{stdout}");

    let log = appends(dir.path());
    let plan = log
        .iter()
        .find(|l| l.starts_with("plan "))
        .expect("a plan claim");
    assert!(
        plan.contains("failed") && plan.contains("validation:"),
        "the plan claim should carry the validation result: {plan}"
    );
}

#[test]
fn ac7_review_record_rejects_a_bad_verdict_and_an_uncited_one() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "thing",
            "--verdict",
            "LGTM",
            "--rationale",
            "looks fine",
            "--cites",
            "bafyreisomething",
        ],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("verdict must be one of"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(appends(dir.path()).is_empty(), "nothing should be appended");

    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "thing",
            "--verdict",
            "APPROVE",
            "--rationale",
            "fine",
        ],
    );
    assert!(
        !out.status.success(),
        "a verdict with no --cites must be rejected"
    );
    assert!(appends(dir.path()).is_empty());
}

#[test]
fn ac7_review_record_accepts_each_permitted_verdict() {
    for verdict in day::record::VERDICTS {
        let dir = workspace(DOC);
        let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);
        let out = day(
            dir.path(),
            &kan,
            &[
                "review",
                "record",
                "thing",
                "--verdict",
                verdict,
                "--rationale",
                "because",
                "--cites",
                "bafyreidesignclaim",
            ],
        );
        assert!(out.status.success(), "verdict {verdict} should be accepted");
        let log = appends(dir.path());
        assert_eq!(log.len(), 1);
        assert!(log[0].contains(verdict), "{}", log[0]);
        assert!(log[0].contains("--cites bafyreidesignclaim"), "{}", log[0]);
    }
}

#[test]
fn ac8_next_names_the_successor_and_what_it_needs() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_claim(
                "design",
                "bafyreid1",
                &["intent"],
                &["design-doc"],
                &["generative-build"],
            ),
            atom_claim(
                "generative-build",
                "bafyreid2",
                &["design-doc"],
                &["code-change"],
                &["adversarial-review"],
            ),
            atom_claim(
                "adversarial-review",
                "bafyreid3",
                &["design-doc", "code-change"],
                &["verdict"],
                &[],
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["next", "design"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(stdout.contains("atom/generative-build"), "{stdout}");
    assert!(stdout.contains("design-doc"), "{stdout}");
    assert!(
        !stdout.contains("adversarial-review"),
        "next should report the immediate successor, not the whole pipeline: {stdout}"
    );

    let out = day(dir.path(), &kan, &["next", "adversarial-review"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(stdout.contains("no successors"), "{stdout}");
}
