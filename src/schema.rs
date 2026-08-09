//! The design-doc schema, declared in kan rather than hard-coded here.
//!
//! A design document's shape is process vocabulary, the same category as the
//! atom vocabulary, so it lives where atoms live: a `schema/<slug>` subject
//! carrying a fenced `day-schema` JSON block, newest claim wins
//! (`docs/CONVENTIONS.md`). day ships **no** hard-coded document shape — a
//! project that wants different sections changes its claim, not day.
//!
//! When no schema is declared, day says so and offers a starter to record.
//! It does not silently fall back to an opinion: a check that quietly
//! validates against something you never chose is worse than one that
//! declines to guess.
//!
//! fallback-untested: the phrase above appears in a DENIAL — this module states
//! that it does *not* fall back to an opinion, and there is no degrade path here
//! to reach. The scan cannot tell a described fallback from a refused one, which
//! is the cost of detecting the words an author naturally writes.

use serde::{Deserialize, Serialize};

use crate::atoms::{self, newest_fenced};
use crate::kan_client::KanClient;

/// Subject-name prefix for schema declarations.
pub const SCHEMA_PREFIX: &str = "schema/";
/// Fence info string marking a schema block inside a claim's text.
fn default_resolution_prefix() -> String {
    "RQ-".to_string()
}

pub const FENCE_INFO: &str = "day-schema";
/// The schema `day design check` uses unless told otherwise.
pub const DEFAULT_SLUG: &str = "design-doc";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Vocabulary(#[from] crate::vocabulary::Error),
    #[error(
        "no design-doc schema is declared for this project (expected a `{FENCE_INFO}` block on \
         subject `{SCHEMA_PREFIX}{DEFAULT_SLUG}`).\n\nA design doc's shape is this project's \
         choice, so day does not assume one. Record a starter schema with:\n\n{starter}"
    )]
    NotDeclared { starter: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    /// Headings that must be present and non-empty, in no particular order.
    #[serde(default)]
    pub sections: Vec<String>,
    /// ID prefix for requirements, e.g. `REQ-`.
    #[serde(default = "default_requirement_prefix")]
    pub requirement_prefix: String,
    /// ID prefix for acceptance criteria, e.g. `AC-`.
    #[serde(default = "default_criterion_prefix")]
    pub criterion_prefix: String,
    #[serde(default)]
    pub min_requirements: usize,
    #[serde(default)]
    pub min_criteria: usize,
    /// Tokens that mark a document as unfinished wherever they appear
    /// outside a fenced code block.
    #[serde(default)]
    pub placeholders: Vec<String>,
    /// Heading whose backtick-quoted paths must exist on disk. Empty
    /// disables the check.
    #[serde(default)]
    pub paths_section: String,
    /// Heading whose bullet list becomes one `decide` claim each when a
    /// design is recorded.
    #[serde(default)]
    pub resolved_section: String,
    /// Prefix marking a **stable id** on a resolved-question bullet, e.g.
    /// `RQ-1`. Declared alongside `requirement_prefix` and `criterion_prefix`
    /// because it is the same mechanism for the same reason (day#36).
    ///
    /// Ids exist so re-recording a design is incremental. `day design record`
    /// appends one `decide` per resolved-question bullet, and `/design`
    /// explicitly supports iterating — so without ids, every iteration
    /// re-appends every decision already recorded. Keying on *text* was the
    /// obvious alternative and breaks the moment a bullet is reworded, which is
    /// exactly what iterating on a design does.
    #[serde(default = "default_resolution_prefix")]
    pub resolution_prefix: String,
    /// Path roots that belong to **another repository**, so a citation under
    /// one is reported as unchecked rather than as missing (day#84).
    ///
    /// The whole coordination surface between day and kan is documents in one
    /// repo about code in the other, so this recurs by construction. What made
    /// it worth a schema field rather than a heuristic is that the warning
    /// **changed what got written**: `kan/src/workspace.rs` was replaced with a
    /// symbol name to silence it, leaving the document less precise than it
    /// would have been with no check at all. A linter that degrades the artifact
    /// it validates is worse than absent.
    ///
    /// Declared rather than inferred, on the same argument day#136 settled one
    /// module over: "this segment is not a directory here" would also swallow a
    /// genuine typo in a top-level path, and an exclusion nobody can see is one
    /// nobody can correct. Empty by default, so a project that never cites
    /// across a repo boundary is unaffected.
    #[serde(default)]
    pub paths_external: Vec<String>,
}

impl crate::atoms::Versioned for Schema {
    /// A design-doc schema. v1 is every block written before versioning
    /// existed, which an absent `_version` still means.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;
}

fn default_requirement_prefix() -> String {
    "REQ-".to_string()
}

fn default_criterion_prefix() -> String {
    "AC-".to_string()
}

impl Schema {
    /// The schema day suggests when a project has none. Offered as text to
    /// record, never applied implicitly.
    pub fn starter() -> Self {
        Self {
            sections: [
                "Summary",
                "Requirements",
                "Acceptance Criteria",
                "Architecture",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            requirement_prefix: default_requirement_prefix(),
            criterion_prefix: default_criterion_prefix(),
            min_requirements: 2,
            min_criteria: 2,
            placeholders: ["TODO", "TBD"].iter().map(|s| s.to_string()).collect(),
            paths_section: "Architecture".to_string(),
            resolved_section: "Resolved Questions".to_string(),
            resolution_prefix: default_resolution_prefix(),
            // Empty in the starter, deliberately. A project that cites across
            // a repo boundary declares the root it cites into; suggesting one
            // here would ship an exclusion nobody asked for, and day#84's whole
            // complaint is about a check quietly deciding what not to look at.
            paths_external: Vec::new(),
        }
    }

    /// A ready-to-run `kan` invocation recording [`Self::starter`], so the
    /// error path hands over something runnable instead of prose.
    pub fn starter_command(slug: &str) -> String {
        let json =
            serde_json::to_string_pretty(&Self::starter()).unwrap_or_else(|_| "{}".to_string());
        format!(
            "  kan observe \"$(cat <<'EOF'\nDesign-doc schema for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{slug}"
        )
    }

    /// Loads the live schema for `slug` from kan.
    pub fn load(client: &KanClient, slug: &str) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{slug}");
        match newest_fenced::<Self>(client, &subject)? {
            Some((_cid, schema)) => Ok(schema),
            None => Err(Error::NotDeclared {
                starter: Self::starter_command(slug),
            }),
        }
    }

    /// Whether a schema is already declared for `slug`.
    pub fn is_declared(client: &KanClient, slug: &str) -> Result<bool, Error> {
        let subject = format!("{SCHEMA_PREFIX}{slug}");
        Ok(newest_fenced::<Self>(client, &subject)?.is_some())
    }

    /// Records this schema as a claim. Used by `day init` so a fresh repo
    /// reaches a working `day design check` with a command rather than a
    /// copy-paste — the starter has one definition either way, so the
    /// printed and recorded forms cannot disagree.
    pub fn record(&self, client: &KanClient, slug: &str) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string());
        let text = format!("Design-doc schema for this project.\n\n```{FENCE_INFO}\n{json}\n```\n");
        Ok(crate::vocabulary::declare(
            client,
            crate::vocabulary::Declaration {
                subject: &format!("{SCHEMA_PREFIX}{slug}"),
                verb: "observe",
                text: &text,
                title: None,
                kind: None,
                also_cite: &[],
                act: crate::vocabulary::Act::Declare,
            },
        )?
        .cid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_round_trips_through_its_own_fenced_block() {
        let command = Schema::starter_command(DEFAULT_SLUG);
        let parsed: Schema = atoms::extract_fenced(&command)
            .expect("the starter command should contain a schema block")
            .expect("the starter command's block should be valid schema JSON");
        assert_eq!(parsed, Schema::starter());
    }

    #[test]
    fn omitted_fields_fall_back_to_the_id_prefix_defaults() {
        let schema: Schema = serde_json::from_str(r#"{"sections": ["Summary"]}"#).unwrap();
        assert_eq!(schema.requirement_prefix, "REQ-");
        assert_eq!(schema.criterion_prefix, "AC-");
        assert_eq!(schema.min_requirements, 0);
    }
}
