//! `.design/bridging.md` AC-1..AC-11 — declaring teloi with witnesses,
//! declaring bridges, and checking realizability, through the real binary
//! against a stub kan.

#![cfg(unix)]

mod common;

use std::path::Path;
use std::process::Command;

use common::{appends, atom_claim, claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .output()
        .expect("failed to run day")
}

/// A `telos/<slug>` claim carrying declared witnesses.
fn telos_claim(slug: &str, cid: &str, witnesses: &[&str]) -> StubClaim {
    let list = witnesses
        .iter()
        .map(|w| format!("\"{w}\""))
        .collect::<Vec<_>>()
        .join(", ");
    claim(
        &format!("telos/{slug}"),
        cid,
        &format!("A telos.\n\n```day-telos\n{{\"witnesses\": [{list}]}}\n```\n"),
    )
}

/// A `bridge/<slug>` claim carrying a plan.
fn bridge_claim(slug: &str, cid: &str, telos: &str, have: &[&str], plan_json: &str) -> StubClaim {
    let have_list = have
        .iter()
        .map(|h| format!("\"{h}\""))
        .collect::<Vec<_>>()
        .join(", ");
    claim(
        &format!("bridge/{slug}"),
        cid,
        &format!(
            "A bridge.\n\n```day-bridge\n{{\"telos\": \"{telos}\", \"have\": [{have_list}], \
             \"plan\": {plan_json}}}\n```\n"
        ),
    )
}

/// design -> build -> review, the shape day's own pipeline uses.
fn pipeline() -> Vec<StubClaim> {
    vec![
        atom_claim("design", "bafyreid1", &["intent"], &["design-doc"], &[]),
        atom_claim("build", "bafyreid2", &["design-doc"], &["code-change"], &[]),
        atom_claim(
            "review",
            "bafyreid3",
            &["design-doc", "code-change"],
            &["verdict"],
            &[],
        ),
    ]
}

#[test]
fn ac1_a_telos_records_its_declared_witnesses() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "shipped",
            "It is published.",
            "--witness",
            "published-artifact",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = appends(dir.path());
    let witnesses =
        day::atoms::extract_fenced::<day::bridge::Witnesses>(&log[0], day::bridge::TELOS_FENCE)
            .expect("a day-telos block should be written")
            .expect("it should parse");
    assert_eq!(witnesses.witnesses, vec!["published-artifact"]);

    // A telos without witnesses still records, and carries no block.
    let kan = write_kan_stub(dir.path(), &[]);
    let out = day(
        dir.path(),
        &kan,
        &["telos", "declare", "plain", "No witnesses here."],
    );
    assert!(out.status.success());
    let log = appends(dir.path());
    assert!(
        !log[0].contains("day-telos"),
        "a witness-less telos should stay a plain statement: {}",
        log[0]
    );
}

#[test]
fn ac2_and_ac7_a_bridge_records_its_plan() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &pipeline());

    let out = day(
        dir.path(),
        &kan,
        &[
            "bridge",
            "declare",
            "b",
            "--telos",
            "shipped",
            "--have",
            "intent",
            "--plan",
            "design > build",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let log = appends(dir.path());
    let plan = day::atoms::extract_fenced::<day::bridge::Plan>(&log[0], day::bridge::FENCE_INFO)
        .expect("a day-bridge block should be written")
        .expect("it should parse");
    assert_eq!(plan.telos, "shipped");
    assert_eq!(plan.have, vec!["intent"]);
    assert_eq!(
        plan.plan,
        day::bridge::parse("design > build").unwrap(),
        "the recorded plan should be the parsed grammar"
    );
}

#[test]
fn ac3_a_plan_naming_an_undeclared_atom_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &pipeline());

    let out = day(
        dir.path(),
        &kan,
        &[
            "bridge",
            "declare",
            "b",
            "--telos",
            "shipped",
            "--plan",
            "design > nonexistent",
        ],
    );
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("nonexistent"), "{stderr}");
    assert!(
        appends(dir.path()).is_empty(),
        "nothing should be recorded for an unresolvable plan"
    );
}

#[test]
fn ac3_malformed_plan_syntax_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &pipeline());
    let out = day(
        dir.path(),
        &kan,
        &[
            "bridge", "declare", "b", "--telos", "shipped", "--plan", "design >",
        ],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("plan syntax"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn ac4_and_ac6_a_reaching_plan_passes_and_artifacts_survive_intermediate_steps() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(telos_claim("shipped", "bafyreitelos", &["verdict"]));
    // review needs design-doc, which `build` in the middle does not re-emit.
    claims.push(bridge_claim(
        "b",
        "bafyreibridge",
        "shipped",
        &["intent"],
        r#"{"seq": [{"atom": "design"}, {"atom": "build"}, {"atom": "review"}]}"#,
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(stdout.contains("reaches telos/shipped"), "{stdout}");
    assert!(stdout.contains("verdict"), "{stdout}");
}

#[test]
fn ac4_an_unreachable_plan_names_the_uncovered_witness() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(telos_claim(
        "shipped",
        "bafyreitelos",
        &["published-artifact"],
    ));
    claims.push(bridge_claim(
        "b",
        "bafyreibridge",
        "shipped",
        &["intent"],
        r#"{"seq": [{"atom": "design"}, {"atom": "build"}]}"#,
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("does not reach"), "{stdout}");
    assert!(stdout.contains("published-artifact"), "{stdout}");
}

#[test]
fn ac5_an_atom_placed_where_its_inputs_are_missing_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(telos_claim("shipped", "bafyreitelos", &["verdict"]));
    // No `have`, so design's `intent` is unavailable.
    claims.push(bridge_claim(
        "b",
        "bafyreibridge",
        "shipped",
        &[],
        r#"{"atom": "design"}"#,
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success());
    assert!(stdout.contains("atom/design needs [intent]"), "{stdout}");
}

#[test]
fn ac7_an_alternative_only_offers_what_every_branch_produces() {
    let dir = tempfile::tempdir().unwrap();
    let base = vec![
        atom_claim("fast", "bafyreif1", &["intent"], &["code-change"], &[]),
        atom_claim(
            "careful",
            "bafyreif2",
            &["intent"],
            &["code-change", "proof"],
            &[],
        ),
        atom_claim("certify", "bafyreif3", &["proof"], &["certificate"], &[]),
        atom_claim("ship", "bafyreif4", &["code-change"], &["released"], &[]),
        telos_claim("done", "bafyreit", &["released"]),
    ];

    // Both branches produce code-change, so `ship` is satisfied.
    let mut ok = base.clone();
    ok.push(bridge_claim(
        "good",
        "bafyreibg",
        "done",
        &["intent"],
        r#"{"seq": [{"any": [{"atom": "fast"}, {"atom": "careful"}]}, {"atom": "ship"}]}"#,
    ));
    let kan = write_kan_stub(dir.path(), &ok);
    let out = day(dir.path(), &kan, &["bridge", "check", "good"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );

    // Only `careful` produces proof, so `certify` cannot rely on it.
    let dir = tempfile::tempdir().unwrap();
    let mut bad = base;
    bad.push(bridge_claim(
        "risky",
        "bafyreibr",
        "done",
        &["intent"],
        r#"{"seq": [{"any": [{"atom": "fast"}, {"atom": "careful"}]}, {"atom": "certify"}]}"#,
    ));
    let kan = write_kan_stub(dir.path(), &bad);
    let out = day(dir.path(), &kan, &["bridge", "check", "risky"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!out.status.success(), "{stdout}");
    assert!(stdout.contains("proof"), "{stdout}");
}

#[test]
fn ac8_a_target_with_no_witnesses_says_so_and_does_not_fail() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(claim(
        "telos/vague",
        "bafyreiv",
        "A telos with no witnesses.",
    ));
    claims.push(bridge_claim(
        "b",
        "bafyreibridge",
        "vague",
        &["intent"],
        r#"{"atom": "design"}"#,
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "an undeclared witness list is a gap, not a failure: {stdout}"
    );
    assert!(stdout.contains("declares no witnesses"), "{stdout}");
    assert!(stdout.contains("--witness"), "it should say how to fix it");
}

/// `.design/bridging.md` AC-9. day checks whether an arrangement *could*
/// reach a telos and stops there. The moment it reports how far along
/// something is, it has become a task tracker — which is what the smell test
/// in CLAUDE.md warns hardest about.
#[test]
fn ac9_no_verb_reports_or_tracks_progress() {
    let out = Command::new(env!("CARGO_BIN_EXE_day"))
        .arg("--help")
        .output()
        .expect("failed to run day --help");
    let help = String::from_utf8_lossy(&out.stdout).to_lowercase();
    for progress_word in [
        "progress",
        "status of",
        "complete",
        "done",
        "remaining",
        "todo",
    ] {
        assert!(
            !help.contains(progress_word),
            "day's surface should not speak of progress; found {progress_word:?}"
        );
    }
}

#[test]
fn ac10_output_states_that_realizability_is_frame_internal() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(telos_claim("shipped", "bafyreitelos", &["verdict"]));
    claims.push(bridge_claim(
        "b",
        "bafyreibridge",
        "shipped",
        &["intent"],
        r#"{"seq": [{"atom": "design"}, {"atom": "build"}, {"atom": "review"}]}"#,
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("single frame"), "{stdout}");
    assert!(stdout.contains("not checked"), "{stdout}");
}

#[test]
fn checking_an_undeclared_bridge_says_so() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &pipeline());
    let out = day(dir.path(), &kan, &["bridge", "check", "ghost"]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("ghost"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
