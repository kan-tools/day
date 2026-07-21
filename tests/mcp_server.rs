//! `.design/scaffold.md` AC-11: `day mcp` serves over stdio, advertises the
//! doctor and session_context tools, and a `doctor` tool call returns the
//! same result the CLI verb prints. Speaks raw line-delimited JSON-RPC to
//! the real binary, matching kan's own `tests/mcp_server.rs` in proving the
//! actual subprocess wiring rather than making library calls.

#![cfg(unix)]

mod common;

use std::process::Stdio;

use common::{atom_claim, write_kan_stub};
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn ac11_lists_tools_and_the_doctor_tool_matches_the_cli() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
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
        ],
    );

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
    for expected in ["doctor", "session_context", "design_check", "next"] {
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
}
