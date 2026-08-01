//! `.design/honest-reads.md` — day reports the declarations it cannot account
//! for, end to end.
//!
//! The unit tests in `src/atoms.rs`'s `version_gate` module cover the contract
//! itself across all seven block types. These cover what only a real run can:
//! that the refusal reaches an actual verb's output, that one unreadable
//! declaration costs only itself, and that a failed *read* is never reported as
//! an absent artifact.
//!
//! Both reproductions in AC-2 were run against the shipped binary before this
//! milestone and passed at exit 0, which is why they are the fixtures.

#![cfg(unix)]

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::{claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", write_git_stub(dir))
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

/// A minimal git stub: one `v*` tag and no changed files. `assess docs` reads
/// git for the release boundary, so without this the whole assessment errors
/// before producing a report — and a test asserting on an empty stdout would
/// have "passed" for the wrong reason on the negative half.
fn write_git_stub(dir: &Path) -> PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        r#"#!/bin/sh
case "$1" in
  tag) printf 'v9.9.9\n' ;;
  diff|ls-files|log) ;;
  *) exit 0 ;;
esac
"#,
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

/// A `kan` stub that **fails** on the named subjects and serves the rest, so a
/// read error can be told apart from an empty result — the distinction day#81
/// was about.
/// A kan whose reads fail.
///
/// **Since day#71 the failure is the whole log, not one subject.** day makes a
/// single `kan show --all --json`, so "this subject is unreadable while the
/// others are fine" is a kan that can no longer exist, and a stub modelling it
/// would test a path day does not have. The guards therefore fail the bulk read
/// as well as the named subjects — which is what a real unreadable log does.
///
/// Worth stating plainly, because it is a capability day traded away: day can
/// no longer distinguish *one* unreadable subject from a readable log that
/// simply lacks it. If kan's `show --all` ever silently omits a subject it
/// could not read, day would see an absence rather than an error. That is a
/// question for kan's contract (does ADR-71 guarantee all-or-nothing?), not
/// something day can check from its side.
fn write_failing_kan_stub(dir: &Path, claims: &[StubClaim], fail_on: &[&str]) -> PathBuf {
    let real = write_kan_stub(dir, claims);
    let wrapper = dir.join("kan-failing.sh");
    let mut guards: String = fail_on
        .iter()
        .map(|s| {
            format!("  if [ \"$1\" = \"show\" ] && [ \"$2\" = \"{s}\" ]; then echo 'kan: could not decrypt log' >&2; exit 1; fi\n")
        })
        .collect();
    if !fail_on.is_empty() {
        guards.push_str(
            "  if [ \"$1\" = \"show\" ] && [ \"$2\" = \"--all\" ]; then echo 'kan: could not decrypt log' >&2; exit 1; fi\n",
        );
    }
    std::fs::write(
        &wrapper,
        format!("#!/bin/sh\n{guards}exec {} \"$@\"\n", real.display()),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
    wrapper
}

fn atom_block(slug: &str, cid: &str, body: &str) -> StubClaim {
    claim(
        &format!("atom/{slug}"),
        cid,
        &format!("The {slug} atom.\n\n```day-atom\n{body}\n```\n"),
    )
}

/// AC-2, first half: an atom declaring a field this day does not know is
/// refused, where it used to load as though the field were absent and report
/// `composition: ok` at exit 0.
///
/// `requires` stands for whatever a later day adds. The point is not that
/// `requires` is meaningful — it is that day cannot know whether it is, and so
/// must not certify a vocabulary it read only part of.
#[test]
fn ac2_an_atom_declaring_an_unknown_field_is_refused_not_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "design",
                "bafyd",
                r#"{"in":["intent"],"out":["design-doc"],"next":["build"],"requires":["approval"]}"#,
            ),
            atom_block(
                "build",
                "bafyb",
                r#"{"in":["design-doc"],"out":["code-change"],"next":[]}"#,
            ),
        ],
    );

    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let all = format!("{stdout}{stderr}");

    assert!(
        all.contains("requires"),
        "the refusal should name the field day could not account for: {all}"
    );
    assert!(
        !all.contains("composition: ok"),
        "day must not certify a vocabulary it read only part of: {all}"
    );
    assert_ne!(
        out.status.code(),
        Some(0),
        "an unreadable declaration is not a clean run: {all}"
    );

    // AC-5: the readable atom still resolves — one unreadable declaration costs
    // itself, not the vocabulary.
    assert!(
        all.contains("atom/build") || all.contains("build"),
        "the readable atom should still load: {all}"
    );
}

/// AC-2's negative control, and the half that makes the test above mean
/// something: the same vocabulary **without** the unknown field is clean.
#[test]
fn ac2_the_same_vocabulary_without_the_unknown_field_is_clean() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "design",
                "bafyd",
                r#"{"in":["intent"],"out":["design-doc"],"next":["build"]}"#,
            ),
            atom_block(
                "build",
                "bafyb",
                r#"{"in":["design-doc"],"out":["code-change"],"next":[]}"#,
            ),
        ],
    );
    let out = day(dir.path(), &kan, &["doctor"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("composition: ok"), "{stdout}");
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// AC-4: the two causes are distinguishable in real output, and the reader is
/// told which one is theirs to fix.
///
/// A block declaring a newer `_version` is valid JSON that this day is simply
/// too old to read — telling that reader their claim is malformed sends them to
/// fix something that is not broken, which is day#60's misdirection repeated.
#[test]
fn ac4_version_skew_and_a_broken_block_read_differently() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
            atom_block("broken", "bafyx", r#"{"in":["a"],"out":["b"],}"#),
            atom_block("fine", "bafyok", r#"{"in":["a"],"out":["b"]}"#),
        ],
    );
    let out = day(dir.path(), &kan, &["doctor"]);
    let all = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let skew = all
        .lines()
        .find(|l| l.contains("atom/future"))
        .unwrap_or_default();
    assert!(
        skew.contains("upgrade day"),
        "a too-new block should tell the reader their binary is behind: {all}"
    );
    let broken = all
        .lines()
        .find(|l| l.contains("atom/broken"))
        .unwrap_or_default();
    assert!(
        !broken.contains("upgrade day"),
        "a malformed block must not tell the reader to upgrade: {all}"
    );

    // AC-5: two unreadable declarations, and the third still loads.
    assert!(
        all.contains("atom/fine") || all.contains("fine "),
        "the readable atom should still load: {all}"
    );
    // Each unreadable subject is named exactly once — prefixing an error that
    // already names its subject is what printed `telos/bad: telos/bad: …`.
    assert_eq!(
        all.matches("atom/future").count(),
        1,
        "the subject should be named once, not twice: {all}"
    );
}

/// AC-9, restated for the read day actually makes.
///
/// **The invariant is unchanged: day never reports an absence it did not
/// verify.** What changed with day#71 is the granularity. day used to read a
/// subject at a time, so one unreadable subject degraded to `[UNCHECKED]`
/// inside an otherwise complete report. It now makes a single
/// `kan show --all --json`, so a read either succeeds for everything or fails
/// for everything, and the honest response to the latter is to fail loudly
/// rather than to publish a report built on nothing.
///
/// This asserts the property rather than the old shape: on an unreadable log
/// day must (a) exit 2, because could-not-check outranks
/// checked-and-found-something, (b) name the cause, and (c) **never** state
/// that anything was absent. (c) is the one that matters — a false negative
/// here is exactly the defect the honest-reads milestone exists to prevent.
///
/// Per-subject read failure is gone as a *capability*, not just as a test
/// fixture, and that is a real trade recorded in `write_failing_kan_stub`: if
/// kan's `show --all` were ever to silently omit a subject it could not read,
/// day would see an absence rather than an error and could not tell.
#[test]
fn ac9_an_unreadable_log_is_an_error_never_a_false_absence() {
    let docs_schema = claim(
        "schema/docs",
        "bafyds",
        "Docs schema.\n\n```day-docs\n{\"version_source\":\"Cargo.toml\",\"version_key\":\"version\"}\n```\n",
    );

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "version = \"9.9.9\"\n").unwrap();
    let claims = [
        docs_schema.clone(),
        claim("release", "bafyr", "v9.9.9 published."),
    ];
    let kan = write_failing_kan_stub(dir.path(), &claims, &["release"]);

    let out = day(dir.path(), &kan, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        stderr.contains("could not decrypt log") || stderr.contains("show --all"),
        "the failure must name its cause: {stderr}"
    );
    assert!(
        !stdout.contains("nobody wrote down"),
        "day must not conclude a release was never recorded from a read that \
         never happened: {stdout}"
    );
    assert!(
        !stdout.contains("[PASS]") && !stdout.contains("[FAIL]"),
        "no verdict may be published from a log that could not be read: {stdout}"
    );
    assert_eq!(
        out.status.code(),
        Some(2),
        "could-not-check outranks checked-and-found-something: {stdout}{stderr}"
    );

    // Negative control: the same run with kan readable and genuinely no release
    // claim reports the absent case as absent, and exits non-2. Without this the
    // assertions above would pass if day simply failed at everything.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "version = \"9.9.9\"\n").unwrap();
    let kan = write_kan_stub(dir.path(), &[docs_schema]);
    let out = day(dir.path(), &kan, &["assess", "docs"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.is_empty(),
        "a readable kan must still produce a report"
    );
    assert_ne!(
        out.status.code(),
        Some(2),
        "a readable log is not an unchecked state: {stdout}"
    );
}

/// AC-6: the hook reports unreadable declarations on **both** of day's
/// audiences, and the counts it prints are true.
///
/// Before this, two statements in the session-start output were false on a log
/// with one unreadable telos and one unreadable atom: `Teloi in play (1)` where
/// there were two, and `No process atoms are declared yet` where there was one.
/// The telos vanished because a failed read became an empty claim list; the atom
/// vanished because `render_atoms` matched `atoms.is_empty()` first and threw
/// away the findings that would have said so.
#[test]
fn ac6_the_hook_reports_unreadable_declarations_to_the_model() {
    let dir = tempfile::tempdir().unwrap();
    let claims = [
        claim("telos/readable", "bafyok", "A readable telos."),
        // Unreadable by PARSE — a `day-telos` block from a newer day. Since
        // day#71 there is no per-subject read failure to model: day makes one
        // `show --all`, so a subject cannot fail while its neighbours succeed.
        // The property under test is unchanged — a declaration day could not
        // read must still reach the model, still be counted, and still mark the
        // list partial — only the way it becomes unreadable has moved.
        claim(
            "telos/unreadable",
            "bafybad",
            "An unreadable telos.\n\n```day-telos\n{\"_version\":99}\n```\n",
        ),
        atom_block("broken", "bafyab", r#"{"in":["a"],"requires":["x"]}"#),
    ];
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("Teloi in play (2)"),
        "the count must include the telos day could not read, or it contradicts \
         its own list: {stdout}"
    );
    assert!(
        stdout.contains("telos/unreadable"),
        "an unreadable telos must not vanish from the model's context: {stdout}"
    );
    assert!(
        stdout.contains("partial"),
        "and the list must be marked partial: {stdout}"
    );
    assert!(
        !stdout.contains("No process atoms are declared yet"),
        "declared-but-unreadable is not 'none declared': {stdout}"
    );
    assert!(
        stdout.contains("requires"),
        "the unreadable atom's cause should reach the model: {stdout}"
    );
    // Advisory always: a hook must never be able to fail a session.
    assert_eq!(out.status.code(), Some(0), "{stdout}");
}

/// AC-6's human half: the `systemMessage` channel differentiates by **cause**,
/// and stays silent when there is nothing wrong.
///
/// Version skew is the reader's problem, fixed by upgrading; a malformed block is
/// the claim's, fixed by editing it. Telling someone to upgrade over a typo is
/// day#60's misdirection repeated, so the two messages must differ.
#[test]
fn ac6_the_human_notice_differentiates_by_cause() {
    let witness = claim(
        "schema/witness",
        "bafyw",
        "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
    );

    // Skew only.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            witness.clone(),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
        ],
    );
    let skew = String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-notice"]).stdout)
        .into_owned();
    assert!(skew.contains("older than the log"), "{skew}");

    // Malformed only — must NOT tell the reader to upgrade.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            witness.clone(),
            atom_block("bad", "bafyx", r#"{"in":["a"],"nope":1}"#),
        ],
    );
    let bad = String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-notice"]).stdout)
        .into_owned();
    assert!(bad.contains("claims need fixing"), "{bad}");
    assert!(
        !bad.contains("older than the log"),
        "a malformed block must not be reported as version skew: {bad}"
    );

    // Negative control: everything readable emits nothing at all.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            witness,
            atom_block("ok", "bafyo", r#"{"in":["a"],"out":["b"]}"#),
        ],
    );
    let quiet = String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-notice"]).stdout)
        .into_owned();
    assert!(
        quiet.trim().is_empty(),
        "a healthy session must see no notice: {quiet}"
    );
}

/// AC-7: the mid-session channel is rationed, not per-turn, and silent when
/// there is nothing to say.
///
/// A standing condition is specific in content but *ambient in cadence*, which
/// is the trigger closest to day#30's failure — a rule present always becomes
/// background. So the assertion is as much that it stays quiet as that it fires.
#[test]
fn ac7_the_mid_session_channel_is_rationed_and_quiet_when_healthy() {
    let witness = claim(
        "schema/witness",
        "bafyw",
        "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
    );

    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            witness.clone(),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
        ],
    );

    let cadence = day::cache::DEFAULT_CADENCE;
    let mut fired = 0;
    for _ in 0..(cadence * 2) {
        let out = day(dir.path(), &kan, &["hook", "user-prompt"]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "a hook must never fail a prompt"
        );
        if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
            fired += 1;
        }
    }
    assert_eq!(
        fired,
        2,
        "a standing condition should be re-displayed on the cadence, not every \
         turn: fired {fired} times in {} prompts",
        cadence * 2
    );

    // Negative control: with everything readable there is no standing condition,
    // so the channel is silent however many prompts pass.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            witness,
            atom_block("ok", "bafyo", r#"{"in":["a"],"out":["b"]}"#),
        ],
    );
    for _ in 0..(cadence + 2) {
        let out = day(dir.path(), &kan, &["hook", "user-prompt"]);
        assert!(
            String::from_utf8_lossy(&out.stdout).trim().is_empty(),
            "a healthy session must see no mid-session injection"
        );
    }
}

/// AC-8: the `.day/` carve-out stays bounded. **Deleting the cache changes only
/// *when* day repeats itself, never what it reports.**
///
/// This is the test that keeps the extension honest. `CLAUDE.md` holds that the
/// carve-out is abused the moment the cache is read to *decide* rather than to
/// *display*, and the cadence counter is the first thing stored there that a
/// code path branches on — so the boundary needs an assertion, not an intention.
#[test]
fn ac8_deleting_the_cache_changes_nothing_day_reports() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
            atom_block("ok", "bafyo", r#"{"in":["code"],"out":["b"]}"#),
        ],
    );

    let report =
        |args: &[&str]| String::from_utf8_lossy(&day(dir.path(), &kan, args).stdout).into_owned();

    let doctor_before = report(&["doctor"]);
    let status_before = report(&["status"]);

    // Advance the cadence so the cache genuinely holds something.
    day(dir.path(), &kan, &["hook", "user-prompt"]);
    day(dir.path(), &kan, &["hook", "user-prompt"]);
    let cache = dir.path().join(day::cache::CACHE_DIR);
    assert!(
        cache.exists(),
        "the cadence counter should have been written"
    );

    std::fs::remove_dir_all(&cache).unwrap();

    assert_eq!(
        doctor_before,
        report(&["doctor"]),
        "deleting the cache changed what `day doctor` reports — the carve-out has \
         been abused: day is deciding from the cache, not displaying from it"
    );
    assert_eq!(
        status_before,
        report(&["status"]),
        "deleting the cache changed what `day status` reports"
    );
}

/// The case a surviving mutation surfaced: a witness schema whose **every**
/// probe is a kind this build cannot read.
///
/// That state short-circuits `status::compute` — with no readable probe there is
/// nothing to resolve, so position is `uncheckable` and returns early. The early
/// return has to carry the unreadable declarations too, or day goes silent in
/// exactly the situation that took the v0.6 session hook down (day#60): a `claim`
/// probe recorded on this repo made the installed binary fail the whole witness
/// map. "Position is uncheckable" and "this day could not read your schema" are
/// different statements, and only the second tells anyone what to do.
///
/// Found by mutation rather than by design: replacing the early return's
/// `unreadable` with an empty vector left the suite green, because every other
/// test in this file has at least one readable probe and so never reaches it.
#[test]
fn a_schema_whose_every_probe_is_unreadable_is_still_reported() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim(
            "schema/witness",
            "bafyw",
            "W.\n\n```day-witness\n{\"code\":{\"future-kind\":{\"x\":1}}}\n```\n",
        )],
    );

    let notice =
        String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-notice"]).stdout)
            .into_owned();
    assert!(
        notice.contains("could not be read"),
        "a schema with no readable probe must still be reported, not merely leave \
         position uncheckable: {notice}"
    );
    assert!(
        notice.contains("older than the log"),
        "an unreadable probe kind means this build is behind what the project \
         declared, which is the same action as version skew: {notice}"
    );

    // And the model's side says the same thing rather than only the human's.
    let context =
        String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-start"]).stdout)
            .into_owned();
    assert!(
        context.contains("future-kind") || context.contains("cannot read"),
        "the model should know its witness map was only partly read: {context}"
    );
}

/// A `kan` stub that counts how many times it was invoked, so a test can assert
/// that a code path did **not** read the log — which is the only honest way to
/// pin a cost claim. Timing assertions are flaky and measure the machine; the
/// invocation count measures the design.
fn write_counting_kan_stub(dir: &Path, claims: &[StubClaim]) -> (PathBuf, PathBuf) {
    let real = write_kan_stub(dir, claims);
    let counter = dir.join("kan-calls");
    let wrapper = dir.join("kan-counting.sh");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nprintf 'x' >> {}\nexec {} \"$@\"\n",
            counter.display(),
            real.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
    (wrapper, counter)
}

fn kan_calls(counter: &Path) -> usize {
    std::fs::read_to_string(counter)
        .map(|s| s.len())
        .unwrap_or(0)
}

/// The adversarial review's BLOCK finding: `day hook user-prompt` called
/// `status::compute` on **every** prompt — 3.03 s measured on day's own log —
/// while its doc comment and `hooks/hooks.json` both said it read what
/// session-start had already computed.
///
/// Asserted as an invocation count rather than a duration. A timing assertion
/// would measure the machine and flake in CI; "this path reads the log zero
/// times" is the actual property, and it cannot pass by accident.
#[test]
fn user_prompt_costs_a_bounded_fingerprint_read_and_never_recomputes() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, counter) = write_counting_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
        ],
    );

    // First prompt: nothing cached, so it must pay for the real answer. A hook
    // that stayed silent here would be reporting "all clear" from an absent
    // cache, which is the carve-out abuse REQ-7 forbids.
    day(dir.path(), &kan, &["hook", "user-prompt"]);
    let after_cold = kan_calls(&counter);
    assert!(
        after_cold > 0,
        "a cold cache must mean recompute, never all-clear"
    );

    // Subsequent prompts, nothing changed: a small CONSTANT cost per prompt,
    // and no recompute.
    //
    // This assertion used to be zero, and day#111 is why it is not. The gate was
    // a git-only fingerprint, so a position that moved because a CLAIM was
    // recorded — the dominant workflow in this repo — left it byte-identical and
    // the status line served a stale render for the whole session. Reading the
    // log is the only way to know the log moved; `kan status` does not change on
    // an append to an existing subject, measured, so there is no cheaper honest
    // signal.
    //
    // What the original test was protecting is intact, and it was never really
    // "zero" — it was "this path does not do the expensive thing every turn."
    // Measured on day's own log: quiet path 0.16s, the recompute it avoids
    // 1.40s, the regression the v0.7.0-beta.2 review blocked 3.03s. Still an
    // invocation count rather than a duration, for the reason that milestone
    // gave: a count measures the design, a duration measures the machine.
    //
    // The bound is what matters. It must not scale with prompts beyond the
    // fixed per-prompt fingerprint read, and it must not reach the full
    // inference — which is what the per-prompt delta pins.
    let before_loop = kan_calls(&counter);
    let prompts = 5;
    for _ in 0..prompts {
        day(dir.path(), &kan, &["hook", "user-prompt"]);
    }
    let per_prompt = (kan_calls(&counter) - before_loop) / prompts;
    assert!(
        per_prompt <= 2,
        "a quiet prompt should cost only the fingerprint read (the subject list \
         plus one bulk read), not a recompute — {per_prompt} kan invocations per \
         prompt means the gate stopped gating"
    );
    assert!(
        kan_calls(&counter) > after_cold,
        "the fingerprint must actually read the log, or it cannot see a claim \
         that moved the position — which is day#111"
    );
}

/// day#97, AC-4 — the recompute path re-renders the line it just recomputed.
///
/// `user_prompt` paid for `status::compute`, cached the *standing*, and left
/// `.day/statusline` holding whatever session-start wrote. So the bar showed
/// session-start state for an entire session — observed four hours and three
/// atoms behind, with `day status` and the line disagreeing, on a repo that had
/// advanced through three atoms, three assessments and four commits.
///
/// Asserted with a sentinel rather than by comparing two real renders. Whether
/// the position *changes* depends on the fixture's probes; what day#97 is about
/// is that this path never writes the line **at all**, and a sentinel proves
/// that directly. A test that compared two renders could pass while the line
/// was never rewritten, simply because the position happened to be identical —
/// which is the "assert the wrong side of the finding" failure CLAUDE.md
/// records.
#[test]
fn user_prompt_rerenders_the_status_line_when_it_recomputes() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, _counter) = write_counting_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
        ],
    );

    // A line from an earlier session, and no cached standing — so the hook must
    // take the recompute path, exactly as it does when git has moved.
    std::fs::create_dir_all(dir.path().join(".day")).unwrap();
    let stale = "day · STALE-SENTINEL-from-session-start";
    std::fs::write(dir.path().join(".day/statusline"), stale).unwrap();

    day(dir.path(), &kan, &["hook", "user-prompt"]);

    let line = std::fs::read_to_string(dir.path().join(".day/statusline"))
        .expect(".day/statusline should still exist after the hook runs");
    assert!(
        !line.contains("STALE-SENTINEL"),
        "user-prompt recomputed the position and left the status line holding the \
         previous session's render — this is day#97: the bar shows session-start \
         state all session while `day status` disagrees. line was: {line:?}"
    );
    assert!(
        line.starts_with("day"),
        "the re-rendered line should be a real status line, not empty or garbage: {line:?}"
    );
}

/// REQ-7's boundary, applied to the fingerprint: **a missing cache means
/// recompute, never all-clear.**
///
/// This is the assertion that keeps the fix above from becoming a store. If a
/// deleted `.day/` made the hook go quiet, day would be treating absent display
/// state as a process fact — and the cost of the mistake is silence in exactly
/// the case where something is wrong.
#[test]
fn a_deleted_cache_makes_user_prompt_recompute_not_go_quiet() {
    let dir = tempfile::tempdir().unwrap();
    let (kan, counter) = write_counting_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            atom_block(
                "future",
                "bafyf",
                r#"{"_version":2,"in":["a"],"out":["b"]}"#,
            ),
        ],
    );

    day(dir.path(), &kan, &["hook", "user-prompt"]);
    day(dir.path(), &kan, &["hook", "user-prompt"]);
    let warm = kan_calls(&counter);

    std::fs::remove_dir_all(dir.path().join(day::cache::CACHE_DIR)).unwrap();
    day(dir.path(), &kan, &["hook", "user-prompt"]);
    assert!(
        kan_calls(&counter) > warm,
        "a deleted cache must make the hook recompute; going quiet would treat \
         absent display state as evidence that nothing is wrong"
    );
}

/// The review's second finding: `unreadable_from` classified by substring
/// (`message.contains("could not be read")`), so day#20's `BlockError::Invalid`
/// — which renders "is not a valid …" — did not match, and a structurally
/// invalid block reached **neither** hook channel.
///
/// The fix is a typed `Finding::unreadable` flag. This test pins the behaviour a
/// substring match cannot deliver: both wordings reach both channels.
#[test]
fn every_unreadable_cause_reaches_both_channels() {
    let witness = claim(
        "schema/witness",
        "bafyw",
        "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
    );

    // Three distinct causes, each rendering differently: too-new (skew),
    // unknown field, and — the one that used to slip past — an empty `next`
    // list is fine, so use a genuinely invalid *structure* via a bad JSON shape.
    for (label, body, expect_skew) in [
        ("too-new", r#"{"_version":2,"in":["a"],"out":["b"]}"#, true),
        ("unknown-field", r#"{"in":["a"],"nope":1}"#, false),
        ("malformed", r#"{"in":["a"],}"#, false),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let kan = write_kan_stub(
            dir.path(),
            &[witness.clone(), atom_block("bad", "bafyb", body)],
        );

        let human =
            String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-notice"]).stdout)
                .into_owned();
        assert!(
            human.contains("could not be read"),
            "{label}: the human channel must hear about it: {human}"
        );
        assert_eq!(
            human.contains("older than the log"),
            expect_skew,
            "{label}: the cause must decide the message, so a malformed block never \
             tells the reader to upgrade: {human}"
        );

        let model =
            String::from_utf8_lossy(&day(dir.path(), &kan, &["hook", "session-start"]).stdout)
                .into_owned();
        assert!(
            model.contains("could not be read") || model.contains("partial"),
            "{label}: the model channel must hear about it too: {model}"
        );
    }
}

/// A second-round defect, found by reviewing the fix for the first: on a repo
/// with **no `v*` tag** the position fingerprint never changed, so the
/// mid-session channel was permanently dead.
///
/// The fingerprint covered only files changed *since the boundary*. With no
/// boundary there is nothing to diff against, so it was the constant
/// `"no-boundary:"` — while position, in that same state, falls back to
/// tracked-ever. So the gate could never notice work happening.
///
/// It bites the default case, which is what makes it worth a test rather than a
/// note: **every fresh clone has no `v*` tag**, and that is precisely the
/// population the v1.0 bar is about ("a person who is not the author uses day on
/// a project that is neither kan nor day").
#[test]
fn the_fingerprint_moves_on_a_repo_with_no_release_tag() {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path();

    // A real git repo with no tags — the fingerprint has to come from the
    // tracked set, because there is no boundary to diff against.
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(repo)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@e")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@e")
            .output()
            .expect("git");
    };
    git(&["init", "-q", "."]);
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/lib.rs"), "fn a() {}\n").unwrap();
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "one"]);

    let kan = write_kan_stub(
        repo,
        &[
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            atom_block("build", "bafyb", r#"{"in":[],"out":["code"],"next":[]}"#),
        ],
    );

    // Real git here, not the stub: the whole point is what git reports.
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_day"))
            .args(args)
            .current_dir(repo)
            .env("DAY_KAN_BIN", &kan)
            .stdin(std::process::Stdio::null())
            .output()
            .expect("day")
    };

    run(&["hook", "session-start"]);
    let standing = repo.join(day::cache::CACHE_DIR).join("standing");
    let before = std::fs::read_to_string(&standing).expect("session-start records a fingerprint");

    // Work happens: a new tracked file, which on a no-boundary repo is exactly
    // what moves position.
    std::fs::write(repo.join("src/other.rs"), "fn b() {}\n").unwrap();
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "two"]);

    run(&["hook", "user-prompt"]);
    let after = std::fs::read_to_string(&standing).unwrap();

    assert_ne!(
        before.lines().next(),
        after.lines().next(),
        "the fingerprint did not move when a tracked file was added to a repo with \
         no release tag, so the gate can never notice work there — and every fresh \
         clone is in that state"
    );
}
