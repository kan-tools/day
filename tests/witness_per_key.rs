//! The per-key layer, driven through the API that existed **before** it.
//!
//! Separate from `tests/witness_layers.rs` on purpose, and the reason is a
//! property of the demonstration rather than of the code: that file imports
//! `day::layers`, so a tree without the change cannot compile it, and
//! `scripts/revert-demo.py` correctly reports `DID-NOT-COMPILE` — which it
//! states outright "says nothing about whether they assert the fix".
//!
//! Everything here goes through [`WitnessSchema::load`], which exists on both
//! sides of the change. Revert `src/` and these fail rather than failing to
//! build, which is the difference between a demonstration and a build error
//! wearing one's clothes.

mod common;

use common::{claim, write_kan_stub, StubClaim};
use day::kan_client::KanClient;
use day::probe::Probe;
use day::telos::WitnessSchema;

fn key_claim(key: &str, cid: &str, body: &str) -> StubClaim {
    claim(
        &format!("schema/witness/{key}"),
        cid,
        &format!("The {key} witness.\n\n```day-witness\n{body}\n```\n"),
    )
}

fn loaded(claims: &[StubClaim]) -> WitnessSchema {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(dir.path(), claims);
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());
    WitnessSchema::load(&client).expect("a declared witness schema should load")
}

/// **REQ-11, through the loader every caller already uses.**
///
/// Two keys, two claims, both survive. Before per-key subjects these could only
/// be two claims on `schema/witness`, where `newest_fenced` takes the newest
/// whole and the first is lost — which is why adding one entry has meant
/// restating every entry, twelve times so far to reach eighteen.
///
/// Driving `load` rather than the assembler is what makes this reversible: on a
/// tree without the change, `load` reads only the parent subject, finds no
/// claim there, and returns `NotDeclared`.
#[test]
fn load_resolves_two_keys_declared_on_their_own_subjects() {
    let schema = loaded(&[
        key_claim("published-artifact", "bafyone", r#"{"tag": "v*"}"#),
        key_claim("design-doc", "bafytwo", r#"{"path": ".design/*.md"}"#),
    ]);

    assert!(
        matches!(schema.probes.get("published-artifact"), Some(Probe::Tag(p)) if p == "v*"),
        "the first key must resolve; got {:?}",
        schema.probes
    );
    assert!(
        matches!(schema.probes.get("design-doc"), Some(Probe::Path(p)) if p == ".design/*.md"),
        "and so must the second — losing one is the whole-block defect: {:?}",
        schema.probes
    );
}

/// **REQ-12, through the loader.** A per-key claim overrides the legacy block
/// for its own key and leaves the block's other keys standing.
#[test]
fn load_overlays_a_key_on_the_legacy_block() {
    let schema = loaded(&[
        claim(
            "schema/witness",
            "bafylegacy",
            "Witness probes.\n\n```day-witness\n{\"published-artifact\": {\"tag\": \"v*\"}, \
             \"design-doc\": {\"path\": \".design/*.md\"}}\n```\n",
        ),
        key_claim("published-artifact", "bafykey", r#"{"tag": "v1.*"}"#),
    ]);

    assert!(
        matches!(schema.probes.get("published-artifact"), Some(Probe::Tag(p)) if p == "v1.*"),
        "the key's own subject is the newer layer: {:?}",
        schema.probes
    );
    assert!(
        matches!(schema.probes.get("design-doc"), Some(Probe::Path(p)) if p == ".design/*.md"),
        "the block's other keys must survive the overlay: {:?}",
        schema.probes
    );
}
