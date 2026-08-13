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

use common::{repo_root, schema_claim, unreadable_kan, write_kan_stub};
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

/// fallback: bridge-check-errored
///
/// day#141: a bridge check that ERRORS is a could-not-check, never "its plan
/// could not reach it". `record_tier` used `.unwrap_or(false)`, so an atom
/// retracted after the bridge was declared — `bridge::Error::UndeclaredAtoms`,
/// a real state of a real log — rendered as a checked-and-negative verdict day
/// never computed. day's own log holds no broken bridge, which is exactly why
/// nothing else exercises this mode.
#[test]
fn fallback_bridge_check_errored() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = [
        common::claim(
            "telos/target",
            "bafyreit",
            "A telos with no witnesses, so no probe needs git.",
        ),
        common::claim(
            "bridge/broken",
            "bafyreib",
            "A bridge.\n\n```day-bridge\n{\"telos\": \"target\", \"have\": [\"intent\"], \
             \"plan\": {\"atom\": \"ghost\"}}\n```\n",
        ),
    ];
    // The plan names `ghost` and the log declares no atoms at all, so
    // `bridge::check` must error rather than answer — the mode under test,
    // distinct from "checked and unreachable".
    assert!(
        fixture.iter().all(|c| !c.subject.starts_with("atom/")),
        "premise: the fixture must declare NO atoms, so the check errors \
         rather than answers"
    );
    let kan = write_kan_stub(dir.path(), &fixture);

    let out = day(dir.path(), &kan, &["assess", "telos", "target"]);
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
    let probe = common::run_stub(&kan, &["--help"], dir.path());
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

/// An unreadable design subject refuses deduplication rather than guessing.
///
/// `newest_of_kind` asks whether an identical observe/plan pair is already on
/// the subject, so an unchanged design pass records nothing. `KanClient::show`
/// is served from the memoized bulk read and returns `Error::Unaccounted` for a
/// subject `status --json` listed and `show --all --json` did not — so the
/// question can fail to be answered, and the answer day assumes decides whether
/// a claim is written.
///
/// The visibility decision now lives before both deduplication helpers. They
/// receive only a checked claim slice, so neither can turn unreadable into
/// empty. A write may refuse; it must not guess whether a duplicate is safe.
#[test]
fn an_unreadable_subject_refuses_before_deduplication() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/design.rs"), "// fixture\n").unwrap();
    std::fs::create_dir_all(dir.path().join(".design")).unwrap();
    let doc = "# Feature: a thing\n\n## Summary\nIt does the thing.\n\n\
        ## Requirements\n- REQ-1: first\n\n## Acceptance Criteria\n\
        - [ ] AC-1: covers (REQ-1)\n\n## Architecture\nTouches `src/design.rs`.\n\n\
        ## Resolved Questions\n- RQ-1: chose the first thing\n";
    std::fs::write(dir.path().join(".design/thing.md"), doc).unwrap();

    let honest = write_kan_stub(dir.path(), &[schema_claim("design-doc", "bafyreischema")]);
    let record = |kan: &Path| {
        let output = day(dir.path(), kan, &["design", "record", ".design/thing.md"]);
        (
            output.status.success(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    };

    // Premise, asserted rather than assumed: against a readable log the second
    // pass DOES skip. Without this the test passes on a day that never skips at
    // all, which would make the fallback assertion below meaningless.
    assert!(record(&honest).0);
    let (readable_ok, readable_second) = record(&honest);
    assert!(readable_ok, "{readable_second}");
    assert!(
        readable_second.contains("(unchanged)"),
        "premise: the skip must work when the subject IS readable, or this test \
         cannot tell the fallback from the ordinary path and would pass on a day \
         that never skips at all: {readable_second}"
    );

    // Now the same second pass, with `thing` dropped from the bulk read only.
    // `status --json` still lists it, so day sees a subject it cannot account
    // for rather than a subject that does not exist.
    let dropping = dir.path().join("kan-dropping.sh");
    std::fs::write(
        &dropping,
        format!(
            "#!/bin/sh\n\
             if [ \"$1\" = show ] && [ \"$2\" = --all ]; then\n\
               {inner} \"$@\" | python3 -c 'import json,sys; d=json.load(sys.stdin); \
             d[\"subjects\"]=[e for e in d[\"subjects\"] if e[\"subject\"]!=\"thing\"]; \
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

    let (degraded_ok, degraded) = record(&dropping);
    assert!(!degraded_ok, "an unreadable write unexpectedly succeeded");
    assert!(
        degraded.contains("did not return") || degraded.contains("unaccounted"),
        "the refusal must name the failed read: {degraded}"
    );
}

/// **fallback: kan-omits-excluded-by-trust** — day#120.
///
/// `excluded_by_trust` is `#[serde(default)]`, so a kan that does not emit it
/// reads as zero and day behaves exactly as it did before: an empty subject is
/// absent, and the loaders offer their starter. That is the right degradation —
/// the alternative, treating "no field" as "possibly withheld", would refuse to
/// declare a schema on every genuinely-fresh project, which is the population
/// `telos/v1.0` names.
///
/// The mode is unreachable here by construction — day pins kan >= 0.9.1 and
/// every supported kan emits the field — which is exactly why it needs a test.
#[test]
fn fallback_kan_omits_excluded_by_trust() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join(".design")).unwrap();
    std::fs::write(
        dir.path().join(".design/t.md"),
        "# F: t\n\n## Summary\ns\n\n## Requirements\n- REQ-1: a\n\n\
         ## Acceptance Criteria\n- [ ] AC-1: a\n\n## Architecture\nx\n",
    )
    .unwrap();

    // A kan whose envelope carries NO `excluded_by_trust` key at all, for a
    // subject that is genuinely empty.
    let kan = dir.path().join("kan-old.sh");
    std::fs::write(
        &kan,
        "#!/bin/sh\n\
         case \"$1\" in\n\
           --version) echo 'kan 0.11.0-beta.1'; exit 0 ;;\n\
           --help) echo stub; exit 0 ;;\n\
           show) if [ \"$2\" = --all ]; then \
             printf '{\"v\":1,\"subjects\":[{\"v\":1,\"subject\":\"schema/design-doc\",\"claims\":[]}]}\\n'; \
             exit 0; fi ;;\n\
           status|issues) printf '{\"v\":1,\"subjects\":[{\"subject\":\"schema/design-doc\",\"state\":\"Unclassified\"}]}\\n'; exit 0 ;;\n\
         esac\n\
         exit 0\n",
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&kan, std::fs::Permissions::from_mode(0o755)).unwrap();

    // premise: the payload really does omit the field. Asserted against the
    // stub's own output, so a later edit that adds it makes this test say so
    // instead of quietly exercising the path day is always on.
    let payload = common::run_stub(&kan, &["show", "--all", "--json"], dir.path());
    let payload = String::from_utf8_lossy(&payload.stdout);
    assert!(
        !payload.contains("excluded_by_trust"),
        "premise: the fixture kan must omit `excluded_by_trust` entirely, or \
         this exercises the path every supported kan is on: {payload}"
    );

    let out = day(dir.path(), &kan, &["design", "check", ".design/t.md"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("no design-doc schema is declared"),
        "a kan that omits the field must degrade to the pre-day#120 reading — \
         absent, with the starter offered: {text}"
    );
    assert!(
        !text.contains("trust base"),
        "and must never claim claims were withheld, which it has no evidence \
         of: {text}"
    );
}

/// fallback: notice-degrades-when-kan-cannot-read
///
/// **A separate slug from `hook-degrades-when-kan-cannot-read`, deliberately.**
/// That test drives `hook session-start` and never reaches `session_notice`,
/// which is a different function with the opposite contract: session-start must
/// still say something when degraded (day#60 — silence is its failure mode),
/// and session-notice must say *nothing* unless it has something to report.
/// Reusing the slug would have satisfied `a_documented_fallback_names_a_test_
/// that_reaches_it` — its own doc admits nothing binds a slug to its site — with
/// no test reaching the path.
///
/// The mode is a kan that RUNS but cannot read. The compat check is the reason
/// this needs asserting rather than being obvious: `client.version()` fails
/// here, which classifies as `Compat::Unknown`, and `compat_notice` must
/// **stay quiet** on Unknown. `compat::render` phrases that case as "kan:
/// reachable, version unknown", and this hook never called `probe`, so emitting
/// it would have day assert a reachability it never established.
#[test]
fn fallback_notice_degrades_when_kan_cannot_read() {
    let dir = tempfile::tempdir().unwrap();
    let kan = unreadable_kan(dir.path());

    // premise: this kan RUNS, so the degraded-read path is reached rather than
    // the absent-kan one — which is `hooks/bootstrap-check.sh`'s job, not this.
    let probe = common::run_stub(&kan, &["--help"], dir.path());
    assert!(
        probe.status.success(),
        "premise: the stub kan must succeed on --help, or this exercises the \
         kan-is-absent path instead of the cannot-read one"
    );

    let out = day(dir.path(), &kan, &["hook", "session-notice"]);
    assert!(
        out.status.success(),
        "a hook must never fail the session: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.trim().is_empty(),
        "session-notice must stay silent when it cannot read: a notice here \
         would be day reporting a pairing or a transition it never established. \
         Got: {text}"
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
    let shown = common::run_stub(&kan, &["show", "schema/witness", "--json"], dir.path());
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
            .map(common::strip_line_comments)
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
/// **The first version of this test was vacuous**, which a second cold review
/// caught: it wrote a probe script it never read, then asserted that
/// `sh -c 'echo v9.9.9*'` emits a `*` and `sh -c 'exit 0'` exits 0 — two
/// properties of `sh`, neither able to fail, in the round that fixed the premise
/// check for exactly this. It never invoked day at all.
///
/// What it must assert is the property the fallback carries: **a real documented
/// invocation still runs under `sh`.** Narrower coverage is the price of the
/// fallback; not running at all would make it worthless.
#[test]
fn fallback_documented_invocations_without_zsh() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_day"), bin.join("day")).unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    // premise: `sh` is NOT zsh, and passes an unmatched glob through literally —
    // which is the whole reason the corpus prefers zsh, and the reason this
    // fallback buys less than the mode it replaces.
    let glob = Command::new("sh")
        .arg("-c")
        .arg("echo v9.9.9*")
        .output()
        .expect("sh should be runnable");
    assert!(
        glob.status.success() && String::from_utf8_lossy(&glob.stdout).contains('*'),
        "premise: `sh` must pass an unmatched glob through literally. If it \
         started refusing, the fallback would no longer be the permissive shell \
         and this test would be measuring something else."
    );

    // A real invocation from `docs/CONVENTIONS.md`, run through the fallback
    // shell against a stub kan — the same bar `documented_invocations.rs` holds
    // the corpus to: it must parse and run.
    let documented = "day telos declare v05-shipped \"day v0.5 is published.\" \
                      --witness published-artifact --scope 'published-artifact=v0.5*'";
    let out = Command::new("sh")
        .arg("-c")
        .arg(documented)
        .current_dir(dir.path())
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .env("DAY_KAN_BIN", &kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("sh should be runnable");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !text.contains("For more information, try '--help'") && !text.contains("command not found"),
        "a documented invocation must still parse and run under the fallback \
         shell: {text}"
    );
    assert!(
        common::appends(dir.path())
            .iter()
            .any(|a| a.contains("v05-shipped")),
        "and it must actually reach kan, not merely exit quietly"
    );
}

/// fallback: legacy-witness-block
///
/// **The mode day's own repo is never in.** `src/layers.rs` resolves a witness
/// from three layers, and this repo has a legacy whole-block claim on
/// `schema/witness` and no per-key subjects at all — so every run here takes the
/// legacy path and nothing exercises the other side of the branch. That is
/// day#91's shape exactly: the fallback is the mode in production, and the new
/// path is the one no existing test would reach.
///
/// The premise is asserted **both ways round**, because the contrast is what
/// makes this a test rather than a restatement: the same key resolves from the
/// block when no per-key subject exists, and from the key's own subject when one
/// does.
#[test]
fn fallback_legacy_witness_block() {
    use day::kan_client::KanClient;
    use day::probe::Probe;
    use day::telos::WitnessSchema;

    let block = common::claim(
        "schema/witness",
        "bafylegacyfallback",
        "Witness probes.\n\n```day-witness\n{\"published-artifact\": {\"tag\": \"v*\"}}\n```\n",
    );
    let key = common::claim(
        "schema/witness/published-artifact",
        "bafyperkey",
        "The published-artifact witness.\n\n```day-witness\n{\"tag\": \"v9.*\"}\n```\n",
    );

    let resolve = |claims: &[common::StubClaim]| {
        let dir = tempfile::tempdir().unwrap();
        let bin = write_kan_stub(dir.path(), claims);
        let client = KanClient::with_bin(dir.path(), bin.to_string_lossy().to_string());
        match WitnessSchema::load(&client)
            .unwrap()
            .probes
            .get("published-artifact")
        {
            Some(Probe::Tag(p)) => p.clone(),
            other => panic!("expected a tag probe, got {other:?}"),
        }
    };

    let legacy_only = [block.clone()];
    assert!(
        !legacy_only
            .iter()
            .any(|c| c.subject.starts_with("schema/witness/")),
        "premise: the fixture must declare no `schema/witness/<key>` subject. \
         With one, this takes the per-key path and observes nothing about the \
         fallback — which is the mode day's own repo is always in."
    );

    assert_eq!(
        resolve(&legacy_only),
        "v*",
        "with no per-key subject the legacy block must decide — anything else \
         would be a silent migration for every project that has one"
    );

    // and the branch this repo never takes.
    assert_eq!(
        resolve(&[block, key]),
        "v9.*",
        "with a per-key subject present it must win; if this reads `v*` the \
         per-key layer is inert and the fallback is the only path there is"
    );
}

/// The footer names the repo from the directory when there is no remote —
/// the fresh `git init` state, which is the population `telos/v1.0` names
/// (`.design/harness-footer.md` REQ-12).
#[test]
fn fallback_no_remote() {
    let dir = tempfile::tempdir().unwrap();
    repo_without_a_release(dir.path());

    let out = Command::new("git")
        .args(["remote"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        "premise: the fixture must have no remote — with one configured this \
         exercises the org/name path day's own repo is always on"
    );

    let git = day::git::Git::new(dir.path());
    assert_eq!(
        git.remote_url().unwrap(),
        None,
        "no remote must read as None, never as an error"
    );
    let ctx = day::hooks::footer_context(&git);
    let dirname = std::fs::canonicalize(dir.path())
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        ctx.repo.as_deref(),
        Some(dirname.as_str()),
        "with no remote the footer names the main checkout's directory"
    );
}

/// A remote URL of no recognised shape falls back to the directory name
/// rather than a mangled `org/name` (`.design/harness-footer.md` REQ-14) —
/// a wrong repo name is worse than a plain one.
#[test]
fn fallback_unrecognised_remote() {
    let dir = tempfile::tempdir().unwrap();
    repo_without_a_release(dir.path());
    git(dir.path(), &["remote", "add", "origin", "/a/local/path"]);

    // **The premise reads the FIXTURE, not the parser.** It used to assert
    // `repo_from_remote("/a/local/path").is_none()` — a property of the code
    // under test, true whatever the fixture does, so deleting the `remote add`
    // above left this passing and the test measuring REQ-12's fallback
    // instead of REQ-14's. A premise that inspects only its own side of the
    // interface cannot notice when the scenario evaporates.
    let configured = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(dir.path())
        .output()
        .expect("git should be runnable");
    let url = String::from_utf8_lossy(&configured.stdout)
        .trim()
        .to_string();
    assert_eq!(
        url, "/a/local/path",
        "premise: the fixture must have a remote configured, and of no \
         recognised shape — with none, this exercises the no-remote fallback"
    );
    assert!(
        day::footer::repo_from_remote(&url).is_none(),
        "premise: and that configured URL must be unrecognised, or this reads \
         the org/name path"
    );

    let ctx = day::hooks::footer_context(&day::git::Git::new(dir.path()));
    let dirname = std::fs::canonicalize(dir.path())
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        ctx.repo.as_deref(),
        Some(dirname.as_str()),
        "an unrecognised remote must yield the directory name, never a \
         mangled org/name"
    );
}

/// Every footer read failing omits every segment — the footer's context
/// assembly must not fail a hook over a decoration
/// (`.design/harness-footer.md` REQ-7).
#[test]
fn fallback_footer_reads_degrade() {
    let dir = tempfile::tempdir().unwrap();
    let git = day::git::Git::with_bin(
        dir.path(),
        dir.path().join("no-such-git").display().to_string(),
    );

    assert!(
        git.sync_state().is_err(),
        "premise: every git read must fail — against a working repo this \
         exercises the populated path instead"
    );

    let ctx = day::hooks::footer_context(&git);
    assert!(
        ctx.repo.is_none() && ctx.branch.is_none() && ctx.sync.is_none() && ctx.checkout.is_none(),
        "every segment must be omitted, not defaulted: {ctx:?}"
    );

    // And what remains still renders: one line, no empty context line.
    let rendered = day::footer::render_unreadable(
        &day::footer::Surround {
            context: ctx,
            role: None,
            withheld: 0,
        },
        day::footer::Style::Plain,
        100,
    );
    assert_eq!(
        rendered.lines().count(),
        1,
        "an all-absent context earns no line: {rendered:?}"
    );
}

/// A cache written by an older day carries no variants file, and the status
/// line must serve the single rendering it did write rather than a blank bar
/// (`.design/harness-footer.md`, the width-variant change).
///
/// Found by dogfooding the *other* direction: a debug build's hook wrote the
/// variants and the installed release printed every one of them at once,
/// because a single file changed shape under two binaries that are the same
/// program in principle and different builds in practice.
#[test]
fn fallback_cache_without_variants() {
    let dir = tempfile::tempdir().unwrap();
    let cache = dir.path().join(day::cache::CACHE_DIR);
    std::fs::create_dir_all(&cache).unwrap();
    std::fs::write(cache.join(day::cache::STATUS_LINE_FILE), "day - build").unwrap();

    assert!(
        !cache.join(day::cache::VARIANTS_FILE).exists(),
        "premise: the variants file must be absent — with one present this \
         exercises the current-format path, which every other session is on"
    );

    assert_eq!(
        day::cache::read_status_line(dir.path()).as_deref(),
        Some("day - build"),
        "an older day's cache must still serve"
    );

    // And what the status line does with it: no variant headers, so it is
    // printed as it stands rather than dropped.
    assert_eq!(
        day::footer::select("day - build", 80, day::footer::Style::Emoji),
        None,
        "text with no variant headers must report that it has none, so the \
         caller prints it verbatim"
    );
}

/// `COLUMNS` unset — a Claude Code older than v2.1.153, or a hand-run
/// `day status-line` — lays the footer out against an assumed 80 columns
/// rather than against zero, which would elide everything.
#[test]
fn fallback_no_columns_assume_80() {
    let signals = day::footer::EnvSignals::default();
    assert!(
        signals.columns.is_none(),
        "premise: COLUMNS must be unset — with it set this measures the \
         ordinary path Claude Code puts every session on"
    );
    assert_eq!(signals.width(), day::footer::ASSUMED_COLUMNS);

    // An unparseable or zero value is the same state, not a zero-width
    // terminal: laying out against 0 would drop every segment.
    for bogus in ["", "   ", "not-a-number", "0"] {
        let signals = day::footer::EnvSignals {
            columns: Some(bogus.to_string()),
            ..day::footer::EnvSignals::default()
        };
        assert_eq!(
            signals.width(),
            day::footer::ASSUMED_COLUMNS,
            "COLUMNS={bogus:?} must read as unknown, never as a width"
        );
    }

    // And a real value is honoured, or the assertions above pass vacuously.
    let signals = day::footer::EnvSignals {
        columns: Some("133".into()),
        ..day::footer::EnvSignals::default()
    };
    assert_eq!(signals.width(), 133);
}

/// A `schema/injection` fixture: the whole-block legacy claim, and a per-key
/// claim for one field.
///
/// Shared by the three tests below so the same log shape is read three ways —
/// the layers only mean anything relative to each other.
fn injection_client(
    dir: &std::path::Path,
    claims: &[common::StubClaim],
) -> day::kan_client::KanClient {
    let bin = write_kan_stub(dir, claims);
    day::kan_client::KanClient::with_bin(dir, bin.to_string_lossy().to_string())
}

/// fallback: config-shipped-default
///
/// **Layer 1, and the mode a fresh clone is in.** With no claim anywhere, every
/// field of a config struct resolves to day's shipped default and reports
/// `Layer::Default`. Registered rather than hatched because the variant is
/// reachable and reached: `Layer`'s own doc predicted it would arrive with the
/// first loader that has defaults to fall back to, and this is that loader.
///
/// The premise is asserted rather than assumed — a fixture that accidentally
/// carried an injection claim would observe the legacy layer and still pass.
#[test]
fn fallback_config_shipped_default() {
    use day::blocks::InjectionSchema;
    use day::layers::{self, Layer};

    let dir = tempfile::tempdir().unwrap();
    let unrelated = [common::claim(
        "practice",
        "bafyreiunrelated",
        "An item that has nothing to do with injection.",
    )];
    assert!(
        !unrelated
            .iter()
            .any(|c| c.subject.starts_with("schema/injection")),
        "premise: the fixture must carry no `schema/injection` claim, or this \
         observes a declared value and says nothing about the default layer"
    );

    let client = injection_client(dir.path(), &unrelated);
    let effective = layers::config::<InjectionSchema>(&client, "injection").unwrap();

    assert_eq!(
        effective.value,
        InjectionSchema::default(),
        "with nothing declared, every field is day's shipped default"
    );
    assert_eq!(
        effective.provenance.get("cadence"),
        Some(&Layer::Default),
        "and it says so, rather than reporting a claim that does not exist"
    );
    assert!(
        !effective.declared,
        "nothing declared it: `declared` is about whether a claim contributed, \
         not about whether a value came out"
    );
}

/// fallback: legacy-config-block
///
/// **Layer 2, and the mode day's own repo is in today.** A whole-block claim on
/// `schema/injection` sets every field it carries, and a project that never
/// adopts per-key subjects must see exactly today's behaviour — REQ-12's "no
/// migration", which is a promise about every existing project rather than a
/// nicety.
///
/// The premise is asserted **both ways round**, following
/// `fallback_legacy_witness_block`: the same key resolves from the block when no
/// per-key subject exists, and from the key's own subject when one does. Without
/// the contrast this restates the fixture.
#[test]
fn fallback_legacy_config_block() {
    use day::blocks::InjectionSchema;
    use day::layers::{self, Layer};

    let block = common::claim(
        "schema/injection",
        "bafyreilegacyinjection",
        "Injection settings.\n\n```day-injection\n{\"cadence\": 7}\n```\n",
    );
    let key = common::claim(
        "schema/injection/cadence",
        "bafyreiperkeycadence",
        "Cadence for this project.\n\n```day-injection\n{\"cadence\": 99}\n```\n",
    );

    let resolve = |claims: &[common::StubClaim]| {
        let dir = tempfile::tempdir().unwrap();
        let client = injection_client(dir.path(), claims);
        layers::config::<InjectionSchema>(&client, "injection").unwrap()
    };

    let legacy_only = [block.clone()];
    assert!(
        !legacy_only
            .iter()
            .any(|c| c.subject.starts_with("schema/injection/")),
        "premise: the fixture must declare no `schema/injection/<key>` subject. \
         With one, this takes the per-key path and observes nothing about the \
         fallback — which is the mode day's own repo is always in."
    );

    let from_block = resolve(&legacy_only);
    assert_eq!(
        from_block.value.cadence, 7,
        "with no per-key subject the legacy block must decide — anything else \
         is a silent migration for every project that has one"
    );
    assert_eq!(
        from_block.provenance.get("cadence"),
        Some(&Layer::LegacyBlock("bafyreilegacyinjection".into()))
    );

    // The other way round: the same block, now overlaid.
    let with_key = resolve(&[block, key]);
    assert_eq!(
        with_key.value.cadence, 99,
        "a per-key claim overrides the legacy block for its own key"
    );
    assert_eq!(
        with_key.provenance.get("cadence"),
        Some(&Layer::Key("bafyreiperkeycadence".into()))
    );
    assert_eq!(
        with_key.value.max_practice_items,
        InjectionSchema::default().max_practice_items,
        "and leaves the block's other fields alone — the whole point of per-key \
         subjects is that setting one does not reset the rest"
    );
}

/// fallback: retracted-key-subject
///
/// **REQ-21's shape, which is what `kan retract` actually leaves behind.** The
/// subject REMAINS after a retraction, carrying only a `Retraction` and no
/// block. "Subject exists, no block" must read as *this key is absent* — so the
/// key falls back through the layers below it — and never as a read failure.
///
/// This is the mechanism granular retraction turns on, and it is the one the
/// design says the whole-block shape structurally cannot offer. A stub whose
/// folded view omits the retracted claim is what day actually reads, so that is
/// what this drives.
#[test]
fn fallback_retracted_key_subject() {
    use day::blocks::InjectionSchema;
    use day::layers::{self, Layer};

    let dir = tempfile::tempdir().unwrap();
    // The per-key subject exists and carries no `day-injection` block, which is
    // the post-retraction state: kan folds the retracted claim out of the view
    // and the subject stays.
    let claims = [
        common::claim(
            "schema/injection",
            "bafyreilegacyinjection",
            "Injection settings.\n\n```day-injection\n{\"cadence\": 7}\n```\n",
        ),
        common::retraction_claim("schema/injection/cadence", "bafyreiretraction"),
    ];
    assert!(
        claims
            .iter()
            .any(|c| c.subject == "schema/injection/cadence"),
        "premise: the per-key SUBJECT must be present, or this measures the \
         no-subject path instead of the retracted one"
    );

    let client = injection_client(dir.path(), &claims);
    let effective = layers::config::<InjectionSchema>(&client, "injection").unwrap();

    assert_eq!(
        effective.value.cadence, 7,
        "a retracted key falls back to the layer below it — here the legacy \
         block. Reporting a read failure instead would make retraction \
         unusable, which is the capability this design exists for."
    );
    assert_eq!(
        effective.provenance.get("cadence"),
        Some(&Layer::LegacyBlock("bafyreilegacyinjection".into())),
        "and provenance names the layer that actually decided it"
    );
}
