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

/// **This test is named for a case it does not exercise**, which is why it is
/// now two tests.
///
/// `missing_kan()` sets `DAY_KAN_BIN` to a path that does not exist — so this
/// drives the *override* path, not the absent-install path a user without kan
/// is in. The distinction went unnoticed because both produced one message, and
/// that message told an override user to "set `DAY_KAN_BIN` to its path" when
/// the variable was already set and was what selected the missing file.
///
/// It also asserted `stderr.contains("cargo install kan")` under the label "the
/// message should say how to fix it". When the override message was first split
/// out, its text read "`cargo install kan` will not fix it" — and this
/// assertion **passed against a sentence saying the opposite of what it
/// checks**. A phrase-presence grep cannot tell a recommendation from its
/// negation; `CLAUDE.md` files that as a defect class, and it very nearly
/// shipped here. The remedy string is now kept out of the override text
/// entirely, which is what makes the two cases distinguishable at all.
#[test]
fn ac2_doctor_reports_a_bad_day_kan_bin_as_an_override_not_a_missing_install() {
    let dir = tempfile::tempdir().unwrap();
    let out = day(dir.path(), &missing_kan(dir.path()), &["doctor"]);

    assert!(!out.status.success(), "doctor should fail without kan");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("kan is not reachable"),
        "expected a clear message, got: {stderr}"
    );
    assert!(
        stderr.contains("DAY_KAN_BIN is set and selected that path"),
        "the override case must say the variable chose this path, got: {stderr}"
    );
    assert!(
        !stderr.contains("cargo install kan"),
        "the override case must NOT recommend installing kan — the variable is \
         already set and points at the missing file, so an install fixes nothing. \
         This assertion is the negation the old phrase-presence check could not \
         express. Got: {stderr}"
    );
}

/// The case the test above was named for and never covered: **no
/// `DAY_KAN_BIN`, and no `kan` on `PATH`.** Here `cargo install kan` genuinely
/// is the remedy, and the message must say so.
///
/// `PATH` is set to an empty directory rather than cleared, because an unset
/// `PATH` makes the OS fall back to a default search path on some platforms —
/// which would find a real kan and test nothing.
#[test]
fn ac2_doctor_recommends_installing_kan_when_it_is_genuinely_absent() {
    let dir = tempfile::tempdir().unwrap();
    let empty = dir.path().join("empty-path");
    std::fs::create_dir_all(&empty).unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .arg("doctor")
        .current_dir(dir.path())
        .env_remove("DAY_KAN_BIN")
        .env("PATH", &empty)
        .output()
        .expect("failed to run day");

    assert!(!out.status.success(), "doctor should fail without kan");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("kan is not reachable"),
        "expected a clear message, got: {stderr}"
    );
    assert!(
        stderr.contains("cargo install kan"),
        "with no override set, an absent kan IS fixed by installing it, and this \
         is the one case where the message should say so. Got: {stderr}"
    );
    assert!(
        !stderr.contains("is set and selected that path"),
        "nothing set DAY_KAN_BIN here, so the override wording would be a lie: \
         {stderr}"
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

/// Found by the v0.3 adversarial review, cleaning up after itself: kan never
/// destroys a subject, so a fully-retracted telos still appears in `status`.
/// Listing it as "in play" would make retraction look like it had not worked.
#[test]
fn a_telos_whose_every_claim_is_retracted_is_not_in_play() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim("telos/live", "bafyreilive", "A telos still in play."),
            // A retraction claim carries no text and no title — exactly what
            // a subject looks like once everything on it is retracted.
            common::retraction_claim("telos/gone", "bafyreigone"),
        ],
    );

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());

    // Scoped to the teloi section deliberately. The retracted subject may
    // still appear under "Still open", but that line is a passthrough of
    // `kan issues` — how kan classifies a subject is kan's judgment, and day
    // second-guessing it would be worse than the noise.
    let teloi_section: String = stdout
        .lines()
        .skip_while(|l| !l.starts_with("Teloi in play"))
        .take_while(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(teloi_section.contains("telos/live"), "{stdout}");
    assert!(
        !teloi_section.contains("telos/gone"),
        "a fully-retracted telos should not be listed as in play: {teloi_section}"
    );
    assert!(teloi_section.contains("Teloi in play (1)"), "{stdout}");
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
