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

    /// The most recent `v*` tag by creation date, if any.
    pub fn latest_version_tag(&self) -> Result<Option<String>, Error> {
        Ok(self.tags_matching("v*")?.into_iter().next())
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
}

/// Whether `path` is inside a git working tree, without invoking git —
/// used to give a clearer error than a git failure would.
pub fn looks_like_repo(root: &Path) -> bool {
    root.join(".git").exists()
}
