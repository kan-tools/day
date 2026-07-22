//! day#27 — does the argument shape day builds actually parse against the
//! real `kan` binary?
//!
//! Every other integration test in this repo stubs kan through
//! `DAY_KAN_BIN`, which is right for hermetic CI and useless for this
//! question: a stub accepts whatever day sends it, so those tests validate
//! day against day's own idea of kan's CLI rather than against kan's
//! contract. That blind spot let `docs/CONVENTIONS.md` document
//! `kan result "<text>" --subject <s>` — a command that does not run —
//! through several releases, and very nearly shipped it as day's own
//! printed output.
//!
//! Two different guarantees live here:
//!
//! - [`append_is_only_used_with_verbs_whose_subject_is_a_flag`] is
//!   **hermetic** and always runs. It is the one that actually protects the
//!   invariant, by construction rather than by observation.
//! - The `conformance_*` tests run the real binary and **skip when kan is
//!   not installed**, because `CLAUDE.md` requires that no test need a real
//!   kan. They catch drift in kan's surface that day's own source cannot
//!   reveal.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

use day::kan_client::{KanClient, Write};

/// The real `kan`, or `None` when it is not installed.
///
/// Deliberately ignores `DAY_KAN_BIN`: that variable exists to point day at
/// a *stub*, and a stub is exactly what these tests must not talk to.
fn real_kan() -> Option<&'static str> {
    Command::new("kan")
        .arg("--help")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| "kan")
}

/// kan anchors claims to the HEAD commit, so it needs a git repository —
/// hence a scratch one per test. Scratch rather than this repo because a
/// probe against the real log leaves real claims, which `CLAUDE.md` names
/// as its own kind of defect.
fn scratch_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args(args)
            .current_dir(dir.path())
            .output()
            .expect("git should be available")
            .status
            .success();
        assert!(ok, "git {args:?} failed while building the scratch repo");
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "conformance@example.invalid"]);
    git(&["config", "user.name", "conformance"]);
    git(&["commit", "-q", "--allow-empty", "-m", "scratch"]);
    dir
}

fn sources() -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
        for entry in std::fs::read_dir(dir)
            .expect("src should be readable")
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).expect("source should be readable");
                out.push((path, text));
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
    out
}

/// The guarantee that holds without kan installed, and the one that would
/// actually have prevented day#27's bug.
///
/// `KanClient::append` builds `<verb> <text> --subject <s>`. That shape is
/// correct for exactly three of kan's verbs. `result` takes
/// `<SUBJECT> <TEXT>` positionally, and `relate`/`same` take their subjects
/// positionally too — so routing any of them through `append` would emit a
/// command kan rejects, and no stub-based test could tell.
///
/// If day grows a verb that needs one of those, it needs its own method
/// with its own argument order; adding it to this list instead is the
/// mistake this test exists to stop.
#[test]
fn append_is_only_used_with_verbs_whose_subject_is_a_flag() {
    const SUBJECT_AS_FLAG: [&str; 3] = ["observe", "plan", "decide"];

    let mut found = 0;
    for (path, text) in sources() {
        let mut rest = text.as_str();
        while let Some(at) = rest.find("Write::new(\"") {
            let after = &rest[at + "Write::new(\"".len()..];
            let verb: String = after.chars().take_while(|c| *c != '"').collect();
            assert!(
                SUBJECT_AS_FLAG.contains(&verb.as_str()),
                "{} passes `{verb}` to Write::new, but append builds \
                 `<verb> <text> --subject <s>` and only {SUBJECT_AS_FLAG:?} take their \
                 subject as a flag. `kan {verb}` takes it positionally, so this would \
                 emit a command kan rejects — and every stub-based test would still pass.",
                path.display()
            );
            found += 1;
            rest = after;
        }
    }
    assert!(found > 0, "the scan should have found Write::new calls");
}

/// Runs every argument shape `append` can build against the real binary.
///
/// Exercises `KanClient` itself rather than a transcription of what it
/// emits — a hand-written list of arguments would be a second copy of the
/// thing under test, and could agree with the docs while the code disagreed.
#[test]
fn conformance_append_shapes_are_accepted_by_real_kan() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();
    let client = KanClient::with_bin(dir.path(), bin);

    // Bare: `<verb> <text> --subject <s>`.
    let mut previous = None;
    for verb in ["observe", "plan", "decide"] {
        let cid = client
            .append(Write::new(verb, "conformance", &format!("{verb} shape")))
            .unwrap_or_else(|e| panic!("real kan rejected day's `{verb}` shape: {e}"));
        assert!(cid.starts_with("bafy"), "expected a CID, got {cid:?}");
        previous = Some(cid);
    }

    // With `--cites`, chained from a CID day captured itself.
    let cites = [previous.expect("a prior claim")];
    client
        .append(Write::new("observe", "conformance", "cites shape").cites(&cites))
        .expect("real kan rejected day's --cites shape");

    // With `--title`/`--kind`, which kan accepts only together.
    client
        .append(
            Write::new("decide", "conformance-declared", "declaring shape")
                .declaring("Conformance", "idea"),
        )
        .expect("real kan rejected day's --title/--kind shape");

    // And the reads day depends on.
    assert!(
        client.show("conformance").is_ok_and(|c| !c.is_empty()),
        "real kan returned no claims for a subject day just wrote to"
    );
    client.subjects().expect("real kan rejected `kan status`");
    client.issues().expect("real kan rejected `kan issues`");
}

/// `docs/CONVENTIONS.md` tells a reader to record an assessment with
/// `kan result`, and `.design/assess-telos.md` REQ-12 has `day assess telos`
/// *printing* that command. A documented command that does not run is worse
/// than no documentation, so the documented form is checked directly.
///
/// This is the exact shape that was wrong: the page said
/// `kan result "<text>" --subject <s>`, pattern-matched from the three verbs
/// above, and nothing executed it. See kan-tools/kan#78 for the asymmetry.
#[test]
fn conformance_the_documented_kan_result_form_runs() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();

    let seed = Command::new(bin)
        .args(["observe", "seed", "--subject", "telos/conformance"])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(seed.status.success(), "seeding the subject failed");
    let cid = String::from_utf8_lossy(&seed.stdout).trim().to_string();

    // Subject first, positionally — NOT `--subject`.
    let out = Command::new(bin)
        .args([
            "result",
            "telos/conformance",
            "the assessment text",
            "--cites",
            &cid,
        ])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(
        out.status.success(),
        "the `kan result` form documented in docs/CONVENTIONS.md was rejected: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // And the form the docs used to recommend is genuinely rejected, which
    // is what made this worth a test rather than a one-line correction. If
    // kan#78 is resolved by accepting both spellings this will fail — that
    // is the intended signal, not a false alarm: day's docs would then be
    // needlessly strict and should be revisited.
    let wrong = Command::new(bin)
        .args([
            "result",
            "the assessment text",
            "--subject",
            "telos/conformance",
        ])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(
        !wrong.status.success(),
        "kan now accepts `result --subject`; kan#78 may have been resolved, so \
         docs/CONVENTIONS.md and this test should be revisited"
    );
}
