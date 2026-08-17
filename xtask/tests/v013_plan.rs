#![cfg(unix)]

use sha2::Digest;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

const CID: &str = "bafy-plan-fixture";
const RFC: &str = "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua";
const SOURCE: &str =
    "35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md";
const COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const PLAN: &str = "fixture plan\n35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md\n";
type ManifestMutation = (&'static str, fn(&mut serde_json::Value));

fn executable(path: &Path, text: &str) {
    std::fs::write(path, text).unwrap();
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

fn run(root: &Path, path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["release", "verify-plan-v013"])
        .current_dir(root)
        .env(
            "PATH",
            format!("{}:{}", path.display(), std::env::var("PATH").unwrap()),
        )
        .output()
        .unwrap()
}

fn write_manifest(root: &Path, manifest: &serde_json::Value) {
    std::fs::write(
        root.join(".release/v0.13-plan.json"),
        serde_json::to_vec_pretty(manifest).unwrap(),
    )
    .unwrap();
}

fn assert_rejected(root: &Path, bin: &Path, label: &str) {
    let output = run(root, bin);
    assert!(
        !output.status.success(),
        "{label} mutation was accepted: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn cid_rooted_plan_resolution_checks_the_claim_artifact_and_bytes() {
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    std::fs::create_dir_all(root.path().join(".release")).unwrap();
    std::fs::create_dir_all(root.path().join(".design")).unwrap();
    std::fs::create_dir_all(root.path().join(".claims")).unwrap();
    std::fs::write(
        root.path().join(".design/v0.13-workflow-ergonomics.md"),
        PLAN,
    )
    .unwrap();
    std::fs::write(
        root.path().join(".claims/plan.md"),
        format!("published {CID}"),
    )
    .unwrap();
    let digest = format!("{:x}", sha2::Sha256::digest(PLAN.as_bytes()));
    let manifest = serde_json::json!({
        "schema": 1,
        "cid": CID,
        "subject": "v0.13-workflow-ergonomics",
        "rfc_result": RFC,
        "normative_source": SOURCE,
        "artifact": {"commit": COMMIT, "path": ".design/v0.13-workflow-ergonomics.md", "sha256": digest},
        "published_file": ".claims/plan.md"
    });
    write_manifest(root.path(), &manifest);
    let claim = serde_json::json!({
        "claims": [{
            "cid": CID,
            "kind": "Plan",
            "subject": "v0.13-workflow-ergonomics",
            "text": PLAN,
            "cites": [RFC],
            "artifacts": [format!("Commit(\"{COMMIT}\")"), format!("FileAt(\".design/v0.13-workflow-ergonomics.md\", \"{COMMIT}\")")]
        }]
    });
    executable(
        &bin.join("kan"),
        &format!(
            "#!/bin/sh\ncat <<'JSON'\n{}\nJSON\n",
            serde_json::to_string(&claim).unwrap()
        ),
    );
    executable(
        &bin.join("git"),
        &format!("#!/bin/sh\nprintf '%s' '{}'\n", PLAN.replace('\'', "'\\''")),
    );

    let output = run(root.path(), &bin);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Plan resolved"));

    let mutations: [ManifestMutation; 7] = [
        ("cid", |value| value["cid"] = "bafy-wrong".into()),
        ("subject", |value| value["subject"] = "wrong".into()),
        ("RFC result", |value| {
            value["rfc_result"] = "bafy-wrong".into()
        }),
        ("normative source", |value| {
            value["normative_source"] = "wrong".into()
        }),
        ("commit", |value| {
            value["artifact"]["commit"] = "b".repeat(40).into()
        }),
        ("path", |value| value["artifact"]["path"] = "wrong".into()),
        ("digest", |value| {
            value["artifact"]["sha256"] = "0".repeat(64).into()
        }),
    ];
    for (label, mutate) in mutations {
        let mut changed = manifest.clone();
        mutate(&mut changed);
        write_manifest(root.path(), &changed);
        assert_rejected(root.path(), &bin, label);
    }

    write_manifest(root.path(), &manifest);
    std::fs::write(
        root.path().join(".design/v0.13-workflow-ergonomics.md"),
        "mutated mirror\n",
    )
    .unwrap();
    assert_rejected(root.path(), &bin, "current mirror bytes");

    std::fs::write(
        root.path().join(".design/v0.13-workflow-ergonomics.md"),
        PLAN,
    )
    .unwrap();
    std::fs::write(root.path().join(".claims/plan.md"), "wrong published claim").unwrap();
    assert_rejected(root.path(), &bin, "published claim presence");
}
