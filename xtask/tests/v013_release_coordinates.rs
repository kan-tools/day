#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn executable(path: &Path, text: &str) {
    std::fs::write(path, text).unwrap();
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

fn run(root: &Path, bin: &Path, command: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["release", command, SHA])
        .current_dir(root)
        .env(
            "PATH",
            format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
        )
        .output()
        .unwrap()
}

#[test]
fn candidate_and_publication_resolve_every_coordinate_to_one_sha() {
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    executable(
        &bin.join("git"),
        &format!(
            "#!/bin/sh\ncase \"$1 $2\" in\n  'rev-parse HEAD') echo {SHA} ;;\n  'status --porcelain') exit 0 ;;\n  'rev-list -n') echo {SHA} ;;\n  *) exit 97 ;;\nesac\n"
        ),
    );
    executable(
        &bin.join("gh"),
        &format!(
            "#!/bin/sh\ncase \"$1 $2\" in\n  'run list') echo '[{{\"databaseId\":42,\"headSha\":\"{SHA}\",\"status\":\"completed\",\"conclusion\":\"success\"}}]' ;;\n  'release view') echo '{{\"tagName\":\"v0.13.0-beta.1\",\"isDraft\":false}}' ;;\n  *) exit 97 ;;\nesac\n"
        ),
    );
    executable(
        &bin.join("curl"),
        "#!/bin/sh\ncase \" $* \" in\n  *' --output '*) while [ \"$1\" != --output ]; do shift; done; : > \"$2\" ;;\n  *) echo '{\"version\":{\"num\":\"0.13.0-beta.1\"}}' ;;\nesac\n",
    );
    executable(
        &bin.join("tar"),
        &format!("#!/bin/sh\necho '{{\"git\":{{\"sha1\":\"{SHA}\",\"dirty\":false}}}}'\n"),
    );
    executable(
        &bin.join("kan"),
        &format!(
            "#!/bin/sh\necho '{{\"claims\":[{{\"cid\":\"bafy-release\",\"kind\":\"Result\",\"subject\":\"release\",\"text\":\"v0.13.0-beta.1 candidate {SHA} — shipped\"}}]}}'\n"
        ),
    );

    for command in ["verify-candidate-v013", "verify-publication-v013"] {
        let output = run(root.path(), &bin, command);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    executable(
        &bin.join("gh"),
        "#!/bin/sh\ncase \"$1 $2\" in\n  'run list') echo '[{\"databaseId\":42,\"headSha\":\"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\",\"status\":\"completed\",\"conclusion\":\"success\"}]' ;;\n  *) exit 97 ;;\nesac\n",
    );
    let output = run(root.path(), &bin, "verify-candidate-v013");
    assert!(!output.status.success(), "wrong-SHA workflow was accepted");
    assert!(String::from_utf8_lossy(&output.stderr).contains("no completed successful run"));

    executable(&bin.join("gh"), "#!/bin/sh\nexit 97\n");
    let output = run(root.path(), &bin, "verify-candidate-v013");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("COULD-NOT-CHECK"));

    executable(
        &bin.join("git"),
        "#!/bin/sh\ncase \"$1 $2\" in\n  'rev-list -n') echo 'fatal: unknown revision' >&2; exit 128 ;;\n  *) exit 97 ;;\nesac\n",
    );
    let output = run(root.path(), &bin, "verify-publication-v013");
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("FINDING"));

    executable(
        &bin.join("tar"),
        "#!/bin/sh\necho '{\"git\":{\"sha1\":\"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\",\"dirty\":false}}'\n",
    );
    executable(
        &bin.join("git"),
        &format!(
            "#!/bin/sh\ncase \"$1 $2\" in\n  'rev-list -n') echo {SHA} ;;\n  *) exit 97 ;;\nesac\n"
        ),
    );
    executable(
        &bin.join("gh"),
        &format!("#!/bin/sh\ncase \"$1 $2\" in\n  'run list') echo '[{{\"databaseId\":42,\"headSha\":\"{SHA}\",\"status\":\"completed\",\"conclusion\":\"success\"}}]' ;;\n  'release view') echo '{{\"tagName\":\"v0.13.0-beta.1\",\"isDraft\":false}}' ;;\n  *) exit 97 ;;\nesac\n"),
    );
    let output = run(root.path(), &bin, "verify-publication-v013");
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("VCS metadata"));

    executable(&bin.join("tar"), "#!/bin/sh\necho 'not-json'\n");
    let output = run(root.path(), &bin, "verify-publication-v013");
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("malformed"));

    executable(
        &bin.join("tar"),
        &format!("#!/bin/sh\necho '{{\"git\":{{\"sha1\":\"{SHA}\",\"dirty\":false}}}}'\n"),
    );
    for contradictory in [
        format!("v0.13.0-beta.1 was rejected: this is not candidate {SHA}"),
        format!("release notes mention v0.13.0-beta.1 and candidate {SHA}"),
    ] {
        executable(
            &bin.join("kan"),
            &format!(
                "#!/bin/sh\necho '{{\"claims\":[{{\"cid\":\"bafy-release\",\"kind\":\"Result\",\"subject\":\"release\",\"text\":\"{contradictory}\"}}]}}'\n"
            ),
        );
        let output = run(root.path(), &bin, "verify-publication-v013");
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("canonical Result"));
    }
}
