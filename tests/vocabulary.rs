//! `.design/vocabulary-verbs.md` AC-1..AC-10 — the declaration verbs, `day
//! init`'s baseline setup, and the session-end hook, driven through the real
//! binary against a stub kan.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{appends, claim, missing_kan, schema_claim, write_kan_stub};

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("failed to run day")
}

#[test]
fn ac1_declaring_a_new_telos_cites_nothing_and_redeclaring_cites_the_prior_claim() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "legible-process",
            "The record suffices.",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = appends(dir.path());
    assert_eq!(log.len(), 1);
    assert!(log[0].starts_with("decide "), "{}", log[0]);
    assert!(
        log[0].contains("--subject telos/legible-process"),
        "{}",
        log[0]
    );
    assert!(
        !log[0].contains("--cites"),
        "a first declaration cites nothing: {}",
        log[0]
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("declared"),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );

    // Redeclaring against a log that already has a claim is revision.
    let kan = write_kan_stub(
        dir.path(),
        &[claim(
            "telos/legible-process",
            "bafyreiprior",
            "The record suffices.",
        )],
    );
    let out = day(
        dir.path(),
        &kan,
        &["telos", "declare", "legible-process", "Revised statement."],
    );
    assert!(out.status.success());
    let log = appends(dir.path());
    let last = log.last().unwrap();
    assert!(last.contains("--cites bafyreiprior"), "{last}");
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("revised"),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn ac2_title_and_kind_are_passed_through() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "x",
            "A statement.",
            "--title",
            "X",
            "--kind",
            "idea",
        ],
    );
    assert!(out.status.success());
    let log = appends(dir.path());
    assert!(log[0].contains("--title X"), "{}", log[0]);
    assert!(log[0].contains("--kind idea"), "{}", log[0]);
}

/// Found by the adversarial review: `--title` without `--kind` used to be
/// accepted, and the title was then silently discarded, because both the
/// vocabulary writer and the kan client only pass the pair. Silently
/// dropping what a user explicitly asked for is the worst of the available
/// behaviors, so it is now a parse-time error.
#[test]
fn a_title_without_a_kind_is_refused_rather_than_silently_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    for args in [
        vec!["telos", "declare", "x", "A statement.", "--title", "X"],
        vec!["telos", "declare", "x", "A statement.", "--kind", "idea"],
    ] {
        let out = day(dir.path(), &kan, &args);
        assert!(!out.status.success(), "{args:?} should be refused");
        assert!(appends(dir.path()).is_empty(), "nothing should be appended");
    }
}

#[test]
fn ac3_tension_cites_both_teloi_and_refuses_when_one_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim("telos/a", "bafyreia", "A."),
            claim("telos/b", "bafyreib", "B."),
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &["telos", "tension", "a", "b", "they pull apart"],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = appends(dir.path());
    assert_eq!(log.len(), 3, "a claim carrying the why, plus two edges");
    assert!(log[0].contains("--subject telos/a"), "{}", log[0]);
    assert!(log[0].contains("--cites bafyreia"), "{}", log[0]);
    assert!(log[0].contains("--cites bafyreib"), "{}", log[0]);
    assert!(log[0].contains("they pull apart"), "{}", log[0]);

    // day#18: the tension is a queryable edge, not only prose. Two edges,
    // because kan's relation is directed and visible only from its source —
    // with one, "what is this telos in tension with" would answer from
    // whichever side the arguments happened to be typed in and lie by
    // omission from the other.
    assert_eq!(
        log[1], "relate telos/a in-tension-with telos/b --cites bafyreistub00000001",
        "the a→b edge should cite the claim carrying the reason"
    );
    assert_eq!(
        log[2], "relate telos/b in-tension-with telos/a --cites bafyreistub00000001",
        "the b→a edge should exist too, and cite the same reason"
    );

    // A tension asserted against a telos that was never declared would be a
    // claim about nothing.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[claim("telos/a", "bafyreia", "A.")]);
    let out = day(dir.path(), &kan, &["telos", "tension", "a", "ghost", "why"]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("telos/ghost"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(appends(dir.path()).is_empty(), "nothing should be appended");
}

#[test]
fn ac4_a_declared_atom_round_trips_through_the_interface_parser() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "atom",
            "declare",
            "generative-build",
            "--in",
            "design-doc",
            "--out",
            "code-change",
            "--next",
            "adversarial-review",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let log = appends(dir.path());
    assert!(log[0].starts_with("observe "), "{}", log[0]);
    let interface = day::atoms::extract_interface(&log[0])
        .expect("the appended text should carry a day-atom block")
        .expect("the block should parse");
    assert_eq!(interface.inputs, vec!["design-doc"]);
    assert_eq!(interface.outputs, vec!["code-change"]);
    assert_eq!(interface.next, vec!["adversarial-review"]);
}

#[test]
fn ac5_a_non_composing_atom_is_reported_and_still_recorded() {
    let dir = tempfile::tempdir().unwrap();
    // `review` needs an input nothing produces. The atom declared below is
    // the one that introduces the failure, and the stub reflects appends, so
    // the check genuinely runs over the newly-declared interface rather than
    // over a pre-arranged one.
    let kan = write_kan_stub(
        dir.path(),
        &[common::atom_claim(
            "review",
            "bafyreireview",
            &["verified-spec"],
            &["verdict"],
            &[],
        )],
    );

    let out = day(
        dir.path(),
        &kan,
        &[
            "atom",
            "declare",
            "build",
            "--out",
            "code-change",
            "--next",
            "review",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "declaring must not be gated: {stdout}"
    );
    assert!(stdout.contains("does not compose yet"), "{stdout}");
    assert!(stdout.contains("Recorded anyway"), "{stdout}");
    assert_eq!(
        appends(dir.path()).len(),
        1,
        "the claim should still be appended"
    );
}

#[test]
fn ac6_init_records_the_baseline_schema_once_and_reruns_cleanly() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(dir.path(), &kan, &["init"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = appends(dir.path());
    assert_eq!(log.len(), 1, "init should record the baseline schema");
    assert!(log[0].contains("--subject schema/design-doc"), "{}", log[0]);

    // The recorded schema must be what `Schema::load` reads back.
    let recorded =
        day::atoms::extract_fenced::<day::schema::Schema>(&log[0], day::schema::FENCE_INFO)
            .expect("init should record a day-schema block")
            .expect("the recorded block should parse");
    assert_eq!(recorded, day::schema::Schema::starter());

    // Re-running against a log that already has one records nothing.
    let kan = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);
    let out = day(dir.path(), &kan, &["init"]);
    assert!(out.status.success());
    assert!(appends(dir.path()).is_empty(), "init should be idempotent");
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("already declared"),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let out = day(dir.path(), &kan, &["init", "--force"]);
    assert!(out.status.success());
    assert_eq!(appends(dir.path()).len(), 1, "--force should re-record");
}

#[test]
fn ac7_init_never_touches_config_and_print_records_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(dir.path(), &kan, &["init", "--print"]);
    assert!(out.status.success());
    assert!(
        appends(dir.path()).is_empty(),
        "--print must record nothing"
    );

    let out = day(dir.path(), &kan, &["init"]);
    assert!(out.status.success());
    assert!(
        !dir.path().join(".claude").exists(),
        "init writes claims, never config"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("/plugin install"), "{stdout}");
}

#[test]
fn ac8_session_end_lists_open_subjects_and_survives_a_missing_kan() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim("bug-42", "bafyreibug", "still broken"),
            claim(
                "telos/legible-process",
                "bafyreitelos",
                "The record suffices.",
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["hook", "session-end"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(stdout.contains("bug-42"), "{stdout}");
    assert!(stdout.contains("telos/legible-process"), "{stdout}");
    assert!(stdout.contains("Nothing here blocks"), "{stdout}");

    let out = day(
        dir.path(),
        &missing_kan(dir.path()),
        &["hook", "session-end"],
    );
    assert!(
        out.status.success(),
        "a hook must never fail the session, even without kan"
    );
}

#[test]
fn ac10_telos_and_atom_declarations_share_one_citation_behavior() {
    // Both verbs route through src/vocabulary.rs, so a prior claim on the
    // subject is cited identically whichever kind of vocabulary it is.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim("telos/t", "bafyreitprior", "prior telos"),
            common::atom_claim("a", "bafyreiaprior", &["x"], &["y"], &[]),
        ],
    );

    day(dir.path(), &kan, &["telos", "declare", "t", "revised"]);
    day(
        dir.path(),
        &kan,
        &["atom", "declare", "a", "--in", "x", "--out", "y"],
    );

    let log = appends(dir.path());
    assert_eq!(log.len(), 2);
    assert!(log[0].contains("--cites bafyreitprior"), "{}", log[0]);
    assert!(log[1].contains("--cites bafyreiaprior"), "{}", log[1]);
}
