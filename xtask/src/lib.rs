pub mod capability;
pub mod command;
pub mod evidence;
pub mod outcome;
pub mod profile;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use capability::process::{Process, ProcessRequest};
use command::{CensusCommand, EvidenceCommand, TrailingArgs, ValidateCommand, Xtask};
use outcome::{CouldNotCheck, Finding, Outcome};

pub fn run(cli: Xtask, root: &Path, process: &dyn Process) -> Outcome<()> {
    match cli {
        Xtask::Validate { command } => run_validate(command, root, process),
        Xtask::Evidence { command } => run_evidence(command, root, process),
        Xtask::Census { command } => run_census(command, root, process),
    }
}

fn run_validate(command: ValidateCommand, root: &Path, process: &dyn Process) -> Outcome<()> {
    match command {
        ValidateCommand::Profile { name, list } => {
            let Some(profile) = profile::by_name(&name) else {
                return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                    "unknown validation profile `{name}`"
                )));
            };
            if list {
                for check in profile.checks {
                    println!("{check}");
                }
                Outcome::Passed(())
            } else {
                run_profile(profile, root, process)
            }
        }
        ValidateCommand::Rfc { self_test } => {
            let args = self_test.then_some("--self-test").into_iter();
            run_legacy(root, process, "scripts/check-rfcs-adrs.sh", args)
        }
        ValidateCommand::Publication { rfc } => match rfc {
            0 => run_legacy(
                root,
                process,
                "scripts/check-rfc0-publication.py",
                std::iter::empty::<&str>(),
            ),
            1 => run_legacy(
                root,
                process,
                "scripts/check-rfc1-denotational-publication.py",
                std::iter::empty::<&str>(),
            ),
            other => Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "no publication validator is registered for RFC {other}"
            ))),
        },
        ValidateCommand::Vectors { path, self_test } => {
            let mut args = vec![path
                .unwrap_or_else(|| PathBuf::from("rfcs/vectors/1-process-model.json"))
                .into_os_string()];
            if self_test {
                args.push(OsString::from("--self-test"));
            }
            run_legacy(root, process, "scripts/check-rfc1-vectors.py", args)
        }
        ValidateCommand::Formal { self_test } => {
            let args = self_test.then_some("--self-test").into_iter();
            run_legacy(
                root,
                process,
                "scripts/check-rfc1-formal-obligations.py",
                args,
            )
        }
    }
}

fn run_evidence(command: EvidenceCommand, root: &Path, process: &dyn Process) -> Outcome<()> {
    match command {
        EvidenceCommand::BehaviourDiff(TrailingArgs { args }) => {
            run_legacy(root, process, "scripts/behaviour-diff.py", args)
        }
        EvidenceCommand::Mutate(TrailingArgs { args }) => {
            run_legacy(root, process, "scripts/mutate.py", args)
        }
        EvidenceCommand::Revert(TrailingArgs { args }) => {
            run_legacy(root, process, "scripts/revert-demo.py", args)
        }
    }
}

fn run_census(command: CensusCommand, root: &Path, process: &dyn Process) -> Outcome<()> {
    match command {
        CensusCommand::Demonstrations(TrailingArgs { args }) => {
            evidence::demonstration_census::run(root, process, &args)
        }
        CensusCommand::Findings(TrailingArgs { args }) => {
            evidence::finding_census::run(root, process, &args)
        }
    }
}

fn run_profile(
    profile: &'static profile::Profile,
    root: &Path,
    process: &dyn Process,
) -> Outcome<()> {
    for check in profile.checks {
        eprintln!("==> {check}");
        let result = match *check {
            "rfc" => run_validate(ValidateCommand::Rfc { self_test: false }, root, process),
            "rfc-self-test" => {
                run_validate(ValidateCommand::Rfc { self_test: true }, root, process)
            }
            "cargo-build" => run_program(
                root,
                process,
                "cargo",
                ["build", "--workspace", "--all-targets"],
            ),
            "cargo-test" => run_program(
                root,
                process,
                "cargo",
                ["test", "--workspace", "--no-fail-fast"],
            ),
            "cargo-clippy" => run_program(
                root,
                process,
                "cargo",
                [
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--",
                    "-D",
                    "warnings",
                ],
            ),
            "cargo-fmt" => run_program(root, process, "cargo", ["fmt", "--all", "--", "--check"]),
            unknown => Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "profile `{}` contains unknown check `{unknown}`",
                profile.name
            ))),
        };
        if !result.is_passed() {
            return result;
        }
    }
    Outcome::Passed(())
}

fn run_legacy<I, S>(root: &Path, process: &dyn Process, program: &str, args: I) -> Outcome<()>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    run_program(root, process, program, args)
}

fn run_program<I, S>(root: &Path, process: &dyn Process, program: &str, args: I) -> Outcome<()>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, root);
    match process.run(&request) {
        Err(error) => Outcome::CouldNotCheck(CouldNotCheck::new(error)),
        Ok(output) => {
            print!("{}", output.stdout);
            eprint!("{}", output.stderr);
            if output.status == 0 {
                Outcome::Passed(())
            } else {
                Outcome::Finding(Finding::new(format!(
                    "`{}` exited {}",
                    request.display(),
                    output.status
                )))
            }
        }
    }
}
