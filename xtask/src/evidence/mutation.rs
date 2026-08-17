use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::capability::process::{Process, ProcessOutput, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if args.len() != 4 {
        return Outcome::CouldNotCheck(CouldNotCheck::new(
            "usage: xtask evidence mutate <path> <anchor> <replacement> <name>",
        ));
    }
    let path = root.join(&args[0]);
    let anchor = args[1].to_string_lossy();
    let replacement = args[2].to_string_lossy();
    let name = args[3].to_string_lossy();
    let original = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(error) => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "could not read {}: {error}",
                path.display()
            )))
        }
    };
    let Some(offset) = original.find(anchor.as_ref()) else {
        println!("{name}: ANCHOR-MISSING (the mutation never happened)");
        return Outcome::CouldNotCheck(CouldNotCheck::reported("mutation anchor is absent", 2));
    };
    if anchor == replacement {
        println!("{name}: ANCHOR-MISSING (replacement identical to anchor)");
        return Outcome::CouldNotCheck(CouldNotCheck::reported(
            "mutation replacement is identical",
            2,
        ));
    }

    let base_build = match cargo(root, process, ["test", "--workspace", "--no-run"]) {
        Ok(output) => output,
        Err(error) => return Outcome::CouldNotCheck(error),
    };
    if base_build.status != 0 {
        println!("{name}: BASELINE-RED (the tree does not build; nothing was mutated)");
        return Outcome::CouldNotCheck(CouldNotCheck::reported("baseline does not build", 3));
    }
    let base = match suite(root, process) {
        Ok(output) => output,
        Err(error) => return Outcome::CouldNotCheck(error),
    };
    if base.status != 0 {
        println!("{name}: BASELINE-RED (the suite was already failing; nothing was mutated)");
        for line in failing_tests(&base).into_iter().take(4) {
            println!("    {line}");
        }
        println!("    Fix the baseline first — against a red suite every mutation reports CAUGHT.");
        return Outcome::CouldNotCheck(CouldNotCheck::reported("baseline suite is red", 3));
    }

    let mut mutated = original.clone();
    mutated.replace_range(offset..offset + anchor.len(), replacement.as_ref());
    if let Err(error) = fs::write(&path, mutated) {
        return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
            "could not write mutation to {}: {error}",
            path.display()
        )));
    }
    let guard = Restore {
        path: path.clone(),
        original,
        active: true,
    };

    let result = match cargo(root, process, ["test", "--workspace", "--no-run"]) {
        Err(error) => Outcome::CouldNotCheck(error),
        Ok(build) if build.status != 0 => {
            println!("{name}: DID-NOT-COMPILE (inconclusive — says nothing about coverage)");
            Outcome::CouldNotCheck(CouldNotCheck::reported("mutant did not compile", 2))
        }
        Ok(_) => match suite(root, process) {
            Err(error) => Outcome::CouldNotCheck(error),
            Ok(tests) if tests.status != 0 => {
                println!("{name}: CAUGHT");
                for line in failing_tests(&tests) {
                    println!("    {line}");
                }
                Outcome::Passed(())
            }
            Ok(_) => {
                println!("{name}: *** SURVIVED *** — nothing asserts this");
                Outcome::Finding(Finding::reported("mutation survived"))
            }
        },
    };

    if let Err(error) = guard.restore() {
        eprintln!("NOT-RESTORED: {}: {error}", path.display());
        return Outcome::CouldNotCheck(CouldNotCheck::reported(
            "the source file could not be restored",
            2,
        ));
    }
    match cargo(
        root,
        process,
        ["build", "--quiet", "--workspace", "--all-targets"],
    ) {
        Ok(output) if output.status == 0 => {}
        _ => {
            println!("    warning: the post-restore rebuild failed; `target/` may still");
            println!("    hold artifacts built from the mutant. Run `cargo build` before");
            println!("    probing anything by hand.");
        }
    }
    result
}

struct Restore {
    path: PathBuf,
    original: String,
    active: bool,
}

impl Restore {
    fn restore(mut self) -> std::io::Result<()> {
        fs::write(&self.path, &self.original)?;
        self.active = false;
        Ok(())
    }
}

impl Drop for Restore {
    fn drop(&mut self) {
        if self.active {
            if let Err(error) = fs::write(&self.path, &self.original) {
                eprintln!("NOT-RESTORED: {}: {error}", self.path.display());
            }
        }
    }
}

fn suite(root: &Path, process: &dyn Process) -> Result<ProcessOutput, CouldNotCheck> {
    cargo(
        root,
        process,
        ["test", "--quiet", "--workspace", "--no-fail-fast"],
    )
}

fn cargo<I, S>(root: &Path, process: &dyn Process, args: I) -> Result<ProcessOutput, CouldNotCheck>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new("cargo", args, root);
    process.run(&request).map_err(CouldNotCheck::new)
}

fn failing_tests(output: &ProcessOutput) -> Vec<String> {
    format!("{}{}", output.stdout, output.stderr)
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("---- "))
        .map(str::to_owned)
        .collect()
}
