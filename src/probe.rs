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

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::git::Git;

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

/// Evaluates a probe.
///
/// `path` and `tag` are reads over substrates day already has, so they run
/// unconditionally. `command` runs only under [`Authorization::Run`].
pub fn evaluate(probe: &Probe, git: &Git, auth: Authorization) -> Verdict {
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

    #[test]
    fn a_command_probe_is_not_run_without_authorization() {
        let git = Git::new(std::env::temp_dir());
        let verdict = evaluate(&Probe::Command("false".into()), &git, Authorization::Report);
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
            evaluate(&Probe::Command("true".into()), &git, run),
            Verdict::Satisfied(_)
        ));
        let failed = evaluate(&Probe::Command("false".into()), &git, run);
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
            let _ = evaluate(&Probe::Command(argv.clone()), &git, run);
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
}
