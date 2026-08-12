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

/// AC-8, and the harness footer's AC-12: the status line makes **zero** kan
/// and **zero** git invocations, *asserted against counting stubs*.
///
/// **Counting, not missing.** This pointed both binaries at nonexistent
/// paths, which only catches an invocation whose failure is fatal — adding a
/// `Git::sync_state()` and a `client.identity()` to the `status-line`
/// handler, discarding both results, left it passing. That is the same
/// mistake `user_prompt` made and the same fix it got: pin the *invocation
/// count*, which measures the design, not a duration or a survival, which
/// measure the machine. The footer keeps growing reads, and every one of them
/// belongs to the session-start hook (REQ-10); this is what keeps that true.
#[test]
fn ac8_the_status_line_reads_only_the_cache() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, git) = design_present_build_current(dir.path());
    // Populate the cache via a real session start.
    day(dir.path(), &kan, &git, &["hook", "session-start"]);

    let (counting_kan, kan_calls) = common::write_counting_stub(dir.path(), "kan", &kan);
    let (counting_git, git_calls) = common::write_counting_stub(dir.path(), "git", &git);
    let out = day(dir.path(), &counting_kan, &counting_git, &["status-line"]);
    assert!(out.status.success(), "status-line must exit zero");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("atom: build"),
        "should render from the cache: {stdout:?}"
    );
    assert_eq!(
        common::stub_calls(&kan_calls),
        0,
        "the status line invoked kan; it has 300ms before Claude Code cancels \
         it, and every read belongs in the hook"
    );
    assert_eq!(
        common::stub_calls(&git_calls),
        0,
        "the status line invoked git; same budget, same rule"
    );

    // Non-vacuity: the counters must be capable of counting. The hook, which
    // legitimately reads both, moves them.
    day(
        dir.path(),
        &counting_kan,
        &counting_git,
        &["hook", "session-start"],
    );
    assert!(
        common::stub_calls(&kan_calls) > 0 && common::stub_calls(&git_calls) > 0,
        "the counting stubs never counted anything, so the assertions above \
         prove nothing"
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

/// The assessment baseline answers from the one bulk read: kan invocation
/// count does not scale with the number of declared atoms.
///
/// The 2026-08 review costed `last_assessed_atom`'s loop as one subprocess
/// per atom — the pre-day#71 shape — and the loop is in fact memo-served, so
/// the finding was false. This pins the property so the next reader gets an
/// assertion instead of an argument, and a regression back to per-subject
/// subprocess reads fails here rather than re-adding ~50ms per atom to every
/// session start with the suite green. An invocation count, not a duration:
/// a count measures the design, a duration measures the machine.
#[test]
fn the_assessment_baseline_does_not_cost_a_read_per_atom() {
    let count_with = |n_atoms: usize| -> usize {
        let dir = tempfile::tempdir().unwrap();
        let mut claims = vec![witness_schema(
            "bafyreiw",
            r#"{"design-doc":{"path":".design/*.md"}}"#,
        )];
        for i in 0..n_atoms {
            claims.push(atom(
                &format!("a{i}"),
                &format!("bafyreia{i}"),
                &[],
                &["thing"],
                &[],
                &[],
            ));
            claims.push(result_claim(
                &format!("atom/a{i}"),
                &format!("bafyreir{i}"),
                "assessed.",
                1_784_000_000_000_000 + i as i64,
            ));
        }
        let kan = write_kan_stub(dir.path(), &claims);
        // A wrapper that counts every kan invocation, then delegates to the
        // real stub — the same mechanism `tests/honest_reads.rs` uses.
        let counter = dir.path().join("kan-calls");
        let counting = dir.path().join("kan-counting.sh");
        std::fs::write(
            &counting,
            format!(
                "#!/bin/sh\nprintf 'x\\n' >> {}\nexec {} \"$@\"\n",
                counter.display(),
                kan.display()
            ),
        )
        .unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&counting, std::fs::Permissions::from_mode(0o755)).unwrap();
        let git = write_git_stub(dir.path(), &[], &["src/lib.rs"]);
        let out = day(dir.path(), &counting, &git, &["status"]);
        assert_eq!(out.status.code(), Some(0));
        std::fs::read_to_string(&counter)
            .map(|s| s.lines().count())
            .unwrap_or(0)
    };
    let small = count_with(2);
    let large = count_with(8);
    assert!(small > 0, "the counting stub was never invoked");
    assert_eq!(
        small, large,
        "kan invocations scale with the number of atoms ({small} for 2, {large} for 8) — \
         a per-subject subprocess read crept back in"
    );
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

/// The `session-notice` hook emits a `systemMessage`-only JSON payload on a
/// transition — the human-facing event marker — and it is valid JSON carrying
/// the notice, never a blocking construct.
#[test]
fn session_notice_emits_systemmessage_json_on_a_transition() {
    let dir = tempfile::tempdir().unwrap();
    let assessment = result_claim(
        "atom/build",
        "bafyreiassess",
        "build assessed.",
        1_784_000_000_000_000,
    );
    let (kan, git) = build_done_review_current(dir.path(), &[assessment]);
    let out = day(dir.path(), &kan, &git, &["hook", "session-notice"]);
    assert!(out.status.success(), "hooks always exit zero");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("must be JSON: {e}: {stdout}"));
    let msg = value["systemMessage"]
        .as_str()
        .expect("a systemMessage field");
    assert!(msg.contains("`build`"), "names the assessed atom: {msg}");
    assert!(msg.contains("moved to review"), "{msg}");
    // Advisory: nothing that reads as a decision.
    assert!(value.get("decision").is_none() && value.get("continue").is_none());
}

/// With no transition and nothing off-sequence, the notice hook is silent —
/// a quiet session shows no notice at all, not an empty `{}`.
#[test]
fn session_notice_is_silent_when_there_is_nothing_to_mark() {
    let dir = tempfile::tempdir().unwrap();
    // No assessment recorded, so no baseline, so no transition.
    let (kan, git) = build_done_review_current(dir.path(), &[]);
    let out = day(dir.path(), &kan, &git, &["hook", "session-notice"]);
    assert!(out.status.success());
    assert!(
        out.stdout.is_empty(),
        "a quiet session emits nothing: {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
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

/// day#95 — **`day status` exits zero even when it cannot read the log at
/// all.**
///
/// `--help` states "always exits zero", and the contract is load-bearing:
/// anything wiring this into a hook, a prompt or a CI step is entitled to rely
/// on it, and a position report that can fail a step is not advisory —
/// `telos/affordance-not-enforcement` forbids exactly that. It exited 2.
///
/// The fixture is [`common::unreadable_kan`], not [`common::missing_kan`]:
/// with kan absent, `probe` fails first and this path is never reached. Every
/// existing test used a working stub or no kan at all, so the mode the defect
/// lives in had no fixture — which is why it survived. The premise is asserted
/// below: the same stub must make `day doctor` fail, or "status exits zero"
/// proves nothing.
#[test]
fn day95_status_exits_zero_and_explains_itself_when_the_log_cannot_be_read() {
    let dir = tempfile::tempdir().unwrap();
    let kan = common::unreadable_kan(dir.path());
    let git = write_git_stub(dir.path(), &[], &[]);

    // Premise: this really is a kan whose log cannot be read.
    let doctor = day(dir.path(), &kan, &git, &["doctor"]);
    assert_ne!(
        doctor.status.code(),
        Some(0),
        "premise: doctor must fail on this fixture, or status exiting zero is \
         not evidence of anything"
    );

    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "`day status` documents \"always exits zero\": {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        stdout.contains("could not be read"),
        "and must not go quiet about why: {stdout}"
    );
    assert!(
        stdout.contains("day doctor"),
        "pointing at the verb whose job is to diagnose: {stdout}"
    );
}

/// day#95 — `day init` may not claim the log reads when it never tried.
///
/// It printed `kan: reachable` after probing only that the binary runs, so
/// day's two health checks disagreed about the same repo — and the one that
/// disagreed is the one a new project runs first.
#[test]
fn day95_init_does_not_claim_a_log_it_could_not_read() {
    let dir = tempfile::tempdir().unwrap();
    let kan = common::unreadable_kan(dir.path());
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["init", "--print"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("kan: reachable"),
        "init must not report a log it never read: {stdout}"
    );
    assert!(
        stdout.contains("could not be read"),
        "it must say what it found instead: {stdout}"
    );
    // The wiring steps are what you need in order to fix this, so they are
    // still printed — degrading is not withholding.
    assert!(stdout.contains("claude mcp add day"), "{stdout}");

    // F6, from the cold review: the honest report is not for `--print` only.
    // The default invocation died with a bare `error: … (exit status: 1)`,
    // because the recording path ran before the message was reached — so the
    // flag nobody passes explained itself and the one everybody uses did not.
    // That is the exact state day#95 describes.
    let bare = day(dir.path(), &kan, &git, &["init"]);
    let bare_out = String::from_utf8_lossy(&bare.stdout);
    assert!(
        bare_out.contains("could not be read"),
        "`day init` without --print must explain itself too: {bare_out}{}",
        String::from_utf8_lossy(&bare.stderr)
    );
    assert!(
        bare_out.contains("NOT recorded"),
        "and must say the baseline was skipped rather than imply it landed: {bare_out}"
    );
    assert_ne!(
        bare.status.code(),
        Some(0),
        "it did not do what was asked, so the exit code must say so: {bare_out}"
    );

    // And the readable case still says so plainly.
    let good = tempfile::tempdir().unwrap();
    let ok_kan = write_kan_stub(good.path(), &[]);
    let ok = day(good.path(), &ok_kan, &git, &["init", "--print"]);
    let ok_out = String::from_utf8_lossy(&ok.stdout);
    assert!(ok_out.contains("kan: reachable"), "{ok_out}");
    assert!(!ok_out.contains("could not be read"), "{ok_out}");
}

/// day#192 — **the `schema/witness` prompt carries the caveat the command it
/// hands over actually needs.**
///
/// A `path` probe is meaningful only when the work CREATES the file. If the file
/// exists before the atom runs — scaffolding, a template, a file the work
/// appends to — the probe is satisfied from the start and can never be false.
/// That is a witness which cannot fail, which day#86 holds is worse than none,
/// and the failure is silent: the probe parses, matches, and reports the atom
/// done before anything has happened.
///
/// This message is printed at exactly the moment someone is about to declare
/// probes and has not yet, so it is where the wrong answer gets typed. The
/// guidance shipped with no test, which is this repo's recorded pattern — a
/// requirement whose artifact is prose fails nothing, so nothing catches its
/// removal. Asserted on the rendered output rather than on the source string,
/// because what matters is that a person reading the prompt sees it.
#[test]
fn the_witness_prompt_warns_that_a_path_probe_needs_the_work_to_create_the_file() {
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
    let text = String::from_utf8_lossy(&out.stdout).to_string();

    assert!(
        text.contains("No witness probes are declared"),
        "premise: this is the prompt the caveat attaches to:\n{text}"
    );
    assert!(
        text.contains("meaningful only when the work CREATES the file"),
        "the caveat must reach the person about to type the command, not only \
         docs/CONVENTIONS.md:\n{text}"
    );
    assert!(
        text.contains("claim` probe"),
        "and it must name the alternative, since a caveat with no remedy just \
         makes the reader hesitate:\n{text}"
    );
}
