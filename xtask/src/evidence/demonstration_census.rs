use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::path::Path;

use crate::capability::process::{Process, ProcessRequest};
use crate::evidence::trailer;
use crate::outcome::{CouldNotCheck, Finding, Outcome};

const NOTHING_TO_CHECK: u8 = 3;

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if args.len() > 1 {
        return Outcome::CouldNotCheck(CouldNotCheck::new(
            "usage: xtask census demonstrations [<base>..<head>]",
        ));
    }
    let resolved = match resolve_span(root, process, args.first().map(OsString::as_os_str)) {
        Ok(value) => value,
        Err(error) => return Outcome::CouldNotCheck(error),
    };
    let (span, commits) = resolved;
    if commits.is_empty() {
        println!("NOTHING-TO-CHECK: no commits in {span}");
        return Outcome::CouldNotCheck(CouldNotCheck::reported(
            format!("no commits in {span}"),
            NOTHING_TO_CHECK,
        ));
    }

    let accounted = match accounted_elsewhere(root, process, &commits) {
        Ok(value) => value,
        Err(error) => return Outcome::CouldNotCheck(error),
    };
    let mut buckets: BTreeMap<&str, Vec<String>> = BTreeMap::from([
        ("demonstrated", Vec::new()),
        ("exempt", Vec::new()),
        ("unaccounted", Vec::new()),
    ]);
    for sha in &commits {
        let body = match git(root, process, ["log", "-1", "--format=%B", sha]) {
            Ok(value) => value,
            Err(error) => return Outcome::CouldNotCheck(error),
        };
        let subject = match git(root, process, ["log", "-1", "--format=%s", sha]) {
            Ok(value) => value.trim().to_owned(),
            Err(error) => return Outcome::CouldNotCheck(error),
        };
        let bucket =
            if trailer::parse_message(&body).is_ok_and(|claim| claim.outcome == "DEMONSTRATED") {
                "demonstrated"
            } else if body.lines().any(|line| line.starts_with("No trailer:"))
                || accounted.contains_key(sha)
            {
                "exempt"
            } else {
                "unaccounted"
            };
        let note = accounted
            .get(sha)
            .map(|reason| format!("  [accounted later: {reason}]"))
            .unwrap_or_default();
        buckets
            .get_mut(bucket)
            .expect("all classifier buckets are initialized")
            .push(format!("{} {subject}{note}", &sha[..7]));
    }

    print_report(&span, commits.len(), &buckets);
    if buckets["unaccounted"].is_empty() {
        Outcome::Passed(())
    } else {
        println!(
            "\nUNACCOUNTED: these commits carry no demonstration and state no reason. \
             A docs-only commit needs one too — this repo executes its own documentation."
        );
        Outcome::Finding(Finding::reported("one or more commits are unaccounted"))
    }
}

fn resolve_span(
    root: &Path,
    process: &dyn Process,
    explicit: Option<&OsStr>,
) -> Result<(String, Vec<String>), CouldNotCheck> {
    if let Some(span) = explicit {
        let description = span.to_string_lossy().into_owned();
        let commits = git_os(
            root,
            process,
            [OsStr::new("rev-list"), OsStr::new("--reverse"), span],
        )?;
        return Ok((
            description,
            commits.split_whitespace().map(str::to_owned).collect(),
        ));
    }
    for base_ref in ["refs/remotes/origin/main", "main"] {
        if git(
            root,
            process,
            ["rev-parse", "--verify", &format!("{base_ref}^{{commit}}")],
        )
        .is_err()
        {
            continue;
        }
        let Ok(base) = git(root, process, ["merge-base", base_ref, "HEAD"]) else {
            continue;
        };
        let base = base.trim();
        let commits = git(
            root,
            process,
            [
                "rev-list",
                "--reverse",
                "--no-merges",
                &format!("{base}..HEAD"),
            ],
        )?;
        return Ok((
            format!("{}..HEAD", &base[..7]),
            commits.split_whitespace().map(str::to_owned).collect(),
        ));
    }
    Err(CouldNotCheck::new(
        "no `main` or `origin/main` to take a merge base from, so there is no range to account for",
    ))
}

fn accounted_elsewhere(
    root: &Path,
    process: &dyn Process,
    commits: &[String],
) -> Result<HashMap<String, String>, CouldNotCheck> {
    let span = commits.iter().cloned().collect::<HashSet<_>>();
    let mut accounted = HashMap::new();
    for sha in commits {
        let body = git(root, process, ["log", "-1", "--format=%B", sha])?;
        for line in body.lines() {
            let Some(rest) = line.strip_prefix("Accounts-for:") else {
                continue;
            };
            let mut parts = rest.trim().splitn(2, char::is_whitespace);
            let Some(named) = parts.next().filter(|part| !part.is_empty()) else {
                continue;
            };
            let Some(reason) = parts.next().map(str::trim).filter(|part| !part.is_empty()) else {
                continue;
            };
            let Ok(full) = git(root, process, ["rev-parse", &format!("{named}^{{commit}}")]) else {
                continue;
            };
            let full = full.trim();
            if span.contains(full) && full != sha {
                accounted.insert(full.to_owned(), reason.to_owned());
            }
        }
    }
    Ok(accounted)
}

fn print_report(span: &str, total: usize, buckets: &BTreeMap<&str, Vec<String>>) {
    println!("span: {span}");
    println!("| bucket | count |\n| --- | --- |");
    for name in ["demonstrated", "exempt", "unaccounted"] {
        println!("| {name} | {} |", buckets[name].len());
    }
    println!("| **total** | **{total}** |");
    for name in ["demonstrated", "exempt", "unaccounted"] {
        if !buckets[name].is_empty() {
            println!("\n{name}:");
            for line in &buckets[name] {
                println!("  {line}");
            }
        }
    }
}

fn git<I, S>(root: &Path, process: &dyn Process, args: I) -> Result<String, CouldNotCheck>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    git_os(root, process, args)
}

fn git_os<I, S>(root: &Path, process: &dyn Process, args: I) -> Result<String, CouldNotCheck>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let request = ProcessRequest::new(
        "git",
        args.into_iter()
            .map(|arg| arg.as_ref().to_os_string())
            .collect::<Vec<_>>(),
        root,
    );
    let output = process.run(&request).map_err(CouldNotCheck::new)?;
    if output.status != 0 {
        return Err(CouldNotCheck::new(format!(
            "{}: {}",
            request.display(),
            output.stderr.trim()
        )));
    }
    Ok(output.stdout)
}
