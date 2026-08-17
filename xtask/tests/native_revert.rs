use std::process::Command;

use xtask::capability::process::SystemProcess;
use xtask::evidence::revert::{demonstrate, DemonstrateRequest, DemonstrationOutcome};

fn git(root: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn a_real_reverse_patch_fails_its_named_test_and_restores() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir_all(root.join("tests")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='fixture'\nversion='0.0.0'\nedition='2021'\n",
    )
    .unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn answer() -> i32 { 3 }\n").unwrap();
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["add", "Cargo.toml", "src/lib.rs"]);
    git(root, &["commit", "--no-gpg-sign", "-qm", "bad baseline"]);

    let fixed = "pub fn answer() -> i32 { 2 }\n";
    std::fs::write(root.join("src/lib.rs"), fixed).unwrap();
    std::fs::write(
        root.join("tests/a.rs"),
        "#[test]\nfn observes_fix() { assert_eq!(fixture::answer(), 2); }\n",
    )
    .unwrap();
    let patch = git(root, &["diff", "--unified=0", "HEAD"]);
    let result = demonstrate(
        root,
        &SystemProcess,
        DemonstrateRequest {
            patch: &patch,
            names: &["a::observes_fix".into()],
            label: "fixture",
            include: &[],
            exclude: &[],
            target_dir: None,
        },
    )
    .unwrap();

    assert_eq!(result.outcome, DemonstrationOutcome::Demonstrated);
    assert_eq!(result.caught, ["a::observes_fix"]);
    assert_eq!(
        std::fs::read_to_string(root.join("src/lib.rs")).unwrap(),
        fixed
    );
}
