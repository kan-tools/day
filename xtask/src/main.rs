use std::process::ExitCode;

use clap::Parser;
use xtask::capability::process::SystemProcess;
use xtask::command::Cli;
use xtask::outcome::Outcome;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let root = match cli.root() {
        Ok(root) => root,
        Err(error) => {
            eprintln!("COULD-NOT-CHECK: {error}");
            return ExitCode::from(2);
        }
    };

    match xtask::run(cli.command, &root, &SystemProcess) {
        Outcome::Passed(()) => ExitCode::SUCCESS,
        Outcome::Finding(finding) => {
            eprintln!("FINDING: {}", finding.detail);
            ExitCode::from(1)
        }
        Outcome::CouldNotCheck(unknown) => {
            eprintln!("COULD-NOT-CHECK: {}", unknown.detail);
            ExitCode::from(2)
        }
    }
}
