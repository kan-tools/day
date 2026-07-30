//! Project-declared block schemas — a vocabulary day validates but did not
//! invent.
//!
//! day owns seven fenced block types and, until this, a project could invent
//! none. A research program tried to instantiate day's process for a
//! non-software domain and needed exactly one more — `research-claim`, carrying
//! a claim's evidential station — and had nowhere to put it (day#74).
//!
//! A project records a `day-blocks` block on `schema/blocks` naming its block
//! types and their fields, and day then validates those blocks **under the same
//! contract as its own**: refused when they violate the declaration, version-
//! gated through [`crate::atoms::version_gate`], and reported as unreadable on
//! both hook channels. Leaving the declarable path tolerant while day's own
//! seven are strict would be day#78's inconsistency reintroduced at the moment
//! the surface widens.
//!
//! **day's own seven stay struct-defined, and that is deliberate.** day#74 asked
//! for `day-atom` to become the first instance of this mechanism — "one less
//! special case". It is declined: a declaration beside a Rust struct is two
//! sources of truth with no compiler between them, which is exactly the
//! `extract_fenced` defect `v0.7.0-beta.2`'s own review found, where a `fence`
//! parameter and `T::FENCE` could disagree. One mechanism for what day writes,
//! one for what a project invents, and [`RESERVED_FENCES`] keeps them from
//! meeting.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::atoms::{self, BlockError, Versioned};
use crate::kan_client::KanClient;
use crate::schema::SCHEMA_PREFIX;

/// Subject slug day looks for: `schema/blocks`.
pub const BLOCKS_SLUG: &str = "blocks";
/// Fence info string marking a block-schema map inside a claim's text.
pub const FENCE_INFO: &str = "day-blocks";

/// Fence names a project may **not** declare, because day already reads them
/// with a Rust struct as their schema.
///
/// Without this, a project could declare `day-atom` and day would have two
/// readers for one fence with no rule for which wins. `v0.7.0-beta.2` removed
/// exactly that ambiguity from `extract_fenced` by deleting its `fence`
/// parameter; reintroducing it one level up would undo the fix.
pub const RESERVED_FENCES: &[&str] = &[
    atoms::FENCE_INFO,
    crate::bridge::FENCE_INFO,
    crate::bridge::TELOS_FENCE,
    crate::telos::FENCE_INFO,
    crate::schema::FENCE_INFO,
    crate::docs::FENCE_INFO,
    crate::tension::FENCE_INFO,
    FENCE_INFO,
];

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
}

/// Which fields a declared block must carry, and which it may.
///
/// **Names and required/optional only — no value types.** day#34 established
/// that a schema arriving from a claim should not bring a pattern language, and
/// day#70 held the same line for `subject`. Value types would let day catch
/// `medium: 7` where a station name was meant, which is the research loop's
/// highest-value rule — but a type language is a language, and every addition to
/// it is a decision day then owns for every project. day validates the *shape* a
/// project declared; the project's own tooling interprets the values.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSpec {
    /// Fields a block of this type must carry.
    #[serde(default)]
    pub required: Vec<String>,
    /// Fields it may carry. Anything named in neither is refused.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional: Vec<String>,
}

impl FieldSpec {
    /// Whether a block body satisfies this spec.
    ///
    /// Returns the reason on failure, phrased for the person who wrote the
    /// claim — this is the claim's problem, not the reader's, so it never
    /// suggests upgrading day.
    pub fn check(&self, name: &str, body: &serde_json::Value) -> Result<(), String> {
        let Some(object) = body.as_object() else {
            return Err(format!(
                "a `{name}` block must be a JSON object, found {}",
                shape_of(body)
            ));
        };

        let missing: Vec<&str> = self
            .required
            .iter()
            .filter(|f| !object.contains_key(f.as_str()))
            .map(String::as_str)
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "`{name}` requires {} that {} missing: {}",
                if missing.len() == 1 {
                    "a field"
                } else {
                    "fields"
                },
                if missing.len() == 1 { "is" } else { "are" },
                missing.join(", ")
            ));
        }

        // Undeclared fields are refused, for the same reason day's own blocks
        // refuse them since `v0.7.0-beta.2`: a project declared what this block
        // means, and silently ignoring part of an instance certifies a shape day
        // only partly read.
        let undeclared: Vec<&str> = object
            .keys()
            .filter(|k| !self.required.iter().any(|f| f == *k))
            .filter(|k| !self.optional.iter().any(|f| f == *k))
            .map(String::as_str)
            .collect();
        if !undeclared.is_empty() {
            let mut declared: Vec<&str> = self
                .required
                .iter()
                .chain(self.optional.iter())
                .map(String::as_str)
                .collect();
            declared.sort_unstable();
            return Err(format!(
                "`{name}` does not declare {}; declared fields are: {}",
                undeclared.join(", "),
                if declared.is_empty() {
                    "none".to_string()
                } else {
                    declared.join(", ")
                }
            ));
        }
        Ok(())
    }
}

fn shape_of(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "a boolean",
        serde_json::Value::Number(_) => "a number",
        serde_json::Value::String(_) => "a string",
        serde_json::Value::Array(_) => "an array",
        serde_json::Value::Object(_) => "an object",
    }
}

/// The block types a project declared, by name.
///
/// `transparent` over the map for the same reason [`crate::telos::WitnessSchema`]
/// is: every key is *data* — a block type this project invented — so refusing
/// unknown keys would refuse the project's own vocabulary. The strictness lives
/// one level down, on [`FieldSpec`], which does refuse a field it does not
/// declare.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct BlockSchemas {
    pub blocks: BTreeMap<String, FieldSpec>,
    /// Declarations this build could not read, with the reason. Reported, never
    /// silently dropped — the rule `WitnessSchema` established after a `claim`
    /// probe took down the installed v0.6 binary's whole witness map.
    #[serde(skip)]
    pub unsupported: BTreeMap<String, String>,
}

impl<'de> Deserialize<'de> for BlockSchemas {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Parsed one entry at a time, so a spec this day cannot read costs that
        // block type and nothing else.
        let raw = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut blocks = BTreeMap::new();
        let mut unsupported = BTreeMap::new();
        for (name, value) in raw {
            match serde_json::from_value::<FieldSpec>(value) {
                Ok(spec) => {
                    blocks.insert(name, spec);
                }
                Err(e) => {
                    unsupported.insert(name, e.to_string());
                }
            }
        }
        Ok(Self {
            blocks,
            unsupported,
        })
    }
}

impl Versioned for BlockSchemas {
    const SUPPORTED_VERSION: u64 = atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;

    /// A project may not declare a name day already reads with a struct.
    ///
    /// Refused at parse, through the same [`BlockError::Invalid`] path day#20's
    /// empty plan nodes use — so it is reported like any other unreadable
    /// declaration rather than needing its own surface.
    fn validate(&self) -> Result<(), String> {
        let clashes: Vec<&str> = self
            .blocks
            .keys()
            .filter(|name| RESERVED_FENCES.contains(&name.as_str()))
            .map(String::as_str)
            .collect();
        if clashes.is_empty() {
            return Ok(());
        }
        Err(format!(
            "{} {} reserved: day reads {} with a built-in schema, so declaring {} \
             here would give one fence two readers with no rule for which wins",
            clashes.join(", "),
            if clashes.len() == 1 { "is" } else { "are" },
            if clashes.len() == 1 { "it" } else { "them" },
            if clashes.len() == 1 { "it" } else { "them" },
        ))
    }
}

impl BlockSchemas {
    /// What day suggests when a project has none. Offered, never applied — the
    /// same contract every other `starter` here holds.
    ///
    /// The example is the research loop's `research-claim`, because a starter
    /// showing a real declared vocabulary teaches the mechanism better than a
    /// placeholder would.
    pub fn starter() -> Self {
        let mut blocks = BTreeMap::new();
        blocks.insert(
            "research-claim".to_string(),
            FieldSpec {
                required: vec![
                    "medium".to_string(),
                    "scope_coords".to_string(),
                    "anchor_ref".to_string(),
                ],
                optional: vec!["decay_note".to_string(), "situated_verdict".to_string()],
            },
        );
        Self {
            blocks,
            unsupported: BTreeMap::new(),
        }
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::starter()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nBlock schemas for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{BLOCKS_SLUG}"
        )
    }

    /// Reads the project's declarations, or an empty set when none is recorded.
    ///
    /// Absent is not an error: a project that declares no block types of its own
    /// is the common case, and day's built-ins are unaffected either way.
    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{BLOCKS_SLUG}");
        Ok(atoms::newest_fenced::<Self>(client, &subject)?
            .map(|(_cid, schemas)| schemas)
            .unwrap_or_default())
    }

    /// Pulls a declared block of the named type out of a claim's text and
    /// validates it against what the project declared.
    ///
    /// `None` when the claim carries no such block. The version gate is
    /// [`atoms::version_gate`] — the same one day's own blocks use — so a
    /// project can version its own vocabulary and an older day says "this day
    /// reads `research-claim` v1, this block declares v2" rather than reporting
    /// the claim as malformed.
    pub fn extract<'a>(
        &'a self,
        text: &str,
        name: &'a str,
    ) -> Option<Result<serde_json::Value, BlockError>> {
        let spec = self.blocks.get(name)?;
        // Leaked deliberately: `BlockError` carries `&'static str` because
        // day's own fences are constants, and a declared name is not. The leak
        // is bounded by the number of distinct declared block types a process
        // reads, which is the size of this map.
        let fence: &'static str = Box::leak(name.to_string().into_boxed_str());

        let open = format!("```{name}");
        let start = text.find(&open)? + open.len();
        let rest = &text[start..];
        let end = rest.find("```")?;
        let body = rest[..end].trim();

        Some(
            atoms::version_gate(body, fence, Self::SUPPORTED_VERSION).and_then(|value| {
                spec.check(name, &value)
                    .map_err(|reason| BlockError::Invalid { fence, reason })
                    .map(|()| value)
            }),
        )
    }
}

/// Subject slug for declared injection settings: `schema/injection`.
pub const INJECTION_SLUG: &str = "injection";
/// Fence info string marking injection settings inside a claim's text.
pub const INJECTION_FENCE: &str = "day-injection";

/// How day paces what it injects, declared per project.
///
/// `v0.7.0-beta.2` shipped the cadence as a constant with day#82 filed to measure
/// it against recall rather than intuition. This makes it declared, which is what
/// `docs/ROADMAP.md` promised beta.3 would do — and it lives here rather than on
/// `practice` because `practice` is prose projected into a model's context and
/// this is typed config; putting the first structured value there would make
/// `practice` two things.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InjectionSchema {
    /// User prompts between re-displays of a standing condition. `0` disables
    /// the periodic channel entirely, which is a legitimate choice — day#30's
    /// evidence is that an always-present rule becomes background, and a project
    /// may reasonably decide the risk is not worth the reminder.
    #[serde(default = "default_cadence")]
    pub cadence: u32,
}

fn default_cadence() -> u32 {
    crate::cache::DEFAULT_CADENCE
}

impl Default for InjectionSchema {
    fn default() -> Self {
        Self {
            cadence: default_cadence(),
        }
    }
}

impl Versioned for InjectionSchema {
    const SUPPORTED_VERSION: u64 = atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = INJECTION_FENCE;
}

impl InjectionSchema {
    /// Reads the project's declaration, or day's default when none is recorded.
    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{INJECTION_SLUG}");
        Ok(atoms::newest_fenced::<Self>(client, &subject)?
            .map(|(_cid, schema)| schema)
            .unwrap_or_default())
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::default()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nInjection settings for this project.\n\n\
             ```{INJECTION_FENCE}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{INJECTION_SLUG}"
        )
    }
}

/// Subject slug for a declared cycle boundary: `schema/cycle`.
pub const CYCLE_SLUG: &str = "cycle";
/// Fence info string marking a cycle declaration inside a claim's text.
pub const CYCLE_FENCE: &str = "day-cycle";

/// The default tag pattern day treats as a cycle boundary when none is
/// declared: a release.
pub const DEFAULT_BOUNDARY_TAGS: &str = "v*";

/// What ends a cycle, declared per project (day#76).
///
/// Position inference asks "since when", and day#60 answered it with *the last
/// release* — which is right for software and wrong for everything else. A
/// research program's cycle is the **pass**, a paper's is a freeze, a review's is
/// an arc boundary. day#76 put the general point well: the insight that "does an
/// artifact of this type exist" needs bounding is process-generic, and only the
/// *default binding* to releases is software-specific.
///
/// So this declares the binding. Absent, day keeps release semantics, which is
/// what every existing project already relies on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CycleSchema {
    /// Git tag glob whose newest match ends a cycle. `pass/*` for the research
    /// loop, `v*` (the default) for a project whose cycle is a release.
    #[serde(default = "default_boundary_tags")]
    pub tags: String,
}

fn default_boundary_tags() -> String {
    DEFAULT_BOUNDARY_TAGS.to_string()
}

impl Default for CycleSchema {
    fn default() -> Self {
        Self {
            tags: default_boundary_tags(),
        }
    }
}

impl Versioned for CycleSchema {
    const SUPPORTED_VERSION: u64 = atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = CYCLE_FENCE;

    /// An empty pattern matches no tag, which is not "every cycle" — it is a
    /// project silently losing its boundary and falling back to the cumulative
    /// reading day#60 exists to prevent. Refused rather than tolerated, because
    /// the failure would look exactly like working.
    fn validate(&self) -> Result<(), String> {
        if self.tags.trim().is_empty() {
            return Err(
                "an empty tag pattern matches nothing, so position would silently \
                 fall back to the cumulative reading day#60 replaced — declare a \
                 pattern, or omit the declaration to keep release semantics"
                    .to_string(),
            );
        }
        Ok(())
    }
}

impl CycleSchema {
    /// Reads the project's declaration, or release semantics when none is
    /// recorded.
    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{CYCLE_SLUG}");
        Ok(atoms::newest_fenced::<Self>(client, &subject)?
            .map(|(_cid, c)| c)
            .unwrap_or_default())
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::default()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nWhat ends a cycle in this project.\n\n\
             ```{CYCLE_FENCE}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{CYCLE_SLUG}"
        )
    }
}

#[cfg(test)]
mod tests {
    /// day#76: the boundary binding is declared, and absent it is a release.
    ///
    /// The insight day#60 encoded — that "does an artifact of this type exist" is
    /// always-yes on a repo with history and needs bounding — is process-generic;
    /// only the binding to `v*` was software-specific.
    #[test]
    fn a_declared_cycle_replaces_the_release_binding() {
        let declared = parse_block::<CycleSchema>(r#"{"tags":"pass/*"}"#).unwrap();
        assert_eq!(declared.tags, "pass/*");

        // Absent means release semantics, so every existing project is unaffected.
        assert_eq!(CycleSchema::default().tags, DEFAULT_BOUNDARY_TAGS);
        assert_eq!(
            parse_block::<CycleSchema>(r#"{}"#).unwrap().tags,
            DEFAULT_BOUNDARY_TAGS
        );
    }

    /// An empty pattern matches no tag — which is not "every cycle", it is a
    /// project silently losing its boundary and reverting to the cumulative
    /// reading day#60 replaced. Refused, because the failure would look exactly
    /// like working.
    #[test]
    fn an_empty_cycle_pattern_is_refused() {
        for empty in [r#"{"tags":""}"#, r#"{"tags":"   "}"#] {
            let e = parse_block::<CycleSchema>(empty).unwrap_err();
            assert!(matches!(e, BlockError::Invalid { .. }), "{empty}: {e:?}");
            assert!(!e.is_version_skew(), "the project's to fix: {e}");
        }
        // Negative control.
        assert!(parse_block::<CycleSchema>(r#"{"tags":"pass/*"}"#).is_ok());
    }

    /// day#77: a project moves WHICH closed set from code to a claim, without
    /// gaining free text. The closedness is the property both vocabularies exist
    /// to preserve.
    #[test]
    fn a_declared_vocabulary_replaces_days_four_without_opening_them_up() {
        let loop_vocab: VerdictVocabulary = serde_json::from_str(
            r#"{"verdicts":["NOVEL-AS-SEARCHED","REDISCOVERED+\u0394","SPECIALIZATION","COLLISION","SUBSUMED"]}"#,
        )
        .unwrap();

        assert!(loop_vocab.permits(&normalize("novel-as-searched")));
        assert!(loop_vocab.permits(&normalize("Novel As Searched")));
        // day's own four are NOT permitted here, which is the point: a declared
        // vocabulary replaces the set rather than extending it.
        assert!(!loop_vocab.permits(&normalize("APPROVE")));
        // And nothing outside it is.
        assert!(!loop_vocab.permits(&normalize("looks good to me")));
    }

    /// The default is day's four, so a project that declares nothing is
    /// unaffected — the backward-compatibility half of day#77.
    #[test]
    fn the_default_vocabulary_is_days_own_four() {
        let d = VerdictVocabulary::default();
        for v in crate::record::DEFAULT_VERDICTS {
            assert!(d.permits(v), "{v} should be permitted by default");
        }
        assert_eq!(d.verdicts.len(), 4);
    }

    /// An empty vocabulary accepts nothing, which locks a project out of its own
    /// review verb. Refused at parse, like day#20's empty plan nodes — a
    /// declaration that cannot mean anything is not a declaration.
    #[test]
    fn an_empty_or_duplicated_vocabulary_is_refused() {
        let empty = parse_block::<VerdictVocabulary>(r#"{"verdicts":[]}"#).unwrap_err();
        assert!(matches!(empty, BlockError::Invalid { .. }), "{empty:?}");
        assert!(!empty.is_version_skew(), "the project's to fix: {empty}");

        let dupe = parse_block::<VerdictVocabulary>(r#"{"verdicts":["A","B","A"]}"#).unwrap_err();
        assert!(matches!(dupe, BlockError::Invalid { .. }), "{dupe:?}");
        assert!(
            dupe.to_string().contains('A'),
            "names the duplicate: {dupe}"
        );

        // Negative control: a normal vocabulary parses.
        assert!(parse_block::<VerdictVocabulary>(r#"{"verdicts":["A","B"]}"#).is_ok());
    }

    /// Normalization is one function, shared by the declaration and the
    /// recorder. Two would be a second source of truth for what a verdict *is* —
    /// the divergence class this milestone keeps finding.
    #[test]
    fn normalization_is_shared_so_declaration_and_argument_cannot_disagree() {
        assert_eq!(normalize("  novel as searched "), "NOVEL-AS-SEARCHED");
        let v: VerdictVocabulary =
            serde_json::from_str(r#"{"verdicts":["Novel As Searched"]}"#).unwrap();
        assert!(
            v.permits(&normalize("NOVEL-AS-SEARCHED")),
            "a declaration written in prose must match the argument form"
        );
    }

    use super::*;
    use crate::atoms::parse_block;

    fn spec(required: &[&str], optional: &[&str]) -> FieldSpec {
        FieldSpec {
            required: required.iter().map(|s| s.to_string()).collect(),
            optional: optional.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn body(json: &str) -> serde_json::Value {
        serde_json::from_str(json).unwrap()
    }

    /// AC-1: required fields are required, optional ones are not.
    #[test]
    fn a_declared_spec_requires_what_it_declares_required() {
        let s = spec(&["medium", "scope_coords"], &["decay_note"]);

        assert!(s
            .check(
                "research-claim",
                &body(r#"{"medium":"anchor-verified","scope_coords":"x"}"#)
            )
            .is_ok());
        // Optional present is fine.
        assert!(s
            .check(
                "research-claim",
                &body(r#"{"medium":"a","scope_coords":"x","decay_note":"n"}"#)
            )
            .is_ok());

        let e = s
            .check("research-claim", &body(r#"{"medium":"a"}"#))
            .unwrap_err();
        assert!(e.contains("scope_coords"), "names the missing field: {e}");
        assert!(
            !e.contains("upgrade"),
            "the claim's problem, not the reader's: {e}"
        );
    }

    /// AC-2: a field the declaration does not name is refused — and the refusal
    /// tracks the *declaration*, not a fixed list.
    ///
    /// The negative control is what makes that true rather than asserted: the
    /// same block against a spec that does declare the field resolves.
    #[test]
    fn an_undeclared_field_is_refused_and_tracks_the_declaration() {
        let narrow = spec(&["medium"], &[]);
        let e = narrow
            .check(
                "research-claim",
                &body(r#"{"medium":"a","anchor_ref":"r"}"#),
            )
            .unwrap_err();
        assert!(e.contains("anchor_ref"), "names the field it refused: {e}");
        assert!(e.contains("medium"), "and says what IS declared: {e}");

        // Negative control: declare it, and the identical block resolves.
        let wide = spec(&["medium"], &["anchor_ref"]);
        assert!(wide
            .check(
                "research-claim",
                &body(r#"{"medium":"a","anchor_ref":"r"}"#)
            )
            .is_ok());
    }

    /// A block body that is not an object at all. Reported as the claim's
    /// problem with its actual shape named, rather than as a missing field.
    #[test]
    fn a_non_object_body_is_refused_by_shape() {
        let s = spec(&["medium"], &[]);
        let e = s
            .check("research-claim", &body(r#"["medium"]"#))
            .unwrap_err();
        assert!(e.contains("an array"), "{e}");
    }

    /// AC-5: a project may not declare a name day already reads with a struct.
    ///
    /// Refused at parse through `Versioned::validate`, so it arrives as a
    /// `BlockError::Invalid` and is reported like any other unreadable
    /// declaration — no separate surface.
    #[test]
    fn a_reserved_fence_cannot_be_declared() {
        for reserved in RESERVED_FENCES {
            let json = format!(r#"{{"{reserved}":{{"required":["x"]}}}}"#);
            let e = parse_block::<BlockSchemas>(&json).unwrap_err();
            assert!(
                matches!(e, BlockError::Invalid { .. }),
                "{reserved} should be refused as invalid: {e:?}"
            );
            assert!(
                e.to_string().contains(reserved),
                "the refusal should name the reserved fence: {e}"
            );
            assert!(
                !e.is_version_skew(),
                "reserved is the project's to fix: {e}"
            );
        }
    }

    /// The negative control for the above: a name day does *not* own declares
    /// fine. Without this, the test would pass if every declaration were refused.
    #[test]
    fn a_name_day_does_not_own_declares_fine() {
        let parsed =
            parse_block::<BlockSchemas>(r#"{"research-claim":{"required":["medium"]}}"#).unwrap();
        assert!(parsed.blocks.contains_key("research-claim"));
        assert!(parsed.unsupported.is_empty());
    }

    /// AC-3: a declared block inherits the version gate, and version skew is
    /// distinguishable from a field-spec violation.
    ///
    /// This is what sharing `atoms::version_gate` buys: a project's own
    /// vocabulary gets "upgrade day" for the one and "fix the claim" for the
    /// other, without either message being written twice.
    #[test]
    fn a_declared_block_is_version_gated_like_a_built_in() {
        let schemas =
            parse_block::<BlockSchemas>(r#"{"research-claim":{"required":["medium"]}}"#).unwrap();

        let too_new = schemas
            .extract(
                "x\n\n```research-claim\n{\"_version\":2,\"medium\":\"a\"}\n```\n",
                "research-claim",
            )
            .expect("the block is present")
            .unwrap_err();
        assert!(too_new.is_version_skew(), "{too_new}");
        assert!(too_new.to_string().contains("upgrade day"), "{too_new}");

        let bad_field = schemas
            .extract(
                "x\n\n```research-claim\n{\"medium\":\"a\",\"nope\":1}\n```\n",
                "research-claim",
            )
            .expect("the block is present")
            .unwrap_err();
        assert!(!bad_field.is_version_skew(), "{bad_field}");
        assert!(
            !bad_field.to_string().contains("upgrade day"),
            "a spec violation must not tell the reader to upgrade: {bad_field}"
        );

        // And a valid one resolves, so the assertions above are about the gate
        // rather than about extraction failing.
        let ok = schemas
            .extract(
                "x\n\n```research-claim\n{\"_version\":1,\"medium\":\"a\"}\n```\n",
                "research-claim",
            )
            .expect("present")
            .expect("valid");
        assert_eq!(ok["medium"], "a");
        assert!(
            ok.get(crate::atoms::VERSION_KEY).is_none(),
            "the version key must be stripped, not left in the body"
        );
    }

    /// An undeclared block type is simply not this project's vocabulary — `None`,
    /// not an error. day validates what a project declared; it does not object to
    /// text it was never told about.
    #[test]
    fn an_undeclared_block_type_is_not_an_error() {
        let schemas =
            parse_block::<BlockSchemas>(r#"{"research-claim":{"required":["medium"]}}"#).unwrap();
        assert!(schemas
            .extract("x\n\n```something-else\n{}\n```\n", "something-else")
            .is_none());
    }

    /// A spec this build cannot read costs that block type and nothing else,
    /// and is reported rather than dropped — the rule `WitnessSchema`
    /// established after an unreadable probe took down a whole witness map.
    #[test]
    fn an_unreadable_spec_costs_only_itself_and_is_reported() {
        let parsed =
            parse_block::<BlockSchemas>(r#"{"good":{"required":["a"]},"bad":{"requiredd":["a"]}}"#)
                .unwrap();
        assert!(parsed.blocks.contains_key("good"));
        assert!(!parsed.blocks.contains_key("bad"));
        assert!(
            parsed.unsupported.contains_key("bad"),
            "an unreadable spec must be reported: {parsed:?}"
        );
    }

    /// AC-6's unit half: the declared cadence round-trips, and `0` is a
    /// legitimate declaration meaning "never re-display".
    #[test]
    fn the_injection_cadence_is_declarable() {
        let declared = parse_block::<InjectionSchema>(r#"{"cadence":25}"#).unwrap();
        assert_eq!(declared.cadence, 25);
        assert_eq!(
            parse_block::<InjectionSchema>(r#"{"cadence":0}"#)
                .unwrap()
                .cadence,
            0
        );
        // Absent means day's default, not zero.
        assert_eq!(
            parse_block::<InjectionSchema>(r#"{}"#).unwrap().cadence,
            crate::cache::DEFAULT_CADENCE
        );
        // And an undeclared field is refused, like every other struct-shaped block.
        assert!(parse_block::<InjectionSchema>(r#"{"cadance":10}"#).is_err());
    }
}

/// Subject slug for declared review-verdict vocabularies: `schema/verdicts`.
pub const VERDICTS_SLUG: &str = "verdicts";
/// Fence info string marking a verdict vocabulary inside a claim's text.
pub const VERDICTS_FENCE: &str = "day-verdicts";

/// The closed set of verdicts `day review record` accepts, declared per project.
///
/// **The value of a closed set is that it forces adjudication**, and that
/// survives being declared: a project moves *which* set from code to a claim
/// without gaining free text. day#77 put it well — free text is the thing both
/// vocabularies exist to prevent.
///
/// day's own four are the default, so nothing changes for a project that
/// declares nothing. The research loop's positioning reviews need a different
/// fixed set with the same discipline
/// (`NOVEL-AS-SEARCHED` / `REDISCOVERED+Δ` / `SPECIALIZATION` / `COLLISION` /
/// `SUBSUMED`), which is what this is for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerdictVocabulary {
    /// The permitted verdicts, in the order a reader should see them.
    pub verdicts: Vec<String>,
}

impl Default for VerdictVocabulary {
    fn default() -> Self {
        Self {
            verdicts: crate::record::DEFAULT_VERDICTS
                .iter()
                .map(|v| v.to_string())
                .collect(),
        }
    }
}

impl Versioned for VerdictVocabulary {
    const SUPPORTED_VERSION: u64 = atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = VERDICTS_FENCE;

    /// An empty vocabulary would accept nothing, which is not a vocabulary — it
    /// is a project locking itself out of its own review verb. Refused for the
    /// same reason day#20's empty plan nodes are: a declaration that cannot mean
    /// anything is not a declaration.
    fn validate(&self) -> Result<(), String> {
        if self.verdicts.is_empty() {
            return Err(
                "a verdict vocabulary must permit at least one verdict — an empty set \
                 accepts nothing, which locks the project out of `day review record`"
                    .to_string(),
            );
        }
        let mut seen = std::collections::BTreeSet::new();
        let dupes: Vec<&str> = self
            .verdicts
            .iter()
            .filter(|v| !seen.insert(v.as_str()))
            .map(String::as_str)
            .collect();
        if !dupes.is_empty() {
            return Err(format!(
                "declared twice: {}. A verdict set is a set — a duplicate means one \
                 of the two was meant to be something else",
                dupes.join(", ")
            ));
        }
        Ok(())
    }
}

impl VerdictVocabulary {
    /// Reads the project's declaration, or day's four when none is recorded.
    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{VERDICTS_SLUG}");
        Ok(atoms::newest_fenced::<Self>(client, &subject)?
            .map(|(_cid, v)| v)
            .unwrap_or_default())
    }

    /// Whether a verdict is permitted, comparing in the normalized form
    /// `day review record` writes.
    pub fn permits(&self, normalized: &str) -> bool {
        self.verdicts.iter().any(|v| normalize(v) == normalized)
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::default()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nReview verdict vocabulary for this project.\n\n\
             ```{VERDICTS_FENCE}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{VERDICTS_SLUG}"
        )
    }
}

/// The form a verdict is stored in: upper-cased, spaces to hyphens.
///
/// Shared by the declaration and the recorder so a project can declare
/// `Novel As Searched` and have `--verdict novel-as-searched` match it. One
/// function, because two would be a second source of truth for what a verdict
/// *is* — the divergence class this milestone keeps finding.
pub fn normalize(verdict: &str) -> String {
    verdict.trim().to_uppercase().replace(' ', "-")
}
