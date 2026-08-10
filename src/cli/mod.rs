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

/// Parses `--scope <witness>=<pattern>`. Split on the FIRST `=` only: a
/// pattern may itself contain one, and a witness type may not.
fn parse_scope(raw: &str) -> Result<(String, String), String> {
    let (witness, pattern) = raw
        .split_once('=')
        .ok_or_else(|| format!("expected <witness>=<pattern>, got `{raw}`"))?;
    if witness.trim().is_empty() || pattern.trim().is_empty() {
        return Err(format!("both sides of `=` must be non-empty, got `{raw}`"));
    }
    Ok((witness.trim().to_string(), pattern.trim().to_string()))
}

/// Parses `--witness-any a,b` into one alternative set.
///
/// **A single member is refused rather than accepted quietly.** `--witness-any
/// a` is either a typo for `--witness a` or a half-written group, and both are
/// better as an error than as a declaration that reads like a disjunction and
/// behaves like a conjunct. An empty set is refused for the sharper reason that
/// it can never be satisfied by anything, which would make the whole telos
/// permanently unmet — the quiet-check failure this milestone exists to end,
/// arriving through the declaration rather than through a probe.
fn parse_witness_any(raw: &str) -> Result<Vec<String>, String> {
    let members: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|m| !m.is_empty())
        .map(str::to_string)
        .collect();
    if members.len() < 2 {
        // Says `--witness` without its placeholder on purpose. The exact string
        // `--witness` followed by a bracketed placeholder is the signature of
        // the solo-guess remedy day removed, and `tests/plugin.rs` reserves it
        // so that scan can stay precise. This message is about which flag to
        // use, not about how to establish a witness, so it loses nothing.
        return Err(format!(
            "expected at least two comma-separated types, got `{raw}` — \
             a one-member alternative is just `--witness`"
        ));
    }
    Ok(members)
}

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
    #[error(transparent)]
    Tension(#[from] crate::tension::Error),
    #[error(transparent)]
    Status(#[from] crate::status::Error),
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
    /// Report where the work currently sits: the inferred atom, its satisfied
    /// inputs, its met and unmet criteria, what the graph says follows, and
    /// any off-sequence finding. Inferred from artifacts, never tracked;
    /// always exits zero.
    Status,
    /// Print the cached status line. Reads **only** the render cache — never
    /// kan, never git — because Claude Code cancels an in-flight status line
    /// at 300ms. Written by `day hook session-start`. Hidden: it is for the
    /// harness, not for people, who run `day status`.
    #[command(name = "status-line", hide = true)]
    StatusLine,
    /// Entry point harness hooks call; prints advisory context, never blocks
    Hook {
        /// The harness event: session-start, session-notice, user-prompt, or session-end
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
        /// A set of artifact types, ANY ONE of which evidences this telos, as
        /// `a,b` (repeatable — each occurrence is one alternative set). Use it
        /// when several different artifacts would each independently show the
        /// telos holds; `--witness` twice means both are required.
        #[arg(long = "witness-any", value_parser = parse_witness_any)]
        witness_any: Vec<Vec<String>>,
        /// Narrow which instances of a witness count, as `<witness>=<pattern>`
        /// (repeatable). The project's schema/witness map still decides which
        /// kind of probe runs; this only tightens its pattern, so `'v0.5*'`
        /// names a narrower class rather than one artifact. Quote the
        /// argument: an unquoted `*` is expanded by the shell first, and zsh
        /// errors on a failed match rather than passing it through (day#83).
        #[arg(long = "scope", value_parser = parse_scope)]
        scopes: Vec<(String, String)>,
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
        /// An atom slug this one composes into, forward only (repeatable).
        /// Must not form a cycle: use --revisits for an edge that sends you
        /// back.
        #[arg(long = "next")]
        next: Vec<String>,
        /// An atom slug a negative outcome here sends you back to (repeatable).
        /// Feedback, not sequence — it is never treated as an ordering.
        #[arg(long = "revisits")]
        revisits: Vec<String>,
        /// A witness type that evidences this atom is done (repeatable).
        /// Resolved through schema/witness, the same probes teloi use.
        #[arg(long = "done")]
        done: Vec<String>,
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
    /// Check whether an atom's declared `done` criteria are met
    Atom {
        /// The atom slug, e.g. `generative-build`
        slug: String,
        /// Execute `command` probes. Without this they are reported but never
        /// run, matching `assess telos`.
        #[arg(long)]
        run: bool,
        /// Seconds a command probe may run before it is killed
        #[arg(long, default_value_t = crate::probe::DEFAULT_TIMEOUT_SECS)]
        timeout: u64,
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
            // day#95: `probe` proves the *binary* runs. It does not prove this
            // repo's log can be read, and `init` printed "kan: reachable" on
            // the strength of it — so in a repo where kan cannot derive
            // identity (a git repo with no commits, say), `day init --print`
            // said "you are set up" while `day doctor` correctly said the
            // opposite. day's two health checks disagreed, and the one that
            // disagreed is the one a new project runs first.
            //
            // Asked here, once, and handed to the renderer, so the line cannot
            // claim more than was verified: the string is computed from the
            // read rather than written beside it.
            let log = client.subjects().map(|_| ());
            // **The honest report is not for `--print` only.** The first fix
            // computed `log` and then let the recording path below fail with a
            // bare `error: … failed (exit status: 1)`, so `day init --print`
            // explained itself and `day init` — the default invocation, in the
            // exact state day#95 describes — did not. Found by a cold review of
            // this branch, which ran both.
            //
            // Recording genuinely cannot happen against a log day cannot read,
            // so it is skipped and *said to be skipped*, the wiring is printed
            // anyway (it is what you need in order to fix this), and the exit
            // code reports that the requested work did not all happen.
            if log.is_err() && !print {
                print!("{}", init_instructions(log.as_ref()));
                println!(
                    "\nThe baseline design-doc schema was NOT recorded: that needs a log \
                     day can read.\nRe-run `day init` once `day doctor` is clean."
                );
                return Ok(ExitCode::from(EXIT_UNAVAILABLE));
            }
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
            print!("{}", init_instructions(log.as_ref()));
            Ok(ExitCode::SUCCESS)
        }
        Command::Telos(TelosAction::Declare {
            slug,
            statement,
            title,
            kind,
            witnesses,
            witness_any,
            scopes,
        }) => {
            // `--witness` first, then each `--witness-any` set, so the declared
            // order matches the order given rather than depending on how clap
            // happened to bucket the flags.
            let groups: Vec<crate::bridge::Group> = witnesses
                .into_iter()
                .map(crate::bridge::Group::One)
                .chain(witness_any.into_iter().map(crate::bridge::Group::Any))
                .collect();
            // Witnesses are appended as a block only when given, so a telos
            // stays a plain statement unless it opts into being a
            // machine-checkable bridge target.
            let groups_for_caution = groups.clone();
            let text = if groups.is_empty() && scopes.is_empty() {
                statement.clone()
            } else {
                crate::bridge::Witnesses {
                    witnesses: groups,
                    scope: scopes.into_iter().collect(),
                }
                .to_claim_text(&statement)
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
            // After the declaration, never instead of it. day#138: three
            // witnesses were declared that could not fail, and nothing said so
            // until someone assessed the telos and read the numbers.
            let types: Vec<String> = groups_for_caution
                .iter()
                .flat_map(|g| {
                    g.members()
                        .into_iter()
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                })
                .collect();
            let git = crate::git::Git::new(cwd.clone());
            print!(
                "{}",
                crate::telos::cautions_for(&client, &git, &types, crate::telos::Declared::Telos)?
            );
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
        // The reason lands on `tension/<a>--<b>`, NOT on either telos
        // subject (day#32). Everywhere day renders a telos it shows the
        // newest claim carrying text, so a reason recorded here would
        // displace the telos statement in injected session context and in
        // assessments — which is exactly what it did for four of six teloi.
        // A telos subject carries its declaration and its edges; the reason
        // lives on the subject that is about the relationship.
        Command::Telos(TelosAction::Tension { a, b, why }) => {
            let prefix = crate::atoms::TELOS_PREFIX;
            let subject_a = format!("{prefix}{a}");
            let subject_b = format!("{prefix}{b}");
            let tension = crate::tension::Tension::new(&a, &b);
            let subject = tension.subject();

            // Citing both teloi keeps `declare`'s existence check: a tension
            // against a telos nobody declared is a claim about nothing.
            let outcome = crate::vocabulary::declare(
                &client,
                crate::vocabulary::Declaration {
                    subject: &subject,
                    verb: "decide",
                    text: &tension.to_claim_text(&why),
                    title: None,
                    kind: None,
                    also_cite: &[subject_a.clone(), subject_b.clone()],
                    act: crate::vocabulary::Act::Declare,
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
            revisits,
            done,
            note,
        }) => {
            // Kept before the move into `Interface`, because the caution runs
            // after the declaration and needs the same list day just recorded.
            let done_for_caution = done.clone();
            let interface = crate::atoms::Interface {
                inputs,
                outputs,
                next,
                revisits,
                done,
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

            // day#146. `done` criteria are witness types resolved by the same
            // probes a telos's witnesses are, and this verb never asked. Five
            // of day's own nine atoms were declared with a `claim` probe as
            // their sole criterion — structurally unable to report unmet — and
            // `day assess atom`, which `day status` names as the gate, has
            // passed on them ever since.
            //
            // After the declaration, never instead of it: same rule as the
            // telos side. Reported, never refused.
            let git = crate::git::Git::new(cwd.clone());
            print!(
                "{}",
                crate::telos::cautions_for(
                    &client,
                    &git,
                    &done_for_caution,
                    crate::telos::Declared::Atom
                )?
            );

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
            match hooks::dispatch(&event, &client, &cwd) {
                Ok(text) => print!("{text}"),
                Err(e) => println!("## day\n\n{e}"),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Design(DesignAction::Check { path, schema }) => {
            let schema = crate::schema::Schema::load(&client, &schema)?;
            let doc = crate::record::read_document(&path)?;
            let mut report = crate::design::check(&doc, &schema, &cwd);

            // day#41: validate against the decisions already on the subject, not
            // only against the document's own structure. The subject is the
            // document's filename stem, the same inference `design record` uses.
            //
            // A read failure is reported, not swallowed: "day could not check
            // the record" and "the record holds nothing" are different, which is
            // the rule `a_failed_kan_read_is_never_swallowed` now enforces.
            let subject = crate::record::slug_for(&path);
            match client.show(&subject) {
                Ok(claims) => {
                    let recorded: Vec<String> = claims
                        .iter()
                        .filter(|c| c.kind == "Decision")
                        .filter_map(|c| c.text.clone())
                        .collect();
                    report.findings.extend(crate::design::check_against_record(
                        &doc, &schema, &recorded,
                    ));
                }
                Err(e) => report.findings.push(crate::design::Finding {
                    verdict: crate::design::Verdict::Warn,
                    message: format!(
                        "could not read `{subject}`, so decisions already recorded there \
                         were not checked against this document: {e}"
                    ),
                }),
            }
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
        Command::Next { atom } => match crate::record::next(&client, &atom) {
            Ok(out) => {
                print!("{out}");
                Ok(ExitCode::SUCCESS)
            }
            // A typo'd atom gets told what IS declared — the reader's next act
            // is to find the right name, and day holds the list (`telos/v1.0`:
            // error messages that teach). Enriched here rather than in
            // `record::next`'s own error because the list is presentation, and
            // the exit path mirrors main's: same prefix, same code.
            Err(e @ crate::record::Error::NoSuchAtom(_)) => {
                let (atoms, _) = crate::atoms::load(&client)?;
                let declared: Vec<String> = atoms.iter().map(|a| a.name.clone()).collect();
                eprintln!("error: {e}{}", crate::telos::list_known("atoms", &declared));
                Ok(ExitCode::from(EXIT_UNAVAILABLE))
            }
            Err(e) => Err(e.into()),
        },
        // Display only, and always exit zero: status *reports* where the work
        // sits (AC-11): `day assess atom` is the gate that exits non-zero, so
        // a status finding never fails a script that merely asked where it is.
        // Runs no command probe — position and `done` are resolved with
        // Authorization::Report inside `status::compute`.
        // **Always exits zero, including when the read fails** (day#95).
        //
        // `--help` states that contract, and anything wiring `day status` into
        // a hook, a prompt or a CI step is entitled to rely on it — a position
        // report that can fail a step is not advisory, which is what
        // `telos/affordance-not-enforcement` forbids. It exited 2 whenever
        // `kan status --json` failed, e.g. in a git repo with no commits.
        //
        // Degrading is not going quiet: the error is printed, in the same
        // "installed but not readable here" shape `day hook session-start`
        // already uses for this exact state. `day doctor` keeps exiting
        // non-zero — diagnosing is its job, and it is the command whose failure
        // is supposed to mean something.
        Command::Status => {
            let git = crate::git::Git::new(cwd.clone());
            match crate::status::compute(&client, &git) {
                Ok(status) => print!("{}", status.render_long()),
                Err(e) => print!("{}", crate::status::render_unreadable(&e)),
            }
            Ok(ExitCode::SUCCESS)
        }
        // The one place day reads the cache. It never touches kan or git —
        // pointing `DAY_KAN_BIN` at a nonexistent path leaves this untouched,
        // which is what AC-8 asserts. An absent cache prints nothing, its
        // documented empty state, not an error.
        //
        // Claude Code pipes session JSON to the status line on stdin and does
        // not guarantee the command's cwd is the project root — the documented
        // way to learn it is `workspace.current_dir` on stdin. So the root is
        // read from there, falling back to the process cwd (which is what a
        // person running this by hand, or a test, gets).
        Command::StatusLine => {
            let root = statusline_root(cwd);
            if let Some(line) = crate::cache::read_status_line(&root) {
                print!("{line}");
            }
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
                (true, _) => {
                    let sweep = crate::telos::all_slugs(&client)?;
                    if sweep.retracted > 0 {
                        // Said once, up front — the hook's telos list already
                        // excludes these, and the two surfaces disagreeing
                        // about how many teloi exist was itself a finding.
                        println!(
                            "{} fully retracted telos subject(s) not assessed — nothing live \
                             declares them. Name one explicitly to inspect it.",
                            sweep.retracted
                        );
                    }
                    sweep.slugs
                }
                (false, Some(slug)) => vec![slug],
                (false, None) => {
                    eprintln!("error: name a telos, or pass --all to assess every declared one");
                    return Ok(ExitCode::from(EXIT_UNAVAILABLE));
                }
            };

            let mut clean = true;
            let mut unavailable = false;
            let mut rendered = 0usize;
            for (i, slug) in slugs.iter().enumerate() {
                if i > 0 {
                    println!("{}", "-".repeat(60));
                }
                match crate::telos::assess(&client, &git, slug, auth) {
                    Ok(report) => {
                        // In a sweep the run-constant coda prints once at the
                        // end; a single assessment keeps it attached. Fourteen
                        // identical copies per run trained exactly the
                        // skimming the coda warns against.
                        if all {
                            print!("{}", report.render_bare());
                        } else {
                            print!("{}", report.render());
                        }
                        rendered += 1;
                        clean &= report.is_clean();
                    }
                    // A named telos that cannot be assessed is a failed
                    // invocation, not a clean one: a typo'd slug exiting 0
                    // would read as "assessed, nothing wrong" to any script.
                    // In an `--all` sweep the others are still worth
                    // reporting, so the error is printed and the run
                    // continues — but the exit code still says a check did
                    // not run.
                    // No subject prefix here: every `telos::Error` that concerns
                    // one subject names it. Prefixing anyway is what produced
                    // `telos/bad: telos/bad: …` for the one variant that
                    // already did.
                    Err(e) => {
                        println!("{e}");
                        unavailable = true;
                    }
                }
            }
            // Once, and only when at least one assessment rendered — the coda
            // reads "the evidence above", and above an all-errors sweep there
            // is none.
            if all && rendered > 0 {
                print!("{}", crate::telos::Report::coda());
            }
            // "Could not check" outranks "checked and found something": a
            // check that never ran is the weaker guarantee of the two.
            Ok(match (unavailable, clean) {
                (true, _) => ExitCode::from(EXIT_UNAVAILABLE),
                (false, false) => ExitCode::from(EXIT_FINDINGS),
                (false, true) => ExitCode::SUCCESS,
            })
        }
        Command::Assess(AssessAction::Atom { slug, run, timeout }) => {
            let git = crate::git::Git::new(cwd.clone());
            let auth = if run {
                crate::probe::Authorization::Run {
                    timeout: std::time::Duration::from_secs(timeout),
                }
            } else {
                crate::probe::Authorization::Report
            };
            let report = crate::telos::assess_atom(&client, &git, &slug, auth)?;
            print!("{}", report.render());
            Ok(if report.is_clean() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(EXIT_FINDINGS)
            })
        }
        Command::Assess(AssessAction::Docs { since }) => {
            let git = crate::git::Git::new(cwd.clone());
            let report = crate::docs::assess(&client, &git, &cwd, since.as_deref())?;
            print!("{}", report.render());
            // Same precedence the telos sweep uses, and for the same reason
            // (day#81): "could not check" outranks "checked and found
            // something", because a check that never ran is the weaker
            // guarantee of the two.
            Ok(match (report.unchecked(), report.is_clean()) {
                (true, _) => ExitCode::from(EXIT_UNAVAILABLE),
                (false, false) => ExitCode::from(EXIT_FINDINGS),
                (false, true) => ExitCode::SUCCESS,
            })
        }
        Command::Mcp => {
            mcp::serve(cwd).await?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// The directory the status line should read its cache from.
///
/// Claude Code delivers `workspace.current_dir` on stdin and does not promise
/// the command runs in the project root, so stdin is the authority when
/// present. Reading stdin is guarded by [`IsTerminal`]: a person who types
/// `day status-line` at a prompt must not have it block waiting for EOF on
/// their terminal, and a harness always pipes JSON (or nothing) rather than a
/// tty. An unparseable or dir-less payload falls back to the process cwd.
///
/// fallback-untested: four sub-states reach that `return fallback` — a tty,
/// empty stdin, unparseable JSON, and JSON with no `current_dir` — and none has
/// a fixture, because reaching them means driving the hidden `status-line` verb
/// with a stdin no test builds today. day#130, rather than hatched silently.
fn statusline_root(fallback: PathBuf) -> PathBuf {
    use std::io::{IsTerminal, Read};

    let mut stdin = std::io::stdin();
    if stdin.is_terminal() {
        return fallback;
    }
    let mut buf = String::new();
    if stdin.read_to_string(&mut buf).is_err() || buf.trim().is_empty() {
        return fallback;
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&buf) else {
        return fallback;
    };
    value
        .get("workspace")
        .and_then(|w| w.get("current_dir"))
        .or_else(|| value.get("cwd"))
        .and_then(|d| d.as_str())
        .map(PathBuf::from)
        .unwrap_or(fallback)
}

/// Prints, never mutates — the same contract `kan mcp install` set. day
/// touches a user's Claude Code config only by telling them what to add.
///
/// `log` is the result of actually reading this repo's log, not a description
/// of it (day#95). Taking the `Result` rather than a `bool` or a pre-rendered
/// string is deliberate: the caller cannot hand over a verdict it did not
/// obtain, and the error it did obtain is the thing worth printing.
pub fn init_instructions(log: Result<&(), &crate::kan_client::Error>) -> String {
    let exe = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "day".to_string());

    let mut out = String::new();
    match log {
        Ok(()) => out.push_str("kan: reachable, and this repo's log reads\n\n"),
        // Reported, and the wiring below is still printed: the steps are what
        // you need in order to fix this, so withholding them would be the least
        // useful moment to go quiet.
        Err(e) => out.push_str(&format!(
            "kan: found on PATH, but this repo's log could not be read ({e}).\n                  That is expected if kan has never seen this repo. `day doctor` says more.\n                  The wiring below applies either way.\n\n"
        )),
    }
    out.push_str("Wire day into this repo — either path works.\n\n");
    out.push_str("1. As a Claude Code plugin (recommended; brings the skills and the\n");
    out.push_str("   session-start hook with it):\n");
    out.push_str("     /plugin install <path to the day repo, or its marketplace entry>\n\n");
    out.push_str("2. Piecemeal, without the plugin:\n");
    out.push_str(&format!("     claude mcp add day -- {exe} mcp\n"));
    out.push_str("   plus two SessionStart hooks in .claude/settings.json — one injects\n");
    out.push_str("   process context to the model, one shows the human a notice when the\n");
    out.push_str("   work has moved past its last assessment:\n");
    out.push_str(&format!(
        "     {{\"hooks\": {{\"SessionStart\": [\n       \
         {{\"hooks\": [{{\"type\": \"command\", \"command\": \"{exe} hook session-start\"}}]}},\n       \
         {{\"hooks\": [{{\"type\": \"command\", \"command\": \"{exe} hook session-notice\"}}]}}\n     ]}}}}\n\n"
    ));
    // The status line is deliberately NOT in the plugin path above: a Claude
    // Code plugin's settings support only the `agent` and `subagentStatusLine`
    // keys, so a plugin cannot declare the top-level `statusLine` (verified
    // against the plugins reference). It is opt-in, in the user's own settings,
    // either way — so it is documented here for both paths rather than implied
    // to come with the plugin when it cannot.
    out.push_str("To see day's process position at a glance, add a status line to your own\n");
    out.push_str("settings (~/.claude/settings.json) — it reads a cache the session-start\n");
    out.push_str("hook writes, so it never shells out and never lags:\n");
    out.push_str(&format!(
        "     {{\"statusLine\": {{\"type\": \"command\", \"command\": \"{exe} status-line\"}}}}\n\n"
    ));
    // day#77 ask #2, answered as *offer* rather than *record*. The four
    // declarations below all have working defaults, so recording them at init
    // would impose four choices on a project that has not made them — and one
    // of them (`schema/blocks`) has no sensible default at all: its starter
    // carries the research loop's `research-claim`, which is an example of the
    // shape, not a vocabulary any other project wants. What was actually
    // missing is that a project had no way to learn these exist; the starters
    // were written and nothing printed them.
    out.push_str("Optional declarations. Each has a working default, so a project that\n");
    out.push_str("declares none of them is fully configured — these exist for when the\n");
    out.push_str("default is wrong (docs/CONVENTIONS.md):\n\n");
    out.push_str(&format!(
        "  schema/{}     invent a fenced block type of your own\n\
         {}\n\n",
        crate::blocks::BLOCKS_SLUG,
        crate::blocks::BlockSchemas::starter_command()
    ));
    out.push_str(&format!(
        "  schema/{}   which review verdicts this project accepts\n\
         {}\n\n",
        crate::blocks::VERDICTS_SLUG,
        crate::blocks::VerdictVocabulary::starter_command()
    ));
    out.push_str(&format!(
        "  schema/{}      which tags bound a cycle (default: {})\n\
         {}\n\n",
        crate::blocks::CYCLE_SLUG,
        crate::blocks::DEFAULT_BOUNDARY_TAGS,
        crate::blocks::CycleSchema::starter_command()
    ));
    out.push_str(&format!(
        "  schema/{}  how often a standing condition is re-shown (default: every {})\n\
         {}\n\n",
        crate::blocks::INJECTION_SLUG,
        crate::cache::DEFAULT_CADENCE,
        crate::blocks::InjectionSchema::starter_command()
    ));

    out.push_str("day stores nothing of its own: teloi, atoms, and assessments all live in\n");
    out.push_str("kan as claims (docs/CONVENTIONS.md). The only file day writes is a\n");
    out.push_str("gitignored, disposable render cache under .day/ (display only). Nothing\n");
    out.push_str("above is written for you.\n");
    out
}

/// Exit code used when day cannot reach kan at all. Exposed so `main` can
/// map the error case without duplicating the constant.
pub fn unavailable() -> ExitCode {
    ExitCode::from(EXIT_UNAVAILABLE)
}
