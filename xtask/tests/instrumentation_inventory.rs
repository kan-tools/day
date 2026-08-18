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
