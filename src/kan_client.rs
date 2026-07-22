//! day's only integration with kan is kan's public CLI (ADR-18: the
//! companion tool consumes kan via its CLI/MCP, it does not link kan as a
//! library or touch its data model). Every read here is a subprocess call to
//! the `kan` binary, parsed from its stdout — the same shape kan's own
//! `GitAncestry` provider uses for git.
//!
//! Nothing in this module appends, retracts, or rejects a claim. day records
//! claims by *instructing* an agent to call kan's write verbs (the commands
//! do this); the binary itself only ever runs kan's read verbs, so there is
//! no path by which day can alter or destroy a subject.
//!
//! One honest caveat: kan initializes its own workspace (`.kan/`) on first
//! use, so running a kan read verb in a repo kan has never seen creates an
//! empty log there. That is kan's behavior, not day's, and it touches no
//! claims — but it does mean "day never causes a write to disk" would be
//! too strong a claim to make.

use std::path::PathBuf;
use std::process::Command;

/// Overrides the `kan` binary day shells out to. Exists so tests can point
/// at a stub emitting canned `kan` output instead of requiring a real kan
/// install in CI.
pub const KAN_BIN_ENV: &str = "DAY_KAN_BIN";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kan is not reachable (tried to run `{bin}`): {source}\nInstall it with `cargo install kan`, or set {KAN_BIN_ENV} to its path.")]
    NotReachable {
        bin: String,
        #[source]
        source: std::io::Error,
    },
    #[error("`{bin} {args}` failed ({status}){stderr}")]
    Failed {
        bin: String,
        args: String,
        status: String,
        stderr: String,
    },
}

/// One live claim as `kan show` prints it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub cid: String,
    pub kind: String,
    /// The claim's narrative text, when its body carries one (`Status`
    /// claims and relations do not).
    pub text: Option<String>,
    /// The declared subject title, present only on `Subject` claims. A
    /// subject's name is an rkey, not a label; this is what it's called.
    pub title: Option<String>,
}

pub struct KanClient {
    bin: String,
    cwd: PathBuf,
}

impl KanClient {
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        let bin = std::env::var(KAN_BIN_ENV).unwrap_or_else(|_| "kan".to_string());
        Self {
            bin,
            cwd: cwd.into(),
        }
    }

    pub fn with_bin(cwd: impl Into<PathBuf>, bin: impl Into<String>) -> Self {
        Self {
            bin: bin.into(),
            cwd: cwd.into(),
        }
    }

    pub fn bin(&self) -> &str {
        &self.bin
    }

    fn run(&self, args: &[&str]) -> Result<String, Error> {
        let output = Command::new(&self.bin)
            .args(args)
            .current_dir(&self.cwd)
            .output()
            .map_err(|source| Error::NotReachable {
                bin: self.bin.clone(),
                source,
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(Error::Failed {
                bin: self.bin.clone(),
                args: args.join(" "),
                status: output.status.to_string(),
                stderr: if stderr.is_empty() {
                    stderr
                } else {
                    format!(": {stderr}")
                },
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Cheapest possible reachability check — `kan --help` touches no
    /// workspace state, so it distinguishes "kan isn't installed" from
    /// "kan is installed but this isn't a kan repo".
    pub fn probe(&self) -> Result<(), Error> {
        self.run(&["--help"]).map(|_| ())
    }

    /// Every subject in the log, via bare `kan status` (which prints one
    /// line per subject).
    pub fn subjects(&self) -> Result<Vec<String>, Error> {
        let out = self.run(&["status"])?;
        Ok(out.lines().filter_map(parse_subject_line).collect())
    }

    /// Subjects that are not yet resolved, via `kan issues` — which prints
    /// the same `[Local("subject")]` line shape `status` does.
    pub fn issues(&self) -> Result<Vec<String>, Error> {
        let out = self.run(&["issues"])?;
        Ok(out.lines().filter_map(parse_subject_line).collect())
    }

    /// A subject's live claims, via `kan show <subject>`.
    pub fn show(&self, subject: &str) -> Result<Vec<Claim>, Error> {
        let out = self.run(&["show", subject])?;
        Ok(out.lines().filter_map(parse_claim_line).collect())
    }

    /// Appends a narrative claim through kan's own write verb and returns
    /// the CID kan prints.
    ///
    /// This is the v0.2 invariant, stated precisely (`docs/ROADMAP.md`): day
    /// writes, but only ever by invoking kan's public verbs. kan signs,
    /// content-addresses, and owns the log format; day never touches storage
    /// and still has no destroy path, because kan exposes none to reach.
    ///
    /// Chaining is the point. day assembles `--cites` from CIDs it captured
    /// itself, which makes the "pass a file path to `--cites`" class of error
    /// unreachable rather than merely documented against — that bug existed
    /// in the prose instructions this replaces.
    pub fn append(&self, write: Write<'_>) -> Result<String, Error> {
        let mut args: Vec<&str> = vec![write.verb, write.text];
        args.push("--subject");
        args.push(write.subject);
        for cid in write.cites {
            args.push("--cites");
            args.push(cid);
        }
        if let (Some(title), Some(kind)) = (write.title, write.kind) {
            args.extend_from_slice(&["--title", title, "--kind", kind]);
        }
        Ok(self.run(&args)?.trim().to_string())
    }

    /// Asserts a domain-semantic edge between two subjects, via
    /// `kan relate <A> <KIND> <B>`.
    ///
    /// Deliberately **not** routed through [`Self::append`]. That method
    /// builds `<verb> <text> --subject <s>`, and `kan relate` takes its two
    /// subjects positionally with no text at all — the same argument-shape
    /// asymmetry that put a command which does not run into
    /// `docs/CONVENTIONS.md` for several releases (day#27, kan#78). A verb
    /// with a different shape gets its own method;
    /// `tests/kan_conformance.rs` enforces that.
    ///
    /// A relation carries no narrative body, so whatever *reason* the edge
    /// has must live in a claim the edge cites. Callers pass that CID here.
    pub fn relate(&self, a: &str, kind: &str, b: &str, cites: &[String]) -> Result<String, Error> {
        let mut args: Vec<&str> = vec!["relate", a, kind, b];
        for cid in cites {
            args.push("--cites");
            args.push(cid);
        }
        Ok(self.run(&args)?.trim().to_string())
    }
}

/// One append, as arguments rather than a long parameter list — the write
/// verbs differ only in which kan verb they invoke.
pub struct Write<'a> {
    pub verb: &'a str,
    pub text: &'a str,
    pub subject: &'a str,
    pub cites: &'a [String],
    pub title: Option<&'a str>,
    pub kind: Option<&'a str>,
}

impl<'a> Write<'a> {
    pub fn new(verb: &'a str, subject: &'a str, text: &'a str) -> Self {
        Self {
            verb,
            text,
            subject,
            cites: &[],
            title: None,
            kind: None,
        }
    }

    pub fn cites(mut self, cites: &'a [String]) -> Self {
        self.cites = cites;
        self
    }

    pub fn declaring(mut self, title: &'a str, kind: &'a str) -> Self {
        self.title = Some(title);
        self.kind = Some(kind);
        self
    }
}

/// `kan status` prints `[Local("subject")]: Kind — body  (cid)`. Only the
/// subject name is wanted here; the trailing summary is `kan`'s own
/// rendering and day never re-interprets it.
fn parse_subject_line(line: &str) -> Option<String> {
    let line = line.trim();
    if !line.starts_with('[') {
        return None;
    }
    let start = line.find("(\"")? + 2;
    let rest = &line[start..];
    let end = rest.find("\")")?;
    Some(rest[..end].to_string())
}

/// `kan show` prints a header line then `  <cid>  <Kind>  <Debug body>` per
/// claim. The body is Rust `Debug` output, so any `text:` field inside it is
/// escaped — [`unescape_debug_string`] undoes that.
fn parse_claim_line(line: &str) -> Option<Claim> {
    if !line.starts_with("  ") {
        return None;
    }
    // Fields are separated by whitespace *runs*, so `splitn` on a single
    // whitespace char would yield empty fields between kan's double spaces.
    let mut rest = line.trim();
    let (cid, tail) = split_once_whitespace(rest)?;
    if !cid.starts_with("bafy") {
        return None;
    }
    rest = tail;
    let (kind, body) = split_once_whitespace(rest)?;
    let (cid, kind) = (cid.to_string(), kind.to_string());
    Some(Claim {
        cid,
        kind,
        text: extract_debug_field(body, "text"),
        title: extract_debug_field(body, "title"),
    })
}

/// Splits off the first whitespace-delimited field, returning it and the
/// remainder with leading whitespace trimmed.
fn split_once_whitespace(s: &str) -> Option<(&str, &str)> {
    let end = s.find(char::is_whitespace)?;
    Some((&s[..end], s[end..].trim_start()))
}

/// Pulls a named string field out of a `Debug`-rendered claim body,
/// respecting backslash escapes when hunting for the closing quote.
fn extract_debug_field(body: &str, field: &str) -> Option<String> {
    let needle = format!("{field}: \"");
    let start = body.find(&needle)? + needle.len();
    let rest = &body[start..];
    let mut escaped = false;
    for (i, c) in rest.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' => escaped = true,
            '"' => return Some(unescape_debug_string(&rest[..i])),
            _ => {}
        }
    }
    None
}

/// Inverse of Rust's `Debug` string escaping, enough of it for claim text:
/// `\"`, `\\`, `\n`, `\r`, `\t`, `\0`, `\'`, and `\u{...}`. Unknown escapes
/// pass through with the backslash dropped rather than erroring — a claim
/// whose text day can't perfectly round-trip is still worth surfacing.
fn unescape_debug_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('0') => out.push('\0'),
            Some('u') => {
                // \u{1f600}
                let mut hex = String::new();
                if chars.next() == Some('{') {
                    for c in chars.by_ref() {
                        if c == '}' {
                            break;
                        }
                        hex.push(c);
                    }
                }
                match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                    Some(c) => out.push(c),
                    None => out.push('\u{fffd}'),
                }
            }
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_status_subject_line() {
        let line = r#"[Local("process-layer")]: Decision — Decision { text: "x" }  (bafyrei)"#;
        assert_eq!(parse_subject_line(line).as_deref(), Some("process-layer"));
    }

    #[test]
    fn ignores_the_show_header_line() {
        assert_eq!(parse_claim_line("process-layer (5 live claim(s)):"), None);
    }

    #[test]
    fn parses_a_claim_line_with_escaped_text() {
        let line =
            r#"  bafyreiabc  Observation  Observation { text: "he said \"hi\"\nthen left" }"#;
        let claim = parse_claim_line(line).expect("claim line");
        assert_eq!(claim.cid, "bafyreiabc");
        assert_eq!(claim.kind, "Observation");
        assert_eq!(claim.text.as_deref(), Some("he said \"hi\"\nthen left"));
    }

    #[test]
    fn parses_a_claim_body_with_no_text_field() {
        let line = "  bafyreiabc  Status  Status { value: Resolved }";
        let claim = parse_claim_line(line).expect("claim line");
        assert_eq!(claim.text, None);
    }
}
