//! `.design/assess-telos.md` AC-1..AC-12 — telos assessment through the
//! shipped binary.
//!
//! The load-bearing ones here are AC-4 and AC-5: a command probe is a program
//! named by a kan claim, so "it did not run" and "no shell was involved" have
//! to be demonstrated by a probe that *would* leave a trace and then does
//! not, rather than asserted from day's own output.

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

/// A stub `git` answering the two reads a probe uses.
///
/// It **filters by the pattern it is given**, which matters: a stub that
/// returned everything regardless would make a scoped probe and an unscoped
/// one indistinguishable, and AC-7 exists precisely to tell them apart. The
/// first version of this stub did exactly that and the day#34 regression
/// test passed against unfixed code.
fn write_git_stub(dir: &Path, tags: &[&str], tracked: &[&str]) -> std::path::PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
# `git tag --list <pattern> ...` and `git ls-files -- <pathspec>` both put
# the pattern in $3, and `case` gives real glob matching.
pattern="$3"
match() {{
  for item in $1; do
    case "$item" in
      $pattern) printf '%s
' "$item" ;;
    esac
  done
}}
case "$1" in
  tag) match "{tags}" ;;
  ls-files) match "{tracked}" ;;
  *) echo "git stub: unsupported read $1" >&2; exit 1 ;;
esac
"#,
            tags = tags.join(" "),
            tracked = tracked.join(" "),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn telos_claim(slug: &str, cid: &str, witnesses: &[&str]) -> StubClaim {
    scoped_telos_claim(slug, cid, witnesses, &[])
}

/// A telos declaring witnesses and, optionally, a scope narrowing which
/// instances of each count.
fn scoped_telos_claim(
    slug: &str,
    cid: &str,
    witnesses: &[&str],
    scope: &[(&str, &str)],
) -> StubClaim {
    let list = witnesses
        .iter()
        .map(|w| format!("\"{w}\""))
        .collect::<Vec<_>>()
        .join(",");
    let scope_json = if scope.is_empty() {
        String::new()
    } else {
        let pairs = scope
            .iter()
            .map(|(w, p)| format!("\"{w}\":\"{p}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!(",\"scope\":{{{pairs}}}")
    };
    claim(
        &format!("telos/{slug}"),
        cid,
        &format!("A telos.\n\n```day-telos\n{{\"witnesses\":[{list}]{scope_json}}}\n```\n"),
    )
}

fn witness_schema(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes.\n\n```day-witness\n{body}\n```\n"),
    )
}

#[test]
fn ac1_each_declared_witness_is_named_with_a_status() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact", "design-doc"]),
            witness_schema(
                "bafyreiw",
                r#"{"published-artifact":{"tag":"v*"},"design-doc":{"path":".design/*.md"}}"#,
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("published-artifact"), "{stdout}");
    assert!(stdout.contains("design-doc"), "{stdout}");
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(stdout.contains("v1.0.0"), "{stdout}");
}

#[test]
fn ac3_a_probe_matching_nothing_is_unsatisfied_and_fails_the_run() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    // No tags at all.
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    // AC-9: an unsatisfied material probe exits non-zero.
    assert_eq!(out.status.code(), Some(1), "{stdout}");
}

/// AC-4 and AC-5, the ones that matter. The probe would create a sentinel
/// file if it ran, and would create a *different* one if a shell interpreted
/// it. Asserting day's output says "NOT RUN" proves nothing on its own; the
/// filesystem does.
#[test]
fn ac4_a_command_probe_is_not_executed_without_run() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("probe-ran");
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["passing-tests"]),
            witness_schema(
                "bafyreiw",
                &format!(
                    r#"{{"passing-tests":{{"command":"touch {}"}}}}"#,
                    sentinel.display()
                ),
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[NOT RUN]"), "{stdout}");
    assert!(stdout.contains("--run"), "{stdout}");
    assert!(
        !sentinel.exists(),
        "a command probe executed without --run being passed"
    );
    // Not-run is absence of evidence, not failure.
    assert_eq!(out.status.code(), Some(0), "{stdout}");

    // And with --run, the same probe does execute.
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(sentinel.exists(), "--run should have executed the probe");
}

/// AC-5 end to end: metacharacters arriving from a claim stay literal.
#[test]
fn ac5_metacharacters_in_a_declared_probe_never_reach_a_shell() {
    let dir = tempfile::tempdir().unwrap();
    let pwned = dir.path().join("pwned");
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["passing-tests"]),
            witness_schema(
                "bafyreiw",
                &format!(
                    r#"{{"passing-tests":{{"command":"true; touch {}"}}}}"#,
                    pwned.display()
                ),
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run"],
    );
    assert!(
        !pwned.exists(),
        "a `;` in a claim-declared probe reached a shell: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn ac7_a_probe_exceeding_the_timeout_is_killed_and_reported() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["slow"]),
            witness_schema("bafyreiw", r#"{"slow":{"command":"sleep 30"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let started = std::time::Instant::now();
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run", "--timeout", "1"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[TIMEOUT]"), "{stdout}");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "the assessment should return at the timeout rather than waiting the probe out"
    );
    // Unknown evidence is not absent evidence.
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

#[test]
fn ac2_with_no_witness_schema_day_explains_and_offers_a_starter() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[telos_claim("shipped", "bafyreit", &["published-artifact"])],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(combined.contains("schema/witness"), "{combined}");
    assert!(combined.contains("day-witness"), "{combined}");
    assert!(combined.contains("kan observe"), "{combined}");
}

#[test]
fn ac11_a_telos_without_witnesses_is_reported_as_not_checkable() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[claim("telos/vague", "bafyreit", "A telos.")]);
    let git = write_git_stub(dir.path(), &[], &[]);

    // AC-15, the premise. `telos/vague` must genuinely carry no `day-telos`
    // block, or this asserts the remedy against a telos that was witnessed all
    // along and the whole test means nothing. `tests/fallbacks.rs` states the
    // convention and ten tests there follow it; these two did not.
    let declared = String::from_utf8_lossy(
        &std::process::Command::new(&kan)
            .args(["show", "telos/vague"])
            .current_dir(dir.path())
            .output()
            .expect("the stub should answer")
            .stdout,
    )
    .to_string();
    // Two assertions, and the first is what keeps the second honest: an empty
    // or failed read also "contains no day-telos", so absence alone would be a
    // premise that cannot fail -- the shape this AC exists to stop.
    assert!(
        declared.contains("A telos."),
        "premise check must actually read the telos, or its absence proves \
         nothing: {declared}"
    );
    assert!(
        !declared.contains("day-telos"),
        "premise: the fixture telos must declare no witnesses: {declared}"
    );

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "vague"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("declares no witnesses"), "{stdout}");
    // It must route to the interview rather than hand the reader a command to
    // declare a witness itself — day#86: a trivially satisfiable witness
    // reports the telos met forever, which is worse than none.
    assert!(stdout.contains("/witness-interview vague"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "unassessable is not failing");
}

#[test]
fn ac11_a_witness_with_no_declared_probe_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["adoption"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[NO PROBE]"), "{stdout}");
    assert!(stdout.contains("adoption"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "nothing checked is not failing");
}

/// AC-12: the assessment reads. It appends nothing, and the command it prints
/// for the reader uses `kan result`'s real argument order — subject first,
/// positionally. `tests/kan_conformance.rs` proves that form is the one a
/// real kan accepts.
#[test]
fn ac12_assessing_writes_nothing_and_prints_a_runnable_record_command() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        common::appends(dir.path()).is_empty(),
        "assessing must append nothing to the log"
    );
    assert!(
        stdout.contains("kan result telos/shipped"),
        "the record command must put the subject first, positionally: {stdout}"
    );
    assert!(
        !stdout.contains("result --subject"),
        "the record command must not use --subject, which kan result rejects: {stdout}"
    );
    assert!(stdout.contains("--cites"), "{stdout}");
}

#[test]
fn ac10_assess_telos_is_a_subcommand_of_assess() {
    let dir = tempfile::tempdir().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["assess", "--help"])
        .current_dir(dir.path())
        .output()
        .expect("failed to run day assess --help");
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("telos"), "{help}");
}

/// AC-2's second half, which the missing-schema test above does not cover:
/// the probe map is *data*, so changing the claim changes what is checked
/// with no code and no config file edited — the property `schema/design-doc`
/// and `schema/docs` already have.
#[test]
fn ac2_changing_the_witness_claim_changes_what_is_checked() {
    let dir = tempfile::tempdir().unwrap();
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    // Probing tags: satisfied, because a tag exists.
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert_eq!(out.status.code(), Some(0));

    // Same telos, same binary, same working tree — only the claim changed.
    // Now it probes tracked files, and the stub tracks none.
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw2", r#"{"published-artifact":{"path":"dist/*"}}"#),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    assert!(stdout.contains("dist/*"), "{stdout}");
    assert_eq!(out.status.code(), Some(1));
}

/// Found by the adversarial review: a telos that cannot be assessed exited 0,
/// so a typo'd slug read as a clean assessment. "Could not check" must not be
/// spelled the same way as "checked and found nothing wrong".
#[test]
fn an_unassessable_telos_does_not_exit_zero() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[claim("telos/real", "bafyreit", "A telos.")]);
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "does-not-exist"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Pins the property rather than the prose: the error **names the subject it
    // concerns**, which is what lets a caller that already knows the subject
    // avoid printing it twice. This used to assert a substring of the wording
    // and broke when `honest-reads` made every subject-scoped error
    // self-describing — the behaviour it was protecting is unchanged.
    assert!(
        stdout.contains("telos/does-not-exist"),
        "the error should name the subject it concerns: {stdout}"
    );
    assert!(
        stdout.contains("declared"),
        "and should say the telos is not declared: {stdout}"
    );
    // Named exactly once — a caller that prefixes an already-self-describing
    // error is what printed `telos/bad: telos/bad: …`.
    assert_eq!(
        stdout.matches("telos/does-not-exist").count(),
        1,
        "the subject should be named once, not twice: {stdout}"
    );
    assert_eq!(
        out.status.code(),
        Some(2),
        "a check that could not run must not be indistinguishable from a clean one: {stdout}"
    );

    // An --all sweep still reports every telos it *can* assess.
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "--all"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("telos/real"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// `.design/telos-subject-shape.md` AC-7 — day#34's false positive, inverted
/// into a regression test.
///
/// The reported bug: `telos/v05-shipped` says "day v0.5 is published" and the
/// project probe `{"tag": "v*"}` matched `v0.4.0-beta.1`, so the assessment
/// said MATERIAL against a release that predates the telos. With a scope, the
/// same log and the same tags must report MISSING.
#[test]
fn ac7_a_scope_turns_the_day34_false_positive_into_a_miss() {
    let dir = tempfile::tempdir().unwrap();
    // Exactly the situation that produced the bug: a v0.4 tag, no v0.5 tag.
    let git = write_git_stub(dir.path(), &["v0.4.0-beta.1"], &[]);
    let schema = witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#);

    // Unscoped: the original false positive, preserved so the test shows the
    // difference rather than asserting the fix in isolation.
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("v05-shipped", "bafyreit", &["published-artifact"]),
            schema.clone(),
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "v05-shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(stdout.contains("v0.4.0-beta.1"), "{stdout}");

    // Scoped to this milestone: same tags, same probe kind, honest answer.
    let kan = write_kan_stub(
        dir.path(),
        &[
            scoped_telos_claim(
                "v05-shipped",
                "bafyreit",
                &["published-artifact"],
                &[("published-artifact", "v0.5*")],
            ),
            schema,
        ],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "v05-shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING]"), "{stdout}");
    assert!(
        stdout.contains("v0.5*"),
        "the report should name the scope it probed: {stdout}"
    );
    assert_eq!(out.status.code(), Some(1), "{stdout}");
}

/// `.design/telos-subject-shape.md` AC-8. A scope must never reach a command
/// probe: honouring it would let a telos claim decide what day executes, and
/// commands originate only from `schema/witness`.
///
/// Asserted on the argv actually executed, not on day's description of it —
/// the stub records what it was given.
#[test]
fn ac8_a_scope_never_alters_what_a_command_probe_executes() {
    let dir = tempfile::tempdir().unwrap();
    let git = write_git_stub(dir.path(), &[], &[]);
    let recorded = dir.path().join("argv.log");

    // The probe appends its own arguments, so the test can compare what ran.
    let probe = dir.path().join("probe.sh");
    std::fs::write(
        &probe,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> {}\nexit 0\n",
            recorded.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o755)).unwrap();

    let schema = witness_schema(
        "bafyreiw",
        &format!(
            r#"{{"passing-tests":{{"command":"{} --flag"}}}}"#,
            probe.display()
        ),
    );
    let kan = write_kan_stub(
        dir.path(),
        &[
            scoped_telos_claim(
                "shipped",
                "bafyreit",
                &["passing-tests"],
                &[("passing-tests", "SCOPE-MUST-NOT-APPEAR")],
            ),
            schema,
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "shipped", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let argv = std::fs::read_to_string(&recorded).unwrap_or_default();

    assert_eq!(
        argv.trim(),
        "--flag",
        "the scope changed the executed argv: {argv:?}"
    );
    assert!(
        !argv.contains("SCOPE-MUST-NOT-APPEAR"),
        "a telos scope reached a command probe's argv: {argv:?}"
    );
    // And the reader is told, rather than left believing the narrowing applied.
    assert!(stdout.contains("ignored"), "{stdout}");
    assert!(stdout.contains("decide what runs"), "{stdout}");
}

/// AC-2, end to end — **a telos with an any-of group exits clean on one
/// satisfied member**, through the shipped binary rather than through
/// `is_clean` alone.
///
/// The premise is asserted first, day#91-style: the fixture must really be in
/// the state where one member fails, or a green exit proves nothing. `git`
/// reports no tags, so `published-artifact` is genuinely `[MISSING]`; the
/// design doc exists, so `design-doc` is `[MATERIAL]`. Under the conjunction
/// this replaces, that combination exited non-zero.
#[test]
fn an_any_of_group_exits_clean_when_one_member_is_satisfied() {
    let dir = tempfile::tempdir().unwrap();
    let claims = vec![
        claim(
            "telos/either",
            "bafyreie",
            "Either will do.\n\n```day-telos\n{\"witnesses\":[[\"published-artifact\",\"design-doc\"]]}\n```\n",
        ),
        witness_schema(
            "bafyreiw",
            r#"{"published-artifact": {"tag": "v*"}, "design-doc": {"path": ".design/*.md"}}"#,
        ),
    ];
    let kan = write_kan_stub(dir.path(), &claims);
    // No tags, one design doc: exactly one member can be satisfied.
    let git = write_git_stub(dir.path(), &[], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "either"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    // Premise: the group really does contain a failing member.
    assert!(
        stdout.contains("[MISSING] published-artifact"),
        "premise: one member must genuinely fail, or a clean exit proves nothing: {stdout}"
    );
    assert!(
        stdout.contains("[MATERIAL] design-doc"),
        "premise: the other member must genuinely be satisfied: {stdout}"
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "one satisfied alternative is what the group declared to be enough: {stdout}"
    );
    // Without this line the report contradicts itself to a reader: a
    // `[MISSING]` above a clean exit reads as a bug.
    assert!(
        stdout.contains("any of [published-artifact | design-doc]"),
        "the report must say these are alternatives: {stdout}"
    );
}

/// AC-2 — and it still fails when *every* member fails.
#[test]
fn an_any_of_group_fails_when_no_member_is_satisfied() {
    let dir = tempfile::tempdir().unwrap();
    let claims = vec![
        claim(
            "telos/either",
            "bafyreie",
            "Either will do.\n\n```day-telos\n{\"witnesses\":[[\"published-artifact\",\"design-doc\"]]}\n```\n",
        ),
        witness_schema(
            "bafyreiw",
            r#"{"published-artifact": {"tag": "v*"}, "design-doc": {"path": ".design/*.md"}}"#,
        ),
    ];
    let kan = write_kan_stub(dir.path(), &claims);
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "either"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_ne!(
        out.status.code(),
        Some(0),
        "a disjunction is not a way to never fail: {stdout}"
    );
}

/// AC-16 — **a telos witnessed only by commands names the exact invocation.**
///
/// Premise first, day#91-style: the fixture must genuinely be in the state
/// where nothing material was checked, or the assertion is about a report that
/// had evidence anyway.
///
/// This is REQ-12's whole answer, and it is deliberately only legibility. RQ-10
/// proposed making such a telos satisfiable a second way -- by counting a
/// recorded assessment -- and RQ-11 rejected it: a witness whose evidence is
/// "someone said so" consumes a flattened verdict and makes the flattening
/// durable.
#[test]
fn a_command_only_telos_names_the_run_invocation_it_needs() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("guarded", "bafyreit", &["passing-tests"]),
            witness_schema("bafyreiw", r#"{"passing-tests":{"command":"true"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "guarded"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("[NOT RUN]"),
        "premise: nothing material may have been checked: {stdout}"
    );
    assert!(
        stdout.contains("day assess telos guarded --run"),
        "the report must name the exact invocation, not just the flag: {stdout}"
    );
    // Absence of evidence is still not failure.
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// AC-27 — **the report says its exit code is derived**, so a clean exit cannot
/// be read as a durable property of the telos.
///
/// The fine-grained witness state is the assessment; the binary is a lens over
/// it, recomputed per invocation and never stored. day already refuses to store
/// one -- `kan result` on a telos is prose, not a boolean -- and this is the
/// report refusing to imply one.
#[test]
fn the_report_says_its_verdict_is_a_reading_rather_than_a_stored_result() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("shipped", "bafyreit", &["published-artifact"]),
            witness_schema("bafyreiw", r#"{"published-artifact":{"tag":"v*"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &["v1.0.0"], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[MATERIAL]"),
        "premise: this run must actually be clean, or the disclaimer is moot: {stdout}"
    );
    assert!(
        stdout.contains("never permanently met"),
        "a clean assessment must not read as a property the telos now has: {stdout}"
    );
}

/// Writes a script that exits with `code`, for the negated-command cases.
/// A file rather than `sh -c "exit N"`: argv is split on whitespace and exec'd
/// directly, which is guardrail 1 and the reason day#125's `test -z "$(...)"`
/// workaround is correctly unexpressible.
fn exits_with(dir: &Path, name: &str, code: i32) -> std::path::PathBuf {
    let script = dir.join(name);
    std::fs::write(&script, format!("#!/bin/sh\nexit {code}\n")).unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

/// REQ-4/REQ-5 — **an absence is probeable, and is `VACUOUS` without a
/// precondition.**
///
/// day#125's guest-tree telos -- "our tooling leaves no trace on repositories we
/// are guests in" -- is satisfied by the *absence* of tracked files, and every
/// probe day had was an existence check. All three states are driven, because
/// the middle one is what makes the other two mean anything.
#[test]
fn an_absence_is_probeable_and_vacuous_without_its_precondition() {
    let probes = r#"{"leaves-no-trace": {"absent": {
        "forbidden": {"path": ".kan/*"},
        "given": {"claim": {"kind": "Decision"}}
    }}}"#;
    let run = |dir: &Path, claims: Vec<StubClaim>, tracked: &[&str]| {
        let mut all = vec![
            telos_claim("guest", "bafyreit", &["leaves-no-trace"]),
            witness_schema("bafyreiw", probes),
        ];
        all.extend(claims);
        let kan = write_kan_stub(dir, &all);
        let git = write_git_stub(dir, &[], tracked);
        let out = day(dir, &kan, &git, &["assess", "telos", "guest"]);
        (
            String::from_utf8_lossy(&out.stdout).to_string(),
            out.status.code(),
        )
    };
    let worked = || {
        vec![common::decision_claim(
            "some/work",
            "bafyreid",
            "We did work here.",
            10,
        )]
    };

    // (1) VACUOUS — day never ran here, so the absence of its files shows only
    // that the situation never arose. Deciding this from git history instead
    // would report vacuous FOREVER, because leaving no trace also leaves no
    // history of a trace.
    let dir = tempfile::tempdir().unwrap();
    let (stdout, code) = run(dir.path(), vec![], &[]);
    assert!(
        stdout.contains("[VACUOUS]"),
        "absence without a precondition establishes nothing: {stdout}"
    );
    // **The class invariant, not a bare zero.** `Verdict::is_failure` counts
    // only `Unsatisfied`, so every could-not-check verdict -- ERROR, NOT RUN,
    // TIMEOUT, VACUOUS -- exits zero alike. A cold review read AC-7 as requiring
    // VACUOUS to exit non-zero; that would make it *stricter than ERROR*, which
    // is incoherent, since an error is the stronger could-not-check. The AC was
    // wrong and is amended; whether could-not-check should affect the exit code
    // AT ALL is a real question about every one of those verdicts, filed rather
    // than settled here by giving one of them special treatment.
    let vacuous_code = code;

    // (2) SATISFIED — work happened here and left nothing tracked.
    let dir = tempfile::tempdir().unwrap();
    let (stdout, code) = run(dir.path(), worked(), &["src/lib.rs"]);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");
    assert!(stdout.contains("absent, as required"), "{stdout}");
    assert_eq!(code, Some(0), "{stdout}");

    // (3) UNSATISFIED — work happened and left a trace. The telos is violated,
    // and this is the state the whole probe exists to reach.
    let dir = tempfile::tempdir().unwrap();
    let (stdout, code) = run(dir.path(), worked(), &[".kan/seed-id"]);
    assert!(
        stdout.contains("[MISSING]") && stdout.contains("forbidden thing is present"),
        "{stdout}"
    );
    assert_eq!(code, Some(1), "{stdout}");

    // **The class invariant, asserted as a comparison rather than as a bare
    // zero.** A previous version asserted `Some(0)` with a message ABOUT the
    // class, which a cold review correctly called out: the message changed and
    // the assertion did not, so it still pinned a literal. What the rule
    // actually says is that a could-not-establish is not a finding about the
    // work, and that is only visible next to one that is.
    assert_ne!(
        vacuous_code, code,
        "a vacuous witness must not exit the same way as a probe that ran and \
         found nothing -- one is a could-not-establish and the other is a finding"
    );
    assert_eq!(
        vacuous_code,
        Some(0),
        "and it exits like ERROR, NOT RUN and TIMEOUT, which `is_failure` \
         already groups it with -- day#139 asks whether that grouping is right \
         at all, for every one of them rather than for this one"
    );
}

/// day#137 / REQ-18 — **a forbidden command must declare which non-zero exit
/// means "found nothing", and any other code is an error rather than a pass.**
///
/// `run_command` maps every non-zero exit to `Unsatisfied`, which is
/// conservative for an existence check and a false clean once inverted: a
/// mistyped pathspec exits non-zero exactly as "searched and found nothing"
/// does, so the forbidden thing would report absent.
#[test]
fn a_forbidden_command_must_declare_what_finding_nothing_looks_like() {
    let telos_and_schema = |dir: &Path, probe: &str| {
        let kan = write_kan_stub(
            dir,
            &[
                telos_claim("clean", "bafyreit", &["no-secrets"]),
                witness_schema("bafyreiw", &format!(r#"{{"no-secrets": {probe}}}"#)),
                common::decision_claim("some/work", "bafyreid", "Work happened.", 10),
            ],
        );
        let git = write_git_stub(dir, &[], &[]);
        (kan, git)
    };

    // Undeclared: refused before running, so the error cannot depend on what
    // the command happened to do.
    let dir = tempfile::tempdir().unwrap();
    let scan = exits_with(dir.path(), "scan.sh", 1);
    let (kan, git) = telos_and_schema(
        dir.path(),
        &format!(
            r#"{{"absent": {{"forbidden": {{"command": "{}"}},
                            "given": {{"claim": {{"kind": "Decision"}}}}}}}}"#,
            scan.display()
        ),
    );
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "clean", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[ERROR]") && stdout.contains("found_nothing_exit"),
        "an undeclared exit code must refuse, not guess: {stdout}"
    );

    // Declared, and the command exits with it: the forbidden thing is absent.
    let dir = tempfile::tempdir().unwrap();
    let scan = exits_with(dir.path(), "scan.sh", 1);
    let (kan, git) = telos_and_schema(
        dir.path(),
        &format!(
            r#"{{"absent": {{"forbidden": {{"command": "{}"}},
                            "given": {{"claim": {{"kind": "Decision"}}}},
                            "found_nothing_exit": 1}}}}"#,
            scan.display()
        ),
    );
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "clean", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL]"), "{stdout}");

    // day#137's case: the command fails for a DIFFERENT reason. Exit 2 is not
    // the declared "found nothing", and reporting it as absent is the false
    // clean this guards.
    let dir = tempfile::tempdir().unwrap();
    let broken = exits_with(dir.path(), "scan.sh", 2);
    let (kan, git) = telos_and_schema(
        dir.path(),
        &format!(
            r#"{{"absent": {{"forbidden": {{"command": "{}"}},
                            "given": {{"claim": {{"kind": "Decision"}}}},
                            "found_nothing_exit": 1}}}}"#,
            broken.display()
        ),
    );
    let out = day(
        dir.path(),
        &kan,
        &git,
        &["assess", "telos", "clean", "--run"],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[ERROR]") && stdout.contains("neither 0 nor"),
        "a command that failed for another reason must not read as \"found nothing\": {stdout}"
    );
    assert!(
        !stdout.contains("[MATERIAL]"),
        "and it must certainly not report the forbidden thing absent: {stdout}"
    );
}

/// A forbidden command is not executed without `--run`, exactly as a positive
/// one is not. Negation widens what can be *expressed*, never what runs.
#[test]
fn a_forbidden_command_is_not_run_without_authorization() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("it-ran");
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("clean", "bafyreit", &["no-secrets"]),
            witness_schema(
                "bafyreiw",
                &format!(
                    r#"{{"no-secrets": {{"absent": {{
                        "forbidden": {{"command": "touch {}"}},
                        "given": {{"claim": {{"kind": "Decision"}}}},
                        "found_nothing_exit": 1}}}}}}"#,
                    sentinel.display()
                ),
            ),
            common::decision_claim("some/work", "bafyreid", "Work happened.", 10),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "clean"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[NOT RUN]"), "{stdout}");
    assert!(
        !sentinel.exists(),
        "a forbidden command executed without --run: {stdout}"
    );
}

/// REQ-6 — **a witness can require evidence its own author cannot supply.**
///
/// A telos about adoption, review, or anyone else's judgement is otherwise
/// satisfiable by the person who declared it: day#86 holds that a witness which
/// cannot fail is worse than none, and one its author can satisfy at will is
/// that defect with a person in the loop. This is what makes an adoption witness
/// for the v1.0 bar mean anything.
#[test]
fn a_witness_can_exclude_evidence_authored_by_the_declaring_identity() {
    let probes = r#"{"adopted": {"claim": {"kind": "Result", "subject": "adoption",
                                           "not_authored_by": "self"}}}"#;
    // The stub signs as this DID and reports it for `kan identity did`, so
    // "self" resolves to the same key the fixture's claims carry.
    let mine = common::STUB_AUTHOR;

    // Only the author's own claim: the exclusion must bite.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("adopted", "bafyreit", &["adopted"]),
            witness_schema("bafyreiw", probes),
            common::result_claim("adoption", "bafyreia", "I used it myself.", 10),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "adopted"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[MISSING]"),
        "a telos about someone else must not be satisfiable by its own author \
         (identity {mine}): {stdout}"
    );

    // A claim from someone else, same shape otherwise: now it counts.
    let dir = tempfile::tempdir().unwrap();
    let mut foreign = common::result_claim("adoption", "bafyreib", "We shipped with it.", 20);
    foreign.author = "did:key:zSomeoneElseEntirely".to_string();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("adopted", "bafyreit", &["adopted"]),
            witness_schema("bafyreiw", probes),
            foreign,
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "adopted"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[MATERIAL]"),
        "evidence from another identity is exactly what this witness is for: {stdout}"
    );
}

/// An exclusion that cannot be resolved is an **error**, never a silent match.
///
/// If kan cannot establish the workspace identity -- a blocked keychain, a
/// missing key -- then day cannot tell whose evidence it is looking at. Matching
/// everything would turn a witness the author cannot satisfy into one they can,
/// which is the quiet check this whole milestone is about.
#[test]
fn an_unresolvable_authorship_exclusion_is_reported_rather_than_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("adopted", "bafyreit", &["adopted"]),
            witness_schema(
                "bafyreiw",
                r#"{"adopted": {"claim": {"kind": "Result", "not_authored_by": "self"}}}"#,
            ),
            common::result_claim("adoption", "bafyreia", "Something.", 10),
        ],
    );
    // Premise: kan can no longer say who we are.
    common::without_identity(dir.path());
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "adopted"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[ERROR]") && stdout.contains("could not establish"),
        "an exclusion that stopped excluding must say so: {stdout}"
    );
    assert!(
        !stdout.contains("[MATERIAL]"),
        "and must not report the witness satisfied: {stdout}"
    );
}

/// day#141: a bridge check that ERRORS is a could-not-check, never
/// "its plan could not reach it".
///
/// `record_tier` used `.unwrap_or(false)`, so an atom retracted after the
/// bridge was declared — `bridge::Error::UndeclaredAtoms`, a real state of a
/// real log — rendered as a checked-and-negative verdict day never computed.
#[test]
fn a_bridge_check_error_is_reported_as_could_not_check() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("target", "bafyreit", &["design-doc"]),
            witness_schema("bafyreiw", r#"{"design-doc":{"path":".design/*.md"}}"#),
            // The plan names an atom nobody declared, so `bridge::check`
            // errors rather than answering.
            claim(
                "bridge/broken",
                "bafyreib",
                "A bridge.\n\n```day-bridge\n{\"telos\": \"target\", \"have\": [\"intent\"], \
                 \"plan\": {\"atom\": \"ghost\"}}\n```\n",
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "target"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("could not be checked"),
        "an errored bridge check must render as could-not-check: {stdout}"
    );
    assert!(
        stdout.contains("ghost"),
        "the could-not-check line must name its cause: {stdout}"
    );
    assert!(
        !stdout.contains("could not reach"),
        "an errored check must never render as a negative verdict: {stdout}"
    );
}

/// "newest on `X`" is decided by `recorded_at`, not by iteration order.
///
/// `show_all` groups the log per subject, so the last match iterated is the
/// newest only within one subject. The earlier-listed subject here carries the
/// LATER assessment; the label must name it.
#[test]
fn newest_on_names_the_claim_with_the_latest_recorded_at() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("t", "bafyreit", &["assessment"]),
            witness_schema(
                "bafyreiw",
                r#"{"assessment":{"claim":{"kind":"Result","subject":"atom/*"}}}"#,
            ),
            // Listed first, recorded later.
            common::result_claim("atom/one", "bafyreia", "Assessed one.", 200),
            // Listed last, recorded earlier — iteration order would name this.
            common::result_claim("atom/two", "bafyreib", "Assessed two.", 100),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "t"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("newest on `atom/one`"),
        "the label must follow recorded_at, not iteration order: {stdout}"
    );
}

/// A match set with no `recorded_at` at all has no newest to name, and the
/// label must not claim one — "e.g." is the honest form, as day#112 chose
/// for tags.
#[test]
fn an_undated_match_set_is_labelled_eg_not_newest() {
    let dir = tempfile::tempdir().unwrap();
    let mut one = claim("atom/one", "bafyreia", "Assessed one.");
    one.kind = "Result".to_string();
    let mut two = claim("atom/two", "bafyreib", "Assessed two.");
    two.kind = "Result".to_string();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("t", "bafyreit", &["assessment"]),
            witness_schema(
                "bafyreiw",
                r#"{"assessment":{"claim":{"kind":"Result","subject":"atom/*"}}}"#,
            ),
            one,
            two,
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "t"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("e.g. on `atom/one` (one of 2)"),
        "undated matches must not be labelled newest: {stdout}"
    );
    assert!(
        !stdout.contains("newest on"),
        "no recorded_at means no recency claim: {stdout}"
    );
}

/// A fully retracted telos is not swept by `--all` — and the exclusion is
/// counted rather than silent.
///
/// Found on day's own log: two scratch teloi, correctly retracted after a
/// verification run, still drew a full assessment block each — including a
/// `/witness-interview` suggestion and a record command citing the
/// *retraction's* CID. The session hook already excluded them, so the two
/// surfaces disagreed about how many teloi exist.
#[test]
fn a_fully_retracted_telos_is_excluded_from_the_all_sweep_and_counted() {
    let dir = tempfile::tempdir().unwrap();
    let mut retraction = claim("telos/gone", "bafyreir", "");
    retraction.kind = "Retraction".to_string();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("alive", "bafyreit", &["design-doc"]),
            witness_schema("bafyreiw", r#"{"design-doc":{"path":".design/*.md"}}"#),
            retraction,
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "--all"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("telos/alive"),
        "the live telos is still assessed: {stdout}"
    );
    assert!(
        !stdout.contains("telos/gone"),
        "a retracted telos must not draw an assessment block: {stdout}"
    );
    assert!(
        stdout.contains("1 fully retracted telos subject(s) not assessed"),
        "the exclusion is counted, never silent: {stdout}"
    );

    // Naming it explicitly still works — exclusion is about the sweep, not
    // about the subject becoming unaddressable.
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "gone"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("telos/gone"),
        "an explicitly named retracted telos is still inspectable: {stdout}"
    );
}

/// In an `--all` sweep the run-constant coda prints once, not per telos.
///
/// The per-telos record command (whose `--cites` differs each time) stays per
/// telos; the exit-code and single-frame pedagogy — identical every time —
/// printed fourteen copies on day's own log, which trains exactly the
/// skimming it warns against. A single assessment keeps the coda attached.
#[test]
fn the_all_sweep_prints_the_run_constant_coda_once() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos_claim("first", "bafyreia", &["design-doc"]),
            telos_claim("second", "bafyreib", &["design-doc"]),
            witness_schema("bafyreiw", r#"{"design-doc":{"path":".design/*.md"}}"#),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[".design/a.md"]);

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "--all"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let codas = stdout.matches("exit code is a reading").count();
    assert_eq!(
        codas, 1,
        "the coda must print exactly once per run: {stdout}"
    );
    let record = stdout.matches("To record it:").count();
    assert_eq!(record, 2, "the record command stays per telos: {stdout}");

    // A single assessment keeps the coda attached.
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "first"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        stdout.matches("exit code is a reading").count(),
        1,
        "single-telos output is unchanged: {stdout}"
    );
}
