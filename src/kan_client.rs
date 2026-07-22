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
    #[error(
        "could not read `{args}` output from kan: {detail}\n\nThis usually means kan's \
         --json shape changed. day pins to a shape version rather than parsing rendered \
         output, so this is an error instead of a silently empty result."
    )]
    Shape { args: String, detail: String },
    #[error("`{bin} {args}` failed ({status}){stderr}")]
    Failed {
        bin: String,
        args: String,
        status: String,
        stderr: String,
    },
}

/// The `--json` shape version day understands.
///
/// kan documents the shape as **versioned and additive-only**, and the
/// rendered form as free to change. day reads the structured form for
/// exactly that reason: it parsed the rendered form once, kan changed it,
/// and day read a full log as empty while reporting success.
///
/// Checked rather than assumed. A shape day does not know is an error with a
/// message, never a silently empty read — that failure mode is the one this
/// migration exists to end.
const SHAPE_VERSION: u32 = 1;

/// One live claim, from `kan show --json`.
///
/// Unknown fields are ignored by construction, which is what makes kan's
/// additive-only promise usable: a field added upstream cannot break day.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Claim {
    pub cid: String,
    pub kind: String,
    /// The claim's narrative text, when its body carries one (`Status`
    /// claims and relations do not).
    #[serde(default)]
    pub text: Option<String>,
    /// The declared subject title, present only on `Subject` claims. A
    /// subject's name is an rkey, not a label; this is what it's called.
    #[serde(default)]
    pub title: Option<String>,
    /// The signing DID. Exposed by `--json` and not by the rendered form;
    /// day#25's locally-signed injection scoping has no other way to tell
    /// whose claim it is reading.
    #[serde(default)]
    pub author: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct ShowEnvelope {
    v: u32,
    #[serde(default)]
    claims: Vec<Claim>,
}

#[derive(Debug, serde::Deserialize)]
struct SubjectsEnvelope {
    v: u32,
    #[serde(default)]
    subjects: Vec<SubjectEntry>,
}

#[derive(Debug, serde::Deserialize)]
struct SubjectEntry {
    subject: String,
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

    /// This workspace's identity, via `kan identity did`.
    ///
    /// `did` is the public identifier and is explicitly safe to share.
    /// **Never `kan identity phrase`**, which prints the recovery phrase for
    /// the signing key.
    ///
    /// Returns `None` rather than an error on any failure, deliberately.
    /// kan's identity access can block on a macOS keychain prompt that never
    /// arrives non-interactively — kan's own `src/sign.rs` documents this,
    /// and it silently emptied day's reads once already. A caller deciding
    /// whether to trust a claim needs a value it can branch on, not an error
    /// that aborts a hook, because the right response to "identity unknown"
    /// is to project nothing and say so.
    pub fn identity(&self) -> Option<String> {
        let did = self.run(&["identity", "did"]).ok()?.trim().to_string();
        (!did.is_empty()).then_some(did)
    }

    /// Every subject in the log, via `kan status --json`.
    pub fn subjects(&self) -> Result<Vec<String>, Error> {
        self.subject_names(&["status", "--json"])
    }

    /// Subjects that are not yet resolved, via `kan issues --json`.
    pub fn issues(&self) -> Result<Vec<String>, Error> {
        self.subject_names(&["issues", "--json"])
    }

    fn subject_names(&self, args: &[&str]) -> Result<Vec<String>, Error> {
        let out = self.run(args)?;
        let envelope: SubjectsEnvelope = parse(&out, args)?;
        check_shape(envelope.v, args)?;
        Ok(envelope.subjects.into_iter().map(|s| s.subject).collect())
    }

    /// A subject's live claims, via `kan show <subject> --json`.
    pub fn show(&self, subject: &str) -> Result<Vec<Claim>, Error> {
        let args = ["show", subject, "--json"];
        let out = self.run(&args)?;
        let envelope: ShowEnvelope = parse(&out, &args)?;
        check_shape(envelope.v, &args)?;
        Ok(envelope.claims)
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

/// Deserializes a `--json` envelope, naming the command when it fails.
///
/// A parse failure here is loud on purpose. The whole point of migrating off
/// the rendered form is that a shape day cannot read must never look like an
/// empty log.
fn parse<T: serde::de::DeserializeOwned>(out: &str, args: &[&str]) -> Result<T, Error> {
    serde_json::from_str(out).map_err(|source| Error::Shape {
        args: args.join(" "),
        detail: source.to_string(),
    })
}

/// Refuses a `--json` shape version day does not know.
fn check_shape(v: u32, args: &[&str]) -> Result<(), Error> {
    if v == SHAPE_VERSION {
        return Ok(());
    }
    Err(Error::Shape {
        args: args.join(" "),
        detail: format!(
            "kan reported --json shape v{v}; day understands v{SHAPE_VERSION}. \
             kan's shape is additive-only, so a higher version is usually readable — \
             but day will not guess, because guessing wrong reads as an empty log."
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shape day reads, exactly as `kan show --json` emits it.
    const SHOW: &str = r#"{
      "v": 1,
      "subject": "telos/a",
      "claims": [
        {"cid":"bafyreia","kind":"Decision","subject":"telos/a",
         "author":"did:key:zabc","text":"A telos.","cites":[],"artifacts":[]},
        {"cid":"bafyreib","kind":"Subject","subject":"telos/a",
         "author":"did:key:zabc","title":"A"},
        {"cid":"bafyreic","kind":"Relation","subject":"telos/a",
         "author":"did:key:zabc","relation":"InTensionWith","target":"telos/b"}
      ],
      "inbound": []
    }"#;

    #[test]
    fn claims_come_back_with_the_fields_day_reads() {
        let envelope: ShowEnvelope = parse(SHOW, &["show"]).expect("should parse");
        assert_eq!(envelope.v, SHAPE_VERSION);
        assert_eq!(envelope.claims.len(), 3);

        assert_eq!(envelope.claims[0].text.as_deref(), Some("A telos."));
        assert_eq!(envelope.claims[0].author.as_deref(), Some("did:key:zabc"));
        // A title rides on the Subject claim, not the narrative one.
        assert_eq!(envelope.claims[1].title.as_deref(), Some("A"));
        // Relations carry no narrative body, which is why a tension's reason
        // needs a subject of its own.
        assert_eq!(envelope.claims[2].text, None);
        assert_eq!(envelope.claims[2].kind, "Relation");
    }

    /// kan's shape is additive-only, so a field day has never heard of must
    /// not break it. This is the property that makes pinning a shape version
    /// safe rather than brittle.
    #[test]
    fn an_unknown_field_is_ignored_rather_than_fatal() {
        let json = r#"{"v":1,"claims":[
            {"cid":"bafyreia","kind":"Decision","text":"x","invented_later":{"a":1}}
        ]}"#;
        let envelope: ShowEnvelope = parse(json, &["show"]).expect("additive change must parse");
        assert_eq!(envelope.claims[0].text.as_deref(), Some("x"));
    }

    /// The failure this whole migration exists to end. day parsed kan's
    /// rendered output, kan changed it, and day reported an empty vocabulary
    /// against seven declared atoms at exit 0. Unreadable output must now be
    /// an error carrying the command that produced it.
    #[test]
    fn output_day_cannot_read_is_an_error_not_an_empty_result() {
        let err = parse::<ShowEnvelope>("telos/a (2 live claim(s)):", &["show", "telos/a"])
            .expect_err("rendered output must not parse as a shape");
        let rendered = err.to_string();
        assert!(rendered.contains("show telos/a"), "{rendered}");
        assert!(rendered.contains("silently empty"), "{rendered}");
    }

    /// A shape version day does not know is refused for the same reason.
    /// Additive-only means a higher version is *probably* readable, and
    /// "probably" is what produced a silently empty log once already.
    #[test]
    fn an_unknown_shape_version_is_refused() {
        assert!(check_shape(SHAPE_VERSION, &["status"]).is_ok());
        let err = check_shape(SHAPE_VERSION + 1, &["status"]).expect_err("unknown shape");
        assert!(err.to_string().contains("day understands"), "{err}");
    }

    #[test]
    fn subject_lists_come_back_in_order() {
        let json = r#"{"v":1,"subjects":[
            {"subject":"atom/design","state":"Unclassified"},
            {"subject":"telos/a","state":"Settled","value":"Open"}
        ]}"#;
        let envelope: SubjectsEnvelope = parse(json, &["status"]).expect("should parse");
        let names: Vec<String> = envelope.subjects.into_iter().map(|s| s.subject).collect();
        assert_eq!(names, vec!["atom/design", "telos/a"]);
    }
}
