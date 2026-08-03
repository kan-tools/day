//! `.design/declared-blocks.md` — a project's declarations must have an
//! **effect**, not merely parse.
//!
//! The unit tests in `src/blocks.rs` cover the schemas themselves. These exist
//! because those were not enough: mutating `cycle_boundary_matching(&cycle.tags)`
//! back to the hardcoded `cycle_boundary()` left the whole suite green. A
//! declaration that parses and is then ignored is worse than no declaration —
//! the project believes it configured something.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

/// A real git repo with three tags in a known order. Real git rather than a
/// stub, because what is being asserted is which tag day *selects*, and a stub
/// that returned a fixed answer would assert nothing.
/// A git repo with a commit and **no tags** — so `cycle_boundary` finds none
/// and position takes its cumulative reading. This is the DEFAULT mode (no
/// release means no boundary), and the one a mechanism wired only for the
/// tagged path silently skips.
fn init_repo(dir: &Path) {
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@e")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@e")
            .output()
            .expect("git");
    };
    git(&["init", "-q", "."]);
    std::fs::write(dir.join("Cargo.toml"), "version = \"1.0.0\"\n").unwrap();
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "one"]);
}

fn repo_with_tags(dir: &Path, tags: &[&str]) {
    // Dates are set EXPLICITLY and increasing. day selects a boundary with
    // `--sort=-creatordate`, and a lightweight tag's creatordate is its commit's
    // date — so three commits made in the same second tie, the sort falls back
    // to something else, and the test asserts whatever git felt like. A fixture
    // that depends on wall-clock ordering is flaky by construction, and this one
    // was: it selected the OLDEST pass tag on the first run.
    let git = |args: &[&str], when: &str| {
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@e")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@e")
            .env("GIT_AUTHOR_DATE", when)
            .env("GIT_COMMITTER_DATE", when)
            .output()
            .expect("git");
    };
    let at = |n: i64| format!("{} +0000", 1_700_000_000 + n * 3600);

    git(&["init", "-q", "."], &at(0));
    std::fs::write(dir.join("Cargo.toml"), "version = \"1.0.0\"\n").unwrap();
    std::fs::write(dir.join("README.md"), "# 1.0.0\n").unwrap();
    git(&["add", "-A"], &at(0));
    git(&["commit", "-q", "-m", "one"], &at(0));
    for (i, tag) in tags.iter().enumerate() {
        let when = at(i as i64 + 1);
        std::fs::write(dir.join("README.md"), format!("# 1.0.0\n{i}\n")).unwrap();
        git(&["add", "-A"], &when);
        git(&["commit", "-q", "-m", "step"], &when);
        git(&["tag", tag], &when);
    }
}

fn docs_schema() -> StubClaim {
    claim(
        "schema/docs",
        "bafyd",
        "D.\n\n```day-docs\n{\"version_source\":\"Cargo.toml\",\"version_key\":\"version\",\
         \"version_files\":[\"README.md\"]}\n```\n",
    )
}

fn cycle_claim(tags: &str) -> StubClaim {
    claim(
        "schema/cycle",
        "bafyc",
        &format!("C.\n\n```day-cycle\n{{\"tags\":\"{tags}\"}}\n```\n"),
    )
}

/// day#76, and the assertion a parse test cannot make: **the declared cycle
/// changes which tag day treats as the boundary.**
///
/// The tags are ordered `pass/006 < v1.0.0 < pass/007` on purpose. Release
/// semantics select `v1.0.0`; a `pass/*` cycle selects `pass/007`. Any fixture
/// where both patterns select the same tag would pass whether or not the
/// declaration was honoured.
#[test]
fn a_declared_cycle_changes_which_tag_bounds_the_cycle() {
    // Release semantics, no declaration.
    let dir = tempfile::tempdir().unwrap();
    repo_with_tags(dir.path(), &["pass/006", "v1.0.0", "pass/007"]);
    let kan = write_kan_stub(dir.path(), &[docs_schema()]);
    let undeclared =
        String::from_utf8_lossy(&day(dir.path(), &kan, &["assess", "docs"]).stdout).into_owned();
    assert!(
        undeclared.contains("v1.0.0"),
        "undeclared, the boundary should be the release: {undeclared}"
    );
    assert!(
        !undeclared.contains("pass/007"),
        "and must not already be the pass tag, or this test proves nothing: {undeclared}"
    );

    // Same repo, same tags, one declaration.
    let dir = tempfile::tempdir().unwrap();
    repo_with_tags(dir.path(), &["pass/006", "v1.0.0", "pass/007"]);
    let kan = write_kan_stub(dir.path(), &[docs_schema(), cycle_claim("pass/*")]);
    let declared =
        String::from_utf8_lossy(&day(dir.path(), &kan, &["assess", "docs"]).stdout).into_owned();
    assert!(
        declared.contains("pass/007"),
        "the declared cycle should select the newest pass tag: {declared}"
    );
    assert!(
        !declared.contains("v1.0.0 is tagged"),
        "and should no longer reconcile against the release: {declared}"
    );
}

/// The other half of day#76, which is the half it would be easy to ship without:
/// **position and `assess docs` must agree about what "since" means.**
///
/// A project whose cycles are passes getting pass-relative position and
/// release-relative docs reconciliation would be two answers to one question,
/// disagreeing with no indication why.
///
/// **The fixture is built so the boundary genuinely discriminates**, which took
/// two attempts and both failures are worth recording. First the assertion was
/// `!status.contains("v1.0.0")` — but `day status` never prints a tag, so it
/// passed no matter what the code did, and mutation testing caught it. Then the
/// atom had no declared inputs, which makes it a *source* atom that
/// `Standing::is_source` excludes from "current" by design — so it was never
/// current under either boundary and the two outputs were identical for a reason
/// that had nothing to do with the boundary.
#[test]
fn position_and_assess_docs_use_the_same_declared_boundary() {
    // Commits: … → src/lib.rs @pass/007 → docs/x.md (untagged).
    //   since v1.0.0  : src/lib.rs AND docs/x.md  → code present, design-doc present
    //   since pass/007: docs/x.md only            → code ABSENT, design-doc present
    // So `build` (in: design-doc, out: code) is current under `pass/*` and not
    // under release semantics. Nothing else in the fixture differs.
    let build = |dir: &Path| {
        let git = |args: &[&str], when: &str| {
            Command::new("git")
                .args(args)
                .current_dir(dir)
                .env("GIT_AUTHOR_NAME", "t")
                .env("GIT_AUTHOR_EMAIL", "t@e")
                .env("GIT_COMMITTER_NAME", "t")
                .env("GIT_COMMITTER_EMAIL", "t@e")
                .env("GIT_AUTHOR_DATE", when)
                .env("GIT_COMMITTER_DATE", when)
                .output()
                .expect("git");
        };
        let at = |n: i64| format!("{} +0000", 1_700_000_000 + n * 3600);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("docs")).unwrap();
        std::fs::write(dir.join("Cargo.toml"), "version = \"1.0.0\"\n").unwrap();
        std::fs::write(dir.join("README.md"), "# 1.0.0\n").unwrap();

        git(&["init", "-q", "."], &at(0));
        git(&["add", "-A"], &at(0));
        git(&["commit", "-q", "-m", "base"], &at(0));
        git(&["tag", "pass/006"], &at(0));

        std::fs::write(dir.join("docs/old.md"), "old\n").unwrap();
        git(&["add", "-A"], &at(1));
        git(&["commit", "-q", "-m", "release work"], &at(1));
        git(&["tag", "v1.0.0"], &at(1));

        std::fs::write(dir.join("src/lib.rs"), "fn a() {}\n").unwrap();
        git(&["add", "-A"], &at(2));
        git(&["commit", "-q", "-m", "code"], &at(2));
        git(&["tag", "pass/007"], &at(2));

        std::fs::write(dir.join("docs/x.md"), "new\n").unwrap();
        git(&["add", "-A"], &at(3));
        git(&["commit", "-q", "-m", "docs since the pass"], &at(3));
    };

    let vocabulary = |extra: Vec<StubClaim>| {
        let mut claims = vec![
            docs_schema(),
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"design-doc\":{\"path\":\"docs/*\"},\
                 \"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            claim(
                "atom/build",
                "bafyb",
                "B.\n\n```day-atom\n{\"in\":[\"design-doc\"],\"out\":[\"code\"],\"next\":[]}\n```\n",
            ),
        ];
        claims.extend(extra);
        claims
    };

    let position = |extra: Vec<StubClaim>| -> String {
        let dir = tempfile::tempdir().unwrap();
        build(dir.path());
        let kan = write_kan_stub(dir.path(), &vocabulary(extra));
        String::from_utf8_lossy(&day(dir.path(), &kan, &["status"]).stdout).into_owned()
    };

    let undeclared = position(vec![]);
    let declared = position(vec![cycle_claim("pass/*")]);

    assert_ne!(
        undeclared, declared,
        "declaring a cycle changed nothing about position — the declaration \
         parses and is then ignored, which is worse than not having it"
    );
    assert!(
        declared.contains("build"),
        "under `pass/*` nothing has touched src/ since pass/007, so `build` has \
         not produced this cycle's output and is where the work sits: {declared}"
    );
    assert!(
        !undeclared.contains("Current atom: build"),
        "under release semantics src/lib.rs changed since v1.0.0, so `build` has \
         produced its output and is not current: {undeclared}"
    );

    // And the other surface agrees, which is the point of the test.
    let dir = tempfile::tempdir().unwrap();
    build(dir.path());
    let kan = write_kan_stub(dir.path(), &vocabulary(vec![cycle_claim("pass/*")]));
    let docs =
        String::from_utf8_lossy(&day(dir.path(), &kan, &["assess", "docs"]).stdout).into_owned();
    assert!(
        docs.contains("pass/007"),
        "assess docs must use the same declared cycle position does: {docs}"
    );
}

/// day#77, and again the assertion a parse test cannot make: **a declared
/// vocabulary changes which verdicts `day review record` accepts.**
#[test]
fn a_declared_vocabulary_changes_what_review_record_accepts() {
    let vocab = claim(
        "schema/verdicts",
        "bafyv",
        "V.\n\n```day-verdicts\n{\"verdicts\":[\"NOVEL-AS-SEARCHED\",\"SUBSUMED\"]}\n```\n",
    );

    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[vocab]);

    // The project's own verdict is accepted.
    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "claim-x",
            "--verdict",
            "novel-as-searched",
            "--rationale",
            "r",
            "--cites",
            "bafyabc",
        ],
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // day's own four are now REFUSED, which is the point: a declared vocabulary
    // replaces the closed set rather than extending it. Free text is the thing
    // both vocabularies exist to prevent.
    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "claim-x",
            "--verdict",
            "APPROVE",
            "--rationale",
            "r",
            "--cites",
            "bafyabc",
        ],
    );
    assert_ne!(out.status.code(), Some(0));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("NOVEL-AS-SEARCHED"),
        "the error should name what IS permitted: {err}"
    );

    // Negative control: with no declaration, day's four still work unchanged.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(
        dir.path(),
        &kan,
        &[
            "review",
            "record",
            "claim-x",
            "--verdict",
            "APPROVE",
            "--rationale",
            "r",
            "--cites",
            "bafyabc",
        ],
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "an undeclared project must keep day's four: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// The declared injection cadence has an effect on how often the mid-session
/// channel speaks — the same "does the declaration do anything" question.
#[test]
fn a_declared_cadence_changes_how_often_the_channel_speaks() {
    let base = || {
        vec![
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"code\":{\"path\":\"src/*\"}}\n```\n",
            ),
            claim(
                "atom/future",
                "bafyf",
                &format!("F.\n\n```day-atom\n{}\n```\n", common::too_new_atom_body()),
            ),
        ]
    };

    let fired_over = |claims: Vec<StubClaim>, prompts: usize| -> usize {
        let dir = tempfile::tempdir().unwrap();
        let kan = write_kan_stub(dir.path(), &claims);
        let mut fired = 0;
        for _ in 0..prompts {
            let out = day(dir.path(), &kan, &["hook", "user-prompt"]);
            if !String::from_utf8_lossy(&out.stdout).trim().is_empty() {
                fired += 1;
            }
        }
        fired
    };

    let mut declared = base();
    declared.push(claim(
        "schema/injection",
        "bafyi",
        "I.\n\n```day-injection\n{\"cadence\":2}\n```\n",
    ));

    let with_declaration = fired_over(declared, 6);
    let with_default = fired_over(base(), 6);

    assert!(
        with_declaration > with_default,
        "a declared cadence of 2 should speak more often over 6 prompts than \
         day's default of {}: declared fired {with_declaration}, default fired \
         {with_default}",
        day::cache::DEFAULT_CADENCE
    );
}

/// The declaration for the `block` predicate tests: `research-claim` with two
/// required fields and one optional.
fn blocks_schema() -> StubClaim {
    claim(
        "schema/blocks",
        "bafybs",
        "B.\n\n```day-blocks\n{\"research-claim\":{\"required\":[\"medium\",\"anchor_ref\"],\
         \"optional\":[\"decay_note\"]}}\n```\n",
    )
}

/// A witness schema with the predicate under test **and its negative control**:
/// `loose` is `stationed` with the `block` conjunct removed and nothing else
/// changed, so any difference between the two verdicts is the predicate's doing.
fn station_witnesses() -> StubClaim {
    claim(
        "schema/witness",
        "bafyw",
        "W.\n\n```day-witness\n{\
         \"stationed\":{\"claim\":{\"kind\":\"Observation\",\"subject\":\"claim/*\",\
         \"block\":\"research-claim\"}},\
         \"loose\":{\"claim\":{\"kind\":\"Observation\",\"subject\":\"claim/*\"}}}\n```\n",
    )
}

fn telos_for(slug: &str, cid: &str, witness: &str) -> StubClaim {
    claim(
        &format!("telos/{slug}"),
        cid,
        &format!("T.\n\n```day-telos\n{{\"witnesses\":[\"{witness}\"]}}\n```\n"),
    )
}

fn research_claim(body: &str) -> StubClaim {
    claim(
        "claim/one",
        "bafyc1",
        &format!("A claim.\n\n```research-claim\n{body}\n```\n"),
    )
}

/// AC-8, and the reason REQ-2 is not nominal: a **declared** block is
/// witnessable, so day validates one because something asked, not because a
/// `doctor` pass was added to make the requirement true.
///
/// The two runs differ only in the instance's body. The negative control is the
/// `loose` witness, identical but for the `block` conjunct — it matches both
/// claims, which is what proves the discrimination belongs to the predicate and
/// not to some other difference in the fixture.
#[test]
fn a_declared_block_decides_whether_a_witness_is_material() {
    let verdicts = |body: &str| -> (String, String) {
        let dir = tempfile::tempdir().unwrap();
        let kan = write_kan_stub(
            dir.path(),
            &[
                blocks_schema(),
                station_witnesses(),
                telos_for("stationed", "bafyt1", "stationed"),
                telos_for("loose", "bafyt2", "loose"),
                research_claim(body),
            ],
        );
        let line = |slug: &str| {
            let out = day(dir.path(), &kan, &["assess", "telos", slug]);
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .find(|l| {
                    l.contains("[MATERIAL]") || l.contains("[MISSING]") || l.contains("[ERROR]")
                })
                .unwrap_or("<no verdict>")
                .to_string()
        };
        (line("stationed"), line("loose"))
    };

    let (valid, valid_control) =
        verdicts("{\"medium\":\"anchor-verified\",\"anchor_ref\":\"lean:Thm1\"}");
    let (violating, violating_control) =
        verdicts("{\"medium\":\"anchor-verified\",\"anchor_ref\":\"lean:Thm1\",\"undeclared\":1}");

    assert!(
        valid.contains("[MATERIAL]"),
        "a claim carrying a valid instance should satisfy the witness: {valid}"
    );
    assert!(
        violating.contains("[MISSING]"),
        "an instance violating the declaration should not satisfy it: {violating}"
    );
    // The control: without the `block` conjunct, both claims match. If this
    // failed, the test above would be measuring the fixture rather than the
    // predicate.
    assert!(
        valid_control.contains("[MATERIAL]") && violating_control.contains("[MATERIAL]"),
        "the same probe without `block` should match both claims, or the \
         discrimination above is not the predicate's: valid={valid_control}, \
         violating={violating_control}"
    );
}

/// The other half of AC-8, and `v0.7.0-beta.2`'s contract applied to the new
/// path: **day cannot check what it could not read**, and says so rather than
/// reporting the absence it would otherwise infer. Both cases would render as
/// `MISSING` under a lazier mapping, and both would be a lie — one about a
/// project that never declared the type, one about a claim from a newer day.
#[test]
fn a_block_day_could_not_check_is_an_error_not_an_absence() {
    let verdict = |claims: &[StubClaim], slug: &str| -> String {
        let dir = tempfile::tempdir().unwrap();
        let kan = write_kan_stub(dir.path(), claims);
        let out = day(dir.path(), &kan, &["assess", "telos", slug]);
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .find(|l| l.contains("[MATERIAL]") || l.contains("[MISSING]") || l.contains("[ERROR]"))
            .unwrap_or("<no verdict>")
            .to_string()
    };

    // A witness naming a block type this project never declared.
    let undeclared = verdict(
        &[
            blocks_schema(),
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"ghost\":{\"claim\":{\"kind\":\"Observation\",\
                 \"block\":\"never-declared\"}}}\n```\n",
            ),
            telos_for("ghosted", "bafyt", "ghost"),
        ],
        "ghosted",
    );
    assert!(
        undeclared.contains("[ERROR]") && undeclared.contains("never-declared"),
        "a witness naming an undeclared block type should error and name it: {undeclared}"
    );

    // An instance from a newer day. day cannot tell whether it would have
    // matched, so neither MATERIAL nor MISSING is honest.
    let skewed = verdict(
        &[
            blocks_schema(),
            station_witnesses(),
            telos_for("stationed", "bafyt1", "stationed"),
            research_claim("{\"_version\":2,\"medium\":\"a\",\"anchor_ref\":\"r\"}"),
        ],
        "stationed",
    );
    assert!(
        skewed.contains("[ERROR]") && skewed.contains("upgrade day"),
        "a version-skewed instance should be unchecked, not absent: {skewed}"
    );
}

/// day#77 ask #2, and the adversarial review's F2: four `starter_command()`
/// methods existed and **nothing called them**, so a project adopting day got
/// no hint that `schema/blocks`, `schema/verdicts`, `schema/cycle`, or
/// `schema/injection` exist at all.
///
/// The answer is *offer*, not *record* — each has a working default, and
/// `schema/blocks`'s starter carries the research loop's `research-claim`,
/// which is an example of the shape rather than a vocabulary another project
/// wants. So `day init` prints them and writes nothing, which the second
/// assertion pins.
#[test]
fn init_offers_every_declaration_a_project_can_make() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(dir.path(), &kan, &["init", "--print"]);
    let text = String::from_utf8_lossy(&out.stdout);

    for subject in [
        "schema/blocks",
        "schema/verdicts",
        "schema/cycle",
        "schema/injection",
    ] {
        assert!(
            text.contains(subject),
            "`day init` should offer {subject}, or a project cannot discover it: {text}"
        );
    }

    // The fences, not just the subject names — a starter a user cannot run is
    // not an offer.
    for fence in [
        day::blocks::FENCE_INFO,
        day::blocks::VERDICTS_FENCE,
        day::blocks::CYCLE_FENCE,
        day::blocks::INJECTION_FENCE,
    ] {
        assert!(
            text.contains(fence),
            "`day init` should print a runnable starter carrying {fence}: {text}"
        );
    }

    // Each default is stated, because "optional" without the fallback tells a
    // reader nothing about what declining costs.
    assert!(
        text.contains(day::blocks::DEFAULT_BOUNDARY_TAGS)
            && text.contains(&day::cache::DEFAULT_CADENCE.to_string()),
        "`day init` should state what each default is: {text}"
    );
}

/// `.design/declared-blocks.md` AC-4, for an **instance** rather than a
/// declaration — the half the adversarial review found unmet.
///
/// `claims_matching` refuses to answer a witness from an instance it could not
/// check, returning `Verdict::Error`. Position inference then reduced that to
/// `Presence::Unknown`, which is honest about the presence and silent about the
/// reason: a project whose `research-claim` day cannot read was told only that
/// its position was unknowable. `telos/honest-reads` is precisely that no report
/// day makes asserts a completeness it did not verify.
///
/// **Run in both boundary modes.** The first fix wired only the
/// `Some(boundary)` path, so it did nothing on a repo with no `v*` tag — which
/// is every fresh clone, and was every fixture here. `CLAUDE.md` records that
/// exact failure for the position fingerprint; this is the same trap one
/// mechanism over, so the mode is a parameter of the test rather than whatever
/// the fixture happened to be.
#[test]
fn an_instance_day_cannot_check_is_reported_on_both_channels() {
    let fixture = || {
        vec![
            claim(
                "schema/blocks",
                "bafybs",
                "B.\n\n```day-blocks\n{\"research-claim\":{\"required\":[\"medium\"]}}\n```\n",
            ),
            claim(
                "schema/witness",
                "bafyw",
                "W.\n\n```day-witness\n{\"station\":{\"claim\":{\"kind\":\"Observation\",\
                 \"subject\":\"claim/*\",\"block\":\"research-claim\"}}}\n```\n",
            ),
            claim(
                "atom/collect",
                "bafya",
                "C.\n\n```day-atom\n{\"in\":[],\"out\":[\"station\"]}\n```\n",
            ),
        ]
    };
    // The instance this day cannot check.
    let skewed = claim(
        "claim/future",
        "bafyf",
        "F.\n\n```research-claim\n{\"_version\":2,\"medium\":\"a\"}\n```\n",
    );
    // The control: same shape, a version this day reads.
    let readable = claim(
        "claim/fine",
        "bafyok",
        "F.\n\n```research-claim\n{\"medium\":\"a\"}\n```\n",
    );

    for tagged in [false, true] {
        let mode = if tagged {
            "with a release tag"
        } else {
            "no release tag"
        };

        let run = |extra: StubClaim| -> (String, String) {
            let dir = tempfile::tempdir().unwrap();
            if tagged {
                repo_with_tags(dir.path(), &["v1.0.0"]);
            } else {
                init_repo(dir.path());
            }
            let mut claims = fixture();
            claims.push(extra);
            let kan = write_kan_stub(dir.path(), &claims);
            let notice = day(dir.path(), &kan, &["hook", "session-notice"]);
            let start = day(dir.path(), &kan, &["hook", "session-start"]);
            (
                String::from_utf8_lossy(&notice.stdout).into_owned(),
                String::from_utf8_lossy(&start.stdout).into_owned(),
            )
        };

        let (notice, start) = run(skewed.clone());
        assert!(
            notice.contains("partial"),
            "{mode}: session-notice should report an unreadable instance: {notice}"
        );
        assert!(
            start.contains("partial") && start.contains("research-claim"),
            "{mode}: session-start should mark its context partial and name the \
             block: {start}"
        );
        assert!(
            start.contains("upgrade day"),
            "{mode}: a version-skewed instance asks the reader to upgrade day, \
             not to fix the claim: {start}"
        );

        // Negative control: with a readable instance, both channels are silent.
        // Without this the assertions above would pass against a warning that
        // always fires.
        let (notice, start) = run(readable.clone());
        assert!(
            notice.trim().is_empty(),
            "{mode}: a readable instance should produce no notice: {notice}"
        );
        assert!(
            !start.contains("partial"),
            "{mode}: a readable instance should not mark the context partial: {start}"
        );
    }
}
