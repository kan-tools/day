//! day's second substrate: git, **read-only**.
//!
//! Until v0.4 day talked to kan and nothing else. Answering "which files
//! changed since the last release" needs a commit-level view that kan does
//! not expose (`kan show` prints no artifacts or anchors — kan-tools/kan#61),
//! so day reads git directly.
//!
//! Every call here is a read subcommand. There is deliberately no method
//! that stages, commits, tags, checks out, or pushes, and
//! `tests/assess.rs` greps this module for the mutating ones so the
//! read-only claim stays true rather than merely intended. Keeping all git
//! access behind this one module is what makes that grep a guarantee instead
//! of a spot check.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Overrides the `git` binary, so tests can point at a stub.
pub const GIT_BIN_ENV: &str = "DAY_GIT_BIN";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("git is not reachable (tried to run `{bin}`): {source}")]
    NotReachable {
        bin: String,
        #[source]
        source: std::io::Error,
    },
    #[error("`{bin} {args}` failed ({status}){stderr}")]
    Failed {
        bin: String,
        args: String,
        status: String,
        stderr: String,
    },
}

pub struct Git {
    bin: String,
    root: PathBuf,
}

impl Git {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            bin: std::env::var(GIT_BIN_ENV).unwrap_or_else(|_| "git".to_string()),
            root: root.into(),
        }
    }

    /// A `Git` pointed at a specific binary, so a test can supply a stub
    /// without mutating the process environment — `DAY_GIT_BIN` is global,
    /// and parallel tests setting it race.
    pub fn with_bin(root: impl Into<PathBuf>, bin: impl Into<String>) -> Self {
        Self {
            bin: bin.into(),
            root: root.into(),
        }
    }

    /// The repository root these reads run against. Exposed so a command
    /// probe runs in the same directory day is assessing, rather than
    /// wherever the process happened to be started.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn run(&self, args: &[&str]) -> Result<String, Error> {
        let output = Command::new(&self.bin)
            .args(args)
            .current_dir(&self.root)
            .output()
            .map_err(|source| Error::NotReachable {
                bin: self.bin.clone(),
                source,
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(Error::Failed {
                bin: self.bin.clone(),
                args: args.join(" "),
                status: output.status.to_string(),
                stderr: if stderr.is_empty() {
                    stderr
                } else {
                    format!(": {stderr}")
                },
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Tags matching a glob, newest first by creation date.
    pub fn tags_matching(&self, pattern: &str) -> Result<Vec<String>, Error> {
        let out = self.run(&["tag", "--list", pattern, "--sort=-creatordate"])?;
        Ok(out
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }

    /// Tags matching a glob with their creation dates (unix seconds), newest
    /// first.
    ///
    /// The dates are what makes "was this tag created *this* cycle"
    /// answerable. Same `git tag --list` read as [`Self::tags_matching`],
    /// with a format string — no new subcommand, so the read-only whitelist
    /// in `tests/assess.rs` still covers it.
    pub fn tags_with_dates(&self, pattern: &str) -> Result<Vec<(String, i64)>, Error> {
        let out = self.run(&[
            "tag",
            "--list",
            pattern,
            "--sort=-creatordate",
            "--format=%(refname:strip=2)\t%(creatordate:unix)",
        ])?;
        Ok(out
            .lines()
            .filter_map(|line| {
                let (name, date) = line.trim_end().split_once('\t')?;
                // A tag whose date git could not render is skipped rather
                // than defaulted to 0: an epoch date would make it look
                // older than every boundary and silently never count.
                Some((name.to_string(), date.trim().parse().ok()?))
            })
            .filter(|(name, _)| !name.is_empty())
            .collect())
    }

    /// The most recent `v*` tag by creation date, if any.
    pub fn latest_version_tag(&self) -> Result<Option<String>, Error> {
        Ok(self.tags_matching("v*")?.into_iter().next())
    }

    /// The current **cycle boundary**: the last release, as a tag and the
    /// moment it was cut.
    ///
    /// `None` in a repo with no release — which is a real state, not an
    /// error, and one position must handle by falling back to its cumulative
    /// behaviour. Treating an unbounded repo as "everything is the current
    /// cycle" would make a fresh clone report every atom as current.
    pub fn cycle_boundary(&self) -> Result<Option<Boundary>, Error> {
        Ok(self
            .tags_with_dates("v*")?
            .into_iter()
            .next()
            .map(|(tag, at_unix)| Boundary { tag, at_unix }))
    }

    /// Tracked files matching a pathspec.
    ///
    /// Deliberately `ls-files` rather than a glob crate walking the working
    /// tree. It adds no dependency, reuses a substrate day already reads
    /// under the read-only whitelist, and asks a stricter question:
    /// tracked-in-git is stronger evidence than a file merely existing, so a
    /// build output or a stray local file cannot witness a telos.
    pub fn tracked_files(&self, pathspec: &str) -> Result<Vec<String>, Error> {
        let out = self.run(&["ls-files", "--", pathspec])?;
        Ok(out
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }

    /// Files changed between `since` and the working tree.
    pub fn changed_files(&self, since: &str) -> Result<Vec<String>, Error> {
        let out = self.run(&["diff", "--name-only", since])?;
        Ok(out
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }

    /// Files matching a pathspec that changed between `since` and the working
    /// tree — [`Self::tracked_files`] scoped to one cycle.
    ///
    /// The pathspec goes to git rather than being matched in day, so a
    /// `path` probe means exactly the same thing bounded as unbounded. Doing
    /// the glob here instead would need a matcher of day's own, and it would
    /// disagree with `ls-files` at the edges — which is the sort of drift
    /// that makes a probe report differently depending on which question
    /// asked it.
    pub fn changed_files_matching(&self, since: &str, pathspec: &str) -> Result<Vec<String>, Error> {
        let out = self.run(&["diff", "--name-only", since, "--", pathspec])?;
        Ok(out
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }
}

/// The boundary of the current cycle: the last release.
///
/// Derived from git on every read, never stored — day owns no state, and a
/// boundary that went stale in a cache would be worse than no boundary at
/// all. Carries both the tag (for a diff) and its time (for a claim's
/// `recorded_at`), because a cycle has to be expressible in both substrates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boundary {
    /// The tag naming the release, e.g. `v0.6.0-beta.1`.
    pub tag: String,
    /// When it was created, in seconds since the epoch.
    pub at_unix: i64,
}

impl Boundary {
    /// The boundary in the unit kan stamps claims with: **microseconds**
    /// since the epoch. Verified against the real binary rather than assumed
    /// — `recorded_at` is an integer, and `tests/kan_conformance.rs` is what
    /// caught day typing it as a string.
    pub fn at_micros(&self) -> i64 {
        self.at_unix.saturating_mul(1_000_000)
    }
}

/// Whether `path` is inside a git working tree, without invoking git —
/// used to give a clearer error than a git failure would.
pub fn looks_like_repo(root: &Path) -> bool {
    root.join(".git").exists()
}
