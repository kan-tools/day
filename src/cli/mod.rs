//! day's CLI surface. Deliberately four verbs: the walking skeleton for the
//! process layer, not its final vocabulary (`.design/` in this repo tracks
//! what comes next). Verbs are grouped by what they're for — setting day up
//! (`init`), inspecting process state (`doctor`), being called by a harness
//! (`hook`), and serving the same reads to agents over MCP (`mcp`).

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::{doctor, hooks, kan_client::KanClient, mcp};

/// Exit code for "day ran fine, but the process state it inspected has
/// findings" — distinct from a hard failure so scripts can tell the two
/// apart.
const EXIT_FINDINGS: u8 = 1;
/// Exit code for "day could not run the check at all" (kan missing, etc).
const EXIT_UNAVAILABLE: u8 = 2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error(transparent)]
    Doctor(#[from] doctor::Error),
    #[error(transparent)]
    UnknownEvent(#[from] hooks::UnknownEvent),
    #[error(transparent)]
    Mcp(#[from] mcp::Error),
}

#[derive(Debug, Parser)]
#[command(
    name = "day",
    version,
    about = "Structured process for AI-assisted development."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print the steps to wire day into this repo, and check kan is reachable
    Init,
    /// Check kan reachability and verify the live atom vocabulary composes
    Doctor,
    /// Entry point harness hooks call; prints advisory context, never blocks
    Hook {
        /// The harness event (currently: session-start)
        event: String,
    },
    /// MCP server over stdio
    Mcp,
}

pub async fn run(cli: Cli) -> Result<ExitCode, Error> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let client = KanClient::new(cwd.clone());

    match cli.command {
        Command::Init => {
            client.probe()?;
            print!("{}", init_instructions());
            Ok(ExitCode::SUCCESS)
        }
        Command::Doctor => {
            let report = doctor::run(&client)?;
            print!("{}", report.render());
            Ok(if report.is_healthy() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(EXIT_FINDINGS)
            })
        }
        // Always exit 0: a hook that can fail a session is a blocking hook
        // by another name. Errors are printed as context, not raised.
        Command::Hook { event } => {
            match hooks::dispatch(&event, &client) {
                Ok(text) => print!("{text}"),
                Err(e) => println!("## day\n\n{e}"),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Mcp => {
            mcp::serve(cwd).await?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Prints, never mutates — the same contract `kan mcp install` set. day
/// touches a user's Claude Code config only by telling them what to add.
pub fn init_instructions() -> String {
    let exe = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "day".to_string());

    let mut out = String::new();
    out.push_str("kan: reachable\n\n");
    out.push_str("Wire day into this repo — either path works.\n\n");
    out.push_str("1. As a Claude Code plugin (recommended; brings the skills and the\n");
    out.push_str("   session-start hook with it):\n");
    out.push_str("     /plugin install <path to the day repo, or its marketplace entry>\n\n");
    out.push_str("2. Piecemeal, without the plugin:\n");
    out.push_str(&format!("     claude mcp add day -- {exe} mcp\n"));
    out.push_str("   plus a SessionStart hook in .claude/settings.json:\n");
    out.push_str(&format!(
        "     {{\"hooks\": {{\"SessionStart\": [{{\"hooks\": [{{\"type\": \"command\", \"command\": \"{exe} hook session-start\"}}]}}]}}}}\n\n"
    ));
    out.push_str("day stores nothing of its own: teloi, atoms, and assessments all live in\n");
    out.push_str("kan as claims (docs/CONVENTIONS.md). Nothing above is written for you.\n");
    out
}

/// Exit code used when day cannot reach kan at all. Exposed so `main` can
/// map the error case without duplicating the constant.
pub fn unavailable() -> ExitCode {
    ExitCode::from(EXIT_UNAVAILABLE)
}
