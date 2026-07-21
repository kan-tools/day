//! `.design/scaffold.md` AC-2, AC-3, AC-6 — real subprocess invocations of
//! the `day` binary against a stub kan, not library calls, since what's
//! being proven is the wiring.

#![cfg(unix)]

mod common;

use std::process::Command;

use common::{atom_claim, claim, missing_kan, write_kan_stub};

fn day(dir: &std::path::Path, kan: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("failed to run day")
}

#[test]
fn ac2_init_doctor_and_hook_all_succeed_when_kan_is_reachable() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    for args in [vec!["init"], vec!["doctor"], vec!["hook", "session-start"]] {
        let out = day(dir.path(), &kan, &args);
        assert!(
            out.status.success(),
            "day {args:?} should exit 0, got {:?}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

#[test]
fn ac2_doctor_exits_non_zero_with_a_clear_message_when_kan_is_absent() {
    let dir = tempfile::tempdir().unwrap();
    let out = day(dir.path(), &missing_kan(dir.path()), &["doctor"]);

    assert!(!out.status.success(), "doctor should fail without kan");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("kan is not reachable"),
        "expected a clear message, got: {stderr}"
    );
    assert!(
        stderr.contains("cargo install kan"),
        "the message should say how to fix it, got: {stderr}"
    );
}

#[test]
fn ac3_doctor_reports_incompatible_interfaces_and_names_both_atoms() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_claim(
                "design",
                "bafyreidesign",
                &["idea"],
                &["design-doc"],
                &["build"],
            ),
            atom_claim(
                "build",
                "bafyreibuild",
                &["verified-spec"],
                &["code-change"],
                &[],
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(!out.status.success(), "findings should not exit 0");
    assert!(
        stdout.contains("atom/design"),
        "should name the upstream atom: {stdout}"
    );
    assert!(
        stdout.contains("atom/build"),
        "should name the downstream atom: {stdout}"
    );
    assert!(
        stdout.contains("verified-spec"),
        "should name the unsatisfied input: {stdout}"
    );
}

#[test]
fn ac3_doctor_reports_success_on_a_composing_set() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_claim(
                "design",
                "bafyreidesign",
                &["idea"],
                &["design-doc"],
                &["build"],
            ),
            atom_claim(
                "build",
                "bafyreibuild",
                &["design-doc"],
                &["code-change"],
                &[],
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        out.status.success(),
        "a composing set should exit 0: {stdout}"
    );
    assert!(stdout.contains("composition: ok"), "got: {stdout}");
}

#[test]
fn newest_interface_bearing_claim_supersedes_the_older_one() {
    // Per-atom additive versioning: revising an atom appends a claim, it
    // never rewrites one, so the newest block is the live interface.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_claim(
                "design",
                "bafyreiold",
                &["idea"],
                &["wrong-output"],
                &["build"],
            ),
            atom_claim(
                "design",
                "bafyreinew",
                &["idea"],
                &["design-doc"],
                &["build"],
            ),
            atom_claim(
                "build",
                "bafyreibuild",
                &["design-doc"],
                &["code-change"],
                &[],
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "the superseding claim should be the one checked: {stdout}"
    );
}

#[test]
fn ac6_session_start_hook_lists_recorded_telos_subjects() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "telos/interface-stability",
                "bafyreitelos",
                "The CLI vocabulary stays legible to an agent that has never seen it before.",
            ),
            atom_claim("design", "bafyreidesign", &["idea"], &["design-doc"], &[]),
        ],
    );

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(
        stdout.contains("telos/interface-stability"),
        "should surface the telos subject: {stdout}"
    );
    assert!(
        stdout.contains("legible to an agent"),
        "should surface the telos text: {stdout}"
    );
}

#[test]
fn a_telos_stays_identifiable_when_the_newest_claim_is_commentary_about_it() {
    // Found by dogfooding: recording a tension against a telos made the
    // hook show the tension instead of the telos, in the tool whose own
    // telos is legibility. The declared title is what the subject *is*.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "telos/legible-process",
                "bafyreideclare",
                "The process a project followed is reconstructable from the record alone.",
            ),
            common::subject_claim("telos/legible-process", "bafyreititle", "Legible process"),
            claim(
                "telos/legible-process",
                "bafyreitension",
                "Tension: this trades off against affordance-not-enforcement.",
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("Legible process"),
        "the telos title should survive later commentary: {stdout}"
    );
}

#[test]
fn ac6_session_start_hook_exits_zero_with_no_teloi_recorded() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success(), "an empty log is not an error");
    assert!(
        stdout.contains("No teloi are recorded"),
        "should say so plainly: {stdout}"
    );
}

#[test]
fn req4_the_hook_never_fails_the_session_even_without_kan() {
    // A hook that can fail a session is a blocking hook by another name.
    let dir = tempfile::tempdir().unwrap();
    let out = day(
        dir.path(),
        &missing_kan(dir.path()),
        &["hook", "session-start"],
    );

    assert!(
        out.status.success(),
        "the session-start hook must exit 0 even when kan is missing"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("kan is not reachable"), "got: {stdout}");
}

#[test]
fn an_unknown_hook_event_is_named_but_still_exits_zero() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(dir.path(), &kan, &["hook", "not-an-event"]);

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("unknown hook event"), "got: {stdout}");
}

#[test]
fn init_prints_both_install_paths_and_never_mutates_config() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(dir.path(), &kan, &["init"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(stdout.contains("/plugin install"), "got: {stdout}");
    assert!(stdout.contains("claude mcp add day"), "got: {stdout}");
    assert!(stdout.contains("hook session-start"), "got: {stdout}");
    assert!(
        !dir.path().join(".claude").exists(),
        "init must not write into the repo's config"
    );
}
