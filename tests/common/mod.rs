//! A stub `kan` binary, so day's integration tests exercise real subprocess
//! wiring without requiring a kan install (or a kan log) in CI.
//!
//! The stub emits kan's **`--json` shape**, which is what day reads. kan
//! documents that shape as versioned and additive-only, and the rendered
//! form as free to change — day parsed the rendered form once, kan changed
//! it, and day read a full log as empty at exit 0. The stub models the
//! contract day actually depends on, so a divergence surfaces here.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// The DID the stub signs claims with, and reports from `kan identity did`.
pub const STUB_AUTHOR: &str = "did:key:zStubAuthor";

/// A `day-atom` block body **this build cannot read**, because it declares a
/// version above the one this build supports.
///
/// Derived, never hardcoded. Nine fixtures across three files spelled this
/// `{"_version":2, …}`, which was too new for as long as `day-atom` was v1 and
/// stopped being too new the moment day#113 shipped v2. Five tests failed
/// loudly and told us; the ones that would have kept passing are the reason
/// this is a function. A fixture whose whole job is to be ahead of the reader
/// has to be *defined* as ahead of the reader — CLAUDE.md's rule that a fixture
/// must reach the mode the defect lives in, applied to the fixture itself.
pub fn too_new_atom_body() -> String {
    use day::atoms::Versioned;
    format!(
        r#"{{"_version":{},"in":["a"],"out":["b"]}}"#,
        day::atoms::Interface::SUPPORTED_VERSION + 1
    )
}

/// One canned claim on one subject.
#[derive(Clone)]
pub struct StubClaim {
    pub subject: String,
    pub cid: String,
    pub kind: String,
    pub text: String,
    /// Who signed it. Defaults to [`STUB_AUTHOR`]; set it to anything else
    /// to model a claim from another actor.
    pub author: String,
    /// When kan recorded it, **microseconds since the epoch** — the integer
    /// real kan emits. `None` omits the field, modelling a kan (or shape) that
    /// does not emit it.
    pub recorded_at: Option<i64>,
}

pub fn claim(subject: &str, cid: &str, text: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Observation".to_string(),
        text: text.to_string(),
        author: STUB_AUTHOR.to_string(),
        recorded_at: None,
    }
}

/// A `Result` claim — an assessment (`kan result`) — recorded at a given time.
/// `recorded_at` is what orders assessments across subjects, so a test that
/// needs "the last one" sets distinct timestamps.
pub fn result_claim(subject: &str, cid: &str, text: &str, recorded_at: i64) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Result".to_string(),
        text: text.to_string(),
        author: STUB_AUTHOR.to_string(),
        recorded_at: Some(recorded_at),
    }
}

/// A `Decision` claim recorded at a given time — what `day review record`
/// appends for a verdict, and what a `claim` probe narrowed by a text marker
/// has to tell apart from every other decision in the log.
pub fn decision_claim(subject: &str, cid: &str, text: &str, recorded_at: i64) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Decision".to_string(),
        text: text.to_string(),
        author: STUB_AUTHOR.to_string(),
        recorded_at: Some(recorded_at),
    }
}

/// Removes the stub's identity, modelling kan being unable to establish it —
/// a blocked keychain, a missing key. day must fail closed here.
pub fn without_identity(dir: &Path) {
    let _ = std::fs::remove_file(dir.join("kan-stub-data").join("identity"));
}

/// A `Subject` claim, which carries a `title` field instead of `text` —
/// what `kan <verb> --title --kind` appends alongside a narrative claim.
pub fn subject_claim(subject: &str, cid: &str, title: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Subject".to_string(),
        text: title.to_string(),
        author: STUB_AUTHOR.to_string(),
        recorded_at: None,
    }
}

/// A `Retraction` claim, which carries neither text nor title — what a
/// subject looks like once everything on it has been retracted.
pub fn retraction_claim(subject: &str, cid: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Retraction".to_string(),
        text: String::new(),
        author: STUB_AUTHOR.to_string(),
        recorded_at: None,
    }
}

/// An `atom/<slug>` claim carrying a `day-atom` interface block, written the
/// way `docs/CONVENTIONS.md` tells a human or agent to write one.
pub fn atom_claim(
    slug: &str,
    cid: &str,
    inputs: &[&str],
    outputs: &[&str],
    next: &[&str],
) -> StubClaim {
    let list = |xs: &[&str]| {
        xs.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let text = format!(
        "The {slug} atom.\n\n```day-atom\n{{\"in\": [{}], \"out\": [{}], \"next\": [{}]}}\n```\n",
        list(inputs),
        list(outputs),
        list(next),
    );
    claim(&format!("atom/{slug}"), cid, &text)
}

/// Writes a `kan` stub into `dir` and returns its path, ready to hand to
/// day through `DAY_KAN_BIN`.
pub fn write_kan_stub(dir: &Path, claims: &[StubClaim]) -> PathBuf {
    let data = dir.join("kan-stub-data");
    std::fs::create_dir_all(&data).unwrap();
    // Re-stubbing means "start from this log state", so any writes recorded
    // against the previous stub are cleared — otherwise a test that stubs
    // twice sees the first phase's appends in the second phase's assertions.
    let _ = std::fs::remove_file(data.join("appends.log"));
    // The stub signs everything as this DID, so a test can make a claim
    // "foreign" simply by declaring it with a different author.
    std::fs::write(data.join("identity"), STUB_AUTHOR).unwrap();
    let _ = std::fs::remove_file(data.join("append-count"));

    let mut subjects: Vec<&str> = claims.iter().map(|c| c.subject.as_str()).collect();
    subjects.sort_unstable();
    subjects.dedup();

    // `kan status --json` / `kan issues --json`: an envelope of subjects.
    // The stub has no status model, so every subject is open and issues
    // returns the same set.
    let status = serde_json::json!({
        "v": 1,
        "subjects": subjects
            .iter()
            .map(|s| serde_json::json!({"subject": s, "subjects": [s], "state": "Unclassified"}))
            .collect::<Vec<_>>(),
    });
    let status = serde_json::to_string_pretty(&status).unwrap();
    std::fs::write(data.join("status.json"), &status).unwrap();
    std::fs::write(data.join("issues.json"), &status).unwrap();

    // `kan show <subject>`: header line, then one line per live claim,
    // oldest first — the order day relies on to pick the newest interface.
    for subject in &subjects {
        let for_subject: Vec<&StubClaim> =
            claims.iter().filter(|c| c.subject == *subject).collect();
        let show = serde_json::json!({
            "v": 1,
            "subject": subject,
            "subjects": [subject],
            "claims": for_subject.iter().map(|c| claim_json(c)).collect::<Vec<_>>(),
            "inbound": [],
        });
        std::fs::write(
            data.join(show_filename(subject)),
            serde_json::to_string_pretty(&show).unwrap(),
        )
        .unwrap();
    }

    // The read-back helper lives in its own file rather than inline in the
    // shell below: it builds kan's JSON shape, and JSON braces inside a
    // `format!` string would need doubling everywhere, which is exactly the
    // kind of escaping that hides mistakes.
    std::fs::write(data.join("append.py"), STUB_APPEND_PY).unwrap();
    // `kan show --all --json` (kan#123 / ADR-71), which `ClaimLog` uses since
    // day#71. Built by globbing the per-subject files at CALL time rather than
    // being written once here, so a subject created by an append during the
    // test is visible to the bulk read too — a stub whose bulk read went stale
    // the moment day wrote anything would test a log that cannot exist.
    std::fs::write(data.join("show_all.py"), STUB_SHOW_ALL_PY).unwrap();

    let script = dir.join("kan-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
DATA="{data}"
case "$1" in
  --help) echo "kan (test stub)"; exit 0 ;;
  identity)
    # `kan identity did` prints the public identifier. A stub whose identity
    # file is absent models kan being unable to reach the keychain, which is
    # a real state day has to fail closed on rather than guess through.
    if [ -f "$DATA/identity" ]; then cat "$DATA/identity"; exit 0; fi
    echo "identity unavailable" >&2; exit 1 ;;
  status) cat "$DATA/status.json"; exit 0 ;;
  show)
    if [ "$2" = "--all" ]; then python3 "$DATA/show_all.py" "$DATA"; exit 0; fi
    f="$DATA/show-$(printf '%s' "$2" | tr '/' '_').json"
    if [ -f "$f" ]; then cat "$f"; else printf '{{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}}\n' "$2"; fi
    exit 0 ;;
  issues) cat "$DATA/issues.json" 2>/dev/null; exit 0 ;;
  observe|plan|decide|result|resolve)
    verb="$1"
    # Log the whole invocation so tests can assert on the chain day built,
    # then print a CID the way kan does, since day chains on that output.
    # Records are separated by a marker, not by newlines: claim text is
    # routinely multi-line (a fenced interface block is), so one-line-per-
    # append would split a single write across several records.
    n=$(cat "$DATA/append-count" 2>/dev/null || echo 0)
    n=$((n + 1))
    printf '%s' "$n" > "$DATA/append-count"
    printf '%s\n<<<END-OF-APPEND>>>\n' "$*" >> "$DATA/appends.log"
    cid=$(printf 'bafyreistub%08d' "$n")

    # An append is then readable: without this the stub is write-only, and
    # any behavior that writes and then reads back (declaring an atom, then
    # checking whether the vocabulary composes) is untestable against it.
    shift
    text="$1"
    subj="general"
    while [ $# -gt 0 ]; do
      if [ "$1" = "--subject" ]; then subj="$2"; fi
      shift
    done
    python3 "$DATA/append.py" "$DATA" "$subj" "$cid" "$text" "" "" "$verb"

    printf '%s\n' "$cid"
    exit 0 ;;
  relate)
    # `kan relate <A> <KIND> <B>` — two positional subjects, no text. The
    # shape differs from the append verbs above on purpose: that asymmetry
    # is real in kan (kan#78), and a stub that quietly accepted day's append
    # shape here would hide exactly the class of bug day#27 exists to catch.
    n=$(cat "$DATA/append-count" 2>/dev/null || echo 0)
    n=$((n + 1))
    printf '%s' "$n" > "$DATA/append-count"
    printf '%s\n<<<END-OF-APPEND>>>\n' "$*" >> "$DATA/appends.log"
    cid=$(printf 'bafyreistub%08d' "$n")

    # Readable afterwards, from the SOURCE subject only — kan's relation is
    # directed and `kan show <target>` does not surface an edge pointing at
    # it. Mirroring that here keeps the stub from implying a symmetry the
    # real binary does not have.
    python3 "$DATA/append.py" "$DATA" "$2" "$cid" "" "$3" "$4"

    printf '%s\n' "$cid"
    exit 0 ;;
  *) echo "kan stub: unsupported command $1" >&2; exit 1 ;;
esac
"#,
            data = data.display(),
        ),
    )
    .unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    script
}

/// One claim in kan's `--json` shape. Which fields are present depends on
/// the body: narrative claims carry `text`, `Subject` claims carry `title`,
/// and a `Retraction` carries neither — day has to cope with all three, so
/// the stub emits all three faithfully rather than always filling `text`.
fn claim_json(claim: &StubClaim) -> serde_json::Value {
    let mut value = serde_json::json!({
        "cid": claim.cid,
        "kind": claim.kind,
        "subject": claim.subject,
        "author": claim.author,
    });
    let map = value.as_object_mut().unwrap();
    match claim.kind.as_str() {
        "Retraction" => {
            map.insert("supersedes".into(), claim.cid.clone().into());
        }
        "Subject" => {
            map.insert("title".into(), claim.text.clone().into());
        }
        _ => {
            map.insert("text".into(), claim.text.clone().into());
        }
    }
    // Emitted as the integer real kan uses (microseconds since epoch), and
    // omitted when absent — so the stub models both a kan that carries
    // `recorded_at` and one (or a shape) that does not.
    if let Some(at) = claim.recorded_at {
        map.insert("recorded_at".into(), at.into());
    }
    value
}

fn show_filename(subject: &str) -> String {
    format!("show-{}.json", subject.replace('/', "_"))
}

/// Path to a binary that does not exist, for the "kan is absent" cases.
pub fn missing_kan(dir: &Path) -> PathBuf {
    dir.join("definitely-not-installed-kan")
}

/// A kan that **runs but cannot read this repo's log**: `--help` succeeds, so
/// `KanClient::probe` is satisfied, and every read verb fails.
///
/// This is day#95's actual state, and it is not the same as kan being absent.
/// It happens in a git repo with no commits, where kan cannot derive repo
/// identity — and it is the state in which `day status` exited 2 against a
/// documented "always exits zero", and `day init` printed `kan: reachable`
/// while `day doctor` correctly reported the opposite. Neither defect is
/// reachable with [`missing_kan`], because `probe` fails first and both verbs
/// take an earlier path.
pub fn unreadable_kan(dir: &Path) -> PathBuf {
    let script = dir.join("kan-unreadable.sh");
    std::fs::write(
        &script,
        "#!/bin/sh\ncase \"$1\" in\n  --help) echo \"kan (test stub)\"; exit 0 ;;\n  \
         *) echo \"could not derive repo identity\" >&2; exit 1 ;;\nesac\n",
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    script
}

/// Every write the stub kan received, one entry per invocation, in order.
/// Entries may span multiple lines — a claim carrying a fenced interface
/// block does — so they are split on the stub's record marker.
pub fn appends(dir: &Path) -> Vec<String> {
    std::fs::read_to_string(dir.join("kan-stub-data").join("appends.log"))
        .map(|s| {
            s.split("<<<END-OF-APPEND>>>")
                .map(str::trim)
                .filter(|e| !e.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// A `schema/<slug>` claim carrying day's own starter schema, so tests
/// validate against the same shape day suggests to users rather than a
/// fixture that could drift from it.
pub fn schema_claim(slug: &str, cid: &str) -> StubClaim {
    let json = serde_json::to_string(&day::schema::Schema::starter()).unwrap();
    claim(
        &format!("schema/{slug}"),
        cid,
        &format!("Design-doc schema.\n\n```day-schema\n{json}\n```\n"),
    )
}

/// The repo root, so tests can assert on shipped plugin/doc files.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Appends a claim to the stub's JSON so a write is readable afterwards.
///
/// Without this the stub is write-only, and anything that writes then reads
/// back — declaring an atom, then checking the vocabulary composes — cannot
/// be tested against it.
/// `kan show --all --json`: every subject's live claims in one envelope.
///
/// Each entry is a full `ShowJson`, matching what ADR-71 actually emits —
/// repeated `trust` field and all — because day parses these with the same
/// parser it uses for a single subject, and a slimmer stub entry would let day
/// pass here while failing against the real binary.
const STUB_SHOW_ALL_PY: &str = r#"
import json, sys, pathlib

data = pathlib.Path(sys.argv[1])
entries = []
for f in sorted(data.glob("show-*.json")):
    entry = json.loads(f.read_text())
    entry.setdefault("trust", {"base": "Solo", "authors": []})
    entry.setdefault("excluded_by_trust", 0)
    entries.append(entry)
print(json.dumps({
    "v": 1,
    "trust": {"base": "Solo", "authors": []},
    "excluded_by_trust": 0,
    "subjects": entries,
}))
"#;

const STUB_APPEND_PY: &str = r#"
import json, os, sys

data, subj, cid, text = sys.argv[1:5]
relation = sys.argv[5] if len(sys.argv) > 5 and sys.argv[5] else None
target = sys.argv[6] if len(sys.argv) > 6 and sys.argv[6] else None
verb = sys.argv[7] if len(sys.argv) > 7 else "observe"

# The ClaimKind kan actually produces for each write verb. The stub used to
# record EVERYTHING as an Observation, which is a fidelity gap of exactly the
# kind `tests/kan_conformance.rs` exists to catch: day filters claims by kind,
# so a stub that flattens every kind lets a kind-sensitive bug pass. Found when
# day#36's incremental recording read back its own `decide` claims and saw none.
KIND_FOR_VERB = {
    "observe": "Observation",
    "plan": "Plan",
    "decide": "Decision",
    "result": "Result",
    "resolve": "Result",
}

path = os.path.join(data, "show-%s.json" % subj.replace("/", "_"))
if os.path.exists(path):
    with open(path) as fh:
        doc = json.load(fh)
else:
    doc = {"v": 1, "subject": subj, "subjects": [subj], "claims": [], "inbound": []}

claim = {"cid": cid, "subject": subj, "author": "did:key:zStubAuthor"}
if relation:
    # A relation carries no narrative body -- the property that made a
    # tension's reason need a subject of its own.
    claim["kind"] = "Relation"
    claim["relation"] = relation
    claim["target"] = target
else:
    claim["kind"] = KIND_FOR_VERB.get(verb, "Observation")
    claim["text"] = text
doc["claims"].append(claim)

with open(path, "w") as fh:
    json.dump(doc, fh)

status_path = os.path.join(data, "status.json")
if os.path.exists(status_path):
    with open(status_path) as fh:
        status = json.load(fh)
else:
    status = {"v": 1, "subjects": []}
if not any(s["subject"] == subj for s in status["subjects"]):
    status["subjects"].append(
        {"subject": subj, "subjects": [subj], "state": "Unclassified"}
    )
    with open(status_path, "w") as fh:
        json.dump(status, fh)
"#;

/// A throwaway Cargo crate in a temp dir, for driving the harnesses in
/// `scripts/` end to end.
///
/// **The harnesses are tested against a real crate and a real cargo**, never
/// against a mocked one. Both `mutate.py` and `revert-demo.py` decide what they
/// report by reading libtest's output, so a mock would validate each harness
/// against its author's idea of that output — the stub-shaped blind spot
/// `tests/kan_conformance.rs` exists as the deliberate exception to. Each
/// scratch crate has no dependencies, so a scenario costs one trivial compile.
pub struct ScratchCrate {
    dir: tempfile::TempDir,
}

impl ScratchCrate {
    /// A crate with a `Cargo.toml` and nothing else. `[workspace]` is declared
    /// so the crate is not silently adopted into an enclosing workspace.
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("a scratch dir");
        let me = Self { dir };
        me.write(
            "Cargo.toml",
            "[package]\nname = \"scratch\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\
             [workspace]\n",
        );
        me
    }

    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    pub fn write(&self, rel: &str, contents: &str) -> &Self {
        let path = self.root().join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, contents).unwrap();
        self
    }

    pub fn read(&self, rel: &str) -> String {
        std::fs::read_to_string(self.root().join(rel))
            .unwrap_or_else(|e| panic!("{rel} should be readable: {e}"))
    }

    pub fn git(&self, args: &[&str]) -> String {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(self.root())
            .output()
            .expect("git should be runnable");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).to_string()
    }

    /// An initialised repo with one commit of whatever has been written so far.
    pub fn commit_all(&self, message: &str) -> &Self {
        if !self.root().join(".git").exists() {
            self.git(&["init", "-q", "-b", "main"]);
            self.git(&["config", "user.email", "t@example.com"]);
            self.git(&["config", "user.name", "t"]);
        }
        self.git(&["add", "-A"]);
        self.git(&["commit", "-qm", message]);
        self
    }

    /// Run one of day's own `scripts/` against this crate, and return its
    /// combined output plus whether it exited 0.
    pub fn run_script(&self, script: &str, args: &[&str]) -> (String, bool) {
        let path = repo_root().join("scripts").join(script);
        let out = std::process::Command::new("python3")
            .arg(&path)
            .args(args)
            .current_dir(self.root())
            // A scratch crate must build into its OWN tree, whatever the
            // ambient environment says. `revert-demo.py --verify` sets
            // `CARGO_TARGET_DIR` so a worktree shares the caller's artifact
            // cache, and that variable is inherited all the way down to the
            // scratch crate's cargo — so `<scratch>/target/debug/scratch` was
            // never created and the mutation test failed for a reason that had
            // nothing to do with mutation.
            //
            // Found by running `--verify` on the commit that added these tests,
            // which is the harness catching a test of the harness. It is day#91
            // exactly: a mode this repo is never in until something puts it
            // there.
            .env_remove("CARGO_TARGET_DIR")
            .output()
            .expect("python3 should be runnable");
        (
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
            out.status.success(),
        )
    }
}

impl Default for ScratchCrate {
    fn default() -> Self {
        Self::new()
    }
}
