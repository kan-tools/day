//! Atoms: the composable process units of the telos-driven model
//! (`docs/TELOS.md`), and the composition check `day doctor` runs over them.
//!
//! Per `docs/CONVENTIONS.md`, an atom lives entirely in kan as claims on an
//! `atom/<slug>` subject, and its interface is a fenced `day-atom` JSON
//! block inside the claim text. The vocabulary is **per-atom additive**:
//! each atom is its own subject, "the current vocabulary" is just the live
//! non-retracted fold, and the newest interface-bearing claim on a subject
//! is that atom's current interface. day never mutates or deletes anything
//! to revise an atom — a newer claim supersedes an older one, kan's own
//! append/fold/retract pattern rather than a second versioning mechanic.

use serde::{Deserialize, Serialize};

use crate::kan_client::{self, KanClient};

/// Subject-name prefix for atom declarations.
pub const ATOM_PREFIX: &str = "atom/";
/// Subject-name prefix for telos declarations.
pub const TELOS_PREFIX: &str = "telos/";
/// Fence info string marking an interface block inside a claim's text.
pub const FENCE_INFO: &str = "day-atom";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] kan_client::Error),
    #[error("{subject}: {source} (claim {cid})")]
    Block {
        subject: String,
        cid: String,
        #[source]
        source: BlockError,
    },
}

/// The metadata key naming the reader version a block requires.
///
/// Underscore-prefixed to mark it as metadata rather than declared content: a
/// project's own block could legitimately want a field called `v` or
/// `version`, and this must never collide with one. Deliberately not
/// `$`-prefixed, because declared block schemas (day#74) may end up expressed
/// as JSON Schema, where `$` names are reserved.
pub const VERSION_KEY: &str = "_version";

/// The version an absent [`VERSION_KEY`] means. Every block written before
/// versioning existed is a v1 block, so absence has to be the first version
/// rather than an error.
pub const IMPLICIT_VERSION: u64 = 1;

/// The block version a type understands, declared **per block type** so a
/// reader fails only on the block that actually changed rather than on the
/// whole vocabulary.
///
/// This is the honest half of refusing unknown fields. `deny_unknown_fields`
/// *detects* that a block says more than this day can read; the version is what
/// lets the message say **why** — "this day reads `day-atom` v1, this block
/// declares v2, upgrade day" rather than a parse error that reads as the
/// project's mistake. day#60's lesson was that the v0.6 binary failed loudly
/// and misdirected the reader; detection without an actionable message repeats
/// it.
pub trait Versioned {
    /// The highest version of this block type this build can read.
    const SUPPORTED_VERSION: u64;
    /// The fence info string this block is declared under, for diagnostics.
    const FENCE: &'static str;
}

/// Why a fenced block could not be read into its type.
#[derive(Debug, thiserror::Error)]
pub enum BlockError {
    /// The block declares a version newer than this build reads. **Not the
    /// project's mistake** — the reader is behind, and the message says so.
    #[error(
        "`{fence}` block declares {VERSION_KEY} {declared}, but this day reads \
         up to {supported} — upgrade day to read it"
    )]
    TooNew {
        fence: &'static str,
        declared: u64,
        supported: u64,
    },
    /// The block is malformed at a version this build does read — invalid
    /// JSON, or a field the block type does not declare. **This one is the
    /// claim's problem**, and the message points at the claim.
    #[error("`{fence}` block could not be read: {source}")]
    Malformed {
        fence: &'static str,
        #[source]
        source: serde_json::Error,
    },
}

impl BlockError {
    /// Whether the reader is behind the log, rather than the log being wrong.
    /// The two need different actions from different people, which is the
    /// whole reason they are distinct variants.
    pub fn is_version_skew(&self) -> bool {
        matches!(self, BlockError::TooNew { .. })
    }
}

/// An atom's declared interface. `inputs`/`outputs` are free-form type
/// names — day checks that they *match*, deliberately not what they mean;
/// the vocabulary of type names is the project's to choose and evolve.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    #[serde(rename = "in", default)]
    pub inputs: Vec<String>,
    #[serde(rename = "out", default)]
    pub outputs: Vec<String>,
    /// Atoms this one declares it composes into, by slug.
    #[serde(default)]
    pub next: Vec<String>,
    /// Witness types that would evidence this atom is **done**, resolved
    /// through the same `schema/witness` probes teloi use. `in`/`out`/`next`
    /// say what an atom consumes, produces, and leads to; this says how you
    /// know it finished. Absent means no completion criteria are declared —
    /// reported as such, never treated as met.
    ///
    /// Additive: `skip_serializing_if` keeps every block written before this
    /// existed byte-identical, the same mechanism `Witnesses::scope` uses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub done: Vec<String>,
}

impl crate::atoms::Versioned for Interface {
    /// An atom's interface. v1 is every block written before versioning
    /// existed, which an absent `_version` still means.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    /// Slug, i.e. the subject name minus the `atom/` prefix.
    pub name: String,
    /// CID of the claim carrying this interface — the evidence pointer, so
    /// a finding can always be traced back to the claim that caused it.
    pub cid: String,
    pub interface: Interface,
}

impl Atom {
    pub fn subject(&self) -> String {
        format!("{ATOM_PREFIX}{}", self.name)
    }
}

impl Interface {
    /// Renders the claim text `day atom declare` appends — the exact shape
    /// [`extract_interface`] reads back. Write and read share the
    /// `Interface` type and this one function, so a hand-written block and a
    /// day-written block cannot mean different things.
    pub fn to_claim_text(&self, slug: &str, note: Option<&str>) -> String {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        let note = note
            .map(|n| format!("{n}\n\n"))
            .unwrap_or_else(|| format!("The {slug} atom.\n\n"));
        format!("{note}```{FENCE_INFO}\n{json}\n```\n")
    }
}

/// Something wrong with the live atom set. Advisory: day reports these and
/// exits non-zero, it never rewrites the log to "fix" them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Every atom implicated, so callers can render both sides of a
    /// mismatch rather than just the source.
    pub atoms: Vec<String>,
    pub message: String,
    /// True when this finding is a declaration **this build is too old to
    /// read**, rather than one that is wrong.
    ///
    /// Carried as a flag rather than left for a caller to grep out of
    /// `message`, because the two need different actions from different people
    /// — upgrade the binary, or fix the claim — and a caller deciding that by
    /// substring match would break the first time the wording changed.
    pub version_skew: bool,
}

/// Reads every `atom/<slug>` subject's live claims and takes the newest
/// interface-bearing claim as that atom's current interface.
pub fn load(client: &KanClient) -> Result<(Vec<Atom>, Vec<Finding>), Error> {
    let mut atoms = Vec::new();
    let mut findings = Vec::new();

    let mut subjects: Vec<String> = client
        .subjects()?
        .into_iter()
        .filter(|s| s.starts_with(ATOM_PREFIX))
        .collect();
    subjects.sort();

    for subject in subjects {
        let name = subject[ATOM_PREFIX.len()..].to_string();
        let claims = client.show(&subject)?;
        // Latest interface-bearing claim wins: `kan show` prints a
        // subject's live claims oldest-first, so the last match is current.
        let latest = claims.iter().rev().find_map(|c| {
            c.text
                .as_deref()
                .and_then(extract_interface)
                .map(|r| (c, r))
        });

        match latest {
            Some((claim, Ok(interface))) => atoms.push(Atom {
                name,
                cid: claim.cid.clone(),
                interface,
            }),
            // The `BlockError` already names the fence and says whether this
            // day is behind the log or the block is wrong, so this wrapper adds
            // only the subject and the claim. Restating it here is what made
            // the message say "not valid interface JSON (not valid day-atom
            // JSON: …)" — the same thing twice, in two vocabularies.
            Some((claim, Err(e))) => findings.push(Finding {
                atoms: vec![name.clone()],
                version_skew: e.is_version_skew(),
                message: format!("{subject}: {e} — claim {}", claim.cid),
            }),
            None => findings.push(Finding {
                atoms: vec![name.clone()],
                version_skew: false,
                message: format!(
                    "{subject}: no `{FENCE_INFO}` interface block on any live claim, so it can't be composition-checked"
                ),
            }),
        }
    }

    Ok((atoms, findings))
}

/// A claim's prose with fenced blocks removed.
///
/// Both uses found by dogfooding: rendering a telos statement printed the
/// whole `day-telos` block back at the reader, and — worse — the witness
/// scan matched every witness type against the block that *declares* it, so
/// every telos reported its own declaration as a prose assertion that the
/// witness had been satisfied. A declaration is not an assertion of success.
pub fn prose_only(text: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.trim().to_string()
}

/// Pulls the first fenced block with the given info string out of a claim's
/// text and deserializes it. Returns `None` when the claim carries no such
/// block at all (most claims don't), `Some(Err(..))` when it carries one
/// that doesn't parse — the difference matters, since the second is a real
/// finding and the first isn't.
///
/// Shared by atoms (`day-atom`) and design-doc schemas (`day-schema`): one
/// embedded-block convention, not two, so a project learns the pattern once.
pub fn extract_fenced<T: serde::de::DeserializeOwned + Versioned>(
    text: &str,
    fence: &str,
) -> Option<Result<T, BlockError>> {
    let open = format!("```{fence}");
    let start = text.find(&open)? + open.len();
    let rest = &text[start..];
    let end = rest.find("```")?;
    Some(parse_block::<T>(rest[..end].trim()))
}

/// The version gate, then the typed parse.
///
/// **The version is read and then removed before the typed parse**, rather
/// than being a field on every block type. Three reasons, and the third is
/// what decides it:
///
/// 1. The seven block types need no new field, so nothing day already writes
///    changes shape and no round-trip becomes non-byte-identical.
/// 2. `deny_unknown_fields` and `serde(flatten)` do not compose, so a shared
///    metadata struct was never available anyway.
/// 3. [`crate::telos::WitnessSchema`] is `transparent` over a map from witness
///    type to probe. A `_version` *field* there would be read as a witness type
///    literally named `_version`; stripping it first is the only approach that
///    works for a block whose body is a map rather than a struct.
fn parse_block<T: serde::de::DeserializeOwned + Versioned>(json: &str) -> Result<T, BlockError> {
    let malformed = |source| BlockError::Malformed {
        fence: T::FENCE,
        source,
    };

    let mut value: serde_json::Value = serde_json::from_str(json).map_err(malformed)?;

    // A non-object block (a bare array, say) carries no metadata and cannot be
    // version-gated; hand it to the typed parse, which is what will reject it.
    if let Some(object) = value.as_object_mut() {
        if let Some(declared) = object.remove(VERSION_KEY) {
            let Some(declared) = declared.as_u64() else {
                // A `_version` that is not a number is a malformed block rather
                // than a version this day cannot read: day cannot tell whether
                // it is behind, so it must not claim to be.
                return Err(malformed(serde::de::Error::custom(format!(
                    "{VERSION_KEY} must be a positive integer, found `{declared}`"
                ))));
            };
            if declared > T::SUPPORTED_VERSION {
                return Err(BlockError::TooNew {
                    fence: T::FENCE,
                    declared,
                    supported: T::SUPPORTED_VERSION,
                });
            }
        }
    }

    serde_json::from_value(value).map_err(malformed)
}

/// [`extract_fenced`] specialized to an atom's `day-atom` interface block.
pub fn extract_interface(text: &str) -> Option<Result<Interface, BlockError>> {
    extract_fenced(text, FENCE_INFO)
}

/// Reads the newest claim on `subject` carrying a `fence` block, returning
/// the parsed value with the CID of the claim it came from. The
/// newest-wins rule every kan-backed vocabulary in day uses.
pub fn newest_fenced<T: serde::de::DeserializeOwned + Versioned>(
    client: &KanClient,
    subject: &str,
    fence: &str,
) -> Result<Option<(String, T)>, Error> {
    let claims = client.show(subject)?;
    for claim in claims.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        match extract_fenced::<T>(text, fence) {
            Some(Ok(value)) => return Ok(Some((claim.cid.clone(), value))),
            // An unreadable block on the newest claim is not silently skipped
            // in favour of an older good one — that would hide the error, and
            // would silently resolve an *older* declaration as though it were
            // current, which is worse than failing.
            Some(Err(source)) => {
                return Err(Error::Block {
                    subject: subject.to_string(),
                    cid: claim.cid.clone(),
                    source,
                })
            }
            None => continue,
        }
    }
    Ok(None)
}

/// The composition check: every declared `next` edge must name an atom that
/// exists, and every atom that has upstream atoms must have its inputs
/// covered by what those upstreams produce. A derived read over kan's fold —
/// the same category of computation as kan's own status fold, computing
/// nothing into the log.
///
/// Coverage is checked against the **transitive** upstream closure, not just
/// the immediate predecessor. Artifacts accumulate along a bridging path
/// rather than being consumed by the next step: an adversarial review needs
/// both the design doc and the code change, and the design doc is still
/// there when the review runs even though the build step in between didn't
/// re-emit it. Checking only immediate predecessors reports that as a
/// failure, which is how this rule was found — day's own two atoms tripped
/// it the first time `day doctor` ran against a real log.
///
/// An atom with no upstream atoms is a source; its inputs come from outside
/// the vocabulary and are not checked.
pub fn check(atoms: &[Atom]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for atom in atoms {
        for successor in &atom.interface.next {
            if !atoms.iter().any(|a| &a.name == successor) {
                findings.push(Finding {
                    atoms: vec![atom.name.clone(), successor.clone()],
                    version_skew: false,
                    message: format!(
                        "{} declares next: {successor}, but no {ATOM_PREFIX}{successor} subject exists in the live vocabulary",
                        atom.subject()
                    ),
                });
            }
        }
    }

    for atom in atoms {
        let upstream = ancestors(atoms, &atom.name);
        if upstream.is_empty() {
            continue;
        }

        let available: Vec<&str> = upstream
            .iter()
            .flat_map(|a| a.interface.outputs.iter().map(String::as_str))
            .collect();
        let missing: Vec<&str> = atom
            .interface
            .inputs
            .iter()
            .filter(|input| !available.contains(&input.as_str()))
            .map(String::as_str)
            .collect();

        if !missing.is_empty() {
            let mut implicated: Vec<String> = upstream.iter().map(|a| a.name.clone()).collect();
            implicated.push(atom.name.clone());
            findings.push(Finding {
                atoms: implicated,
                version_skew: false,
                message: format!(
                    "{}: interfaces do not compose — needs input(s) [{}] that nothing upstream produces (upstream {} produce [{}])",
                    atom.subject(),
                    missing.join(", "),
                    upstream
                        .iter()
                        .map(|a| a.subject())
                        .collect::<Vec<_>>()
                        .join(", "),
                    available.join(", "),
                ),
            });
        }
    }

    findings
}

/// Every atom that can reach `name` through `next` edges. Breadth-first with
/// a visited set, so a cyclic vocabulary terminates rather than hanging —
/// cycles are legal here (a drift-evaluation atom feeding back into design
/// is a real pattern), they just must not be walked twice.
fn ancestors<'a>(atoms: &'a [Atom], name: &str) -> Vec<&'a Atom> {
    let mut found: Vec<&Atom> = Vec::new();
    let mut frontier = vec![name.to_string()];
    let mut seen: Vec<String> = vec![name.to_string()];

    while let Some(current) = frontier.pop() {
        for candidate in atoms {
            if !candidate.interface.next.contains(&current) {
                continue;
            }
            if seen.contains(&candidate.name) {
                continue;
            }
            seen.push(candidate.name.clone());
            frontier.push(candidate.name.clone());
            found.push(candidate);
        }
    }

    found.sort_by(|a, b| a.name.cmp(&b.name));
    found
}

/// `.design/honest-reads.md` REQ-1 and REQ-2, at the level the contract lives:
/// one gate, exercised against **every** block type day owns.
///
/// These are unit tests rather than seven end-to-end runs on purpose. Each block
/// type is read by a different verb (`doctor`, `assess telos`, `bridge check`,
/// `design check`, `assess docs`), so driving all seven through their verbs would
/// test the verbs, at seven subprocesses per case, while testing the gate once.
/// The gate is what has to hold for all seven.
#[cfg(test)]
mod version_gate {
    use super::*;
    use crate::{bridge, docs, schema, telos, tension};

    /// One row per block type day owns. `strict` records whether the body is a
    /// **struct**, where an unrecognised key is a field the type does not
    /// declare and must be refused — or a **map**, where every key is data and
    /// refusing unknown ones would refuse the project's own vocabulary.
    ///
    /// Adding a block type to day means adding a row here, which is the point:
    /// this table is the inventory, and a new block type that is neither strict
    /// nor deliberately lax cannot be added without someone deciding which.
    fn parse_all_seven(
        mutate: impl Fn(&str) -> String,
    ) -> Vec<(&'static str, bool, Result<(), String>)> {
        fn attempt<T: serde::de::DeserializeOwned + Versioned>(json: &str) -> Result<(), String> {
            parse_block::<T>(json)
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        vec![
            (
                "day-atom",
                true,
                attempt::<Interface>(&mutate(r#"{"in":["a"],"out":["b"]}"#)),
            ),
            (
                "day-telos",
                true,
                attempt::<bridge::Witnesses>(&mutate(r#"{"witnesses":["a"]}"#)),
            ),
            (
                "day-bridge",
                true,
                attempt::<bridge::Plan>(&mutate(
                    r#"{"telos":"t","have":["intent"],"plan":{"atom":"design"}}"#,
                )),
            ),
            (
                // A map from witness type to probe, so any key is a witness
                // type this project declared. Strictness lives one level down,
                // on the probe — see the `Versioned` impl for `WitnessSchema`.
                "day-witness",
                false,
                attempt::<telos::WitnessSchema>(&mutate(r#"{"code-change":{"path":"src/*"}}"#)),
            ),
            (
                "day-schema",
                true,
                attempt::<schema::Schema>(&mutate(
                    r#"{"sections":["Summary"],"requirement_prefix":"REQ-","criterion_prefix":"AC-","min_requirements":1,"min_criteria":1,"placeholders":[],"paths_section":"Architecture"}"#,
                )),
            ),
            (
                "day-docs",
                true,
                attempt::<docs::DocsSchema>(&mutate(
                    r#"{"version_source":"Cargo.toml","version_key":"version"}"#,
                )),
            ),
            (
                "day-tension",
                true,
                attempt::<tension::Tension>(&mutate(r#"{"between":["a","b"]}"#)),
            ),
        ]
    }

    /// AC-1's negative control, and it has to come first: if the untouched
    /// bodies did not parse, every assertion below would pass for the wrong
    /// reason.
    #[test]
    fn every_block_type_parses_its_own_minimal_body() {
        for (fence, _, result) in parse_all_seven(str::to_string) {
            assert!(result.is_ok(), "{fence} rejected a valid body: {result:?}");
        }
    }

    /// AC-1: an unrecognised field is **refused**, not dropped, for every
    /// struct-shaped block.
    ///
    /// Before this, all of them parsed and silently discarded the field — which
    /// for a vocabulary that exists to *constrain* something is a false
    /// certification rather than lost information.
    #[test]
    fn every_struct_shaped_block_refuses_an_unrecognised_field() {
        let inject = |json: &str| json.replacen('{', r#"{"nonsense_field":1,"#, 1);
        let rows = parse_all_seven(inject);
        assert!(
            rows.iter().filter(|(_, strict, _)| *strict).count() >= 6,
            "the strict set should not have quietly shrunk"
        );
        for (fence, strict, result) in rows {
            if !strict {
                continue;
            }
            let err = result.expect_err(&format!("{fence} silently dropped an unknown field"));
            assert!(
                err.contains("nonsense_field"),
                "{fence} should name the field it refused: {err}"
            );
        }
    }

    /// The `day-witness` map's contract, which is deliberately *not* the one
    /// above and would be wrong if it were: an unrecognised key is a witness
    /// type this project declared, and refusing it would refuse the project's
    /// own vocabulary. What must not happen is the probe being dropped
    /// silently — it is set aside and reported, so a reader never mistakes
    /// "unreadable here" for "no probe declared".
    #[test]
    fn an_unreadable_probe_is_set_aside_and_reported_not_dropped() {
        let schema = parse_block::<telos::WitnessSchema>(
            r#"{"code-change":{"path":"src/*"},"exotic":{"future-kind":{"x":1}}}"#,
        )
        .expect("a witness type day has never heard of is the project's business");

        assert!(schema.probes.contains_key("code-change"));
        assert!(
            !schema.probes.contains_key("exotic"),
            "an unreadable probe must not land in the usable set"
        );
        assert!(
            schema.unsupported.contains_key("exotic"),
            "and must be reported rather than dropped: {schema:?}"
        );
    }

    /// AC-3: an absent `_version` and an explicit `_version: 1` are the same
    /// block, so nothing written before versioning existed needs touching.
    #[test]
    fn an_absent_version_means_the_first_version() {
        let implicit = parse_block::<Interface>(r#"{"in":["a"],"out":["b"]}"#).unwrap();
        let explicit =
            parse_block::<Interface>(r#"{"_version":1,"in":["a"],"out":["b"]}"#).unwrap();
        assert_eq!(implicit, explicit);
        assert_eq!(IMPLICIT_VERSION, 1);
    }

    /// AC-3: `_version` is accepted on every block type, not only the one it
    /// was implemented against — including `day-witness`, whose body is a *map*
    /// and where a `_version` field would otherwise read as a witness type
    /// literally named `_version`.
    #[test]
    fn the_version_key_is_stripped_from_every_block_type() {
        let inject = |json: &str| json.replacen('{', r#"{"_version":1,"#, 1);
        for (fence, _, result) in parse_all_seven(inject) {
            assert!(
                result.is_ok(),
                "{fence} should accept an explicit v1: {result:?}"
            );
        }
        // The map case specifically: `_version` must not survive as a key.
        let schema =
            parse_block::<telos::WitnessSchema>(r#"{"_version":1,"code-change":{"path":"src/*"}}"#)
                .unwrap();
        assert!(
            !schema.probes.contains_key(VERSION_KEY)
                && !schema.unsupported.contains_key(VERSION_KEY),
            "the version key leaked into the witness map: {schema:?}"
        );
        assert!(schema.probes.contains_key("code-change"));
    }

    /// AC-4: a block this day is too old to read reports **the reader is
    /// behind**, distinguishably from a block that is simply wrong. The two
    /// need different actions from different people, which is why they are
    /// different variants rather than one message.
    #[test]
    fn a_too_new_block_blames_the_reader_and_a_broken_one_blames_the_claim() {
        let too_new = parse_block::<Interface>(r#"{"_version":2,"in":["a"]}"#).unwrap_err();
        assert!(too_new.is_version_skew());
        let rendered = too_new.to_string();
        assert!(
            rendered.contains('2'),
            "names the declared version: {rendered}"
        );
        assert!(rendered.contains('1'), "and the supported one: {rendered}");
        assert!(
            rendered.contains("upgrade day"),
            "and says whose problem it is: {rendered}"
        );

        // Malformed, at a version this day does read.
        let broken = parse_block::<Interface>(r#"{"in":["a"],}"#).unwrap_err();
        assert!(!broken.is_version_skew());
        assert!(
            !broken.to_string().contains("upgrade day"),
            "a broken block must not tell the reader to upgrade: {broken}"
        );

        // An unknown field is the claim's problem too, not version skew — it is
        // only skew when the block *says* it needs a newer reader. Getting this
        // backwards would tell every project their day was out of date.
        let unknown = parse_block::<Interface>(r#"{"in":["a"],"requires":["x"]}"#).unwrap_err();
        assert!(!unknown.is_version_skew(), "{unknown}");
    }

    /// A `_version` that is not a number is the claim's problem, not the
    /// reader's: day cannot tell whether it is behind, so it must not claim to
    /// be. The tempting alternative — treat anything unparseable as "probably
    /// newer" — would send every reader to upgrade over a typo.
    #[test]
    fn a_non_numeric_version_is_malformed_not_skew() {
        let e = parse_block::<Interface>(r#"{"_version":"two","in":["a"]}"#).unwrap_err();
        assert!(!e.is_version_skew(), "{e}");
        assert!(e.to_string().contains(VERSION_KEY), "{e}");
    }

    /// Versions are **per block type**, so a `day-atom` this day cannot read
    /// does not make it unable to read a `day-telos`. One shared version bumped
    /// for one block would invalidate all seven for an older reader, which is
    /// the whole-vocabulary blast radius the smallest-unit rule exists to avoid.
    #[test]
    fn a_too_new_block_of_one_type_does_not_affect_another() {
        assert!(parse_block::<Interface>(r#"{"_version":2,"in":["a"]}"#).is_err());
        assert!(parse_block::<bridge::Witnesses>(r#"{"witnesses":["a"]}"#).is_ok());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atom(name: &str, inputs: &[&str], outputs: &[&str], next: &[&str]) -> Atom {
        Atom {
            name: name.to_string(),
            cid: format!("bafy-{name}"),
            interface: Interface {
                inputs: inputs.iter().map(|s| s.to_string()).collect(),
                outputs: outputs.iter().map(|s| s.to_string()).collect(),
                next: next.iter().map(|s| s.to_string()).collect(),
                done: vec![],
            },
        }
    }

    #[test]
    fn a_written_interface_reads_back_identical() {
        let interface = Interface {
            inputs: vec!["design-doc".into()],
            outputs: vec!["code-change".into()],
            next: vec!["adversarial-review".into()],
            done: vec![],
        };
        let text = interface.to_claim_text("generative-build", None);
        let parsed = extract_interface(&text)
            .expect("the written text should contain a block")
            .expect("the written block should be valid");
        assert_eq!(parsed, interface);
    }

    #[test]
    fn extracts_a_fenced_interface_block() {
        let text = "The build atom.\n\n```day-atom\n{\"in\": [\"design-doc\"], \"out\": [\"code-change\"]}\n```\n";
        let interface = extract_interface(text)
            .expect("block present")
            .expect("valid json");
        assert_eq!(interface.inputs, vec!["design-doc"]);
        assert_eq!(interface.outputs, vec!["code-change"]);
        assert!(interface.next.is_empty());
    }

    #[test]
    fn claim_text_with_no_block_is_not_a_finding() {
        assert!(extract_interface("just an ordinary observation").is_none());
    }

    #[test]
    fn malformed_block_is_distinguishable_from_absent_block() {
        let text = "```day-atom\n{not json}\n```";
        assert!(extract_interface(text).expect("block present").is_err());
    }

    #[test]
    fn composing_interfaces_pass() {
        let atoms = vec![
            atom("design", &["idea"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn mismatched_interfaces_name_both_atoms() {
        let atoms = vec![
            atom("design", &["idea"], &["design-doc"], &["build"]),
            atom("build", &["verified-spec"], &["code-change"], &[]),
        ];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].atoms, vec!["design", "build"]);
        assert!(findings[0].message.contains("verified-spec"));
    }

    #[test]
    fn an_artifact_stays_available_past_the_step_that_did_not_re_emit_it() {
        // The case day's own vocabulary hit: review needs the design doc
        // and the code change; build sits in between and only produces the
        // code change. The design doc is still there when review runs.
        let atoms = vec![
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &["review"]),
            atom("review", &["design-doc", "code-change"], &["verdict"], &[]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn a_cyclic_vocabulary_terminates() {
        let atoms = vec![
            atom("design", &["drift-report"], &["design-doc"], &["drift"]),
            atom("drift", &["design-doc"], &["drift-report"], &["design"]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn a_source_atoms_inputs_are_not_checked() {
        let atoms = vec![atom("design", &["intent"], &["design-doc"], &[])];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn dangling_successor_is_a_finding() {
        let atoms = vec![atom("design", &["idea"], &["design-doc"], &["nonexistent"])];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1);
        assert!(findings[0]
            .message
            .contains("no atom/nonexistent subject exists"));
    }
}
