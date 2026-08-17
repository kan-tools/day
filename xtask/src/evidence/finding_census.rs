use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;

use serde_json::{json, Value};

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding as FindingOutcome, Outcome};

#[derive(Debug)]
struct Finding {
    cid: String,
    summary: String,
    status: Option<String>,
    reason: String,
}

pub fn run(root: &Path, process: &dyn Process, args: &[OsString]) -> Outcome<()> {
    let mut subject = None;
    let mut as_json = false;
    for arg in args {
        if arg == "--json" {
            as_json = true;
        } else if arg.to_string_lossy().starts_with('-') || subject.is_some() {
            return Outcome::CouldNotCheck(CouldNotCheck::new(
                "usage: xtask census findings <subject> [--json]",
            ));
        } else {
            subject = Some(arg.to_string_lossy().into_owned());
        }
    }
    let Some(subject) = subject else {
        return Outcome::CouldNotCheck(CouldNotCheck::new(
            "usage: xtask census findings <subject> [--json]",
        ));
    };

    let claims = match read_subject(root, process, &subject) {
        Ok(value) => value,
        Err(error) => return Outcome::CouldNotCheck(error),
    };
    let (findings, complaints) = census(&claims);
    if findings.is_empty() {
        println!("NO-FINDINGS: `{subject}` carries no claim announcing a finding.");
        return Outcome::CouldNotCheck(CouldNotCheck::reported(
            format!("`{subject}` carries no findings"),
            3,
        ));
    }
    let unaccounted = findings
        .iter()
        .filter(|finding| finding.status.is_none())
        .count();

    if as_json {
        let rows = findings
            .iter()
            .map(|finding| {
                json!({
                    "cid": finding.cid,
                    "summary": finding.summary,
                    "status": finding.status,
                    "reason": finding.reason,
                })
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "findings": rows,
                "complaints": complaints,
            }))
            .expect("the census report contains only JSON values")
        );
    } else {
        print_human(&findings);
    }
    for complaint in &complaints {
        println!("\nMALFORMED: {complaint}");
    }
    if unaccounted > 0 || !complaints.is_empty() {
        println!(
            "\nUNACCOUNTED: {unaccounted} finding(s) carry no disposition. Record one as \
             `Disposition: <cid> fixed|accepted|open <reason>` in a claim on this subject. \
             A finding nobody disposed of is a finding that stops existing between rounds — \
             which is how a severity-1 was reported closed while surviving the suite."
        );
        Outcome::Finding(FindingOutcome::reported(
            "one or more findings are unaccounted or malformed",
        ))
    } else {
        Outcome::Passed(())
    }
}

fn read_subject(
    root: &Path,
    process: &dyn Process,
    subject: &str,
) -> Result<Vec<Value>, CouldNotCheck> {
    let request = ProcessRequest::new("kan", ["show", "--all", "--json"], root);
    let output = process.run(&request).map_err(CouldNotCheck::new)?;
    if output.status != 0 {
        return Err(CouldNotCheck::new(format!(
            "kan show --all --json exited {}: {}",
            output.status,
            output.stderr.trim().chars().take(200).collect::<String>()
        )));
    }
    let envelope: Value = serde_json::from_str(&output.stdout).map_err(|error| {
        CouldNotCheck::new(format!("kan's output did not parse as JSON: {error}"))
    })?;
    envelope
        .get("subjects")
        .and_then(Value::as_array)
        .and_then(|subjects| {
            subjects
                .iter()
                .find(|entry| entry.get("subject").and_then(Value::as_str) == Some(subject))
        })
        .and_then(|entry| entry.get("claims"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| CouldNotCheck::new(format!("no subject `{subject}` in the log")))
}

fn census(claims: &[Value]) -> (Vec<Finding>, Vec<String>) {
    let mut findings = Vec::new();
    let mut dispositions = HashMap::new();
    let mut complaints = Vec::new();
    for claim in claims {
        let text = claim.get("text").and_then(Value::as_str).unwrap_or("");
        if announces_finding(text) {
            findings.push(Finding {
                cid: claim
                    .get("cid")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_owned(),
                summary: text
                    .trim()
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(120)
                    .collect(),
                status: None,
                reason: String::new(),
            });
        }
        for line in text.lines() {
            let Some(rest) = line.strip_prefix("Disposition:") else {
                continue;
            };
            let mut fields = rest.trim().splitn(3, char::is_whitespace);
            let (Some(cid), Some(status)) = (fields.next(), fields.next()) else {
                continue;
            };
            if cid.len() < 20 || !["fixed", "accepted", "open"].contains(&status) {
                continue;
            }
            let reason = fields.next().unwrap_or("").trim();
            if ["accepted", "open"].contains(&status) && reason.is_empty() {
                complaints.push(format!(
                    "disposition `{status}` for {} states no reason; accepted/open require one",
                    cid.chars().take(12).collect::<String>()
                ));
                continue;
            }
            dispositions.insert(cid.to_owned(), (status.to_owned(), reason.to_owned()));
        }
    }
    for finding in &mut findings {
        if let Some((status, reason)) = dispositions.get(&finding.cid) {
            finding.status = Some(status.clone());
            finding.reason = reason.clone();
        }
    }
    (findings, complaints)
}

fn announces_finding(text: &str) -> bool {
    text.lines().any(|line| {
        let before_colon = line.split(':').next().unwrap_or("");
        before_colon
            .split(|character: char| !(character.is_alphanumeric() || character == '_'))
            .any(|word| word == "FINDING")
    })
}

fn print_human(findings: &[Finding]) {
    let mut buckets = HashMap::new();
    for finding in findings {
        *buckets
            .entry(finding.status.as_deref().unwrap_or("unaccounted"))
            .or_insert(0usize) += 1;
    }
    println!("| bucket | count |\n| --- | --- |");
    for name in ["fixed", "accepted", "open", "unaccounted"] {
        println!("| {name} | {} |", buckets.get(name).unwrap_or(&0));
    }
    println!("| **total** | **{}** |", findings.len());
    for finding in findings {
        let mark = finding.status.as_deref().unwrap_or("UNACCOUNTED");
        let reason = if finding.reason.is_empty() {
            String::new()
        } else {
            format!(" — {}", finding.reason)
        };
        println!(
            "\n  {mark:11} {}  {}{reason}",
            finding.cid.chars().take(12).collect::<String>(),
            finding.summary
        );
    }
}
