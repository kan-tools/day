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
        self.latest_tag_matching(crate::blocks::DEFAULT_BOUNDARY_TAGS)
    }

    /// The newest tag matching `pattern`.
    ///
    /// Takes the pattern for the same reason [`Self::cycle_boundary_matching`]
    /// does: `assess docs` reconciles the tag against the recorded release, and
    /// day#76 asks that BOTH it and position resolve "since" against the
    /// project's declared cycle. Half of day#76 would be worse than none — a
    /// project whose cycles are passes would get pass-relative position and
    /// release-relative docs reconciliation, disagreeing with each other with no
    /// indication why.
    pub fn latest_tag_matching(&self, pattern: &str) -> Result<Option<String>, Error> {
        Ok(self.tags_matching(pattern)?.into_iter().next())
    }

    /// The current **cycle boundary**: the last release, as a tag and the
    /// moment it was cut.
    ///
    /// `None` in a repo with no release — which is a real state, not an
    /// error, and one position must handle by falling back to its cumulative
    /// behaviour. Treating an unbounded repo as "everything is the current
    /// cycle" would make a fresh clone report every atom as current.
    pub fn cycle_boundary(&self) -> Result<Option<Boundary>, Error> {
        self.cycle_boundary_matching(crate::blocks::DEFAULT_BOUNDARY_TAGS)
    }

    /// The newest tag matching `pattern`, which is what ends a cycle.
    ///
    /// The pattern is a parameter because a cycle is not always a release
    /// (day#76): a research program's is a pass, a paper's a freeze. day#60's
    /// insight — that "does an artifact of this type exist" is always-yes on a
    /// repo with history and needs bounding — is process-generic; only the
    /// binding to `v*` was software-specific.
    pub fn cycle_boundary_matching(&self, pattern: &str) -> Result<Option<Boundary>, Error> {
        Ok(self
            .tags_with_dates(pattern)?
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
    /// A cheap fingerprint of everything a `path`/`tag` probe reads.
    ///
    /// **0.04 s against `status::compute`'s 3.0 s.** That ratio is the whole
    /// reason it exists: a `UserPromptSubmit` hook runs on every prompt, so it
    /// can afford this and cannot afford that. When the fingerprint is unchanged,
    /// nothing a git-backed probe reads has moved.
    ///
    /// It covers **both** reading modes, which the first version did not. With a
    /// boundary, position reads files changed *since* it; with no `v*` tag it
    /// falls back to tracked-ever (`git ls-files`). A fingerprint of only the
    /// changed-since set was therefore the constant `"no-boundary:"` on any repo
    /// without a release — so it never moved, and the mid-session channel was
    /// permanently dead on **every fresh clone**, which is exactly the population
    /// the v1.0 bar is about. Found by the review of the fix that introduced it.
    ///
    /// It deliberately does **not** cover `claim` probes, which read the kan log
    /// — those need the expensive path (day#71), and day says so rather than
    /// implying live coverage it does not have.
    ///
    /// fallback: no-boundary-fingerprint
    pub fn position_fingerprint(&self) -> Result<String, Error> {
        let boundary = self
            .cycle_boundary()?
            .map(|b| b.tag)
            .unwrap_or_else(|| "no-boundary".to_string());
        // No boundary to diff against is a real state, not a failure: the
        // changed-since half is then simply empty.
        let mut changed = self.changed_files(&boundary).unwrap_or_default();
        changed.sort();
        // The tracked set, which is what the no-boundary fallback reads.
        //
        // fallback: no-boundary-fingerprint
        let mut tracked = self.tracked_files("*").unwrap_or_default();
        tracked.sort();
        Ok(fingerprint_of(&boundary, &changed, &tracked))
    }

    /// Branch, ahead/behind, and working-tree dirtiness, from **one**
    /// `git status` read (REQ-4/AC-4 of `.design/harness-footer.md`): one
    /// source, so the counts can never disagree with the dirtiness the way
    /// two reads taken moments apart can.
    pub fn sync_state(&self) -> Result<SyncState, Error> {
        let out = self.run(&["status", "--porcelain=v2", "--branch"])?;
        let mut state = SyncState::default();
        for line in out.lines() {
            if let Some(head) = line.strip_prefix("# branch.head ") {
                // `(detached)` is git's own rendering for a detached HEAD;
                // passed through rather than translated, because inventing a
                // second spelling for the same state helps nobody.
                let head = head.trim();
                if !head.is_empty() {
                    state.branch = Some(head.to_string());
                }
            } else if let Some(ab) = line.strip_prefix("# branch.ab ") {
                // `+A -B`. Absent entirely when there is no upstream, which
                // stays `None` — no upstream is a real state, not zero-zero.
                let mut parts = ab.split_whitespace();
                if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                    if let (Ok(ahead), Ok(behind)) = (
                        a.trim_start_matches('+').parse(),
                        b.trim_start_matches('-').parse(),
                    ) {
                        state.ahead_behind = Some((ahead, behind));
                    }
                }
            } else if !line.starts_with('#') && !line.trim().is_empty() {
                // Any non-header entry — changed, renamed, unmerged, or
                // untracked — is work the footer should call dirty.
                state.dirty = true;
            }
        }
        Ok(state)
    }

    /// The `origin` remote's URL, or `None` when there is none. A repo with
    /// no remote — a fresh `git init` — is a real state and not an error, so
    /// a failed lookup degrades to `None` while an unreachable git binary
    /// stays an error.
    ///
    /// fallback: no-remote
    pub fn remote_url(&self) -> Result<Option<String>, Error> {
        match self.run(&["remote", "get-url", "origin"]) {
            Ok(out) => {
                let url = out.trim().to_string();
                Ok((!url.is_empty()).then_some(url))
            }
            Err(Error::Failed { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// The main checkout's git directory, absolute. Its **parent** names the
    /// repository when there is no remote (REQ-12) — deliberately not
    /// `--show-toplevel`, which names the *current* directory and therefore
    /// renders a worktree's own name precisely in the case the footer exists
    /// to make visible (RQ-7, measured).
    pub fn common_dir(&self) -> Result<PathBuf, Error> {
        let out = self.run(&["rev-parse", "--git-common-dir"])?;
        let trimmed = out.trim();
        // An empty answer is a failed read, never a path: joining it onto the
        // root would silently name the *current* directory, which is exactly
        // the confusion this method exists to avoid.
        if trimmed.is_empty() {
            return Err(self.empty_output("rev-parse --git-common-dir"));
        }
        let path = PathBuf::from(trimmed);
        Ok(if path.is_absolute() {
            path
        } else {
            self.root.join(path)
        })
    }

    /// The current checkout's working-tree root. For **relativising** a
    /// worktree against the main checkout, never for naming the repo — that
    /// is [`Self::common_dir`]'s job, per RQ-7.
    pub fn toplevel(&self) -> Result<PathBuf, Error> {
        let out = self.run(&["rev-parse", "--show-toplevel"])?;
        let trimmed = out.trim();
        if trimmed.is_empty() {
            return Err(self.empty_output("rev-parse --show-toplevel"));
        }
        Ok(PathBuf::from(trimmed))
    }

    /// A read that exited zero and printed nothing where a value is
    /// mandatory. Reported as a failure rather than defaulted: rendering
    /// from output git would never produce is how a stubbed or broken git
    /// turns into a confident wrong answer.
    fn empty_output(&self, args: &str) -> Error {
        Error::Failed {
            bin: self.bin.clone(),
            args: args.to_string(),
            status: "exit 0 with empty output".to_string(),
            stderr: String::new(),
        }
    }

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
    pub fn changed_files_matching(
        &self,
        since: &str,
        pathspec: &str,
    ) -> Result<Vec<String>, Error> {
        let out = self.run(&["diff", "--name-only", since, "--", pathspec])?;
        Ok(out
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect())
    }
}

/// What one `git status --porcelain=v2 --branch` read reports (REQ-4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyncState {
    /// The branch name, or git's own `(detached)` marker. `None` when the
    /// header was absent.
    pub branch: Option<String>,
    /// Commits ahead of and behind the upstream. `None` when there is no
    /// upstream — a real state, distinct from `(0, 0)`.
    pub ahead_behind: Option<(u64, u64)>,
    /// Whether any entry — changed, renamed, unmerged, untracked — is in the
    /// working tree.
    pub dirty: bool,
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

/// The fingerprint's rendering, separated from the git reads so it can be
/// pinned by a test that needs no repository.
///
/// Both file lists are hashed — with NUL element separators, which no path
/// can contain — rather than one inlined with `,`, which a path *can*
/// contain: `["a,b"]` and `["a", "b"]` used to render identically, so a
/// position move between those states served a stale line, silently. And FNV
/// rather than `DefaultHasher`, whose value this is compared against across
/// runs of possibly different toolchains — `src/record.rs` holds the
/// algorithm and the rationale. The boundary stays readable up front: it is
/// the one part a person scanning the cache file can act on.
fn fingerprint_of(boundary: &str, changed: &[String], tracked: &[String]) -> String {
    let mut bytes = Vec::new();
    for list in [changed, tracked] {
        bytes.extend_from_slice(&(list.len() as u64).to_le_bytes());
        for path in list {
            bytes.extend_from_slice(path.as_bytes());
            bytes.push(0);
        }
    }
    format!("{boundary}:{:016x}", crate::record::fnv1a(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The comma-ambiguity defect: one path containing a comma and two paths
    /// used to render the same inlined string, so a real position move could
    /// serve a stale fingerprint. NUL-separated hashing makes the two feeds
    /// distinct by construction.
    #[test]
    fn a_path_containing_a_comma_is_not_two_paths() {
        let one = fingerprint_of("v1", &["a,b".into()], &[]);
        let two = fingerprint_of("v1", &["a".into(), "b".into()], &[]);
        assert_ne!(one, two, "{one} vs {two}");
    }

    /// The NUL separator's own case: equal counts, equal concatenation, so
    /// neither the length prefix nor the bytes alone can tell them apart —
    /// only the element boundary can.
    #[test]
    fn an_element_boundary_cannot_migrate() {
        let one = fingerprint_of("v1", &["ab".into(), "c".into()], &[]);
        let two = fingerprint_of("v1", &["a".into(), "bc".into()], &[]);
        assert_ne!(one, two);
    }

    /// The two lists are length-prefixed, so an element cannot migrate from
    /// one list to the other and feed the same bytes.
    #[test]
    fn a_changed_file_is_not_a_tracked_file() {
        let changed = fingerprint_of("v1", &["a".into()], &[]);
        let tracked = fingerprint_of("v1", &[], &["a".into()]);
        assert_ne!(changed, tracked);
    }
}
