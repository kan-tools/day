//! `.design/vocabulary-packs.md` REQ-11, REQ-12, REQ-16, REQ-17 and REQ-21,
//! built for `schema/witness` first.
//!
//! A configuration key becomes its own subject, so newest-wins *per subject* is
//! per-key resolution with no change to the fold. The cost this removes is
//! measured rather than asserted: `schema/witness` has been restated **twelve
//! times** to reach eighteen entries, because `newest_fenced` takes the newest
//! claim whole, and its own newest claim records that "this is the third time
//! the cost has been paid on this subject".

mod common;

use common::{claim, retraction_claim, write_kan_stub, StubClaim};
use day::kan_client::KanClient;
use day::layers::{self, Layer};
use day::probe::Probe;
use day::telos::WitnessSchema;

/// A claim on the parent subject carrying the whole map — the shape every
/// declaration written before per-key subjects has.
fn legacy_block(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes for this project.\n\n```day-witness\n{body}\n```\n"),
    )
}

/// A claim on a key's own subject, carrying that key's **value** — the shape
/// REQ-21 requires be declared rather than implied.
fn key_claim(key: &str, cid: &str, body: &str) -> StubClaim {
    claim(
        &format!("schema/witness/{key}"),
        cid,
        &format!("The {key} witness.\n\n```day-witness\n{body}\n```\n"),
    )
}

fn effective(claims: &[StubClaim]) -> (WitnessSchema, std::collections::BTreeMap<String, Layer>) {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(dir.path(), claims);
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());
    let e = layers::witness(&client).expect("the assembler should read the stub log");
    (e.value, e.provenance)
}

fn tag_of(schema: &WitnessSchema, key: &str) -> Option<String> {
    match schema.probes.get(key) {
        Some(Probe::Tag(p)) => Some(p.clone()),
        _ => None,
    }
}

/// **REQ-11, and the test that fails before the change.**
///
/// Two keys set by two separate claims both survive. Under the whole-block fold
/// these are two claims on one subject and `newest_fenced` takes the newest
/// whole, so the first is lost entirely — which is why adding one entry has
/// meant restating all of them, twelve times so far.
#[test]
fn two_keys_on_their_own_subjects_both_resolve() {
    let (schema, _) = effective(&[
        key_claim("published-artifact", "bafyone", r#"{"tag": "v*"}"#),
        key_claim("design-doc", "bafytwo", r#"{"path": ".design/*.md"}"#),
    ]);

    assert_eq!(tag_of(&schema, "published-artifact").as_deref(), Some("v*"));
    assert!(
        matches!(schema.probes.get("design-doc"), Some(Probe::Path(p)) if p == ".design/*.md"),
        "both keys must resolve; got {:?}",
        schema.probes
    );
}

/// **REQ-12 — no migration.** A project that never adopts per-key subjects sees
/// exactly what it saw before, including the error when nothing is declared.
#[test]
fn no_claim_anywhere_still_reports_not_declared() {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(
        dir.path(),
        &[claim("unrelated", "bafyx", "nothing to do with it")],
    );
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());

    let err = WitnessSchema::load(&client).expect_err("no layer contributes, so this must error");
    assert!(
        format!("{err}").contains("kan observe"),
        "the error must still carry the starter command it always did: {err}"
    );
}

/// **REQ-12.** A legacy whole-block claim alone resolves to what that block
/// declared, byte-for-byte.
#[test]
fn a_legacy_block_alone_reads_exactly_as_before() {
    let (schema, provenance) = effective(&[legacy_block(
        "bafylegacy",
        r#"{"published-artifact": {"tag": "v*"}, "design-doc": {"path": ".design/*.md"}}"#,
    )]);

    assert_eq!(tag_of(&schema, "published-artifact").as_deref(), Some("v*"));
    assert_eq!(schema.probes.len(), 2);
    assert_eq!(
        provenance.get("published-artifact"),
        Some(&Layer::LegacyBlock("bafylegacy".to_string())),
        "every key a whole block sets shares that block's CID — which is exactly \
         why per-key provenance needs per-key subjects"
    );
}

/// **REQ-12, the overlay.** A per-key claim wins for its own key and leaves the
/// block's other keys untouched.
#[test]
fn a_per_key_claim_overrides_the_legacy_block_and_leaves_the_rest() {
    let (schema, provenance) = effective(&[
        legacy_block(
            "bafylegacy",
            r#"{"published-artifact": {"tag": "v*"}, "design-doc": {"path": ".design/*.md"}}"#,
        ),
        key_claim("published-artifact", "bafykey", r#"{"tag": "v1.*"}"#),
    ]);

    assert_eq!(
        tag_of(&schema, "published-artifact").as_deref(),
        Some("v1.*"),
        "the key's own subject is the newest layer and must win"
    );
    assert!(
        matches!(schema.probes.get("design-doc"), Some(Probe::Path(p)) if p == ".design/*.md"),
        "the block's other keys must survive — losing them is the defect this replaces"
    );
    assert_eq!(
        provenance.get("published-artifact"),
        Some(&Layer::Key("bafykey".to_string()))
    );
    assert_eq!(
        provenance.get("design-doc"),
        Some(&Layer::LegacyBlock("bafylegacy".to_string()))
    );
}

/// **REQ-16 — granular retraction, the capability per-key subjects exist for.**
///
/// `kan retract` leaves the subject in place carrying only a `Retraction`, so
/// the key is gone while its subject remains.
#[test]
fn retracting_a_keys_only_claim_removes_exactly_that_key() {
    let (schema, provenance) = effective(&[
        key_claim("design-doc", "bafykeep", r#"{"path": ".design/*.md"}"#),
        retraction_claim("schema/witness/published-artifact", "bafygone"),
    ]);

    assert!(
        !schema.probes.contains_key("published-artifact"),
        "a retracted key must be absent; got {:?}",
        schema.probes
    );
    assert!(
        schema.probes.contains_key("design-doc"),
        "and every other key must survive"
    );
    assert!(!provenance.contains_key("published-artifact"));
}

/// **REQ-21.** "Subject exists, no block" reads as *this key is absent*, never
/// as a read failure — and where a legacy block also declares the key, the
/// legacy value shows through.
///
/// **This is REQ-20's stated limit, asserted rather than hoped for**: a key
/// inherited from the legacy block cannot be granularly retracted, because the
/// claim carrying it also carries every other key. Every adopting project
/// passes through this hybrid state, and an earlier draft of the design claimed
/// the capability uniformly.
#[test]
fn a_retracted_key_falls_back_to_the_legacy_block_rather_than_failing() {
    let (schema, provenance) = effective(&[
        legacy_block("bafylegacy", r#"{"published-artifact": {"tag": "v*"}}"#),
        retraction_claim("schema/witness/published-artifact", "bafygone"),
    ]);

    assert_eq!(
        tag_of(&schema, "published-artifact").as_deref(),
        Some("v*"),
        "the per-key layer contributes nothing, so the legacy layer decides — \
         retraction is NOT granular for a key the block also sets (REQ-20)"
    );
    assert_eq!(
        provenance.get("published-artifact"),
        Some(&Layer::LegacyBlock("bafylegacy".to_string()))
    );
}

/// A malformed per-key block costs **that key** and nothing else.
///
/// `WitnessSchema`'s own doc records what the alternative did: a probe kind the
/// installed binary could not read failed the whole schema, and with it every
/// hook and status line in the session.
#[test]
fn a_malformed_key_costs_that_key_and_not_the_map() {
    let (schema, _) = effective(&[
        key_claim("design-doc", "bafyok", r#"{"path": ".design/*.md"}"#),
        key_claim(
            "published-artifact",
            "bafybad",
            r#"{"nonsense-kind": "v*"}"#,
        ),
    ]);

    assert!(
        schema.probes.contains_key("design-doc"),
        "the readable key must still load"
    );
    assert!(
        schema.unsupported.contains_key("published-artifact"),
        "and the unreadable one must be REPORTED, not dropped: {:?}",
        schema.unsupported
    );
}

/// A per-key claim replaces the legacy entry in **all** buckets: a legacy
/// material/record pair overlaid by a material-only key loses its record half,
/// rather than keeping half a question nobody declared.
#[test]
fn a_material_only_key_clears_the_legacy_record_half() {
    let (schema, _) = effective(&[
        legacy_block(
            "bafylegacy",
            r#"{"release": {"material": {"tag": "v*"}, "record": {"claim": {"kind": "Result"}}}}"#,
        ),
        key_claim("release", "bafykey", r#"{"tag": "v1.*"}"#),
    ]);

    assert_eq!(tag_of(&schema, "release").as_deref(), Some("v1.*"));
    assert!(
        !schema.records.contains_key("release"),
        "the legacy record half must not survive a material-only override: {:?}",
        schema.records
    );
}

/// A deeper rkey is not a key of this map. Flattening it would invent a witness
/// named `a/b` that nobody declared.
#[test]
fn a_nested_subject_is_not_a_witness() {
    let (schema, _) = effective(&[
        key_claim("design-doc", "bafyok", r#"{"path": ".design/*.md"}"#),
        key_claim("a/b", "bafydeep", r#"{"tag": "v*"}"#),
    ]);

    assert_eq!(
        schema.probes.keys().collect::<Vec<_>>(),
        vec!["design-doc"],
        "only the single-segment key is a witness"
    );
}

/// **REQ-12, and a regression this change introduced before a review caught it.**
///
/// A legacy block of `{}` is a declaration that names no keys. The whole-block
/// loader returned an empty schema for it; deciding "is anything declared" from
/// whether any key resolved turned that into `NotDeclared`, which is a
/// behaviour change REQ-12 forbids in the words "byte-identical to today's
/// behaviour".
#[test]
fn an_empty_legacy_block_is_a_declaration_not_an_absence() {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(dir.path(), &[legacy_block("bafyempty", "{}")]);
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());

    let schema = WitnessSchema::load(&client)
        .expect("an empty block is a declaration; the loader must not report it absent");
    assert!(schema.probes.is_empty());
}

/// **`telos/honest-reads`, and day#60 exactly.**
///
/// A per-key claim written by a NEWER day is version skew, fixed by upgrading.
/// A malformed block is the claim's problem, fixed by editing it. Telling
/// someone to edit a claim that is fine is the failure day#60 records.
///
/// The first version of this code handed the block error to the entry parser as
/// a string, so the real message came back wrapped in serde's `unknown variant
/// … expected one of `path`, `tag`, …` — pointing the reader at the probe kinds
/// rather than at their day version.
#[test]
fn a_key_from_the_future_says_upgrade_rather_than_naming_probe_kinds() {
    let (schema, _) = effective(&[key_claim(
        "published-artifact",
        "bafyfuture",
        r#"{"_version": 999, "tag": "v*"}"#,
    )]);

    let reason = schema
        .unsupported
        .get("published-artifact")
        .expect("a block from the future must be reported, not dropped");

    assert!(
        reason.contains("upgrade day"),
        "the remedy must be the one that works: {reason}"
    );
    assert!(
        !reason.contains("unknown variant"),
        "and must not be wrapped in a parse error that blames the claim: {reason}"
    );
}
