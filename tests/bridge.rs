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
    let witnesses = day::atoms::extract_fenced::<day::bridge::Witnesses>(&log[0])
        .expect("a day-telos block should be written")
        .expect("it should parse");
    assert_eq!(
        witnesses.witnesses,
        vec![day::bridge::Group::One("published-artifact".into())],
        "a `--witness` flag declares a one-member group"
    );

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
    let plan = day::atoms::extract_fenced::<day::bridge::Plan>(&log[0])
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

    // AC-15, the premise: the target must genuinely declare no witnesses, or
    // this asserts the unwitnessed remedy against a witnessed telos.
    assert!(
        !claims
            .iter()
            .any(|c| c.subject == "telos/vague" && c.text.contains("day-telos")),
        "premise: the fixture telos must declare no witnesses"
    );

    let out = day(dir.path(), &kan, &["bridge", "check", "b"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "an undeclared witness list is a gap, not a failure: {stdout}"
    );
    assert!(stdout.contains("declares no witnesses"), "{stdout}");
    assert!(
        stdout.contains("/witness-interview"),
        "it should say how to fix it, and the fix is the interview rather than a \
         guess at a witness: {stdout}"
    );
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

/// AC-3 — **a plan covers an any-of group by producing any one member**, and
/// the report names which.
///
/// The `pipeline()` fixture produces `design-doc`, `code-change` and `verdict`
/// and never produces `published-artifact`. So the group
/// `published-artifact | verdict` is covered by its second member only — a
/// fixture that would be uncovered under the conjunction this replaces, which
/// is what makes it evidence rather than decoration.
#[test]
fn an_any_of_group_is_covered_by_one_member_and_the_report_names_it() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(claim(
        "telos/shipped",
        "bafyreitg",
        "Shipped.\n\n```day-telos\n{\"witnesses\":[[\"published-artifact\",\"verdict\"]]}\n```\n",
    ));
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
    assert!(
        out.status.success(),
        "one produced member covers the group: {stdout}"
    );
    assert!(stdout.contains("reaches telos/shipped"), "{stdout}");
    // REQ-3: naming the member is the point. "reaches" without saying how
    // cannot distinguish a plan satisfying the strong alternative from one
    // scraping by on the weak one.
    assert!(
        stdout.contains("published-artifact | verdict -> verdict"),
        "the report must name the member it counted: {stdout}"
    );
}

/// AC-4 — **a group with no member produced is uncovered, and is named whole.**
///
/// Naming the group rather than one arbitrary member matters: the reader's next
/// action is to produce *any* of them, and a message naming one would send them
/// to satisfy a requirement the telos never made.
#[test]
fn an_any_of_group_with_no_member_produced_is_uncovered_and_named_whole() {
    let dir = tempfile::tempdir().unwrap();
    let mut claims = pipeline();
    claims.push(claim(
        "telos/shipped",
        "bafyreitg",
        "Shipped.\n\n```day-telos\n{\"witnesses\":[[\"published-artifact\",\"assessment\"]]}\n```\n",
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
    assert!(
        !out.status.success(),
        "no member produced means the group is not reached: {stdout}"
    );
    assert!(
        stdout.contains("published-artifact | assessment"),
        "the whole group is the thing that is missing, not one member: {stdout}"
    );
}

/// AC-1 — **`--witness-any` declares a group; `--witness` twice still declares
/// a conjunction.** The two flags have to stay visibly different, because the
/// whole defect was that day had only one of them and read it as the other.
#[test]
fn witness_any_declares_a_group_and_witness_declares_a_conjunct() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "mixed",
            "Statement.",
            "--witness",
            "design-doc",
            "--witness-any",
            "published-artifact,assessment",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = appends(dir.path());
    let witnesses = day::atoms::extract_fenced::<day::bridge::Witnesses>(&log[0])
        .expect("a day-telos block should be written")
        .expect("it should parse");
    assert_eq!(
        witnesses.witnesses,
        vec![
            day::bridge::Group::One("design-doc".into()),
            day::bridge::Group::Any(vec!["published-artifact".into(), "assessment".into()]),
        ]
    );
}

/// A one-member alternative is refused rather than accepted quietly.
///
/// It is either a typo for `--witness` or a half-written group, and both are
/// better as an error than as a declaration that *reads* like a disjunction and
/// behaves like a conjunct — which is the shape of misreading this whole change
/// exists to end.
#[test]
fn a_one_member_witness_any_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "typo",
            "Statement.",
            "--witness-any",
            "design-doc",
        ],
    );
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("at least two"),
        "the error should say what is wrong: {stderr}"
    );
    assert!(
        appends(dir.path()).is_empty(),
        "nothing should be recorded for a refused declaration"
    );
}

/// day#138 — **a witness that cannot fail is reported at declare time.**
///
/// `telos/legible-process` was declared with three witnesses that were all
/// already satisfied on this repo, so it reported met forever — day#86's own
/// objection, inside the declaration written to close day#86. Nothing said so
/// until someone assessed the telos and read the numbers.
///
/// A `claim` probe is the structural case: kan is append-only and day never
/// retracts, so a claim that matched once matches forever. That is the
/// guarantee day is built on, read as a limitation.
#[test]
fn a_witness_that_cannot_fail_is_reported_when_it_is_declared() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyreiw",
                "Probes.\n\n```day-witness\n{\"verdict\":{\"claim\":{\"kind\":\"Decision\"}}}\n```\n",
            ),
            // Premise: a matching claim must already exist, or "already
            // satisfied" is not the state under test.
            claim("some/subject", "bafyreid", "A decision."),
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "t",
            "Statement.",
            "--witness",
            "verdict",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "reported, never refused -- affordance, not enforcement: {stdout}"
    );
    assert!(
        stdout.contains("declared `telos/t`"),
        "the declaration still happens: {stdout}"
    );
    assert!(
        stdout.contains("cannot stop being satisfied"),
        "an append-only probe cannot report absent once satisfied: {stdout}"
    );
    assert!(stdout.contains("day#86"), "{stdout}");
}

/// **day#146 — the same check, on the verb that never ran it.**
///
/// `day atom declare` takes `done` criteria that are witness types resolved by
/// the same probes a telos's witnesses are, and it did not call the check
/// above. Five of day's own nine atoms were declared with a `claim` probe as
/// their sole criterion, so `day assess atom` — which `day status` names as the
/// gate to wire into CI — reports `[MATERIAL]` for them and always will.
///
/// The message differs from the telos wording deliberately: a reader acts on
/// this in a different place, so it names `day assess atom` rather than day#86.
#[test]
fn an_atom_criterion_that_cannot_fail_is_reported_when_it_is_declared() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "schema/witness",
                "bafyreiw",
                "Probes.\n\n```day-witness\n{\"assessment\":{\"claim\":{\"kind\":\"Observation\"}}}\n```\n",
            ),
            // The premise, and it has to MATCH: `common::claim` records kind
            // `Observation`, so a `Result` probe here left the criterion
            // monotone-but-unsatisfied — a different branch with different
            // wording, which the first run of this test said out loud.
            claim("some/subject", "bafyreir", "An observation."),
        ],
    );

    let out = day(
        dir.path(),
        &kan,
        &[
            "atom",
            "declare",
            "a",
            "--in",
            "x",
            "--out",
            "y",
            "--done",
            "assessment",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "reported, never refused -- the same rule as the telos side: {stdout}"
    );
    assert!(
        stdout.contains("cannot stop being satisfied"),
        "an append-only probe cannot report absent once satisfied: {stdout}"
    );
    assert!(
        stdout.contains("day assess atom"),
        "the message must name the verb this actually breaks, not day#86 -- a \
         reader acts on an atom criterion somewhere else: {stdout}"
    );
}

/// And the negative control, without which the test above passes on a build
/// that prints the caution unconditionally.
///
/// A `path` probe is not monotone — files get deleted — and is unsatisfied
/// here, so there is nothing to say and nothing should be said.
#[test]
fn an_atom_criterion_that_can_fail_draws_no_caution() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[claim(
            "schema/witness",
            "bafyreiw",
            "Probes.\n\n```day-witness\n{\"code-change\":{\"path\":\"nowhere/*.rs\"}}\n```\n",
        )],
    );

    let out = day(
        dir.path(),
        &kan,
        &[
            "atom",
            "declare",
            "a",
            "--in",
            "x",
            "--out",
            "y",
            "--done",
            "code-change",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("Declared, and worth knowing"),
        "a criterion that can fail is the normal case and must print nothing \
         extra: {stdout}"
    );
}

/// The other half: a witness with no probe is named, which is day#125's
/// friction 2 — four teloi were declared with witnesses and the fact that none
/// was checkable surfaced much later, at `day status`.
#[test]
fn a_witness_with_no_declared_probe_is_named_at_declare_time() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[]);

    let out = day(
        dir.path(),
        &kan,
        &[
            "telos",
            "declare",
            "t",
            "Statement.",
            "--witness",
            "certificate",
        ],
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert!(
        stdout.contains("certificate") && stdout.contains("no probe is declared"),
        "a witness with nothing behind it must be named when it is written, not \
         discovered at `day status` later: {stdout}"
    );
}
