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
    // behaves — clap rejects the unknown flag with status 2.
    let old = dir.path().join("kan-old.sh");
    std::fs::write(
        &old,
        format!(
            "#!/bin/sh\nif [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
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
