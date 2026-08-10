//! **day#50 — the install instructions, and the two ways they rot.**
//!
//! The headline defect is fixed and verified by running it: `README.md` pins
//! both crates, and `cargo install kan --version 0.9.1-beta.1` followed by
//! `cargo install day --version 0.11.0-beta.2` into a scratch `--root` produces
//! a pair where `day doctor` reports `composition: ok`. That is the v1.0 bar's
//! "install and onboarding that work without the author present", and it works.
//!
//! **The pins are the problem now.** Both are hand-written version strings in
//! prose, and each has a different thing it must agree with:
//!
//! - the **day** pin must be the current release — [`crate::compat`] does not
//!   care, but a reader installing four releases back does. `day assess docs`
//!   covers this one, because `schema/docs` lists `README.md` under
//!   `version_files`.
//! - the **kan** pin must be a version day actually supports. Nothing covered
//!   this. `assess docs` checks day's own version string and has no opinion
//!   about kan's, so the day day's floor moves, the README goes on telling
//!   strangers to install a kan day will refuse — and every check stays green.
//!
//! That asymmetry is the reason this file exists rather than a wider
//! `assess docs`. The version a doc *carries* and the versions a doc *tells you
//! to install* are different properties, and only one of them was being asked.
//!
//! **And the narrative rots differently again.** `## Status` led with
//! `v0.7.0-beta.2` while the crate was at `0.11.0-beta.2` — four releases —
//! and `assess docs` passed the whole time, correctly: it asks whether the file
//! carries the current version *somewhere*, and the install pin satisfied that.
//! A file can be version-consistent by that measure and still present a
//! four-release-old summary as what day is.

mod common;

use common::repo_root;

fn readme() -> String {
    std::fs::read_to_string(repo_root().join("README.md")).expect("README.md should be readable")
}

fn crate_version() -> String {
    let toml = std::fs::read_to_string(repo_root().join("Cargo.toml")).expect("Cargo.toml");
    toml.lines()
        .find_map(|l| l.strip_prefix("version = \""))
        .and_then(|rest| rest.split('"').next())
        .expect("Cargo.toml should declare a version")
        .to_string()
}

/// The version argument of the one documented `cargo install <crate>` line.
///
/// Derived from the README rather than passed in, because a test that is told
/// which version to expect asserts the tester's memory, not the document.
fn documented_install_version(krate: &str) -> String {
    let needle = format!("cargo install {krate} --version ");
    let text = readme();
    let mut found: Vec<String> = text
        .lines()
        .filter_map(|l| l.trim().strip_prefix(&needle).map(str::to_string))
        .map(|rest| {
            rest.split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .collect();
    found.dedup();
    assert_eq!(
        found.len(),
        1,
        "README.md should document exactly one pinned `cargo install {krate}`, \
         found {found:?} — two pins disagree silently, and a reader takes \
         whichever they scroll to first"
    );
    found.remove(0)
}

/// **The kan pin must name a kan day is measured to work with.**
///
/// `tests/fixtures/kan-compat.tsv` is the measurement — one row per released
/// kan, produced by `scripts/run-kan-compat-cell.sh` driving the real binary —
/// and `src/compat.rs`'s constants are set from it. This ties the third place
/// the floor appears, the one a stranger actually types, to the same source.
///
/// Asserted against the **`ok` rows**, not against `OLDEST_SUPPORTED` alone. A
/// version at or above the floor is not necessarily one that was measured: the
/// floor is where support *starts*, and `incompatible` rows can in principle sit
/// above it. What the README should name is a pairing someone ran.
///
/// And specifically the **newest** such pairing. Membership alone would be
/// satisfied forever by the floor — the README sat at `0.9.1-beta.1` while kan
/// shipped through `0.11.0-beta.1`, all measured `ok`, and a stranger following
/// it installed a kan two releases behind for no reason. Requiring the newest
/// means adding a row obliges updating the line, which is the point: the edit
/// becomes a build failure rather than something to remember.
#[test]
fn the_documented_kan_version_is_one_day_was_measured_against() {
    let pinned = documented_install_version("kan");
    let tsv = std::fs::read_to_string(repo_root().join("tests/fixtures/kan-compat.tsv"))
        .expect("kan-compat.tsv should ship");

    let measured_ok: Vec<&str> = tsv
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let mut cols = l.split('\t');
            let version = cols.next()?;
            (cols.next()? == "ok").then_some(version)
        })
        .collect();

    // The table having no `ok` row would make the assertion below vacuous —
    // `tests/kan_compat.rs` guards that too, and it is cheap to refuse here
    // rather than pass for the reason a missing table would produce.
    assert!(
        !measured_ok.is_empty(),
        "kan-compat.tsv records no working pairing, so this check cannot mean \
         anything"
    );
    assert!(
        measured_ok.contains(&format!("v{pinned}").as_str()),
        "README.md tells a reader to `cargo install kan --version {pinned}`, \
         which is not among the kan versions measured `ok` for this day: {measured_ok:?}. \
         Either the pin is stale or the row is missing; both send a stranger to \
         a kan day will refuse, and `day assess docs` has no opinion about kan's \
         version so nothing else here would notice."
    );

    // The rows are in release order, which `tests/kan_compat.rs` relies on for
    // the same reason — `measured_ok().last()` is what sets `NEWEST_MEASURED`.
    let newest = measured_ok.last().expect("checked non-empty above");
    assert_eq!(
        &format!("v{pinned}"),
        newest,
        "README.md pins kan {pinned}, but {newest} is measured `ok` and is newer. \
         A pin that is merely *supported* stays at the floor forever: this line \
         said 0.9.1-beta.1 while 0.9.2, 0.10.0 and 0.11.0 had all been measured."
    );
}

/// **The day pin must be this crate's version.**
///
/// `day assess docs` also covers this, and the duplication is deliberate: that
/// check runs against a real log and is advisory, this one fails the build. The
/// property is the same and the guarantee is not — day's own rule is that a
/// mechanism people can route around is one that eventually gets routed around.
#[test]
fn the_documented_day_version_is_the_current_release() {
    assert_eq!(
        documented_install_version("day"),
        crate_version(),
        "README.md pins a `cargo install day --version` that is not this crate's \
         version, so the documented install produces a day older than the docs \
         beside it"
    );
}

/// **`## Status` must lead with the current version.**
///
/// It led with `v0.7.0-beta.2` across four releases. Nothing caught it, and
/// `assess docs` was right not to: it asks whether `README.md` carries the
/// current version anywhere, and the install pin already satisfied that. So the
/// one line a stranger reads to learn what day *is* was four releases stale
/// while every version check passed.
///
/// Only the **first** version mentioned in the section is constrained. The rest
/// of `## Status` is deliberately historical — `**v0.7.0-beta.1** remains the
/// foundation` is a true sentence that should not have to change every release,
/// and a check demanding every mention be current would force deleting the
/// history to stay green.
#[test]
fn the_status_section_leads_with_the_current_version() {
    let text = readme();
    let status = text
        .split("\n## Status\n")
        .nth(1)
        .expect("README.md should have a `## Status` section")
        .split("\n## ")
        .next()
        .expect("a section ends at the next heading or at EOF");

    let first = status
        .split_whitespace()
        .find_map(|w| {
            let w = w.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-');
            w.strip_prefix('v')
                .filter(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
                .map(str::to_string)
        })
        .expect("`## Status` should name a version — it is what the section is about");

    assert_eq!(
        first,
        crate_version(),
        "`## Status` leads with v{first}, but this crate is {}. The section is \
         what a stranger reads to learn what day currently does; `assess docs` \
         cannot catch this because the install pin already satisfies \
         \"README.md carries the current version\".",
        crate_version()
    );
}

/// **The bootstrap script's install advice carries the same derived pins.**
///
/// The one surface whose entire job is teaching an uncontextualized user how
/// to install printed the two commands the README documents as broken: a
/// plain `cargo install day` errors (every release is a pre-release), and a
/// plain `cargo install kan` installs the one stable kan, 0.1.0 — below the
/// measured floor, and the worse failure because it looks like it worked
/// (day#50). The README's pins are derived and tested; the script's were
/// hand-written and tested by nothing. This drives the script with the
/// binaries hidden and asserts its output names the same pins the README's
/// tests derive — so the script cannot drift from the measurement any more
/// than the README can.
#[test]
fn the_bootstrap_script_pins_the_versions_it_tells_a_stranger_to_install() {
    let script = repo_root().join("hooks/bootstrap-check.sh");
    let out = std::process::Command::new("sh")
        .arg(&script)
        // A PATH without ~/.cargo/bin hides day and kan while keeping the
        // POSIX tools the script itself needs.
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("bootstrap-check.sh should run");
    assert!(out.status.success(), "the script exits 0 unconditionally");
    let msg = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        msg.contains("cargo install day --version"),
        "with day missing, the script should print a pinned day install; got: {msg}"
    );

    let day_pin = format!("cargo install day --version {}", crate_version());
    assert!(
        msg.contains(&day_pin),
        "the script's day pin should be this crate's version — expected \
         `{day_pin}` in: {msg}"
    );

    // The kan pin: newest `ok` row of the compat table, exactly as
    // `the_documented_kan_version_is_one_day_was_measured_against` derives it.
    let tsv = std::fs::read_to_string(repo_root().join("tests/fixtures/kan-compat.tsv"))
        .expect("kan-compat.tsv should ship");
    let newest_ok = tsv
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let mut cols = l.split('\t');
            let version = cols.next()?;
            (cols.next()? == "ok").then_some(version)
        })
        .last()
        .expect("kan-compat.tsv records at least one ok row");
    let kan_pin = format!(
        "cargo install kan --version {}",
        newest_ok.trim_start_matches('v')
    );
    assert!(
        msg.contains(&kan_pin),
        "the script's kan pin should be the newest measured-ok kan — expected \
         `{kan_pin}` in: {msg}"
    );

    // And the broken forms are gone: an unpinned install command must not
    // appear. A pinned line contains the unpinned prefix, so assert on the
    // JSON line-break that followed the bare command.
    for broken in ["cargo install day\\n", "cargo install kan\\n"] {
        assert!(
            !msg.contains(broken),
            "the script still prints the unpinned `{broken}` the README \
             documents as broken (day#50)"
        );
    }
}
