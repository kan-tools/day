//! `.design/design-atom-backing.md` AC-1..AC-8 — the design/review/next
//! surface, driven through the real `day` binary against a stub kan.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{appends, atom_claim, claim, decision_claim, schema_claim, write_kan_stub};

const DOC: &str = "# Feature: a thing\n\n\
    ## Summary\nIt does the thing.\n\n\
    ## Requirements\n- REQ-1: first\n- REQ-2: second\n\n\
    ## Acceptance Criteria\n- [ ] AC-1: covers (REQ-1)\n- [ ] AC-2: covers (REQ-2)\n\n\
    ## Architecture\nTouches `src/design.rs`.\n\n\
    ## Resolved Questions\n- **Q1**: chose the first thing\n- **Q2**: chose the second thing\n";

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("failed to run day")
}

/// A workspace containing a design doc plus the `src/design.rs` path it
/// references, so path-existence checks have something real to resolve.
fn workspace(doc: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/design.rs"), "// fixture\n").unwrap();
    std::fs::create_dir_all(dir.path().join(".design")).unwrap();
    std::fs::write(dir.path().join(".design/thing.md"), doc).unwrap();
    dir
}

#[test]
fn ac1_check_passes_a_clean_doc_and_names_a_missing_section() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    assert!(
        out.status.success(),
        "clean doc should pass: {}",
        String::from_utf8_lossy(&out.stdout)
    );

    std::fs::write(
        dir.path().join(".design/thing.md"),
        DOC.replace("## Architecture", "## Notes"),
    )
    .unwrap();
    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("section missing: Architecture"), "{stdout}");
}

#[test]
fn ac2_revising_the_schema_claim_changes_the_result_with_no_file_edited() {
    let dir = workspace(DOC);
    let mut schema = day::schema::Schema::starter();
    schema.sections.push("Threat Model".to_string());
    let revised = claim(
        "schema/design-doc",
        "bafyreirevised",
        &format!(
            "Revised schema.\n\n```day-schema\n{}\n```\n",
            serde_json::to_string(&schema).unwrap()
        ),
    );
    let kan = write_kan_stub(
        dir.path(),
        &[schema_claim("design-doc", "bafyreischema"), revised],
    );

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "the newest schema claim should win");
    assert!(stdout.contains("section missing: Threat Model"), "{stdout}");
}

/// AC-3, amended during implementation: an uncovered requirement **warns**
/// rather than fails. Running this check over kan's own design docs showed
/// several map criteria to requirements positionally (AC-1 covers REQ-1)
/// instead of naming them — a convention the check cannot detect, so failing
/// on it would be failing on a false signal. Recorded as a decide claim on
/// the `design-atom-backing` subject rather than changed silently.
#[test]
fn ac3_an_uncovered_requirement_is_named() {
    let dir = workspace(&DOC.replace("- [ ] AC-2: covers (REQ-2)", "- [ ] AC-2: covers something"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("REQ-2 is not named by any acceptance criterion"),
        "{stdout}"
    );
    assert!(
        out.status.success(),
        "an unnamed requirement warns; it does not fail the document"
    );
}

#[test]
fn ac4_a_referenced_path_that_does_not_exist_is_named() {
    let dir = workspace(&DOC.replace("`src/design.rs`", "`src/nope.rs`"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("src/nope.rs"), "{stdout}");
}

#[test]
fn check_without_a_declared_schema_explains_and_offers_a_starter() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(dir.path(), &kan, &["design", "check", ".design/thing.md"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(
        stderr.contains("no design-doc schema is declared"),
        "{stderr}"
    );
    assert!(
        stderr.contains("kan observe") && stderr.contains("day-schema"),
        "the error should hand over a runnable starter: {stderr}"
    );
}

#[test]
fn ac5_record_appends_observe_then_plan_then_one_decide_per_resolved_question() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "record", ".design/thing.md"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let log = appends(dir.path());
    assert_eq!(
        log.len(),
        4,
        "expected observe + plan + 2 decides, got {log:#?}"
    );
    assert!(log[0].starts_with("observe "), "{}", log[0]);
    assert!(log[1].starts_with("plan "), "{}", log[1]);
    assert!(log[2].starts_with("decide "), "{}", log[2]);
    assert!(log[3].starts_with("decide "), "{}", log[3]);

    // The chain is real: plan cites the observe's CID, decides cite the
    // plan's -- and every --cites value is a CID, never a path.
    assert!(log[1].contains("--cites bafyreistub00000001"), "{}", log[1]);
    assert!(log[2].contains("--cites bafyreistub00000002"), "{}", log[2]);
    assert!(log[3].contains("--cites bafyreistub00000002"), "{}", log[3]);
    for line in &log {
        assert!(
            !line.contains("--cites .design"),
            "a file path must never be passed to --cites: {line}"
        );
    }
    assert!(log.iter().all(|l| l.contains("--subject thing")));
}

#[test]
fn ac6_a_failing_doc_is_still_recorded_with_the_result_embedded() {
    let dir = workspace(&DOC.replace("## Architecture", "## Notes"));
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(dir.path(), &kan, &["design", "record", ".design/thing.md"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "a failing doc must still record -- gating would just mean it goes unrecorded"
    );
    assert!(stdout.contains("recorded anyway"), "{stdout}");

    let log = appends(dir.path());
    let plan = log
        .iter()
        .find(|l| l.starts_with("plan "))
        .expect("a plan claim");
    assert!(
        plan.contains("failed") && plan.contains("validation:"),
        "the plan claim should carry the validation result: {plan}"
    );
}

#[test]
fn ac7_review_record_rejects_a_bad_verdict_and_an_uncited_one() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "thing",
            "--verdict",
            "LGTM",
            "--rationale",
            "looks fine",
            "--cites",
            "bafyreisomething",
        ],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("verdict must be one of"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(appends(dir.path()).is_empty(), "nothing should be appended");

    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "thing",
            "--verdict",
            "APPROVE",
            "--rationale",
            "fine",
        ],
    );
    assert!(
        !out.status.success(),
        "a verdict with no --cites must be rejected"
    );
    assert!(appends(dir.path()).is_empty());
}

#[test]
fn ac7_review_record_accepts_each_permitted_verdict() {
    for verdict in day::record::DEFAULT_VERDICTS {
        let dir = workspace(DOC);
        let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);
        let out = day(
            dir.path(),
            &kan,
            &[
                "review",
                "record",
                "thing",
                "--verdict",
                verdict,
                "--rationale",
                "because",
                "--cites",
                "bafyreidesignclaim",
            ],
        );
        assert!(out.status.success(), "verdict {verdict} should be accepted");
        let log = appends(dir.path());
        assert_eq!(log.len(), 1);
        assert!(log[0].contains(verdict), "{}", log[0]);
        assert!(log[0].contains("--cites bafyreidesignclaim"), "{}", log[0]);
    }
}

#[test]
fn ac8_next_names_the_successor_and_what_it_needs() {
    let dir = workspace(DOC);
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_claim(
                "design",
                "bafyreid1",
                &["intent"],
                &["design-doc"],
                &["generative-build"],
            ),
            atom_claim(
                "generative-build",
                "bafyreid2",
                &["design-doc"],
                &["code-change"],
                &["adversarial-review"],
            ),
            atom_claim(
                "adversarial-review",
                "bafyreid3",
                &["design-doc", "code-change"],
                &["verdict"],
                &[],
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["next", "design"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(stdout.contains("atom/generative-build"), "{stdout}");
    assert!(stdout.contains("design-doc"), "{stdout}");
    assert!(
        !stdout.contains("adversarial-review"),
        "next should report the immediate successor, not the whole pipeline: {stdout}"
    );

    let out = day(dir.path(), &kan, &["next", "adversarial-review"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(stdout.contains("no successors"), "{stdout}");
}

/// day#36: re-recording an iterated design must not re-append decisions already
/// on the subject.
///
/// `/design` explicitly supports iterating, and every iteration that resolves a
/// question adds a bullet — so before this, the second run rewrote every decide
/// from the first. On `.design/assess-telos.md` that would have been 10 decides,
/// 8 of them duplicates.
///
/// The key is a stable id, not the bullet's text. Text was the obvious choice
/// and breaks precisely when iterating: a sharpened wording records twice, and a
/// rewording that changed the *meaning* records once and is silently wrong.
#[test]
fn recording_an_iterated_design_is_incremental() {
    let dir = tempfile::tempdir().unwrap();
    let doc = dir.path().join("iterating.md");
    std::fs::write(
        &doc,
        "# Feature: Iterating\n\n## Summary\ns\n\n## Requirements\n\
         - REQ-1: a.\n- REQ-2: b.\n\n## Acceptance Criteria\n\
         - [ ] AC-1: REQ-1.\n- [ ] AC-2: REQ-2.\n\n## Architecture\n\
         In `src/probe.rs`.\n\n## Resolved Questions\n\
         - RQ-1: the first thing.\n- RQ-2: the second thing.\n",
    )
    .unwrap();

    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafysc")]);

    let first = day(
        dir.path(),
        &kan,
        &["design", "record", doc.to_str().unwrap()],
    );
    let first = String::from_utf8_lossy(&first.stdout);
    assert_eq!(
        first.matches("decide").count(),
        2,
        "the first record should append both decisions: {first}"
    );

    let second = day(
        dir.path(),
        &kan,
        &["design", "record", doc.to_str().unwrap()],
    );
    let second = String::from_utf8_lossy(&second.stdout);
    assert_eq!(
        second.matches("decide").count(),
        0,
        "the second record must append nothing — both ids are already on the \
         subject: {second}"
    );
    assert!(
        second.contains("RQ-1") && second.contains("RQ-2"),
        "and must say WHICH were skipped, since 'recorded 0' and 'recorded 0, \
         skipped 2' are different facts: {second}"
    );
}

/// **day#119 — the observe/plan pair was appended on every run.**
///
/// `design record` was idempotent for resolutions and not for the pair, so
/// three passes over one document left three `Observation`s and three `Plan`s
/// beside exactly one `Decision` per id. Non-destruction makes the cost
/// permanent: they can only be retracted, by hand, which is a workaround a user
/// should not have to know about.
///
/// **Both halves are checked, and that is the whole subtlety.** The observe text
/// carries the validation report's summary and the plan text carries the
/// document's own summary line, so they change on different edits. A first
/// version decided from the observe half alone and reported `(unchanged)` for a
/// document whose Summary had been rewritten — caught by running it, against a
/// comment asserting the two were derived from the same thing.
#[test]
fn an_unchanged_design_pass_records_no_second_observe_or_plan() {
    let dir = tempfile::tempdir().unwrap();
    let doc = dir.path().join("steady.md");
    let body = |summary: &str| {
        format!(
            "# Feature: Steady\n\n## Summary\n{summary}\n\n## Requirements\n\
             - REQ-1: a.\n- REQ-2: b.\n\n## Acceptance Criteria\n\
             - [ ] AC-1: REQ-1.\n- [ ] AC-2: REQ-2.\n\n## Architecture\n\
             In `src/probe.rs`.\n\n## Resolved Questions\n\
             - RQ-1: the first thing.\n"
        )
    };
    std::fs::write(&doc, body("the original summary")).unwrap();

    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafysc")]);
    let run = |kan: &std::path::Path| {
        String::from_utf8_lossy(
            &day(
                dir.path(),
                kan,
                &["design", "record", doc.to_str().unwrap()],
            )
            .stdout,
        )
        .to_string()
    };

    let first = run(&kan);
    assert!(
        !first.contains("(unchanged)"),
        "the first pass has nothing to be unchanged from: {first}"
    );

    let second = run(&kan);
    assert_eq!(
        second.matches("(unchanged)").count(),
        2,
        "an identical document must append neither half, and must say so \
         rather than printing the existing CIDs as though they were written: \
         {second}"
    );

    // The counts are untouched, so the *report summary* — and with it the
    // observe text — is identical. Only the plan text moves.
    std::fs::write(&doc, body("a completely rewritten summary")).unwrap();
    let third = run(&kan);
    assert!(
        !third.contains("(unchanged)"),
        "a document whose Summary changed is a design that changed, and must \
         record a new pair even though the validation report is identical — \
         deciding from the observe half alone loses exactly this edit: {third}"
    );

    // **The case a cold review found, and the reason both texts were not
    // enough.** Reverse a requirement's meaning: the finding counts do not
    // move, so the observe text is identical, and the Summary section is
    // untouched, so the plan text is identical too. Both claim texts are
    // *summaries*; only a fingerprint over the source can see this.
    let reversed = body("the original summary").replace(
        "- REQ-1: a.",
        "- REQ-1: the system stores NOTHING of its own.",
    );
    std::fs::write(&doc, &reversed).unwrap();
    run(&kan);
    let flipped = reversed.replace(
        "- REQ-1: the system stores NOTHING of its own.",
        "- REQ-1: the system stores EVERYTHING in a sidecar database.",
    );
    // premise: this edit really is invisible to both summaries. Asserted on the
    // fixture rather than assumed, so a later change to what the texts contain
    // makes this test say so instead of passing for a new reason.
    assert_eq!(
        reversed.lines().count(),
        flipped.lines().count(),
        "premise: the edit must not change the document's shape, only a \
         requirement's meaning"
    );
    std::fs::write(&doc, &flipped).unwrap();
    let fourth = run(&kan);
    assert!(
        !fourth.contains("(unchanged)"),
        "a requirement reversed in meaning is a design that changed. Reporting \
         `(unchanged)` here records nothing on an append-only log AND asserts \
         to the user that there was nothing to record — the only finding in \
         this branch where day states something false rather than staying \
         silent: {fourth}"
    );
}

/// The negative control, and the backward-compatibility half: a document with no
/// ids behaves exactly as it did before day#36 — every bullet recorded, every
/// time — and day says so rather than leaving the duplication to be discovered.
#[test]
fn a_document_without_ids_still_records_every_time_and_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let doc = dir.path().join("unidentified.md");
    std::fs::write(
        &doc,
        "# Feature: No ids\n\n## Summary\ns\n\n## Requirements\n\
         - REQ-1: a.\n- REQ-2: b.\n\n## Acceptance Criteria\n\
         - [ ] AC-1: REQ-1.\n- [ ] AC-2: REQ-2.\n\n## Architecture\n\
         In `src/probe.rs`.\n\n## Resolved Questions\n\
         - the first thing.\n- the second thing.\n",
    )
    .unwrap();

    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafysc")]);

    for pass in 1..=2 {
        let out = day(
            dir.path(),
            &kan,
            &["design", "record", doc.to_str().unwrap()],
        );
        let out = String::from_utf8_lossy(&out.stdout);
        assert_eq!(
            out.matches("decide").count(),
            2,
            "pass {pass}: an un-idded document keeps its pre-day#36 behaviour: {out}"
        );
        assert!(
            out.contains("carry no id"),
            "pass {pass}: and day must say the duplication is coming: {out}"
        );
    }
}

/// day#41: a decision recorded on the subject that the document does not carry
/// is reported.
///
/// The scenario is kan's v0.7 release, where it cost two decided requirements:
/// five items were decided and recorded as `decide` claims, the release was
/// re-scoped, a new design doc was written **from the session rather than from
/// the log**, and two fell through. Nothing rejected them — one was recovered
/// hours later by accident.
///
/// Exact rather than heuristic, because day#36's ids make it so. Matching prose
/// to a requirement would either miss the case this exists for or cry wolf until
/// it was switched off.
#[test]
fn a_decision_on_the_record_that_the_document_dropped_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/probe.rs"), "// fixture\n").unwrap();
    let doc = dir.path().join("rescoped.md");

    // The document carries RQ-1 only. RQ-2 was decided and is on the record.
    std::fs::write(
        &doc,
        "# Feature: Rescoped\n\n## Summary\ns\n\n## Requirements\n\
         - REQ-1: a.\n- REQ-2: b.\n\n## Acceptance Criteria\n\
         - [ ] AC-1: REQ-1.\n- [ ] AC-2: REQ-2.\n\n## Architecture\n\
         In `src/probe.rs`.\n\n## Resolved Questions\n- RQ-1: kept.\n",
    )
    .unwrap();

    let kan = write_kan_stub(
        dir.path(),
        &[
            schema_claim("design-doc", "bafysc"),
            decision_claim("rescoped", "bafyd1", "RQ-1: kept.", 10),
            decision_claim("rescoped", "bafyd2", "RQ-2: dropped in the re-scope.", 20),
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &["design", "check", doc.to_str().unwrap()],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("RQ-2"),
        "the dropped decision should be named: {stdout}"
    );
    assert!(
        !stdout.contains("RQ-1,") && !stdout.contains(": RQ-1"),
        "and the one the document DOES carry must not be reported — a check that \
         lists everything is a check nobody reads: {stdout}"
    );
}

/// The negative control: a document covering every recorded decision reports
/// none of them. Without this, the test above would pass if day reported every
/// decision unconditionally.
#[test]
fn a_document_covering_the_record_reports_nothing() {
    let dir = tempfile::tempdir().unwrap();
    // The Architecture section references this, and day validates that a design
    // is grounded in real code. Created so this test fails on day#41's behaviour
    // rather than on an unrelated path check.
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/probe.rs"), "// fixture\n").unwrap();
    let doc = dir.path().join("complete.md");
    std::fs::write(
        &doc,
        "# Feature: Complete\n\n## Summary\ns\n\n## Requirements\n\
         - REQ-1: a.\n- REQ-2: b.\n\n## Acceptance Criteria\n\
         - [ ] AC-1: REQ-1.\n- [ ] AC-2: REQ-2.\n\n## Architecture\n\
         In `src/probe.rs`.\n\n## Resolved Questions\n- RQ-1: kept.\n- RQ-2: also kept.\n",
    )
    .unwrap();

    let kan = write_kan_stub(
        dir.path(),
        &[
            schema_claim("design-doc", "bafysc"),
            decision_claim("complete", "bafyd1", "RQ-1: kept.", 10),
            decision_claim("complete", "bafyd2", "RQ-2: also kept.", 20),
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &["design", "check", doc.to_str().unwrap()],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("not in the document"),
        "every recorded decision is covered, so nothing should be reported: {stdout}"
    );
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// **day#158 — a design pass cited an adversarial review's finding as the claim
/// it superseded.**
///
/// `newest_of_kind` took the newest `Observation` of ANY kind on the subject,
/// and `skills/adversarial-review/SKILL.md` records every finding as
/// `kan observe "<finding>" --subject <subject>` — on the design subject. So
/// after a review the "previous pair" was a review finding, which lacks the
/// document fingerprint. Three failures followed in one run over an UNCHANGED
/// document: a false migration note, a second live observe/plan pair (day#119
/// re-broken), and a citation asserting the design pass superseded the review's
/// finding — into a log day cannot retract from, at exit 0.
///
/// It fires on this repo's own prescribed workflow: `/design` → `day design
/// record` → `/adversarial-review`. day's `witness-model` subject already shows
/// the near-miss, with a `Result` sitting between two design passes.
///
/// The fix selects by what the mechanism WRITES — an opening day emits and
/// nothing else does — shared with the `format!` that writes it so the two
/// cannot drift.
#[test]
fn an_intervening_claim_is_not_mistaken_for_the_previous_design_pass() {
    let dir = tempfile::tempdir().unwrap();
    let doc = dir.path().join("thing.md");
    std::fs::write(
        &doc,
        "# Feature: thing\n\n## Summary\ns\n\n## Requirements\n- REQ-1: a\n- REQ-2: b\n\n\
         ## Acceptance Criteria\n- [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
         In `src/probe.rs`.\n\n## Resolved Questions\n- RQ-1: chose a\n",
    )
    .unwrap();

    let claims = vec![schema_claim("design-doc", "bafysc")];
    let kan = write_kan_stub(dir.path(), &claims);
    let run = || {
        String::from_utf8_lossy(
            &day(
                dir.path(),
                &kan,
                &["design", "record", doc.to_str().unwrap()],
            )
            .stdout,
        )
        .to_string()
    };

    run();
    // premise: without anything intervening, a second pass skips. If this ever
    // stops holding, the assertion below cannot tell the fix from the ordinary
    // path and would pass for the wrong reason.
    let second = run();
    assert_eq!(
        second.matches("(unchanged)").count(),
        2,
        "premise: an unchanged document must skip both halves: {second}"
    );

    // A review finding, recorded exactly as `/adversarial-review` prescribes —
    // an `Observation`, on the design subject, newer than the pair.
    let out = std::process::Command::new(&kan)
        .args([
            "observe",
            "BLOCK-1. The fix keys on a shape real kan never emits.",
            "--subject",
            "thing",
        ])
        .current_dir(dir.path())
        .output()
        .expect("the stub should accept a write");
    assert!(out.status.success(), "the fixture's own write must succeed");

    let third = run();
    assert_eq!(
        third.matches("(unchanged)").count(),
        2,
        "an intervening review finding is not a design pass — the document did \
         not change, so nothing may be appended and nothing may be cited as \
         superseded: {third}"
    );
    assert!(
        !third.contains("predates the document fingerprint"),
        "and the migration note must not fire: the fingerprinted pair is still \
         there, one claim further back: {third}"
    );
}
