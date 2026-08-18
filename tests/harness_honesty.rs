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

/// The commit date of a tag, in the `%cs` form `CHANGELOG.md`'s headings use.
fn tag_commit_date(tag: &str) -> String {
    let out = Command::new("git")
        .args(["log", "-1", "--format=%cs", tag])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// **Every released tag has a `CHANGELOG.md` section, dated as the tag is
/// dated.**
///
/// The changelog is written by hand, which is the shape this repo has shipped
/// wrong repeatedly: a list nothing derives grows a hole the moment somebody
/// tags without editing it. It did, within about an hour of the file landing —
/// `v0.12.0-beta.2` was cut and the newest section stayed `v0.12.0-beta.1`,
/// with the released work still sitting under `[Unreleased]`. A reviewer caught
/// that; a reviewer is not a mechanism.
///
/// **The date is checked against `git log -1 --format=%cs <tag>`**, which is
/// what the entries were written from. A heading that disagrees is either a
/// typo or — the case worth catching — an item attributed to the wrong release,
/// which is how day#131 came to sit under `[Unreleased]` after shipping in
/// `v0.12.0-beta.1`.
///
/// **It reports could-not-check rather than passing** over an empty tag set,
/// for exactly the reason the migration check above does: a completeness
/// assertion over nothing is vacuously true, and a shallow checkout is the
/// usual cause.
#[test]
fn every_released_tag_has_a_changelog_section() {
    let tags = released_tags();
    assert!(
        !tags.is_empty(),
        "could not check: no `v*.*.*` tags are visible, so this test has \
         nothing to be complete about. That is not a pass — see \
         `every_released_tag_has_a_migration_expectation`."
    );

    let text = read("CHANGELOG.md");
    let problems: Vec<String> = tags
        .iter()
        .filter_map(|tag| {
            let heading = format!("## [{tag}] — ");
            match text.lines().find(|l| l.starts_with(&heading)) {
                None => Some(format!("{tag}: no `{heading}…` section")),
                Some(line) => {
                    let dated = line[heading.len()..].trim();
                    let actual = tag_commit_date(tag);
                    (dated != actual)
                        .then(|| format!("{tag}: section dated {dated}, tag committed {actual}"))
                }
            }
        })
        .collect();

    assert!(
        problems.is_empty(),
        "CHANGELOG.md does not account for every released tag:\n{}\n\n\
         Add the section before tagging. Which release contains an item is \
         decided by `git tag --contains <commit>`, never by an issue's close \
         date — day#131 closed six hours after the tag it shipped in.",
        problems.join("\n")
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

fn stub_new_release_gates(repo: &Path, bin: &Path) {
    let scripts = repo.join("scripts");
    stub(&scripts, "check-design-corpus.sh", "exit 0\n");
    stub(&scripts, "process-census.sh", "exit 0\n");
    let target = repo.join("target/debug");
    std::fs::create_dir_all(&target).unwrap();
    stub(&target, "day", "exit 0\n");
    stub(
        bin,
        "gh",
        "case \"$1\" in\n\
         issue) echo '{\"state\":\"CLOSED\",\"closedByPullRequestsReferences\":[{\"number\":999}]}' ;;\n\
         pr) echo '{\"mergedAt\":\"2026-08-14T00:00:00Z\"}' ;;\n\
         *) exit 97 ;;\n\
         esac\n",
    );
}

#[test]
fn release_dispositions_are_verified_before_any_build() {
    let script = read("scripts/cut-release.sh");
    let gate = script
        .find("# --- 1c. every issue in this release has a merged disposition")
        .expect("the release script must verify merged issue dispositions");
    let build = script
        .find("cargo build --workspace --all-targets")
        .expect("the release script must retain its build gate");
    assert!(
        gate < build,
        "issue dispositions must be checked before building"
    );
    assert!(
        script.contains("for issue in 177 167 162"),
        "the release disposition gate must cover issues #177, #167, and #162"
    );
    assert!(
        script.contains(".state == \"CLOSED\"") && script.contains(".mergedAt != null"),
        "a manual close or an unmerged closing PR must not satisfy the release gate"
    );
    assert!(
        script.contains("could not read issue #$issue")
            && script.contains("has no merged closing pull request"),
        "unreadable and unsatisfied dispositions must both fail closed"
    );
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
    let text = read("skills/adversarial-review/SKILL.md");
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
    let matrix = read("tests/fixtures/kan-compat.tsv");
    let newest = matrix
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let tag = fields.next()?;
            (fields.next()? == "ok").then_some(tag)
        })
        .next_back()
        .expect("the compatibility matrix must contain an ok row");
    assert!(
        yaml.contains("cargo run --quiet -p xtask -- evidence revert --verify"),
        "the job must re-derive with the native harness, not read the trailer"
    );
    assert!(
        yaml.contains(&format!("KAN_TAG: {newest}"))
            && yaml.contains("--tag \"${KAN_TAG}\" --force kan")
            && yaml.contains("test -x \"$(command -v kan)\""),
        "the verifier must install and prove the newest measured kan is present; kan-backed tests cannot re-derive from a red missing-reader baseline"
    );
    assert!(
        yaml.contains("GH_TOKEN: ${{ github.token }}"),
        "demonstrated tests may run accepted-RFC validation; the verifier must provide the same read-only GitHub authority as ordinary CI or report a red baseline before applying any revert"
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
    assert!(
        yaml.contains("grep '^Accounts-for:'")
            && yaml.contains("git rev-parse \"${named}^{commit}\"")
            && yaml.contains("[ -z \"${reason:-}\" ]")
            && yaml.contains("if [ \"$accounted\" = \"true\" ]"),
        "the verifier must honor the census's complete Accounts-for grammar, \
         including its required reason; accepting less can suppress a live \
         contradiction check, while accepting none can make retired evidence \
         permanently red"
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
    stub_new_release_gates(repo, &bin);
    stub(
        &repo.join("scripts"),
        "run-migration-cell.sh",
        "echo refused-honestly\n",
    );
    // Step 3c resolves the corpus capture the same way; the stub writes the
    // row the real script would, so the pre-tag staged state includes a NEW
    // file — the state that makes a bare `git restore` recovery incomplete,
    // since restore does not remove new files.
    stub(
        &repo.join("scripts"),
        "capture-block-corpus.sh",
        "mkdir -p tests/fixtures/block-corpus\n\
         echo '{\"body\":{},\"fence\":\"day-atom\"}' > \"tests/fixtures/block-corpus/$2.jsonl\"\n",
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
    stub_new_release_gates(repo, &bin);
    stub(
        &repo.join("scripts"),
        "run-migration-cell.sh",
        "echo refused-honestly\n",
    );
    // Step 3c resolves the corpus capture the same way; the stub writes the
    // row the real script would, so the pre-tag staged state includes a NEW
    // file — the state that makes a bare `git restore` recovery incomplete,
    // since restore does not remove new files.
    stub(
        &repo.join("scripts"),
        "capture-block-corpus.sh",
        "mkdir -p tests/fixtures/block-corpus\n\
         echo '{\"body\":{},\"fence\":\"day-atom\"}' > \"tests/fixtures/block-corpus/$2.jsonl\"\n",
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
    // And the corpus row — a NEW file, the state a bare `git restore`
    // cannot clean, which is what makes the compound recovery line necessary.
    assert!(
        staged.contains("block-corpus/v9.9.9.jsonl"),
        "premise: the corpus row must be STAGED when the script dies here; got {staged:?}"
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

/// What each census exit code means to its caller.
///
/// **Extracted so it can be driven.** The version before this gave the census a
/// fourth code (3, an empty range) and left this caller asserting the three-code
/// contract — so on `main` after a merge, where the range is legitimately empty,
/// it reported the phantom accusation the round it shipped in was named for. A
/// guarantee fixed in the mechanism and not in its only caller: day#101, fourth
/// instance, inside this milestone's own tooling.
///
/// It was unobservable from inside the test that held it, because the empty-range
/// mode is one this repo is never in — day#91, on top. A function can be handed
/// the code directly, which is what
/// [`every_census_exit_code_means_something_different_to_the_caller`] does.
fn interpret_census(code: Option<i32>, head_is_published: bool) -> Result<(), String> {
    match code {
        Some(0) => Ok(()),
        // Legitimately nothing to account for — `main` after a merge.
        //
        // **`head_is_published` is weaker than it sounds, and saying so is the
        // point.** It means "HEAD is reachable from some `origin/*` ref", which
        // every pushed branch satisfies — this branch included. It is not "HEAD
        // has no commits of its own". What makes the arm safe is the code
        // itself: exit 3 means the census found no commits between the base and
        // HEAD, so HEAD already *is* the base. The predicate is a second opinion,
        // not the guard, and an earlier comment here claimed it was the guard.
        Some(3) if head_is_published => Ok(()),
        Some(3) => Err(
            "the census found no commits to check, but HEAD is ahead of what has \
             been published, so there were commits it should have found"
                .to_string(),
        ),
        Some(2) => Err(
            "the census could not determine a range, which is a could-not-check \
             and not a pass"
                .to_string(),
        ),
        _ => Err(
            "a commit carries no `Demonstrated-by:` trailer and states no \
             `No trailer:` reason. A docs-only commit needs one too — this repo \
             executes its own documentation."
                .to_string(),
        ),
    }
}

/// Every exit code means something different to the caller, including the two
/// the caller could not see before.
#[test]
fn every_census_exit_code_means_something_different_to_the_caller() {
    assert!(
        interpret_census(Some(0), false).is_ok(),
        "0 is accounted-for"
    );
    assert!(
        interpret_census(Some(3), true).is_ok(),
        "an empty range on published HEAD is `main` after a merge, and must pass \
         — otherwise the suite is red on main forever and cut-release.sh, which \
         runs it, can never cut another release"
    );
    assert!(
        interpret_census(Some(3), false).is_err(),
        "an empty range on a branch with unpublished commits is a real failure"
    );
    assert!(
        interpret_census(Some(2), false).is_err(),
        "could-not-check is not a pass"
    );
    assert!(
        interpret_census(Some(1), false).is_err(),
        "unaccounted fails"
    );

    // And the distinctions are not cosmetic: each error says a different thing,
    // so the reader is not sent after a phantom commit when git failed.
    let unaccounted = interpret_census(Some(1), false).unwrap_err();
    let unknowable = interpret_census(Some(2), false).unwrap_err();
    assert_ne!(
        unaccounted, unknowable,
        "a git failure and a missing demonstration must not read identically — \
         that is exactly what turned CI red while accusing a commit that does \
         not exist"
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
/// either demonstrated or exempt **with a stated reason**. Whether a
/// stated reason is *true* is a judgement, and a false one has already been
/// caught by review rather than here.
///
/// Reports could-not-check rather than passing when the range is empty.
#[test]
fn every_commit_is_accounted_for_under_the_demonstration_rule() {
    let out = Command::new(repo_root().join("scripts/demonstration-census.py"))
        .current_dir(repo_root())
        .output()
        .expect("the demonstration census shim should be runnable");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    // Exit CODES, not the presence or absence of a phrase in the output. An
    // earlier version asserted `!text.contains("could not check")` — and a commit
    // subject on this branch contains that phrase, so the check fired on a clean
    // census. Keying on an absence, inside the test that exists to stop
    // hand-written evidence.
    //
    // **All four codes are handled here, and that is the point.** The version
    // before this one gave the census a fourth code (3, an empty range) and left
    // this caller asserting the three-code contract, so on `main` after a merge —
    // where the range is legitimately empty — it reported the phantom accusation
    // the round it shipped in was named for. A guarantee fixed in the mechanism
    // and not in its only caller: day#101, fourth instance, in this milestone's
    // own tooling.
    let ahead = String::from_utf8(
        Command::new("git")
            .args(["rev-list", "--count", "HEAD", "--not", "--remotes=origin"])
            .current_dir(repo_root())
            .output()
            .expect("git should be runnable")
            .stdout,
    )
    .unwrap();
    if let Err(why) = interpret_census(out.status.code(), ahead.trim() == "0") {
        panic!("{why}\n{text}");
    }
}

/// RFC 1 AC-17 — the derivative HTML is checked against a fresh rendering of
/// the canonical Markdown. Presence, a source link, and a MathJax script are
/// not freshness: all three survive when the source changes and the derivative
/// does not, which is the exact hostile mutation this test drives.
#[test]
fn stale_denotational_html_is_rejected() {
    let dir = tempfile::tempdir().expect("a scratch dir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("scripts")).unwrap();
    std::fs::create_dir_all(root.join("rfcs/1")).unwrap();
    std::fs::copy(
        repo_root().join("scripts/render-denotational-semantics.py"),
        root.join("scripts/render-denotational-semantics.py"),
    )
    .unwrap();
    std::fs::copy(
        repo_root().join("rfcs/1/denotational-semantics.md"),
        root.join("rfcs/1/denotational-semantics.md"),
    )
    .unwrap();
    std::fs::copy(
        repo_root().join("rfcs/1/denotational-semantics.html"),
        root.join("rfcs/1/denotational-semantics.html"),
    )
    .unwrap();
    let source = root.join("rfcs/1/denotational-semantics.md");
    let mut changed = std::fs::read_to_string(&source).unwrap();
    changed.push_str("\nA hostile source-only mutation.\n");
    std::fs::write(&source, changed).unwrap();

    let out = Command::new("python3")
        .args(["scripts/render-denotational-semantics.py", "--check"])
        .current_dir(root)
        .output()
        .expect("python3 should be runnable");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.status.success() && text.contains("is out of date"),
        "a source-only change must make the freshness check fail for the stated reason:\n{text}"
    );
}

/// RFC 1 AC-17 — the companion's real Decision, exact FileAt address, bytes,
/// repository, projection, and Publication claim all resolve from a no-local
/// clone. The checker's self-test mutates every coordinate and physically hides
/// the projection, so a presence-only implementation cannot satisfy this test.
#[test]
fn denotational_publication_resolves_and_rejects_hostile_mutations() {
    let out = Command::new(repo_root().join("scripts/check-rfc1-denotational-publication.py"))
        .arg("--self-test")
        .current_dir(repo_root())
        .output()
        .expect("the denotational publication checker should run");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.success()
            && text.contains("RFC 1 denotational publication: fresh clone resolved"),
        "the real publication and every hostile mutation must be checked:\n{text}"
    );
}

/// GitHub Actions checks out `https://github.com/kan-tools/day` while ordinary
/// local clones commonly retain the equivalent `.git` suffix. Publication
/// identity must canonicalize that transport spelling without accepting a
/// different repository.
#[test]
fn publication_checkers_accept_githubs_suffixless_origin() {
    let dir = tempfile::tempdir().expect("a scratch dir");
    let checkout = dir.path().join("day");
    let cloned = Command::new("git")
        .args(["clone", "--quiet", "--no-local"])
        .arg(repo_root())
        .arg(&checkout)
        .status()
        .expect("git clone should run");
    assert!(
        cloned.success(),
        "the publication fixture clone must succeed"
    );
    let configured = Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            "https://github.com/kan-tools/day",
        ])
        .current_dir(&checkout)
        .status()
        .expect("git remote set-url should run");
    assert!(
        configured.success(),
        "the suffixless CI origin must be installed"
    );

    for checker in [
        "scripts/check-rfc0-publication.py",
        "scripts/check-rfc1-denotational-publication.py",
    ] {
        std::fs::copy(repo_root().join(checker), checkout.join(checker))
            .expect("the checker under test should replace the committed fixture copy");
        let out = Command::new(checkout.join(checker))
            .current_dir(&checkout)
            .env("DAY_XTASK_MANIFEST", repo_root().join("Cargo.toml"))
            .output()
            .expect("the publication checker should run");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success(),
            "{checker} must accept GitHub's suffixless spelling of the same repository:\n{text}"
        );
    }
}

/// The ordinary push suite exercises the current published-claim tree format;
/// the compatibility matrix separately preserves the oldest supported CLI
/// pairing. Pinning ordinary CI to the floor made a fresh clone unreadable as
/// soon as kan's publication layout advanced while its CLI stayed compatible.
#[test]
fn ordinary_ci_uses_the_newest_measured_kan() {
    let matrix = read("tests/fixtures/kan-compat.tsv");
    let newest = matrix
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let tag = fields.next()?;
            (fields.next()? == "ok").then_some(tag)
        })
        .next_back()
        .expect("the compatibility matrix must contain an ok row");
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains(&format!("KAN_TAG: {newest}")),
        "ordinary CI must use newest measured kan {newest}; the compatibility matrix owns the older floor"
    );
    assert!(
        ci.contains("--tag \"${KAN_TAG}\" --force kan"),
        "a dedicated kan-cache miss must replace any stale kan binary restored by the general Rust cache"
    );
    assert!(
        ci.contains("GH_TOKEN: ${{ github.token }}"),
        "Accepted RFC validation must authenticate gh with GitHub's read-only workflow token"
    );
}

/// RFC lifecycle mutation tests must continue to reach their intended guards
/// after a proposal becomes Accepted; otherwise live review validation can
/// mask the append-only history checks the harness is meant to protect.
#[test]
fn rfc_acceptance_self_tests_survive_the_acceptance_transition() {
    let checker = read("xtask/src/validate/rfc.rs");
    assert!(
        checker.contains("recursive-publication")
            && checker.contains("- Kan-claim: bafyrecursive\\n- Authors:")
            && checker.contains("normative RFC bytes contain a claim-CID backlink"),
        "the native recursive-publication guard and its mutation must remain paired"
    );
    let out = Command::new("bash")
        .args(["scripts/check-rfcs-adrs.sh", "--self-test"])
        .current_dir(repo_root())
        .output()
        .expect("the RFC self-test should run");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.success()
            && text.contains("accepted-metadata mutation rejected")
            && text.contains("historical-renumber mutation rejected")
            && text.contains("forged-review mutation rejected")
            && text.contains("short-review mutation rejected")
            && text.contains("duplicate-vector-key mutation rejected"),
        "acceptance mutations must still reach their intended guards:\n{text}"
    );
}

/// RFC 8785 canonicalization starts from I-JSON. Duplicate names and integers
/// that cannot be represented exactly in the interoperable IEEE-754 range must
/// be rejected before `serde_json::Value` can collapse or round them.
#[test]
fn rfc1_vector_loader_rejects_ambiguous_jcs_input() {
    let source = read("rfcs/vectors/1-process-model.json");
    let cases = [
        (
            "duplicate-name.json",
            source.replacen(
                "\"subject\": \"telos/releasable\",",
                "\"subject\": \"telos/attacker\",\n      \"subject\": \"telos/releasable\",",
                1,
            ),
            "duplicate JSON property name: subject",
        ),
        (
            "unsafe-integer.json",
            source.replacen(
                "\"_version\": 3,",
                "\"_version\": 3,\n      \"unsafe\": 9007199254740993,",
                1,
            ),
            "integer is not exactly representable as IEEE-754",
        ),
        (
            "saturating-u64-integer.json",
            source.replacen(
                "\"_version\": 3,",
                "\"_version\": 3,\n      \"unsafe\": 18446744073709551615,",
                1,
            ),
            "integer is not exactly representable as IEEE-754",
        ),
        (
            "rounded-large-integer.json",
            source.replacen(
                "\"_version\": 3,",
                "\"_version\": 3,\n      \"unsafe\": 295147905179352825857,",
                1,
            ),
            "integer is not exactly representable as IEEE-754",
        ),
        (
            "unicode-noncharacter.json",
            source.replacen(
                "\"_version\": 3,",
                "\"_version\": 3,\n      \"forbidden\": \"\\uFFFF\",",
                1,
            ),
            "string contains an I-JSON noncharacter",
        ),
    ];
    let dir = tempfile::tempdir().unwrap();
    for (name, hostile, expected) in cases {
        let path = dir.path().join(name);
        std::fs::write(&path, hostile).unwrap();
        let out = Command::new("cargo")
            .args(["run", "--quiet", "-p", "xtask", "--", "validate", "vectors"])
            .arg(&path)
            .current_dir(repo_root())
            .output()
            .expect("the RFC 1 vector checker should run");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            !out.status.success() && text.contains(expected),
            "{name} must be rejected as non-I-JSON for the intended reason:\n{text}"
        );
    }
}

/// RFC 1 AC-10 — incorporated notation must remain type-distinct, and every
/// unresolved formal choice introduced by the companion must remain visible in
/// RFC 1's authoritative unresolved-question census.
#[test]
fn rfc1_formal_vocabulary_and_obligation_census_are_consistent() {
    let out = Command::new(repo_root().join("scripts/check-rfc1-formal-obligations.py"))
        .arg("--self-test")
        .current_dir(repo_root())
        .output()
        .expect("the RFC 1 formal-obligation checker should run");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.success()
            && text.contains("witness-topology-collision mutation rejected")
            && text
                .contains("missing-epistemic-site-and-telos-relative-topology mutation rejected")
            && text.contains(
                "missing-effective-realization-fragment-and-provability-ledger mutation rejected"
            ),
        "notation collisions and missing unresolved choices must be rejected:\n{text}"
    );
}

/// **An empty range is its own outcome, distinct from could-not-check.**
///
/// A census over no commits is vacuously complete, which is the failure class
/// this whole milestone is about. It must not exit 0, and it must not exit the
/// same way as "a commit is unaccounted" — a caller that cannot tell those apart
/// has to grep the output, which is how the absence-keyed assertion above got
/// written in the first place.
///
/// Induced with `HEAD..HEAD`, which is a real range with nothing in it.
#[test]
fn the_census_reports_an_empty_range_distinctly() {
    let out = Command::new(repo_root().join("scripts/demonstration-census.py"))
        .arg("HEAD..HEAD")
        .current_dir(repo_root())
        .output()
        .expect("the demonstration census shim should be runnable");

    // premise: the range really is empty — otherwise this passes for the
    // ordinary reason that the branch is clean.
    let listed = Command::new("git")
        .args(["rev-list", "HEAD..HEAD"])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    assert!(
        String::from_utf8_lossy(&listed.stdout).trim().is_empty(),
        "premise: `HEAD..HEAD` must contain no commits"
    );

    assert_eq!(
        out.status.code(),
        Some(3),
        "an empty range must exit 3 — distinct from 1 (a commit is unaccounted) \
         and from 2 (the range could not be determined at all). On `main` after \
         a merge the range is legitimately empty, and sharing a code with either \
         of the others made the check impossible to pass there — which would \
         have blocked the next release, since cut-release.sh runs the suite."
    );
}

/// **A range that cannot be determined is could-not-check, and does not look
/// like a finding.**
///
/// This is the defect that turned CI red on this branch: the census let a failed
/// `git` raise, Python exited 1, and 1 is its code for "a commit is
/// unaccounted" — so the first run outside the author's machine accused a
/// commit that does not exist. `actions/checkout` creates no local `main`, which
/// is a state this repo is never in and every CI run is.
#[test]
fn the_census_reports_an_unresolvable_range_as_could_not_check() {
    let out = Command::new(repo_root().join("scripts/demonstration-census.py"))
        .arg("no-such-ref..HEAD")
        .current_dir(repo_root())
        .output()
        .expect("the demonstration census shim should be runnable");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // premise: the ref really is absent, so the range really is unknowable.
    let resolved = Command::new("git")
        .args(["rev-parse", "--verify", "no-such-ref"])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    assert!(
        !resolved.status.success(),
        "premise: `no-such-ref` must not resolve, or this measures nothing"
    );

    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknowable range must exit 2, not 1: a git failure is not a finding \
         about a commit, and reporting it as one is could-not-check dressed as \
         checked-and-found-a-defect.\n{text}"
    );
    assert!(
        !text.contains("Traceback"),
        "and it must not crash: a stack trace is not a verdict.\n{text}"
    );
}

/// **Every workflow that runs the suite fetches what the suite needs.**
///
/// `every_commit_is_accounted_for_under_the_demonstration_rule` asks the census
/// for the range between `main` and HEAD. A checkout without full history has
/// neither `main` nor `origin/main`, so the census reports could-not-check and
/// the test refuses to call that a pass — correctly, and the job fails.
///
/// That is not hypothetical and not cheap: `release.yml` had the default depth,
/// so `v0.11.0-beta.1` was tagged and **did not publish**. `ci.yml` passed at the
/// same tag, because it already fetched full history. One workflow had been
/// taught and the other had not, which is a guarantee wired at one call site —
/// day#101 — expressed in YAML.
#[test]
fn every_workflow_that_runs_the_suite_fetches_full_history() {
    let mut offenders = Vec::new();
    let mut checked = 0;
    for entry in std::fs::read_dir(repo_root().join(".github/workflows"))
        .unwrap()
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "yml") {
            continue;
        }
        let yaml = std::fs::read_to_string(&path).unwrap();
        // Only workflows that actually run the suite need the refs.
        if !yaml.contains("cargo test --workspace") {
            continue;
        }
        checked += 1;
        if !yaml.contains("fetch-depth: 0") {
            offenders.push(path.display().to_string());
        }
    }
    // premise: at least one workflow runs the suite, or this is complete about
    // nothing. That is the vacuity the whole milestone is about, and the first
    // draft of this test guarded it with `|| true`, which is worse than not
    // guarding it.
    assert!(
        checked > 0,
        "could not check: no workflow was found running `cargo test --workspace`"
    );
    assert!(
        offenders.is_empty(),
        "these workflows run `cargo test --workspace` without full history: \
         {offenders:?}\n\n\
         The census needs `main` or `origin/main` to take a merge base from, and \
         a shallow checkout has neither — so the suite fails with a \
         could-not-check. v0.11.0-beta.1 was tagged and did not publish for \
         exactly this."
    );
}

/// **The release workflow creates the GitHub Release**, and its notes guard is
/// checked on the section rather than on the whole file.
///
/// `v0.12.0-beta.2` published to crates.io and had no GitHub Release for a day.
/// The other seventeen had been backfilled by hand in one batch, so nothing
/// looked wrong until tags were enumerated against releases — `cut-release.sh`'s
/// own lesson one step later: what is mechanized gets done, what is ritual gets
/// dropped.
///
/// Two properties, because the second is the one that rots quietly. The step
/// must exist at all; and its emptiness guard must read the extracted SECTION,
/// not the file it appends 79 link definitions to — checking after the append
/// is a guard that can never fire.
#[test]
fn the_release_workflow_creates_a_github_release_with_a_guard_that_can_fire() {
    let yaml = read(".github/workflows/release.yml");

    assert!(
        yaml.contains("gh release create"),
        "release.yml must create the GitHub Release on the tag push that \
         publishes the crate; a step beside the tag is the one that gets dropped"
    );
    assert!(
        yaml.contains("contents: write"),
        "and must declare the permission that lets it, rather than depending on \
         a repository default somebody can change without touching this file"
    );

    let guard = yaml
        .split("if ! grep -q")
        .nth(1)
        .expect("release.yml should guard against empty release notes");
    let guarded_file: String = guard
        .split_whitespace()
        .nth(1)
        .expect("the guard should name the file it checks")
        .trim_end_matches(';')
        .to_string();
    assert_eq!(
        guarded_file, "section.md",
        "the emptiness guard must read the extracted section. It read \
         `{guarded_file}` — and if that is the file the link definitions are \
         appended to, the guard is non-empty for every tag including one with \
         no section, which is a check that cannot fail."
    );
}

/// **`Accounts-for:` accounts, and does not absolve.**
///
/// The census gained an append-shaped accounting path because its only other
/// remedy was rewriting a pushed commit message — the operation kan refuses for
/// a claim and day refuses for a subject. An escape hatch is the right shape
/// here, and an unbounded one is the rule switched off, so this pins the three
/// bounds against a scratch repository rather than against day's own history:
/// an unaccounted commit is still unaccounted; naming a commit outside the span
/// does not absolve anything; and accounting never counts as *demonstrated*,
/// because appending a sentence is not running the tool.
#[test]
fn accounts_for_is_bounded_to_the_span_and_never_reads_as_demonstrated() {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@example.invalid")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@example.invalid")
            .output()
            .expect("git should be runnable");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    let census = |range: &str| {
        let out = Command::new(repo_root().join("scripts/demonstration-census.py"))
            .arg(range)
            .current_dir(repo)
            .output()
            .expect("the demonstration census shim should be runnable");
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stdout).to_string(),
        )
    };

    git(&["init", "-q", "-b", "main"]);
    git(&["commit", "-q", "--allow-empty", "-m", "base"]);
    let base = git(&["rev-parse", "HEAD"]);
    git(&[
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        "outside the span, unaccounted",
    ]);
    let outside = git(&["rev-parse", "HEAD"]);
    let span_start = git(&["rev-parse", "HEAD"]);
    git(&[
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        "the one to be accounted for",
    ]);
    let target = git(&["rev-parse", "HEAD"]);
    git(&[
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        "a retired demonstration\n\nDemonstrated-by: revert=HEAD tests=old_instrument outcome=DEMONSTRATED",
    ]);
    let demonstrated_target = git(&["rev-parse", "HEAD"]);
    let _ = base;

    // premise: unaccounted before anything accounts for it.
    let (code, out) = census(&format!("{span_start}..HEAD"));
    assert_eq!(
        code,
        Some(1),
        "premise: an unaccounted commit must fail: {out}"
    );

    // Naming a commit OUTSIDE the span absolves nothing.
    git(&[
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        &format!("reaches past the range\n\nAccounts-for: {outside} a reason\n\nNo trailer: this commit itself"),
    ]);
    let (code, out) = census(&format!("{span_start}..HEAD"));
    assert_eq!(
        code,
        Some(1),
        "an Accounts-for naming a commit outside the span must not absolve \
         anything inside it, and must not absolve the out-of-span commit \
         either: {out}"
    );

    // Naming the in-span commit accounts for it — and as EXEMPT, not
    // demonstrated.
    git(&[
        "commit",
        "-q",
        "--allow-empty",
        "-m",
        &format!(
            "accounts for both\n\nAccounts-for: {target} a stated reason\n\
             Accounts-for: {demonstrated_target} retired instrument\n\nNo trailer: this commit itself"
        ),
    ]);
    let (code, out) = census(&format!("{span_start}..HEAD"));
    assert_eq!(
        code,
        Some(0),
        "the named in-span commit must now be accounted: {out}"
    );
    assert!(
        out.contains("| demonstrated | 0 |"),
        "accounting must never read as demonstrated — appending a sentence is \
         not running the tool: {out}"
    );
    assert!(
        out.contains("accounted later: a stated reason"),
        "the reason must be surfaced for review, or the hatch is unauditable: {out}"
    );
    assert!(
        out.contains("accounted later: retired instrument"),
        "a retired trailer must be visibly reclassified rather than still \
         counted as demonstrated: {out}"
    );
}

/// **The finding census: every review finding is disposed of, or it is the
/// verdict.**
///
/// Findings were recorded as claims and their disposition written as prose,
/// from memory, once per round. That cannot fail to omit a member — round 2
/// found three severity-1 defects on `harness-footer`, the disposition claim
/// written before round 3 opened "SEVERITY 1 — BOTH FIXED" and named two, and
/// the third survived the full suite while being reported closed. A different
/// model in a different harness found it again.
///
/// `CLAUDE.md`: *a list that can be derived must be derived, and a count and a
/// list are different guarantees.* This repo derives its atom directory, its
/// block corpus and its witness map, and had never derived the one list whose
/// omissions carry ACROSS rounds.
///
/// Driven against synthetic logs rather than day's own, so the test does not
/// change meaning as real findings are disposed of — and so the un-disposed
/// case is reachable at all, which it will not be once the record is clean.
#[test]
fn the_finding_census_separates_unaccounted_from_could_not_check() {
    let script = repo_root().join("scripts/finding-census.py");
    assert!(script.is_file(), "scripts/finding-census.py should exist");

    // The census reads kan, so a stub kan supplies the log. Written as the
    // envelope kan actually emits, since that is the contract day depends on.
    let dir = tempfile::tempdir().unwrap();
    let log = |claims: &str| {
        format!(
            r#"{{"v":1,"trust":{{"base":"Solo","authors":[]}},"excluded_by_trust":0,
                 "subjects":[{{"v":1,"subject":"s","subjects":["s"],
                 "excluded_by_trust":0,"claims":[{claims}]}}]}}"#
        )
    };
    let claim = |cid: &str, text: &str| {
        format!(
            r#"{{"cid":"{cid}","kind":"Observation","author":"did:key:zA","text":{}}}"#,
            serde_json::Value::String(text.to_string())
        )
    };
    let run = |body: &str| {
        let kan = dir.path().join("kan");
        std::fs::write(&kan, format!("#!/bin/sh\ncat <<'JSON'\n{body}\nJSON\n")).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&kan, std::fs::Permissions::from_mode(0o755)).unwrap();
        let out = Command::new(&script)
            .arg("s")
            .current_dir(dir.path())
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    dir.path().display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .output()
            .expect("the finding census shim should be runnable");
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stdout).to_string(),
        )
    };

    let finding = claim(
        "bafyfinding0000000000",
        "FINDING (severity 1): a thing is wrong.",
    );

    // A finding with no disposition is THE verdict — exit 1.
    let (code, out) = run(&log(&finding));
    assert_eq!(
        code,
        Some(1),
        "an undisposed finding must be the verdict: {out}"
    );

    // Disposed of: accounted, exit 0.
    let disposed = format!(
        "{finding},{}",
        claim(
            "bafydisposition000000",
            "Disposition: bafyfinding0000000000 fixed",
        )
    );
    let (code, out) = run(&log(&disposed));
    assert_eq!(code, Some(0), "a disposed finding must be accounted: {out}");

    // `accepted` without a reason is REFUSED rather than counted — the whole
    // point is that an acceptance is reviewable.
    let unreasoned = format!(
        "{finding},{}",
        claim(
            "bafybad00000000000000",
            "Disposition: bafyfinding0000000000 accepted"
        )
    );
    let (code, out) = run(&log(&unreasoned));
    assert_eq!(
        code,
        Some(1),
        "an unreasoned acceptance must not account: {out}"
    );
    assert!(out.contains("MALFORMED"), "and must say why: {out}");

    // A subject with no findings is NOT clean — it is a separate state, so a
    // caller cannot read "nothing to account for" as "everything accounted".
    let (code, _) = run(&log(&claim("bafyplain000000000000", "An ordinary claim.")));
    assert_eq!(code, Some(3), "no findings is its own exit code");

    // And a kan that cannot be read is could-not-check, never a finding.
    let kan = dir.path().join("kan");
    std::fs::write(&kan, "#!/bin/sh\necho boom >&2\nexit 3\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&kan, std::fs::Permissions::from_mode(0o755)).unwrap();
    let out = Command::new(&script)
        .arg("s")
        .current_dir(dir.path())
        .env(
            "PATH",
            format!(
                "{}:{}",
                dir.path().display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unreadable kan is could-not-check, which outranks checked-and-clean"
    );
}

/// **A cell that could not run says so, and never manufactures a pairing fact.**
///
/// `scripts/run-kan-compat-cell.sh` sorted a failed `cargo test` by grepping the
/// log for `could not compile`: one toolchain problem was distinguished as
/// `unbuildable`, and every other one — cargo refusing to start, a killed
/// process, a broken environment — fell through to `incompatible`. Its own
/// comment says conflating those "would record a toolchain problem as a
/// compatibility fact", so the rule was stated in the right place and enforced
/// by nothing, which is the defect class `CLAUDE.md` records for prose in a doc
/// comment.
///
/// It cost a real answer: an empty `CARGO_TARGET_DIR` in the environment
/// produced `incompatible` for four consecutive kan releases that day
/// demonstrably works with, and the only thing that caught it was already
/// knowing the answer. Transcribed, it would have moved day's published floor.
///
/// The reproduction here is that exact environment, and it is hermetic: cargo
/// refuses before it builds anything, so no kan and no toolchain work is
/// needed. `/bin/echo` stands in for the binary only to clear the executable
/// check the script makes first.
#[test]
fn the_compat_cell_reports_could_not_run_rather_than_a_pairing_fact() {
    let script = repo_root().join("scripts/run-kan-compat-cell.sh");
    assert!(script.is_file(), "the cell script should exist");

    let out = Command::new(&script)
        .arg("/bin/echo")
        .current_dir(repo_root())
        // The defect verbatim. Cargo rejects an empty target directory before
        // it does any work, which is why this is fast and why the old
        // classifier never saw `could not compile`.
        .env("CARGO_TARGET_DIR", "")
        .output()
        .expect("sh should be runnable");
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    assert_ne!(
        token, "incompatible",
        "an environment that stopped cargo is not a fact about kan. This is the \
         regression: could-not-check reported as checked-and-found-a-defect, \
         and it is transcribed into tests/fixtures/kan-compat.tsv by hand.\n\
         stderr: {stderr}"
    );
    assert_eq!(
        token, "could-not-run",
        "the cell must name the non-measurement.\nstderr: {stderr}"
    );
    assert_eq!(
        out.status.code(),
        Some(2),
        "and exit non-zero: could-not-check outranks checked-and-clean, and a \
         caller reading only the token will otherwise write a row from it.\n\
         stderr: {stderr}"
    );
}

/// **The matrix refuses to compare or cache a non-measurement.**
///
/// The token is only half the guarantee. `kan-compat.yml` compares the cell's
/// outcome against the committed row and fails with "a pairing changed" — an
/// error it reserves for day having moved — so a `could-not-run` reaching that
/// step reports the wrong defect against the wrong repo. Worse, the outcome
/// file is the cache value under a key naming an immutable tag, so a
/// non-answer cached once is served to every later run.
///
/// A guarantee wired at one call site is day#101, and this is the second call
/// site.
#[test]
fn the_matrix_never_compares_or_caches_an_unmeasured_cell() {
    let yaml = read(".github/workflows/kan-compat.yml");
    let run_step = yaml
        .split("- name: Run the conformance suite against it")
        .nth(1)
        .expect("the workflow should still have the step that runs the cell")
        .split("- name:")
        .next()
        .expect("the step should be delimited by the next one");

    assert!(
        run_step.contains("if ! scripts/run-kan-compat-cell.sh"),
        "the workflow must observe the cell's exit code; reading only its stdout \
         is how a non-answer becomes a row.\n{run_step}"
    );
    assert!(
        run_step.contains("rm -f kan-compat-outcome.txt"),
        "and it must delete the outcome file before failing — a cache entry \
         under an immutable tag key is a claim that the question was \
         answered.\n{run_step}"
    );
    assert!(
        run_step.contains("exit 1"),
        "and fail the cell, rather than falling through to the comparison \
         step.\n{run_step}"
    );
}

/// **A path nothing was written to is a non-measurement, not a tag verdict.**
///
/// This test used to assert something wider — that a non-kan executable is
/// refused — and that assertion is gone deliberately rather than quietly. Two
/// preflights were tried and neither can carry the claim: `--help` reports
/// argv[0] rather than the program, and behavioural identity is
/// indistinguishable from "kan too old to do what day needs" for every tag
/// before 0.9.1, which is what nine `incompatible` rows record.
///
/// So a non-kan executable supplied by hand now yields `incompatible`, and the
/// guarantee that CI never supplies one lives in
/// `the_matrix_installs_kan_from_a_pinned_source`. That is deliberately NOT
/// asserted here: it is a behaviour this repo accepts, not one it wants, and
/// pinning it with a test would read as an endorsement.
///
/// What survives is the half that is genuinely the cell's to know. A missing
/// path means the caller did not build kan, which the cell cannot tell from a
/// tag that will not build — so it refuses to guess.
#[test]
fn the_cell_reports_a_missing_binary_as_a_non_measurement() {
    let script = repo_root().join("scripts/run-kan-compat-cell.sh");

    let out = Command::new(&script)
        .arg("/nonexistent/kan")
        .current_dir(repo_root())
        .output()
        .expect("sh should be runnable");
    let token = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_ne!(
        token, "unbuildable",
        "a missing path is the caller not having built kan; this script cannot \
         tell that from a tag that will not build, and must not guess"
    );
    assert_eq!(token, "could-not-run");
    assert_eq!(out.status.code(), Some(2));
}

/// **`unbuildable` is not in the cell's vocabulary at all.**
///
/// The narrower guarantee behind the test above, asserted on the source because
/// it is a property of what the script *can* say rather than of one input. That
/// outcome belongs to the workflow's install step, which decides whether the
/// TAG builds before the cell is invoked.
#[test]
fn the_cell_cannot_emit_a_verdict_about_whether_kan_builds() {
    let text = read("scripts/run-kan-compat-cell.sh");
    let emitting: Vec<&str> = text
        .lines()
        .filter(|l| l.contains("echo \"unbuildable\"") || l.trim() == "echo unbuildable")
        .collect();
    assert!(
        emitting.is_empty(),
        "the cell emits `unbuildable`, which is a claim about the kan tag it \
         was handed rather than about the pairing it measured: {emitting:?}"
    );
}

/// **The matrix distinguishes "kan did not build" from "kan could not be
/// built here".**
///
/// The workflow mapped every nonzero `cargo install` — DNS failure, registry
/// outage, killed process, full disk — to `unbuildable`, then wrote and cached
/// it under a key naming an immutable tag. A cold review hit precisely that: its
/// first attempt failed to resolve the host. Keyed on the positive signal, the
/// same rule the cell uses.
#[test]
fn the_matrix_does_not_publish_an_infrastructure_failure_as_a_tag_verdict() {
    let yaml = read(".github/workflows/kan-compat.yml");
    let build_step = yaml
        .split("- name: Build this kan")
        .nth(1)
        .expect("the workflow should still build kan")
        .split("- name:")
        .next()
        .expect("delimited by the next step");

    assert!(
        build_step.contains("could not compile"),
        "`unbuildable` must be keyed on cargo reaching compilation and failing, \
         not on any nonzero exit:\n{build_step}"
    );
    assert!(
        build_step.contains("could-not-run"),
        "and everything else must be named as a non-measurement:\n{build_step}"
    );

    let run_step = yaml
        .split("- name: Run the conformance suite against it")
        .nth(1)
        .expect("the workflow should still run the cell")
        .split("- name:")
        .next()
        .expect("delimited by the next step");
    assert!(
        run_step.contains("could-not-run") && run_step.contains("exit 1"),
        "and a could-not-run install must fail the cell rather than falling \
         through to a row:\n{run_step}"
    );
}

/// **The matrix establishes kan's identity by pinning the source, which is the
/// only place that guarantee can hold.**
///
/// A cold review found the cell publishing `incompatible` for a non-kan
/// executable — a durable fact about a pairing nobody measured. Two preflights
/// were tried and both failed, for reasons that are properties of the problem
/// rather than of the attempts:
///
/// - `--help` cannot identify a program. clap derives the usage line from
///   argv[0], so a real kan invoked under another filename fails the check and
///   any binary placed at a path named `kan` passes it. Measured: the same kan
///   binary printed `Usage: kan-bin-v0.12.0-beta.4`.
/// - Behaviour cannot identify it either. For every kan before 0.9.1, "not kan"
///   and "kan too old to do what day needs" are the same observation — which is
///   what the nine `incompatible` rows record. Any check strict enough to catch
///   an impostor rejects the genuine old versions the table exists to hold.
///
/// So the guarantee is asserted where it is actually made. The matrix installs
/// kan from a pinned git tag, which establishes provenance before the cell runs.
/// Deleting the preflight without pinning this would have been removing a check
/// and calling it a design decision.
#[test]
fn the_matrix_installs_kan_from_a_pinned_source() {
    let yaml = read(".github/workflows/kan-compat.yml");
    let build_step = yaml
        .split("- name: Build this kan")
        .nth(1)
        .expect("the workflow should still build kan")
        .split("- name:")
        .next()
        .expect("delimited by the next step");

    assert!(
        build_step.contains("--git https://github.com/kan-tools/kan"),
        "kan must come from its own repository, not from whatever is on the \
         runner: this is the identity guarantee the cell deliberately does not \
         re-derive.\n{build_step}"
    );
    assert!(
        build_step.contains("--tag \"${{ matrix.kan }}\""),
        "and pinned to the tag the row is about, so the binary measured is the \
         one the row names.\n{build_step}"
    );

    // And the cell must not have grown a preflight back, which would re-assert
    // a guarantee it cannot make.
    let cell = read("scripts/run-kan-compat-cell.sh");
    assert!(
        !cell.contains("does not name itself kan"),
        "the cell must not claim to identify kan; that check was removed \
         because neither `--help` nor behaviour can carry it"
    );
}

/// **The cell runs where `timeout` does not exist.**
///
/// `timeout` is GNU coreutils. Stock macOS does not ship it, and this repo is
/// developed on macOS — so making it a hard dependency would mean the cell
/// reports could-not-run on the maintainer's own machine, which is honest and
/// useless. The bound degrades, and says so on stderr rather than silently,
/// because an unannounced degrade removes the guarantee precisely where nobody
/// is looking.
#[test]
fn the_cell_does_not_require_gnu_coreutils() {
    let text = read("scripts/run-kan-compat-cell.sh");
    // **The failure message quotes a line, not the file.** Dumping the whole
    // script here made `scripts/revert-demo.py` report DID-NOT-COMPILE for a
    // test that merely failed: the script's own comments contain the phrase
    // `could not compile`, and the harness keys that outcome on finding it
    // anywhere in the combined output. Filed as a harness defect; the fix here
    // is that an assertion message should be readable anyway.
    assert!(
        text.contains("command -v timeout"),
        "the cell must check for `timeout` rather than assuming it; no \
         `command -v timeout` line found in scripts/run-kan-compat-cell.sh"
    );
    assert!(
        text.contains("will NOT be bounded"),
        "and must say so on stderr when it is absent — a silent degrade is the \
         failure mode, not the missing binary"
    );
    assert!(
        !text.contains("command -v timeout >/dev/null 2>&1 || die"),
        "and must not make it fatal: stock macOS has no `timeout`, and this \
         repo is developed there"
    );
}

/// **The native census consumes the shared trailer grammar.**
///
/// The old Python census duplicated `revert-demo.py`'s regex. That is the shape
/// that let one tool accept a claim the other could not replay. The migrated
/// census must call the shared Rust parser, and its compatibility file must be
/// delegation only rather than a third grammar.
#[test]
fn the_native_census_uses_the_shared_trailer_grammar() {
    let census = read("xtask/src/evidence/demonstration_census.rs");
    let grammar = read("xtask/src/evidence/trailer.rs");
    let shim = read("scripts/demonstration-census.py");
    assert!(
        census.contains("trailer::parse_message"),
        "the census must consume the shared parser"
    );
    assert!(
        grammar.contains("plain_and_scoped_trailers_share_one_grammar")
            && grammar.contains("fabricated_or_ambiguous_claims_are_rejected"),
        "the shared grammar must retain positive and hostile fixtures"
    );
    assert!(
        shim.contains("census demonstrations") && !shim.contains("TRAILER_RE"),
        "the compatibility path must delegate without retaining policy"
    );
}
