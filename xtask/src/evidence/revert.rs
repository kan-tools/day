use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::capability::process::{Process, ProcessOutput, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePatch {
    pub path: String,
    pub header: String,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    pub new_start: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilteredPatch {
    pub text: String,
    pub report: Vec<String>,
    pub touched: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demonstration {
    pub caught: Vec<String>,
    pub outcome: DemonstrationOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemonstrationOutcome {
    Demonstrated,
    Vacuous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevertError {
    pub outcome: &'static str,
    pub detail: String,
}

pub struct DemonstrateRequest<'a> {
    pub patch: &'a str,
    pub names: &'a [String],
    pub label: &'a str,
    pub include: &'a [String],
    pub exclude: &'a [String],
    pub target_dir: Option<&'a Path>,
    pub allow_rejects: bool,
}

#[derive(Default)]
struct CliArgs {
    tests: Option<Vec<String>>,
    rev: Option<String>,
    include: Vec<String>,
    exclude: Vec<String>,
}

pub fn is_verify(args: &[OsString]) -> bool {
    args.iter().any(|arg| arg == "--verify")
}

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if is_verify(args) {
        return run_verify(root, process, args);
    }
    let options = match parse_args(args) {
        Ok(options) => options,
        Err(detail) => return Outcome::CouldNotCheck(CouldNotCheck::new(detail)),
    };
    let Some(names) = options.tests.as_ref() else {
        return Outcome::CouldNotCheck(CouldNotCheck::new(
            "--tests is required unless --verify is given",
        ));
    };
    let repository = match git(root, process, ["rev-parse", "--show-toplevel"]) {
        Ok(value) => PathBuf::from(value.trim()),
        Err(error) => return reported_error(error),
    };
    let (patch, label) = if let Some(rev) = &options.rev {
        match patch_for_rev(&repository, process, rev) {
            Ok(patch) => (patch, rev.clone()),
            Err(error) => return reported_error(error),
        }
    } else {
        let patch = match git(&repository, process, ["diff", "--unified=0", "HEAD"]) {
            Ok(value) => value,
            Err(error) => return reported_error(error),
        };
        if let Ok(untracked) = git(
            &repository,
            process,
            ["ls-files", "--others", "--exclude-standard"],
        ) {
            let paths = untracked
                .split_whitespace()
                .filter(|path| !path.starts_with("tests/"))
                .collect::<Vec<_>>();
            if !paths.is_empty() {
                println!("note: untracked and therefore NOT reverted: {paths:?}");
            }
        }
        (patch, "worktree".to_owned())
    };

    match demonstrate(
        &repository,
        process,
        DemonstrateRequest {
            patch: &patch,
            names,
            label: &label,
            include: &options.include,
            exclude: &options.exclude,
            target_dir: None,
            allow_rejects: false,
        },
    ) {
        Err(error) => reported_error(error),
        Ok(result) if result.outcome == DemonstrationOutcome::Vacuous => {
            println!("\n*** VACUOUS *** — the fix was reverted and {names:?} still passed.");
            println!("The test does not observe the finding it was written to close.");
            Outcome::Finding(Finding::reported("revert demonstration was vacuous"))
        }
        Ok(result) => {
            let quiet = names
                .iter()
                .filter(|name| !result.caught.contains(name))
                .collect::<Vec<_>>();
            if !quiet.is_empty() {
                let rendered = quiet
                    .iter()
                    .map(|name| format!("'{name}'"))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!(
                    "note: named but did not fail under revert, so NOT in the trailer: [{rendered}]"
                );
            }
            println!(
                "\nDEMONSTRATED — failed under revert: {}",
                result.caught.join(", ")
            );
            if options.rev.as_deref().is_none_or(|rev| rev == "HEAD") {
                println!("\nPaste into the commit message:\n");
                let scope = if options.include.is_empty() {
                    String::new()
                } else {
                    format!("include={} ", options.include.join(","))
                };
                println!(
                    "    Demonstrated-by: revert=HEAD tests={} {scope}outcome=DEMONSTRATED",
                    result.caught.join(",")
                );
            } else {
                println!(
                    "\nNo trailer printed: this demonstrated {}, which is not the commit a trailer would land on.",
                    options.rev.as_deref().unwrap_or_default()
                );
            }
            Outcome::Passed(())
        }
    }
}

fn run_verify(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if args.len() != 2 || args[0] != "--verify" {
        return Outcome::CouldNotCheck(CouldNotCheck::new(
            "usage: xtask evidence revert --verify <rev>",
        ));
    }
    let spec = args[1].to_string_lossy();
    let repository = match git(root, process, ["rev-parse", "--show-toplevel"]) {
        Ok(value) => PathBuf::from(value.trim()),
        Err(error) => return reported_error(error),
    };
    let rev = match git(
        &repository,
        process,
        ["rev-parse", "--verify", &format!("{spec}^{{commit}}")],
    ) {
        Ok(value) => value.trim().to_owned(),
        Err(error) => return reported_error(error),
    };
    let body = match git(&repository, process, ["log", "-1", "--format=%B", &rev]) {
        Ok(value) => value,
        Err(error) => return reported_error(error),
    };
    if !body
        .lines()
        .any(|line| line.starts_with(crate::evidence::trailer::PREFIX))
    {
        println!("{spec}: no Demonstrated-by: trailer; nothing to verify");
        return Outcome::Passed(());
    }
    let claim = match crate::evidence::trailer::parse_message(&body) {
        Ok(value) => value,
        Err(error) => {
            return reported_error(RevertError::new(
                "REVERT-FAILED",
                format!("{rev} carries a trailer that does not parse: {error}"),
            ))
        }
    };
    if claim.outcome != "DEMONSTRATED" {
        println!(
            "{spec}: *** {} *** — a trailer may only claim DEMONSTRATED.",
            claim.outcome
        );
        return Outcome::Finding(Finding::reported("trailer claims a non-demonstration"));
    }
    let include = claim
        .include
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let patch = match patch_for_rev(&repository, process, &rev) {
        Ok(value) => value,
        Err(error) => return reported_error(error),
    };
    let head = match git(
        &repository,
        process,
        ["rev-parse", "--verify", "HEAD^{commit}"],
    ) {
        Ok(value) => value.trim().to_owned(),
        Err(error) => return reported_error(error),
    };

    let first = verify_attempt(
        &repository,
        process,
        &rev,
        &patch,
        &claim.tests,
        &include,
        false,
    );
    let result = match first {
        Err(error) if error.outcome == "BASELINE-RED" && rev != head => {
            println!(
                "{rev}: historical baseline is red; retrying the same reversion against audited HEAD {head}"
            );
            verify_attempt(
                &repository,
                process,
                &head,
                &patch,
                &claim.tests,
                &include,
                true,
            )
        }
        other => other,
    };
    let result = match result {
        Ok(value) => value,
        Err(error) => return reported_error(error),
    };
    let mut expected = claim.tests.clone();
    let mut actual = result.caught.clone();
    expected.sort();
    actual.sort();
    if result.outcome != DemonstrationOutcome::Demonstrated || expected != actual {
        println!("{spec}: *** VACUOUS *** — the trailer claims DEMONSTRATED");
        return Outcome::Finding(Finding::reported("trailer did not re-derive"));
    }
    println!(
        "{spec}: DEMONSTRATED (re-derived; caught by {})",
        result.caught.join(", ")
    );
    Outcome::Passed(())
}

fn verify_attempt(
    repository: &Path,
    process: &dyn Process,
    at: &str,
    patch: &str,
    names: &[String],
    include: &[String],
    allow_rejects: bool,
) -> Result<Demonstration, RevertError> {
    let temp = tempfile::Builder::new()
        .prefix("revert-demo-")
        .tempdir()
        .map_err(|error| RevertError::new("REVERT-FAILED", error.to_string()))?;
    let tree = temp.path().join("tree");
    git(
        repository,
        process,
        [
            OsString::from("worktree"),
            OsString::from("add"),
            OsString::from("--detach"),
            tree.as_os_str().to_owned(),
            OsString::from(at),
        ],
    )?;
    let result = demonstrate(
        &tree,
        process,
        DemonstrateRequest {
            patch,
            names,
            label: at,
            include,
            exclude: &[],
            target_dir: None,
            allow_rejects,
        },
    );
    let cleanup = git(
        repository,
        process,
        [
            OsString::from("worktree"),
            OsString::from("remove"),
            OsString::from("--force"),
            tree.as_os_str().to_owned(),
        ],
    );
    if let Err(error) = cleanup {
        return Err(RevertError::new(
            "NOT-RESTORED",
            format!("could not remove verification worktree: {}", error.detail),
        ));
    }
    result
}

fn parse_args(args: &[OsString]) -> Result<CliArgs, String> {
    let mut parsed = CliArgs::default();
    let mut index = 0;
    while index < args.len() {
        let flag = args[index].to_string_lossy();
        let value = |index: &mut usize| -> Result<String, String> {
            *index += 1;
            args.get(*index)
                .map(|value| value.to_string_lossy().into_owned())
                .ok_or_else(|| format!("{flag} requires a value"))
        };
        match flag.as_ref() {
            "--tests" => {
                parsed.tests = Some(
                    value(&mut index)?
                        .split(',')
                        .filter(|name| !name.is_empty())
                        .map(str::to_owned)
                        .collect(),
                );
            }
            "--rev" => parsed.rev = Some(value(&mut index)?),
            "--include" => parsed.include.push(value(&mut index)?),
            "--exclude" => parsed.exclude.push(value(&mut index)?),
            unknown => return Err(format!("unknown revert option `{unknown}`")),
        }
        index += 1;
    }
    Ok(parsed)
}

fn patch_for_rev(root: &Path, process: &dyn Process, rev: &str) -> Result<String, RevertError> {
    let parents = git(root, process, ["rev-list", "--parents", "-n", "1", rev])?;
    match parents.split_whitespace().count() {
        count if count > 2 => Err(RevertError::new(
            "REVERT-FAILED",
            format!("{rev} is a merge commit; a merge has no single change to invert"),
        )),
        2 => git(
            root,
            process,
            ["diff", "--unified=0", &format!("{rev}^"), rev],
        ),
        _ => git(root, process, ["show", "--format=", "--unified=0", rev]),
    }
}

fn git<I, S>(root: &Path, process: &dyn Process, args: I) -> Result<String, RevertError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new("git", args, root);
    let output = process
        .run(&request)
        .map_err(|error| RevertError::new("REVERT-FAILED", error))?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(RevertError::new(
            "REVERT-FAILED",
            format!("{}: {}", request.display(), output.stderr.trim()),
        ))
    }
}

fn reported_error(error: RevertError) -> Outcome<()> {
    println!("{}: {}", error.outcome, error.detail);
    Outcome::CouldNotCheck(CouldNotCheck::reported(error.detail, 1))
}

impl RevertError {
    fn new(outcome: &'static str, detail: impl Into<String>) -> Self {
        Self {
            outcome,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Default)]
struct TestSelection {
    ran: BTreeSet<String>,
    failed: BTreeSet<String>,
}

pub fn demonstrate(
    root: &Path,
    process: &dyn Process,
    request: DemonstrateRequest<'_>,
) -> Result<Demonstration, RevertError> {
    let filtered = filter_patch(request.patch, root, request.include, request.exclude);
    println!("What would be reverted ({}):", request.label);
    for line in &filtered.report {
        println!("{line}");
    }
    if filtered.text.trim().is_empty() {
        return Err(RevertError::new(
            "REVERT-FAILED",
            "nothing left to revert once the test half was excluded. Either the change is test-only, or --include/--exclude excluded the fix.",
        ));
    }

    let baseline = run_tests(root, process, request.names, request.target_dir)?;
    require_ran(request.names, &baseline, "baseline")?;
    let already_failed = baseline
        .values()
        .flat_map(|selection| selection.failed.iter().cloned())
        .collect::<BTreeSet<_>>();
    if !already_failed.is_empty() {
        return Err(RevertError::new(
            "BASELINE-RED",
            format!(
                "already failing before the revert: {already_failed:?}. A demonstration against a red baseline reports the strongest possible result for the wrong reason."
            ),
        ));
    }

    let snapshot = Snapshot::take(root, &filtered.touched)?;
    let mut apply_args = vec!["apply", "-R", "--unidiff-zero"];
    if request.allow_rejects {
        apply_args.push("--reject");
    }
    apply_args.push("-");
    let apply = ProcessRequest::new("git", apply_args, root).with_stdin(filtered.text.into_bytes());
    let applied = process
        .run(&apply)
        .map_err(|error| RevertError::new("REVERT-FAILED", error));
    let result = match applied {
        Err(error) => Err(error),
        Ok(output) if output.status != 0 && !request.allow_rejects => Err(RevertError::new(
            "REVERT-FAILED",
            format!("the reverse patch did not apply: {}", output.stderr.trim()),
        )),
        Ok(output) => {
            if output.status != 0 {
                println!(
                    "current-head fallback: rejected overlapping historical hunks; checking every still-applicable hunk"
                );
            }
            let under_revert = run_tests(root, process, request.names, request.target_dir)?;
            require_ran(request.names, &under_revert, "under revert")?;
            let caught = under_revert
                .iter()
                .filter(|(_, selection)| {
                    !selection.ran.is_empty() && selection.ran.is_subset(&selection.failed)
                })
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>();
            Ok(Demonstration {
                outcome: if caught.is_empty() {
                    DemonstrationOutcome::Vacuous
                } else {
                    DemonstrationOutcome::Demonstrated
                },
                caught,
            })
        }
    };

    snapshot.restore()?;
    let after_restore = run_tests(root, process, request.names, request.target_dir)?;
    let still_failed = after_restore
        .values()
        .flat_map(|selection| selection.failed.iter().cloned())
        .collect::<BTreeSet<_>>();
    if !still_failed.is_empty() {
        return Err(RevertError::new(
            "NOT-RESTORED",
            format!("the named tests do not pass again after restoring: {still_failed:?}"),
        ));
    }
    result
}

struct Snapshot {
    root: PathBuf,
    files: Vec<(String, Option<SavedFile>)>,
    active: bool,
}

struct SavedFile {
    bytes: Vec<u8>,
    mode: u32,
}

impl Snapshot {
    fn take(root: &Path, touched: &[String]) -> Result<Self, RevertError> {
        let mut files = Vec::new();
        for relative in touched {
            let path = root.join(relative);
            let saved = if path.exists() {
                Some(SavedFile {
                    bytes: std::fs::read(&path).map_err(|error| {
                        RevertError::new("REVERT-FAILED", format!("{}: {error}", path.display()))
                    })?,
                    mode: mode(&path)?,
                })
            } else {
                None
            };
            files.push((relative.clone(), saved));
        }
        Ok(Self {
            root: root.to_path_buf(),
            files,
            active: true,
        })
    }

    fn restore(mut self) -> Result<(), RevertError> {
        restore_files(&self.root, &self.files)?;
        self.active = false;
        Ok(())
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        if self.active {
            if let Err(error) = restore_files(&self.root, &self.files) {
                eprintln!("NOT-RESTORED: {}", error.detail);
            }
        }
    }
}

fn restore_files(root: &Path, files: &[(String, Option<SavedFile>)]) -> Result<(), RevertError> {
    for (relative, saved) in files {
        let path = root.join(relative);
        match saved {
            None => {
                if path.exists() {
                    std::fs::remove_file(&path).map_err(|error| {
                        RevertError::new("NOT-RESTORED", format!("{}: {error}", path.display()))
                    })?;
                }
            }
            Some(saved) => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).map_err(|error| {
                        RevertError::new("NOT-RESTORED", format!("{}: {error}", parent.display()))
                    })?;
                }
                std::fs::write(&path, &saved.bytes).map_err(|error| {
                    RevertError::new("NOT-RESTORED", format!("{}: {error}", path.display()))
                })?;
                set_mode(&path, saved.mode)?;
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn mode(path: &Path) -> Result<u32, RevertError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|metadata| metadata.permissions().mode())
        .map_err(|error| RevertError::new("REVERT-FAILED", format!("{}: {error}", path.display())))
}

#[cfg(unix)]
fn set_mode(path: &Path, value: u32) -> Result<(), RevertError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(value))
        .map_err(|error| RevertError::new("NOT-RESTORED", format!("{}: {error}", path.display())))
}

#[cfg(not(unix))]
fn mode(_path: &Path) -> Result<u32, RevertError> {
    Ok(0)
}

#[cfg(not(unix))]
fn set_mode(_path: &Path, _value: u32) -> Result<(), RevertError> {
    Ok(())
}

fn run_tests(
    root: &Path,
    process: &dyn Process,
    specs: &[String],
    target_dir: Option<&Path>,
) -> Result<BTreeMap<String, TestSelection>, RevertError> {
    let mut results = BTreeMap::new();
    for spec in specs {
        let (args, _) = cargo_args(spec);
        let mut build_args = args[..args.len() - 1].to_vec();
        build_args.push("--no-run".into());
        let build = cargo(root, process, build_args, target_dir)?;
        if build.status != 0 {
            return Err(RevertError::new(
                "DID-NOT-COMPILE",
                "the tree does not build, so the named tests could not run",
            ));
        }
        let output = cargo(root, process, args, target_dir)?;
        results.insert(spec.clone(), parse_test_selection(&output));
    }
    Ok(results)
}

fn cargo(
    root: &Path,
    process: &dyn Process,
    args: Vec<String>,
    target_dir: Option<&Path>,
) -> Result<ProcessOutput, RevertError> {
    let mut request = ProcessRequest::new("cargo", args, root);
    if let Some(target) = target_dir {
        request = request.with_env("CARGO_TARGET_DIR", target.as_os_str());
    }
    process
        .run(&request)
        .map_err(|error| RevertError::new("DID-NOT-COMPILE", error))
}

fn parse_test_selection(output: &ProcessOutput) -> TestSelection {
    let mut selection = TestSelection::default();
    for line in format!("{}{}", output.stdout, output.stderr).lines() {
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((name, verdict)) = rest.rsplit_once(" ... ") else {
            continue;
        };
        if verdict == "ok" || verdict == "FAILED" {
            selection.ran.insert(name.to_owned());
            if verdict == "FAILED" {
                selection.failed.insert(name.to_owned());
            }
        }
    }
    selection
}

fn require_ran(
    specs: &[String],
    results: &BTreeMap<String, TestSelection>,
    when: &str,
) -> Result<(), RevertError> {
    let missing = specs
        .iter()
        .filter(|spec| {
            results
                .get(*spec)
                .is_none_or(|selection| selection.ran.is_empty())
        })
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(RevertError::new(
            "NO-SUCH-TEST",
            format!(
                "{when}: no test matched {missing:?}. A filter that matches nothing exits 0, so this can never be read as a pass."
            ),
        ))
    }
}

pub fn split_files(patch: &str) -> Vec<FilePatch> {
    let mut sections = Vec::new();
    let mut path = None;
    let mut lines = Vec::new();
    for line in patch.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            if let Some(previous) = path.take() {
                sections.push(parse_section(previous, &lines.join("")));
            }
            path = rest
                .trim_end()
                .split_once(" b/")
                .map(|(_, value)| value.to_owned());
            lines.clear();
            lines.push(line);
        } else if path.is_some() {
            lines.push(line);
        }
    }
    if let Some(previous) = path {
        sections.push(parse_section(previous, &lines.join("")));
    }
    sections
}

fn parse_section(path: String, section: &str) -> FilePatch {
    let mut header = String::new();
    let mut hunks = Vec::new();
    let mut current = None::<Hunk>;
    for line in section.split_inclusive('\n') {
        if line.starts_with("@@") {
            if let Some(hunk) = current.take() {
                hunks.push(hunk);
            }
            current = Some(Hunk {
                new_start: new_start(line).unwrap_or(0),
                text: line.to_owned(),
            });
        } else if let Some(hunk) = &mut current {
            hunk.text.push_str(line);
        } else {
            header.push_str(line);
        }
    }
    if let Some(hunk) = current {
        hunks.push(hunk);
    }
    FilePatch {
        path,
        header,
        hunks,
    }
}

fn new_start(header: &str) -> Option<usize> {
    let plus = header
        .split_whitespace()
        .find(|part| part.starts_with('+'))?;
    plus.trim_start_matches('+').split(',').next()?.parse().ok()
}

pub fn cfg_test_module_line(source: &str) -> Option<usize> {
    let lines = source.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let stripped = line.trim();
        let Some(after) = stripped.strip_prefix("#[cfg(test)]") else {
            continue;
        };
        if declares_module(after) {
            return Some(index + 1);
        }
        for following in &lines[index + 1..] {
            let following = following.trim();
            if following.is_empty() || following.starts_with('#') {
                continue;
            }
            if declares_module(following) {
                return Some(index + 1);
            }
            break;
        }
    }
    None
}

fn declares_module(text: &str) -> bool {
    let text = text.trim();
    text.starts_with("mod ") || text.starts_with("pub mod ")
}

pub fn filter_patch(
    patch: &str,
    root: &Path,
    include: &[String],
    exclude: &[String],
) -> FilteredPatch {
    let mut text = String::new();
    let mut report = Vec::new();
    let mut touched = Vec::new();
    for file in split_files(patch) {
        if !include.is_empty() && !matches_any(&file.path, include) {
            report.push(format!("  skipped {} (not in --include)", file.path));
            continue;
        }
        if matches_any(&file.path, exclude) {
            report.push(format!("  skipped {} (excluded)", file.path));
            continue;
        }
        if !matches_any(&file.path, include) && file.path.starts_with("tests/") {
            report.push(format!(
                "  skipped {} (test side; --include it to revert it)",
                file.path
            ));
            continue;
        }
        let cutoff = file
            .path
            .ends_with(".rs")
            .then(|| std::fs::read_to_string(root.join(&file.path)).ok())
            .flatten()
            .and_then(|source| cfg_test_module_line(&source));
        let kept = file
            .hunks
            .iter()
            .filter(|hunk| cutoff.is_none_or(|line| hunk.new_start < line))
            .collect::<Vec<_>>();
        let dropped = file.hunks.len() - kept.len();
        if kept.is_empty() {
            report.push(format!(
                "  skipped {} (all {} hunk(s) test-side)",
                file.path,
                file.hunks.len()
            ));
            continue;
        }
        let note = if dropped == 0 {
            String::new()
        } else {
            format!(" ({dropped} test-side hunk(s) kept in place)")
        };
        report.push(format!(
            "  reverting {}: {}/{} hunk(s){note}",
            file.path,
            kept.len(),
            file.hunks.len()
        ));
        touched.push(file.path);
        text.push_str(&file.header);
        for hunk in kept {
            text.push_str(&hunk.text);
        }
    }
    FilteredPatch {
        text,
        report,
        touched,
    }
}

fn matches_any(path: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|pattern| {
        path == pattern || path.starts_with(&format!("{}/", pattern.trim_end_matches('/')))
    })
}

pub fn cargo_args(spec: &str) -> (Vec<String>, String) {
    let Some((target, filter)) = spec.split_once("::") else {
        return (
            vec![
                "test".into(),
                "--workspace".into(),
                "--no-fail-fast".into(),
                spec.into(),
            ],
            spec.into(),
        );
    };
    if target == "lib" {
        (
            vec!["test".into(), "--lib".into(), filter.into()],
            filter.into(),
        )
    } else {
        (
            vec!["test".into(), "--test".into(), target.into(), filter.into()],
            filter.into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATCH: &str = "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-old\n+new\n@@ -8 +8 @@\n-old test\n+new test\ndiff --git a/tests/a.rs b/tests/a.rs\n--- a/tests/a.rs\n+++ b/tests/a.rs\n@@ -1 +1 @@\n-old\n+new\n";

    #[test]
    fn production_hunks_are_kept_and_test_hunks_are_not() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(
            dir.path().join("src/lib.rs"),
            "fn production() {}\n\n#[cfg(test)]\nmod tests {}\n",
        )
        .unwrap();
        let filtered = filter_patch(PATCH, dir.path(), &[], &[]);
        assert_eq!(filtered.touched, ["src/lib.rs"]);
        assert!(filtered.text.contains("@@ -1 +1 @@"));
        assert!(!filtered.text.contains("@@ -8 +8 @@"));
        assert!(filtered
            .report
            .iter()
            .any(|line| line.contains("tests/a.rs")));
    }

    #[test]
    fn an_explicit_include_overrides_the_test_directory_rule() {
        let filtered = filter_patch(PATCH, Path::new("/unused"), &["tests/a.rs".into()], &[]);
        assert_eq!(filtered.touched, ["tests/a.rs"]);
    }

    #[test]
    fn cfg_test_on_a_function_does_not_end_production() {
        let source =
            "#[cfg(test)]\nfn helper() {}\nfn production() {}\n#[cfg(test)] mod tests {}\n";
        assert_eq!(cfg_test_module_line(source), Some(4));
    }

    #[test]
    fn target_qualification_keeps_test_runs_narrow() {
        assert_eq!(
            cargo_args("plugin::specific").0,
            ["test", "--test", "plugin", "specific"]
        );
        assert_eq!(cargo_args("lib::specific").0, ["test", "--lib", "specific"]);
    }
}
