use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::capability::process::{Process, ProcessRequest};
use crate::outcome::{CouldNotCheck, Finding, Outcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationKind {
    Rfc0,
    Denotational,
}

impl PublicationKind {
    fn vector(self) -> &'static str {
        match self {
            Self::Rfc0 => "rfcs/vectors/0-publication.json",
            Self::Denotational => "rfcs/vectors/1-denotational-publication.json",
        }
    }
    fn prefix(self) -> &'static str {
        match self {
            Self::Rfc0 => "RFC 0 PUBLICATION CHECK FAILED",
            Self::Denotational => "RFC 1 DENOTATIONAL PUBLICATION CHECK FAILED",
        }
    }
    fn success(self) -> &'static str {
        match self {
            Self::Rfc0 => "RFC 0 publication",
            Self::Denotational => "RFC 1 denotational publication",
        }
    }
}

#[derive(Debug)]
enum CheckError {
    Finding(String),
    Unavailable(String),
}

pub fn run(
    kind: PublicationKind,
    root: &Path,
    process: &dyn Process,
    args: &[OsString],
) -> Outcome<()> {
    let self_test = match args {
        [] => false,
        [flag] if flag == "--self-test" => true,
        _ => {
            return Outcome::CouldNotCheck(CouldNotCheck::new(format!(
                "usage: xtask validate publication --rfc {} [--self-test]",
                if kind == PublicationKind::Rfc0 { 0 } else { 1 }
            )))
        }
    };
    match check(kind, root, process, self_test) {
        Ok(cid) => {
            println!("{}: fresh clone resolved {cid}", kind.success());
            Outcome::Passed(())
        }
        Err(CheckError::Finding(detail)) => {
            eprintln!("{}: {detail}", kind.prefix());
            Outcome::Finding(Finding::reported(detail))
        }
        Err(CheckError::Unavailable(detail)) => {
            eprintln!("{}: {detail}", kind.prefix());
            Outcome::CouldNotCheck(CouldNotCheck::reported(detail, 2))
        }
    }
}

fn check(
    kind: PublicationKind,
    root: &Path,
    process: &dyn Process,
    self_test: bool,
) -> Result<String, CheckError> {
    let source = std::fs::read_to_string(root.join(kind.vector()))
        .map_err(|error| finding(format!("{}: {error}", kind.vector())))?;
    let vector: Value =
        serde_json::from_str(&source).map_err(|error| finding(error.to_string()))?;
    let temp = tempfile::Builder::new()
        .prefix(match kind {
            PublicationKind::Rfc0 => "day-rfc0-fresh-clone-",
            PublicationKind::Denotational => "day-rfc1-denotational-fresh-clone-",
        })
        .tempdir()
        .map_err(|error| unavailable(format!("could not create fresh-clone directory: {error}")))?;
    let checkout = temp.path().join("clone");
    let origin = canonical_origin(root, process)?;
    command(
        root,
        process,
        "git",
        [
            OsString::from("clone"),
            OsString::from("--quiet"),
            OsString::from("--no-local"),
            root.as_os_str().to_owned(),
            checkout.clone().into_os_string(),
        ],
    )?;
    require_projection(kind, &checkout, &vector)?;
    let subject = required_string(vector.get("subject"), "publication subject is absent")?;
    let author = required_string(vector.get("author"), "publication author is absent")?;
    let output = command(
        &checkout,
        process,
        "kan",
        ["show", subject, "--json", "--trust", author],
    )?;
    let envelope: Value =
        serde_json::from_str(&output).map_err(|error| finding(error.to_string()))?;
    let claims = envelope
        .get("claims")
        .and_then(Value::as_array)
        .ok_or_else(|| finding("kan publication response has no claims array"))?;
    validate(kind, &vector, &checkout, claims, &origin, process)?;
    if self_test {
        self_test_mutations(kind, &vector, &checkout, claims, &origin, process)?;
    }
    Ok(required_string(vector.get("claim_cid"), "publication claim CID is absent")?.to_owned())
}

fn validate(
    kind: PublicationKind,
    vector: &Value,
    checkout: &Path,
    claims: &[Value],
    repository_origin: &str,
    process: &dyn Process,
) -> Result<(), CheckError> {
    let version = if kind == PublicationKind::Rfc0 { 2 } else { 1 };
    require(
        vector.get("version").and_then(Value::as_i64) == Some(version),
        "publication vector version changed",
    )?;
    require(
        vector.get("claim_location").and_then(Value::as_str) == Some("external"),
        "claim is not external",
    )?;
    require(
        vector
            .get("normative_cid_embedding")
            .and_then(Value::as_str)
            == Some("forbidden"),
        "CID embedding is not forbidden",
    )?;
    require(
        vector.get("repository").and_then(Value::as_str) == Some(repository_origin),
        "fixture repository origin is wrong",
    )?;
    let path = required_string(vector.get("artifact_path"), "artifact path is absent")?;
    let commit = required_string(vector.get("artifact_commit"), "artifact commit is absent")?;
    let cid = required_string(vector.get("claim_cid"), "claim CID is absent")?;
    let subject = required_string(vector.get("subject"), "claim subject is absent")?;
    let claim = claims
        .iter()
        .find(|claim| claim.get("cid").and_then(Value::as_str) == Some(cid))
        .ok_or_else(|| {
            finding(if kind == PublicationKind::Rfc0 {
                "published claim CID does not resolve"
            } else {
                "published companion claim CID does not resolve"
            })
        })?;
    require(
        claim.get("kind").and_then(Value::as_str) == Some("Decision")
            && claim.get("subject").and_then(Value::as_str) == Some(subject),
        if kind == PublicationKind::Rfc0 {
            "claim kind or subject is wrong"
        } else {
            "companion claim kind or subject is wrong"
        },
    )?;
    let expected_file = format!("FileAt(\"{path}\", \"{commit}\")");
    require(
        strings(claim.get("artifacts")).contains(&expected_file.as_str()),
        "claim does not carry the exact commit/path address",
    )?;
    let committed = command(
        checkout,
        process,
        "git",
        ["show", &format!("{commit}:{path}")],
    )?;
    let digest = command_with_stdin(
        checkout,
        process,
        "shasum",
        ["-a", "256"],
        committed.as_bytes(),
    )?;
    require(
        digest.split_whitespace().next() == vector.get("artifact_sha256").and_then(Value::as_str),
        if kind == PublicationKind::Rfc0 {
            "addressed artifact bytes changed"
        } else {
            "addressed companion bytes changed"
        },
    )?;
    require(
        !committed
            .as_bytes()
            .windows(b"Kan-claim:".len())
            .any(|window| window == b"Kan-claim:"),
        if kind == PublicationKind::Rfc0 {
            "normative RFC bytes contain their own claim CID"
        } else {
            "normative companion bytes contain their own claim CID"
        },
    )?;
    if kind == PublicationKind::Rfc0 {
        let closed = claims.iter().any(|item| {
            item.get("kind").and_then(Value::as_str) == Some("Status")
                && item.get("status").and_then(Value::as_str) == Some("Closed")
                && strings(item.get("cites")).contains(&cid)
        });
        require(closed, "fixture claim has no closed status")?;
        require(
            claims
                .iter()
                .any(|item| item.get("kind").and_then(Value::as_str) == Some("Publication")),
            "fixture has no signed Publication claim",
        )?;
    } else {
        let addressed = format!("Commit(\"{commit}\")");
        require(
            claims.iter().any(|item| {
                item.get("kind").and_then(Value::as_str) == Some("Publication")
                    && strings(item.get("artifacts")).contains(&addressed.as_str())
            }),
            "companion has no Publication claim at the addressed commit",
        )?;
    }
    Ok(())
}

fn self_test_mutations(
    kind: PublicationKind,
    vector: &Value,
    checkout: &Path,
    claims: &[Value],
    origin: &str,
    process: &dyn Process,
) -> Result<(), CheckError> {
    for (name, field, value) in [
        ("cid", "claim_cid", Value::String("bafywrong".into())),
        ("commit", "artifact_commit", Value::String("0".repeat(40))),
        (
            "path",
            "artifact_path",
            Value::String(
                if kind == PublicationKind::Rfc0 {
                    "rfcs/fixtures/wrong.md"
                } else {
                    "rfcs/1/wrong.md"
                }
                .into(),
            ),
        ),
        ("bytes", "artifact_sha256", Value::String("0".repeat(64))),
        (
            "repository",
            "repository",
            Value::String("https://example.com/wrong.git".into()),
        ),
    ] {
        let mut candidate = vector.clone();
        candidate[field] = value;
        if validate(kind, &candidate, checkout, claims, origin, process).is_ok() {
            return Err(finding(format!("self-test accepted {name} mutation")));
        }
    }
    let without_publication = claims
        .iter()
        .filter(|claim| claim.get("kind").and_then(Value::as_str) != Some("Publication"))
        .cloned()
        .collect::<Vec<_>>();
    if validate(
        kind,
        vector,
        checkout,
        &without_publication,
        origin,
        process,
    )
    .is_ok()
    {
        return Err(finding("self-test accepted missing Publication claim"));
    }
    if kind == PublicationKind::Denotational {
        let projection = checkout.join(required_string(
            vector.get("projection_path"),
            "projection path is absent",
        )?);
        let hidden = projection.with_extension("hidden");
        std::fs::rename(&projection, &hidden).map_err(|error| unavailable(error.to_string()))?;
        let result = require_projection(kind, checkout, vector);
        std::fs::rename(&hidden, &projection).map_err(|error| unavailable(error.to_string()))?;
        if result.is_ok() {
            return Err(finding("self-test accepted missing companion projection"));
        }
    }
    Ok(())
}

fn require_projection(
    kind: PublicationKind,
    checkout: &Path,
    vector: &Value,
) -> Result<(), CheckError> {
    let projection = checkout.join(required_string(
        vector.get("projection_path"),
        "projection path is absent",
    )?);
    let present = projection.is_dir()
        && std::fs::read_dir(&projection)
            .map_err(|error| unavailable(error.to_string()))?
            .filter_map(Result::ok)
            .any(|entry| entry.path().extension() == Some(OsStr::new("md")));
    require(
        present,
        if kind == PublicationKind::Rfc0 {
            "published claim projection is absent from fresh clone"
        } else {
            "published companion projection is absent from fresh clone"
        },
    )
}

fn canonical_origin(root: &Path, process: &dyn Process) -> Result<String, CheckError> {
    let mut checkout = root.to_path_buf();
    let mut seen = BTreeSet::new();
    loop {
        let origin = command(&checkout, process, "git", ["remote", "get-url", "origin"])?;
        let origin = origin.trim();
        if origin.starts_with("https://")
            || origin.starts_with("ssh://")
            || origin.starts_with("git@")
        {
            return Ok(normalize_remote(origin));
        }
        let candidate = PathBuf::from(origin);
        let candidate = if candidate.is_absolute() {
            candidate
        } else {
            checkout.join(candidate)
        };
        let candidate = std::fs::canonicalize(candidate)
            .map_err(|_| finding("repository origin chain is invalid"))?;
        require(
            candidate.is_dir() && seen.insert(candidate.clone()),
            "repository origin chain is invalid",
        )?;
        checkout = candidate;
    }
}
fn normalize_remote(origin: &str) -> String {
    let mut value = origin.trim_end_matches('/').to_owned();
    if value.starts_with("https://github.com/") && !value.ends_with(".git") {
        value.push_str(".git");
    }
    value
}
fn command<I, S>(
    cwd: &Path,
    process: &dyn Process,
    program: &str,
    args: I,
) -> Result<String, CheckError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, cwd);
    let output = process.run(&request).map_err(unavailable)?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(finding(format!(
            "{}: {}",
            request.display(),
            output.stderr.trim()
        )))
    }
}
fn command_with_stdin<I, S>(
    cwd: &Path,
    process: &dyn Process,
    program: &str,
    args: I,
    input: &[u8],
) -> Result<String, CheckError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let request = ProcessRequest::new(program, args, cwd).with_stdin(input.to_vec());
    let output = process.run(&request).map_err(unavailable)?;
    if output.status == 0 {
        Ok(output.stdout)
    } else {
        Err(finding(format!(
            "{}: {}",
            request.display(),
            output.stderr.trim()
        )))
    }
}
fn strings(value: Option<&Value>) -> Vec<&str> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
fn required_string(value: Option<&Value>, detail: impl Into<String>) -> Result<&str, CheckError> {
    value
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| finding(detail))
}
fn require(condition: bool, detail: impl Into<String>) -> Result<(), CheckError> {
    condition.then_some(()).ok_or_else(|| finding(detail))
}
fn finding(detail: impl Into<String>) -> CheckError {
    CheckError::Finding(detail.into())
}
fn unavailable(detail: impl Into<String>) -> CheckError {
    CheckError::Unavailable(detail.into())
}
