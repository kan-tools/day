use std::path::PathBuf;

use xtask::outcome::Outcome;

#[test]
fn tracked_inventory_matches_actual_surfaces() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let outcome = xtask::validate::instrumentation::run(
        &root,
        std::path::Path::new(".release/instrumentation.json"),
    );
    assert!(matches!(outcome, Outcome::Passed(())));
}

#[test]
fn checked_dependency_boundaries_cannot_be_relabelled() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let source = std::fs::read(root.join(".release/instrumentation.json")).unwrap();
    let mut inventory: serde_json::Value = serde_json::from_slice(&source).unwrap();
    let entries = inventory["entries"].as_array_mut().unwrap();
    let rfc = entries
        .iter_mut()
        .find(|entry| entry["id"] == "command:validate/rfc")
        .unwrap();
    rfc["layer"] = "deterministic-invariant".into();
    let manifest = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(manifest.path(), serde_json::to_vec(&inventory).unwrap()).unwrap();

    let outcome = xtask::validate::instrumentation::run(&root, manifest.path());
    assert!(matches!(outcome, Outcome::Finding(_)));
}
