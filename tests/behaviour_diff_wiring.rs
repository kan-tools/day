use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn nested_cargo_checks_have_one_dedicated_gate() {
    let behaviour = std::fs::read_to_string(root().join("tests/behaviour_diff.rs")).unwrap();
    assert_eq!(behaviour.matches("#[test]").count(), 8);
    assert_eq!(
        behaviour
            .matches(
                "#[ignore = \"runs nested Cargo builds; use the dedicated behaviour-diff gate\"]"
            )
            .count(),
        8,
        "every nested-Cargo test must stay out of the ordinary suite"
    );

    let command = "cargo test --test behaviour_diff -- --ignored --test-threads=1";
    let ci = std::fs::read_to_string(root().join(".github/workflows/ci.yml")).unwrap();
    assert_eq!(
        ci.matches(command).count(),
        1,
        "CI must run one dedicated gate"
    );
    let job = ci
        .split("  behaviour-diff:\n")
        .nth(1)
        .and_then(|rest| rest.split("\n  test:\n").next())
        .expect("CI must retain the dedicated behaviour-diff job");
    assert!(
        !job.lines().any(|line| line.trim_start().starts_with("if:")),
        "the dedicated CI job or one of its steps must not be conditionally skipped"
    );
    let release = std::fs::read_to_string(root().join(".github/workflows/release.yml")).unwrap();
    assert_eq!(
        release.matches(command).count(),
        1,
        "release must rerun the gate"
    );
    let cut = std::fs::read_to_string(root().join("scripts/cut-release.sh")).unwrap();
    assert_eq!(
        cut.matches(command).count(),
        1,
        "the pre-tag release path must run the gate"
    );
}
