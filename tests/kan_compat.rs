//! `.design/kan-compat.md` AC-6 — the range day *reports* must be the range CI
//! *measured*.
//!
//! `src/compat.rs` prints a supported range to every user running `day doctor`.
//! That range is only worth anything if it is derived from
//! `tests/fixtures/kan-compat.tsv`, which is where the matrix records what
//! actually happened. Nothing stops someone widening the constant by hand, and
//! a range nobody measured is worse than no range — it is a specific false
//! claim about another program.
//!
//! Hermetic and always-on, deliberately. The first version of this check was a
//! CI job that grepped the constants out of `src/compat.rs`; that check would
//! have passed silently the first time rustfmt reflowed a struct literal onto
//! one line — the same class of defect as a mutation harness grepping for
//! `FAILED`. Reading the constants through the crate cannot drift from the
//! constants.

use day::compat::{Version, NEWEST_MEASURED, OLDEST_SUPPORTED};

/// The `ok` rows of the committed table, oldest first.
fn measured_ok() -> Vec<Version> {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kan-compat.tsv");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should be readable: {e}", path.display()));

    let mut versions: Vec<Version> = text
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let tag = fields.next()?;
            let outcome = fields.next()?.trim();
            (outcome == "ok").then(|| {
                Version::parse(tag).unwrap_or_else(|| panic!("unparseable tag in table: {tag:?}"))
            })
        })
        .collect();
    versions.sort_by_key(|v| (v.major, v.minor, v.patch));
    versions
}

#[test]
fn the_table_records_at_least_one_working_pairing() {
    // A table with no `ok` row would make every assertion below vacuous, and
    // would also mean day works with no released kan at all.
    assert!(
        !measured_ok().is_empty(),
        "tests/fixtures/kan-compat.tsv has no `ok` row — day would be claiming \
         a supported range it never measured"
    );
}

#[test]
fn the_declared_range_matches_what_was_measured() {
    let ok = measured_ok();
    let oldest = ok.first().expect("at least one ok row");
    let newest = ok.last().expect("at least one ok row");

    // Compared on release order: the table's tags are pre-releases
    // (`v0.7.1-beta.1`) and the constants are plain releases, which is the
    // intended relationship — a `-beta.N` of a supported version is supported.
    assert_eq!(
        (oldest.major, oldest.minor, oldest.patch),
        (
            OLDEST_SUPPORTED.major,
            OLDEST_SUPPORTED.minor,
            OLDEST_SUPPORTED.patch
        ),
        "OLDEST_SUPPORTED is {OLDEST_SUPPORTED} but the oldest measured-ok kan \
         is {oldest}. day would print users a range it never measured. Move the \
         constant and the table together."
    );
    assert_eq!(
        (newest.major, newest.minor, newest.patch),
        (
            NEWEST_MEASURED.major,
            NEWEST_MEASURED.minor,
            NEWEST_MEASURED.patch
        ),
        "NEWEST_MEASURED is {NEWEST_MEASURED} but the newest measured-ok kan is \
         {newest}."
    );
}

/// Every row must carry an outcome the matrix can actually produce. A typo'd
/// outcome silently drops out of the `ok` set and would quietly narrow the
/// range day reports.
///
/// The cell emits a fourth token, `could-not-run`, and it is deliberately **not**
/// accepted here: it names a measurement that did not happen, which is the one
/// thing this table must never hold. A row is a claim that day was run against
/// that kan, and the outcome vocabulary is where that stays true.
#[test]
fn every_row_records_a_known_outcome() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kan-compat.tsv");
    let text = std::fs::read_to_string(path).unwrap();

    let mut rows = 0;
    for line in text
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
    {
        let mut fields = line.split('\t');
        let tag = fields.next().expect("a tag");
        let outcome = fields.next().unwrap_or("").trim();
        assert!(
            matches!(outcome, "ok" | "incompatible" | "unbuildable"),
            "{tag}: {outcome:?} is not an outcome scripts/run-kan-compat-cell.sh emits"
        );
        assert!(
            Version::parse(tag).is_some(),
            "{tag:?} is not a parseable version tag"
        );
        rows += 1;
    }
    assert!(rows > 0, "the table should have rows");
}

/// The reason the floor is where it is, pinned so it cannot move silently.
///
/// Since day#71 the floor is **kan 0.9.1**, where `show --all --json` landed
/// (kan#123 / ADR-71). `ClaimLog` makes exactly that call, so a kan without it
/// cannot answer a single claim probe — day does not fall back, deliberately:
/// a fallback is a two-mode mechanism, and this repo's record says that is
/// where defects hide (day#91, and twice in `v0.7.0-beta.3` alone).
///
/// The previous floor was 0.7.0, where `show --json` first appeared. That
/// reason still holds for everything below it; 0.9.0 is simply a higher bar
/// day now genuinely needs.
#[test]
fn the_floor_is_where_the_bulk_read_landed() {
    let ok = measured_ok();
    let oldest = ok.first().unwrap();
    assert_eq!(
        (oldest.major, oldest.minor, oldest.patch),
        (0, 9, 1),
        "day's oldest supported kan is expected to be 0.9.1, the first with \
         `show --all --json`. If the floor moved, update this test and the \
         `why` column — the floor is a fact about a specific kan change, not a \
         number."
    );
}

// ---------------------------------------------------------------------------
// AC-5 and AC-7: what a user actually sees, end to end through `day doctor`.
//
// The unit tests in `src/compat.rs` cover `classify`. These exist because
// classification is not the deliverable — the rendered line is, and a verdict
// computed correctly and rendered into a report nobody can act on is the
// defect this repo keeps finding.
// ---------------------------------------------------------------------------

#[cfg(unix)]
mod rendered {
    use std::path::Path;
    use std::process::Command;

    /// A kan stub whose `--version` we control. `tests/common`'s stub answers
    /// the read verbs and not this one, and the whole point here is the
    /// version string.
    fn stub_kan(dir: &Path, version_line: &str) -> std::path::PathBuf {
        let path = dir.join("kan-stub.sh");
        std::fs::write(
            &path,
            format!(
                "#!/bin/sh\n\
                 case \"$1\" in\n\
                 --help) echo kan; exit 0 ;;\n\
                 --version) {version_line} ;;\n\
                 status|issues) echo '{{\"v\":1,\"subjects\":[]}}'; exit 0 ;;\n\
                 show) echo '{{\"v\":1,\"subject\":\"x\",\"subjects\":[],\"claims\":[],\"inbound\":[]}}'; exit 0 ;;\n\
                 *) exit 0 ;;\n\
                 esac\n"
            ),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        path
    }

    fn doctor_against(version_line: &str) -> (String, bool) {
        let dir = tempfile::tempdir().unwrap();
        let kan = stub_kan(dir.path(), version_line);
        let out = Command::new(env!("CARGO_BIN_EXE_day"))
            .args(["doctor"])
            .current_dir(dir.path())
            .env("DAY_KAN_BIN", &kan)
            .output()
            .expect("day should run");
        (
            String::from_utf8_lossy(&out.stdout).into_owned(),
            out.status.success(),
        )
    }

    /// AC-5. The two skew directions must read differently, because they call
    /// for opposite responses — collapsing them into one "mismatch" makes the
    /// benign case as loud as the real one, and a warning that fires on every
    /// kan release is a warning nobody reads.
    #[test]
    fn the_two_skew_directions_read_differently() {
        let (old, _) = doctor_against("echo 'kan 0.6.0-beta.1'; exit 0");
        assert!(
            old.contains("OLDER than this day supports") && old.contains("Upgrade kan"),
            "an old kan should name the fix: {old}"
        );

        let (new, _) = doctor_against("echo 'kan 1.2.0'; exit 0");
        assert!(
            new.contains("newer than this day was measured against")
                && new.contains("Normally fine"),
            "a newer kan should be noted without alarm: {new}"
        );
        assert!(
            !new.contains("Upgrade kan"),
            "a newer kan is not something the user fixes by upgrading kan: {new}"
        );
    }

    /// AC-4's rendered half. A version day cannot read is *unknown*, never a
    /// mismatch — claiming incompatibility from a failed read would break day
    /// against any kan whose `--version` output shifts, which is a
    /// self-inflicted outage rather than a warning.
    #[test]
    fn an_unreadable_version_says_so_rather_than_guessing() {
        for line in ["echo kan; exit 0", "exit 1"] {
            let (out, _) = doctor_against(line);
            assert!(
                out.contains("version unknown") && out.contains("cannot tell"),
                "an unreadable version should report unknown, not a mismatch: {out}"
            );
            assert!(
                !out.contains("OLDER than"),
                "an unreadable version must never render as too old: {out}"
            );
        }
    }

    /// AC-5's negative control. A supported pairing prints the range and
    /// nothing else — if this failed, every assertion above would be measuring
    /// a warning that always fires.
    #[test]
    fn a_supported_pairing_prints_no_warning() {
        let (out, _) = doctor_against("echo 'kan 0.9.1-beta.1'; exit 0");
        assert!(
            out.contains("kan: 0.9.1-beta.1 (supported:"),
            "a supported pairing should state the range: {out}"
        );
        for alarm in ["OLDER", "newer than", "unknown"] {
            assert!(
                !out.contains(alarm),
                "a supported pairing should not mention {alarm:?}: {out}"
            );
        }
    }

    /// AC-7. Advisory, never blocking — the non-negotiable, checked on the one
    /// surface where a version check would be most tempting to enforce from.
    #[test]
    fn a_mismatch_never_changes_the_exit_code() {
        for line in [
            "echo 'kan 0.1.0'; exit 0",
            "echo 'kan 99.0.0'; exit 0",
            "exit 1",
        ] {
            let (out, ok) = doctor_against(line);
            assert!(
                ok,
                "day doctor must exit zero regardless of the kan pairing — a \
                 version check is advisory, not a gate: {out}"
            );
            // And it must still do the work, not bail early on the version.
            assert!(
                out.contains("atoms:") && out.contains("composition:"),
                "a mismatch must not short-circuit the reads: {out}"
            );
        }
    }
}
