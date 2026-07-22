//! day's CLI surface, grouped by what each verb is for: setting day up
//! (`init`), declaring the project's vocabulary (`telos`, `atom`),
//! inspecting process state (`doctor`, `next`), working with design
//! documents and reviews (`design`, `review`), being called by a harness
//! (`hook`), and serving the same reads to agents over MCP (`mcp`).
//!
//! Two absences are deliberate. There is **no `revise` verb** — kan is
//! append-only, so a revision is just a later claim and `declare` cites the
//! prior one automatically. There are **no read verbs** — kan's own
//! `show`/`status` plus `doctor` and `session_context` already cover
//! reading, and duplicating kan's surface costs more than it returns.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::{doctor, hooks, kan_client::KanClient, mcp};

/// kan's `RelationKind` for two subjects that pull against each other.
/// Shipped in kan#60; before that, tension between teloi could only be
/// prose, which is why day#18 existed.
const TENSION_RELATION: &str = "in-tension-with";

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
    #[error(transparent)]
    Schema(#[from] crate::schema::Error),
    #[error(transparent)]
    Record(#[from] crate::record::Error),
    #[error(transparent)]
    Vocabulary(#[from] crate::vocabulary::Error),
    #[error(transparent)]
    Bridge(#[from] crate::bridge::Error),
    #[error(transparent)]
    Atoms(#[from] crate::atoms::Error),
    #[error(transparent)]
    Docs(#[from] crate::docs::Error),
    #[error(transparent)]
    Telos(#[from] crate::telos::Error),
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
    /// Record this project's baseline vocabulary and print the wiring steps
    Init {
        /// Print only; record nothing
        #[arg(long)]
        print: bool,
        /// Re-record the baseline even if it already exists
        #[arg(long)]
        force: bool,
    },
    /// Declare or revise a telos
    #[command(subcommand)]
    Telos(TelosAction),
    /// Declare or revise a process atom
    #[command(subcommand)]
    Atom(AtomAction),
    /// Plan a path from here to a telos, and check it could get there
    #[command(subcommand)]
    Bridge(BridgeAction),
    /// Check kan reachability and verify the live atom vocabulary composes
    Doctor,
    /// Assess whether what shipped matches what the record says
    #[command(subcommand)]
    Assess(AssessAction),
    /// Validate and record design documents
    #[command(subcommand)]
    Design(DesignAction),
    /// Record an adversarial-review verdict
    #[command(subcommand)]
    Review(ReviewAction),
    /// Report what the atom graph says follows an atom
    Next {
        /// The atom slug, e.g. `design`
        atom: String,
    },
    /// Entry point harness hooks call; prints advisory context, never blocks
    Hook {
        /// The harness event: session-start or session-end
        event: String,
    },
    /// MCP server over stdio
    Mcp,
}

/// Teloi are declared and revised with the same verb: kan is append-only, so
/// a revision is just a later claim citing the earlier one.
#[derive(Debug, Subcommand)]
pub enum TelosAction {
    /// Declare a telos, or revise it by declaring again
    Declare {
        /// Slug, e.g. `legible-process` (becomes `telos/legible-process`)
        slug: String,
        /// The telos statement
        statement: String,
        /// Declare the subject's title (requires --kind)
        #[arg(long, requires = "kind")]
        title: Option<String>,
        /// Declare the subject's kind: issue, idea, or question (requires --title)
        #[arg(long, requires = "title")]
        kind: Option<String>,
        /// An artifact type that would evidence this telos (repeatable).
        /// Types, not instances — many concrete artifacts of a declared type
        /// satisfy the telos equally, which is the weak equivalence.
        #[arg(long = "witness")]
        witnesses: Vec<String>,
    },
    /// Record that two teloi are in tension, and why
    Tension {
        /// First telos slug
        a: String,
        /// Second telos slug
        b: String,
        /// Why they pull against each other
        why: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum AtomAction {
    /// Declare a process atom, or revise it by declaring again
    Declare {
        /// Slug, e.g. `generative-build` (becomes `atom/generative-build`)
        slug: String,
        /// A type this atom requires (repeatable)
        #[arg(long = "in")]
        inputs: Vec<String>,
        /// A type this atom produces (repeatable)
        #[arg(long = "out")]
        outputs: Vec<String>,
        /// An atom slug this one composes into (repeatable)
        #[arg(long = "next")]
        next: Vec<String>,
        /// Prose describing the atom, above the generated interface block
        #[arg(long)]
        note: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum BridgeAction {
    /// Declare a bridge, or revise it by declaring again
    Declare {
        /// Slug, e.g. `v0.3` (becomes `bridge/v0.3`)
        slug: String,
        /// Target telos slug this bridge aims at
        #[arg(long)]
        telos: String,
        /// The plan: `a > b` in sequence, `a & b` concurrently, `a | b` as
        /// alternatives, parentheses to group
        #[arg(long)]
        plan: String,
        /// An artifact type already available where this bridge starts
        /// (repeatable) — the "here" in "a path from here to a telos"
        #[arg(long = "have")]
        have: Vec<String>,
        /// Prose describing the bridge, above the generated plan block
        #[arg(long)]
        note: Option<String>,
    },
    /// Check whether a declared bridge could reach its target telos
    Check {
        /// The bridge slug
        slug: String,
    },
}

/// v0.4's assessment surface. `docs` is the first leaf; telos assessment
/// sits beside it as the rest of v0.4 lands.
#[derive(Debug, Subcommand)]
pub enum AssessAction {
    /// Check that the docs still match what shipped
    Docs {
        /// Boundary to assess from (a git ref). Overrides the reconciled
        /// release boundary and skips the reconciliation check.
        #[arg(long)]
        since: Option<String>,
    },
    /// Check whether a telos's declared witnesses were actually produced
    Telos {
        /// The telos slug, e.g. `v05-shipped`. Omit with --all.
        slug: Option<String>,
        /// Assess every declared telos
        #[arg(long, conflicts_with = "slug")]
        all: bool,
        /// Execute `command` probes. Without this they are reported but
        /// never run, so a reader sees what would execute before
        /// authorizing it.
        #[arg(long)]
        run: bool,
        /// Seconds a command probe may run before it is killed
        #[arg(long, default_value_t = crate::probe::DEFAULT_TIMEOUT_SECS)]
        timeout: u64,
    },
}

#[derive(Debug, Subcommand)]
pub enum DesignAction {
    /// Validate a design document against the project's live schema
    Check {
        /// Path to the design document
        path: PathBuf,
        /// Schema slug to validate against
        #[arg(long, default_value = crate::schema::DEFAULT_SLUG)]
        schema: String,
    },
    /// Record a design pass into kan: observe, plan, and one decide per
    /// resolved question. Records even if validation fails.
    Record {
        /// Path to the design document
        path: PathBuf,
        /// Subject to record on (default: the document's filename stem)
        #[arg(long)]
        subject: Option<String>,
        /// Schema slug to validate against
        #[arg(long, default_value = crate::schema::DEFAULT_SLUG)]
        schema: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReviewAction {
    /// Append a verdict claim citing the design claim it audits
    Record {
        /// Subject being reviewed
        subject: String,
        /// One of APPROVE, APPROVE-WITH-FOLLOW-UPS, REDIRECT, BLOCK
        #[arg(long)]
        verdict: String,
        /// One-line rationale
        #[arg(long)]
        rationale: String,
        /// CID of the design claim being audited (required)
        #[arg(long, required = true)]
        cites: Vec<String>,
    },
}

pub async fn run(cli: Cli) -> Result<ExitCode, Error> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let client = KanClient::new(cwd.clone());

    match cli.command {
        // Records claims (the user's own append-only, attributable data) but
        // never writes config (opaque, unattributable, awkward to undo).
        // That split is what keeps `init` from being a silently-mutating
        // setup command.
        Command::Init { print, force } => {
            client.probe()?;
            if !print {
                let slug = crate::schema::DEFAULT_SLUG;
                if force || !crate::schema::Schema::is_declared(&client, slug)? {
                    let cid = crate::schema::Schema::starter().record(&client, slug)?;
                    println!("recorded baseline design-doc schema on `schema/{slug}` ({cid})\n");
                } else {
                    println!(
                        "baseline design-doc schema already declared on `schema/{slug}` \
                         — nothing recorded (use --force to re-record)\n"
                    );
                }
            }
            print!("{}", init_instructions());
            Ok(ExitCode::SUCCESS)
        }
        Command::Telos(TelosAction::Declare {
            slug,
            statement,
            title,
            kind,
            witnesses,
        }) => {
            // Witnesses are appended as a block only when given, so a telos
            // stays a plain statement unless it opts into being a
            // machine-checkable bridge target.
            let text = if witnesses.is_empty() {
                statement.clone()
            } else {
                crate::bridge::Witnesses { witnesses }.to_claim_text(&statement)
            };
            let outcome = crate::vocabulary::declare(
                &client,
                crate::vocabulary::Declaration {
                    subject: &format!("{}{slug}", crate::atoms::TELOS_PREFIX),
                    verb: "decide",
                    text: &text,
                    title: title.as_deref(),
                    kind: kind.as_deref(),
                    also_cite: &[],
                    act: crate::vocabulary::Act::Declare,
                },
            )?;
            print!("{}", outcome.render());
            Ok(ExitCode::SUCCESS)
        }
        // Emits both a claim and a pair of edges. The claim carries the
        // *why*, because a kan relation has no narrative body; the edges make
        // the tension queryable, which prose never was (day#18).
        //
        // Two edges, not one. `kan relate` is directed and the relation is
        // visible only from its source — verified against a real kan, where
        // `kan show telos/b` does not surface an edge declared from
        // `telos/a`. Tension is symmetric, so representing it faithfully in a
        // directed model takes both directions; with one edge, asking "what
        // is this telos in tension with" would answer correctly from one side
        // and lie by omission from the other, and which side you got would
        // depend on the order the arguments happened to be typed in.
        Command::Telos(TelosAction::Tension { a, b, why }) => {
            let prefix = crate::atoms::TELOS_PREFIX;
            let subject_a = format!("{prefix}{a}");
            let subject_b = format!("{prefix}{b}");
            let text = format!("Tension: {subject_a} vs {subject_b}. {why}");
            let outcome = crate::vocabulary::declare(
                &client,
                crate::vocabulary::Declaration {
                    subject: &subject_a,
                    verb: "decide",
                    text: &text,
                    title: None,
                    kind: None,
                    also_cite: std::slice::from_ref(&subject_b),
                    act: crate::vocabulary::Act::Relate { what: "tension" },
                },
            )?;
            print!("{}", outcome.render());

            let cites = [outcome.cid.clone()];
            let mut edges = Vec::new();
            for (from, to) in [(&subject_a, &subject_b), (&subject_b, &subject_a)] {
                edges.push(client.relate(from, TENSION_RELATION, to, &cites)?);
            }
            for (edge, (from, to)) in edges
                .iter()
                .zip([(&subject_a, &subject_b), (&subject_b, &subject_a)])
            {
                println!("  {from} {TENSION_RELATION} {to} ({edge})");
            }
            Ok(ExitCode::SUCCESS)
        }
        // Reports composition findings but records regardless: declaring a
        // multi-atom chain necessarily passes through states where it does
        // not yet compose, whatever order you declare it in.
        Command::Atom(AtomAction::Declare {
            slug,
            inputs,
            outputs,
            next,
            note,
        }) => {
            let interface = crate::atoms::Interface {
                inputs,
                outputs,
                next,
            };
            let outcome = crate::vocabulary::declare(
                &client,
                crate::vocabulary::Declaration {
                    subject: &format!("{}{slug}", crate::atoms::ATOM_PREFIX),
                    verb: "observe",
                    text: &interface.to_claim_text(&slug, note.as_deref()),
                    title: None,
                    kind: None,
                    also_cite: &[],
                    act: crate::vocabulary::Act::Declare,
                },
            )?;
            print!("{}", outcome.render());

            let report = doctor::run(&client)?;
            if !report.is_healthy() {
                println!("\nThe vocabulary does not compose yet:");
                for finding in &report.findings {
                    println!("  ! {}", finding.message);
                }
                println!(
                    "\nRecorded anyway — a chain of atoms passes through this state while \
                     you declare it."
                );
            }
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
        Command::Design(DesignAction::Check { path, schema }) => {
            let schema = crate::schema::Schema::load(&client, &schema)?;
            let doc = crate::record::read_document(&path)?;
            let report = crate::design::check(&doc, &schema, &cwd);
            print!("{}", report.render());
            Ok(if report.is_clean() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(EXIT_FINDINGS)
            })
        }
        // Records regardless of the validation result, so exit status
        // reflects "did the append succeed", not "was the doc perfect".
        Command::Design(DesignAction::Record {
            path,
            subject,
            schema,
        }) => {
            let schema = crate::schema::Schema::load(&client, &schema)?;
            let recorded =
                crate::record::design(&client, &path, &cwd, subject.as_deref(), &schema)?;
            print!("{}", recorded.render());
            Ok(ExitCode::SUCCESS)
        }
        Command::Review(ReviewAction::Record {
            subject,
            verdict,
            rationale,
            cites,
        }) => {
            let cid = crate::record::review(&client, &subject, &verdict, &rationale, &cites)?;
            println!("recorded verdict on `{subject}` ({cid})");
            Ok(ExitCode::SUCCESS)
        }
        Command::Next { atom } => {
            print!("{}", crate::record::next(&client, &atom)?);
            Ok(ExitCode::SUCCESS)
        }
        // The plan is parsed and its atoms resolved before anything is
        // written: a bridge naming an atom that does not exist is a claim
        // about nothing.
        Command::Bridge(BridgeAction::Declare {
            slug,
            telos,
            plan,
            have,
            note,
        }) => {
            let node = crate::bridge::parse(&plan)?;
            let (declared, _) = crate::atoms::load(&client)?;
            let undeclared: Vec<String> = crate::bridge::referenced(&node)
                .into_iter()
                .filter(|name| !declared.iter().any(|a| &a.name == name))
                .collect();
            if !undeclared.is_empty() {
                return Err(crate::bridge::Error::UndeclaredAtoms(undeclared.join(", ")).into());
            }

            let plan = crate::bridge::Plan {
                telos,
                have,
                plan: node,
            };
            let outcome = crate::vocabulary::declare(
                &client,
                crate::vocabulary::Declaration {
                    subject: &format!("{}{slug}", crate::bridge::BRIDGE_PREFIX),
                    verb: "observe",
                    text: &plan.to_claim_text(&slug, note.as_deref()),
                    title: None,
                    kind: None,
                    also_cite: &[],
                    act: crate::vocabulary::Act::Declare,
                },
            )?;
            print!("{}", outcome.render());

            let report = crate::bridge::check(&client, &slug)?;
            print!("\n{}", report.render());
            Ok(ExitCode::SUCCESS)
        }
        Command::Bridge(BridgeAction::Check { slug }) => {
            let report = crate::bridge::check(&client, &slug)?;
            print!("{}", report.render());
            Ok(if report.is_reachable() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(EXIT_FINDINGS)
            })
        }
        // `--run` is read here and nowhere else: authorization is a decision
        // a person makes at a terminal, per invocation. There is deliberately
        // no environment variable and no config for it, and `src/mcp.rs`
        // cannot reach this path.
        Command::Assess(AssessAction::Telos {
            slug,
            all,
            run,
            timeout,
        }) => {
            let git = crate::git::Git::new(cwd.clone());
            let auth = if run {
                crate::probe::Authorization::Run {
                    timeout: std::time::Duration::from_secs(timeout),
                }
            } else {
                crate::probe::Authorization::Report
            };

            let slugs = match (all, slug) {
                (true, _) => crate::telos::all_slugs(&client)?,
                (false, Some(slug)) => vec![slug],
                (false, None) => {
                    eprintln!("error: name a telos, or pass --all to assess every declared one");
                    return Ok(ExitCode::from(EXIT_UNAVAILABLE));
                }
            };

            let mut clean = true;
            let mut unavailable = false;
            for (i, slug) in slugs.iter().enumerate() {
                if i > 0 {
                    println!("{}", "-".repeat(60));
                }
                match crate::telos::assess(&client, &git, slug, auth) {
                    Ok(report) => {
                        print!("{}", report.render());
                        clean &= report.is_clean();
                    }
                    // A named telos that cannot be assessed is a failed
                    // invocation, not a clean one: a typo'd slug exiting 0
                    // would read as "assessed, nothing wrong" to any script.
                    // In an `--all` sweep the others are still worth
                    // reporting, so the error is printed and the run
                    // continues — but the exit code still says a check did
                    // not run.
                    Err(e) => {
                        println!("{}{slug}: {e}", crate::atoms::TELOS_PREFIX);
                        unavailable = true;
                    }
                }
            }
            // "Could not check" outranks "checked and found something": a
            // check that never ran is the weaker guarantee of the two.
            Ok(match (unavailable, clean) {
                (true, _) => ExitCode::from(EXIT_UNAVAILABLE),
                (false, false) => ExitCode::from(EXIT_FINDINGS),
                (false, true) => ExitCode::SUCCESS,
            })
        }
        Command::Assess(AssessAction::Docs { since }) => {
            let git = crate::git::Git::new(cwd.clone());
            let report = crate::docs::assess(&client, &git, &cwd, since.as_deref())?;
            print!("{}", report.render());
            Ok(if report.is_clean() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(EXIT_FINDINGS)
            })
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
