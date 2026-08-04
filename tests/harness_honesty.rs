//! `.design/verification-that-can-fail.md` — the harnesses in `scripts/` and
//! `.github/workflows/` are held to the rule they exist to enforce.
//!
//! Four measurement tools asserted more than they verified in the session that
//! produced this milestone, and none of it was in day's shipped behaviour. So
//! this file is about the tooling rather than the tool: that the release script
//! refuses what it cannot verify, that the migration matrix has no version with
//! a hole where its row should be, and that the revert harness's trailer is a
//! claim something can refute.

mod common;

use common::repo_root;
use std::path::Path;
use std::process::Command;

fn read(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should be readable: {e}", path.display()))
}

fn released_tags() -> Vec<String> {
    let out = Command::new("git")
        .args(["tag", "--list", "v*.*.*"])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

/// `.design/verification-that-can-fail.md` AC-12 — **every released tag has a
/// measured expectation row.**
///
/// day#118's window closed from both ends: `scripts/cut-release.sh` writes the
/// row before the tag, and this asserts the file has no hole. The point of
/// having it here as well as in the script is that a row lost in a rebase is
/// visible on the next push rather than on the next release.
///
/// **It reports could-not-check rather than passing** when no tags are visible.
/// A shallow checkout fetches no tags, and a completeness check over an empty
/// set is vacuously true — which is the entire failure class this milestone is
/// about, and it would be this test committing it. `.github/workflows/ci.yml`
/// therefore checks out with `fetch-depth: 0`.
#[test]
fn every_released_tag_has_a_migration_expectation() {
    let tags = released_tags();
    assert!(
        !tags.is_empty(),
        "could not check: no `v*.*.*` tags are visible, so this test has nothing \
         to be complete about. That is not a pass. A shallow checkout is the \
         usual cause — ci.yml uses `fetch-depth: 0` for exactly this reason."
    );

    let tsv = read("tests/fixtures/migration-expectations.tsv");
    let rows: Vec<&str> = tsv
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .collect();

    let missing: Vec<&String> = tags
        .iter()
        .filter(|tag| {
            !rows
                .iter()
                .any(|r| r.split('\t').next() == Some(tag.as_str()))
        })
        .collect();

    assert!(
        missing.is_empty(),
        "released tags with no expectation row: {missing:?}\n\n\
         A row is measured, never assumed (day#118). `scripts/cut-release.sh` \
         writes one before tagging; if one of these predates that, measure it \
         with `scripts/run-migration-cell.sh` against a build of that tag."
    );
}

/// AC-12 — **the matrix enumerates the tag being released, like every other.**
///
/// The exclusion was correct about its stated reason and wrong about its
/// consequence, so what is asserted here is the absence of the mechanism rather
/// than the presence of a comment: a workflow that filters `GITHUB_REF_NAME` out
/// of the tag list has the window back regardless of what it says about why.
#[test]
fn the_matrix_does_not_exclude_the_tag_being_released() {
    let yaml = read(".github/workflows/migration-matrix.yml");
    let enumerate = yaml
        .split("- id: tags")
        .nth(1)
        .expect("the workflow should still have a `tags` step")
        .split("- id: hash")
        .next()
        .expect("the `tags` step should be followed by another");

    assert!(
        !enumerate.contains("grep -v"),
        "the tag enumeration filters its list again:\n{enumerate}\n\n\
         day#118: excluding the tag being released is what left a version with \
         no cell at its own push, so its missing row could not fail until the \
         next release."
    );
    assert!(
        enumerate.contains("git tag --list 'v*.*.*'"),
        "the tag enumeration no longer lists released tags at all:\n{enumerate}"
    );
}

/// AC-27 — **the origin/main guard says what it now proves, and the closing
/// instruction pushes both refs.**
///
/// `cut-release.sh` makes a commit now (the migration row), so the guard proves
/// the tagged commit's *parent* is on origin rather than the commit itself. That
/// narrowing is a real cost of putting the row in the tagged tree, and the one
/// thing that keeps it small is pushing `main` and the tag together — a tag
/// pushed alone would name a commit the remote has never seen.
#[test]
fn the_release_script_states_the_narrowed_guarantee_and_pushes_both_refs() {
    let script = read("scripts/cut-release.sh");
    assert!(
        script.contains("PARENT is on origin"),
        "scripts/cut-release.sh no longer records that its origin/main check \
         proves the tagged commit's parent is pushed, not the commit itself"
    );
    assert!(
        script.contains("git push origin main %s"),
        "the closing instruction must push `main` and the tag together; the \
         tagged commit is the row commit this script made, so it is not on origin"
    );
}

/// AC-13 — **a missing row refuses the release before anything is built.**
///
/// Driven, not read. Asserting that the script *contains* a refusal checks day's
/// own side of the interface, which `CLAUDE.md` records as this repo's
/// most-repeated failure; what matters is that the refusal fires and that it
/// fires early. Both are observed here from outside: the exit status, the
/// message, and a stub `cargo` that records every invocation it received — so
/// "before building" is a measurement rather than a reading of the source order.
#[test]
fn cut_release_refuses_before_building_when_a_row_is_missing() {
    let dir = tempfile::tempdir().expect("a scratch dir");
    let repo = dir.path();
    let bin = repo.join("bin");
    std::fs::create_dir_all(&bin).unwrap();

    // Stubs for everything the script resolves through PATH. `cargo` logs each
    // invocation, which is what makes "before building" observable.
    stub(&bin, "kan", "exit 0\n");
    stub(&bin, "day", "exit 0\n");
    stub(&bin, "jq", "echo 9.9.9\n");
    stub(
        &bin,
        "cargo",
        "echo \"$@\" >> \"$(dirname \"$0\")/../cargo.log\"\necho '{}'\nexit 0\n",
    );

    for cmd in [
        vec!["init", "-q", "-b", "main"],
        vec!["config", "user.email", "t@example.com"],
        vec!["config", "user.name", "t"],
    ] {
        run_git(repo, &cmd);
    }
    std::fs::create_dir_all(repo.join("tests/fixtures")).unwrap();
    std::fs::write(
        repo.join("tests/fixtures/migration-expectations.tsv"),
        "# tag\texpected-outcome\nv0.1.0\tprotocol-mismatch\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("Cargo.toml"),
        "[package]\nname='x'\nversion='9.9.9'\n",
    )
    .unwrap();
    run_git(repo, &["add", "-A"]);
    run_git(repo, &["commit", "-qm", "init"]);
    // Two released tags, one of which has no row. The one WITH a row is what
    // keeps this from passing for the trivial reason that nothing was checked.
    run_git(repo, &["tag", "v0.1.0"]);
    run_git(repo, &["tag", "v0.2.0"]);

    let script = repo_root().join("scripts/cut-release.sh");
    let out = Command::new("sh")
        .arg(&script)
        .arg("v9.9.9")
        .current_dir(repo)
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .output()
        .expect("the release script should be runnable");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the script should refuse; it exited 0.\nstdout: {}\nstderr: {stderr}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        stderr.contains("v0.2.0"),
        "the refusal should name the tag with no row; got: {stderr}"
    );
    assert!(
        !stderr.contains("v0.1.0"),
        "v0.1.0 has a row and should not be named: {stderr}"
    );

    // The premise this criterion actually turns on: nothing was built. A
    // refusal that arrives after the ten-minute verify block is a different and
    // much weaker guarantee than the one AC-13 claims.
    let log = std::fs::read_to_string(repo.join("cargo.log")).unwrap_or_default();
    assert!(
        !log.contains("build") && !log.contains("test"),
        "the script built before refusing; cargo received: {log}"
    );
}

fn stub(bin: &Path, name: &str, body: &str) {
    let path = bin.join(name);
    std::fs::write(&path, format!("#!/bin/sh\n{body}")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}

fn run_git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("git should be runnable");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
