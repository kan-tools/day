//! `.design/honest-reads.md` REQ-9, forward direction (AC-10): **this build
//! reads every block shape any released version wrote.**
//!
//! That is a guarantee rather than a characterization, so it is asserted here —
//! hermetic, no network, no old binaries — and runs on every push. The backward
//! direction (a released binary reading *this* commit's shapes) cannot be a
//! guarantee, because a released binary cannot be fixed; it is a characterization
//! and lives in `.github/workflows/migration-matrix.yml`, on release.
//!
//! The corpus is **generated, not written**: `scripts/capture-block-corpus.sh`
//! builds each released tag and drives that tag's own `day` binary, capturing
//! what it actually appended. A hand-written corpus could only record what a
//! maintainer believes an old version wrote, which is not evidence of anything.
//!
//! This is the test day#78 needed. `v0.7.0-beta.1` silently drops the narrowing
//! predicates day#70 added — nothing caught it, because every other test runs one
//! binary against its own shapes.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/block-corpus")
}

struct Captured {
    tag: String,
    fence: String,
    body: serde_json::Value,
}

fn load_corpus() -> Vec<Captured> {
    let mut out = Vec::new();
    let dir = corpus_dir();
    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("corpus dir {} is unreadable: {e}", dir.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }
        let tag = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        let text = std::fs::read_to_string(&path).unwrap();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let value: serde_json::Value = serde_json::from_str(line)
                .unwrap_or_else(|e| panic!("{tag}: corpus line is not JSON: {e}\n{line}"));
            out.push(Captured {
                tag: tag.clone(),
                fence: value["fence"].as_str().expect("fence").to_string(),
                body: value["body"].clone(),
            });
        }
    }
    out
}

/// Deserializing a captured body into the type this build uses for that fence.
/// Returns the error message on failure, so a break names the shape and the
/// version that wrote it rather than just failing.
fn resolve(fence: &str, body: &serde_json::Value) -> Result<(), String> {
    fn attempt<T: serde::de::DeserializeOwned>(body: &serde_json::Value) -> Result<(), String> {
        serde_json::from_value::<T>(body.clone())
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
    match fence {
        "day-atom" => attempt::<day::atoms::Interface>(body),
        "day-telos" => attempt::<day::bridge::Witnesses>(body),
        "day-bridge" => attempt::<day::bridge::Plan>(body),
        "day-witness" => attempt::<day::telos::WitnessSchema>(body),
        "day-schema" => attempt::<day::schema::Schema>(body),
        "day-docs" => attempt::<day::docs::DocsSchema>(body),
        "day-tension" => attempt::<day::tension::Tension>(body),
        other => Err(format!(
            "the corpus holds a `{other}` block this test does not know how to \
             resolve — add it to `resolve`, because a fence nobody checks is a \
             fence nobody is protecting"
        )),
    }
}

/// AC-10: every shape any released version wrote still resolves under this
/// build.
///
/// This is the requirement that makes `deny_unknown_fields` safe to add. Turning
/// a tolerant reader strict risks breaking exactly this, and the risk is
/// invisible to a suite whose fixtures were all written against the current
/// shape.
#[test]
fn ac10_this_build_resolves_every_shape_a_released_version_wrote() {
    let corpus = load_corpus();
    assert!(
        !corpus.is_empty(),
        "the corpus is empty — regenerate it with scripts/capture-block-corpus.sh"
    );

    let mut failures = Vec::new();
    for Captured { tag, fence, body } in &corpus {
        if let Err(e) = resolve(fence, body) {
            failures.push(format!(
                "  {tag} wrote a `{fence}` this build cannot read: {e}\n    {}",
                serde_json::to_string(body).unwrap_or_default()
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "backward compatibility broke for {} shape(s):\n{}\n\nA released version \
         wrote these. If a field was genuinely removed rather than made optional, \
         that is a breaking change to a log format, not a refactor.",
        failures.len(),
        failures.join("\n")
    );
}

/// The corpus has to actually span day's release history, or the test above
/// passes by covering nothing. Pins the versions and the fences present, so
/// silently losing coverage fails rather than quietly shrinking the guarantee.
#[test]
fn ac10_the_corpus_spans_the_release_history_it_claims_to() {
    let corpus = load_corpus();
    let tags: BTreeSet<&str> = corpus.iter().map(|c| c.tag.as_str()).collect();
    let fences: BTreeSet<&str> = corpus.iter().map(|c| c.fence.as_str()).collect();

    // v0.1.x wrote no capturable block, so the corpus starts at v0.2.
    for expected in [
        "v0.2.0-beta.1",
        "v0.3.0-beta.1",
        "v0.4.0-beta.1",
        "v0.5.0-beta.1",
        "v0.6.0-beta.1",
        "v0.7.0-beta.1",
    ] {
        assert!(
            tags.contains(expected),
            "the corpus lost coverage of {expected}: {tags:?}"
        );
    }

    // **All seven** block types day owns (day#87). The corpus reached only four
    // until the capture stub could serve its own writes back: `bridge declare`
    // resolves the atoms its plan names and `telos tension` reads both subjects,
    // so against a write-only stub those verbs were refused and the blocks they
    // would have written never existed to capture. The gap mattered more than its
    // size suggested — `day-bridge` and `day-witness` are the block types whose
    // readers changed most recently (day#34's scope, day#70's `ClaimShape`, which
    // is what day#78 is about), so the uncaptured half was the half with the most
    // history.
    //
    // Listed exhaustively rather than as a count: a count would still pass if one
    // fence were swapped for another.
    for expected in [
        "day-atom",
        "day-telos",
        "day-bridge",
        "day-witness",
        "day-schema",
        "day-docs",
        "day-tension",
    ] {
        assert!(
            fences.contains(expected),
            "the corpus lost coverage of `{expected}`: {fences:?}. If a capture verb \
             stopped writing this block, it fails SILENTLY — the verb is refused, \
             nothing is appended, and the fence simply does not appear. That is how \
             this gap opened the first time."
        );
    }
}

/// The shapes in the corpus must be the ones a released version actually wrote,
/// which means they must not carry `_version` — versioning did not exist in any
/// released version. A corpus row with a `_version` is a sign the fixtures were
/// regenerated against an unreleased build and no longer describe history.
#[test]
fn ac10_no_captured_shape_carries_a_version_no_release_could_write() {
    for Captured { tag, fence, body } in load_corpus() {
        if let Some(object) = body.as_object() {
            assert!(
                !object.contains_key(day::atoms::VERSION_KEY),
                "{tag}'s `{fence}` carries `{}`, which no released version could \
                 write — the corpus was captured from an unreleased build",
                day::atoms::VERSION_KEY
            );
        }
    }
}

/// `.design/forward-only-next.md` AC-15 — the block the **migration matrix**
/// calls "from the future" is actually from the future.
///
/// The matrix (`.github/workflows/migration-matrix.yml`) hands
/// `tests/fixtures/migration-blocks.json` to every released `day` and records
/// what each one does with it. That measurement means "a released reader,
/// handed a block it cannot read, refuses honestly" only while the block is
/// genuinely unreadable — and it was pinned at `_version: 2`, which day#113
/// made a version day reads. Nothing in the matrix would have noticed: every
/// released binary still refuses it (they are all v1), so the rows would stay
/// green while the cell quietly stopped testing what it is named for.
///
/// Asserted against `SUPPORTED_VERSION` rather than a literal, because a
/// literal is exactly what went stale. This is CLAUDE.md's rule for
/// verification tooling — a check must be able to tell "could not read" from
/// "read it fine" — applied to the fixture the check is built on.
#[test]
fn a_from_the_future_block_is_actually_from_the_future() {
    use day::atoms::Versioned;

    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/migration-blocks.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()));
    let blocks: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");

    let mut checked = 0;
    for block in blocks.as_array().expect("an array of blocks") {
        let Some(version) = block.pointer("/body/_version").and_then(|v| v.as_u64()) else {
            continue;
        };
        checked += 1;
        let fence = block["fence"].as_str().unwrap_or_default();
        assert_eq!(
            fence,
            day::atoms::Interface::FENCE,
            "this assertion only knows `day-atom`'s supported version; a versioned \
             `{fence}` fixture needs its own bound here"
        );
        assert!(
            version > day::atoms::Interface::SUPPORTED_VERSION,
            "the migration fixture declares {} {version}, which THIS build reads (up to \
             {}) — so the cell named for a block from the future is measuring a block \
             every reader can parse. Raise it above `SUPPORTED_VERSION`.",
            day::atoms::VERSION_KEY,
            day::atoms::Interface::SUPPORTED_VERSION,
        );
    }
    assert_eq!(
        checked, 1,
        "exactly one migration fixture is supposed to be from the future; \
         finding none means the assertion above ran on nothing"
    );
}
