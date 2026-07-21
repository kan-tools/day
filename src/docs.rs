//! `day assess docs` — does what the docs assert still match what shipped?
//!
//! Two tiers with different powers. The **mechanical** tier can fail: a
//! declared version-carrying file with a stale version string is wrong, full
//! stop. The **evidence** tier only prompts: it reports what changed since
//! the last release and whether any watched doc changed with it, because
//! deciding whether a change *needed* documenting means reading both, and
//! that judgment stays with whoever is reading.
//!
//! What day checks is declared per project on a `schema/docs` subject, not
//! hard-coded — not every project day might serve is a Rust CLI, so day must
//! not assume where a version lives or which files are documentation.

use serde::{Deserialize, Serialize};

use crate::atoms::{self, newest_fenced};
use crate::git::Git;
use crate::kan_client::KanClient;
use crate::schema::SCHEMA_PREFIX;

/// Subject slug day looks for: `schema/docs`.
pub const DOCS_SLUG: &str = "docs";
/// Fence info string marking a docs schema inside a claim's text.
pub const FENCE_INFO: &str = "day-docs";
/// Subject whose claims record that a release happened.
pub const RELEASE_SUBJECT: &str = "release";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error(transparent)]
    Git(#[from] crate::git::Error),
    #[error("could not read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error(
        "no docs schema is declared for this project (expected a `{FENCE_INFO}` block on \
         subject `{SCHEMA_PREFIX}{DOCS_SLUG}`).\n\nWhat counts as documentation, and where \
         the version lives, is this project's choice — day does not assume a layout. \
         Record a starter with:\n\n{starter}"
    )]
    NotDeclared { starter: String },
    #[error("no version found in {file} using key `{key}`")]
    NoVersion { file: String, key: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSchema {
    /// File holding the version of record.
    pub version_source: String,
    /// Key naming the version within that file. day takes the first quoted
    /// value on the first line containing this key, which covers TOML
    /// (`version = "1.0"`) and JSON (`"version": "1.0"`) without knowing
    /// either format.
    #[serde(default = "default_version_key")]
    pub version_key: String,
    /// Files that must carry the current version string.
    #[serde(default)]
    pub version_files: Vec<String>,
    /// Files watched for staleness: if code changed since the last release
    /// and none of these did, that is worth a look.
    #[serde(default)]
    pub doc_files: Vec<String>,
    /// Subject whose claims record that a release happened. Per-project for
    /// the same reason everything else here is: a project may already have a
    /// subject it records releases on.
    #[serde(default = "default_release_subject")]
    pub release_subject: String,
}

fn default_release_subject() -> String {
    RELEASE_SUBJECT.to_string()
}

fn default_version_key() -> String {
    "version".to_string()
}

impl DocsSchema {
    /// What day suggests when a project has none. Offered, never applied.
    pub fn starter() -> Self {
        Self {
            version_source: "Cargo.toml".to_string(),
            version_key: default_version_key(),
            version_files: vec!["README.md".to_string()],
            doc_files: vec!["README.md".to_string()],
            release_subject: default_release_subject(),
        }
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::starter()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nDocs assessment schema for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{DOCS_SLUG}"
        )
    }

    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{DOCS_SLUG}");
        newest_fenced::<Self>(client, &subject, FENCE_INFO)?
            .map(|(_cid, schema)| schema)
            .ok_or_else(|| Error::NotDeclared {
                starter: Self::starter_command(),
            })
    }
}

/// Pulls the version out of the declared source file.
fn read_version(root: &std::path::Path, schema: &DocsSchema) -> Result<String, Error> {
    let path = root.join(&schema.version_source);
    let text = std::fs::read_to_string(&path).map_err(|source| Error::Read {
        path: path.display().to_string(),
        source,
    })?;
    for line in text.lines() {
        if let Some(value) = value_after_key(line, &schema.version_key) {
            return Ok(value);
        }
    }
    Err(Error::NoVersion {
        file: schema.version_source.clone(),
        key: schema.version_key.clone(),
    })
}

/// The value following `key` on a line, without knowing the file's format.
///
/// Taking the first quoted string on the line would return the *key* in
/// JSON, where keys are quoted too. So: find the key, skip whatever
/// separator punctuation follows it, then read to the next delimiter. That
/// covers `version = "1.0"`, `"version": "1.0"`, and `version: 1.0`
/// without day knowing TOML, JSON, or YAML.
fn value_after_key(line: &str, key: &str) -> Option<String> {
    let at = line.find(key)?;
    let after = line[at + key.len()..].trim_start_matches(['"', ':', '=', ' ', '\t']);
    let value: String = after
        .chars()
        .take_while(|c| !matches!(c, '"' | ',' | ' ' | '\t'))
        .collect();
    (!value.is_empty()).then_some(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub level: Level,
    pub message: String,
}

#[derive(Debug)]
pub struct Report {
    pub version: String,
    pub findings: Vec<Finding>,
    pub boundary: Option<String>,
    pub prompts: Vec<String>,
}

impl Report {
    /// Only the mechanical tier decides the exit code. Evidence-tier prompts
    /// are for a reader to act on, not a gate.
    pub fn is_clean(&self) -> bool {
        !self.findings.iter().any(|f| f.level == Level::Fail)
    }

    pub fn render(&self) -> String {
        let mut out = format!("Docs assessment (version {}):\n", self.version);
        for finding in &self.findings {
            let label = match finding.level {
                Level::Pass => "PASS",
                Level::Warn => "WARN",
                Level::Fail => "FAIL",
            };
            out.push_str(&format!("  [{label}] {}\n", finding.message));
        }
        if !self.prompts.is_empty() {
            out.push_str(&format!(
                "\nSince {}:\n",
                self.boundary.as_deref().unwrap_or("the start of history")
            ));
            for prompt in &self.prompts {
                out.push_str(&format!("  {prompt}\n"));
            }
            out.push_str(
                "\n  These are prompts, not failures. Whether a change needed documenting\n  \
                 takes reading both; day only points at the pair.\n",
            );
        }
        out
    }
}

/// The mechanical tier: every declared version-carrying file contains the
/// version from the declared source.
fn check_versions(
    root: &std::path::Path,
    schema: &DocsSchema,
    version: &str,
    findings: &mut Vec<Finding>,
) {
    if schema.version_files.is_empty() {
        findings.push(Finding {
            level: Level::Warn,
            message: "no version-carrying files are declared, so nothing was checked".to_string(),
        });
        return;
    }
    for file in &schema.version_files {
        let path = root.join(file);
        match std::fs::read_to_string(&path) {
            Ok(text) if text.contains(version) => findings.push(Finding {
                level: Level::Pass,
                message: format!("{file} carries {version}"),
            }),
            Ok(_) => findings.push(Finding {
                level: Level::Fail,
                message: format!("{file} does not mention {version} — stale version string"),
            }),
            Err(e) => findings.push(Finding {
                level: Level::Fail,
                message: format!("{file} could not be read ({e})"),
            }),
        }
    }
}

/// Reconciles the two records of "when was the last release": the `release`
/// subject in kan, and the newest `v*` tag in git. Disagreement is a
/// finding, not something to resolve by picking a winner — a release tagged
/// but never recorded, or recorded but never cut, is exactly the drift this
/// assessment exists to surface.
fn reconcile_boundary(
    client: &KanClient,
    git: &Git,
    subject: &str,
    findings: &mut Vec<Finding>,
) -> Result<Option<String>, Error> {
    let tag = git.latest_version_tag()?;
    let release_claims = client.show(subject).unwrap_or_default();
    let recorded = release_claims.iter().rev().find_map(|c| c.text.clone());

    match (&tag, &recorded) {
        (Some(tag), Some(text)) => {
            if text.contains(tag.as_str()) {
                findings.push(Finding {
                    level: Level::Pass,
                    message: format!("release {tag} is both tagged and recorded"),
                });
            } else {
                findings.push(Finding {
                    level: Level::Warn,
                    message: format!(
                        "latest tag is {tag}, but the newest `{subject}` claim does not \
                         mention it — one of the two records may be behind"
                    ),
                });
            }
        }
        (Some(tag), None) => findings.push(Finding {
            level: Level::Warn,
            message: format!(
                "{tag} is tagged but no `{subject}` claim records it — a release nobody \
                 wrote down"
            ),
        }),
        (None, Some(_)) => findings.push(Finding {
            level: Level::Warn,
            message: format!(
                "a `{subject}` claim exists but no v* tag does — a release nobody cut"
            ),
        }),
        (None, None) => {}
    }
    Ok(tag)
}

/// The evidence tier.
///
/// **Amended from `.design/assess-docs.md` during the build.** The design
/// said this would report "claims recorded since the boundary grouped by
/// subject". That is not computable: `kan show` exposes no timestamps and no
/// anchors (kan-tools/kan#61), so day cannot tell which claims fall after a
/// git tag. What *is* computable is what changed on disk since the tag —
/// and comparing "code changed" against "watched docs changed" is both fully
/// derivable and a closer match to the actual failure this exists to catch,
/// which was a README left untouched across a release that changed the tool
/// underneath it.
fn evidence(
    git: &Git,
    schema: &DocsSchema,
    boundary: &str,
    prompts: &mut Vec<String>,
) -> Result<(), Error> {
    let changed = git.changed_files(boundary)?;
    if changed.is_empty() {
        return Ok(());
    }

    let watched: Vec<&String> = schema
        .doc_files
        .iter()
        .filter(|d| changed.contains(d))
        .collect();
    let untouched: Vec<&String> = schema
        .doc_files
        .iter()
        .filter(|d| !changed.contains(d))
        .collect();

    prompts.push(format!("{} file(s) changed", changed.len()));
    if watched.is_empty() && !schema.doc_files.is_empty() {
        prompts.push(format!(
            "none of the watched docs changed: {} — reconcile or confirm they are current",
            schema
                .doc_files
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    } else {
        prompts.push(format!(
            "watched docs changed: {}",
            watched
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if !untouched.is_empty() {
            prompts.push(format!(
                "watched docs unchanged: {}",
                untouched
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    Ok(())
}

/// Runs both tiers. Reads only — it appends no claim, because recording an
/// assessment is a separate act from performing one, and conflating them
/// would let the tool manufacture its own evidence.
pub fn assess(
    client: &KanClient,
    git: &Git,
    root: &std::path::Path,
    since: Option<&str>,
) -> Result<Report, Error> {
    let schema = DocsSchema::load(client)?;
    let version = read_version(root, &schema)?;

    let mut findings = Vec::new();
    check_versions(root, &schema, &version, &mut findings);

    // An explicit --since names the boundary outright, so there is nothing
    // to reconcile.
    let boundary = match since {
        Some(reference) => Some(reference.to_string()),
        None => reconcile_boundary(client, git, &schema.release_subject, &mut findings)?,
    };

    let mut prompts = Vec::new();
    if let Some(boundary) = boundary.as_deref() {
        evidence(git, &schema, boundary, &mut prompts)?;
    }

    Ok(Report {
        version,
        findings,
        boundary,
        prompts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_round_trips_through_its_own_block() {
        let command = DocsSchema::starter_command();
        let parsed: DocsSchema = atoms::extract_fenced(&command, FENCE_INFO)
            .expect("the starter command should carry a block")
            .expect("it should parse");
        assert_eq!(parsed, DocsSchema::starter());
    }

    /// Taking the first quoted string on the line returns the *key* in JSON,
    /// where keys are quoted too — so the value has to be read after the key,
    /// not from the start of the line.
    #[test]
    fn a_version_is_read_from_toml_json_or_yaml_without_knowing_any() {
        for (line, key, expected) in [
            (r#"version = "0.3.0-beta.1""#, "version", "0.3.0-beta.1"),
            (r#"  "version": "1.2.3","#, "version", "1.2.3"),
            ("version: 2.0.0", "version", "2.0.0"),
            // A project whose version key isn't `version` declares its own;
            // that is what `version_key` is for.
            (r#"__version__ = "9.9.9""#, "__version__", "9.9.9"),
        ] {
            assert_eq!(
                value_after_key(line, key).as_deref(),
                Some(expected),
                "failed on {line:?}"
            );
        }
        assert_eq!(value_after_key("nothing here", "version"), None);
    }

    #[test]
    fn a_stale_version_file_fails_and_a_current_one_passes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("current.md"), "we are on 1.0.0 now").unwrap();
        std::fs::write(dir.path().join("stale.md"), "we are on 0.9.0 still").unwrap();
        let schema = DocsSchema {
            version_source: "Cargo.toml".into(),
            version_key: "version".into(),
            version_files: vec!["current.md".into(), "stale.md".into()],
            doc_files: vec![],
            release_subject: "release".into(),
        };
        let mut findings = Vec::new();
        check_versions(dir.path(), &schema, "1.0.0", &mut findings);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].level, Level::Pass);
        assert_eq!(findings[1].level, Level::Fail);
        assert!(findings[1].message.contains("stale.md"));
    }
}
