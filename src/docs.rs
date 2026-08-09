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
    #[error(transparent)]
    Blocks(#[from] crate::blocks::Error),
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
#[serde(deny_unknown_fields)]
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

impl crate::atoms::Versioned for DocsSchema {
    /// A docs-consistency schema. v1 is every block written before versioning
    /// existed, which an absent `_version` still means.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;
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
        newest_fenced::<Self>(client, &subject)?
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
    /// The check could not be performed at all — a read that failed, not a
    /// question that was answered.
    ///
    /// **Distinct from `Fail` on purpose** (day#81). `Fail` means day looked and
    /// found the thing wrong; `Unchecked` means day could not look. Collapsing
    /// them loses the difference `docs/CONVENTIONS.md` makes the exit codes
    /// carry — could-not-check outranks checked-and-found-something — and
    /// collapsing it into the *absent* case is worse still, which is what
    /// day#81 was: an unreadable release subject reported as "a release nobody
    /// wrote down".
    Unchecked,
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

    /// Whether some check could not be performed. Reported separately from
    /// [`Self::is_clean`] because it outranks it: an assessment that could not
    /// run is a weaker guarantee than one that ran and found something.
    pub fn unchecked(&self) -> bool {
        self.findings.iter().any(|f| f.level == Level::Unchecked)
    }

    pub fn render(&self) -> String {
        let mut out = format!("Docs assessment (version {}):\n", self.version);
        for finding in &self.findings {
            let label = match finding.level {
                Level::Pass => "PASS",
                Level::Warn => "WARN",
                Level::Fail => "FAIL",
                Level::Unchecked => "UNCHECKED",
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

/// Does any live claim on `subject` name `tag`?
///
/// **Any**, not the newest. Taking only the newest claim carrying text meant
/// that recording a release correctly and then appending an ordinary note to
/// the same subject flipped it back to "not recorded" — reproduced during the
/// review of the position-honesty milestone. It is the same degenerate fold as
/// the telos-rendering defect: "newest text wins" works only while exactly one
/// kind of claim ever lands on a subject, and nothing enforces that.
///
/// Scanning every live claim also spans this repo's own mixed history — v0.3
/// and v0.4 were recorded with `kan observe`, v0.5 onward with `kan result` —
/// without day having to assume which verb a project uses.
///
/// The cost is a claim that merely *mentions* the tag in passing satisfying it.
/// That is a rarer failure than the one it removes, and day#107 is where
/// correspondence gets expressed properly rather than by substring.
///
/// Shared with [`reconcile_boundary`] on purpose: two surfaces answering the
/// same question from two implementations is how they come to disagree, which
/// is F5's other half.
fn any_claim_names(claims: &[crate::kan_client::Claim], tag: &str) -> bool {
    claims
        .iter()
        .filter_map(|c| c.text.as_deref())
        .any(|text| text.contains(tag))
}

/// day#103 — is the tag that closed the last cycle actually written down?
///
/// The same question [`reconcile_boundary`] asks, extracted so it can be asked
/// from **where position is computed** rather than only from `day assess docs`.
///
/// That reachability is the entire point of the issue. The detector of a skipped
/// `release` atom used to sit downstream of the release atom, in a manual verb —
/// so skipping the atom skipped the alarm, and the record looked complete from
/// the inside for two consecutive releases until the verb was run for an
/// unrelated reason. A check that only runs when you remember to run it is not a
/// check on the thing you forget.
///
/// Returns the finding, or `None` when tag and record agree. Errors are
/// **returned, never swallowed**: a log day could not read is not a boundary
/// that is fine, and reporting it as fine is the failure `telos/honest-reads`
/// names.
///
/// Deliberately narrower than `reconcile_boundary`, which also reports the
/// reverse case (a claim with no tag — "a boundary nobody cut"). That one is an
/// assessment-time observation about the record; this is the one a session needs
/// to see, because it is the one that means work just happened and was lost.
pub fn unrecorded_boundary(client: &KanClient, git: &Git) -> Result<Option<String>, Error> {
    let schema = match DocsSchema::load(client) {
        Ok(schema) => schema,
        // A project that has NOT DECLARED a docs schema has not told day where
        // releases are recorded, so there is no correspondence to check and no
        // finding to make. That is absence, not failure, and the two must not
        // collapse: reporting "day could not read your declaration" for a
        // project that simply has none is the same error in the opposite
        // direction from the one this function exists to fix.
        //
        // Every other error still propagates — a declaration that exists and
        // could not be read is exactly what must never be silently skipped.
        Err(Error::NotDeclared { .. }) => return Ok(None),
        Err(e) => return Err(e),
    };
    // Propagated, not defaulted. The cycle declaration is what says which tags
    // close a cycle (day#76); if day cannot read it, it does not know what to
    // look for, and defaulting to `v*` on a project whose cycles are passes
    // would produce a confident finding about the wrong tag. `status::compute`
    // surfaces the error as unreadable, which is the honest outcome — an
    // absent declaration is still the default, because `load` reports that as
    // its own case rather than as a failure.
    let cycle = crate::blocks::CycleSchema::load(client)?;

    let Some(tag) = git.latest_tag_matching(&cycle.tags)? else {
        // No boundary tag at all is not a finding. A repo that has never
        // released has nothing to have failed to record — and this is the
        // default state of every fresh clone, which CLAUDE.md names as the mode
        // a two-mode mechanism gets tested in least.
        return Ok(None);
    };

    let claims = client.show(&schema.release_subject)?;

    // Correspondence, not mere existence: a `release` claim for the PREVIOUS tag
    // would satisfy "a claim exists" while the current one went unrecorded,
    // which is exactly what happened across v0.7.0-beta.3 and v0.8.0-beta.1.
    if any_claim_names(&claims, &tag) {
        return Ok(None);
    }

    let has_any_text = claims.iter().any(|c| c.text.is_some());
    Ok(Some(if has_any_text {
        format!(
            "{tag} is tagged, but no `{}` claim mentions it — one of the two \
             records is behind",
            schema.release_subject
        )
    } else {
        format!(
            "{tag} is tagged but no `{}` claim records it — a release nobody wrote down",
            schema.release_subject
        )
    }))
}

/// Reconciles the two records of "when was the last release": the `release`
/// subject in kan, and the newest `v*` tag in git. Disagreement is a
/// finding, not something to resolve by picking a winner — a release tagged
/// but never recorded, or recorded but never cut, is exactly the drift this
/// assessment exists to surface.
fn reconcile_boundary(
    client: &KanClient,
    git: &Git,
    cycle_tags: &str,
    subject: &str,
    findings: &mut Vec<Finding>,
) -> Result<Option<String>, Error> {
    let tag = git.latest_tag_matching(cycle_tags)?;

    // day#81: this used to be `client.show(subject).unwrap_or_default()`, which
    // turned "day could not read the release subject" into "no release has been
    // recorded" — a false negative dressed as evidence, and the exact failure
    // `probe.rs`'s `ClaimLog` refuses by name. A read that failed is reported as
    // unchecked and the rest of the assessment continues; what must not happen
    // is falling through to the (Some(tag), None) arm below and announcing a
    // release nobody wrote down, on the strength of a read that never happened.
    let recorded = match client.show(subject) {
        // F5: every live claim, not the newest carrying text — see
        // `any_claim_names`. Shared with `unrecorded_boundary` so `day status`
        // and `day assess docs` cannot answer the same question differently.
        Ok(claims) => Some(claims),
        Err(e) => {
            findings.push(Finding {
                level: Level::Unchecked,
                message: format!(
                    "could not read `{subject}`, so the tag and the record could not be \
                     reconciled: {e}"
                ),
            });
            return Ok(tag);
        }
    };

    // A subject whose claims carry no text is "nothing recorded", not "recorded
    // as nothing" — normalised here so each arm below means exactly what it says
    // and the match stays exhaustive.
    let recorded = recorded
        .filter(|claims: &Vec<crate::kan_client::Claim>| claims.iter().any(|c| c.text.is_some()));

    match (&tag, &recorded) {
        (Some(tag), Some(claims)) => {
            if any_claim_names(claims, tag) {
                findings.push(Finding {
                    level: Level::Pass,
                    message: format!("release {tag} is both tagged and recorded"),
                });
            } else {
                findings.push(Finding {
                    level: Level::Warn,
                    message: format!(
                        "latest tag is {tag}, but no `{subject}` claim mentions it — one \
                         of the two records may be behind"
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
                "a `{subject}` claim exists but no `{cycle_tags}` tag does — a boundary \
                 nobody cut"
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
    // **An assessment verb reports; it does not refuse.** A cold review's
    // MAJOR-4: with the schema unreadable under a narrowed trust base this
    // exited 2, so `assess docs` was permanently unusable in exactly the
    // multi-author repo day is for — and the error's remedy ("re-run where the
    // count is zero") is unreachable, because a collaborator's claim in a
    // committed `.claims/` never goes away.
    //
    // `Level::Unchecked` is day#81's answer to this shape and already renders:
    // a check that could not run is reported as one, and the exit code follows
    // from `is_healthy`, which does not count unchecked as failure. That is the
    // difference between "day cannot answer this" and "your docs are wrong",
    // which is the whole of `telos/honest-reads`.
    let schema = match DocsSchema::load(client) {
        Ok(schema) => schema,
        Err(Error::Atoms(atoms::Error::Kan(
            e @ crate::kan_client::Error::AbsentUnderNarrowedTrust { .. },
        ))) => {
            return Ok(Report {
                version: String::new(),
                findings: vec![Finding {
                    level: Level::Unchecked,
                    message: format!(
                        "the docs schema could not be read, so nothing about the docs was \
                         checked — this is day unable to answer, not a clean assessment: {e}"
                    ),
                }],
                boundary: None,
                prompts: Vec::new(),
            });
        }
        Err(e) => return Err(e),
    };
    let version = read_version(root, &schema)?;

    let mut findings = Vec::new();
    check_versions(root, &schema, &version, &mut findings);

    // An explicit --since names the boundary outright, so there is nothing
    // to reconcile.
    // The project's declared cycle (day#76), so `assess docs` reconciles against
    // the same boundary position does. An unreadable declaration falls back to
    // release semantics and is reported, rather than silently disagreeing with
    // position about what "since" means.
    //
    // fallback-untested: reaching it needs a `schema/cycle` claim that parses as
    // a block and then fails validation, which no fixture builds yet — day#130.
    // The finding it produces IS reported, which is the property that matters
    // most here, and that half is covered.
    let cycle = match crate::blocks::CycleSchema::load(client) {
        Ok(c) => c,
        Err(e) => {
            findings.push(Finding {
                level: Level::Unchecked,
                message: format!(
                    "cycle declaration could not be read, so this reconciled against \
                     release semantics: {e}"
                ),
            });
            crate::blocks::CycleSchema::default()
        }
    };
    let boundary = match since {
        Some(reference) => Some(reference.to_string()),
        None => reconcile_boundary(
            client,
            git,
            &cycle.tags,
            &schema.release_subject,
            &mut findings,
        )?,
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
        let parsed: DocsSchema = atoms::extract_fenced(&command)
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
