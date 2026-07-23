//! `.design/rigor-as-artifact.md` AC-7, AC-8, AC-9, AC-11 — the human surface:
//! `day status` reports where the work sits, the status line reads only the
//! render cache, the cache is written by session-start and regenerates, and
//! nothing here gates or fails a session.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{claim, result_claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, git: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", git)
        // No stdin: `day status-line` reads stdin for `workspace.current_dir`,
        // and an inherited tty/pipe would block it. A null stdin is EOF, so it
        // falls back to the process cwd (this temp dir) — the real harness
        // pipes JSON instead.
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

/// A git stub answering `ls-files` and `tag` from fixed sets, matching the
/// declared pathspec/glob — the same shape `tests/assess_atom.rs` uses.
fn write_git_stub(dir: &Path, tags: &[&str], tracked: &[&str]) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            "#!/bin/sh\npattern=\"$3\"\nmatch() {{ for i in $1; do case \"$i\" in $pattern) printf '%s\\n' \"$i\";; esac; done; }}\ncase \"$1\" in\n  ls-files) match \"{}\" ;;\n  tag) match \"{}\" ;;\n  *) echo unsupported >&2; exit 1 ;;\nesac\n",
            tracked.join(" "),
            tags.join(" "),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn atom(
    slug: &str,
    cid: &str,
    inputs: &[&str],
    outputs: &[&str],
    next: &[&str],
    done: &[&str],
) -> StubClaim {
    let list = |xs: &[&str]| {
        xs.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(",")
    };
    claim(
        &format!("atom/{slug}"),
        cid,
        &format!(
            "The {slug} atom.\n\n```day-atom\n{{\"in\":[{}],\"out\":[{}],\"next\":[{}],\"done\":[{}]}}\n```\n",
            list(inputs), list(outputs), list(next), list(done),
        ),
    )
}

fn witness_schema(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes.\n\n```day-witness\n{body}\n```\n"),
    )
}

/// A design→build pipeline where the design doc exists and no code does, plus
/// a `done` criterion on build that is met.
fn design_present_build_current(dir: &Path) -> (std::path::PathBuf, std::path::PathBuf) {
    let kan = write_kan_stub(
        dir,
        &[
            atom(
                "design",
                "bafyreid",
                &["intent"],
                &["design-doc"],
                &["build"],
                &[],
            ),
            atom(
                "build",
                "bafyreib",
                &["design-doc"],
                &["code-change"],
                &["review"],
                &["design-doc"],
            ),
            witness_schema(
                "bafyreiw",
                r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"}}"#,
            ),
        ],
    );
    // design-doc present, code-change absent → build is the current atom.
    let git = write_git_stub(dir, &[], &[".design/x.md"]);
    (kan, git)
}

/// AC-7: `day status` names the current atom, its satisfied inputs, its met
/// and unmet criteria, and what follows.
#[test]
fn ac7_status_names_the_current_atom_its_inputs_criteria_and_next() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(stdout.contains("Current atom: build"), "{stdout}");
    assert!(stdout.contains("inputs satisfied: design-doc"), "{stdout}");
    // build's `done` criterion is design-doc, which is present → met.
    assert!(stdout.contains("[met] design-doc"), "{stdout}");
    assert!(stdout.contains("next: review"), "{stdout}");
    assert_eq!(out.status.code(), Some(0));
}

/// AC-3: a design doc present and no code change puts inference in `build`;
/// end-to-end through the status verb, not just the unit test.
#[test]
fn ac3_design_present_and_no_code_puts_position_in_build() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Current atom: build"), "{stdout}");
    assert!(!stdout.contains("Current atom: design"), "{stdout}");
}

/// AC-9 first half: `day hook session-start` writes the cache; the status line
/// then renders from it. Deleting the cache and re-running the hook
/// regenerates it.
#[test]
fn ac9_session_start_writes_the_cache_and_it_regenerates() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());

    let out = day(dir.path(), &kan, &git, &["hook", "session-start"]);
    assert!(out.status.success());
    let cache = dir.path().join(".day").join("statusline");
    assert!(cache.exists(), "session-start should write the cache");
    let first = std::fs::read_to_string(&cache).unwrap();
    assert!(
        first.contains("build"),
        "cached line should name the atom: {first}"
    );

    std::fs::remove_file(&cache).unwrap();
    assert!(!cache.exists());
    let out = day(dir.path(), &kan, &git, &["hook", "session-start"]);
    assert!(out.status.success());
    assert!(
        cache.exists(),
        "the cache should regenerate on the next session start"
    );
    assert_eq!(std::fs::read_to_string(&cache).unwrap(), first);
}

/// AC-9 second half: with the cache absent, `day status` still works and does
/// not error — its absence is never a failure.
#[test]
fn ac9_status_works_with_the_cache_absent() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());
    assert!(!dir.path().join(".day").exists());
    let out = day(dir.path(), &kan, &git, &["status"]);
    assert!(out.status.success(), "status must work without a cache");
    assert!(String::from_utf8_lossy(&out.stdout).contains("Current atom: build"));
}

/// AC-8: the status line reads **only** the cache. Point the kan binary at a
/// path that does not exist and it still renders, because it never invokes
/// kan — proof by the one thing that would break if it did.
#[test]
fn ac8_the_status_line_reads_only_the_cache() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());
    // Populate the cache via a real session start.
    day(dir.path(), &kan, &git, &["hook", "session-start"]);

    let missing_kan = dir.path().join("no-such-kan");
    let out = day(dir.path(), &missing_kan, &git, &["status-line"]);
    assert!(
        out.status.success(),
        "status-line must not fail when kan is absent"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("day · build"),
        "should render from the cache: {stdout:?}"
    );
}

/// AC-8 corollary: with no cache at all, the status line prints nothing and
/// still exits zero — the empty state, not an error.
#[test]
fn the_status_line_prints_nothing_when_the_cache_is_absent() {
    let dir = tempfile::tempdir().unwrap();
    let (_kan, git) = design_present_build_current(dir.path());
    let missing_kan = dir.path().join("no-such-kan");
    let out = day(dir.path(), &missing_kan, &git, &["status-line"]);
    assert!(out.status.success());
    assert!(out.stdout.is_empty(), "no cache → no output");
}

/// AC-11: `day status` exits zero even when it has findings to report (here an
/// off-sequence skip). Status reports; `day assess atom` is the gate. A status
/// that failed a script that merely asked where it is would be a blocking
/// construct by another name.
#[test]
fn ac11_status_exits_zero_even_with_an_off_sequence_finding() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom(
                "design",
                "bafyreid",
                &["intent"],
                &["design-doc"],
                &["build"],
                &[],
            ),
            atom(
                "build",
                "bafyreib",
                &["design-doc"],
                &["code-change"],
                &[],
                &[],
            ),
            witness_schema(
                "bafyreiw",
                r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"}}"#,
            ),
        ],
    );
    // code-change present, design-doc absent: build ran without a design.
    let git = write_git_stub(dir.path(), &[], &["src/lib.rs"]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Off-sequence:"), "{stdout}");
    assert_eq!(
        out.status.code(),
        Some(0),
        "status reports findings; it never gates: {stdout}"
    );
}

/// A design→build→review pipeline where the code exists (so `build` is done)
/// and inference sits at `review`. `build` has whatever assessment claims the
/// caller adds.
fn build_done_review_current(
    dir: &Path,
    build_assessments: &[StubClaim],
) -> (std::path::PathBuf, std::path::PathBuf) {
    let mut claims = vec![
        atom(
            "design",
            "bafyreid",
            &["intent"],
            &["design-doc"],
            &["build"],
            &[],
        ),
        atom(
            "build",
            "bafyreib",
            &["design-doc"],
            &["code-change"],
            &["review"],
            &[],
        ),
        atom(
            "review",
            "bafyreir",
            &["code-change"],
            &["verdict"],
            &[],
            &[],
        ),
        witness_schema(
            "bafyreiw",
            r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"}}"#,
        ),
    ];
    claims.extend_from_slice(build_assessments);
    let kan = write_kan_stub(dir, &claims);
    // Both design-doc and code-change present → build's output exists, so
    // build is no longer current; review (code-change present, verdict absent)
    // is the current atom.
    let git = write_git_stub(dir, &[], &[".design/x.md", "src/lib.rs"]);
    (kan, git)
}

/// AC-10: with the last recorded assessment (`atom/build`) implying a position
/// the inferred one has moved past, the transition is named.
#[test]
fn ac10_a_transition_past_the_last_assessed_atom_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let assessment = result_claim(
        "atom/build",
        "bafyreiassess",
        "build's done criteria met.",
        1_784_000_000_000_000,
    );
    let (kan, git) = build_done_review_current(dir.path(), &[assessment]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("last recorded assessment of `build`"),
        "{stdout}"
    );
    assert!(stdout.contains("now: review"), "{stdout}");
    assert_eq!(out.status.code(), Some(0));
}

/// AC-10 second half: with no assessment ever recorded, no transition is
/// claimed — absence of a baseline is not a change.
#[test]
fn ac10_no_assessment_means_no_transition() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = build_done_review_current(dir.path(), &[]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Current atom: review"), "{stdout}");
    assert!(
        !stdout.contains("moved on"),
        "no baseline, no transition: {stdout}"
    );
}

/// AC-10 third case: when the assessed atom is still current, position and the
/// assessment agree, so nothing is said.
#[test]
fn ac10_an_assessment_of_the_current_atom_is_not_a_transition() {
    let dir = tempfile::tempdir().unwrap();
    let assessment = result_claim(
        "atom/review",
        "bafyreiassess",
        "review in progress.",
        1_784_000_000_000_000,
    );
    let (kan, git) = build_done_review_current(dir.path(), &[assessment]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("moved on"),
        "assessed atom is still current: {stdout}"
    );
}

/// AC-14: retracting the assessment that formed the baseline changes what day
/// reports — with no code path and no file touched. Modelled the way day sees
/// a retraction: `kan show` returns only live claims, so a retracted
/// assessment is simply absent from the re-read log.
#[test]
fn ac14_retracting_the_baseline_assessment_removes_the_transition() {
    let dir = tempfile::tempdir().unwrap();
    let assessment = result_claim(
        "atom/build",
        "bafyreiassess",
        "build's done criteria met.",
        1_784_000_000_000_000,
    );

    // With the assessment live, the transition shows.
    let (kan, git) = build_done_review_current(dir.path(), &[assessment]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    assert!(String::from_utf8_lossy(&out.stdout).contains("moved on"));

    // Retract it — from day's side, the claim is simply gone from the log.
    // No day code changed, no file day owns was touched.
    let (kan, git) = build_done_review_current(dir.path(), &[]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("moved on"),
        "a retracted assessment must stop being the baseline"
    );
}

/// The global-recency rule: when several atoms have been assessed, the newest
/// by `recorded_at` is the baseline — even if an older assessment names the
/// current atom. Here `design` was assessed later than `review`, and the work
/// has moved past `design`, so the transition names `design`.
#[test]
fn the_most_recent_assessment_across_atoms_is_the_baseline() {
    let dir = tempfile::tempdir().unwrap();
    let older = result_claim(
        "atom/review",
        "bafyreio",
        "review noted.",
        1_782_000_000_000_000,
    );
    let newer = result_claim(
        "atom/design",
        "bafyrein",
        "design revisited.",
        1_784_000_000_000_000,
    );
    let (kan, git) = build_done_review_current(dir.path(), &[older, newer]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // design is not current (its output design-doc exists), and it is the
    // newest assessment, so it is the baseline.
    assert!(stdout.contains("assessment of `design`"), "{stdout}");
}

/// With no witness schema declared, status cannot infer position and says so
/// rather than claiming "no current atom" — a distinction that matters,
/// because the two have different fixes.
#[test]
fn status_reports_uncheckable_when_no_witness_schema_is_declared() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[atom(
            "build",
            "bafyreib",
            &["design-doc"],
            &["code-change"],
            &[],
            &[],
        )],
    );
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("No witness probes are declared"),);
}
