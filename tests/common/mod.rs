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
}

pub fn claim(subject: &str, cid: &str, text: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Observation".to_string(),
        text: text.to_string(),
        author: STUB_AUTHOR.to_string(),
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
    f="$DATA/show-$(printf '%s' "$2" | tr '/' '_').json"
    if [ -f "$f" ]; then cat "$f"; else printf '{{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}}\n' "$2"; fi
    exit 0 ;;
  issues) cat "$DATA/issues.json" 2>/dev/null; exit 0 ;;
  observe|plan|decide|result|resolve)
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
    python3 "$DATA/append.py" "$DATA" "$subj" "$cid" "$text"

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
    value
}

fn show_filename(subject: &str) -> String {
    format!("show-{}.json", subject.replace('/', "_"))
}

/// Path to a binary that does not exist, for the "kan is absent" cases.
pub fn missing_kan(dir: &Path) -> PathBuf {
    dir.join("definitely-not-installed-kan")
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
const STUB_APPEND_PY: &str = r#"
import json, os, sys

data, subj, cid, text = sys.argv[1:5]
relation = sys.argv[5] if len(sys.argv) > 5 else None
target = sys.argv[6] if len(sys.argv) > 6 else None

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
    claim["kind"] = "Observation"
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
