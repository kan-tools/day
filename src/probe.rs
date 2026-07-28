//! day's third substrate: **project-declared commands**, executed only on
//! explicit request.
//!
//! kan was the first substrate, git (read-only) the second, taken on
//! reluctantly in v0.4. This is the third, and it is the widest: a `command`
//! probe runs a program this repo's own kan log names. `docs/CONVENTIONS.md`
//! says an assessment is judged against *material evidence — builds, tests,
//! diffs* — and a witness like `passing-tests` is not expressible without
//! running something. That is the whole justification, and it is worth
//! stating plainly rather than discovering later.
//!
//! Five guardrails bound it, each with a test:
//!
//! 1. **No shell, ever.** An argv is split on whitespace and executed
//!    directly, the way [`crate::kan_client::KanClient`] and
//!    [`crate::git::Git`] already spawn. There is no `sh -c` path, so `;`,
//!    `|`, `&&`, and backticks arriving from a claim are inert — they become
//!    literal arguments, not operators.
//! 2. **Opt-in per invocation.** A command probe runs only when the caller
//!    passes [`Authorization::Run`]. Otherwise it reports
//!    [`Verdict::NotRun`] along with the exact argv, so a reader sees what
//!    they would be authorizing before authorizing it.
//! 3. **Never over MCP.** `src/mcp.rs` constructs [`Authorization::Report`]
//!    with no way to change it, so an agent calling a read-shaped tool
//!    cannot cause execution.
//! 4. **Bounded.** A probe that outlives its timeout is killed and reported
//!    as [`Verdict::TimedOut`], distinct from both satisfied and failed.
//! 5. **Confined.** Every process day spawns for a probe is spawned here,
//!    which is what makes guardrail 1 greppable rather than merely intended.
//!
//! **None of that applies to a `claim` probe**, and the distinction is the
//! point rather than an oversight. A `claim` probe reads kan — day's *first*
//! substrate, the one it was built on — through the same public read verbs
//! `atoms::load` already uses. It spawns no project-declared program, so
//! there is nothing to shell-escape, nothing for `--run` to gate, and no
//! reason to withhold it from MCP. Folding it into the guardrails above
//! would suggest the guardrails are about probes in general; they are about
//! *executing what a claim names*, which is a narrower and much sharper
//! thing.

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::git::Git;
use crate::kan_client::KanClient;

/// How long a command probe may run before it is killed.
pub const DEFAULT_TIMEOUT_SECS: u64 = 120;

/// How often the timeout loop checks whether the child has exited. Short
/// enough that a fast probe is not padded, long enough not to spin.
const POLL: Duration = Duration::from_millis(50);

// There is deliberately no `Error` type here. Every way a probe can fail to
// answer -- a missing program, a git read that failed, a timeout -- is a
// [`Verdict`], because "the evidence could not be established" is a result
// the report has to show a reader, not an error that aborts the assessment.
// An `--all` sweep must not lose four teloi because the fifth named a
// program that is not installed.

/// What would evidence a witness type. Declared per project on
/// `schema/witness`; day ships no built-in mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Probe {
    /// A git pathspec matching at least one **tracked** file.
    Path(String),
    /// A git tag glob matching at least one tag.
    Tag(String),
    /// A command whose exit status is the evidence: zero means satisfied.
    Command(String),
    /// A claim in the kan log. The one probe kind whose evidence is the
    /// *record* rather than the world — which is what a witness like
    /// `verdict` or `assessment` actually is, and why neither was probeable
    /// before (day#60).
    Claim(ClaimShape),
}

/// Which claims count as evidence: a kan `ClaimKind`, optionally narrowed by
/// any combination of a substring marker, an anchored prefix, and a subject
/// scope.
///
/// Narrowing exists because `kind` alone is often far too broad — every
/// `day review record` writes a `Decision`, but so does every other decision
/// in the log. It picks out *which instances count*, the same job day#34 gave
/// a telos's `scope`, and every predicate here is deliberately a plain
/// string operation rather than a pattern language: a probe definition
/// arrives from a claim, and a regex engine reading claim-supplied input is a
/// wider surface than this needs.
///
/// The predicates are **independent and conjunctive** (day#70). Each is
/// optional, an absent one constrains nothing, and [`Self::matches`] requires
/// every present one. That shape is load-bearing rather than incidental: the
/// frame-relative dimension (whose evidence counts, deferred to kan#117)
/// arrives later as one more optional field and one more conjunct, without
/// re-litigating how the existing predicates are expressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimShape {
    /// A kan claim kind as `kan show --json` renders it — `Observation`,
    /// `Plan`, `Decision`, `Result`, `Subject`, `Relation`. Matched exactly,
    /// against kan's rendering rather than day's idea of it.
    pub kind: String,
    /// A substring the claim's text must contain, **anywhere** in it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    /// A prefix the claim's text must **begin** with.
    ///
    /// Distinct from [`Self::contains`], and the distinction is the whole
    /// reason it exists: `contains` is substring-anywhere, so a probe for
    /// `"adversarial review of"` matched the very decision that *defined*
    /// that marker as well as every real verdict. `day review record` anchors
    /// the marker at the start of the text, so an anchored predicate
    /// separates the two where no other dimension can — a real verdict and
    /// the decision quoting it can share both subject and author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
    /// Which subjects a matching claim may live on, **glob-lite**: a value
    /// ending in `*` is a prefix match on the part before it (`atom/*` is any
    /// `atom/…` subject, bare `*` is any subject at all), and a value without
    /// one is exact (`release` is only the `release` subject).
    ///
    /// Not a glob engine, for day#34's reason: a trailing-`*` prefix covers
    /// the real need (namespace scoping), and a pattern language over
    /// claim-supplied input is a wider surface than that need justifies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl ClaimShape {
    /// Whether one claim on `subject` matches this shape, ignoring when it was
    /// recorded. Time is the caller's business — [`claims_matching`] applies
    /// the cycle boundary — because the same shape means "ever" for an
    /// assessment and "this cycle" for position.
    ///
    /// The subject is passed in rather than read off the claim because
    /// `kan show --json` renders a claim within a subject rather than on one;
    /// [`ClaimLog`] already carries the `(subject, claim)` pair, so scoping
    /// costs no extra read.
    fn matches(&self, subject: &str, claim: &crate::kan_client::Claim) -> bool {
        if claim.kind != self.kind {
            return false;
        }
        // A conjunction of independent predicates: every present one must
        // hold, an absent one is vacuously satisfied. Written as a chain of
        // early returns rather than a single expression so a new dimension is
        // an insertion, not a rewrite.
        if let Some(scope) = &self.subject {
            if !subject_matches(scope, subject) {
                return false;
            }
        }
        // A claim carrying no text (a retraction, a relation) cannot satisfy
        // a text predicate, so a narrowed probe never matches one.
        if let Some(marker) = &self.contains {
            if !claim.text.as_deref().is_some_and(|t| t.contains(marker)) {
                return false;
            }
        }
        if let Some(prefix) = &self.starts_with {
            if !claim.text.as_deref().is_some_and(|t| t.starts_with(prefix)) {
                return false;
            }
        }
        true
    }

    /// How this shape reads in a verdict, e.g. ``Decision` claim(s) starting
    /// with `adversarial review of``.
    fn describe(&self) -> String {
        let mut described = format!("`{}` claim(s)", self.kind);
        if let Some(marker) = &self.contains {
            described.push_str(&format!(" containing `{marker}`"));
        }
        if let Some(prefix) = &self.starts_with {
            described.push_str(&format!(" starting with `{prefix}`"));
        }
        if let Some(scope) = &self.subject {
            described.push_str(&format!(" on `{scope}`"));
        }
        described
    }
}

/// Glob-lite subject scoping: a trailing `*` makes the rest a prefix,
/// anything else is an exact match. Deliberately not a glob crate — see
/// [`ClaimShape::subject`].
fn subject_matches(pattern: &str, subject: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => subject.starts_with(prefix),
        None => subject == pattern,
    }
}

/// Whether the caller has authorized command execution for this invocation.
///
/// A two-variant enum rather than a `bool` so that `mcp.rs` constructing
/// [`Authorization::Report`] reads as a decision at the call site, and so
/// that no caller can pass `true` by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authorization {
    /// Report what would run; execute nothing.
    Report,
    /// Execute command probes, bounded by the timeout.
    Run { timeout: Duration },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The probe found its evidence.
    Satisfied(String),
    /// The probe ran and found nothing.
    Unsatisfied(String),
    /// A command probe that was not authorized. **Not a failure** — absence
    /// of evidence, not evidence of absence. Treating it as a failure would
    /// make every default invocation look broken.
    NotRun(String),
    /// A command probe killed at the timeout.
    TimedOut(String),
    /// Something prevented the probe from being evaluated at all.
    Error(String),
}

impl Verdict {
    /// Only a probe that ran and found nothing counts against the telos.
    pub fn is_failure(&self) -> bool {
        matches!(self, Verdict::Unsatisfied(_))
    }

    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Satisfied(_) => "MATERIAL",
            Verdict::Unsatisfied(_) => "MISSING",
            Verdict::NotRun(_) => "NOT RUN",
            Verdict::TimedOut(_) => "TIMEOUT",
            Verdict::Error(_) => "ERROR",
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Verdict::Satisfied(d)
            | Verdict::Unsatisfied(d)
            | Verdict::NotRun(d)
            | Verdict::TimedOut(d)
            | Verdict::Error(d) => d,
        }
    }
}

/// Evaluates a probe **cumulatively** — "was this ever produced".
///
/// `path`, `tag`, and `claim` are reads over substrates day already has, so
/// they run unconditionally. `command` runs only under [`Authorization::Run`].
///
/// This is assessment's path and it has no notion of a cycle: a telos asks
/// whether work ever landed in its equivalence class, and a release or a
/// review from any time is real evidence for that. Position asks a different
/// question and resolves the same probes against a boundary — see
/// [`crate::position::resolve`]. Keeping the two in separate functions is
/// what makes `assess` cycle-blind by construction rather than by a flag
/// somebody has to remember not to set.
pub fn evaluate(probe: &Probe, git: &Git, log: &ClaimLog<'_>, auth: Authorization) -> Verdict {
    match probe {
        Probe::Path(pathspec) => match git.tracked_files(pathspec) {
            Ok(files) if files.is_empty() => {
                Verdict::Unsatisfied(format!("no tracked file matches `{pathspec}`"))
            }
            Ok(files) => Verdict::Satisfied(summarize(&files, pathspec)),
            Err(e) => Verdict::Error(format!("could not list tracked files: {e}")),
        },
        Probe::Tag(pattern) => match git.tags_matching(pattern) {
            Ok(tags) if tags.is_empty() => {
                Verdict::Unsatisfied(format!("no tag matches `{pattern}`"))
            }
            Ok(tags) => Verdict::Satisfied(format!("git tag {}", tags[0])),
            Err(e) => Verdict::Error(format!("could not list tags: {e}")),
        },
        Probe::Command(argv) => match auth {
            Authorization::Report => Verdict::NotRun(format!(
                "would run `{argv}` — re-run with --run to execute it"
            )),
            Authorization::Run { timeout } => run_command(argv, git.root(), timeout),
        },
        // Deliberately not gated on `auth`. There is nothing to authorize:
        // this reads the log through kan's read verbs and executes nothing.
        Probe::Claim(shape) => claims_matching(shape, log, None),
    }
}

/// The log, read at most once and reused by every `claim` probe.
///
/// A claim witness is not tied to a subject — an assessment may be recorded
/// on any `atom/*`, a verdict on whatever subject was reviewed — so answering
/// one means reading the whole log, which on a real repo is a `kan show` per
/// subject. Doing that per probe made `day status` take seconds on day's own
/// log; every probe shares one read instead, and a caller that declares no
/// claim probe pays nothing because the read is lazy.
///
/// Scoped to a single command, deliberately. **This is not a store**: it
/// lives on the stack for one invocation and dies with it, so day still keeps
/// nothing of its own and a claim recorded between two runs is visible to the
/// second.
pub struct ClaimLog<'a> {
    client: &'a KanClient,
    /// `(subject, claim)` pairs, or the error that stopped the read. Loaded
    /// on first use; `OnceCell` rather than a `Mutex` because a probe
    /// evaluation is single-threaded.
    loaded: std::cell::OnceCell<Result<Vec<(String, crate::kan_client::Claim)>, String>>,
}

impl<'a> ClaimLog<'a> {
    pub fn new(client: &'a KanClient) -> Self {
        Self {
            client,
            loaded: std::cell::OnceCell::new(),
        }
    }

    fn claims(&self) -> Result<&[(String, crate::kan_client::Claim)], &str> {
        self.loaded
            .get_or_init(|| {
                let subjects = self
                    .client
                    .subjects()
                    .map_err(|e| format!("could not list subjects: {e}"))?;
                let mut all = Vec::new();
                for subject in subjects {
                    // A subject day cannot read is an error, never a
                    // silently empty result — the failure mode
                    // `kan_client`'s shape check exists to end. Reporting
                    // "no matching claim" because a read failed would be a
                    // false negative dressed as evidence.
                    let claims = self
                        .client
                        .show(&subject)
                        .map_err(|e| format!("could not read `{subject}`: {e}"))?;
                    all.extend(claims.into_iter().map(|c| (subject.clone(), c)));
                }
                Ok(all)
            })
            .as_ref()
            .map(Vec::as_slice)
            .map_err(String::as_str)
    }
}

/// Whether a live claim matching `shape` exists in the kan log.
///
/// `since` is a **cycle boundary** in microseconds since the epoch, matching
/// `recorded_at` as `kan show --json` emits it: only claims recorded strictly
/// after it count. `None` asks the cumulative question — "was such a claim
/// ever recorded" — which is what an assessment wants.
///
/// **This spawns no project-declared program.** It reads through
/// [`ClaimLog`], which runs `kan status` and `kan show` — the same reads
/// `atoms::load` and `status::last_assessed_atom` already make — and nothing
/// else.
pub fn claims_matching(shape: &ClaimShape, log: &ClaimLog<'_>, since: Option<i64>) -> Verdict {
    let claims = match log.claims() {
        Ok(claims) => claims,
        Err(e) => return Verdict::Error(e.to_string()),
    };

    let window = match since {
        Some(_) => " since the cycle boundary",
        None => "",
    };

    let mut found = 0usize;
    let mut newest: Option<&str> = None;
    for (subject, claim) in claims {
        if !shape.matches(subject, claim) {
            continue;
        }
        if let Some(boundary) = since {
            // An undated claim cannot be placed in a cycle at all, so it does
            // not count as *this* one. Conservative on purpose: the
            // alternative would let a claim from a kan that omits
            // `recorded_at` read as current work.
            if claim.recorded_at.unwrap_or(0) <= boundary {
                continue;
            }
        }
        found += 1;
        newest = Some(subject);
    }

    match found {
        0 => Verdict::Unsatisfied(format!("no live {}{window}", shape.describe())),
        n => Verdict::Satisfied(format!(
            "{n} {}{window}, newest on `{}`",
            shape.describe(),
            newest.unwrap_or_default()
        )),
    }
}

fn summarize(files: &[String], pathspec: &str) -> String {
    match files {
        [one] => format!("tracked: {one}"),
        many => format!("{} tracked files match `{pathspec}`", many.len()),
    }
}

/// Runs a command probe. **This is the only place day spawns a process for a
/// probe**, and the only place the no-shell rule has to hold.
///
/// The argv is split on whitespace and passed to [`Command::new`] directly.
/// Nothing is interpreted: a probe declared as `echo hi; rm -rf /` runs
/// `echo` with the literal arguments `hi;`, `rm`, `-rf`, `/`. That costs
/// pipelines and redirection in probe definitions, which is the right trade
/// for a check whose entire value is being hard to game.
fn run_command(argv: &str, cwd: &Path, timeout: Duration) -> Verdict {
    let mut parts = argv.split_whitespace();
    let Some(program) = parts.next() else {
        return Verdict::Error("probe command is empty".to_string());
    };
    let args: Vec<&str> = parts.collect();

    // Output is discarded rather than captured: the exit status is the
    // evidence, and a probe that prints a megabyte should not be able to
    // fill day's memory or block on a full pipe buffer.
    let child = Command::new(program)
        .args(&args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => return Verdict::Error(format!("could not run `{argv}`: {e}")),
    };

    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return if status.success() {
                    Verdict::Satisfied(format!("`{argv}` exited 0"))
                } else {
                    Verdict::Unsatisfied(format!("`{argv}` exited {status}"))
                };
            }
            Ok(None) => {}
            Err(e) => return Verdict::Error(format!("could not wait on `{argv}`: {e}")),
        }
        if Instant::now() >= deadline {
            // Kill and reap, so the probe cannot outlive the assessment as
            // an orphan holding the terminal.
            let _ = child.kill();
            let _ = child.wait();
            return Verdict::TimedOut(format!("`{argv}` exceeded {}s", timeout.as_secs()));
        }
        std::thread::sleep(POLL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A kan that would fail loudly if invoked. Every test below either uses
    /// a probe kind that never touches kan, or asserts the resulting error.
    fn no_kan() -> KanClient {
        KanClient::with_bin(
            std::env::temp_dir(),
            "definitely-not-a-real-kan-binary".to_string(),
        )
    }

    #[test]
    fn a_command_probe_is_not_run_without_authorization() {
        let git = Git::new(std::env::temp_dir());
        let verdict = evaluate(
            &Probe::Command("false".into()),
            &git,
            &ClaimLog::new(&no_kan()),
            Authorization::Report,
        );
        assert!(matches!(verdict, Verdict::NotRun(_)), "{verdict:?}");
        assert!(
            verdict.detail().contains("--run"),
            "a not-run probe should say how to authorize it: {verdict:?}"
        );
        assert!(!verdict.is_failure(), "not-run is absence of evidence");
    }

    #[test]
    fn exit_status_is_the_evidence() {
        let run = Authorization::Run {
            timeout: Duration::from_secs(10),
        };
        let git = Git::new(std::env::temp_dir());
        assert!(matches!(
            evaluate(
                &Probe::Command("true".into()),
                &git,
                &ClaimLog::new(&no_kan()),
                run
            ),
            Verdict::Satisfied(_)
        ));
        let failed = evaluate(
            &Probe::Command("false".into()),
            &git,
            &ClaimLog::new(&no_kan()),
            run,
        );
        assert!(matches!(failed, Verdict::Unsatisfied(_)), "{failed:?}");
        assert!(failed.is_failure());
    }

    /// The guardrail that matters most. `sh -c "echo hi > marker"` would
    /// create a file; `Command::new("echo")` with those as literal arguments
    /// cannot. Asserting the file's absence is what distinguishes "we did not
    /// use a shell" from "we hope we did not".
    #[test]
    fn shell_metacharacters_are_inert() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("pwned");
        let git = Git::new(dir.path());
        let run = Authorization::Run {
            timeout: Duration::from_secs(10),
        };

        let verdict = evaluate(
            &Probe::Command(format!("echo hi > {}", marker.display())),
            &git,
            &ClaimLog::new(&no_kan()),
            run,
        );
        // `echo` succeeds — it happily prints `hi > /path` — but the
        // redirection never happened.
        assert!(matches!(verdict, Verdict::Satisfied(_)), "{verdict:?}");
        assert!(
            !marker.exists(),
            "a `>` in a probe was interpreted as redirection; day used a shell"
        );

        for argv in [
            format!("true; touch {}", marker.display()),
            format!("true && touch {}", marker.display()),
            format!("true | touch {}", marker.display()),
        ] {
            let _ = evaluate(
                &Probe::Command(argv.clone()),
                &git,
                &ClaimLog::new(&no_kan()),
                run,
            );
            assert!(
                !marker.exists(),
                "metacharacters in {argv:?} reached a shell"
            );
        }
    }

    #[test]
    fn a_probe_that_outlives_its_timeout_is_killed() {
        let git = Git::new(std::env::temp_dir());
        let started = Instant::now();
        let verdict = evaluate(
            &Probe::Command("sleep 30".into()),
            &git,
            &ClaimLog::new(&no_kan()),
            Authorization::Run {
                timeout: Duration::from_millis(200),
            },
        );
        assert!(matches!(verdict, Verdict::TimedOut(_)), "{verdict:?}");
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the assessment should return at the timeout, not wait out the probe"
        );
        // A timeout is not a failing probe: the evidence is unknown, not
        // absent, and exiting non-zero on it would conflate the two.
        assert!(!verdict.is_failure());
    }

    #[test]
    fn a_missing_program_is_an_error_not_a_failure() {
        let git = Git::new(std::env::temp_dir());
        let verdict = evaluate(
            &Probe::Command("definitely-not-a-real-program-xyz".into()),
            &git,
            &ClaimLog::new(&no_kan()),
            Authorization::Run {
                timeout: Duration::from_secs(5),
            },
        );
        assert!(matches!(verdict, Verdict::Error(_)), "{verdict:?}");
        assert!(!verdict.is_failure());
    }

    #[test]
    fn probes_round_trip_through_their_declared_form() {
        let json = r#"{"tag":"v*"}"#;
        let probe: Probe = serde_json::from_str(json).unwrap();
        assert_eq!(probe, Probe::Tag("v*".into()));
        assert_eq!(serde_json::to_string(&probe).unwrap(), json);
    }

    /// A shape with nothing but a kind, for tests that narrow one dimension
    /// at a time. Written as a constructor rather than a `Default` impl
    /// because a shape with no `kind` is not a thing a probe may be.
    fn shape(kind: &str) -> ClaimShape {
        ClaimShape {
            kind: kind.into(),
            contains: None,
            starts_with: None,
            subject: None,
        }
    }

    fn a_claim(kind: &str, text: Option<&str>) -> crate::kan_client::Claim {
        crate::kan_client::Claim {
            cid: "bafy".into(),
            kind: kind.into(),
            text: text.map(str::to_string),
            title: None,
            author: None,
            recorded_at: Some(10),
        }
    }

    /// AC-4 (and day#60's AC-7 before it): a `claim` probe round-trips, and
    /// every optional predicate is omitted when absent so an unnarrowed probe
    /// stays the short form a person would write by hand — which is also what
    /// makes a block written before day#70 still the block day would write.
    #[test]
    fn a_claim_probe_round_trips_through_its_declared_form() {
        let narrowed = r#"{"claim":{"kind":"Decision","contains":"adversarial review of"}}"#;
        let probe: Probe = serde_json::from_str(narrowed).unwrap();
        assert_eq!(
            probe,
            Probe::Claim(ClaimShape {
                contains: Some("adversarial review of".into()),
                ..shape("Decision")
            })
        );
        assert_eq!(serde_json::to_string(&probe).unwrap(), narrowed);

        let bare = r#"{"claim":{"kind":"Result"}}"#;
        let probe: Probe = serde_json::from_str(bare).unwrap();
        assert_eq!(probe, Probe::Claim(shape("Result")));
        assert_eq!(serde_json::to_string(&probe).unwrap(), bare);

        let scoped = r#"{"claim":{"kind":"Result","subject":"atom/*"}}"#;
        let probe: Probe = serde_json::from_str(scoped).unwrap();
        assert_eq!(
            probe,
            Probe::Claim(ClaimShape {
                subject: Some("atom/*".into()),
                ..shape("Result")
            })
        );
        assert_eq!(serde_json::to_string(&probe).unwrap(), scoped);

        let anchored = r#"{"claim":{"kind":"Decision","starts_with":"adversarial review of"}}"#;
        let probe: Probe = serde_json::from_str(anchored).unwrap();
        assert_eq!(
            probe,
            Probe::Claim(ClaimShape {
                starts_with: Some("adversarial review of".into()),
                ..shape("Decision")
            })
        );
        assert_eq!(serde_json::to_string(&probe).unwrap(), anchored);

        // All three at once, to pin the field order the serialized form takes
        // and prove no predicate is dropped on the way through.
        let every =
            r#"{"claim":{"kind":"Decision","contains":"c","starts_with":"s","subject":"x/*"}}"#;
        let probe: Probe = serde_json::from_str(every).unwrap();
        assert_eq!(
            probe,
            Probe::Claim(ClaimShape {
                kind: "Decision".into(),
                contains: Some("c".into()),
                starts_with: Some("s".into()),
                subject: Some("x/*".into()),
            })
        );
        assert_eq!(serde_json::to_string(&probe).unwrap(), every);
    }

    /// REQ-8, at the level the guardrail actually lives: a `claim` probe is
    /// not an execution path. Both authorizations are exercised, because the
    /// probe is deliberately *not* gated on `auth` — if `Run` made it spawn
    /// anything, the timeout in scope here would be the tell.
    ///
    /// The proof is indirect but exact: with kan pointed at a binary that
    /// does not exist, a claim probe can only report the failure to reach
    /// *kan*. Were it routed through `run_command`, the error would name the
    /// probe's own argv instead — and there is no argv to name.
    #[test]
    fn a_claim_probe_never_spawns_a_command() {
        let git = Git::new(std::env::temp_dir());
        let shape = shape("Result");
        for auth in [
            Authorization::Report,
            Authorization::Run {
                timeout: Duration::from_secs(10),
            },
        ] {
            let verdict = evaluate(
                &Probe::Claim(shape.clone()),
                &git,
                &ClaimLog::new(&no_kan()),
                auth,
            );
            let detail = verdict.detail();
            assert!(matches!(verdict, Verdict::Error(_)), "{verdict:?}");
            assert!(
                detail.contains("could not list subjects"),
                "a claim probe must fail as a kan read, not as an execution: {detail}"
            );
            assert!(
                detail.contains("kan"),
                "the failure should name kan, the substrate it reads: {detail}"
            );
        }
    }

    /// The `contains` marker narrows rather than matches loosely, and a
    /// claim with no text can never satisfy a narrowed probe.
    #[test]
    fn a_marker_narrows_which_claims_count() {
        let claim = a_claim;
        let narrowed = ClaimShape {
            contains: Some("adversarial review of".into()),
            ..shape("Decision")
        };
        assert!(narrowed.matches(
            "rigor",
            &claim("Decision", Some("adversarial review of foo: APPROVE — ok"))
        ));
        assert!(!narrowed.matches("rigor", &claim("Decision", Some("an unrelated decision"))));
        assert!(!narrowed.matches("rigor", &claim("Result", Some("adversarial review of foo"))));
        // A relation carries no text, so it cannot contain a marker.
        assert!(!narrowed.matches("rigor", &claim("Decision", None)));

        let bare = shape("Result");
        assert!(bare.matches("atom/build", &claim("Result", None)));
        assert!(!bare.matches("atom/build", &claim("Decision", Some("anything"))));
    }

    /// AC-2: `starts_with` is **anchored** where `contains` is not, and that
    /// difference is the whole of its reason to exist. The two claims here are
    /// the real pair off day's own log: a verdict `day review record` wrote,
    /// and the decision that *defined* the marker by quoting it mid-sentence.
    /// A `contains` probe cannot tell them apart; an anchored one can.
    #[test]
    fn an_anchored_prefix_and_a_substring_are_different_predicates() {
        let marker = "adversarial review of";
        let verdict = a_claim(
            "Decision",
            Some("adversarial review of rigor: APPROVE — holds"),
        );
        let quoting = a_claim(
            "Decision",
            Some("The `verdict` marker is the prefix adversarial review of that record::review writes."),
        );

        let anchored = ClaimShape {
            starts_with: Some(marker.into()),
            ..shape("Decision")
        };
        assert!(anchored.matches("rigor", &verdict));
        assert!(
            !anchored.matches("current-cycle-position", &quoting),
            "a claim that merely quotes the marker must not satisfy an anchored probe"
        );

        // The same value as `contains` matches both — which is what makes the
        // assertion above about anchoring rather than about these fixtures.
        let substring = ClaimShape {
            contains: Some(marker.into()),
            ..shape("Decision")
        };
        assert!(substring.matches("rigor", &verdict));
        assert!(substring.matches("current-cycle-position", &quoting));

        // And a claim with no text satisfies neither.
        assert!(!anchored.matches("rigor", &a_claim("Decision", None)));
    }

    /// AC-1: glob-lite subject scoping — a trailing `*` is a prefix, anything
    /// else is exact, and the pattern is *not* a glob (`atom/*` does not
    /// mean "one path segment", and a `*` in the middle is a literal).
    #[test]
    fn a_subject_scope_is_glob_lite() {
        let scoped = ClaimShape {
            subject: Some("atom/*".into()),
            ..shape("Result")
        };
        let assessed = a_claim("Result", Some("assessed"));
        assert!(scoped.matches("atom/build", &assessed));
        assert!(scoped.matches("atom/generative-build", &assessed));
        // The two false positives day#70 recorded on day's own log.
        assert!(!scoped.matches("release", &assessed));
        assert!(!scoped.matches("spine", &assessed));
        // A prefix, not a path-segment glob: nested subjects still match.
        assert!(scoped.matches("atom/a/b", &assessed));
        // And the prefix must be a prefix, not a substring.
        assert!(!scoped.matches("my-atom/build", &assessed));

        // No `*`: an exact subject.
        let exact = ClaimShape {
            subject: Some("release".into()),
            ..shape("Result")
        };
        assert!(exact.matches("release", &assessed));
        assert!(!exact.matches("release-notes", &assessed));
        assert!(!exact.matches("atom/build", &assessed));

        // Bare `*`: every subject, the same as declaring no scope at all.
        let any = ClaimShape {
            subject: Some("*".into()),
            ..shape("Result")
        };
        for subject in ["release", "atom/build", ""] {
            assert!(any.matches(subject, &assessed), "{subject:?}");
        }

        // A `*` anywhere but the end is a literal character, not a wildcard —
        // the honest consequence of not shipping a glob engine.
        let literal = ClaimShape {
            subject: Some("a*b".into()),
            ..shape("Result")
        };
        assert!(literal.matches("a*b", &assessed));
        assert!(!literal.matches("axb", &assessed));
    }

    /// AC-3: `matches` is a conjunction — every present predicate must hold,
    /// and an absent one constrains nothing.
    ///
    /// Each case below flips exactly one dimension away from a claim that
    /// matches on all four, so a predicate that had been dropped from the
    /// conjunction would fail precisely one assertion here.
    #[test]
    fn every_present_predicate_must_hold() {
        let all = ClaimShape {
            kind: "Decision".into(),
            contains: Some("APPROVE".into()),
            starts_with: Some("adversarial review of".into()),
            subject: Some("atom/*".into()),
        };
        let matching = a_claim(
            "Decision",
            Some("adversarial review of build: APPROVE — holds"),
        );
        assert!(all.matches("atom/build", &matching));

        // Wrong kind.
        assert!(!all.matches(
            "atom/build",
            &a_claim(
                "Result",
                Some("adversarial review of build: APPROVE — holds")
            )
        ));
        // Right kind, right text, wrong subject.
        assert!(!all.matches("release", &matching));
        // Right everywhere but the substring.
        assert!(!all.matches(
            "atom/build",
            &a_claim("Decision", Some("adversarial review of build: REVISE — no"))
        ));
        // Right everywhere but the anchor.
        assert!(!all.matches(
            "atom/build",
            &a_claim("Decision", Some("re: adversarial review of build: APPROVE"))
        ));

        // And omitting a predicate imposes nothing from it: the same claim
        // that failed the subject test above matches once the scope is gone.
        let unscoped = ClaimShape {
            subject: None,
            ..all.clone()
        };
        assert!(unscoped.matches("release", &matching));
    }

    /// The rendered form names every predicate in force, so a reader of a
    /// verdict can see what was actually asked rather than inferring it.
    #[test]
    fn a_verdict_describes_the_predicates_in_force() {
        assert_eq!(shape("Result").describe(), "`Result` claim(s)");
        assert_eq!(
            ClaimShape {
                subject: Some("atom/*".into()),
                ..shape("Result")
            }
            .describe(),
            "`Result` claim(s) on `atom/*`"
        );
        assert_eq!(
            ClaimShape {
                starts_with: Some("adversarial review of".into()),
                ..shape("Decision")
            }
            .describe(),
            "`Decision` claim(s) starting with `adversarial review of`"
        );
    }
}
