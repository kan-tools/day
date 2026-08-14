use day::kan_client::Read;

#[cfg(unix)]
fn client(show: &str, status: &str) -> (tempfile::TempDir, day::kan_client::KanClient) {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("kan-read-stub");
    std::fs::write(
        &path,
        format!(
            r#"#!/bin/sh
case "$1 $2 $3" in
 "show --all --json") printf '%s\n' '{show}' ;;
 "status --json ") printf '%s\n' '{status}' ;;
 *) exit 97 ;;
esac
"#,
        ),
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&path, permissions).unwrap();
    let kan = day::kan_client::KanClient::with_bin(dir.path(), path.to_string_lossy());
    (dir, kan)
}

#[test]
fn read_states_are_distinct_values() {
    let present = Read::Present(vec!["claim"]);
    let absent: Read<Vec<&str>> = Read::Absent;
    let withheld: Read<Vec<&str>> = Read::Withheld { count: 2 };
    let indeterminate: Read<Vec<&str>> = Read::Indeterminate { log_wide: 3 };

    assert!(matches!(present, Read::Present(_)));
    assert!(matches!(absent, Read::Absent));
    assert!(matches!(withheld, Read::Withheld { count: 2 }));
    assert!(matches!(indeterminate, Read::Indeterminate { log_wide: 3 }));
}

#[cfg(unix)]
#[test]
fn real_reader_distinguishes_absent_partial_and_unattributed_withholding() {
    let clean_show = r#"{"v":1,"subjects":[],"excluded_by_trust":0}"#;
    let clean_status = r#"{"v":1,"subjects":[],"excluded_by_trust":0}"#;
    let (_dir, clean) = client(clean_show, clean_status);
    assert!(matches!(clean.show("schema/x").unwrap(), Read::Absent));

    let partial_show = r#"{"v":1,"subjects":[{"v":1,"subject":"schema/x","claims":[],"excluded_by_trust":2}],"excluded_by_trust":2}"#;
    let partial_status = r#"{"v":1,"subjects":[{"subject":"schema/x","state":"Unclassified"}],"excluded_by_trust":2}"#;
    let (_dir, partial) = client(partial_show, partial_status);
    assert!(matches!(
        partial.show("schema/x").unwrap(),
        Read::Withheld { count: 2 }
    ));

    let hidden_show = r#"{"v":1,"subjects":[],"excluded_by_trust":4}"#;
    let hidden_status = r#"{"v":1,"subjects":[],"excluded_by_trust":4}"#;
    let (_dir, hidden) = client(hidden_show, hidden_status);
    assert!(matches!(
        hidden.show("schema/x").unwrap(),
        Read::Indeterminate { log_wide: 4 }
    ));
}

#[cfg(unix)]
#[test]
fn withholding_merge_is_order_independent_and_snapshot_consistent() {
    // The bulk snapshot attributes all three withheld claims. The status
    // snapshot sees a larger total but attributes only one, leaving four whose
    // subject is unknown. Maxima accumulated across snapshots must not be
    // subtracted from one another.
    let show = r#"{"v":1,"subjects":[{"v":1,"subject":"schema/a","claims":[],"excluded_by_trust":3}],"excluded_by_trust":3}"#;
    let status = r#"{"v":1,"subjects":[{"subject":"schema/a","state":"Unclassified","excluded_by_trust":1}],"excluded_by_trust":5}"#;

    let (_dir, show_first) = client(show, status);
    // `show` drives the complete reader sequence itself.
    assert!(matches!(
        show_first.show("schema/missing").unwrap(),
        Read::Indeterminate { log_wide: 4 }
    ));
    assert_eq!(show_first.claims_withheld_from_view(), 5);
    assert_eq!(show_first.unattributed_withheld_from_view(), 4);

    let (_dir, status_first) = client(show, status);
    status_first.subjects().unwrap();
    assert!(matches!(
        status_first.show("schema/missing").unwrap(),
        Read::Indeterminate { log_wide: 4 }
    ));
    assert_eq!(status_first.claims_withheld_from_view(), 5);
    assert_eq!(status_first.unattributed_withheld_from_view(), 4);
}

#[test]
fn migrated_read_site_census_is_pinned() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    fn count(dir: &std::path::Path) -> (usize, usize) {
        let mut counts = (0, 0);
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                let nested = count(&path);
                counts.0 += nested.0;
                counts.1 += nested.1;
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let source = std::fs::read_to_string(path).unwrap();
                counts.0 += source.matches(".show(").count();
                counts.1 += source.matches("newest_fenced::<").count();
            }
        }
        counts
    }
    let (show_sites, fenced_sites) = count(&root);
    assert_eq!(
        show_sites, 15,
        "a direct read site was added or removed; audit its visibility decision"
    );
    assert_eq!(
        fenced_sites, 11,
        "a fenced read site was added or removed; audit its visibility decision"
    );
}
