use std::cell::RefCell;
use std::path::Path;

use xtask::capability::process::{Process, ProcessOutput, ProcessRequest};
use xtask::command::{ValidateCommand, Xtask};
use xtask::outcome::Outcome;

struct FixtureProcess {
    result: Result<ProcessOutput, String>,
    calls: RefCell<Vec<ProcessRequest>>,
}

impl Process for FixtureProcess {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        self.calls.borrow_mut().push(request.clone());
        self.result.clone()
    }
}

#[test]
fn native_rfc_validation_fails_closed_before_requesting_process_authority() {
    let process = FixtureProcess {
        result: Err("process authority must not be reached".to_owned()),
        calls: RefCell::new(Vec::new()),
    };
    let outcome = xtask::run(
        Xtask::Validate {
            command: ValidateCommand::Rfc { self_test: false },
        },
        Path::new("/fixture"),
        &process,
    );
    assert!(matches!(outcome, Outcome::Finding(_)));
    assert!(process.calls.borrow().is_empty());
}

#[test]
fn profiles_reference_only_known_checks_and_release_contains_ci() {
    let known = [
        "rfc",
        "rfc-self-test",
        "cargo-build",
        "cargo-test",
        "cargo-clippy",
        "cargo-fmt",
    ];
    for profile in xtask::profile::ALL {
        assert!(!profile.checks.is_empty(), "{} is empty", profile.name);
        for check in profile.checks {
            assert!(
                known.contains(check),
                "{} names unknown {check}",
                profile.name
            );
        }
    }
    for check in xtask::profile::CI.checks {
        assert!(xtask::profile::RELEASE.checks.contains(check));
    }
}

#[test]
fn rfc_compatibility_shim_is_policy_free_and_the_normative_path_is_preserved() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let shim = std::fs::read_to_string(root.join("scripts/check-rfcs-adrs.sh")).unwrap();
    assert!(shim.starts_with("#!/bin/sh\nset -eu\n"));
    assert!(shim.contains("-p xtask -- validate rfc"));
    assert!(!shim.contains("mutation") && !shim.contains("require_fields"));

    let rfc0 = std::fs::read_to_string(root.join("rfcs/0-rfc-and-adr-process.md")).unwrap();
    assert!(rfc0.contains("`scripts/check-rfcs-adrs.sh` validates"));
}

#[test]
fn process_authority_carries_stdin_and_environment_explicitly() {
    let request = ProcessRequest::new("git", ["apply", "-"], Path::new("/fixture"))
        .with_env("CARGO_TARGET_DIR", "/fixture/target")
        .with_stdin(b"patch bytes".to_vec());
    assert_eq!(request.stdin.as_deref(), Some(b"patch bytes".as_slice()));
    assert_eq!(
        request.env.get(std::ffi::OsStr::new("CARGO_TARGET_DIR")),
        Some(&"/fixture/target".into())
    );
}
