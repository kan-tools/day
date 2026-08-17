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
fn process_results_remain_three_distinct_states() {
    for (result, expected) in [
        (
            Ok(ProcessOutput {
                status: 0,
                stdout: String::new(),
                stderr: String::new(),
            }),
            "passed",
        ),
        (
            Ok(ProcessOutput {
                status: 7,
                stdout: String::new(),
                stderr: String::new(),
            }),
            "finding",
        ),
        (Err("missing cargo".to_owned()), "could-not-check"),
    ] {
        let process = FixtureProcess {
            result,
            calls: RefCell::new(Vec::new()),
        };
        let outcome = xtask::run(
            Xtask::Validate {
                command: ValidateCommand::Rfc { self_test: false },
            },
            Path::new("/fixture"),
            &process,
        );
        let actual = match outcome {
            Outcome::Passed(()) => "passed",
            Outcome::Finding(_) => "finding",
            Outcome::CouldNotCheck(_) => "could-not-check",
        };
        assert_eq!(actual, expected);
    }
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
fn rfc_self_test_is_an_explicit_process_request() {
    let process = FixtureProcess {
        result: Ok(ProcessOutput {
            status: 0,
            stdout: String::new(),
            stderr: String::new(),
        }),
        calls: RefCell::new(Vec::new()),
    };
    let outcome = xtask::run(
        Xtask::Validate {
            command: ValidateCommand::Rfc { self_test: true },
        },
        Path::new("/fixture"),
        &process,
    );
    assert!(outcome.is_passed());
    let calls = process.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].display(), "scripts/check-rfcs-adrs.sh --self-test");
}
