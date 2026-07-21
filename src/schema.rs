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

use serde::{Deserialize, Serialize};

use crate::atoms::{self, newest_fenced};
use crate::kan_client::KanClient;

/// Subject-name prefix for schema declarations.
pub const SCHEMA_PREFIX: &str = "schema/";
/// Fence info string marking a schema block inside a claim's text.
pub const FENCE_INFO: &str = "day-schema";
/// The schema `day design check` uses unless told otherwise.
pub const DEFAULT_SLUG: &str = "design-doc";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(
        "no design-doc schema is declared for this project (expected a `{FENCE_INFO}` block on \
         subject `{SCHEMA_PREFIX}{DEFAULT_SLUG}`).\n\nA design doc's shape is this project's \
         choice, so day does not assume one. Record a starter schema with:\n\n{starter}"
    )]
    NotDeclared { starter: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        match newest_fenced::<Self>(client, &subject, FENCE_INFO)? {
            Some((_cid, schema)) => Ok(schema),
            None => Err(Error::NotDeclared {
                starter: Self::starter_command(slug),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_round_trips_through_its_own_fenced_block() {
        let command = Schema::starter_command(DEFAULT_SLUG);
        let parsed: Schema = atoms::extract_fenced(&command, FENCE_INFO)
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
