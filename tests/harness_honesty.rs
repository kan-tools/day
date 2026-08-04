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

/// A shell pipeline's stages, splitting on `|` **outside quotes** and joining
/// `\`-continuations.
///
/// Quote-aware because it has to be: the `jq` program in the migration matrix is
/// `'split("\n") | map(select(length > 0))'`, whose own `|` is not a pipe. A
/// naive split reads one command as two, and a check that miscounts the thing it
/// is counting is not a check.
fn pipeline_stages(command: &str) -> Vec<String> {
    let mut stages = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for c in command.chars() {
        match (quote, c) {
            (Some(q), _) if c == q => {
                quote = None;
                current.push(c);
            }
            (Some(_), _) => current.push(c),
            (None, '\'') | (None, '"') => {
                quote = Some(c);
                current.push(c);
            }
            (None, '|') => {
                stages.push(std::mem::take(&mut current));
            }
            (None, '\\') => {}
            (None, _) => current.push(c),
        }
    }
    stages.push(current);
    stages
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| !s.is_empty())
        .collect()
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

/// The pipeline stages of the workflow's tag enumeration. Taken as text so the
/// evasions a cold review named can be driven against it directly, rather than
/// only against the file that currently happens to be clean.
fn tag_enumeration_stages(yaml: &str) -> Vec<String> {
    let enumerate = yaml
        .split("- id: tags")
        .nth(1)
        .expect("the workflow should still have a `tags` step")
        .split("- id: hash")
        .next()
        .expect("the `tags` step should be followed by another");

    // The one command that produces the matrix, with its `\`-continuations
    // joined, as a list of pipeline stages.
    let assignment = enumerate
        .lines()
        .map(str::trim)
        .skip_while(|l| !l.starts_with("json=$("))
        .take_while(|l| !l.starts_with("echo "))
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        !assignment.is_empty(),
        "could not check: no `json=$(…)` assignment was found in the tags step. \
         This test would otherwise pass by finding nothing.\n{enumerate}"
    );

    pipeline_stages(
        assignment
            .trim_start_matches("json=$(")
            .trim_end_matches(')'),
    )
}

/// AC-12 — **the matrix enumerates the tag being released, like every other.**
///
/// **Keyed on the pipeline's shape, not on the absence of `grep -v`.** The first
/// version forbade that one spelling, and `grep -vx`, `sed "/$c/d"`,
/// `jq 'map(select(. != $c))'` and `comm` all reinstate the window with the test
/// green — an absence-keyed classifier, in a milestone whose rules forbid them,
/// asserting "the absence of the mechanism" while asserting the absence of one
/// way to spell it.
///
/// What is checked instead is positive and exhaustive: the enumeration is
/// **exactly two stages**, `git tag --list` and the `jq` that shapes it into
/// JSON. Any third stage fails, whatever it is and however it is spelled, and a
/// filter cannot be smuggled into either without changing them. Driven against
/// the named evasions in
/// [`the_workflow_classifiers_catch_the_evasions_they_were_keyed_around`].
#[test]
fn the_matrix_does_not_exclude_the_tag_being_released() {
    let stages = tag_enumeration_stages(&read(".github/workflows/migration-matrix.yml"));
    assert_eq!(
        stages.len(),
        2,
        "the tag enumeration must be exactly `git tag --list … | jq …`. A third \
         stage is a filter, whatever it is called: day#118 is that a version \
         excluded at its own push gets no cell, so its missing row cannot fail \
         until the next release.\nstages: {stages:?}"
    );
    // **Exact equality, not `starts_with`.** Counting stages caught a filter
    // added as a third command and missed one FOLDED INTO the two that are
    // already there — `jq 'split(…) | map(select(. != $c))'` is still two
    // stages, and `git tag --list … --no-contains $c` is still one command.
    // `map(select(. != $c))` was among the evasions the first review named, and
    // the natural place to write it is inside the jq program that is already
    // here. A `starts_with` check on a command whose tail is the interesting
    // part is a prefix check wearing a structural check's clothes.
    //
    // Equality means any change to either stage fails, including a harmless
    // one. That is deliberate: this is a six-line command in a release-gating
    // workflow, and "the enumeration changed, look at it" is the right amount
    // of friction. Both expected forms are written out so the diff says what
    // moved.
    assert_eq!(
        stages,
        vec![
            "git tag --list 'v*.*.*' --sort=creatordate".to_string(),
            "jq -R -s -c 'split(\"\\n\") | map(select(length > 0))'".to_string(),
        ],
        "the tag enumeration must be exactly these two commands. A filter can be \
         folded into either without changing the stage count, which is how \
         day#118's window comes back."
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

/// AC-24 — **`/adversarial-review` states ADR-52 and the demonstration rule.**
///
/// kan's rule — a round of fixes to a `BLOCK` gets its own review — has been
/// validated eight times in this repo and was still a thing a person had to
/// remember. The atom that produces the BLOCK is the right place for it, because
/// that is what a reviewer reads at the moment it applies.
#[test]
fn the_review_atom_states_adr_52_and_the_demonstration_rule() {
    let text = read("commands/adversarial-review.md");
    assert!(
        text.contains("ADR-52"),
        "the review command must name ADR-52: a round of fixes to a BLOCK gets \
         its own review"
    );
    assert!(
        text.contains("Demonstrated-by:"),
        "a fix round closing a finding must be told to carry a demonstration; \
         \"fixed and the suite is green\" is the weaker of the two claims"
    );
}

/// AC-12's other half, and REQ-9 — **the re-derivation job exists and cannot
/// pass by finding nothing.**
///
/// The behaviour is asserted against a real repository in
/// `tests/revert_demo.rs`; what is asserted here is the *wiring*, which is the
/// half a behaviour test cannot see. Both halves matter: a verifier that works
/// and is not wired reaches nobody, which is this repo's most-repeated failure.
#[test]
fn the_revert_demo_job_is_wired_and_fails_when_it_cannot_check() {
    let yaml = read(".github/workflows/revert-demo.yml");
    assert!(
        yaml.contains("scripts/revert-demo.py --verify"),
        "the job must re-derive with the harness, not read the trailer"
    );
    assert!(
        yaml.contains("could not compute a merge base") && yaml.contains("exit 1"),
        "a job that cannot build its commit range must fail rather than pass \
         with nothing to do — could-not-check outranks checked-and-clean, and a \
         verification job is the last place to get that backwards"
    );
    assert!(
        yaml.contains("github.event.pull_request.head.sha"),
        "on a pull_request the default checkout is the MERGE commit, whose \
         message carries no trailer; the job must verify what was written"
    );
    // **The trigger set, positively.** A trigger whose commit range is always
    // empty is a green job that checked nothing: after a merge,
    // `merge-base(origin/main, HEAD)` IS `HEAD`.
    //
    // The first version of this forbade the literal `branches: [main]`, which
    // the ordinary block form
    //     push:
    //       branches:
    //         - main
    // walks straight past — an absence-keyed classifier added in the commit that
    // fixed a can't-fail check, and its `Demonstrated-by:` trailer passed only
    // because reversion happens to restore that exact spelling. A demonstration
    // cannot see this class of weakness, which is worth knowing about the rule
    // as much as about this test.
    assert_eq!(
        workflow_triggers(&yaml),
        vec!["pull_request"],
        "the job must trigger on `pull_request` and nothing else. On a push to \
         main the commit range is empty by construction, so the job would be \
         permanently green for having found nothing — which is the defect it \
         exists to catch."
    );

    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("fetch-depth: 0"),
        "ci.yml must fetch tags, or `every_released_tag_has_a_migration_expectation` \
         has an empty set to be complete about"
    );
}

/// The events a workflow's `on:` block declares, whatever form they are written
/// in. Taken as text for the same reason [`tag_enumeration_stages`] is.
fn workflow_triggers(yaml: &str) -> Vec<String> {
    let on = yaml
        .split("\non:\n")
        .nth(1)
        .expect("the workflow must have an `on:` block")
        .split("\njobs:")
        .next()
        .expect("`on:` must be followed by `jobs:`");
    on.lines()
        .filter(|l| l.starts_with("  ") && !l.starts_with("    "))
        .map(|l| l.trim().trim_end_matches(':').to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// **Both classifiers, driven against the evasions a cold review named.**
///
/// Each had been keyed on the absence of one spelling — `grep -v` for the tag
/// filter, the literal `branches: [main]` for the trigger — and each was
/// therefore evadable by writing the same thing a different way. The trigger one
/// was added in the commit that fixed a can't-fail check, which is the pattern
/// worth remembering: an absence-keyed classifier is what a hurried fix reaches
/// for.
///
/// Neither of the replacements could be validated by the real files, which are
/// clean by construction. They are validated here.
#[test]
fn the_workflow_classifiers_catch_the_evasions_they_were_keyed_around() {
    // A filter that is not spelled `grep -v`. The review named `grep -vx`,
    // `sed "/$c/d"`, `jq 'map(select(. != $c))'` and `comm`; the shape they
    // share is a third pipeline stage, which is what is checked.
    let evaded = "\
      - id: tags\n\
      \x20       run: |\n\
      \x20         json=$(git tag --list 'v*.*.*' --sort=creatordate \\\n\
      \x20                | grep -vx \"$current\" \\\n\
      \x20                | jq -R -s -c 'split(\"\\n\") | map(select(length > 0))')\n\
      \x20         echo \"json=$json\"\n\
      - id: hash\n";
    assert_eq!(
        tag_enumeration_stages(evaded).len(),
        3,
        "a filter spelled any other way is still a third stage, and must be seen"
    );

    // And the clean shape is two, so the check is not simply always failing.
    let clean = "\
      - id: tags\n\
      \x20       run: |\n\
      \x20         json=$(git tag --list 'v*.*.*' --sort=creatordate \\\n\
      \x20                | jq -R -s -c 'split(\"\\n\") | map(select(length > 0))')\n\
      \x20         echo \"json=$json\"\n\
      - id: hash\n";
    assert_eq!(
        tag_enumeration_stages(clean).len(),
        2,
        "the jq program's own `|` is not a pipe; a quote-blind split reads one \
         command as two and the check miscounts what it is counting"
    );

    // **The two evasions that keep the stage count at two.** A second cold
    // review found both, after the count replaced `!contains("grep -v")` — the
    // third round of this class, and the reason the check is now equality.
    let folded_into_jq = clean.replace(
        "map(select(length > 0))",
        "map(select(length > 0)) | map(select(. != $c))",
    );
    let folded_into_git =
        clean.replace("--sort=creatordate", "--sort=creatordate --no-contains $c");
    for (name, evasion) in [
        ("a filter folded into the jq program", folded_into_jq),
        ("a filtering flag on git tag", folded_into_git),
    ] {
        let stages = tag_enumeration_stages(&evasion);
        assert_eq!(stages.len(), 2, "{name} keeps the stage count at two");
        assert_ne!(
            stages,
            tag_enumeration_stages(clean),
            "{name} must still be seen: counting stages cannot catch a filter \
             written inside one of them, which is why the real check compares \
             the commands themselves"
        );
    }

    // The block form of a trigger, which the literal `branches: [main]` misses.
    let block_form =
        "name: x\n\non:\n  pull_request:\n  push:\n    branches:\n      - main\n\njobs:\n  y:\n";
    assert_eq!(
        workflow_triggers(block_form),
        vec!["pull_request".to_string(), "push".to_string()],
        "an ordinary block-form trigger must be seen; the flow-form literal is \
         one of several spellings and forbidding it forbids nothing"
    );
    let only_pr = "name: x\n\non:\n  pull_request:\n\njobs:\n  y:\n";
    assert_eq!(workflow_triggers(only_pr), vec!["pull_request".to_string()]);
}

/// AC-11 — **after a full run, the tagged commit contains a row for its own
/// tag.**
///
/// This was the milestone's headline claim about day#118 and it was driven
/// nowhere: the only coverage of the 3b/4/4b reorder was two `contains()` scans
/// over the script's prose, which satisfy AC-27 as written and say nothing about
/// what the script does. A second cold review found a recovery instruction that
/// could not recover, in a release gate, which is what an unasserted fix costs.
///
/// Driven end to end against stubs for everything the script resolves through
/// PATH, so the assertion is about the tagged tree rather than about the source
/// order.
#[test]
fn cut_release_puts_the_measured_row_in_the_tagged_commit() {
    let dir = tempfile::tempdir().expect("a scratch dir");
    let repo = dir.path();
    let bin = repo.join("bin");
    std::fs::create_dir_all(&bin).unwrap();

    stub(&bin, "kan", "echo bafyreistubbedcid\n");
    stub(&bin, "day", "exit 0\n");
    stub(&bin, "jq", "echo 9.9.9\n");
    // `metadata` must emit JSON for the jq stub to read; everything else — the
    // build, test, clippy and fmt block, and the release build — just succeeds.
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
        "# tag\texpected-outcome\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("Cargo.toml"),
        "[package]\nname='x'\nversion='9.9.9'\n",
    )
    .unwrap();
    // The cell script is resolved relative to the working directory, so the
    // scratch repo needs one. It reports a real outcome token.
    std::fs::create_dir_all(repo.join("scripts")).unwrap();
    stub(
        &repo.join("scripts"),
        "run-migration-cell.sh",
        "echo refused-honestly\n",
    );
    run_git(repo, &["add", "-A"]);
    run_git(repo, &["commit", "-qm", "init"]);

    let script = repo_root().join("scripts/cut-release.sh");
    let mut child = std::process::Command::new("sh")
        .arg(&script)
        .arg("v9.9.9")
        .current_dir(repo)
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("the release script should be runnable");
    // The release-notes prompt reads to EOF.
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"what shipped\n")
        .unwrap();
    let out = child.wait_with_output().expect("the script should finish");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "the script should complete: {text}");

    // AC-11: the row is IN the tagged tree, not in a follow-up commit.
    let tagged = String::from_utf8(
        std::process::Command::new("git")
            .args(["show", "v9.9.9:tests/fixtures/migration-expectations.tsv"])
            .current_dir(repo)
            .output()
            .expect("git should be runnable")
            .stdout,
    )
    .unwrap();
    assert!(
        tagged.contains("v9.9.9\trefused-honestly"),
        "the tagged commit must contain a row for its own tag — that is the whole \
         of day#118, and it is what the workflow's dropped exclusion now relies \
         on. Got:\n{tagged}"
    );

    // And the claim was recorded BEFORE the tag existed, which is the ordering
    // two releases were published without.
    assert!(
        text.contains("recorded bafyreistubbedcid"),
        "the release claim must be recorded, and before the tag: {text}"
    );
}

/// **The recovery instruction actually recovers, at the site where the file is
/// staged.**
///
/// It printed `git checkout -- <path>`, which restores from the INDEX. Correct
/// at the two call sites where nothing is staged, and a no-op at the third,
/// where `git add` has already run — so a maintainer whose commit failed would
/// run the printed command, see nothing change, and meet "working tree is dirty"
/// on the next attempt, named after the thing they had just tried to fix.
///
/// Induced with a failing `pre-commit` hook, which is one of the real ways step
/// 4b's commit fails (gpg signing and an unset `user.email` are others). The
/// assertion is that the printed command, executed, leaves the tree clean —
/// checking that the script *contains* the right string would be checking day's
/// own side of the interface.
#[test]
fn the_release_scripts_recovery_instruction_actually_recovers() {
    let dir = tempfile::tempdir().expect("a scratch dir");
    let repo = dir.path();
    let bin = repo.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    stub(&bin, "kan", "echo bafyreistubbedcid\n");
    stub(&bin, "day", "exit 0\n");
    stub(&bin, "jq", "echo 9.9.9\n");
    stub(&bin, "cargo", "echo '{}'\nexit 0\n");

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
        "# tag\texpected-outcome\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("Cargo.toml"),
        "[package]\nname='x'\nversion='9.9.9'\n",
    )
    .unwrap();
    std::fs::create_dir_all(repo.join("scripts")).unwrap();
    stub(
        &repo.join("scripts"),
        "run-migration-cell.sh",
        "echo refused-honestly\n",
    );
    run_git(repo, &["add", "-A"]);
    run_git(repo, &["commit", "-qm", "init"]);
    // The commit at step 4b now fails, after `git add` has staged the row.
    stub(&repo.join(".git/hooks"), "pre-commit", "exit 1\n");

    let script = repo_root().join("scripts/cut-release.sh");
    let mut child = std::process::Command::new("sh")
        .arg(&script)
        .arg("v9.9.9")
        .current_dir(repo)
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("the release script should be runnable");
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"what shipped\n")
        .unwrap();
    let out = child.wait_with_output().expect("the script should finish");
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    assert!(
        !out.status.success(),
        "the commit failed, so the script must: {stderr}"
    );

    // premise: the row really is staged at this point — the state the wrong
    // instruction was a no-op in. Without this the test passes for the trivial
    // reason that nothing needed restoring.
    let staged = String::from_utf8(
        std::process::Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    assert!(
        staged.contains("migration-expectations.tsv"),
        "premise: the row must be STAGED when the script dies here; got {staged:?}"
    );

    // Now run exactly what it told the maintainer to run.
    let printed = stderr
        .lines()
        .find(|l| {
            l.trim_start().starts_with("git restore") || l.trim_start().starts_with("git checkout")
        })
        .unwrap_or_else(|| panic!("no recovery command was printed:\n{stderr}"))
        .trim()
        .to_string();
    let recovered = std::process::Command::new("sh")
        .arg("-c")
        .arg(&printed)
        .current_dir(repo)
        .output()
        .expect("the printed command should be runnable");
    assert!(
        recovered.status.success(),
        "`{printed}` failed: {}",
        String::from_utf8_lossy(&recovered.stderr)
    );

    let porcelain = String::from_utf8(
        std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    assert!(
        porcelain.trim().is_empty(),
        "`{printed}` was printed as the recovery and left the tree dirty:\n{porcelain}\n\
         The next run dies with \"working tree is dirty\", naming the thing the \
         maintainer just tried to fix."
    );
}

/// **No commit on this branch is unaccounted for under the demonstration rule.**
///
/// The accounting used to be a hand-written table in the design doc, and it was
/// wrong in three consecutive review rounds — first omitting the commit that
/// introduces the rule, then miscounting after that was fixed, then miscounting
/// again. Each round corrected the numbers and left the mechanism.
///
/// `CLAUDE.md` already has the rule this violated: *generate expectation tables
/// from a measurement run, then review them.* So the count is generated, and
/// what is asserted is the only thing a script can decide — that every commit is
/// either demonstrated, exempt **with a stated reason**, or prose. Whether a
/// stated reason is *true* is a judgement, and a false one has already been
/// caught by review rather than here.
///
/// Reports could-not-check rather than passing when the range is empty.
#[test]
fn every_commit_is_accounted_for_under_the_demonstration_rule() {
    let out = Command::new("python3")
        .arg(repo_root().join("scripts/demonstration-census.py"))
        .current_dir(repo_root())
        .output()
        .expect("python3 should be runnable");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !text.contains("could not check"),
        "the census had no commits to be complete about, which is not a pass: {text}"
    );
    assert!(
        out.status.success(),
        "a commit changed something other than prose, carries no \
         `Demonstrated-by:` trailer, and states no `No trailer:` reason:\n{text}"
    );
}
