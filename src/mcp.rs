//! `day mcp`: the same reads the CLI and the hooks expose, offered to agents
//! that have no shell. Every tool dispatches to the exact same functions the
//! CLI calls (`doctor::run`, `hooks::session_start`), so the surfaces cannot
//! drift apart — this module is presentation only.
//!
//! Unlike kan's MCP instructions, which are deliberately non-prescriptive,
//! day's *are* prescriptive: opinions about how to work are this tool's
//! entire reason to exist (ADR-18's boundary rule is what makes that
//! difference principled rather than inconsistent).

use std::path::PathBuf;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt,
};

use crate::{doctor, hooks, kan_client::KanClient};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Boxed for the same reason kan boxes it: `ServerInitializeError` is far
    // larger than `Join`'s payload and skews the enum otherwise.
    #[error("mcp server failed to initialize: {0}")]
    Initialize(#[from] Box<rmcp::service::ServerInitializeError>),
    #[error("mcp server task panicked: {0}")]
    Join(#[from] tokio::task::JoinError),
}

pub async fn serve(cwd: PathBuf) -> Result<(), Error> {
    let server = DayServer::new(cwd)
        .serve(rmcp::transport::stdio())
        .await
        .map_err(Box::new)?;
    server.waiting().await?;
    Ok(())
}

#[derive(Clone)]
pub struct DayServer {
    cwd: PathBuf,
    tool_router: ToolRouter<Self>,
}

impl DayServer {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            tool_router: Self::tool_router(),
        }
    }

    fn client(&self) -> KanClient {
        KanClient::new(self.cwd.clone())
    }
}

#[tool_router]
impl DayServer {
    #[tool(
        description = "Check that kan is reachable and that the project's live atom vocabulary still composes: every declared successor exists, and each upstream atom's outputs cover its downstream atom's inputs. Reports findings; changes nothing."
    )]
    async fn doctor(&self) -> Result<String, ErrorData> {
        let report = doctor::run(&self.client())
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        Ok(report.render())
    }

    #[tool(
        description = "The advisory process context for this repo: the teloi currently in play, the declared process atoms, and any drift warnings. The same text day's session-start hook injects."
    )]
    async fn session_context(&self) -> Result<String, ErrorData> {
        Ok(hooks::session_start(&self.client()))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for DayServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "day is the process layer for this repo; kan is the memory layer underneath it. \
             day stores nothing itself — every durable thing it reports is a kan claim. \
             A telos is a desired state of the world held only up to weak equivalence: \
             several teloi usually apply to one project at once and are in tension with \
             each other, and that tension is information, not a bug. Teloi live on \
             telos/<slug> subjects in kan. An atom is a composable unit of process work \
             (design, build, adversarial review, user testing, drift evaluation, and so \
             on) declared on an atom/<slug> subject, carrying a fenced day-atom JSON \
             block naming its inputs, outputs, and the atoms it composes into; the \
             vocabulary is per-atom additive, so a newer claim supersedes an older one \
             and nothing is ever deleted. The tools read that state: doctor verifies \
             kan is reachable and that the live atom vocabulary composes, and \
             session_context returns the teloi, atoms, and drift warnings in play. \
             Both are advisory — day reports drift, it never blocks an action. Assess \
             work against material evidence (builds, tests, diffs) rather than against \
             your own account of it, and record what you find back into kan.",
        )
    }
}
