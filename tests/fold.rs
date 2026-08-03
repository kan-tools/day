//! day#115 — **an assessment is never a declaration, on every vocabulary
//! subject day reads.**
//!
//! The rule was fixed three times for one subject each, and each fix left the
//! others: `render_teloi` (the telos statement), `docs::unrecorded_boundary`
//! (the release subject), then nothing for two milestones while
//! `tension/<a>--<b>` rendered an assessment as the reason two teloi pull
//! apart and `practice` injected one into every session as guidance.
//!
//! So this test is a **table over subject kinds**, not a case. Adding a
//! vocabulary subject to day means adding a row, which is the point: the next
//! one inherits the rule instead of rediscovering it. A per-subject test would
//! have passed for `telos/*` throughout the entire period both live defects
//! existed — it did, and they did.
//!
//! The fixtures drive the **session-start surface** rather than the fold
//! directly. `src/fold.rs` unit-tests the fold; what those cannot show is
//! whether a surface actually calls it, which is day#101's recurring defect and
//! exactly what was wrong here — the fold existed and two modules did not use
//! it.

#![cfg(unix)]

mod common;

use std::process::Command;

use common::{claim, result_claim, write_kan_stub, StubClaim};

/// One vocabulary subject, with a declaration and an assessment landing on it.
struct Row {
    /// What the subject kind is called, for failure messages.
    what: &'static str,
    claims: Vec<StubClaim>,
    /// Text that must appear: the declaration.
    declaration: &'static str,
    /// Text that must NOT appear: the assessment, which must never be rendered
    /// as though it were the declaration.
    assessment: &'static str,
}

/// Every vocabulary subject day folds prose out of.
///
/// `atom/*`, `bridge/*` and `schema/*` are deliberately absent: they resolve
/// through `newest_fenced`/`extract_interface`, which key on a **fenced block**
/// rather than on recency, so a `Result` carrying no block is already skipped.
/// That is real robustness but it is not this rule doing the work — the block
/// doubles as a type tag — and `a_block_keyed_subject_is_robust_for_its_own
/// _reason` below pins it so the two cannot be confused.
fn rows() -> Vec<Row> {
    vec![
        Row {
            what: "telos/<slug> statement",
            claims: vec![
                claim("telos/legible", "bafyt1", "THE TELOS STATEMENT ITSELF."),
                result_claim("telos/legible", "bafyt2", "AN ASSESSMENT OF THE TELOS.", 9),
            ],
            declaration: "THE TELOS STATEMENT ITSELF.",
            assessment: "AN ASSESSMENT OF THE TELOS.",
        },
        Row {
            what: "tension/<a>--<b> reason",
            claims: vec![
                claim("telos/a", "bafya", "Telos A."),
                claim("telos/b", "bafyb", "Telos B."),
                claim(
                    "tension/a--b",
                    "bafyx1",
                    "THE REASON THEY PULL APART.\n\n```day-tension\n{\"between\":[\"telos/a\",\"telos/b\"]}\n```\n",
                ),
                result_claim("tension/a--b", "bafyx2", "AN ASSESSMENT OF THE TENSION.", 9),
            ],
            declaration: "THE REASON THEY PULL APART.",
            assessment: "AN ASSESSMENT OF THE TENSION.",
        },
        Row {
            what: "practice items",
            claims: vec![
                claim("practice", "bafyp1", "A REAL PRACTICE ITEM."),
                result_claim("practice", "bafyp2", "AN ASSESSMENT OF PRACTICE.", 9),
            ],
            declaration: "A REAL PRACTICE ITEM.",
            assessment: "AN ASSESSMENT OF PRACTICE.",
        },
    ]
}

fn session_start(claims: &[StubClaim]) -> String {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), claims);
    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["hook", "session-start"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The rule, over the table.
///
/// Both halves matter and the second is the one that catches a lazy fix:
/// dropping every `Result` would satisfy "the assessment is not rendered" while
/// silently losing the declaration on any subject where the fold went wrong in
/// the other direction.
#[test]
fn an_assessment_is_never_a_declaration() {
    for row in rows() {
        let rendered = session_start(&row.claims);
        assert!(
            rendered.contains(row.declaration),
            "{}: the declaration vanished — a fold that renders nothing for a \
             subject that plainly has one is worse than the defect being fixed\n{rendered}",
            row.what
        );
        assert!(
            !rendered.contains(row.assessment),
            "{}: an assessment is being rendered as the declaration — day#115, \
             on the surface the model reads every session\n{rendered}",
            row.what
        );
    }
}

/// The premise, asserted rather than assumed: each fixture really does put an
/// assessment on the subject *after* the declaration.
///
/// Without this the table above passes on any fixture whose `Result` claim
/// failed to land — and "the assessment is not rendered" is trivially true of
/// an assessment that does not exist. Three tests in this repo have passed
/// against the defect they were named for because the fixture could not reach
/// the failing state; this is the cheap guard against being the fourth.
#[test]
fn every_row_actually_lands_an_assessment_after_the_declaration() {
    for row in rows() {
        let subjects: Vec<&str> = row
            .claims
            .iter()
            .filter(|c| c.kind == day::fold::ASSESSMENT)
            .map(|c| c.subject.as_str())
            .collect();
        assert_eq!(
            subjects.len(),
            1,
            "{}: expected exactly one assessment in the fixture, got {subjects:?}",
            row.what
        );
        let assessment_at = row
            .claims
            .iter()
            .position(|c| c.kind == day::fold::ASSESSMENT)
            .unwrap();
        let declaration_at = row
            .claims
            .iter()
            .position(|c| c.subject == subjects[0] && c.text.contains(row.declaration))
            .unwrap_or_else(|| panic!("{}: no declaration claim in the fixture", row.what));
        assert!(
            assessment_at > declaration_at,
            "{}: the assessment must be NEWER than the declaration, or recency \
             and role agree and the fixture proves nothing",
            row.what
        );
    }
}

/// `atom/*` is robust for a different reason, and the difference is worth
/// pinning so a later change cannot quietly remove one and rely on the other.
///
/// Its interface comes from the newest claim carrying a `day-atom` **block**,
/// so a `Result` is skipped because it has no block — not because of any rule
/// about kinds. If atoms ever grow a prose field folded by recency, this test
/// keeps passing and `an_assessment_is_never_a_declaration` is where the new
/// row belongs.
#[test]
fn a_block_keyed_subject_is_robust_for_its_own_reason() {
    let rendered = session_start(&[
        claim(
            "atom/build",
            "bafyb1",
            "The build atom.\n\n```day-atom\n{\"in\":[],\"out\":[\"code-change\"],\"next\":[]}\n```\n",
        ),
        result_claim("atom/build", "bafyb2", "AN ASSESSMENT OF THE ATOM.", 9),
    ]);
    assert!(rendered.contains("atom/build"), "{rendered}");
    assert!(
        !rendered.contains("AN ASSESSMENT OF THE ATOM."),
        "{rendered}"
    );
}
