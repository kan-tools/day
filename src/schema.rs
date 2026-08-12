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
    ///
    /// **Skipped when empty, which is a compatibility requirement rather than
    /// tidiness.** `Schema` is `deny_unknown_fields`, so a serialized
    /// `"paths_external": []` is a hard error on every earlier day — and the
    /// starter day prints is what a project records, so day's own suggestion
    /// would have made new projects unreadable to the release before this one.
    /// Measured against the real `v0.12.0-beta.1` binary:
    ///
    /// ```text
    /// error: schema/design-doc: `day-schema` block could not be read:
    ///        unknown field `paths_external`, expected one of `sections`, …
    /// exit 2
    /// ```
    ///
    /// Skipping when empty keeps a schema that does not use the feature
    /// **byte-identical** to what earlier days wrote — the same property
    /// `day-atom` holds for `revisits`, and for the same reason.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths_external: Vec<String>,
}

/// The reader version a `day-schema` block requires once it declares
/// `paths_external` (day#84).
///
/// Stamped **only** when the field is non-empty, so a schema that does not use
/// it stays v1 and readable by every earlier day. This is `day-atom`'s
/// `revisits` rule applied to the second block type that needed it: the version
/// is what turns an older day's refusal from `unknown field paths_external` —
/// which reads as the project's mistake — into "this day reads `day-schema` v1,
/// this block declares v2, upgrade day".
pub const SCHEMA_VERSION_PATHS_EXTERNAL: u64 = 2;

impl crate::atoms::Versioned for Schema {
    /// A design-doc schema. v1 is every block written before versioning
    /// existed, which an absent `_version` still means; v2 adds
    /// `paths_external`.
    const SUPPORTED_VERSION: u64 = SCHEMA_VERSION_PATHS_EXTERNAL;
    const FENCE: &'static str = FENCE_INFO;
}

impl Schema {
    /// The block body, version-stamped only when it needs to be.
    ///
    /// Spliced onto the serialized struct rather than rebuilt through a
    /// `serde_json::Value`, for the reason `Interface::to_block_json` states:
    /// serde_json's map re-sorts keys, so a round-trip would change the bytes
    /// of blocks this feature is supposed to leave alone. Splicing keeps "the
    /// stamp is the only difference" an assertable property rather than a hope.
    pub fn to_block_json(&self) -> String {
        self.stamp(
            serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string()),
            "",
        )
    }

    /// The same body, pretty-printed — what `day init` records and what the
    /// starter command prints. Both go through here so the printed and recorded
    /// forms cannot disagree about the stamp, which is the property
    /// [`Self::record`]'s own doc comment already claims for the schema itself.
    pub fn to_block_json_pretty(&self) -> String {
        self.stamp(
            serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string()),
            "\n ",
        )
    }

    fn stamp(&self, body: String, sep: &str) -> String {
        if self.paths_external.is_empty() {
            return body;
        }
        match body.strip_prefix('{') {
            Some(rest) => {
                format!("{{{sep}\"_version\": {SCHEMA_VERSION_PATHS_EXTERNAL},{rest}")
            }
            None => body,
        }
    }
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
        let json = Self::starter().to_block_json_pretty();
        format!(
            "  kan observe \"$(cat <<'EOF'\nDesign-doc schema for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{slug}"
        )
    }

    /// Loads the live schema for `slug` from kan.
    pub fn load(client: &KanClient, slug: &str) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{slug}");
        // not-per-key: a design-doc schema has no shipped default either;
        // see `src/docs.rs`. Absence is `NotDeclared`, and redeclaring
        // replaces.
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
        // not-per-key: an existence check over the same declaration.
        Ok(newest_fenced::<Self>(client, &subject)?.is_some())
    }

    /// Records this schema as a claim. Used by `day init` so a fresh repo
    /// reaches a working `day design check` with a command rather than a
    /// copy-paste — the starter has one definition either way, so the
    /// printed and recorded forms cannot disagree.
    pub fn record(&self, client: &KanClient, slug: &str) -> Result<String, Error> {
        let json = self.to_block_json_pretty();
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

    /// **day#84's field must not break every earlier day.**
    ///
    /// `Schema` is `deny_unknown_fields`, so a serialized `"paths_external": []`
    /// is a hard error on any day that predates the field — and the starter is
    /// what `day init` RECORDS, so day's own suggestion would have made new
    /// projects unreadable to the previous release. Verified against the real
    /// `v0.12.0-beta.1` binary, which went from exit 2 (`unknown field`) to
    /// exit 1 (the document's own honest verdict) once this held.
    #[test]
    fn a_schema_not_using_an_external_root_is_byte_identical_to_the_old_form() {
        let starter = Schema::starter();

        // premise: the starter really does leave the field empty. If a later
        // edit ships a default root, this test would otherwise quietly start
        // asserting nothing about the compatibility path.
        assert!(
            starter.paths_external.is_empty(),
            "premise: the starter must declare no external roots"
        );

        for rendered in [starter.to_block_json(), starter.to_block_json_pretty()] {
            assert!(
                !rendered.contains("paths_external"),
                "an unused field must not appear at all — `deny_unknown_fields` \
                 makes its mere presence a hard error on an older day: {rendered}"
            );
            assert!(
                !rendered.contains("_version"),
                "and an unstamped block stays v1, readable by every earlier day: \
                 {rendered}"
            );
        }
    }

    /// The other half: a schema that DOES use the field stamps the reader
    /// version, so an older day refuses with "upgrade day" rather than with
    /// `unknown field paths_external`, which reads as the project's mistake.
    /// day#60's lesson, applied to the second block type that needed it.
    #[test]
    fn a_schema_using_an_external_root_declares_the_version_it_requires() {
        let mut schema = Schema::starter();
        schema.paths_external = vec!["kan/".to_string()];

        for rendered in [schema.to_block_json(), schema.to_block_json_pretty()] {
            assert!(
                rendered.contains("\"_version\""),
                "a block using v2 must say so: {rendered}"
            );
            assert!(
                rendered.contains("paths_external"),
                "and must still carry the field it stamped for: {rendered}"
            );
            // The stamp must not corrupt the body it is spliced onto.
            let parsed: Schema =
                crate::atoms::extract_fenced(&format!("x\n\n```{FENCE_INFO}\n{rendered}\n```\n"))
                    .expect("the stamped block is present")
                    .expect("and parses");
            assert_eq!(
                parsed, schema,
                "the stamp is the only difference; the schema round-trips"
            );
        }
    }

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
