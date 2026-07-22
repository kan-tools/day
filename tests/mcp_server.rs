//! `.design/scaffold.md` AC-11: `day mcp` serves over stdio, advertises the
//! doctor and session_context tools, and a `doctor` tool call returns the
//! same result the CLI verb prints. Speaks raw line-delimited JSON-RPC to
//! the real binary, matching kan's own `tests/mcp_server.rs` in proving the
//! actual subprocess wiring rather than making library calls.

#![cfg(unix)]

mod common;

use std::process::Stdio;

use common::{atom_claim, schema_claim, write_kan_stub};
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn ac11_lists_tools_and_the_doctor_tool_matches_the_cli() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("mcp-probe-ran");
    let kan = write_kan_stub(
        dir.path(),
        &[
            // design_check needs a declared schema; day deliberately
            // refuses to validate against one nobody chose.
            schema_claim("design-doc", "bafyreischema"),
            common::claim(
                "schema/docs",
                "bafyreidocs",
                "Docs schema.\n\n```day-docs\n{\"version_source\": \"Cargo.toml\", \
                 \"version_files\": [\"README.md\"]}\n```\n",
            ),
            atom_claim(
                "design",
                "bafyreidesign",
                &["idea"],
                &["design-doc"],
                &["build"],
            ),
            atom_claim(
                "build",
                "bafyreibuild",
                &["design-doc"],
                &["code-change"],
                &[],
            ),
            // assess_telos needs a witnessed telos and a probe map. The
            // probe is a command that would leave a sentinel file, so the
            // "MCP never executes" guarantee is proved by the filesystem
            // rather than by day's own output.
            common::claim(
                "telos/shipped",
                "bafyreitelos",
                "Shipped.\n\n```day-telos\n{\"witnesses\":[\"passing-tests\"]}\n```\n",
            ),
            common::claim(
                "schema/witness",
                "bafyreiwitness",
                &format!(
                    "Witness probes.\n\n```day-witness\n{{\"passing-tests\":\
                     {{\"command\":\"touch {}\"}}}}\n```\n",
                    sentinel.display()
                ),
            ),
        ],
    );

    // assess_docs needs a docs schema, a version source, and git.
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("README.md"), "1.0.0\n").unwrap();
    let git = write_git_stub(dir.path());

    let cli = std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .arg("doctor")
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .output()
        .expect("failed to run day doctor");
    let cli_stdout = String::from_utf8_lossy(&cli.stdout).into_owned();

    let mut child = Command::new(env!("CARGO_BIN_EXE_day"))
        .arg("mcp")
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .env("DAY_GIT_BIN", &git)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("failed to spawn day mcp");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());

    let send = |v: Value| serde_json::to_string(&v).unwrap() + "\n";
    let mut recv_line = String::new();
    macro_rules! recv {
        () => {{
            recv_line.clear();
            tokio::time::timeout(
                std::time::Duration::from_secs(5),
                stdout.read_line(&mut recv_line),
            )
            .await
            .expect("timed out waiting for day mcp response")
            .expect("failed to read from day mcp stdout");
            serde_json::from_str::<Value>(&recv_line).expect("response was not valid JSON")
        }};
    }

    stdin
        .write_all(
            send(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-06-18",
                    "capabilities": {},
                    "clientInfo": {"name": "ac11-test", "version": "0.0.1"}
                }
            }))
            .as_bytes(),
        )
        .await
        .unwrap();
    let init = recv!();
    assert_eq!(init["id"], 1);
    assert!(init["result"]["capabilities"]["tools"].is_object());

    let instructions = init["result"]["instructions"]
        .as_str()
        .expect("day should advertise instructions");
    assert!(
        instructions.contains("telos") && instructions.contains("atom"),
        "instructions should describe day's model: {instructions:?}"
    );

    stdin
        .write_all(
            send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"})).as_bytes(),
        )
        .await
        .unwrap();

    stdin
        .write_all(send(json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})).as_bytes())
        .await
        .unwrap();
    let list = recv!();
    let names: Vec<&str> = list["result"]["tools"]
        .as_array()
        .expect("tools/list should return an array")
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    for expected in [
        "doctor",
        "session_context",
        "design_check",
        "next",
        "bridge_check",
        "assess_docs",
        "assess_telos",
    ] {
        assert!(
            names.contains(&expected),
            "missing tool {expected:?} in {names:?}"
        );
    }
    // The interview itself is deliberately not a tool: a multi-turn
    // interview is not a function call (`.design/design-atom-backing.md`
    // REQ-8). MCP exposes the checks, not the workflow.
    for absent in ["design_record", "review_record"] {
        assert!(
            !names.contains(&absent),
            "{absent:?} should not be an MCP tool, found in {names:?}"
        );
    }

    stdin
        .write_all(
            send(json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/call",
                "params": {"name": "doctor", "arguments": {}}
            }))
            .as_bytes(),
        )
        .await
        .unwrap();
    let call = recv!();
    let text = call["result"]["content"][0]["text"]
        .as_str()
        .expect("doctor should return text content");

    assert_eq!(
        text.trim(),
        cli_stdout.trim(),
        "the MCP tool and the CLI verb must return the same report"
    );
    assert!(text.contains("composition: ok"), "got: {text}");

    // The other half of AC-10, which the adversarial review found was
    // claimed but never asserted: design_check must agree with its CLI verb
    // too, not merely appear in tools/list.
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/thing.rs"), "// fixture\n").unwrap();
    let doc = "# Feature: t\n\n## Summary\nS.\n\n## Requirements\n- REQ-1: a\n- REQ-2: b\n\n\
        ## Acceptance Criteria\n- [ ] AC-1: x (REQ-1)\n- [ ] AC-2: y (REQ-2)\n\n\
        ## Architecture\nTouches `src/thing.rs`.\n";
    std::fs::write(dir.path().join("doc.md"), doc).unwrap();

    let cli = std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["design", "check", "doc.md"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .output()
        .expect("failed to run day design check");
    let cli_check = String::from_utf8_lossy(&cli.stdout).into_owned();

    stdin
        .write_all(
            send(json!({
                "jsonrpc": "2.0",
                "id": 4,
                "method": "tools/call",
                "params": {"name": "design_check", "arguments": {"path": "doc.md"}}
            }))
            .as_bytes(),
        )
        .await
        .unwrap();
    let call = recv!();
    let text = call["result"]["content"][0]["text"]
        .as_str()
        .expect("design_check should return text content");
    assert_eq!(
        text.trim(),
        cli_check.trim(),
        "the MCP design_check tool and the CLI verb must return the same report"
    );

    // Every MCP tool is asserted equivalent to its CLI verb, not merely
    // present. The previous review found design_check had been added
    // without this; assess_docs was then added the same way. Covering all
    // of them here is what stops the pattern recurring a third time.
    let cli = std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["assess", "docs"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .env("DAY_GIT_BIN", &git)
        .output()
        .expect("failed to run day assess docs");
    let cli_assess = String::from_utf8_lossy(&cli.stdout).into_owned();

    stdin
        .write_all(
            send(json!({
                "jsonrpc": "2.0",
                "id": 5,
                "method": "tools/call",
                "params": {"name": "assess_docs", "arguments": {}}
            }))
            .as_bytes(),
        )
        .await
        .unwrap();
    let call = recv!();
    let text = call["result"]["content"][0]["text"]
        .as_str()
        .expect("assess_docs should return text content");
    assert_eq!(
        text.trim(),
        cli_assess.trim(),
        "the MCP assess_docs tool and the CLI verb must return the same report"
    );

    // `.design/assess-telos.md` AC-8. Equivalent to the CLI verb run WITHOUT
    // --run, and — the part that matters — incapable of executing a command
    // probe. An agent calling a read-shaped tool must not be able to run a
    // program this repo's log happens to name.
    let cli = std::process::Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["assess", "telos", "shipped"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .env("DAY_GIT_BIN", &git)
        .output()
        .expect("failed to run day assess telos");
    let cli_telos = String::from_utf8_lossy(&cli.stdout).into_owned();
    assert!(
        !sentinel.exists(),
        "the CLI without --run should not have executed the probe either"
    );

    stdin
        .write_all(
            send(json!({
                "jsonrpc": "2.0",
                "id": 6,
                "method": "tools/call",
                "params": {"name": "assess_telos", "arguments": {"telos": "shipped"}}
            }))
            .as_bytes(),
        )
        .await
        .unwrap();
    let call = recv!();
    let text = call["result"]["content"][0]["text"]
        .as_str()
        .expect("assess_telos should return text content");
    assert_eq!(
        text.trim(),
        cli_telos.trim(),
        "the MCP assess_telos tool and the CLI verb must return the same report"
    );
    assert!(
        text.contains("[NOT RUN]"),
        "a command probe should be reported, not executed, over MCP: {text}"
    );
    assert!(
        !sentinel.exists(),
        "the MCP assess_telos tool executed a command probe; it must never be able to"
    );
}

/// A stub `git` for the MCP test: no tags, no changes.
fn write_git_stub(dir: &std::path::Path) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(&script, "#!/bin/sh\nprintf ''\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}
