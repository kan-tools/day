#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn stub(dir: &std::path::Path, show: &str, status: &str) -> std::path::PathBuf {
    let path = dir.join("kan-config-stub");
    std::fs::write(
        &path,
        format!(
            r#"#!/bin/sh
case "$1 $2 $3" in
  "show --all --json") printf '%s\n' '{show}' ;;
  "status --json ") printf '%s\n' '{status}' ;;
  *) echo "unexpected or mutating kan invocation: $*" >&2; exit 97 ;;
esac
"#
        ),
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&path, permissions).unwrap();
    path
}

fn run(json: bool) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let kan = stub(
        dir.path(),
        r#"{"v":1,"trust":{"base":"Local","authors":[]},"excluded_by_trust":0,"subjects":[]}"#,
        r#"{"v":1,"subjects":[],"trust":{"base":"Local","authors":[]},"excluded_by_trust":0}"#,
    );
    let mut command = Command::new(env!("CARGO_BIN_EXE_day"));
    command
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", kan)
        .arg("config");
    if json {
        command.arg("--json");
    }
    command.output().unwrap()
}

fn run_fixture(show: &str) -> serde_json::Value {
    let dir = tempfile::tempdir().unwrap();
    let subjects: Vec<serde_json::Value> = serde_json::from_str::<serde_json::Value>(show).unwrap()
        ["subjects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| serde_json::json!({"subject": entry["subject"], "state": "Unclassified"}))
        .collect();
    let status = serde_json::json!({"v":1,"subjects":subjects,"trust":{"base":"Local","authors":[]},"excluded_by_trust":0}).to_string();
    let kan = stub(dir.path(), show, &status);
    let output = Command::new(env!("CARGO_BIN_EXE_day"))
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", kan)
        .args(["config", "--json"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn config_inventory_is_complete_and_read_only() {
    let output = run(false);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout);
    for subject in [
        "schema/witness",
        "schema/docs",
        "schema/cycle",
        "schema/injection",
        "schema/verdicts",
        "schema/blocks",
        "schema/design-doc",
    ] {
        assert!(text.contains(subject), "missing {subject}: {text}");
    }
    assert!(text.contains("(default)"), "{text}");
}

#[test]
fn config_json_has_a_versioned_shape() {
    let output = run(true);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["v"], 1);
    assert!(value["rows"]
        .as_array()
        .is_some_and(|rows| !rows.is_empty()));
}

#[test]
fn configuration_rows_carry_real_per_key_provenance_and_unsupported_status() {
    let show = r#"{"v":1,"trust":{"base":"Local","authors":[]},"excluded_by_trust":0,"subjects":[
      {"v":1,"subject":"schema/injection","claims":[{"cid":"bafylegacy","kind":"Observation","text":"```day-injection\n{\"cadence\":9}\n```"}],"excluded_by_trust":0},
      {"v":1,"subject":"schema/injection/max_practice_items","claims":[{"cid":"bafykey","kind":"Observation","text":"```day-injection\n{\"max_practice_items\":7}\n```"}],"excluded_by_trust":0},
      {"v":1,"subject":"schema/witness","claims":[{"cid":"bafybad","kind":"Observation","text":"```day-witness\n{\"halfstruct\":{\"command\":{\"argv\":\"true\"}}}\n```"}],"excluded_by_trust":0}
    ]}"#;
    let value = run_fixture(show);
    let rows = value["rows"].as_array().unwrap();
    let row = |subject: &str, key: &str| {
        rows.iter()
            .find(|r| r["subject"] == subject && r["key"] == key)
            .unwrap()
    };
    assert_eq!(
        row("schema/injection", "cadence")["provenance"],
        "bafylegacy"
    );
    assert_eq!(row("schema/injection", "cadence")["layer"], "legacy-block");
    assert_eq!(
        row("schema/injection", "max_practice_items")["provenance"],
        "bafykey"
    );
    assert_eq!(
        row("schema/injection", "max_practice_items")["layer"],
        "key"
    );
    assert_eq!(row("schema/witness", "halfstruct")["status"], "unsupported");
}

#[test]
fn declaration_inventory_routes_effective_values_through_production_loaders() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/config.rs"),
    )
    .unwrap();
    for loader in [
        "BlockSchemas::load(client)",
        "VerdictVocabulary::load(client)",
        "DocsSchema::load(client)",
        "Schema::load(client, DEFAULT_SLUG)",
        "crate::layers::config::<InjectionSchema>",
        "crate::layers::config::<CycleSchema>",
        "crate::layers::witness(client)",
    ] {
        assert!(
            source.contains(loader),
            "config bypasses production loader {loader}"
        );
    }
    assert_eq!(
        source.matches("atoms::newest_fenced::<T>").count(),
        1,
        "provenance reads must remain confined to the generic declaration adapter"
    );
}
