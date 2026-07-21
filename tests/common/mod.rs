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

    let mut subjects: Vec<&str> = claims.iter().map(|c| c.subject.as_str()).collect();
    subjects.sort_unstable();
    subjects.dedup();

    // `kan status` with no argument: one line per subject.
    let mut status = String::new();
    for subject in &subjects {
        let last = claims.iter().rev().find(|c| c.subject == *subject).unwrap();
        status.push_str(&format!(
            "[Local({:?})]: {} — {} {{ text: {:?} }}  ({})\n",
            subject, last.kind, last.kind, last.text, last.cid,
        ));
    }
    std::fs::write(data.join("status.txt"), status).unwrap();

    // `kan show <subject>`: header line, then one line per live claim,
    // oldest first — the order day relies on to pick the newest interface.
    for subject in &subjects {
        let for_subject: Vec<&StubClaim> =
            claims.iter().filter(|c| c.subject == *subject).collect();
        let mut show = format!("{subject} ({} live claim(s)):\n", for_subject.len());
        for c in for_subject {
            show.push_str(&format!(
                "  {}  {}  {} {{ text: {:?} }}\n",
                c.cid, c.kind, c.kind, c.text,
            ));
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

fn show_filename(subject: &str) -> String {
    format!("show-{}.txt", subject.replace('/', "_"))
}

/// Path to a binary that does not exist, for the "kan is absent" cases.
pub fn missing_kan(dir: &Path) -> PathBuf {
    dir.join("definitely-not-installed-kan")
}

/// The repo root, so tests can assert on shipped plugin/doc files.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
