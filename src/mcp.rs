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
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
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

    #[tool(
        description = "Validate a design document against this project's live design-doc schema (declared in kan): required sections, requirement and acceptance-criterion counts, every requirement covered by a criterion, placeholder text, referenced file paths existing, and unresolved open questions. Reports findings; changes nothing."
    )]
    async fn design_check(
        &self,
        params: Parameters<DesignCheckParams>,
    ) -> Result<String, ErrorData> {
        let client = self.client();
        let schema = crate::schema::Schema::load(&client, crate::schema::DEFAULT_SLUG)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let path = self.cwd.join(&params.0.path);
        let doc = crate::record::read_document(&path)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        Ok(crate::design::check(&doc, &schema, &self.cwd).render())
    }

    #[tool(
        description = "Report what the project's atom graph says follows a given atom, and what each successor needs. Use this instead of assuming a fixed pipeline: composition is declared in kan and differs per project."
    )]
    async fn next(&self, params: Parameters<NextParams>) -> Result<String, ErrorData> {
        crate::record::next(&self.client(), &params.0.atom)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(
        description = "Check whether a declared bridge could reach its target telos: walks the planned arrangement of atoms, verifies each step's inputs are available where it sits, and reports whether the target's declared witnesses are produced. Realizability is assessed within a single frame only. Reports; changes nothing."
    )]
    async fn bridge_check(
        &self,
        params: Parameters<BridgeCheckParams>,
    ) -> Result<String, ErrorData> {
        crate::bridge::check(&self.client(), &params.0.bridge)
            .map(|report| report.render())
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(
        description = "Assess whether this project's docs still match what shipped: checks that declared version-carrying files hold the current version, reconciles the last release recorded in kan against the last git tag, and reports what changed since without any watched doc changing. Reports; changes nothing."
    )]
    async fn assess_docs(&self) -> Result<String, ErrorData> {
        let git = crate::git::Git::new(self.cwd.clone());
        crate::docs::assess(&self.client(), &git, &self.cwd, None)
            .map(|report| report.render())
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct BridgeCheckParams {
    /// The bridge slug, e.g. `v0.3`.
    pub bridge: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DesignCheckParams {
    /// Path to the design document, relative to the repo root.
    pub path: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NextParams {
    /// The atom slug, e.g. `design`.
    pub atom: String,
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
             and nothing is ever deleted. A bridge, on a bridge/<slug> subject, is a \
             planned arrangement of atoms aimed at a target telos — how you get from \
             here to there. Its plan composes atoms in sequence (b may use what a \
             produced), concurrently (neither may rely on the other), or as \
             alternatives (either route suffices, so only what every branch produces \
             can be relied on downstream). A telos may declare witnesses: artifact \
             types that would evidence it, which is what makes \"does this plan reach \
             that telos\" checkable without collapsing the telos to a type. The tools \
             read that state: doctor verifies kan is reachable and that the live atom \
             vocabulary composes, next reports what follows an atom, design_check \
             validates a design document, bridge_check computes whether a plan could \
             reach its target, and session_context returns the teloi, atoms, open \
             subjects, and drift warnings in play. All are advisory — day reports \
             drift, it never blocks an action, and it does not track whether planned \
             steps have happened. Realizability as reported is frame-internal only. \
             Assess work against material evidence (builds, tests, diffs) rather than \
             against your own account of it, and record what you find back into kan.",
        )
    }
}
