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
    // substring check by accident. Every registered event is covered, so a
    // future hook cannot be added outside this guardrail.
    let events = parsed["hooks"]
        .as_object()
        .expect("hooks should be an object");
    assert!(!events.is_empty());
    for (event, entries) in events {
        assert!(entries.is_array(), "{event} should hold an array");
    }

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

/// Guards the adversarial review's blocking finding. Only
/// `UserPromptSubmit`, `UserPromptExpansion`, and `SessionStart` add hook
/// stdout to the model's context; every other event writes to the debug log.
/// Registering a hook whose whole purpose is to say something to the agent
/// on any other event ships a feature that silently reaches nobody.
#[test]
fn hooks_are_only_registered_on_events_that_deliver_stdout_to_the_model() {
    let hooks = read_json("hooks/hooks.json");
    let registered = hooks["hooks"].as_object().expect("hooks object");
    for event in registered.keys() {
        assert!(
            ["UserPromptSubmit", "UserPromptExpansion", "SessionStart"].contains(&event.as_str()),
            "{event} does not add hook stdout to the model's context, so a prompt \
             registered there would reach nobody. If this hook is for a side effect \
             rather than for saying something, widen this test deliberately."
        );
    }
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
    // The verdict is still recorded into kan, but through `day review
    // record`, which enforces the closed verdict set and the citation
    // rather than trusting the prompt to.
    assert!(
        text.contains("day review record"),
        "the verdict should be recorded through day's verb"
    );
    assert!(
        text.contains("--cites"),
        "a verdict must cite the claim it audits"
    );
}

/// `.design/design-atom-backing.md` AC-9. Composition is data: the atom
/// graph in kan says what follows what, so a project can insert a step by
/// changing a claim rather than editing day's prompts. A command naming
/// another command would quietly hard-code a pipeline and undo that.
#[test]
fn ac9_neither_command_hardcodes_an_invocation_of_the_other() {
    let cases = [
        ("commands/design.md", "/adversarial-review"),
        ("commands/adversarial-review.md", "/design"),
    ];
    for (file, forbidden) in cases {
        let text = std::fs::read_to_string(repo_root().join(file)).unwrap();
        for line in text.lines() {
            // A prose mention is fine; an instruction to run it is not.
            let invokes = line.contains(forbidden)
                && (line.contains("Run ")
                    || line.contains("run ")
                    || line.trim_start().starts_with('$'));
            assert!(
                !invokes,
                "{file} should reach the next step via `day next`, not by naming \
                 {forbidden}: {line}"
            );
        }
        assert!(
            text.contains("day next"),
            "{file} should end by asking the atom graph what comes next"
        );
    }
}

/// `.design/vocabulary-verbs.md` AC-11. The conventions are the contract,
/// not day's verbs: a hand-written claim following this page must stay as
/// valid as one day wrote, or day has quietly become required.
#[test]
fn ac11_conventions_state_that_hand_written_claims_remain_valid() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    assert!(
        text.contains("hand-written claim"),
        "CONVENTIONS.md should say hand-written claims remain valid"
    );
    assert!(
        text.contains("no `revise` verb") || text.contains("no revise verb"),
        "CONVENTIONS.md should explain that revision is just a later claim"
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
        day::schema::SCHEMA_PREFIX,
        day::schema::FENCE_INFO,
        day::bridge::BRIDGE_PREFIX,
        day::bridge::FENCE_INFO,
        day::bridge::TELOS_FENCE,
        day::docs::FENCE_INFO,
        day::docs::DOCS_SLUG,
        day::telos::FENCE_INFO,
        day::telos::WITNESS_SLUG,
        day::tension::TENSION_PREFIX,
        day::tension::FENCE_INFO,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }
}

/// `.design/assess-telos.md` AC-15's second half. Command probes make the
/// shell day's third substrate; `CLAUDE.md` is where a future session learns
/// that before adding a fourth spawn site, so it has to say so.
#[test]
fn ac15_claude_md_records_the_command_substrate_and_its_guardrails() {
    let text =
        std::fs::read_to_string(repo_root().join("CLAUDE.md")).expect("CLAUDE.md should exist");
    assert!(
        text.contains("three substrates"),
        "CLAUDE.md should record that day now has three substrates"
    );
    assert!(
        text.contains("src/probe.rs"),
        "CLAUDE.md should name where command execution is confined"
    );
    for guardrail in ["no shell", "--run", "MCP", "timeout"] {
        assert!(
            text.contains(guardrail),
            "CLAUDE.md should record the {guardrail:?} guardrail on command probes"
        );
    }
}

/// `.design/telos-subject-shape.md` AC-11's second half. The page must not
/// still instruct recording a tension's reason on a telos subject — a
/// conventions doc describing the shape day no longer implements is worse
/// than one that says nothing, because it is followed.
#[test]
fn ac11_conventions_no_longer_put_a_tension_reason_on_a_telos_subject() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    assert!(
        text.contains("carries its declaration and its edges"),
        "CONVENTIONS should state what a telos subject carries"
    );
    assert!(
        text.contains("scope"),
        "CONVENTIONS should document witness scoping"
    );
    // The old instruction, which day#32 was filed against.
    assert!(
        !text.contains("day telos tension interface-legibility feature-depth \\\n  \"Every verb")
            || text.contains("tension/<a>--<b>"),
        "the tension example should sit under the tension subject convention"
    );
}
