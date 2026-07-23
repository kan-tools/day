//! `.design/current-cycle-position.md` — position is resolved relative to the
//! **current cycle**, and a claim-shaped witness is probeable at all.
//!
//! The defect these cover is not a crash: it is a report that was true and
//! useless. On a repo with history every artifact type exists — some `v*` tag,
//! some past verdict — so "does one exist" always answered yes, day's own log
//! showed four candidate atoms, and it could never narrow (day#60). Bounding
//! the question by the last release is what makes the answer mean *now*.
//!
//! The load-bearing pair is AC-1/AC-3 (bounded reads) against AC-5
//! (assessment is not bounded). Either alone is easy; together they are the
//! whole design.

#![cfg(unix)]

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::{claim, decision_claim, result_claim, write_kan_stub, StubClaim};

fn day(dir: &Path, kan: &Path, git: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        .env("DAY_GIT_BIN", git)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

/// A git stub that can tell **tracked-ever** from **changed-this-cycle**, and
/// that dates its tags — the two distinctions the whole feature rests on.
///
/// `tags` are `name:unix-date` pairs; `tracked` is every tracked file;
/// `changed` is the subset that changed since the boundary. Keeping those two
/// sets separate in the fixture is the point: a stub where they were the same
/// list would pass whether or not day bounded anything.
fn write_git_stub(dir: &Path, tags: &[&str], tracked: &[&str], changed: &[&str]) -> PathBuf {
    let script = dir.join("git-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
match() {{ for i in $1; do case "$i" in $2) printf '%s\n' "$i";; esac; done; }}
case "$1" in
  ls-files) match "{tracked}" "$3" ;;
  diff) match "{changed}" "$5" ;;
  tag)
    for pair in {tags}; do
      name=${{pair%%:*}}
      date=${{pair##*:}}
      case "$name" in
        $3)
          case "$5" in
            --format=*) printf '%s\t%s\n' "$name" "$date" ;;
            *) printf '%s\n' "$name" ;;
          esac
          ;;
      esac
    done ;;
  *) echo "unsupported: $*" >&2; exit 1 ;;
esac
"#,
            tracked = tracked.join(" "),
            changed = changed.join(" "),
            tags = if tags.is_empty() {
                "''".to_string()
            } else {
                tags.join(" ")
            },
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

fn atom(
    slug: &str,
    cid: &str,
    inputs: &[&str],
    outputs: &[&str],
    next: &[&str],
    done: &[&str],
) -> StubClaim {
    let list = |xs: &[&str]| {
        xs.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(",")
    };
    claim(
        &format!("atom/{slug}"),
        cid,
        &format!(
            "The {slug} atom.\n\n```day-atom\n{{\"in\":[{}],\"out\":[{}],\"next\":[{}],\"done\":[{}]}}\n```\n",
            list(inputs), list(outputs), list(next), list(done),
        ),
    )
}

fn witness_schema(cid: &str, body: &str) -> StubClaim {
    claim(
        "schema/witness",
        cid,
        &format!("Witness probes.\n\n```day-witness\n{body}\n```\n"),
    )
}

/// The boundary tag's date, and times either side of it in the microseconds
/// `recorded_at` actually uses. Named rather than inlined so a reader can see
/// at a glance which claims land in which cycle.
const BOUNDARY_UNIX: i64 = 1_700_000_000;
const BEFORE_BOUNDARY_US: i64 = 1_699_000_000_000_000;
const AFTER_BOUNDARY_US: i64 = 1_701_000_000_000_000;

/// AC-1: with a boundary in force, a `path` witness is present only when a
/// matching file **changed since** it. Tracked-but-unchanged is the case that
/// used to read as present forever, and it is the whole of day#60's path half.
#[test]
fn ac1_a_path_witness_is_present_only_when_it_changed_this_cycle() {
    let probes = r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"}}"#;
    let atoms = |dir: &Path| {
        write_kan_stub(
            dir,
            &[
                atom(
                    "design",
                    "bafyreid",
                    &["intent"],
                    &["design-doc"],
                    &["build"],
                    &[],
                ),
                atom(
                    "build",
                    "bafyreib",
                    &["design-doc"],
                    &["code-change"],
                    &["review"],
                    &[],
                ),
                witness_schema("bafyreiw", probes),
            ],
        )
    };

    // Both files are tracked — a repo with history. Only the design doc
    // changed since the release, so this cycle is a design that has not been
    // built yet, and `build` is where the work sits.
    let dir = tempfile::tempdir().unwrap();
    let kan = atoms(dir.path());
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &[".design/x.md", "src/lib.rs"],
        &[".design/x.md"],
    );
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Current atom: build"),
        "a design doc changed this cycle and no code has: {stdout}"
    );

    // The same repo with the code changed too: `build` has produced its
    // output, so it is no longer where the work sits. Same tracked set in
    // both halves — only the *changed* set differs, which is precisely the
    // distinction the old code could not make.
    let dir = tempfile::tempdir().unwrap();
    let kan = atoms(dir.path());
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &[".design/x.md", "src/lib.rs"],
        &[".design/x.md", "src/lib.rs"],
    );
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("Current atom: build"),
        "code changed this cycle, so build has produced its output: {stdout}"
    );
}

/// AC-2: a `claim` probe reports present when a matching claim exists and
/// absent when none does, and a text marker narrows which claims count.
///
/// Run without any boundary, so this tests the probe itself rather than the
/// cycle filter — AC-3 tests the filter.
#[test]
fn ac2_a_claim_probe_finds_claims_by_kind_and_marker() {
    let probes = r#"{"verdict":{"claim":{"kind":"Decision","contains":"adversarial review of"}},"assessment":{"claim":{"kind":"Result"}}}"#;
    let telos = claim(
        "telos/shipped",
        "bafyreitelos",
        "Shipped.\n\n```day-telos\n{\"witnesses\":[\"verdict\",\"assessment\"]}\n```\n",
    );

    // Neither exists: both witnesses report missing.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &[telos.clone(), witness_schema("bafyw", probes)]);
    let git = write_git_stub(dir.path(), &[], &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MISSING] verdict"), "{stdout}");
    assert!(stdout.contains("[MISSING] assessment"), "{stdout}");

    // A `Result` exists, and a `Decision` that is *not* a review. The
    // assessment resolves; the verdict must not — a marker that matched
    // loosely here would make every decision in a log look like a review,
    // which is the failure the marker exists to prevent.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos.clone(),
            witness_schema("bafyw", probes),
            result_claim("atom/build", "bafyres", "assessed", AFTER_BOUNDARY_US),
            decision_claim(
                "roadmap",
                "bafyredec",
                "we will ship Frames after this",
                AFTER_BOUNDARY_US,
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL] assessment"), "{stdout}");
    assert!(
        stdout.contains("[MISSING] verdict"),
        "an unrelated Decision must not satisfy a marked verdict probe: {stdout}"
    );

    // And with a real review recorded, the verdict resolves too.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            telos,
            witness_schema("bafyw", probes),
            result_claim("atom/build", "bafyres", "assessed", AFTER_BOUNDARY_US),
            decision_claim(
                "rigor",
                "bafyrev",
                "adversarial review of rigor: APPROVE — holds",
                AFTER_BOUNDARY_US,
            ),
        ],
    );
    let git = write_git_stub(dir.path(), &[], &[], &[]);
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[MATERIAL] verdict"), "{stdout}");
}

/// AC-3: for **inference**, a claim recorded before the boundary does not
/// count and one recorded after it does.
///
/// This is the half day#60 called the deeper one. A naive claim probe would
/// have reported last milestone's review as this milestone's verdict —
/// present, current, and wrong.
#[test]
fn ac3_a_claim_from_a_prior_cycle_is_not_this_cycles_evidence() {
    let probes = r#"{"code-change":{"path":"src/*.rs"},"verdict":{"claim":{"kind":"Decision","contains":"adversarial review of"}}}"#;
    let vocabulary = |verdict_at: i64| {
        move |dir: &Path| {
            write_kan_stub(
                dir,
                &[
                    atom(
                        "build",
                        "bafyreib",
                        &["design-doc"],
                        &["code-change"],
                        &["review"],
                        &[],
                    ),
                    atom(
                        "review",
                        "bafyrer",
                        &["code-change"],
                        &["verdict"],
                        &[],
                        &[],
                    ),
                    witness_schema("bafyreiw", probes),
                    decision_claim(
                        "rigor",
                        "bafyrev",
                        "adversarial review of rigor: APPROVE — holds",
                        verdict_at,
                    ),
                ],
            )
        }
    };
    let git_for = |dir: &Path| {
        write_git_stub(
            dir,
            &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
            &["src/lib.rs"],
            &["src/lib.rs"],
        )
    };

    // Verdict from a prior cycle: `review` has not produced *this* cycle's
    // verdict, so it is where the work sits.
    let dir = tempfile::tempdir().unwrap();
    let kan = vocabulary(BEFORE_BOUNDARY_US)(dir.path());
    let git = git_for(dir.path());
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stale = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        stale.contains("Current atom: review"),
        "a verdict from a prior cycle must not count as this cycle's: {stale}"
    );

    // Verdict recorded this cycle: `review` has produced its output and is
    // no longer current. Identical inputs but for the timestamp, so nothing
    // else can explain the difference.
    let dir = tempfile::tempdir().unwrap();
    let kan = vocabulary(AFTER_BOUNDARY_US)(dir.path());
    let git = git_for(dir.path());
    let out = day(dir.path(), &kan, &git, &["status"]);
    let fresh = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        !fresh.contains("Current atom: review"),
        "a verdict recorded this cycle means review is done: {fresh}"
    );
    assert_ne!(
        stale, fresh,
        "the two cycles must not render identically — the boundary did nothing"
    );
}

/// AC-4: with no `v*` tag there is no boundary, and inference falls back to
/// its tracked-ever behaviour rather than treating all of history as current.
///
/// The tempting alternative — "no release, so everything is this cycle" —
/// would make a fresh clone report every atom as current, which is worse than
/// the ambiguity being fixed.
#[test]
fn ac4_no_release_means_no_boundary_and_the_cumulative_reading() {
    let probes = r#"{"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*.rs"}}"#;
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom(
                "design",
                "bafyreid",
                &["intent"],
                &["design-doc"],
                &["build"],
                &[],
            ),
            atom(
                "build",
                "bafyreib",
                &["design-doc"],
                &["code-change"],
                &["review"],
                &[],
            ),
            witness_schema("bafyreiw", probes),
        ],
    );
    // No tags at all. The design doc is tracked but has NOT changed since
    // anything — under a boundary it would read absent, so if the fallback
    // failed to engage, `build` would not be current here.
    let git = write_git_stub(dir.path(), &[], &[".design/x.md"], &[]);
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("Current atom: build"),
        "without a boundary, tracked-ever is the reading: {stdout}"
    );
    assert!(
        !stdout.contains("since"),
        "no boundary exists, so nothing should be reported relative to one: {stdout}"
    );
    assert_eq!(out.status.code(), Some(0));
}

/// AC-5, the guardrail that keeps cycle-relativity from leaking: `assess
/// telos` and `assess atom` produce **byte-identical** output with and
/// without a boundary.
///
/// A telos asks whether work ever landed inside its equivalence class, and a
/// release from any time is real evidence for that. Were assessment bounded,
/// last cycle's shipped telos would start reporting as unmet the moment a new
/// tag was cut — a regression invented entirely by the tool.
///
/// The witnesses are deliberately a `path` and a `claim`, not a `tag`. A tag
/// witness cannot take part in this comparison: the boundary *is* the newest
/// `v*` tag, so introducing one to create a boundary also changes what a tag
/// probe legitimately finds, and the two effects could not be told apart.
/// Both witnesses here are cumulatively present and would be bounded-absent,
/// which is what gives the assertion teeth.
#[test]
fn ac5_assessment_is_identical_with_and_without_a_boundary() {
    let probes = r#"{"code-change":{"path":"src/*.rs"},"assessment":{"claim":{"kind":"Result"}}}"#;
    let claims = || {
        vec![
            claim(
                "telos/shipped",
                "bafyreitelos",
                "Shipped.\n\n```day-telos\n{\"witnesses\":[\"code-change\",\"assessment\"]}\n```\n",
            ),
            atom(
                "build",
                "bafyreib",
                &["design-doc"],
                &["code-change"],
                &[],
                &["code-change", "assessment"],
            ),
            witness_schema("bafyreiw", probes),
            // Everything predates the boundary — the state where a bounded
            // reading and a cumulative one differ most.
            result_claim("atom/build", "bafyres", "assessed", BEFORE_BOUNDARY_US),
        ]
    };

    let mut rendered = Vec::new();
    for tags in [vec![], vec![format!("v0.6.0:{BOUNDARY_UNIX}")]] {
        let dir = tempfile::tempdir().unwrap();
        let kan = write_kan_stub(dir.path(), &claims());
        let tag_refs: Vec<&str> = tags.iter().map(String::as_str).collect();
        // Tracked but unchanged since the boundary, so a bounded `path`
        // read would report absent while a cumulative one reports present.
        let git = write_git_stub(dir.path(), &tag_refs, &["src/lib.rs"], &[]);
        let telos = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
        let atom = day(dir.path(), &kan, &git, &["assess", "atom", "build"]);
        rendered.push((
            String::from_utf8_lossy(&telos.stdout).into_owned(),
            String::from_utf8_lossy(&atom.stdout).into_owned(),
            telos.status.code(),
            atom.status.code(),
        ));
    }

    // The `code-change` and `assessment` witnesses must read MATERIAL in
    // both — proving the comparison is between two *resolved* assessments,
    // not two identically empty ones.
    assert!(
        rendered[0].0.contains("[MATERIAL] code-change"),
        "{}",
        rendered[0].0
    );
    assert!(
        rendered[0].1.contains("[MATERIAL] assessment"),
        "{}",
        rendered[0].1
    );
    assert_eq!(
        rendered[0].0, rendered[1].0,
        "`assess telos` became cycle-relative"
    );
    assert_eq!(
        rendered[0].1, rendered[1].1,
        "`assess atom` became cycle-relative"
    );
    assert_eq!(rendered[0].2, rendered[1].2, "telos exit code differs");
    assert_eq!(rendered[0].3, rendered[1].3, "atom exit code differs");
}

/// AC-6: inference over a schema whose `verdict` is a `command` probe executes
/// nothing and leaves the witness unknowable — with a boundary in force, since
/// that is the path this feature added. A cycle is a question about *when*
/// evidence appeared; it does not make executing something acceptable.
#[test]
fn ac6_inference_still_runs_no_command_probe_under_a_boundary() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join("pwned");
    let probes = format!(
        r#"{{"code-change":{{"path":"src/*.rs"}},"verdict":{{"command":"touch {}"}}}}"#,
        sentinel.display()
    );
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom(
                "build",
                "bafyreib",
                &["design-doc"],
                &["code-change"],
                &["review"],
                &[],
            ),
            atom(
                "review",
                "bafyrer",
                &["code-change"],
                &["verdict"],
                &[],
                &["verdict"],
            ),
            witness_schema("bafyreiw", &probes),
        ],
    );
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &["src/lib.rs"],
        &["src/lib.rs"],
    );

    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !sentinel.exists(),
        "`day status` executed a command probe under a boundary: {stdout}"
    );
    // Unknowable, so `review` stays a candidate rather than looking finished.
    assert!(
        stdout.contains("Current atom: review"),
        "a command-probed output is unknown, not present: {stdout}"
    );
    assert!(
        stdout.contains("[not run]"),
        "the criterion should say it was not run: {stdout}"
    );
    assert_eq!(out.status.code(), Some(0));
}

/// AC-7: a `schema/witness` block written before this feature parses and
/// resolves unchanged.
///
/// The `claim` kind is additive — a new enum variant, `contains` omitted when
/// absent — so an existing project's schema must not need touching. Round-trip
/// serialization is asserted in `src/probe.rs`'s unit tests; this is the half
/// that matters to somebody who already has a log.
#[test]
fn ac7_a_pre_feature_witness_schema_still_parses_and_resolves() {
    let dir = tempfile::tempdir().unwrap();
    // Byte-for-byte the shape day shipped before `claim` existed.
    let legacy = r#"{"published-artifact":{"tag":"v*"},"design-doc":{"path":".design/*.md"},"code-change":{"path":"src/*"}}"#;
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "telos/shipped",
                "bafyreitelos",
                "Shipped.\n\n```day-telos\n{\"witnesses\":[\"published-artifact\",\"design-doc\"]}\n```\n",
            ),
            witness_schema("bafyreiw", legacy),
        ],
    );
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.5.0:{BOUNDARY_UNIX}")],
        &[".design/x.md"],
        &[".design/x.md"],
    );
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{stdout}");
    assert!(stdout.contains("[MATERIAL] published-artifact"), "{stdout}");
    assert!(stdout.contains("[MATERIAL] design-doc"), "{stdout}");
}

/// A probe kind day does not know costs that one witness, not the schema.
///
/// Found by dogfooding, and it is the mirror of a rule day already relies on:
/// `kan_client` asserts that a field kan invents later cannot break day's
/// reads. day did not offer the same tolerance for its own schema, so the
/// moment a `claim` probe was recorded on this repo the *installed* v0.6
/// binary failed the entire witness map — and with it the session hook and
/// the status line, on a tool whose telos is legible process. `future-kind`
/// here stands for whatever the next probe kind turns out to be.
#[test]
fn a_probe_kind_this_version_cannot_read_costs_only_its_own_witness() {
    let dir = tempfile::tempdir().unwrap();
    let probes = r#"{"design-doc":{"path":".design/*.md"},"published-artifact":{"future-kind":{"whatever":1}}}"#;
    let kan = write_kan_stub(
        dir.path(),
        &[
            claim(
                "telos/shipped",
                "bafyreitelos",
                "Shipped.\n\n```day-telos\n{\"witnesses\":[\"design-doc\",\"published-artifact\"]}\n```\n",
            ),
            atom(
                "design",
                "bafyreid",
                &["intent"],
                &["design-doc"],
                &["build"],
                &[],
            ),
            witness_schema("bafyreiw", probes),
        ],
    );
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &[".design/x.md"],
        &[],
    );

    let out = day(dir.path(), &kan, &git, &["assess", "telos", "shipped"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "an unreadable probe must not fail the run: {stdout}{stderr}"
    );
    // The readable half still works...
    assert!(
        stdout.contains("[MATERIAL] design-doc"),
        "the rest of the schema must still resolve: {stdout}"
    );
    // ...and the unreadable one says so, rather than reading as absent.
    // `[ERROR]`, not `[MISSING]`: the evidence was not checked, not found
    // wanting, and only the latter may count against a telos.
    assert!(
        stdout.contains("[ERROR] published-artifact"),
        "an unreadable probe should report as unchecked: {stdout}"
    );
    assert!(
        stdout.contains("cannot read"),
        "and should say why: {stdout}"
    );

    // `day status` survives it too — the surface that actually went down.
    let out = day(dir.path(), &kan, &git, &["status"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "status must survive an unreadable probe: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// The boundary tag itself is not evidence of the cycle it closed, and the
/// release cycle closes properly on both sides.
///
/// `v0.6.0` marks the end of the last cycle, so under that boundary a `tag`
/// witness reads absent — which is what lets `release` be current while this
/// cycle's work is merged but unshipped. An off-by-one here (`>=` instead of
/// `>`) leaves `published-artifact` permanently present, which is the exact
/// ambiguity day#60 is about.
///
/// The second half is the consequence worth stating plainly: because the
/// boundary is always the *newest* `v*` tag, a `tag` witness is absent under
/// every boundary. `release` does not stop being current by observing its own
/// output — it stops because cutting the tag opens a new cycle in which
/// nothing has changed yet, so its **input** goes absent. The cycle closes
/// from the input side, and that is coherent rather than a gap: releasing is
/// what ends a cycle, so it cannot also be evidence within it.
#[test]
fn the_boundary_tag_does_not_witness_its_own_cycle() {
    // `code-change` is probed too, so the fixture has no unknowable inputs
    // and the second half's "nothing is current" means it — an unprobed input
    // keeps its atom a candidate, which would make that assertion vacuous.
    // Both stand in for a merge, which is genuinely not path-shaped; the
    // design leaves `merged-change` unprobed in real schemas.
    let probes = r#"{"code-change":{"path":"src/*.rs"},"merged-change":{"path":"src/*.rs"},"published-artifact":{"tag":"v*"}}"#;
    let atoms = [
        atom(
            "pull-request",
            "bafyreip",
            &["code-change"],
            &["merged-change"],
            &["release"],
            &[],
        ),
        atom(
            "release",
            "bafyreir",
            &["merged-change"],
            &["published-artifact"],
            &[],
            &[],
        ),
        witness_schema("bafyreiw", probes),
    ];

    // Only the boundary tag exists: this cycle has merged work but has not
    // released, so `release` is where it sits.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &atoms);
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &["src/lib.rs"],
        &["src/lib.rs"],
    );
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Current atom: release"),
        "the boundary tag must not witness the cycle it closed: {stdout}"
    );

    // The tag has now been cut, so it becomes the boundary and a fresh cycle
    // opens: nothing has changed since it, `merged-change` goes absent, and
    // `release` is no longer where the work sits. Same tracked file in both
    // halves — only the changed-since set moves, exactly as it would in a
    // real repo the moment a release lands.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &atoms);
    let git = write_git_stub(
        dir.path(),
        &[
            &format!("v0.7.0:{}", BOUNDARY_UNIX + 1),
            &format!("v0.6.0:{BOUNDARY_UNIX}"),
        ],
        &["src/lib.rs"],
        &[],
    );
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("Current atom: release"),
        "cutting the release opens a new cycle, so release is no longer current: {stdout}"
    );
    // And the fresh cycle reports nothing current rather than guessing —
    // `pull-request` needs a `code-change` that this cycle has not produced.
    assert!(
        !stdout.contains("Current atom: pull-request"),
        "a cycle with no work yet has no current atom: {stdout}"
    );
}

/// An undated claim cannot be placed in a cycle, so it does not count as
/// *this* one — while still counting cumulatively, where "ever" is the
/// question and a missing timestamp says nothing about it.
///
/// `recorded_at` is additive and optional in kan's shape, so this is a real
/// state, not a hypothetical: a kan that omits it must degrade to "not this
/// cycle" rather than to "current work".
#[test]
fn an_undated_claim_is_not_current_but_is_still_evidence() {
    let probes = r#"{"code-change":{"path":"src/*.rs"},"verdict":{"claim":{"kind":"Decision","contains":"adversarial review of"}}}"#;
    let mut undated = decision_claim(
        "rigor",
        "bafyrev",
        "adversarial review of rigor: APPROVE — holds",
        0,
    );
    undated.recorded_at = None;

    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(
        dir.path(),
        &[
            atom(
                "review",
                "bafyrer",
                &["code-change"],
                &["verdict"],
                &[],
                &[],
            ),
            claim(
                "telos/reviewed",
                "bafyreitelos",
                "Reviewed.\n\n```day-telos\n{\"witnesses\":[\"verdict\"]}\n```\n",
            ),
            witness_schema("bafyreiw", probes),
            undated,
        ],
    );
    let git = write_git_stub(
        dir.path(),
        &[&format!("v0.6.0:{BOUNDARY_UNIX}")],
        &["src/lib.rs"],
        &["src/lib.rs"],
    );

    // Position: not this cycle's verdict, so `review` is still current.
    let out = day(dir.path(), &kan, &git, &["status"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Current atom: review"),
        "an undated claim cannot be placed in this cycle: {stdout}"
    );

    // Assessment: it was recorded at some point, which is the whole question.
    let out = day(dir.path(), &kan, &git, &["assess", "telos", "reviewed"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("[MATERIAL] verdict"),
        "cumulatively, an undated claim is still evidence it happened: {stdout}"
    );
}
