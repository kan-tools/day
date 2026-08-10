//! day#27 — does the argument shape day builds actually parse against the
//! real `kan` binary?
//!
//! Every other integration test in this repo stubs kan through
//! `DAY_KAN_BIN`, which is right for hermetic CI and useless for this
//! question: a stub accepts whatever day sends it, so those tests validate
//! day against day's own idea of kan's CLI rather than against kan's
//! contract. That blind spot let `docs/CONVENTIONS.md` document
//! `kan result "<text>" --subject <s>` — a command that does not run —
//! through several releases, and very nearly shipped it as day's own
//! printed output.
//!
//! Two different guarantees live here:
//!
//! - [`append_is_only_used_with_verbs_whose_subject_is_a_flag`] is
//!   **hermetic** and always runs. It is the one that actually protects the
//!   invariant, by construction rather than by observation.
//! - The `conformance_*` tests run the real binary and **skip when kan is
//!   not installed**, because `CLAUDE.md` requires that no test need a real
//!   kan. They catch drift in kan's surface that day's own source cannot
//!   reveal.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;

use day::kan_client::{KanClient, Write};

/// The real `kan`, or `None` when it is not installed.
///
/// Deliberately ignores `DAY_KAN_BIN`: that variable exists to point day at
/// a *stub*, and a stub is exactly what these tests must not talk to.
fn real_kan() -> Option<&'static str> {
    Command::new("kan")
        .arg("--help")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| "kan")
}

/// kan anchors claims to the HEAD commit, so it needs a git repository —
/// hence a scratch one per test. Scratch rather than this repo because a
/// probe against the real log leaves real claims, which `CLAUDE.md` names
/// as its own kind of defect.
fn scratch_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let git = |args: &[&str]| {
        let ok = Command::new("git")
            .args(args)
            .current_dir(dir.path())
            .output()
            .expect("git should be available")
            .status
            .success();
        assert!(ok, "git {args:?} failed while building the scratch repo");
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "conformance@example.invalid"]);
    git(&["config", "user.name", "conformance"]);
    git(&["commit", "-q", "--allow-empty", "-m", "scratch"]);
    dir
}

fn sources() -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
        for entry in std::fs::read_dir(dir)
            .expect("src should be readable")
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).expect("source should be readable");
                out.push((path, text));
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
    out
}

/// The guarantee that holds without kan installed, and the one that would
/// actually have prevented day#27's bug.
///
/// `KanClient::append` builds `<verb> <text> --subject <s>`. That shape is
/// correct for exactly three of kan's verbs. `result` takes
/// `<SUBJECT> <TEXT>` positionally, and `relate`/`same` take their subjects
/// positionally too — so routing any of them through `append` would emit a
/// command kan rejects, and no stub-based test could tell.
///
/// If day grows a verb that needs one of those, it needs its own method
/// with its own argument order; adding it to this list instead is the
/// mistake this test exists to stop.
#[test]
fn append_is_only_used_with_verbs_whose_subject_is_a_flag() {
    const SUBJECT_AS_FLAG: [&str; 3] = ["observe", "plan", "decide"];

    let mut found = 0;
    for (path, text) in sources() {
        let mut rest = text.as_str();
        while let Some(at) = rest.find("Write::new(\"") {
            let after = &rest[at + "Write::new(\"".len()..];
            let verb: String = after.chars().take_while(|c| *c != '"').collect();
            assert!(
                SUBJECT_AS_FLAG.contains(&verb.as_str()),
                "{} passes `{verb}` to Write::new, but append builds \
                 `<verb> <text> --subject <s>` and only {SUBJECT_AS_FLAG:?} take their \
                 subject as a flag. `kan {verb}` takes it positionally, so this would \
                 emit a command kan rejects — and every stub-based test would still pass.",
                path.display()
            );
            found += 1;
            rest = after;
        }
    }
    assert!(found > 0, "the scan should have found Write::new calls");
}

/// Runs every argument shape `append` can build against the real binary.
///
/// Exercises `KanClient` itself rather than a transcription of what it
/// emits — a hand-written list of arguments would be a second copy of the
/// thing under test, and could agree with the docs while the code disagreed.
#[test]
fn conformance_append_shapes_are_accepted_by_real_kan() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();
    let client = KanClient::with_bin(dir.path(), bin);

    // Bare: `<verb> <text> --subject <s>`.
    let mut previous = None;
    for verb in ["observe", "plan", "decide"] {
        let cid = client
            .append(Write::new(verb, "conformance", &format!("{verb} shape")))
            .unwrap_or_else(|e| panic!("real kan rejected day's `{verb}` shape: {e}"));
        assert!(cid.starts_with("bafy"), "expected a CID, got {cid:?}");
        previous = Some(cid);
    }

    // With `--cites`, chained from a CID day captured itself.
    let cites = [previous.expect("a prior claim")];
    client
        .append(Write::new("observe", "conformance", "cites shape").cites(&cites))
        .expect("real kan rejected day's --cites shape");

    // With `--title`/`--kind`, which kan accepts only together.
    client
        .append(
            Write::new("decide", "conformance-declared", "declaring shape")
                .declaring("Conformance", "idea"),
        )
        .expect("real kan rejected day's --title/--kind shape");

    // And the reads day depends on.
    assert!(
        client.show("conformance").is_ok_and(|c| !c.is_empty()),
        "real kan returned no claims for a subject day just wrote to"
    );
    client.subjects().expect("real kan rejected `kan status`");
    client.issues().expect("real kan rejected `kan issues`");
}

/// The half day#27 left uncovered, and the reason kan's format change reached
/// production silently.
///
/// That test asserted the argument shapes day *emits* are accepted. It said
/// nothing about whether day can *parse* what comes back — so when kan
/// changed its rendered output, day read a full log as empty and reported
/// "the process vocabulary is empty, which is a valid starting state, not an
/// error" against seven declared atoms, at exit 0. Half an interface was
/// covered and the half that broke was the other one.
///
/// This asserts the round trip: write through day, read back through day,
/// against a real kan.
#[test]
fn conformance_day_reads_back_what_it_wrote_through_a_real_kan() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();
    let client = KanClient::with_bin(dir.path(), bin);

    let cid = client
        .append(
            Write::new("decide", "telos/roundtrip", "A telos statement.")
                .declaring("Round trip", "idea"),
        )
        .expect("append should succeed");

    let claims = client.show("telos/roundtrip").expect("show should parse");
    assert!(
        !claims.is_empty(),
        "day wrote a claim through a real kan and then read the subject as empty —          exactly the failure this test exists to catch"
    );

    // Every field day actually consumes, verified against the real binary
    // rather than against the stub's idea of it.
    assert!(
        claims.iter().any(|c| c.cid == cid),
        "the CID day captured on write did not come back on read: {claims:?}"
    );
    assert!(
        claims
            .iter()
            .any(|c| c.text.as_deref() == Some("A telos statement.")),
        "claim text did not survive the round trip: {claims:?}"
    );
    assert!(
        claims
            .iter()
            .any(|c| c.title.as_deref() == Some("Round trip")),
        "a declared title did not come back — day renders teloi by title: {claims:?}"
    );
    assert!(
        claims.iter().all(|c| c.author.is_some()),
        "author is absent; day#25's locally-signed scoping depends on it: {claims:?}"
    );

    // And the subject is discoverable, which is how doctor finds atoms at all.
    let subjects = client.subjects().expect("status should parse");
    assert!(
        subjects.iter().any(|s| s == "telos/roundtrip"),
        "a subject day just wrote is not in `kan status`: {subjects:?}"
    );
}

/// `KanClient::relate` has its own argument order — two positional subjects
/// and a kind, no text — so `append`'s shape says nothing about whether it
/// is right. This is the case flagged when day#27 landed as "the next place
/// this could rot", added with the verb that needed it.
#[test]
fn conformance_relate_shape_is_accepted_by_real_kan() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();
    let client = KanClient::with_bin(dir.path(), bin);

    let a = client
        .append(Write::new("decide", "telos/a", "A."))
        .expect("seeding telos/a");
    client
        .append(Write::new("decide", "telos/b", "B."))
        .expect("seeding telos/b");

    let edge = client
        .relate("telos/a", "in-tension-with", "telos/b", &[a])
        .expect("real kan rejected day's `relate` shape");
    assert!(edge.starts_with("bafy"), "expected a CID, got {edge:?}");

    // The asymmetry day's two-edge behaviour compensates for: a relation is
    // stored on its source, and the target's own claims do not contain it.
    //
    // THIS ASSERTION WAS ORIGINALLY WORDED WRONG, and the mistake is worth
    // keeping visible. It said "kan now surfaces relations from the target
    // ... that should be revisited" — encoding the question *does kan
    // surface this from the target?*, which is not the question that decides
    // anything. kan answered yes within a day, via a `--json` `inbound`
    // field, and the two-edge design still stood: `inbound` returns a
    // RENDERED STRING (`"telos/a InTensionWith this"`, with the relation kind
    // Debug-formatted into prose and no cid or author), while an outbound
    // relation returns a structured claim. Consuming it would put day back to
    // parsing prose — the coupling that broke it once already (kan#103).
    //
    // So the question that decides is *is the target-side representation
    // STRUCTURED?*, not *does it exist?*. A tripwire written against the
    // shape you know rather than the question you mean will answer the wrong
    // question confidently.
    let from_source = client.show("telos/a").expect("show source");
    let from_target = client.show("telos/b").expect("show target");
    assert!(
        from_source.iter().any(|c| c.kind == "Relation"),
        "the source subject should carry the edge"
    );
    assert!(
        !from_target.iter().any(|c| c.kind == "Relation"),
        "a target subject's own claims now include a relation asserted by another \
         subject. If day can read target-side relations as STRUCTURED claims — not as \
         kan's rendered `inbound` strings — then the second edge `day telos tension` \
         writes is redundant and the design should be revisited. If this fails because \
         `inbound` merely exists, that is not sufficient: see kan#103."
    );
}

/// `docs/CONVENTIONS.md` tells a reader to record an assessment with
/// `kan result`, and `.design/assess-telos.md` REQ-12 has `day assess telos`
/// *printing* that command. A documented command that does not run is worse
/// than no documentation, so the documented form is checked directly.
///
/// This is the exact shape that was wrong: the page said
/// `kan result "<text>" --subject <s>`, pattern-matched from the three verbs
/// above, and nothing executed it. See kan-tools/kan#78 for the asymmetry.
#[test]
fn conformance_the_documented_kan_result_form_runs() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();

    let seed = Command::new(bin)
        .args(["observe", "seed", "--subject", "telos/conformance"])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(seed.status.success(), "seeding the subject failed");
    let cid = String::from_utf8_lossy(&seed.stdout).trim().to_string();

    // Subject first, positionally — NOT `--subject`.
    let out = Command::new(bin)
        .args([
            "result",
            "telos/conformance",
            "the assessment text",
            "--cites",
            &cid,
        ])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(
        out.status.success(),
        "the `kan result` form documented in docs/CONVENTIONS.md was rejected: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// kan#78 is **resolved**: kan accepts `--subject` on `result` too.
///
/// **Split out of [`conformance_the_documented_kan_result_form_runs`], and the
/// reason is worth stating** — folding it in there made day's compatibility
/// floor wrong by five releases. This asserts a property of *kan*, not a
/// dependency of *day*: day emits only the positional form (`Write::new` is
/// structurally unavailable for `result` — see the hermetic test below), so a
/// kan without kan#78 serves day perfectly well. Measured as one suite, every
/// kan through v0.7.0 looked `incompatible` when what they actually lacked was
/// a convenience day does not use.
///
/// `scripts/run-kan-compat-cell.sh` therefore excludes this test: the cell's
/// question is "does day work against this kan", and a kan characterization
/// cannot be allowed to answer it. It still runs in normal CI against the
/// pinned kan, where it does its real job — telling us if kan ever revokes the
/// spelling `docs/CONVENTIONS.md` mentions.
#[test]
fn conformance_kan_78_result_accepts_both_spellings() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();

    let seed = Command::new(bin)
        .args(["observe", "seed", "--subject", "telos/conformance"])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(seed.status.success(), "seeding the subject failed");

    let both = Command::new(bin)
        .args([
            "result",
            "the assessment text",
            "--subject",
            "telos/conformance",
        ])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    assert!(
        both.status.success(),
        "kan#78 was resolved by accepting both spellings; if `result --subject` \
         is rejected again, docs/CONVENTIONS.md and this test need revisiting: {}",
        String::from_utf8_lossy(&both.stderr)
    );
}

/// day#71 — the bulk read `ClaimLog` now depends on (kan#123 / ADR-71).
///
/// **This test is what makes `tests/fixtures/kan-compat.tsv` tell the truth.**
/// The compat matrix runs this file against every released kan, and until this
/// existed nothing here touched `--all` — so the table would have kept
/// reporting v0.8.0 as `ok` while day was in fact broken against it. A table
/// claiming a pairing works for a kan day cannot use is the false completeness
/// `telos/honest-reads` forbids, in the artifact built to prevent it.
///
/// Asserted through `KanClient::show_all` rather than by shelling out directly,
/// so what is checked is the call day actually makes.
#[test]
fn conformance_bulk_read_is_available_and_agrees_with_per_subject_reads() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();
    let client = KanClient::with_bin(dir.path(), bin);

    // Two subjects with two claims each, so a bulk read that dropped a claim
    // or mislabelled a subject cannot pass by coincidence.
    for (subject, texts) in [
        ("telos/bulk-a", ["first on a", "second on a"]),
        ("telos/bulk-b", ["first on b", "second on b"]),
    ] {
        for text in texts {
            client
                .append(Write::new("observe", subject, text))
                .expect("append should succeed against a real kan");
        }
    }

    let bulk = client
        .show_all()
        .expect("kan >= 0.9.0 serves `show --all --json`; day#71 depends on it");

    // AC-2: CID for CID against what the per-subject path returns. A faster
    // path that returns a different answer is a different answer wearing the
    // same name, and `ClaimLog` would inherit the difference silently.
    for subject in ["telos/bulk-a", "telos/bulk-b"] {
        let mut per_subject: Vec<String> = client
            .show(subject)
            .expect("show should succeed")
            .into_iter()
            .map(|c| c.cid)
            .collect();
        let mut from_bulk: Vec<String> = bulk
            .iter()
            .filter(|(s, _)| s == subject)
            .map(|(_, c)| c.cid.clone())
            .collect();
        per_subject.sort();
        from_bulk.sort();
        assert_eq!(
            from_bulk, per_subject,
            "`show --all` and `show {subject}` must return the same claims"
        );
        assert!(
            !from_bulk.is_empty(),
            "the fixture should have produced claims on {subject}"
        );
    }
}

/// day#99, found while fixing it: `/adversarial-review`'s Context block greps
/// `kan status` for telos subjects, and its pattern was `^\[Local("telos/` —
/// a shape kan's status output no longer has. It matched nothing, the line's
/// `|| echo "none"` fired, and the command told every reviewer this repo had
/// **no teloi** while eleven telos subjects sat in the log.
///
/// Step 1 of that skill calls the teloi "the north star" and falls back to
/// orientation docs when there are none, so the effect was not cosmetic: every
/// review run here silently measured against `CLAUDE.md` instead of against the
/// teloi, and reported nothing unusual because the fallback is a legitimate
/// branch.
///
/// This is day depending on the *format* of kan's output, which no stub can
/// check — a stub prints whatever day expects. Hence here, with the real binary,
/// skipping when kan is absent.
///
/// **Re-pointed by the Agent Plugins conversion, not retired.** The skill no
/// longer greps `kan status`; REQ-4 removed the pre-executed line, and the body
/// now instructs `kan show --all --json` filtered to subjects beginning
/// `telos/` (kan#181: the per-subject verb is O(n²) in commit-anchored claims).
/// The *dependency* is unchanged — day still relies on kan emitting telos
/// subjects it can find by prefix — so the cell follows the mechanism rather
/// than being deleted with the string it used to match. Deleting it would have
/// been the easy read of a green suite: the defect it guards is a filter that
/// silently stops matching, and that is exactly as available in JSON as in text.
///
/// Scoped deliberately to day's own requirement, per `CLAUDE.md`: it asserts
/// that the read day ships finds a telos day just wrote. It says nothing about
/// what kan promises about `--all --json`'s schema, which is kan's fact to state
/// and would belong in a test named for it.
#[test]
fn conformance_the_review_skills_telos_read_matches_real_kan_output() {
    let Some(kan) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let repo = scratch_repo();

    let declared = Command::new(kan)
        .args([
            "decide",
            "telos/conformance-probe",
            "A telos written so the review command's grep has something to find.",
        ])
        .current_dir(repo.path())
        .output()
        .expect("kan decide should run");
    assert!(
        declared.status.success(),
        "could not write a telos to the scratch log: {}",
        String::from_utf8_lossy(&declared.stderr)
    );

    // The read the shipped skill actually instructs. Extracted from the file
    // rather than retyped, so the test cannot pass against a read the skill
    // does not ship — the exact gap that let the stale pattern survive.
    let body = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("skills/adversarial-review/SKILL.md"),
    )
    .expect("the review skill must be readable");
    let context = body
        .split_once("## Context")
        .expect("the review skill must still carry a Context section")
        .1
        .split("\n## ")
        .next()
        .unwrap();
    assert!(
        context.contains("kan show --all --json"),
        "the review skill's Context no longer instructs the bulk read. If it \
         moved to another verb, re-point this cell at that verb — do not delete \
         it, because the failure it guards (a telos filter that silently stops \
         matching) survives any change of mechanism."
    );
    assert!(
        context.contains("telos/"),
        "the review skill's Context no longer names the `telos/` prefix it \
         filters on"
    );

    let shown = Command::new(kan)
        .args(["show", "--all", "--json"])
        .current_dir(repo.path())
        .output()
        .expect("kan show --all --json should run");
    assert!(
        shown.status.success(),
        "kan show --all --json failed: {}",
        String::from_utf8_lossy(&shown.stderr)
    );
    let text = String::from_utf8_lossy(&shown.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&text).expect("kan show --all --json must emit valid JSON");

    // The filter the skill describes, applied the way a reading agent would.
    let matched: Vec<String> = parsed["subjects"]
        .as_array()
        .expect("`subjects` must be an array — day reads the log through it")
        .iter()
        .filter_map(|s| s["subject"].as_str())
        .filter(|s| s.starts_with("telos/"))
        .map(|s| s.to_string())
        .collect();

    assert!(
        !matched.is_empty(),
        "the review skill's telos filter matched NOTHING against real \
         `kan show --all --json` output, so `/adversarial-review` would report \
         `none` for a repo that has teloi — which is day#99's second defect \
         recurring through a new mechanism.\n\
         kan emitted:\n{text}"
    );
    assert!(
        matched.iter().any(|s| s == "telos/conformance-probe"),
        "the filter matched {} subject(s) but not the telos just written; \
         matched: {matched:?}",
        matched.len()
    );
}

/// **day#120 — the two shapes kan emits when a trust base withholds claims.**
///
/// This cell exists because the first attempt at day#120 keyed on a **third**
/// shape: an entry with `claims: []` and a non-zero `excluded_by_trust`. kan
/// does not emit that, so the fix was inert against the real binary while a
/// hand-written stub in `tests/bulk_read.rs` reported it working — verbatim the
/// blind spot this file's own module docs describe, in the milestone that
/// quoted them.
///
/// Two shapes, measured rather than assumed:
///
/// - **fully withheld** — the subject is omitted from `show --all --json`
///   entirely, and the count appears only at the **envelope** level;
/// - **partial** — the entry is present with the admitted claims, and the count
///   appears on the **entry**.
///
/// Both are asserted as *properties of kan*, which is what makes this a
/// conformance cell rather than a test of day: if kan ever starts emitting the
/// shape day originally guessed at, this fails and says so.
#[test]
fn conformance_trust_withholding_shapes_are_what_day_keys_on() {
    let Some(bin) = real_kan() else {
        eprintln!("skipping: kan is not installed (this test is advisory, per CLAUDE.md)");
        return;
    };
    let dir = scratch_repo();

    let kan = |args: &[&str], identity: Option<&Path>| -> String {
        let mut cmd = Command::new(bin);
        cmd.args(args).current_dir(dir.path());
        if let Some(id) = identity {
            cmd.env("KAN_IDENTITY_FILE", id);
        }
        let out = cmd.output().expect("kan should run");
        assert!(
            out.status.success(),
            "kan {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).to_string()
    };

    // A claim by the workspace's primary identity, and a second identity that
    // does not admit it under `--trust me`.
    kan(&["observe", "primary claim", "--subject", "shared"], None);
    let role_added = Command::new(bin)
        .args(["identity", "role", "add", "conformance-role"])
        .current_dir(dir.path())
        .output()
        .expect("kan should run");
    if !role_added.status.success() {
        eprintln!("skipping: this kan has no `identity role add`");
        return;
    }
    let role = dir.path().join(".kan/roles.d/conformance-role");
    assert!(role.exists(), "the role key should have been written");

    // --- shape 1: fully withheld -------------------------------------------
    let payload = kan(
        &["show", "--all", "--json", "--trust", "me"],
        Some(role.as_path()),
    );
    let v: serde_json::Value = serde_json::from_str(&payload).expect("kan emits JSON");

    // premise: something really was withheld, or the assertions below are
    // vacuous and would pass against a kan that withholds nothing at all.
    assert!(
        v["excluded_by_trust"].as_u64().unwrap_or(0) > 0,
        "premise: the fixture must actually withhold a claim, got {payload}"
    );
    assert_eq!(
        v["subjects"].as_array().map(Vec::len),
        Some(0),
        "kan OMITS a fully-withheld subject rather than returning it empty. If \
         this ever fails, kan has started emitting the shape day#120's first \
         attempt guessed at, and `read_all`'s per-entry branch becomes \
         reachable: {payload}"
    );

    // --- shape 2: partial ---------------------------------------------------
    kan(
        &["observe", "role claim", "--subject", "shared"],
        Some(role.as_path()),
    );
    let payload = kan(&["show", "--all", "--json", "--trust", "me"], None);
    let v: serde_json::Value = serde_json::from_str(&payload).expect("kan emits JSON");
    let entry = v["subjects"]
        .as_array()
        .and_then(|s| s.iter().find(|e| e["subject"] == "shared"))
        .unwrap_or_else(|| panic!("a partial view still returns the entry: {payload}"));

    assert!(
        !entry["claims"].as_array().expect("claims array").is_empty(),
        "premise: the partial view must still show the admitted claim, or this \
         is the fully-withheld shape again: {payload}"
    );
    assert!(
        entry["excluded_by_trust"].as_u64().unwrap_or(0) > 0,
        "a partial view carries the count ON THE ENTRY — this is what day keys \
         `PartiallyWithheld` on, and a withheld NEWER claim is what makes it \
         matter, since day resolves newest-wins: {payload}"
    );
}
