//! `.design/bulk-read.md` — day#71. One read of the log, not one per subject.
//!
//! These assert **invocation counts**, never durations. `v0.7.0-beta.2`
//! established the rule the hard way: a timing assertion measures the machine
//! and flakes, while an invocation count measures the design. The whole point
//! of this change is a count — kan#123 showed the cost is fixed per-process
//! startup, so nothing inside a read matters and only the number of reads does.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{atom_claim, claim, write_kan_stub, StubClaim};

/// Wraps the shared stub in a script that logs every invocation, so a test can
/// count what day actually asked kan for. The wrapper delegates rather than
/// reimplementing: a second stub would drift from the first, and then these
/// tests would be measuring the wrapper.
fn counting_stub(dir: &Path, claims: &[StubClaim]) -> (std::path::PathBuf, std::path::PathBuf) {
    let inner = write_kan_stub(dir, claims);
    let log = dir.join("kan-calls.log");
    let wrapper = dir.join("kan-counting.sh");
    std::fs::write(
        &wrapper,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> {log}\nexec {inner} \"$@\"\n",
            log = log.display(),
            inner = inner.display(),
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
    (wrapper, log)
}

fn calls(log: &Path) -> Vec<String> {
    std::fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("day should run")
}

/// A witness schema declaring `n` claim probes, all of which resolve against
/// the log. The count is the variable under test.
fn witness_schema(n: usize) -> StubClaim {
    let probes: Vec<String> = (0..n)
        .map(|i| format!("\"kind{i}\":{{\"claim\":{{\"kind\":\"Observation\"}}}}"))
        .collect();
    claim(
        "schema/witness",
        "bafyw",
        &format!("W.\n\n```day-witness\n{{{}}}\n```\n", probes.join(",")),
    )
}

/// A log with `n` atoms whose outputs are exactly the `n` declared claim-probe
/// types, so position inference actually resolves every probe.
///
/// The first version of this fixture declared probes nothing referenced. day
/// resolved none of them, `ClaimLog` never loaded — it is lazy by design — and
/// the test asserted "one read" against zero reads. A fixture that never
/// reaches the mechanism measures nothing.
fn base_log(probes: usize) -> Vec<StubClaim> {
    let mut v = vec![
        // A telos whose witness IS a claim probe, so `assess telos` reads the
        // log rather than reporting that it declares no witnesses.
        claim(
            "telos/t",
            "bafyt",
            "A telos.\n\n```day-telos\n{\"witnesses\":[\"kind0\"]}\n```\n",
        ),
    ];
    for i in 0..probes {
        v.push(atom_claim(
            &format!("atom/a{i}"),
            &format!("bafya{i}"),
            &[],
            &[&format!("kind{i}")],
            &[],
        ));
    }
    for i in 0..6 {
        v.push(claim(
            &format!("subject/{i}"),
            &format!("bafys{i}"),
            "some claim",
        ));
    }
    v
}

/// AC-1, and the claim REQ-1 actually makes: **one read, shared by every claim
/// probe**, not one read per probe.
///
/// The probe count is what varies — 1 vs 5 — because that is the axis a
/// per-probe read would move along. `ClaimLog` is lazy and memoized, so a
/// regression that dropped the sharing would show up here as 5 calls while
/// every other test stayed green.
#[test]
fn the_log_is_read_once_however_many_claim_probes_are_declared() {
    let mut seen = Vec::new();
    for probes in [1usize, 5] {
        let dir = tempfile::tempdir().unwrap();
        let mut claims = base_log(probes);
        claims.push(witness_schema(probes));
        let (kan, log) = counting_stub(dir.path(), &claims);

        let out = day(dir.path(), &kan, &["status"]);
        assert!(
            out.status.success(),
            "day status should succeed: {}",
            String::from_utf8_lossy(&out.stderr)
        );

        let all = calls(&log)
            .iter()
            .filter(|c| c.starts_with("show --all"))
            .count();
        assert_eq!(
            all, 1,
            "with {probes} claim probe(s), the log should be read exactly once, \
             not once per probe"
        );
        seen.push(all);
    }
    assert_eq!(seen, vec![1, 1], "the count must not vary with probe count");
}

/// AC-6. The log is read **at most once**, and not at all when nothing asks.
///
/// The first version of this test asserted `day doctor` never reads the whole
/// log, on the reasoning that it only needs `atom/*`. That reasoning assumed a
/// whole-log read costs N calls; it costs one, and `doctor` was spending eight
/// (`status` + 7 × `show atom/*`) on a question one call answers. Reading
/// everything made it faster. What is worth protecting is laziness, so that is
/// what this asserts.
#[test]
fn the_log_is_read_at_most_once_and_only_when_something_asks() {
    // Nothing asks: `init --print` reports the wiring steps and reads no claims.
    let dir = tempfile::tempdir().unwrap();
    let claims = base_log(1);
    let (kan, log) = counting_stub(dir.path(), &claims);
    day(dir.path(), &kan, &["init", "--print"]);
    assert_eq!(
        calls(&log).iter().filter(|c| c.starts_with("show")).count(),
        0,
        "a command that asks for no claim must not read the log: {:?}",
        calls(&log)
    );

    // Something asks: `doctor` needs the atom vocabulary. One bulk read, and
    // crucially ZERO per-subject reads — the seven `show atom/*` calls it used
    // to make are what this change removes.
    let dir = tempfile::tempdir().unwrap();
    let (kan, log) = counting_stub(dir.path(), &base_log(1));
    day(dir.path(), &kan, &["doctor"]);
    let c = calls(&log);
    assert_eq!(
        c.iter().filter(|x| x.starts_with("show --all")).count(),
        1,
        "the log should be read exactly once: {c:?}"
    );
    assert_eq!(
        c.iter()
            .filter(|x| x.starts_with("show ") && !x.starts_with("show --all"))
            .count(),
        0,
        "no per-subject read should survive: {c:?}"
    );
}

/// AC-3. A kan that cannot serve the bulk read is an **error naming the cause**,
/// never a silently empty log.
///
/// This is the failure mode `src/probe.rs` exists to forbid and
/// `a_failed_kan_read_is_never_swallowed` enforces: reporting "no matching
/// claim" because the read failed is a false negative dressed as evidence. Here
/// it would be worse than usual — every claim probe in the command is answered
/// from this one read, so one swallowed error would silently empty all of them.
#[test]
fn a_kan_without_the_bulk_read_errors_rather_than_reading_an_empty_log() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = base_log(1);
    claims.push(witness_schema(1));
    let (kan, _log) = counting_stub(dir.path(), &claims);

    // An old kan: everything works except `show --all`, exactly as kan < 0.9.1
    // behaves — clap rejects the unknown flag with status 2. It reports its
    // version, because a real one does and because day now classifies "too old"
    // by ASKING rather than by matching clap's wording.
    let old = dir.path().join("kan-old.sh");
    std::fs::write(
        &old,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = --version ]; then echo 'kan 0.8.0-beta.1'; exit 0; fi\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
             printf 'error: unexpected argument '\\''--all'\\'' found\\n' >&2\n exit 2\nfi\n\
             exec {} \"$@\"\n",
            kan.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&old, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day(dir.path(), &old, &["assess", "telos", "t"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("requires kan >=") && text.contains("show --all"),
        "a kan that cannot serve the bulk read must say so and name the version \
         requirement, not surface clap's `unexpected argument`: {text}"
    );
    assert!(
        !text.contains("[MISSING]"),
        "an unreadable log must never render as evidence of absence: {text}"
    );

    // Negative control: the same fixture against a kan that DOES serve it.
    let out = day(dir.path(), &kan, &["assess", "telos", "t"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        !text.contains("requires kan >="),
        "the control must not report a version failure: {text}"
    );
}

/// The capability the whole-log read cost day, recovered — and widened.
///
/// Reading one subject at a time, a subject kan could not serve produced an
/// error naming it. One bulk read cannot: a subject missing from the payload
/// looks exactly like a subject with nothing in it, and day would report an
/// absence it never verified. That is the failure `telos/honest-reads` exists
/// to forbid, and it was introduced not by the bulk read itself but by serving
/// `show()` from the memo — until then `show()` was still a real per-subject
/// read that could still fail.
///
/// The cross-check is *wider* than what it replaces: the per-subject loop could
/// only catch a failure kan **reported**, while comparing `status --json`
/// against the bulk payload also catches one it silently **omitted**.
#[test]
fn a_subject_kan_lists_but_does_not_return_is_reported_not_treated_as_absent() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = base_log(1);
    claims.push(witness_schema(1));
    claims.push(claim(
        "telos/vanishing",
        "bafyvan",
        "Present in status only.",
    ));
    let (inner, _log) = counting_stub(dir.path(), &claims);

    // A kan that lists `telos/vanishing` in `status` but drops it from the
    // bulk payload — the silent-omission case day could not previously see.
    let dropping = dir.path().join("kan-dropping.sh");
    std::fs::write(
        &dropping,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
               {inner} \"$@\" | python3 -c 'import json,sys; d=json.load(sys.stdin); \
             d[\"subjects\"]=[e for e in d[\"subjects\"] if e[\"subject\"]!=\"telos/vanishing\"]; \
             print(json.dumps(d))'\n\
               exit 0\n\
             fi\n\
             exec {inner} \"$@\"\n",
            inner = inner.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&dropping, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day(dir.path(), &dropping, &["hook", "session-notice"]);
    let notice = String::from_utf8_lossy(&out.stdout);
    assert!(
        notice.contains("partial"),
        "a subject kan listed but did not return must mark the report partial: {notice}"
    );

    let out = day(dir.path(), &dropping, &["hook", "session-start"]);
    let started = String::from_utf8_lossy(&out.stdout);
    assert!(
        started.contains("telos/vanishing"),
        "the unaccounted subject must be named, or a reader cannot act on it: {started}"
    );
    assert!(
        started.contains("partial"),
        "and the context must be marked partial: {started}"
    );

    // Negative control: the same fixture through the honest stub is silent.
    // Without this, the assertions above would pass against a check that fires
    // on every run — which is how a warning becomes background noise.
    let out = day(dir.path(), &inner, &["hook", "session-notice"]);
    let notice = String::from_utf8_lossy(&out.stdout);
    assert!(
        notice.trim().is_empty(),
        "a complete bulk read must produce no notice: {notice}"
    );
    let out = day(dir.path(), &inner, &["hook", "session-start"]);
    let started = String::from_utf8_lossy(&out.stdout);
    assert!(
        !started.contains("did not return it in the bulk read"),
        "a complete bulk read must not report an unaccounted subject: {started}"
    );
}

/// The review's F1, which was a BLOCK: `day assess telos` reported
/// `[MISSING]` for evidence the bulk read had dropped.
///
/// The cross-check existed and was wired only into `status::compute`, so the
/// hook channels were protected and the **assess verbs — where day publishes
/// evidentiary verdicts — were not**. That is the same shape as the beta.3
/// BLOCK: a check that exists and is not called from the path that matters.
///
/// The fix moved the check to the read rather than adding another call site,
/// so this asserts the property on the verb that was lying, not on the one that
/// happened to be fixed.
#[test]
fn an_incomplete_log_cannot_answer_a_claim_probe() {
    let dir = tempfile::tempdir().unwrap();
    let claims = vec![
        claim(
            "schema/witness",
            "bafyw",
            "W.\n\n```day-witness\n{\"evidence\":{\"claim\":{\"kind\":\"Observation\",\
             \"subject\":\"proof/*\"}}}\n```\n",
        ),
        claim(
            "telos/t",
            "bafyt",
            "T.\n\n```day-telos\n{\"witnesses\":[\"evidence\"]}\n```\n",
        ),
        claim("proof/one", "bafyp", "The evidence exists."),
    ];
    let (honest, _) = counting_stub(dir.path(), &claims);

    // A kan that lists `proof/one` and omits it from the bulk payload.
    let dropping = dir.path().join("kan-dropping.sh");
    std::fs::write(
        &dropping,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
               {inner} \"$@\" | python3 -c 'import json,sys; d=json.load(sys.stdin); \
             d[\"subjects\"]=[e for e in d[\"subjects\"] if e[\"subject\"]!=\"proof/one\"]; \
             print(json.dumps(d))'\n\
               exit 0\n\
             fi\n\
             exec {inner} \"$@\"\n",
            inner = honest.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&dropping, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day(dir.path(), &dropping, &["assess", "telos", "t"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        !text.contains("[MISSING]"),
        "day must not report evidence absent when it never received the subject \
         carrying it: {text}"
    );
    assert!(
        text.contains("[ERROR]") && text.contains("proof/one"),
        "it must say it could not answer, and name the subject: {text}"
    );

    // Negative control: the same telos through the honest stub resolves. If
    // this failed, the assertions above would pass against a probe that can
    // never answer anything.
    let out = day(dir.path(), &honest, &["assess", "telos", "t"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("[MATERIAL]"),
        "a complete log must still resolve the witness: {text}"
    );
}

/// The other half of F1: `show()` is used directly by readers that never touch
/// `ClaimLog` (`docs.rs`, `record.rs`, `practice.rs`). A subject kan listed and
/// did not return must be an error naming it there too — which is exactly what
/// reading one subject at a time used to give for free.
///
/// **Driven through `assess docs`, whose verdict depends on reading `release`
/// via `client.show`.** The first version of this test used `session-start` and
/// was vacuous: it passed on `status.rs` reporting the unaccounted subject
/// whether or not `show()` refused, so deleting the guard SURVIVED mutation.
/// This one fails when the guard goes, because the verdict itself changes.
#[test]
fn a_direct_show_of_an_unaccounted_subject_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "version = \"9.9.9\"\n").unwrap();
    // `assess docs` reads git for the release tag, so it needs a real repo.
    for args in [
        vec!["init", "-q", "."],
        vec!["add", "-A"],
        vec!["commit", "-q", "-m", "one"],
        vec!["tag", "v9.9.9"],
    ] {
        let out = Command::new("git")
            .args(&args)
            .current_dir(dir.path())
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@e")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@e")
            .output()
            .expect("git");
        // Asserted, because the status was dropped and a setup that could not
        // run then arrived as a verdict about day: `git tag` failed under a
        // global `tag.gpgsign`, and this test reported "a complete log must
        // still assess the release" — a finding about the wrong program. A
        // fixture that cannot build itself is a could-not-check, and it says so
        // here rather than downstream.
        assert!(
            out.status.success(),
            "git {args:?} failed, so this fixture never existed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let claims = vec![
        claim(
            "schema/docs",
            "bafyds",
            "D.\n\n```day-docs\n{\"version_source\":\"Cargo.toml\",\
             \"version_key\":\"version\"}\n```\n",
        ),
        claim("release", "bafyr", "v9.9.9 published."),
    ];
    let (honest, _) = counting_stub(dir.path(), &claims);

    let dropping = dir.path().join("kan-dropping.sh");
    std::fs::write(
        &dropping,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
               {inner} \"$@\" | python3 -c 'import json,sys; d=json.load(sys.stdin); \
             d[\"subjects\"]=[e for e in d[\"subjects\"] if e[\"subject\"]!=\"release\"]; \
             print(json.dumps(d))'\n\
               exit 0\n\
             fi\n\
             exec {inner} \"$@\"\n",
            inner = honest.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&dropping, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day(dir.path(), &dropping, &["assess", "docs"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !text.contains("nobody wrote down"),
        "day must not conclude a release was never recorded from a subject it \
         never received: {text}"
    );
    assert!(
        text.contains("release"),
        "the unaccounted subject must be named: {text}"
    );
    assert_ne!(
        out.status.code(),
        Some(0),
        "an unverified read must not report success: {text}"
    );

    // Negative control: through the honest stub the release IS found, so the
    // assertions above are about the dropped subject and not about the verb.
    let out = day(dir.path(), &honest, &["assess", "docs"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("v9.9.9"),
        "a complete log must still assess the release: {text}"
    );
}

/// The cross-check must fire only on evidence that went **missing**, never on
/// evidence that **arrived**.
///
/// kan's log is shared: another agent may append while day is reading. If day
/// took the subject list *after* the bulk read, a subject created in between
/// would look listed-but-not-returned, and day would call a healthy kan
/// incomplete and refuse to answer. Taken first, that subject shows up as a
/// surplus in the bulk read, which is harmless.
///
/// Deterministic rather than timing-based: the stub reveals the extra subject
/// in `status` **only once `show --all` has been called**, which is exactly the
/// interleaving the wrong order produces. With the reads in the right order the
/// list is taken before that ever happens.
#[test]
fn a_subject_that_arrives_mid_read_is_not_mistaken_for_a_missing_one() {
    let dir = tempfile::tempdir().unwrap();
    let claims = vec![
        claim(
            "schema/witness",
            "bafyw",
            "W.\n\n```day-witness\n{\"evidence\":{\"claim\":{\"kind\":\"Observation\",\
             \"subject\":\"proof/*\"}}}\n```\n",
        ),
        claim(
            "telos/t",
            "bafyt",
            "T.\n\n```day-telos\n{\"witnesses\":[\"evidence\"]}\n```\n",
        ),
        claim("proof/one", "bafyp", "The evidence exists."),
    ];
    let (honest, _) = counting_stub(dir.path(), &claims);

    let racing = dir.path().join("kan-racing.sh");
    let marker = dir.path().join("all-was-read");
    std::fs::write(
        &racing,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then touch {marker}; fi\n\
             if [ \"$1\" = status ] && [ -f {marker} ]; then\n\
               {inner} \"$@\" | python3 -c 'import json,sys; d=json.load(sys.stdin); \
             d[\"subjects\"].append({{\"subject\":\"telos/appeared\",\"subjects\":[\"telos/appeared\"],\
             \"state\":\"Unclassified\"}}); print(json.dumps(d))'\n\
               exit 0\n\
             fi\n\
             exec {inner} \"$@\"\n",
            marker = marker.display(),
            inner = honest.display()
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&racing, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day(dir.path(), &racing, &["assess", "telos", "t"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains("[MATERIAL]"),
        "a subject that appeared mid-read must not make day refuse a witness it \
         can answer: {text}"
    );
    assert!(
        !text.contains("telos/appeared"),
        "and it must not be reported as unaccounted for: {text}"
    );
}
