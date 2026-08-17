use std::path::Path;

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
