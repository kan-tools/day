use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

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
    let invocations = strings(&case, "invocations")?;
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
