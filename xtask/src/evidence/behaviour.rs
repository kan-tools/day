use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::capability::process::{Process, ProcessOutput, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

const IDENTICAL: &str = "IDENTICAL";
const COVERAGE_UNKNOWN: &str = "COVERAGE-UNKNOWN";
const CHANGED_AS_DECLARED: &str = "CHANGED-AS-DECLARED";
const CHANGED_UNEXPLAINED: &str = "CHANGED-UNEXPLAINED";
const BASE_DID_NOT_BUILD: &str = "BASE-DID-NOT-BUILD";
const HEAD_DID_NOT_BUILD: &str = "HEAD-DID-NOT-BUILD";
const CORPUS_EMPTY: &str = "CORPUS-EMPTY";

const KAN_STUB: &str = r#"#!/bin/sh
printf 'kan %s\n' "$*" >> "$BEHAVIOUR_INVOCATIONS"
case "$1" in
  show)
    if [ "$2" = "--all" ]; then cat "$FIXTURE/log.json"; exit 0; fi
    printf '{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}\n' "$2"; exit 0 ;;
  identity) printf 'did:key:zFixtureAuthor\n'; exit 0 ;;
  status|issues) cat "$FIXTURE/status.json"; exit 0 ;;
  *) exit 0 ;;
esac
"#;

const GIT_STUB: &str = r#"#!/bin/sh
printf 'git %s\n' "$*" >> "$BEHAVIOUR_INVOCATIONS"
pattern="$3"
match() {
  for item in $1; do
    case "$item" in $pattern) printf '%s\n' "$item" ;; esac
  done
}
case "$1" in
  tag)      match "$FIXTURE_TAGS" ;;
  ls-files) match "$FIXTURE_FILES" ;;
  rev-parse) printf '%s\n' "$FIXTURE" ;;
  *) exit 0 ;;
esac
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BehaviourArgs {
    since: String,
    expected: BTreeSet<String>,
    expected_fixtures: usize,
    corpus: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ObservationError {
    Fixture(String),
    Execution {
        fixture: String,
        invocation: String,
        status: i32,
        detail: String,
    },
}

impl std::fmt::Display for ObservationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixture(detail) => formatter.write_str(detail),
            Self::Execution {
                fixture,
                invocation,
                status,
                detail,
            } => write!(
                formatter,
                "{fixture}:{invocation} exited {status}: {detail}"
            ),
        }
    }
}

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if let Err(detail) = record_invocation() {
        return Outcome::CouldNotCheck(CouldNotCheck::new(detail));
    }
    let options = match parse_args(root, args) {
        Ok(options) => options,
        Err(detail) => return Outcome::CouldNotCheck(CouldNotCheck::new(detail)),
    };
    let fixtures = match load_fixtures(&options.corpus, options.expected_fixtures) {
        Ok(fixtures) => fixtures,
        Err(detail) => return corpus_error(detail),
    };

    let head = root.join("target").join("debug").join(binary_name("day"));
    let head_build = ProcessRequest::new("cargo", ["build", "--bin", "day"], root);
    let head_output = match process.run(&head_build) {
        Ok(output) => output,
        Err(error) => return build_error(HEAD_DID_NOT_BUILD, error, None),
    };
    if head_output.status != 0 || !head.is_file() {
        return build_error(
            HEAD_DID_NOT_BUILD,
            "the current tree does not build, so there is nothing to compare the base against. This says NOTHING about whether behaviour changed.",
            Some(&head_output),
        );
    }

    let work = match tempfile::Builder::new()
        .prefix("day-behaviour-work-")
        .tempdir()
    {
        Ok(work) => work,
        Err(error) => {
            return corpus_error(format!(
                "could not create a private work directory: {error}"
            ))
        }
    };
    if let Err(error) = write_stubs(work.path()) {
        return corpus_error(error);
    }

    let mut head_observations = BTreeMap::new();
    for fixture in &fixtures {
        match observe(process, &head, fixture, work.path()) {
            Ok(observation) => {
                head_observations.insert(fixture.name.clone(), observation);
            }
            Err(error) => return corpus_error(error.to_string()),
        }
    }

    let base = match build_base(root, process, &options.since) {
        Ok(binary) => binary,
        Err((detail, output)) => return build_error(BASE_DID_NOT_BUILD, detail, output.as_ref()),
    };

    let mut differences = Vec::new();
    let mut invocations = Vec::new();
    for fixture in &fixtures {
        invocations.extend(fixture.invocations.iter().cloned());
        let before = match observe(process, &base, fixture, work.path()) {
            Ok(observation) => observation,
            Err(error) => return corpus_error(error.to_string()),
        };
        differences.extend(compare(
            &fixture.name,
            &before,
            &head_observations[&fixture.name],
            &options.expected,
        ));
    }

    println!(
        "corpus: {} fixture(s) against {}",
        fixtures.len(),
        options.since
    );
    let changed_sources = match changed_sources(root, process, &options.since) {
        Ok(paths) => paths,
        Err(detail) => {
            println!("{COVERAGE_UNKNOWN}: {detail}");
            return Outcome::CouldNotCheck(CouldNotCheck::reported(detail, 3));
        }
    };
    invocations.sort();
    invocations.dedup();
    let unreached = unreached_sources(&changed_sources, &invocations);

    if differences.is_empty() {
        if !unreached.is_empty() {
            println!(
                "{COVERAGE_UNKNOWN}: no fixture output differed, but the corpus runs only {invocations:?} and these changed sources have no fixture plausibly exercising them:"
            );
            for path in unreached {
                println!("  unreached: {path}");
            }
            println!(
                "A diff over code the corpus cannot reach is a could-not-check, not a clean bill. Add a fixture that invokes the affected verb, or say in the commit which change this run did not cover."
            );
            return Outcome::CouldNotCheck(CouldNotCheck::reported(
                "the fixture corpus does not plausibly reach every changed source",
                3,
            ));
        }
        println!("{IDENTICAL}");
        return Outcome::Passed(());
    }

    for difference in &differences {
        let mark = if difference.declared {
            "declared"
        } else {
            "UNEXPLAINED"
        };
        println!("  {mark}: {}", difference.key);
    }
    let unexplained = differences
        .iter()
        .filter(|difference| !difference.declared)
        .collect::<Vec<_>>();
    if unexplained.is_empty() {
        println!("{CHANGED_AS_DECLARED}");
        return Outcome::Passed(());
    }
    for difference in &unexplained {
        println!(
            "\n--- {}\nbefore:\n{}\nafter:\n{}",
            difference.key,
            difference.before.trim_end(),
            difference.after.trim_end()
        );
    }
    println!(
        "\n{CHANGED_UNEXPLAINED}: {} change(s) nobody declared. Declare them with --expect if intended; a fix that changes behaviour it was not aimed at is the defect this exists to surface.",
        unexplained.len()
    );
    Outcome::Finding(Finding::reported("behaviour changed without a declaration"))
}

fn record_invocation() -> Result<(), String> {
    let Some(path) = std::env::var_os("DAY_BEHAVIOUR_DIFF_COUNTER") else {
        return Ok(());
    };
    let mut counter = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| format!("could not open behaviour-diff invocation counter: {error}"))?;
    counter
        .write_all(b"1\n")
        .map_err(|error| format!("could not update behaviour-diff invocation counter: {error}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    pub name: String,
    pub path: PathBuf,
    pub invocations: Vec<String>,
    pub tags: Vec<String>,
    pub tracked: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    pub key: String,
    pub before: String,
    pub after: String,
    pub declared: bool,
}

pub fn load_fixtures(corpus: &Path, expected: usize) -> Result<Vec<Fixture>, String> {
    let mut directories = std::fs::read_dir(corpus)
        .map_err(|error| format!("{}: {error}", corpus.display()))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("case.json").is_file())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    directories.sort();
    if directories.len() != expected {
        return Err(format!(
            "expected {expected} fixture(s), found {}",
            directories.len()
        ));
    }
    if directories.is_empty() {
        return Err(format!("no fixtures under {}", corpus.display()));
    }
    directories
        .into_iter()
        .map(|path| load_fixture(&path))
        .collect()
}

fn load_fixture(path: &Path) -> Result<Fixture, String> {
    let case_path = path.join("case.json");
    let source = std::fs::read_to_string(&case_path)
        .map_err(|error| format!("{}: {error}", case_path.display()))?;
    let case: Value = serde_json::from_str(&source)
        .map_err(|error| format!("{}: {error}", case_path.display()))?;
    let invocations = if case.get("invocations").is_none() {
        return Err(format!(
            "{}: declares no `invocations` key, so it compares nothing while still counting as a fixture",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
    } else {
        strings(&case, "invocations")?
    };
    if invocations.is_empty() {
        return Err(format!(
            "{}: declares no invocations, so it compares nothing while still counting as a fixture",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
    if invocations.iter().any(|invocation| {
        invocation
            .split_whitespace()
            .any(|argument| argument == "--run")
    }) {
        return Err(format!(
            "{}: --run is not allowed in the corpus",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
    Ok(Fixture {
        name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        path: path.to_path_buf(),
        invocations,
        tags: optional_strings(&case, "tags")?,
        tracked: optional_strings(&case, "tracked")?,
    })
}

fn strings(value: &Value, key: &str) -> Result<Vec<String>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("declares no `{key}` array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("`{key}` contains a non-string value"))
        })
        .collect()
}

fn optional_strings(value: &Value, key: &str) -> Result<Vec<String>, String> {
    match value.get(key) {
        None => Ok(Vec::new()),
        Some(_) => strings(value, key),
    }
}

pub fn compare(
    fixture: &str,
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
    declared: &BTreeSet<String>,
) -> Vec<Difference> {
    let keys = before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    keys.into_iter()
        .filter_map(|verb| {
            let old = before.get(&verb).map(String::as_str).unwrap_or("<absent>");
            let new = after.get(&verb).map(String::as_str).unwrap_or("<absent>");
            if old == new {
                return None;
            }
            let key = format!("{fixture}:{verb}");
            Some(Difference {
                declared: declared.contains(&key),
                key,
                before: old.to_owned(),
                after: new.to_owned(),
            })
        })
        .collect()
}

pub fn unreached_sources(changed: &[String], invocations: &[String]) -> Vec<String> {
    changed
        .iter()
        .filter(|path| path.starts_with("src/"))
        .filter(|path| {
            module_verb_hints(path).iter().all(|hint| {
                !invocations
                    .iter()
                    .any(|invocation| invocation.contains(hint))
            })
        })
        .cloned()
        .collect()
}

fn module_verb_hints(path: &str) -> &'static [&'static str] {
    match path {
        "src/footer.rs" | "src/cache.rs" => &["status-line", "session-start", "user-prompt"],
        "src/git.rs" => &[
            "status",
            "assess",
            "session-start",
            "user-prompt",
            "status-line",
        ],
        "src/hooks.rs" => &["session-start", "session-notice", "user-prompt"],
        "src/status.rs" | "src/position.rs" => &["status", "session-start", "user-prompt"],
        "src/kan_client.rs" => &[
            "status",
            "doctor",
            "assess",
            "session-start",
            "next",
            "bridge",
        ],
        "src/cli/mod.rs" => &[
            "status",
            "doctor",
            "assess",
            "next",
            "bridge",
            "init",
            "status-line",
        ],
        "src/telos.rs" => &["assess", "doctor"],
        "src/atoms.rs" => &["doctor", "next", "status"],
        "src/probe.rs" => &["assess", "status"],
        "src/practice.rs" => &["session-start"],
        "src/docs.rs" => &["assess"],
        "src/blocks.rs" => &["doctor", "status", "assess"],
        _ => &[],
    }
}

fn parse_args(root: &Path, args: &[OsString]) -> Result<BehaviourArgs, String> {
    let mut since = "HEAD~1".to_owned();
    let mut expected = BTreeSet::new();
    let mut expected_fixtures = None;
    let mut corpus = None;
    let mut index = 0;
    while index < args.len() {
        let argument = args[index].to_string_lossy();
        let mut value = |name: &str| -> Result<OsString, String> {
            index += 1;
            args.get(index)
                .cloned()
                .ok_or_else(|| format!("{name} requires a value"))
        };
        if let Some(value) = argument.strip_prefix("--since=") {
            since = value.to_owned();
        } else if let Some(value) = argument.strip_prefix("--expect=") {
            expected.insert(value.to_owned());
        } else if let Some(value) = argument.strip_prefix("--expect-fixtures=") {
            expected_fixtures = Some(parse_fixture_count(value)?);
        } else if let Some(value) = argument.strip_prefix("--corpus=") {
            corpus = Some(PathBuf::from(value));
        } else {
            match argument.as_ref() {
                "--since" => since = value("--since")?.to_string_lossy().into_owned(),
                "--expect" => {
                    expected.insert(value("--expect")?.to_string_lossy().into_owned());
                }
                "--expect-fixtures" => {
                    let count = value("--expect-fixtures")?;
                    expected_fixtures = Some(parse_fixture_count(&count.to_string_lossy())?);
                }
                "--corpus" => corpus = Some(PathBuf::from(value("--corpus")?)),
                unknown => return Err(format!("unknown behaviour-diff option `{unknown}`")),
            }
        }
        index += 1;
    }
    let expected_fixtures = expected_fixtures.ok_or_else(|| {
        "usage: xtask evidence behaviour-diff [--since REV] [--expect FIXTURE:VERB]... --expect-fixtures N [--corpus PATH]".to_owned()
    })?;
    let corpus = corpus.unwrap_or_else(|| PathBuf::from("fixtures/behaviour"));
    let corpus = if corpus.is_absolute() {
        corpus
    } else {
        root.join(corpus)
    };
    Ok(BehaviourArgs {
        since,
        expected,
        expected_fixtures,
        corpus,
    })
}

fn parse_fixture_count(value: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|error| format!("--expect-fixtures requires a non-negative integer: {error}"))
}

fn observe(
    process: &dyn Process,
    binary: &Path,
    fixture: &Fixture,
    work: &Path,
) -> Result<BTreeMap<String, String>, ObservationError> {
    let cache = work.join(".day");
    if cache.exists() {
        fs::remove_dir_all(&cache).map_err(|error| {
            ObservationError::Fixture(format!("{}: could not clear .day: {error}", fixture.name))
        })?;
    }
    let mut observations = BTreeMap::new();
    for invocation in &fixture.invocations {
        let invocation_log = work.join("invocations");
        if invocation_log.exists() {
            fs::remove_file(&invocation_log).map_err(|error| {
                ObservationError::Fixture(format!(
                    "{}: could not clear invocation accounting: {error}",
                    fixture.name
                ))
            })?;
        }
        let mut arguments = invocation.split_whitespace();
        let request = ProcessRequest::new(
            binary.as_os_str(),
            arguments.by_ref().map(OsString::from),
            work,
        )
        .with_env("FIXTURE", fixture.path.as_os_str())
        .with_env("FIXTURE_TAGS", fixture.tags.join(" "))
        .with_env("FIXTURE_FILES", fixture.tracked.join(" "))
        .with_env("DAY_KAN_BIN", work.join("kan").as_os_str())
        .with_env("DAY_GIT_BIN", work.join("git").as_os_str())
        .with_env("BEHAVIOUR_INVOCATIONS", invocation_log.as_os_str());
        let output = process
            .run(&request)
            .map_err(|detail| ObservationError::Execution {
                fixture: fixture.name.clone(),
                invocation: invocation.clone(),
                status: 2,
                detail,
            })?;
        let errored = output
            .stdout
            .lines()
            .find(|line| line.trim().starts_with("[ERROR]"));
        if !matches!(output.status, 0 | 1)
            || output.stderr.contains("could not read")
            || errored.is_some()
        {
            let detail = errored
                .map(str::trim)
                .or_else(|| {
                    let stderr = output.stderr.trim();
                    (!stderr.is_empty()).then_some(stderr)
                })
                .unwrap_or_else(|| output.stdout.trim());
            return Err(ObservationError::Execution {
                fixture: fixture.name.clone(),
                invocation: invocation.clone(),
                status: output.status,
                detail: detail.chars().take(200).collect(),
            });
        }
        observations.insert(
            invocation.clone(),
            format!("exit={}\n{}", output.status, output.stdout),
        );
        let calls = fs::read_to_string(&invocation_log).unwrap_or_default();
        observations.insert(format!("{invocation} :: subprocesses"), calls);
        if cache.is_dir() {
            let mut artifacts = fs::read_dir(&cache)
                .map_err(|error| ObservationError::Fixture(error.to_string()))?
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .collect::<Vec<_>>();
            artifacts.sort_by_key(|entry| entry.file_name());
            for artifact in artifacts {
                let value = fs::read(artifact.path())
                    .map_err(|error| ObservationError::Fixture(error.to_string()))?;
                observations.insert(
                    format!(
                        "{invocation} :: .day/{}",
                        artifact.file_name().to_string_lossy()
                    ),
                    String::from_utf8_lossy(&value).into_owned(),
                );
            }
        }
    }
    Ok(observations)
}

fn build_base(
    root: &Path,
    process: &dyn Process,
    revision: &str,
) -> Result<PathBuf, (String, Option<ProcessOutput>)> {
    let rev_parse = ProcessRequest::new("git", ["rev-parse", revision], root);
    let resolved = process.run(&rev_parse).map_err(|error| (error, None))?;
    let sha = resolved.stdout.trim();
    if resolved.status != 0 || sha.is_empty() {
        return Err((
            format!("could not resolve `day` at {revision}"),
            Some(resolved),
        ));
    }
    let prefix = sha.chars().take(12).collect::<String>();
    let cache = std::env::temp_dir().join(format!("day-behaviour-{prefix}"));
    let binary = cache.join("target").join("debug").join(binary_name("day"));
    if binary.is_file() {
        return Ok(binary);
    }
    let tree = cache.join("tree");
    if !tree.is_dir() {
        fs::create_dir_all(&cache)
            .map_err(|error| (format!("{}: {error}", cache.display()), None))?;
        let add = ProcessRequest::new(
            "git",
            [
                OsString::from("worktree"),
                OsString::from("add"),
                OsString::from("--detach"),
                tree.clone().into_os_string(),
                OsString::from(sha),
            ],
            root,
        );
        let output = process.run(&add).map_err(|error| (error, None))?;
        if output.status != 0 {
            return Err((
                "could not create the detached base worktree".into(),
                Some(output),
            ));
        }
    }
    let build = ProcessRequest::new("cargo", ["build", "--bin", "day"], &tree)
        .with_env("CARGO_TARGET_DIR", cache.join("target").as_os_str());
    let output = process.run(&build).map_err(|error| (error, None))?;
    if output.status != 0 || !binary.is_file() {
        return Err((
            format!("could not build `day` at {revision}. This says NOTHING about whether behaviour changed."),
            Some(output),
        ));
    }
    Ok(binary)
}

fn changed_sources(
    root: &Path,
    process: &dyn Process,
    revision: &str,
) -> Result<Vec<String>, String> {
    let range = format!("{revision}..HEAD");
    let request = ProcessRequest::new("git", ["diff", "--name-only", &range], root);
    let output = process.run(&request)?;
    if output.status != 0 {
        return Err(format!(
            "could not enumerate changed sources: {}",
            output.stderr.trim()
        ));
    }
    Ok(output
        .stdout
        .split_whitespace()
        .filter(|path| path.starts_with("src/"))
        .map(str::to_owned)
        .collect())
}

fn corpus_error(detail: impl Into<String>) -> Outcome<()> {
    let detail = detail.into();
    println!(
        "{CORPUS_EMPTY}: {detail}\n\nA fixture that errors compares equal to itself, so this would otherwise have reported IDENTICAL while checking nothing."
    );
    Outcome::CouldNotCheck(CouldNotCheck::reported(detail, 2))
}

fn build_error(
    outcome: &'static str,
    detail: impl Into<String>,
    output: Option<&ProcessOutput>,
) -> Outcome<()> {
    let detail = detail.into();
    println!("{outcome}: {detail}");
    if let Some(output) = output {
        let stderr = output.stderr.trim();
        if !stderr.is_empty() {
            eprintln!(
                "{}",
                stderr
                    .chars()
                    .rev()
                    .take(800)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            );
        }
    }
    Outcome::CouldNotCheck(CouldNotCheck::reported(detail, 2))
}

fn write_stubs(work: &Path) -> Result<(), String> {
    for (name, body) in [("kan", KAN_STUB), ("git", GIT_STUB)] {
        let path = work.join(name);
        fs::write(&path, body).map_err(|error| format!("{}: {error}", path.display()))?;
        set_executable(&path)?;
    }
    Ok(())
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))
        .map_err(|error| format!("{}: {error}", path.display()))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<(), String> {
    Err("behaviour-diff fixture stubs currently require a POSIX host".into())
}

fn binary_name(name: &str) -> OsString {
    let mut value = OsString::from(name);
    value.push(std::env::consts::EXE_SUFFIX);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_uses_the_union_so_new_and_removed_artifacts_are_visible() {
        let before = BTreeMap::from([
            ("same".into(), "x".into()),
            ("removed".into(), "old".into()),
        ]);
        let after = BTreeMap::from([("same".into(), "x".into()), ("added".into(), "new".into())]);
        let differences = compare("fixture", &before, &after, &BTreeSet::new());
        assert_eq!(
            differences
                .iter()
                .map(|difference| difference.key.as_str())
                .collect::<Vec<_>>(),
            ["fixture:added", "fixture:removed"]
        );
        assert_eq!(differences[0].before, "<absent>");
        assert_eq!(differences[1].after, "<absent>");
    }

    #[test]
    fn a_declared_change_is_still_listed_but_not_unexplained() {
        let differences = compare(
            "f",
            &BTreeMap::from([("status".into(), "old".into())]),
            &BTreeMap::from([("status".into(), "new".into())]),
            &BTreeSet::from(["f:status".into()]),
        );
        assert!(differences[0].declared);
    }

    #[test]
    fn changed_code_without_a_plausible_invocation_is_unknown() {
        let changed = vec!["src/footer.rs".into(), "src/telos.rs".into()];
        assert_eq!(
            unreached_sources(&changed, &["assess telos x".into()]),
            ["src/footer.rs"]
        );
    }

    #[test]
    fn empty_invocations_and_run_authority_are_refused() {
        let dir = tempfile::tempdir().unwrap();
        let fixture = dir.path().join("case");
        std::fs::create_dir(&fixture).unwrap();
        std::fs::write(fixture.join("case.json"), r#"{"invocations":[]}"#).unwrap();
        assert!(load_fixtures(dir.path(), 1)
            .unwrap_err()
            .contains("compares nothing"));
        std::fs::write(
            fixture.join("case.json"),
            r#"{"invocations":["assess telos x --run"]}"#,
        )
        .unwrap();
        assert!(load_fixtures(dir.path(), 1).unwrap_err().contains("--run"));
    }
}
