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
fn recorder_provider_is_authenticated_by_the_envelope() {
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
            "--recorder-provider",
            "--fact",
            "the recorder supplied this fact",
            "--material-effect",
            "the plan changed",
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
    assert_eq!(payload["recorded_by"], STUB_AUTHOR);
    assert_eq!(payload["provider"]["provenance"], "recorder");
    assert_eq!(payload["provider"]["principal"], STUB_AUTHOR);
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
fn recorder_source_is_authenticated_by_the_envelope_for_every_fixed_kind() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("basis", "cid-basis", "repository evidence")],
    );
    let kinds = [
        "direction-correction",
        "missing-context",
        "answered-question",
        "stopped-work",
        "approval",
    ];

    for kind in kinds {
        let out = run(
            dir.path(),
            &kan,
            &[
                "intervention",
                "record",
                "work/topic",
                "--kind",
                kind,
                "--summary",
                "the recorder classified its own intervention",
                "--material-effect",
                "the work changed",
                "--recorder-source",
                "--cites",
                "cid-basis",
            ],
        );
        assert!(
            out.status.success(),
            "{kind}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    let writes = appends(dir.path());
    assert_eq!(writes.len(), kinds.len());
    for (write, kind) in writes.iter().zip(kinds) {
        let payload = payload(write, "day-intervention");
        assert_eq!(payload["kind"], kind);
        assert_eq!(payload["recorded_by"], STUB_AUTHOR);
        assert_eq!(payload["source"]["provenance"], "recorder");
        assert_eq!(payload["source"]["principal"], STUB_AUTHOR);
    }
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
            "Human: choose A Agent: acknowledged Human: actually choose B",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(
        !transcript.status.success(),
        "the recording surface must reject transcript-shaped content inside an accepted summary field"
    );
    assert!(
        String::from_utf8_lossy(&transcript.stderr).contains("raw conversation transcript"),
        "the refusal must name the invariant rather than looking like a parse failure"
    );

    let named_speakers = run(
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
            "Alice: choose A. Bob: choose B.",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(
        !named_speakers.status.success(),
        "speaker names must not bypass transcript rejection"
    );

    for hostile in [
        "Alice:choose A Bob:choose B",
        "[Alice] choose A [Bob] choose B",
        "Alice — choose A; Bob — choose B",
        "Alice—choose A; Bob—choose B",
        "Alice–choose A; Bob–choose B",
        "**Alice**\nchoose A\n**Bob**\nchoose B",
        "# Alice\nchoose A\n# Bob\nchoose B",
        "_Alice_\nchoose A\n_Bob_\nchoose B",
        "`Alice`\nchoose A\n`Bob`\nchoose B",
    ] {
        let hostile_transcript = run(
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
                hostile,
                "--material-effect",
                "y",
                "--cites",
                "cid-basis",
            ],
        );
        assert!(
            !hostile_transcript.status.success(),
            "punctuation must not make transcript `{hostile}` durable"
        );
    }

    let transcript_source = run(
        dir.path(),
        &kan,
        &[
            "acquired-input",
            "record",
            "work/topic",
            "--topic",
            "x",
            "--reported-provider",
            "Alice: choose A. Bob: choose B.",
            "--fact",
            "A bounded summary.",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(
        !transcript_source.status.success(),
        "reported provenance is narrative too and must reject transcript-shaped content"
    );

    for command in ["acquired-input", "intervention"] {
        let mut args = vec![command, "record", "Alice: choose A. Bob: choose B."];
        if command == "acquired-input" {
            args.extend(["--topic", "x", "--fact", "summary"]);
        } else {
            args.extend(["--kind", "approval", "--summary", "summary"]);
        }
        args.extend([
            if command == "acquired-input" {
                "--reported-provider"
            } else {
                "--reported-source"
            },
            "human",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ]);
        let transcript_subject = run(dir.path(), &kan, &args);
        assert!(
            !transcript_subject.status.success(),
            "{command} positional work subject must pass the serialized block invariant"
        );
        assert!(
            String::from_utf8_lossy(&transcript_subject.stderr)
                .contains("raw conversation transcript"),
            "{command} must fail for the invariant, not argument parsing"
        );
    }

    let bracketed_source = run(
        dir.path(),
        &kan,
        &[
            "acquired-input",
            "record",
            "work/topic",
            "--topic",
            "x",
            "--reported-provider",
            "[Alice] choose A [Bob] choose B",
            "--fact",
            "summary",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(!bracketed_source.status.success());

    let dashed_subject = run(
        dir.path(),
        &kan,
        &[
            "intervention",
            "record",
            "Alice — choose A; Bob — choose B",
            "--kind",
            "approval",
            "--summary",
            "summary",
            "--reported-source",
            "human",
            "--material-effect",
            "y",
            "--cites",
            "cid-basis",
        ],
    );
    assert!(!dashed_subject.status.success());
    assert!(
        appends(dir.path()).is_empty(),
        "every transcript and malformed request above must refuse before append"
    );

    for bounded_summary in [
        "Decision: choose A. Effect: narrows the release.",
        "Risk: stale evidence. Mitigation: re-run the gate.",
        "RFC 1 source: accepted. release scope: six issues.",
        "Q1: choose A. Q2: choose B.",
        "AC1: rejects transcripts. AC2: accepts summaries.",
    ] {
        let accepted = run(
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
                bounded_summary,
                "--material-effect",
                "y",
                "--cites",
                "cid-basis",
            ],
        );
        assert!(
            accepted.status.success(),
            "ordinary labeled summary was mistaken for a transcript: {bounded_summary}: {}",
            String::from_utf8_lossy(&accepted.stderr)
        );
    }
    assert_eq!(appends(dir.path()).len(), 5);
}
