//! A stub `kan` binary, so day's integration tests exercise real subprocess
//! wiring without requiring a kan install (or a kan log) in CI.
//!
//! The stub emits byte-exact `kan` output: claim bodies are rendered through
//! Rust's own `Debug` formatting, which is what day's parser has to survive,
//! so a change in kan's escaping would surface here rather than silently.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// One canned claim on one subject.
pub struct StubClaim {
    pub subject: String,
    pub cid: String,
    pub kind: String,
    pub text: String,
}

pub fn claim(subject: &str, cid: &str, text: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Observation".to_string(),
        text: text.to_string(),
    }
}

/// A `Subject` claim, which carries a `title` field instead of `text` —
/// what `kan <verb> --title --kind` appends alongside a narrative claim.
pub fn subject_claim(subject: &str, cid: &str, title: &str) -> StubClaim {
    StubClaim {
        subject: subject.to_string(),
        cid: cid.to_string(),
        kind: "Subject".to_string(),
        text: title.to_string(),
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
    let _ = std::fs::remove_file(data.join("append-count"));

    let mut subjects: Vec<&str> = claims.iter().map(|c| c.subject.as_str()).collect();
    subjects.sort_unstable();
    subjects.dedup();

    // `kan status` with no argument: one line per subject.
    let mut status = String::new();
    for subject in &subjects {
        let last = claims.iter().rev().find(|c| c.subject == *subject).unwrap();
        status.push_str(&format!(
            "[Local({:?})]: {} — {}  ({})\n",
            subject,
            last.kind,
            debug_body(last),
            last.cid,
        ));
    }
    std::fs::write(data.join("status.txt"), &status).unwrap();
    // `kan issues` prints the same line shape for the subset that isn't
    // resolved; the stub has no status model, so every subject is open.
    std::fs::write(data.join("issues.txt"), &status).unwrap();

    // `kan show <subject>`: header line, then one line per live claim,
    // oldest first — the order day relies on to pick the newest interface.
    for subject in &subjects {
        let for_subject: Vec<&StubClaim> =
            claims.iter().filter(|c| c.subject == *subject).collect();
        let mut show = format!("{subject} ({} live claim(s)):\n", for_subject.len());
        for c in for_subject {
            show.push_str(&format!("  {}  {}  {}\n", c.cid, c.kind, debug_body(c)));
        }
        std::fs::write(data.join(show_filename(subject)), show).unwrap();
    }

    let script = dir.join("kan-stub.sh");
    std::fs::write(
        &script,
        format!(
            r#"#!/bin/sh
DATA="{data}"
case "$1" in
  --help) echo "kan (test stub)"; exit 0 ;;
  status) cat "$DATA/status.txt"; exit 0 ;;
  show)
    f="$DATA/show-$(printf '%s' "$2" | tr '/' '_').txt"
    if [ -f "$f" ]; then cat "$f"; else echo "$2: no claims"; fi
    exit 0 ;;
  issues) cat "$DATA/issues.txt" 2>/dev/null; exit 0 ;;
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
    esc=$(printf '%s' "$text" \
      | sed 's/\\/\\\\/g; s/"/\\"/g' \
      | awk 'BEGIN{{ORS=""}} NR>1{{printf "\\n"}} {{print}}')
    f="$DATA/show-$(printf '%s' "$subj" | tr '/' '_').txt"
    [ -f "$f" ] || printf '%s (live claims):\n' "$subj" > "$f"
    printf '  %s  Observation  Observation {{ text: "%s" }}\n' "$cid" "$esc" >> "$f"
    grep -q "\[Local(\"$subj\")\]" "$DATA/status.txt" 2>/dev/null \
      || printf '[Local("%s")]: Observation — Observation {{ text: "%s" }}  (%s)\n' \
           "$subj" "$esc" "$cid" >> "$DATA/status.txt"

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

/// kan renders each claim body with Rust's `Debug`, and the field name
/// differs by body: narrative claims carry `text`, `Subject` claims carry
/// `title` plus a `subject_kind`.
fn debug_body(claim: &StubClaim) -> String {
    if claim.kind == "Subject" {
        format!("Subject {{ title: {:?}, subject_kind: Idea }}", claim.text)
    } else {
        format!("{} {{ text: {:?} }}", claim.kind, claim.text)
    }
}

fn show_filename(subject: &str) -> String {
    format!("show-{}.txt", subject.replace('/', "_"))
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
