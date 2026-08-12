//! `.design/vocabulary-packs.md` REQ-11, REQ-12, REQ-17, REQ-20 and REQ-21 for
//! the **config-struct** shape, after `tests/witness_layers.rs` did the map.
//!
//! The two shapes are not one thing, which is the finding RQ-7 records: "per
//! key" is field-wise for a config struct, entry-wise for a map, and undefined
//! for a list. `schema/injection` is the struct the design writes its criteria
//! against, and the difference that matters here is **layer 1** — a struct has a
//! shipped default per field to fall back to, where a map key that nothing
//! declared is simply absent.

mod common;

use common::{claim, retraction_claim, write_kan_stub, StubClaim};
use day::blocks::InjectionSchema;
use day::kan_client::KanClient;
use day::layers::{self, Layer};

/// A claim on the parent subject carrying the whole block — the shape every
/// declaration written before per-key subjects has.
fn legacy_block(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/injection",
        cid,
        &format!("Injection settings for this project.\n\n```day-injection\n{body}\n```\n"),
    )
}

/// A claim on a key's own subject, carrying that key's value under the parent's
/// fence — the shape REQ-21 requires be declared rather than implied.
fn key_claim(key: &str, cid: &str, body: &str) -> StubClaim {
    claim(
        &format!("schema/injection/{key}"),
        cid,
        &format!("The {key} setting.\n\n```day-injection\n{body}\n```\n"),
    )
}

fn resolve(claims: &[StubClaim]) -> layers::Effective<InjectionSchema> {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(dir.path(), claims);
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());
    layers::config::<InjectionSchema>(&client, "injection").expect("the log should resolve")
}

/// **AC-14 — two keys, two claims, both survive.**
///
/// The criterion the design names as the one that fails before the change: under
/// the whole-block fold, two claims on `schema/injection` resolve to the newer
/// one alone, so the first key's value is lost. Per-key subjects make
/// newest-wins *per subject* into per-key resolution with no change to the fold.
#[test]
fn two_keys_set_by_two_claims_both_resolve() {
    let effective = resolve(&[
        key_claim("cadence", "bafyreicadence", r#"{"cadence": 25}"#),
        key_claim(
            "max_practice_items",
            "bafyreimaxitems",
            r#"{"max_practice_items": 30}"#,
        ),
    ]);

    assert_eq!(effective.value.cadence, 25);
    assert_eq!(effective.value.max_practice_items, 30);
    assert_eq!(
        effective.provenance.get("cadence"),
        Some(&Layer::Key("bafyreicadence".into())),
        "and each key names the claim that set it, which is what a per-key CID \
         column exists to make possible"
    );
    assert_eq!(
        effective.provenance.get("max_practice_items"),
        Some(&Layer::Key("bafyreimaxitems".into()))
    );
}

/// **The same two values written the old way, to show what AC-14 is contrasting
/// with.**
///
/// Without this the criterion above reads as a fact about the fixture rather
/// than about the fold. Two whole-block claims on the parent subject: the newer
/// wins entire, and the older claim's field falls back to day's default rather
/// than to what it said.
#[test]
fn two_whole_block_claims_still_lose_the_older_one() {
    let effective = resolve(&[
        legacy_block("bafyreifirst", r#"{"cadence": 25}"#),
        legacy_block("bafyreisecond", r#"{"max_practice_items": 30}"#),
    ]);

    assert_eq!(
        effective.value.max_practice_items, 30,
        "the newer claim wins"
    );
    assert_eq!(
        effective.value.cadence,
        InjectionSchema::default().cadence,
        "and the older claim's field is gone — not merged, not preserved. This \
         is the cost per-key subjects remove, and it is asserted here so the \
         removal is measured against something."
    );
}

/// **AC-15 — no migration, in both of its halves.**
///
/// With no claim anywhere every field is day's shipped default; with a legacy
/// whole-block claim and no per-key subjects, every field is what that block
/// declared. "Byte-identical to today's behaviour" is a promise about every
/// project that already has a declaration, so it is asserted as equality with
/// the value the type itself defines rather than against a transcribed number.
#[test]
fn nothing_declared_and_legacy_only_both_behave_as_before() {
    let empty = resolve(&[]);
    assert_eq!(empty.value, InjectionSchema::default());
    assert!(
        !empty.declared,
        "`declared` reports whether a claim contributed, not whether a value \
         came out — an empty log declared nothing"
    );

    let legacy = resolve(&[legacy_block(
        "bafyreilegacy",
        r#"{"cadence": 7, "max_practice_items": 3}"#,
    )]);
    assert_eq!(legacy.value.cadence, 7);
    assert_eq!(legacy.value.max_practice_items, 3);
    assert_eq!(
        legacy.value.max_practice_item_length,
        InjectionSchema::default().max_practice_item_length,
        "a field the block did not mention still takes the shipped default"
    );
    assert_eq!(
        legacy.provenance.get("max_practice_item_length"),
        Some(&Layer::Default),
        "and reports Default rather than crediting the block with a field it \
         never carried — the distinction a provenance column exists for, and \
         one the typed value cannot make because serde fills defaults silently"
    );
}

/// **AC-16 — a per-key claim overrides its own key and leaves the rest alone.**
#[test]
fn a_per_key_claim_overrides_only_its_own_key() {
    let effective = resolve(&[
        legacy_block(
            "bafyreilegacy",
            r#"{"cadence": 7, "max_practice_items": 3}"#,
        ),
        key_claim("cadence", "bafyreicadence", r#"{"cadence": 25}"#),
    ]);

    assert_eq!(effective.value.cadence, 25);
    assert_eq!(
        effective.value.max_practice_items, 3,
        "the block's other field is untouched"
    );
    assert_eq!(
        effective.provenance.get("cadence"),
        Some(&Layer::Key("bafyreicadence".into()))
    );
    assert_eq!(
        effective.provenance.get("max_practice_items"),
        Some(&Layer::LegacyBlock("bafyreilegacy".into())),
        "and the two keys of one block now report different provenance, which \
         is the whole mechanism"
    );
}

/// **AC-30 — a retracted key subject reads as key-absent, not as a read
/// failure.**
///
/// After `kan retract` the subject remains, carrying only a `Retraction`. Both
/// directions matter: the key falls back to the layer below, and the read
/// succeeds.
#[test]
fn a_retracted_key_falls_back_rather_than_failing() {
    let effective = resolve(&[
        legacy_block("bafyreilegacy", r#"{"cadence": 7}"#),
        retraction_claim("schema/injection/cadence", "bafyreiretracted"),
    ]);

    assert_eq!(
        effective.value.cadence, 7,
        "granular retraction is the capability the whole-block shape cannot \
         offer; a retracted key that failed the read would make it unusable"
    );
    assert_eq!(
        effective.provenance.get("cadence"),
        Some(&Layer::LegacyBlock("bafyreilegacy".into()))
    );
}

/// **REQ-20 — the legacy layer is not granularly retractable, and day must not
/// present the capability as uniform.**
///
/// Provenance is what makes this checkable: a key reporting `LegacyBlock` shares
/// its CID with every other key that claim carried, so retracting it removes
/// them all. Every adopting project passes through this hybrid state, and an
/// earlier draft of the design claimed granular retraction uniformly.
#[test]
fn legacy_keys_share_one_cid_and_per_key_ones_do_not() {
    let effective = resolve(&[
        legacy_block(
            "bafyreilegacy",
            r#"{"cadence": 7, "max_practice_items": 3}"#,
        ),
        key_claim(
            "max_practice_item_length",
            "bafyreilength",
            r#"{"max_practice_item_length": 900}"#,
        ),
    ]);

    let cadence = effective.provenance.get("cadence").unwrap();
    let items = effective.provenance.get("max_practice_items").unwrap();
    assert_eq!(
        cadence, items,
        "two keys inherited from one whole-block claim are inseparable, and \
         reporting the same layer for both is how a caller can say so"
    );
    assert_ne!(
        effective
            .provenance
            .get("max_practice_item_length")
            .unwrap(),
        cadence,
        "while a key with its own subject is separable"
    );
}

/// **REQ-21 — a per-key claim declares the key its subject names, and nothing
/// else.**
///
/// The subject and the block can disagree, and a claim on `.../cadence` that
/// sets `max_practice_items` would otherwise write a key nothing named. Refused
/// with both names, rather than silently applying one of them.
#[test]
fn a_per_key_claim_naming_the_wrong_key_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(
        dir.path(),
        &[key_claim(
            "cadence",
            "bafyreimismatched",
            r#"{"max_practice_items": 30}"#,
        )],
    );
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());

    let err = layers::config::<InjectionSchema>(&client, "injection")
        .expect_err("a claim declaring a key its subject does not name is refused");
    let rendered = err.to_string();
    assert!(
        rendered.contains("cadence") && rendered.contains("max_practice_items"),
        "the message names both the key the subject promised and the one the \
         block delivered, since either could be the typo: {rendered}"
    );
}

/// **A field the type does not have is refused, per-key exactly as in a whole
/// block.**
///
/// A per-key claim carries the parent's fence precisely so it inherits
/// `deny_unknown_fields` and the version gate rather than growing a second
/// grammar. Asserted because "it uses the same fence" is a claim about the code
/// that only a test makes true.
#[test]
fn an_unknown_field_is_refused_on_a_per_key_subject_too() {
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(
        dir.path(),
        &[key_claim(
            "nonesuch",
            "bafyreiunknown",
            r#"{"nonesuch": 1}"#,
        )],
    );
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());

    layers::config::<InjectionSchema>(&client, "injection")
        .expect_err("an unknown field is refused rather than ignored");
}

/// **AC-9's agreement property, one subject: the loader and the assembler cannot
/// disagree, because there is only one of them.**
///
/// `.design/day-config.md` REQ-7 mitigates two readers of one subject with a
/// per-subject agreement test. This is the stronger form available here —
/// `InjectionSchema::load` *is* the assembler — and it is asserted rather than
/// left to inspection, because the loader keeping its own `newest_fenced` call
/// is exactly the drift REQ-17 is about.
#[test]
fn the_loader_and_the_assembler_are_the_same_read() {
    let claims = [
        legacy_block("bafyreilegacy", r#"{"cadence": 7}"#),
        key_claim(
            "max_practice_items",
            "bafyreimaxitems",
            r#"{"max_practice_items": 30}"#,
        ),
    ];
    let dir = tempfile::tempdir().unwrap();
    let bin = write_kan_stub(dir.path(), &claims);
    let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());

    let via_loader = InjectionSchema::load(&client).unwrap();
    let via_assembler = layers::config::<InjectionSchema>(&client, "injection")
        .unwrap()
        .value;
    assert_eq!(via_loader, via_assembler);
    assert_eq!(
        via_loader.max_practice_items, 30,
        "and the loader sees per-key claims at all — a loader still reading the \
         parent subject directly would pass the equality above only if the \
         assembler were also wrong"
    );
}

// ---------------------------------------------------------------------------
// REQ-17, in the form it can honestly take today.
// ---------------------------------------------------------------------------

/// **Every `newest_fenced` call outside the assembler states why it is not
/// per-key.**
///
/// REQ-17 asks for a scan asserting that no `schema/*` loader calls
/// `newest_fenced` directly. Written that way it cannot pass yet and — worse —
/// it could be *made* to pass by matching nothing, which is this repo's most
/// frequently recorded defect. Four subjects are legitimately still on the
/// direct path, and the reasons differ:
///
/// - `schema/blocks` is a `BTreeMap` and belongs on the entry-wise witness path
///   rather than this field-wise one.
/// - `schema/verdicts` is a `Vec`, where RQ-7 records that "per key" is
///   **undefined**; AC-19 wants one subject per permitted verdict.
/// - `schema/docs` and `schema/design-doc` have **no shipped default**: absence
///   is `NotDeclared`, so there is no layer 1 and nothing for a key to fall back
///   to.
/// - `telos/*` witnesses and `bridge/*` plans are a subject's own declaration,
///   where redeclaring must replace.
///
/// **The last two categories are the same category, and RQ-7 separates them by
/// the wrong property.** It scopes declarations out "by the `schema/*` prefix,
/// and by accident rather than by design" — and two `schema/*` subjects are
/// declarations. The property that actually decides it is whether the type has a
/// default, which the type system already knows: `layers::config` requires
/// `T: Default`, so a declaration type cannot be routed here even by mistake.
///
/// So this scan is detection-first and hatch-to-exempt, the same direction as
/// `a_failed_kan_read_is_never_swallowed` and the fallback registry: a NEW
/// loader written on the direct path is an offender until someone states why,
/// and the statement sits at the call rather than in a document.
#[test]
fn every_direct_fenced_read_states_why_it_is_not_per_key() {
    const MARKER: &str = "not-per-key:";
    const LOOKBACK: usize = 15;

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    assert!(!files.is_empty(), "could not check: no sources under src/");

    let mut offenders = Vec::new();
    let mut checked = 0;
    for path in &files {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        // `atoms.rs` defines the reader; `layers.rs` IS the assembler.
        if name == "atoms.rs" || name == "layers.rs" {
            continue;
        }
        let text = std::fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            if !line.contains("newest_fenced::<") {
                continue;
            }
            checked += 1;
            let from = n.saturating_sub(LOOKBACK);
            if !lines[from..n].iter().any(|l| l.contains(MARKER)) {
                offenders.push(format!("{}:{}", path.display(), n + 1));
            }
        }
    }

    assert!(
        checked > 0,
        "could not check: no `newest_fenced::<` call sites found outside the \
         assembler. Either the reader was renamed or this scan stopped matching \
         — a scan that matches nothing reports clean by finding nothing, which \
         is the failure this repo records most often."
    );
    assert!(
        offenders.is_empty(),
        "these read a fenced block directly without saying why it is not \
         resolved per key: {offenders:?}\n\n\
         Add `// {MARKER} <why>` within {LOOKBACK} lines above the call, or \
         route the loader through `layers::config` / `layers::witness`. \
         REQ-17: seven loaders reimplementing an overlay is the shape day#101 \
         records three instances of."
    );
}
