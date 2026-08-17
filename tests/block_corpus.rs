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
use std::process::Command;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/block-corpus")
}

#[test]
fn current_corpus_capture_isolated_commit_ignores_maintainer_signing_policy() {
    let dir = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(dir.path())
            .output()
            .unwrap()
    };
    assert!(run(&["init", "-q"]).status.success());
    assert!(run(&["config", "user.name", "corpus test"])
        .status
        .success());
    assert!(run(&["config", "user.email", "corpus@example.invalid"])
        .status
        .success());
    assert!(run(&["config", "commit.gpgsign", "true"]).status.success());
    assert!(run(&["config", "gpg.program", "false"]).status.success());
    assert!(
        !run(&["commit", "-q", "--allow-empty", "-m", "unsigned fixture"])
            .status
            .success(),
        "premise: inherited signing policy must make the disposable commit fail"
    );
    assert!(run(&[
        "-c",
        "commit.gpgsign=false",
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        "unsigned fixture",
    ])
    .status
    .success());
    let script = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/capture-block-corpus.sh"),
    )
    .unwrap();
    assert!(script.contains("git -c commit.gpgsign=false commit"));
    let migration = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/run-migration-cell.sh"),
    )
    .unwrap();
    assert!(migration.contains("git -c commit.gpgsign=false commit"));
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
///
/// Through `day::atoms::parse_block` — the production entry point — not raw
/// serde. The raw form re-implemented the read without the version gate, so
/// the `_version: 2` atoms v0.10+ really write reported as unreadable here
/// while the shipped reader reads them fine: a corpus harness validating the
/// corpus against its own idea of the reader, which is the stub-test failure
/// mode this suite exists to avoid.
fn resolve(fence: &str, body: &serde_json::Value) -> Result<(), String> {
    fn attempt<T: serde::de::DeserializeOwned + day::atoms::Versioned>(
        body: &serde_json::Value,
    ) -> Result<(), String> {
        day::atoms::parse_block::<T>(&serde_json::to_string(body).map_err(|e| e.to_string())?)
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

/// The released tags the corpus must cover, derived from
/// `tests/fixtures/migration-expectations.tsv` — the committed list
/// `scripts/cut-release.sh` refuses to leave incomplete — rather than from
/// `git tag`, which a shallow CI checkout does not have, or from a literal
/// list, which is how the corpus went stale by eleven releases with every
/// check green (2026-08-10 review, finding 2): the pin list was a floor that
/// never grew, in the test carrying the derivation lesson in its own doc
/// comment.
fn released_tags() -> Vec<String> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/migration-expectations.tsv");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()));
    let tags: Vec<String> = text
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| l.split('\t').next().map(str::to_string))
        .collect();
    assert!(
        !tags.is_empty(),
        "migration-expectations.tsv lists no released tags, so corpus \
         completeness cannot mean anything"
    );
    tags
}

/// `v<major>.<minor>.<patch>-beta.<n>` as a sortable key. Every released day
/// tag has this shape; refusing anything else is deliberate — a tag this
/// cannot order would silently break the monotone-coverage assertion below.
fn semver_key(tag: &str) -> (u64, u64, u64, u64) {
    let parse = |s: &str| -> u64 {
        s.parse()
            .unwrap_or_else(|_| panic!("unparseable release tag component {s:?} in {tag:?}"))
    };
    let rest = tag
        .strip_prefix('v')
        .unwrap_or_else(|| panic!("tag {tag:?} lacks the v prefix"));
    let (triplet, beta) = match rest.split_once("-beta.") {
        Some((t, b)) => (t, parse(b)),
        None => (rest, u64::MAX), // a stable release orders after its betas
    };
    let mut parts = triplet.split('.').map(parse);
    (
        parts.next().expect("major"),
        parts.next().expect("minor"),
        parts.next().expect("patch"),
        beta,
    )
}

/// Tags with a release row and no corpus file, each with the measured reason.
/// v0.1.x had no write verbs at all — driven by the capture script, they
/// append nothing, and an empty fixture claiming to be a version's shapes is
/// worse than none.
const WROTE_NO_BLOCKS: &[&str] = &["v0.1.1-beta.1", "v0.1.2-beta.1"];

/// The corpus has to actually span day's release history, or the resolve test
/// passes by covering nothing. Coverage is derived, not pinned: every tag the
/// expectations file records is expected here, so a new release without a
/// corpus row fails on the push that adds its expectations row —
/// `cut-release.sh` measures both before tagging — instead of never.
#[test]
fn ac10_the_corpus_spans_the_release_history_it_claims_to() {
    let corpus = load_corpus();
    let tags: BTreeSet<&str> = corpus.iter().map(|c| c.tag.as_str()).collect();
    let fences: BTreeSet<&str> = corpus.iter().map(|c| c.fence.as_str()).collect();

    let expected: Vec<String> = released_tags()
        .into_iter()
        .filter(|t| !WROTE_NO_BLOCKS.contains(&t.as_str()))
        .collect();
    let missing: Vec<&String> = expected
        .iter()
        .filter(|t| !tags.contains(t.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "the corpus has no file for released tag(s) {missing:?} — the \
         guarantee \"this build resolves every shape any released version \
         wrote\" is unverified for them. Regenerate with \
         scripts/capture-block-corpus.sh (or, for a tag being cut, \
         `--current <tag>`)."
    );
    let unexpected: Vec<&str> = tags
        .iter()
        .filter(|t| !expected.iter().any(|e| e == *t))
        .copied()
        .collect();
    assert!(
        unexpected.is_empty(),
        "the corpus holds file(s) for {unexpected:?}, which \
         migration-expectations.tsv does not record as released — either the \
         release row is missing (cut-release.sh refuses that state) or the \
         corpus file describes a version that does not exist"
    );

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

/// **A fence a release captured never disappears from a later release.**
///
/// The union check above is satisfied by OLD tags carrying a fence, so it
/// cannot see a fence vanishing from every tag after some point — which is
/// exactly what happened: the capture stub predated `show --all`, so from
/// v0.8.0-beta.1 (the release that moved reads to the day#71 bulk read)
/// `day-bridge`, `day-witness` and `day-tension` silently dropped out of the
/// capture for every subsequent tag, and the guarantee quietly narrowed to
/// shapes last written five releases ago. The generator's failure mode is
/// less output with no error; this is the check keyed on the positive signal.
///
/// day has never removed a write verb, so coverage must be cumulative. If a
/// verb is ever deliberately retired, this test is where that decision gets
/// recorded — as an explicit exception with the release that retired it, not
/// by deleting the assertion.
#[test]
fn a_fence_a_release_captured_never_disappears_from_a_later_release() {
    let corpus = load_corpus();
    let mut tags: Vec<&str> = corpus
        .iter()
        .map(|c| c.tag.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    tags.sort_by_key(|t| semver_key(t));

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for tag in tags {
        let here: BTreeSet<&str> = corpus
            .iter()
            .filter(|c| c.tag == tag)
            .map(|c| c.fence.as_str())
            .collect();
        let lost: Vec<&&str> = seen.iter().filter(|f| !here.contains(**f)).collect();
        assert!(
            lost.is_empty(),
            "{tag} captured no {lost:?}, which earlier release(s) did capture. \
             Either that verb broke at this version — worth knowing — or the \
             capture scenario stopped reaching it, which is how three fences \
             went missing for five releases with the union check green."
        );
        seen.extend(here);
    }
}

/// The shapes in the corpus must be ones the writing version could actually
/// write. A `_version` is legitimate only where the release that introduced
/// that (fence, version) pair is at or below the tag that wrote it — the
/// premise "versioning did not exist in any released version" was true when
/// this test was written and false since v0.10.0-beta.1, which deadlocked the
/// corpus: a faithful regeneration tripped the old assertion, so the corpus
/// could not be regenerated without editing a test, and was not (2026-08-10
/// review, finding 2).
#[test]
fn ac10_no_captured_shape_carries_a_version_its_release_could_not_write() {
    // (fence, version) -> the release that introduced writing it. Append-only:
    // a new versioned shape adds a row here when the writer ships.
    let introduced: &[(&str, u64, &str)] = &[("day-atom", 2, "v0.10.0-beta.1")];

    for Captured { tag, fence, body } in load_corpus() {
        let Some(version) = body.get(day::atoms::VERSION_KEY).and_then(|v| v.as_u64()) else {
            continue;
        };
        let Some((_, _, intro)) = introduced
            .iter()
            .find(|(f, v, _)| *f == fence && *v == version)
        else {
            panic!(
                "{tag}'s `{fence}` carries `{}: {version}`, a versioned shape this \
                 test does not know a writer for — if a release really writes it, \
                 add its introduction row; otherwise the corpus was captured from \
                 an unreleased build",
                day::atoms::VERSION_KEY
            );
        };
        assert!(
            semver_key(&tag) >= semver_key(intro),
            "{tag}'s `{fence}` carries `{}: {version}`, which was not writable \
             until {intro} — the corpus row cannot be what that release wrote",
            day::atoms::VERSION_KEY
        );
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
