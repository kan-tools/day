use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

#[derive(Debug)]
pub(crate) enum ReviewError {
    Finding(String),
    Unavailable(String),
}

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    if args.len() != 5 {
        return failed(ReviewError::Finding(
            "usage: xtask validate review FILE PR_URL START END OVERRIDE".into(),
        ));
    }
    let values = args
        .iter()
        .map(|value| value.to_string_lossy())
        .collect::<Vec<_>>();
    match check(
        root, process, &values[0], &values[1], &values[2], &values[3], &values[4],
    ) {
        Ok(()) => {
            println!("RFC review: {} has verifiable review evidence", values[0]);
            Outcome::Passed(())
        }
        Err(error) => failed(error),
    }
}

pub(crate) fn check(
    root: &Path,
    process: &dyn Process,
    file: &str,
    url: &str,
    start_text: &str,
    end_text: &str,
    override_text: &str,
) -> Result<(), ReviewError> {
    check_shape(url, start_text, end_text, override_text)?;
    let number = pull_number(url).unwrap();
    let start = timestamp(start_text)?;
    let end = timestamp(end_text)?;
    let pr_source = command(
        process,
        root,
        "gh",
        ["pr", "view", url, "--json", "files,headRefOid,createdAt"],
    )
    .map_err(|_| unavailable("Discussion pull request is not readable"))?;
    let pr: Value = serde_json::from_str(&pr_source)
        .map_err(|_| unavailable("Discussion pull request is not readable"))?;
    let files = pr
        .get("files")
        .and_then(Value::as_array)
        .ok_or_else(|| unavailable("Discussion pull request is not readable"))?;
    require(
        files
            .iter()
            .any(|entry| entry.get("path").and_then(Value::as_str) == Some(file)),
        "Discussion pull request does not contain the RFC file",
    )?;
    let head = pr.get("headRefOid").and_then(Value::as_str).unwrap_or("");
    require(
        head.len() == 40
            && head.chars().all(|character| {
                character.is_ascii_hexdigit()
                    && (!character.is_ascii_alphabetic() || character.is_ascii_lowercase())
            }),
        "Discussion pull request has no readable head commit",
    )?;
    let timeline_source = command(
        process,
        root,
        "gh",
        [
            "api",
            &format!("repos/kan-tools/day/issues/{number}/timeline"),
            "--paginate",
        ],
    )
    .map_err(|_| unavailable("proposal timeline is not readable"))?;
    let timeline: Value = serde_json::from_str(&timeline_source)
        .map_err(|_| unavailable("proposal timeline is not readable"))?;
    let events = timeline
        .as_array()
        .ok_or_else(|| unavailable("proposal timeline is not readable"))?;
    let event = events.iter().find(|event| {
        event.get("event").and_then(Value::as_str) == Some("committed")
            && event.get("sha").and_then(Value::as_str) == Some(head)
    });
    let verification = event.and_then(|event| event.get("verification"));
    require(
        verification
            .and_then(|value| value.get("verified"))
            .and_then(Value::as_bool)
            == Some(true)
            && verification
                .and_then(|value| value.get("verified_at"))
                .and_then(Value::as_str)
                .is_some(),
        "proposal head lacks a server-timestamped verified commit event",
    )?;
    let created = timestamp(
        pr.get("createdAt")
            .and_then(Value::as_str)
            .ok_or_else(|| unavailable("Discussion pull request is not readable"))?,
    )?;
    let verified = timestamp(
        verification
            .and_then(|value| value.get("verified_at"))
            .and_then(Value::as_str)
            .unwrap(),
    )?;
    let anchor = created.max(verified);
    require(
        start >= anchor,
        "review clock starts before the proposal head reached GitHub",
    )?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| unavailable(error.to_string()))?
        .as_secs() as i64;
    require(end <= now, "review period has not ended")?;

    if override_text != "None" {
        let (override_url, override_head) = parse_override(override_text)
            .ok_or_else(|| finding("override does not name this PR and its latest commit"))?;
        require(
            override_url == url && override_head == head,
            "override does not name this PR and its latest commit",
        )?;
        let maintainers = maintainers(&effective_root(root))?;
        require(!maintainers.is_empty(), "maintainer registry is empty")?;
        let reactions_source = command(
            process,
            root,
            "gh",
            [
                "api",
                &format!("repos/kan-tools/day/issues/{number}/reactions"),
                "--paginate",
            ],
        )
        .map_err(|_| unavailable("override reactions are not readable"))?;
        let reactions: Value = serde_json::from_str(&reactions_source)
            .map_err(|_| unavailable("override reactions are not readable"))?;
        let rockets = reactions
            .as_array()
            .into_iter()
            .flatten()
            .filter(|reaction| {
                reaction.get("content").and_then(Value::as_str) == Some("rocket")
                    && reaction
                        .get("created_at")
                        .and_then(Value::as_str)
                        .and_then(|value| timestamp(value).ok())
                        .is_some_and(|value| value >= anchor)
            })
            .filter_map(|reaction| reaction.pointer("/user/login").and_then(Value::as_str))
            .collect::<BTreeSet<_>>();
        let missing = maintainers
            .iter()
            .filter(|maintainer| !rockets.contains(maintainer.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        require(
            missing.is_empty(),
            format!(
                "override lacks post-commit rockets from: {}",
                missing.join(", ")
            ),
        )?;
    }
    Ok(())
}

pub(crate) fn check_shape(
    url: &str,
    start_text: &str,
    end_text: &str,
    override_text: &str,
) -> Result<(), ReviewError> {
    pull_number(url).ok_or_else(|| finding("Discussion is not a day pull-request address"))?;
    let start = timestamp(start_text)?;
    let end = timestamp(end_text)?;
    if override_text == "None" {
        return require(
            end - start >= 72 * 60 * 60,
            "fewer than 72 review hours elapsed",
        );
    }
    let (override_url, override_head) = parse_override(override_text)
        .ok_or_else(|| finding("override does not name this PR and its latest commit"))?;
    require(
        override_url == url
            && override_head.len() == 40
            && override_head
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character)),
        "override does not name this PR and its latest commit",
    )
}

impl ReviewError {
    pub(crate) fn into_parts(self) -> (String, bool) {
        match self {
            Self::Finding(detail) => (detail, false),
            Self::Unavailable(detail) => (detail, true),
        }
    }
}

fn pull_number(url: &str) -> Option<&str> {
    url.strip_prefix("https://github.com/kan-tools/day/pull/")
        .filter(|value| !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()))
}
fn parse_override(value: &str) -> Option<(&str, &str)> {
    let rest = value.strip_prefix("unanimous:")?;
    let (url, head) = rest.rsplit_once('@')?;
    Some((url, head))
}
fn maintainers(root: &Path) -> Result<BTreeSet<String>, ReviewError> {
    let source = std::fs::read_to_string(root.join("rfcs/maintainers.txt"))
        .map_err(|error| unavailable(error.to_string()))?;
    Ok(source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect())
}
fn effective_root(root: &Path) -> PathBuf {
    std::env::var_os("DAY_RFC_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.to_path_buf())
}

fn timestamp(value: &str) -> Result<i64, ReviewError> {
    let bytes = value.as_bytes();
    let shape_is_utc_seconds = bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| [4, 7, 10, 13, 16, 19].contains(&index) || byte.is_ascii_digit());
    if !shape_is_utc_seconds {
        return Err(finding(format!("invalid RFC3339 timestamp: {value}")));
    }
    let value = value
        .strip_suffix('Z')
        .ok_or_else(|| finding(format!("invalid RFC3339 timestamp: {value}")))?;
    let (date, time) = value
        .split_once('T')
        .ok_or_else(|| finding(format!("invalid RFC3339 timestamp: {value}Z")))?;
    let date = date
        .split('-')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| finding(format!("invalid RFC3339 timestamp: {value}Z")))?;
    let time = time
        .split(':')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| finding(format!("invalid RFC3339 timestamp: {value}Z")))?;
    let days_in_month = match date.get(1).copied() {
        Some(2) if date[0] % 400 == 0 || (date[0] % 4 == 0 && date[0] % 100 != 0) => 29,
        Some(2) => 28,
        Some(4 | 6 | 9 | 11) => 30,
        Some(1 | 3 | 5 | 7 | 8 | 10 | 12) => 31,
        _ => 0,
    };
    if date.len() != 3
        || time.len() != 3
        || !(1..=9999).contains(&date[0])
        || !(1..=days_in_month).contains(&date[2])
        || !(0..=23).contains(&time[0])
        || !(0..=59).contains(&time[1])
        || !(0..=59).contains(&time[2])
    {
        return Err(finding(format!("invalid RFC3339 timestamp: {value}Z")));
    }
    Ok(days_from_civil(date[0], date[1], date[2]) * 86_400
        + time[0] * 3_600
        + time[1] * 60
        + time[2])
}
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let mp = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn command<I, S>(
    process: &dyn Process,
    cwd: &Path,
    program: &str,
    args: I,
) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, cwd);
    let output = process.run(&request)?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(format!("{}: {}", request.display(), output.stderr.trim()))
    }
}
fn require(condition: bool, detail: impl Into<String>) -> Result<(), ReviewError> {
    condition.then_some(()).ok_or_else(|| finding(detail))
}
fn finding(detail: impl Into<String>) -> ReviewError {
    ReviewError::Finding(detail.into())
}
fn unavailable(detail: impl Into<String>) -> ReviewError {
    ReviewError::Unavailable(detail.into())
}
fn failed(error: ReviewError) -> Outcome<()> {
    match error {
        ReviewError::Finding(detail) => {
            eprintln!("RFC REVIEW CHECK FAILED: {detail}");
            Outcome::Finding(Finding::reported(detail))
        }
        ReviewError::Unavailable(detail) => {
            eprintln!("RFC REVIEW CHECK FAILED: {detail}");
            Outcome::CouldNotCheck(CouldNotCheck::reported(detail, 2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timestamps_preserve_order_and_duration() {
        let start = timestamp("2026-08-01T00:00:00Z").unwrap();
        let end = timestamp("2026-08-04T00:00:00Z").unwrap();
        assert_eq!(end - start, 72 * 60 * 60);
        assert!(timestamp("2026-8-01T00:00:00Z").is_err());
        assert!(timestamp("2026-02-29T00:00:00Z").is_err());
        assert!(timestamp("2024-02-29T00:00:00Z").is_ok());
    }
    #[test]
    fn pull_and_override_shapes_are_closed() {
        assert_eq!(
            pull_number("https://github.com/kan-tools/day/pull/7"),
            Some("7")
        );
        assert!(pull_number("https://example.com/7").is_none());
        assert_eq!(
            parse_override("unanimous:https://github.com/kan-tools/day/pull/7@abc"),
            Some(("https://github.com/kan-tools/day/pull/7", "abc"))
        );
    }
}
