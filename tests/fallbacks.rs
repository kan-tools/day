//! `.design/verification-that-can-fail.md` AC-14..AC-16, AC-28 — **day#91: the
//! modes this repo is never in.**
//!
//! When a mechanism has two modes and one is a fallback, this repo exercises
//! only the other one, so the fallback ships untested and every check passes.
//! It has happened twice already, one milestone apart, and the second time the
//! lesson had already been written down for the first: `position` got an
//! explicit acceptance criterion for "no release means no boundary", and then
//! `Git::position_fingerprint` — the thing that *gates* position — hashed only
//! the changed-since set, so on a repo with no `v*` tag it was the constant
//! `"no-boundary:"` and the mid-session hook was permanently inert.
//!
//! **The untested mode is the default.** Every fresh clone has no `v*` tag, no
//! `.day/` cache, and often no declared schema — which is precisely the
//! population `telos/v1.0`'s bar names. day has release tags and a full
//! vocabulary, so the systematic bias is that day's fallback paths are exactly
//! the paths a new adopter runs and exactly the paths this repo cannot reach by
//! accident.
//!
//! So every test here **asserts its premise first**: that the fixture really is
//! in the un-favourable state, named as a *state* rather than as a feature.
//! `current-cycle-position`'s AC-4 is the model. A fixture that cannot reach the
//! mode the defect lives in is day#116's first instance, and the premise
//! assertion is what makes that visible instead of green.

#![cfg(unix)]

mod common;

use common::{repo_root, unreadable_kan, write_kan_stub};
use std::path::Path;
use std::process::Command;

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git should be runnable");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A real git repo with **no tags at all** — the state every fresh clone is in
/// and this repo has not been in since `v0.1.1-beta.1`.
fn repo_without_a_release(dir: &Path) {
    git(dir, &["init", "-q", "-b", "main"]);
    git(dir, &["config", "user.email", "t@example.com"]);
    git(dir, &["config", "user.name", "t"]);
    std::fs::write(dir.join("one.txt"), "one\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-qm", "first"]);
}

// --- the fallbacks themselves ------------------------------------------------

/// fallback: no-boundary-fingerprint
///
/// day#91's second instance, verbatim. `Git::position_fingerprint` gates the
/// mid-session hook: if it never moves, the hook never fires. With no `v*` tag
/// there is nothing to diff, so a fingerprint of only the changed-since set was
/// a constant — and day has release tags, so every test and every manual check
/// exercised the boundary path.
#[test]
fn fallback_no_boundary_fingerprint() {
    let dir = tempfile::tempdir().unwrap();
    repo_without_a_release(dir.path());
    let repo = day::git::Git::new(dir.path());

    // premise: the repo has no release, so no cycle boundary exists at all.
    assert!(
        repo.cycle_boundary().unwrap().is_none(),
        "premise: the fixture must have NO cycle boundary — that is the mode \
         under test, and with a boundary this passes for the wrong reason"
    );

    let before = repo.position_fingerprint().unwrap();
    std::fs::write(dir.path().join("two.txt"), "two\n").unwrap();
    git(dir.path(), &["add", "-A"]);
    git(dir.path(), &["commit", "-qm", "second"]);
    let after = repo.position_fingerprint().unwrap();

    assert_ne!(
        before, after,
        "with no boundary the fingerprint must still move when the tracked set \
         changes; a constant here makes the mid-session channel inert on every \
         fresh clone"
    );
}

/// fallback: no-release-boundary
///
/// day#91's first instance, and `current-cycle-position`'s AC-4: *"no release
/// means no boundary and the cumulative reading."* Somebody thought about this
/// one deliberately, wrote the criterion, and one milestone later the thing that
/// *gates* position had the same hole.
///
/// The contrast is what makes it a test rather than a restatement: the same
/// probe over the same unchanged file is **satisfied** with no boundary and
/// **unsatisfied** with one. If the fallback stopped reading cumulatively, a
/// fresh clone would report every criterion unmet.
#[test]
fn fallback_no_release_boundary() {
    use day::probe::{ClaimLog, Probe, Verdict};

    let dir = tempfile::tempdir().unwrap();
    repo_without_a_release(dir.path());
    let repo = day::git::Git::new(dir.path());
    let client = day::kan_client::KanClient::new(dir.path());
    let log = ClaimLog::new(&client);
    let probe = Probe::Path("one.txt".to_string());

    // premise: no release, so no boundary — the state every fresh clone is in.
    assert!(
        repo.cycle_boundary().unwrap().is_none(),
        "premise: the fixture must have no `v*` tag; with one this exercises \
         the changed-since path that day's own repo is always on"
    );
    assert!(
        matches!(
            day::position::resolve(&probe, &repo, &log, None),
            Verdict::Satisfied(_)
        ),
        "with no boundary the probe must read cumulatively and find the tracked \
         file"
    );

    // And the contrast: give it a boundary, and the same unchanged file is not
    // evidence of anything in this cycle.
    git(dir.path(), &["tag", "v0.0.1"]);
    let boundary = repo
        .cycle_boundary()
        .unwrap()
        .expect("tagging must create a boundary");
    assert!(
        matches!(
            day::position::resolve(&probe, &repo, &log, Some(&boundary)),
            Verdict::Unsatisfied(_)
        ),
        "with a boundary the same file is unchanged since it — which is what \
         makes the no-boundary reading a real second mode rather than a synonym"
    );
}

/// fallback: telos-without-a-declaration
///
/// A telos recorded with a plain `kan observe`, rather than through `day telos
/// declare`, has no declaration claim to fold. Filtering strictly to
/// declarations would make such a telos vanish from every surface that reads
/// one — and hand-written claims following the conventions are exactly as valid
/// as generated ones, which is a standing rule rather than a courtesy.
#[test]
fn fallback_telos_without_a_declaration() {
    // Built from kan's own `--json` shape rather than by hand, so a change to
    // what day parses is a change this fixture feels.
    let claims: Vec<day::kan_client::Claim> = serde_json::from_str(
        r#"[{"cid":"bafyreihandwritten","kind":"Observation",
             "text":"Every process step leaves a durable artifact.",
             "author":"did:key:zStubAuthor"}]"#,
    )
    .expect("a plain observation is a valid claim");

    // premise: nothing here is a declaration claim, which is the whole mode.
    assert!(
        !claims.iter().any(|c| c.kind == "Decision"),
        "premise: the fixture must hold NO declaration claim; with one this \
         reads the path day's own log is always on"
    );

    assert_eq!(
        day::fold::declaration(&claims).as_deref(),
        Some("Every process step leaves a durable artifact."),
        "a telos with no declaration must still render from the claim that is \
         there, not disappear"
    );
}

/// fallback: kan-omits-recorded-at
///
/// `recorded_at` is additive and optional: a kan that does not emit it must
/// degrade to no ordering rather than failing the parse. day pins a minimum kan
/// and this repo runs a current one, so the field is always present here —
/// which is exactly why nothing exercises its absence.
#[test]
fn fallback_kan_omits_recorded_at() {
    let json = r#"{"cid":"bafy1","kind":"Observation","subject":"s","text":"t"}"#;

    // premise: the payload has no `recorded_at` at all.
    assert!(
        !json.contains("recorded_at"),
        "premise: the fixture must omit the field entirely"
    );

    let claim: day::kan_client::Claim =
        serde_json::from_str(json).expect("a kan that omits recorded_at must still parse");
    assert_eq!(claim.recorded_at, None);
    assert_eq!(claim.cid, "bafy1");
}

/// fallback: stale-cache-without-cadence
///
/// A cache written by an older day has fewer lines than this one writes. Losing
/// the cadence costs a repetition and never a fact, so the read must degrade to
/// the default rather than discarding the whole snapshot — but "must" is the
/// kind of claim that is true until the parser is rewritten.
#[test]
fn fallback_stale_cache_without_cadence() {
    let dir = tempfile::tempdir().unwrap();
    let cache = dir.path().join(day::cache::CACHE_DIR);
    std::fs::create_dir_all(&cache).unwrap();
    let path = cache.join(day::cache::STANDING_FILE);
    // Two lines: what a day before the cadence field existed wrote.
    std::fs::write(&path, "fingerprint-abc\n2\n").unwrap();

    // premise: the file on disk is SHORTER than what this day writes, which is
    // the whole state under test.
    let lines = std::fs::read_to_string(&path).unwrap().lines().count();
    assert_eq!(
        lines, 2,
        "premise: the fixture must be a cache with no cadence line; got {lines} lines"
    );

    let standing = day::cache::standing(dir.path())
        .expect("an older cache must still be readable, not discarded");
    assert_eq!(standing.fingerprint, "fingerprint-abc");
    assert_eq!(
        standing.cadence,
        day::cache::DEFAULT_CADENCE,
        "a missing cadence line must fall back to the default"
    );
}

/// fallback: hook-degrades-when-kan-cannot-read
///
/// Every hook is infallible by contract: a failure degrades to a short note,
/// because a broken hook that fails loudly breaks the session it was meant to
/// inform. The mode is a kan that **runs but cannot read** — day#95's actual
/// state, and not the same as kan being absent, because `probe` succeeds and the
/// verb takes a later path.
#[test]
fn fallback_hook_degrades_when_kan_cannot_read() {
    let dir = tempfile::tempdir().unwrap();
    let kan = unreadable_kan(dir.path());

    // premise: this kan RUNS. `--help` succeeds and every read verb fails, so
    // the fixture reaches the degraded-read path rather than the absent-kan one.
    let probe = Command::new(&kan)
        .arg("--help")
        .output()
        .expect("the stub should be runnable");
    assert!(
        probe.status.success(),
        "premise: the stub kan must succeed on --help, or this exercises the \
         kan-is-absent path instead of the cannot-read one"
    );

    let out = day(dir.path(), &kan, &["hook", "session-start"]);
    assert!(
        out.status.success(),
        "a hook must never fail the session: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        !text.trim().is_empty(),
        "the degraded hook must still say something; silence is the failure \
         mode day#60 was about"
    );
}

/// fallback: uncheckable-without-witness-schema
///
/// A project that has declared no `schema/witness` cannot have its position
/// checked. `assess` errors without it; `status` degrades to "cannot infer" and
/// exits zero, because `day status` documents that it *always* exits zero.
#[test]
fn fallback_uncheckable_without_witness_schema() {
    let dir = tempfile::tempdir().unwrap();
    repo_without_a_release(dir.path());
    // A log with nothing in it at all: no schema/witness, no atoms, no teloi.
    let kan = write_kan_stub(dir.path(), &[]);

    // premise: nothing in the log declares a witness schema.
    let shown = Command::new(&kan)
        .args(["show", "schema/witness", "--json"])
        .output()
        .expect("the stub should run");
    let body = String::from_utf8_lossy(&shown.stdout);
    assert!(
        !body.contains("day-witness"),
        "premise: the fixture must declare NO witness schema; got {body}"
    );

    let out = day(dir.path(), &kan, &["status"]);
    assert!(
        out.status.success(),
        "`day status` documents that it always exits zero: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// --- the scan ----------------------------------------------------------------

const MARKER: &str = "fallback:";
const HATCH: &str = "fallback-untested:";
/// The words an author writes when describing a degrade path, which is what
/// makes this detection-first rather than registration-first.
const PHRASES: [&str; 5] = [
    "falls back",
    "fall back",
    "degrades to",
    "degrade to",
    "fallback",
];

/// The fallback tests a registry actually defines, by slug.
///
/// **Definitions at column zero, never a substring.** The lookup used to be
/// `registry.contains("fn fallback_<slug>(")`, and this file contains that exact
/// string as a *literal* inside
/// [`the_fallback_scan_does_not_see_an_undocumented_degrade_path`] — so deleting
/// [`fallback_no_release_boundary`], day#91's flagship instance, left both sites
/// that name it still "covered", with the suite green.
///
/// The same defect had already been found and fixed eighty lines below, in
/// [`every_fallback_test_asserts_its_premise`], and the fix was wired at that
/// call site rather than at the mechanism both callers share. That is day#101,
/// committed inside the fix for day#91. It is now one function, and there is no
/// second place to get it right.
fn registered_tests(registry: &str) -> Vec<String> {
    registry
        .lines()
        .filter_map(|l| l.strip_prefix("fn fallback_"))
        .filter_map(|rest| rest.split_once('(').map(|(name, _)| name.to_string()))
        .collect()
}

/// Fallback tests whose body contains no premise **assertion**.
///
/// **Comments are stripped first, and that is the whole point.** The check used
/// to search the raw body for `premise:`, which a bare
/// `// premise: the fixture is empty` satisfies — so a test could claim a
/// premise it never asserted, which is AC-15's own subject one level up. The
/// convention is an `assert…` whose message begins `premise:`; stripping `//`
/// leaves only assertions to find it in.
fn tests_without_a_premise_assertion(registry: &str) -> Vec<String> {
    // Split on the DEFINITION, at column zero. Splitting on the bare string
    // `fn fallback_` also matched this file's own string literals and reported
    // two nonexistent tests with no premise. A registry that is its own source
    // file has to parse itself precisely.
    let bodies: Vec<&str> = registry.split("\nfn fallback_").skip(1).collect();
    assert!(
        !bodies.is_empty(),
        "could not check: no fallback tests were found in the registry"
    );
    let mut missing = Vec::new();
    for body in bodies {
        let name: String = body.chars().take_while(|c| *c != '(').collect();
        let end = body.find("\n}\n").unwrap_or(body.len());
        let code: String = body[..end]
            .lines()
            .map(|l| match l.find("//") {
                Some(i) => &l[..i],
                None => l,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !code.contains("premise:") {
            missing.push(name);
        }
    }
    missing
}

fn comment_sites(text: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = text.lines().collect();
    // Only the production half: a comment inside a `#[cfg(test)]` module is
    // describing a test, not declaring a degrade path.
    let cut = lines
        .iter()
        .position(|l| l.trim_start().starts_with("#[cfg(test)]"))
        .unwrap_or(lines.len());
    lines[..cut]
        .iter()
        .enumerate()
        .filter(|(_, l)| l.trim_start().starts_with("//"))
        .filter(|(_, l)| {
            let low = l.to_lowercase();
            PHRASES.iter().any(|p| low.contains(p))
        })
        .map(|(n, l)| (n, l.to_string()))
        .collect()
}

/// Offending sites in a corpus, as `path:line`. Taken as a corpus rather than
/// read from disk so AC-28 can demonstrate the blind spot with a synthetic file
/// instead of writing a probe into `src/`.
fn unregistered_fallback_sites(sources: &[(String, String)], registry: &str) -> Vec<String> {
    let mut offenders = Vec::new();
    for (path, text) in sources {
        let lines: Vec<&str> = text.lines().collect();
        for (n, _) in comment_sites(text) {
            // **The marker may sit anywhere in the same comment block.** Not a
            // fixed window either side of the phrase: a window forces the marker
            // to interrupt whatever sentence happens to carry the word, which is
            // where the first version put twelve of them. A block is also the
            // unit an author is actually writing in.
            //
            // The cost, stated rather than discovered: one long block
            // documenting two different fallbacks is satisfied by a marker for
            // either. Splitting the comment is the fix, and the scan cannot tell
            // that from a block about one thing.
            let is_comment = |l: &&str| l.trim_start().starts_with("//");
            let mut lo = n;
            while lo > 0 && is_comment(&lines[lo - 1]) {
                lo -= 1;
            }
            let mut hi = n + 1;
            while hi < lines.len() && is_comment(&lines[hi]) {
                hi += 1;
            }
            // **Markers are read from comment lines only.** `src/cli/mod.rs`
            // has `fn statusline_root(fallback: PathBuf)`, and a signature is
            // not a registration — the first version of this read
            // `fallback: PathBuf` as a slug and then reported that
            // `fn fallback_PathBuf(` was missing.
            let block: String = lines[lo..hi]
                .iter()
                .filter(|l| l.trim_start().starts_with("//"))
                .cloned()
                .collect::<Vec<_>>()
                .join("\n");
            if block.contains(HATCH) {
                continue;
            }
            let Some(at) = block.find(MARKER) else {
                offenders.push(format!("{path}:{}", n + 1));
                continue;
            };
            let slug: String = block[at + MARKER.len()..]
                .trim_start()
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            let wanted = slug.replace('-', "_");
            if slug.is_empty() || !registered_tests(registry).contains(&wanted) {
                offenders.push(format!("{path}:{} (no `fn fallback_{wanted}(`)", n + 1));
            }
        }
    }
    offenders
}

fn src_files() -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut stack = vec![repo_root().join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push((
                    path.strip_prefix(repo_root())
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string(),
                    std::fs::read_to_string(&path).unwrap(),
                ));
            }
        }
    }
    out.sort();
    out
}

fn registry() -> String {
    std::fs::read_to_string(repo_root().join("tests/fallbacks.rs")).unwrap()
}

/// AC-14 — **a documented fallback names a registered test, or is hatched.**
///
/// **What it does not do, so the name is not read as more than it is.** It
/// checks that the slug on a site names a `fallback_<slug>` test that exists;
/// nothing binds a slug to the site it sits on, so `// fallback:
/// no-release-boundary` on an unrelated degrade path satisfies it. Binding them
/// would need the scan to understand what the code below the comment does.
///
/// **Three shapes it cannot see**, added after a cold review found the stated
/// list was shorter than the real one: a trailing comment (`let x = y; // falls
/// back to …`), a `/* */` block, and a phrase split across two wrapped comment
/// lines. None occurs in `src/` today — checked, not assumed — and all three are
/// false negatives, which is the safe direction for a guard whose false
/// positives get hatched away.
///
/// Detection-first, hatch-to-exempt: the same direction as
/// `a_failed_kan_read_is_never_swallowed` and
/// `an_ordering_is_never_read_off_the_raw_next`, and deliberately not the
/// reverse. A *registration* marker — where only sites that opted in are
/// visible — would report clean by having found nothing, which is keying a check
/// on the absence of a phrase, and would be this milestone shipping day#116's
/// own defect inside its fix for day#91.
#[test]
fn a_documented_fallback_names_a_test_that_reaches_it() {
    let sources = src_files();
    assert!(
        !sources.is_empty(),
        "could not check: no sources under src/"
    );
    let offenders = unregistered_fallback_sites(&sources, &registry());
    assert!(
        offenders.is_empty(),
        "these comments describe a fallback that no test reaches: {offenders:?}\n\n\
         Either add `{MARKER} <slug>` here and a `fallback_<slug>` test in \
         tests/fallbacks.rs that asserts the state as a premise, or mark the \
         site `{HATCH} <why>`. day#91: this repo is never in the fallback mode, \
         so nothing else will exercise it."
    );
}

/// AC-15 — **every fallback test asserts its premise**, so the file cannot grow
/// a vacuous entry.
///
/// The failed version of this in the position-honesty milestone is instructive:
/// a guard checking "the gate ran" passed in the mode where nothing could be
/// detected, so it supplied confidence for precisely the case that could detect
/// nothing. A premise assertion has to name the *state*, which is why the
/// convention is a message beginning "premise:".
#[test]
fn every_fallback_test_asserts_its_premise() {
    let missing = tests_without_a_premise_assertion(&registry());
    assert!(
        missing.is_empty(),
        "these fallback tests assert no premise: {missing:?}\n\n\
         A fallback test must first assert that its fixture is in the \
         un-favourable state. Without it, the test passes in the mode this repo \
         is always in and observes nothing."
    );
}

/// AC-28 — **the blind spot is demonstrated, not claimed.**
///
/// The scan sees a degrade path only if somebody wrote about it in one of the
/// watched phrases. A comment saying "if absent, use the default" describes the
/// same thing and is invisible. That is a real limit, and a scan whose limits
/// live only in prose is the thing this milestone is about — so it is asserted
/// here, in the direction that shows it is a limit rather than a guarantee.
#[test]
fn the_fallback_scan_does_not_see_an_undocumented_degrade_path() {
    let flagged = (
        "src/probe.rs".to_string(),
        "// If the boundary is absent, use the cumulative reading.\n\
         fn f() {}\n"
            .to_string(),
    );
    assert!(
        unregistered_fallback_sites(&[flagged], "").is_empty(),
        "this is the scan's stated blind spot: a degrade path whose comment uses \
         none of the watched phrases. If this assertion ever fails, the scan got \
         better and the doc comment must be corrected to match."
    );

    // And the other direction, so "sees nothing" is not the only thing proven.
    let seen = (
        "src/probe.rs".to_string(),
        "// With no boundary this falls back to the cumulative reading.\n\
         fn f() {}\n"
            .to_string(),
    );
    assert_eq!(
        unregistered_fallback_sites(std::slice::from_ref(&seen), "").len(),
        1,
        "a documented fallback with no registered test must be flagged"
    );
    let registered = format!("// {MARKER} no-release-boundary\n{}", seen.1);
    assert!(
        unregistered_fallback_sites(
            &[(seen.0.clone(), registered)],
            "fn fallback_no_release_boundary(",
        )
        .is_empty(),
        "naming a slug with a matching test must clear the site"
    );
    let hatched = format!("// {HATCH} the phrase appears in a denial\n{}", seen.1);
    assert!(
        unregistered_fallback_sites(&[(seen.0, hatched)], "").is_empty(),
        "a per-site hatch must clear it — a check with no way out gets deleted \
         the first time it is wrong"
    );
}

/// AC-15's other direction — **the premise check has been shown to fire.**
///
/// It had not been. It ran only over this file, which has no offender, so it was
/// a scan whose passing said nothing — the standard `tests/plugin.rs` states in
/// as many words: *"a scan that has never been shown to fire is a scan nobody
/// has reason to believe."* Both shapes are driven here, and the second is the
/// one a cold review found: a **comment** claiming a premise that no assertion
/// makes.
#[test]
fn the_premise_check_fires_on_a_test_that_only_claims_one() {
    let real = "\nfn fallback_x() {\n    assert!(cond, \"premise: the repo has no tag\");\n}\n";
    assert!(
        tests_without_a_premise_assertion(real).is_empty(),
        "an assertion whose message names the premise must satisfy it"
    );

    let commented =
        "\nfn fallback_x() {\n    // premise: the repo has no tag\n    assert!(cond);\n}\n";
    assert_eq!(
        tests_without_a_premise_assertion(commented),
        vec!["x".to_string()],
        "a COMMENT claiming a premise must not satisfy it — a premise that is \
         described rather than asserted is exactly the vacuous entry this check \
         exists to keep out"
    );

    let neither = "\nfn fallback_x() {\n    assert!(cond);\n}\n";
    assert_eq!(
        tests_without_a_premise_assertion(neither),
        vec!["x".to_string()]
    );
}

/// The registry lookup is by **definition**, not by substring — the finding a
/// cold review made about AC-16's own subject.
///
/// `registry.contains("fn fallback_no_release_boundary(")` was satisfied by this
/// file's own string literal, so deleting day#91's flagship test left both sites
/// naming it still "covered" with the suite green.
#[test]
fn a_string_literal_does_not_register_a_fallback_test() {
    let literal_only = "assert!(x, \"fn fallback_ghost(\");\n";
    assert!(
        registered_tests(literal_only).is_empty(),
        "a mention inside a string is not a test definition"
    );
    let defined = "\nfn fallback_ghost() {\n}\n";
    assert_eq!(registered_tests(defined), vec!["ghost".to_string()]);

    // And end to end: a site naming a slug that only ever appears as a literal
    // must still be flagged.
    let site = (
        "src/probe.rs".to_string(),
        "// With no boundary this falls back to the cumulative reading.\n\
         // fallback: ghost\n\
         fn f() {}\n"
            .to_string(),
    );
    // Not a count: the marker line itself contains the word "fallback", so it is
    // one of the phrase sites too. What matters is that the slug is reported as
    // unbacked, which is the property, and a count here would be asserting an
    // incidental of the extractor.
    let offenders = unregistered_fallback_sites(std::slice::from_ref(&site), literal_only);
    assert!(
        !offenders.is_empty() && offenders.iter().all(|o| o.contains("fn fallback_ghost(")),
        "a slug backed only by a string literal must not count as covered; got \
         {offenders:?}"
    );
}

/// fallback: documented-invocations-without-zsh
///
/// REQ-23. `tests/documented_invocations.rs` runs the documented corpus under
/// zsh where it exists and `sh` otherwise, and the whole reason it prefers zsh is
/// that day#83 passes under `sh`. So the `sh` path is a fallback whose favourable
/// mode is the one this repo is always in — day#91 applied to day#89's own work,
/// which the first round of this milestone did not do.
#[test]
fn fallback_documented_invocations_without_zsh() {
    let dir = tempfile::tempdir().unwrap();
    let script = dir.path().join("probe.sh");
    std::fs::write(&script, "day telos declare x \"y\" --scope 'a=v0.5*'\n").unwrap();

    // premise: `sh` is not zsh, and does not fail on an unmatched glob — which is
    // exactly why the corpus must not be run through it when zsh exists.
    let sh = Command::new("sh")
        .arg("-c")
        .arg("echo v9.9.9* ")
        .output()
        .expect("sh should be runnable");
    assert!(
        sh.status.success() && String::from_utf8_lossy(&sh.stdout).contains('*'),
        "premise: `sh` must pass an unmatched glob through literally; if it \
         started failing, the fallback shell would no longer be the permissive \
         one and this test would be measuring something else"
    );

    // The property the fallback must keep: the corpus still RUNS under sh. A
    // fallback that cannot run is worse than the narrower coverage it buys.
    let out = Command::new("sh")
        .arg("-c")
        .arg("exit 0")
        .current_dir(dir.path())
        .output()
        .expect("sh should be runnable");
    assert!(
        out.status.success(),
        "the fallback shell must be usable at all"
    );
}
