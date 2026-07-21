//! `.design/assess-docs.md` AC-1..AC-12 — the docs assessment, its two
//! tiers, and the guarantee that day's git access stays read-only.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, git: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", git)
        .output()
        .expect("failed to run day")
}

/// A stub `git` answering only the two read subcommands day uses.
fn write_git_stub(dir: &Path, tags: &[&str], changed: &[&str]) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
case "$1" in
  tag) printf '%s' "{tags}" ;;
  diff) printf '%s' "{changed}" ;;
  *) echo "git stub: unsupported read $1" >&2; exit 1 ;;
esac
"#,
            tags = tags.join("\n"),
            changed = changed.join("\n"),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn schema_claim(cid: &str, version_files: &[&str], doc_files: &[&str]) -> StubClaim {
    let list = |xs: &[&str]| {
        xs.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    claim(
        "schema/docs",
        cid,
        &format!(
            "Docs schema.\n\n```day-docs\n{{\"version_source\": \"Cargo.toml\", \
             \"version_files\": [{}], \"doc_files\": [{}]}}\n```\n",
            list(version_files),
            list(doc_files)
        ),
    )
}

/// A workspace with a Cargo.toml at 1.0.0 and a README that agrees.
fn workspace() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"x\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("README.md"), "x 1.0.0 is current\n").unwrap();
    dir
}

/// `.design/assess-docs.md` AC-1. day's git access is read-only by
/// construction, and all of it lives behind `src/git.rs` so this grep is a
/// guarantee rather than a spot check.
#[test]
fn ac1_day_never_invokes_a_mutating_git_subcommand() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources = Vec::new();
    for entry in walk(&src) {
        sources.push((entry.clone(), std::fs::read_to_string(&entry).unwrap()));
    }

    // Whitelist rather than blacklist: enumerate the subcommands git.rs is
    // allowed to invoke and assert every call uses one. A blacklist of
    // mutating verbs is both leakier (git has many) and prone to false
    // positives — the first version of this test flagged `git tag --list`,
    // which is a read, because it matched a pattern meant for `git tag -d`.
    const ALLOWED_READS: [&str; 6] = ["tag", "diff", "log", "rev-parse", "show", "status"];
    let git_rs = sources
        .iter()
        .find(|(p, _)| p.file_name().unwrap() == "git.rs")
        .expect("src/git.rs should exist");

    let mut invocations = 0;
    let mut rest = git_rs.1.as_str();
    while let Some(at) = rest.find("self.run(&[\"") {
        let after = &rest[at + "self.run(&[\"".len()..];
        let subcommand: String = after.chars().take_while(|c| *c != '"').collect();
        assert!(
            ALLOWED_READS.contains(&subcommand.as_str()),
            "src/git.rs invokes `git {subcommand}`, which is not one of the permitted \
             read subcommands {ALLOWED_READS:?}. day's git access is read-only."
        );
        invocations += 1;
        rest = after;
    }
    assert!(
        invocations > 0,
        "the scan should have found git invocations"
    );

    // And git is reached from nowhere else, so the whitelist above covers
    // every git call day makes. Checked precisely: the env var naming the
    // git binary appears only in git.rs, and nothing spawns `git` directly.
    for (path, text) in &sources {
        assert!(
            !text.contains("Command::new(\"git\")"),
            "{} spawns git directly; all git access belongs in src/git.rs",
            path.display()
        );
        if path.file_name().unwrap() == "git.rs" {
            continue;
        }
        assert!(
            !text.contains("DAY_GIT_BIN") && !text.contains("GIT_BIN_ENV"),
            "{} reaches for the git binary outside src/git.rs",
            path.display()
        );
    }
}

fn walk(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}

#[test]
fn ac2_a_tag_with_no_release_claim_is_reported_as_divergence() {
    let dir = workspace();
    let kan = write_kan_stub(dir.path(), &[schema_claim("bafyreis", &["README.md"], &[])]);
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("v1.0.0 is tagged but no"), "{stdout}");
    assert!(stdout.contains("nobody wrote down"), "{stdout}");
}

#[test]
fn ac2_a_release_claim_with_no_tag_is_reported_as_divergence() {
    let dir = workspace();
    let kan = write_kan_stub(
        dir.path(),
        &[
            schema_claim("bafyreis", &["README.md"], &[]),
            claim("release", "bafyreir", "released 1.0.0"),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("nobody cut"), "{stdout}");
}

#[test]
fn ac2_agreement_between_the_two_records_passes() {
    let dir = workspace();
    let kan = write_kan_stub(
        dir.path(),
        &[
            schema_claim("bafyreis", &["README.md"], &[]),
            claim("release", "bafyreir", "released v1.0.0 to the registry"),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("both tagged and recorded"), "{stdout}");
}

#[test]
fn ac4_since_overrides_the_boundary_and_skips_reconciliation() {
    let dir = workspace();
    let kan = write_kan_stub(dir.path(), &[schema_claim("bafyreis", &["README.md"], &[])]);
    // A tag exists with no release claim, which would normally warn.
    let git = write_git_stub(dir.path(), &["v1.0.0"], &["src/lib.rs"]);

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "docs", "--since", "HEAD~5"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("nobody wrote down"),
        "--since should skip reconciliation entirely: {stdout}"
    );
    assert!(stdout.contains("Since HEAD~5"), "{stdout}");
}

#[test]
fn ac5_a_stale_version_file_fails_and_a_current_one_passes() {
    let dir = workspace();
    std::fs::write(dir.path().join("STALE.md"), "still on 0.9.0\n").unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[schema_claim("bafyreis", &["README.md", "STALE.md"], &[])],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "a stale version must fail: {stdout}");
    assert!(stdout.contains("README.md carries 1.0.0"), "{stdout}");
    assert!(
        stdout.contains("STALE.md does not mention 1.0.0"),
        "{stdout}"
    );
}

#[test]
fn ac6_changing_the_schema_claim_changes_what_is_checked() {
    let dir = workspace();
    std::fs::write(dir.path().join("OTHER.md"), "no version here\n").unwrap();
    let git = write_git_stub(dir.path(), &[], &[]);

    // Only README is checked: passes.
    let kan = write_kan_stub(dir.path(), &[schema_claim("bafyreis", &["README.md"], &[])]);
    assert!(day(dir.path(), &kan, &git, &["assess", "docs"])
        .status
        .success());

    // A newer schema claim adds OTHER.md: now it fails. No file edited.
    let kan = write_kan_stub(
        dir.path(),
        &[
            schema_claim("bafyreis", &["README.md"], &[]),
            schema_claim("bafyreis2", &["README.md", "OTHER.md"], &[]),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("OTHER.md"),
        "the newest schema claim should win"
    );
}

#[test]
fn ac7_with_no_schema_declared_day_explains_and_offers_a_starter() {
    let dir = workspace();
    let kan = write_kan_stub(dir.path(), &[]);
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(stderr.contains("no docs schema is declared"), "{stderr}");
    assert!(stderr.contains("kan observe"), "{stderr}");
    assert!(stderr.contains("day-docs"), "{stderr}");
}

#[test]
fn ac8_code_changing_without_any_watched_doc_is_prompted() {
    let dir = workspace();
    let kan = write_kan_stub(
        dir.path(),
        &[schema_claim("bafyreis", &["README.md"], &["README.md"])],
    );

    // Source changed, README did not.
    let git = write_git_stub(dir.path(), &["v1.0.0"], &["src/lib.rs", "src/cli.rs"]);
    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("2 file(s) changed"), "{stdout}");
    assert!(
        stdout.contains("none of the watched docs changed"),
        "{stdout}"
    );
    assert!(
        out.status.success(),
        "the evidence tier prompts; it does not fail"
    );

    // README changed too: no prompt to reconcile.
    let git = write_git_stub(dir.path(), &["v1.0.0"], &["src/lib.rs", "README.md"]);
    let out = day(dir.path(), &kan, &git, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("watched docs changed: README.md"),
        "{stdout}"
    );
    assert!(!stdout.contains("none of the watched docs"), "{stdout}");
}

#[test]
fn ac9_assess_writes_nothing_to_kan_or_the_working_tree() {
    let dir = workspace();
    let kan = write_kan_stub(
        dir.path(),
        &[schema_claim("bafyreis", &["README.md"], &["README.md"])],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &["src/lib.rs"]);

    let before = std::fs::read_to_string(dir.path().join("README.md")).unwrap();
    day(dir.path(), &kan, &git, &["assess", "docs"]);

    assert!(
        common::appends(dir.path()).is_empty(),
        "assessing must not record anything -- recording an assessment is a separate act"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join("README.md")).unwrap(),
        before
    );
}

#[test]
fn ac10_assess_docs_is_a_subcommand_of_assess() {
    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["assess", "--help"])
        .output()
        .expect("failed to run day assess --help");
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("docs"), "{help}");
}
