//! day#193 / day#195 — explicit acquired-input and intervention records use
//! kan's write boundary, preserve the actual signer, and distinguish reported
//! provenance from a separately signed source claim.

#![cfg(unix)]

mod common;

use common::{appends, claim, write_kan_stub, STUB_AUTHOR};

fn run(dir: &std::path::Path, kan: &std::path::Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("day event command should run")
}

fn claim_text(append: &str) -> &str {
    append
        .strip_prefix("observe ")
        .and_then(|rest| rest.split_once(" --subject ").map(|(text, _)| text))
        .expect("stub append should contain observe text and subject")
}

fn payload(append: &str, fence: &str) -> serde_json::Value {
    let opening = format!("```{fence}\n");
    let json = claim_text(append)
        .split_once(&opening)
        .and_then(|(_, rest)| rest.split_once("\n```").map(|(json, _)| json))
        .expect("event write should contain the requested fenced JSON block");
    serde_json::from_str(json).expect("event block should be JSON")
}

#[test]
fn acquired_input_is_opt_in_structured_and_reported_without_false_authorship() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("basis", "cid-basis", "repository evidence")],
    );

    let out = run(
        dir.path(),
        &kan,
        &[
            "acquired-input",
            "record",
            "work/topic",
            "--topic",
            "release scope",
            "--reported-provider",
            "the human in this conversation",
            "--fact",
            "RFC 1 remains a guardrail",
            "--decision",
            "keep the critical path",
            "--unresolved",
            "kan identity delivery date",
            "--material-effect",
            "defer generic certification to v1.0",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let writes = appends(dir.path());
    assert_eq!(writes.len(), 1);
    let payload = payload(&writes[0], "day-acquired-input");
    assert_eq!(payload["_version"], 1);
    assert_eq!(payload["recorded_by"], STUB_AUTHOR);
    assert_eq!(payload["work_subject"], "work/topic");
    assert_eq!(payload["basis"], serde_json::json!(["cid-basis"]));
    assert_eq!(payload["provider"]["provenance"], "reported");
    assert_eq!(
        payload["provider"]["description"],
        "the human in this conversation"
    );
    assert!(writes[0].contains("--cites cid-basis"));
}

#[test]
fn reported_human_intervention_remains_authored_by_the_active_signer() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("basis", "cid-basis", "repository evidence")],
    );

    let out = run(
        dir.path(),
        &kan,
        &[
            "intervention",
            "record",
            "work/topic",
            "--kind",
            "answered-question",
            "--summary",
            "the human supplied the missing answer",
            "--material-effect",
            "implementation could continue",
            "--reported-source",
            "the human in this conversation",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let writes = appends(dir.path());
    assert_eq!(writes.len(), 1);
    let payload = payload(&writes[0], "day-intervention");
    assert_eq!(payload["recorded_by"], STUB_AUTHOR);
    assert_eq!(payload["source"]["provenance"], "reported");
    assert_eq!(
        payload["source"]["description"],
        "the human in this conversation"
    );
}

#[test]
fn separately_signed_source_derives_the_principal_and_is_cited() {
    let dir = tempfile::tempdir().unwrap();
    let mut source = claim("input/source", "cid-human", "I choose option A");
    source.author = "did:key:human".into();
    let kan = write_kan_stub(dir.path(), &[source]);

    let out = run(
        dir.path(),
        &kan,
        &[
            "intervention",
            "record",
            "work/topic",
            "--kind",
            "direction-correction",
            "--summary",
            "the direction changed",
            "--material-effect",
            "option B was dropped",
            "--source-claim",
            "cid-human",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let writes = appends(dir.path());
    assert_eq!(writes.len(), 1);
    let payload = payload(&writes[0], "day-intervention");
    assert_eq!(payload["recorded_by"], STUB_AUTHOR);
    assert_eq!(payload["basis"], serde_json::json!(["cid-human"]));
    assert_eq!(payload["source"]["provenance"], "authenticated-claim");
    assert_eq!(payload["source"]["principal"], "did:key:human");
    assert_eq!(payload["source"]["claim"], "cid-human");
    assert!(writes[0].contains("--cites cid-human"));
}

#[test]
fn malformed_or_ambiguous_requests_append_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("basis", "cid-basis", "repository evidence")],
    );

    let empty_effect = run(
        dir.path(),
        &kan,
        &[
            "acquired-input",
            "record",
            "work/topic",
            "--topic",
            "x",
            "--reported-provider",
            "human",
            "--fact",
            "x",
            "--material-effect",
            "",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(!empty_effect.status.success());

    let ambiguous = run(
        dir.path(),
        &kan,
        &[
            "intervention",
            "record",
            "work/topic",
            "--kind",
            "approval",
            "--summary",
            "x",
            "--material-effect",
            "y",
            "--reported-source",
            "human",
            "--source-claim",
            "cid-basis",
        ],
    );
    assert!(!ambiguous.status.success());

    let transcript = run(
        dir.path(),
        &kan,
        &[
            "acquired-input",
            "record",
            "work/topic",
            "--topic",
            "x",
            "--reported-provider",
            "human",
            "--fact",
            "x",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
            "--transcript",
            "raw conversation",
        ],
    );
    assert!(
        !transcript.status.success(),
        "the recording surface must not accept raw transcript content"
    );
    assert!(appends(dir.path()).is_empty());
}
