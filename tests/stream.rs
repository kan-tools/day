//! day#204 — the stream inventory is one bulk-read fold shared by the CLI,
//! MCP, and both handoff skills. These tests drive the real CLI against kan's
//! JSON boundary; renderer-only tests cannot prove the diagnostics arrived.

#![cfg(unix)]

mod common;

use common::{claim, write_kan_stub, write_stub_withheld};

fn run(dir: &std::path::Path, kan: &std::path::Path) -> String {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["stream", "list"])
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("day stream list should run");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn cli_lists_only_visible_live_handoffs_with_bounded_honest_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let mut old = claim("agents/handoff/old", "cid-old", "older handoff");
    old.recorded_at = Some(10);
    let mut main = claim(
        "agents/handoff/main",
        "cid-main",
        &format!("latest {}", "detail ".repeat(40)),
    );
    main.recorded_at = Some(20);
    let unrelated = claim("issue/elsewhere", "cid-other", "not a stream");
    let nested = claim("agents/handoff/main/nested", "cid-nested", "not a thread");
    let kan = write_kan_stub(dir.path(), &[old, main, unrelated, nested]);

    let text = run(dir.path(), &kan);
    assert!(text.contains("2 visible live handoff stream(s)"), "{text}");
    assert!(text.contains("inventory: complete"), "{text}");
    assert!(
        text.find("main").unwrap() < text.find("old").unwrap(),
        "{text}"
    );
    assert!(text.contains("last recorded_at 20 µs"), "{text}");
    assert!(!text.contains("issue/elsewhere"), "{text}");
    assert!(!text.contains("main/nested"), "{text}");
    assert!(!text.contains("stale"), "{text}");
}

#[test]
fn narrowing_and_unaccounted_subjects_make_the_cli_inventory_incomplete() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("agents/handoff/main", "cid-main", "handoff")],
    );
    write_stub_withheld(dir.path(), 2);

    let status = serde_json::json!({
        "v": 1,
        "excluded_by_trust": 2,
        "subjects": [
            {"subject":"agents/handoff/main","excluded_by_trust":0},
            {"subject":"agents/handoff/dropped","excluded_by_trust":0}
        ]
    });
    std::fs::write(
        dir.path().join("kan-stub-data/status.json"),
        serde_json::to_vec(&status).unwrap(),
    )
    .unwrap();

    let text = run(dir.path(), &kan);
    assert!(text.contains("inventory: INCOMPLETE"), "{text}");
    assert!(text.contains("2 claim(s) withheld"), "{text}");
    assert!(text.contains("agents/handoff/dropped"), "{text}");
    assert!(text.contains("1 visible live handoff stream(s)"), "{text}");
}

#[test]
fn omitted_published_read_fields_are_unknown_not_clean_zero() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("agents/handoff/main", "cid-main", "handoff")],
    );
    std::fs::write(
        dir.path().join("kan-stub-data/show_all.py"),
        r#"import json, pathlib, sys
data = pathlib.Path(sys.argv[1])
entries = [json.loads(p.read_text()) for p in sorted(data.glob("show-*.json"))]
print(json.dumps({"v":1,"excluded_by_trust":0,"subjects":entries}))
"#,
    )
    .unwrap();

    let text = run(dir.path(), &kan);
    assert!(text.contains("inventory: INCOMPLETE"), "{text}");
    assert!(
        text.contains("omitted `published_read_error_count`"),
        "{text}"
    );
    assert!(text.contains("omitted `published_read_errors`"), "{text}");
}

#[test]
fn published_read_failures_are_delivered_not_rounded_to_complete() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim("agents/handoff/main", "cid-main", "handoff")],
    );
    std::fs::write(
        dir.path().join("kan-stub-data/show_all.py"),
        r#"import json, pathlib, sys
data = pathlib.Path(sys.argv[1])
entries = [json.loads(p.read_text()) for p in sorted(data.glob("show-*.json"))]
print(json.dumps({"v":1,"excluded_by_trust":0,
                  "published_read_error_count":1,
                  "published_read_errors":[{"path":".claims/bad.md","reason":"invalid"}],
                  "subjects":entries}))
"#,
    )
    .unwrap();

    let text = run(dir.path(), &kan);
    assert!(text.contains("inventory: INCOMPLETE"), "{text}");
    assert!(text.contains("1 published-claim read error(s)"), "{text}");
    assert!(text.contains(".claims/bad.md"), "{text}");
    assert!(text.contains("invalid"), "{text}");
}
