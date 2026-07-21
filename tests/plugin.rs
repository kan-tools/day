//! `.design/scaffold.md` AC-4, AC-5, AC-7, AC-8, AC-9 — the shipped plugin
//! is well-formed, its hooks cannot block, and the skills and conventions it
//! promises are actually present.

mod common;

use common::repo_root;
use serde_json::Value;

fn read_json(rel: &str) -> Value {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should exist and be readable: {e}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{} should be valid JSON: {e}", path.display()))
}

#[test]
fn ac4_plugin_manifest_is_valid_json_with_name_and_description() {
    let manifest = read_json(".claude-plugin/plugin.json");
    assert_eq!(manifest["name"].as_str(), Some("day"));
    assert!(
        !manifest["description"].as_str().unwrap_or("").is_empty(),
        "plugin.json needs a non-empty description"
    );
}

#[test]
fn ac4_mcp_registration_declares_the_day_stdio_server() {
    let mcp = read_json(".mcp.json");
    let day = &mcp["mcpServers"]["day"];
    assert_eq!(day["command"].as_str(), Some("day"));
    assert_eq!(day["args"][0].as_str(), Some("mcp"));
}

/// REQ-4's guardrail, not a one-time check: day's hooks are advisory by
/// construction, and a blocking hook must never be able to land here
/// unnoticed. These are the values a Claude Code hook uses to deny, block,
/// or halt — none may appear in day's shipped hook config.
#[test]
fn ac5_shipped_hooks_declare_no_blocking_decisions() {
    let path = repo_root().join("hooks/hooks.json");
    let raw = std::fs::read_to_string(&path).expect("hooks.json should exist");
    let parsed: Value = serde_json::from_str(&raw).expect("hooks.json should be valid JSON");

    // Parse first so a malformed file fails loudly rather than passing the
    // substring check by accident.
    assert!(parsed["hooks"]["SessionStart"].is_array());

    let hook_commands = raw.to_lowercase();
    for forbidden in [
        "\"decision\": \"block\"",
        "\"decision\":\"block\"",
        "\"permissiondecision\": \"deny\"",
        "\"permissiondecision\":\"deny\"",
        "\"continue\": false",
        "\"continue\":false",
        "pretooluse",
    ] {
        assert!(
            !hook_commands.contains(forbidden),
            "day's hooks must stay advisory; found blocking construct {forbidden:?} in hooks.json"
        );
    }
}

#[test]
fn ac5_the_session_start_hook_invokes_day_and_nothing_else() {
    let hooks = read_json("hooks/hooks.json");
    let command = hooks["hooks"]["SessionStart"][0]["hooks"][0]["command"]
        .as_str()
        .expect("a SessionStart command should be declared");
    assert_eq!(command, "day hook session-start");
}

#[test]
fn ac7_and_ac8_the_plugin_ships_both_atoms_as_commands() {
    for (file, must_contain) in [
        ("commands/design.md", "design document"),
        ("commands/adversarial-review.md", "APPROVE WITH FOLLOW-UPS"),
    ] {
        let text = std::fs::read_to_string(repo_root().join(file))
            .unwrap_or_else(|e| panic!("{file} should ship with the plugin: {e}"));
        assert!(text.starts_with("---"), "{file} needs command frontmatter");
        assert!(
            text.contains(must_contain),
            "{file} should contain {must_contain:?}"
        );
        assert!(
            text.contains("```day-atom"),
            "{file} should declare its atom interface"
        );
    }
}

#[test]
fn ac8_the_review_atom_declares_all_four_verdicts() {
    let text = std::fs::read_to_string(repo_root().join("commands/adversarial-review.md")).unwrap();
    for verdict in ["APPROVE", "APPROVE WITH FOLLOW-UPS", "REDIRECT", "BLOCK"] {
        assert!(text.contains(verdict), "missing verdict {verdict:?}");
    }
    assert!(
        text.contains("kan decide"),
        "the verdict should be recorded into kan"
    );
}

#[test]
fn ac9_conventions_document_the_prefixes_the_code_actually_reads() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md"))
        .expect("docs/CONVENTIONS.md should exist");
    // The doc and the code must agree, so read the constants rather than
    // hardcoding the strings twice.
    for token in [
        day::atoms::ATOM_PREFIX,
        day::atoms::TELOS_PREFIX,
        day::atoms::FENCE_INFO,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }
}
