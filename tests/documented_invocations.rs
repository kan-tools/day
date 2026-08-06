//! `.design/verification-that-can-fail.md` AC-17..AC-19 — **day#89: the
//! documented invocations actually run.**
//!
//! `tests/plugin.rs` validates `docs/CONVENTIONS.md` for *content* — that it
//! names the prefixes and fence strings the code uses, documents the `done`
//! field, carries the inference rule — and all of that has caught real drift.
//! Nothing validated it for *executability*. A documented command could be
//! syntactically wrong, shell-hostile, or name a flag that no longer exists, and
//! every test stayed green.
//!
//! day already knows this failure from the other side of the boundary:
//! `tests/kan_conformance.rs` exists as the deliberate exception to the stub
//! rule because a stub validates day against day's own idea of kan's CLI rather
//! than kan's contract — which is how this same page documented a `kan result`
//! invocation that does not run, through several releases. **day had the mirror
//! gap for itself.** day#83 was the instance: `--scope published-artifact=v0.5*`
//! unquoted, which fails in zsh — macOS's default shell and this repo's — with
//! `no matches found`. It had never worked for anyone copying it.
//!
//! **The bar is deliberately low**: not that every example produces the right
//! output, which would need per-example fixtures and would rot, but that every
//! documented invocation *parses and runs*. That is the property day#83
//! violated.

#![cfg(unix)]

mod common;

use common::{repo_root, write_kan_stub};
use std::path::Path;
use std::process::Command;

/// **An explicit list, not a discovered count.** A generator whose failure mode
/// is "less output" needs an exhaustive expectation — the block corpus silently
/// omitted three of seven block types, twice, because its check was a number.
/// [`the_corpus_covers_every_page_that_carries_a_shell_block`] asserts this list
/// is still complete.
const PAGES: [&str; 5] = [
    "README.md",
    "docs/CONVENTIONS.md",
    "commands/adversarial-review.md",
    "commands/design.md",
    "commands/witness-interview.md",
];

const FENCES: [&str; 4] = ["```bash", "```console", "```sh", "```shell"];

/// One documented command, with where it came from.
struct Invocation {
    page: String,
    line: usize,
    command: String,
}

/// Every `day …` invocation in a page's shell fences, with `\`-continuations
/// joined — a multi-line command read line-by-line is a different command, and
/// four of this repo's own examples are multi-line.
fn invocations(page: &str, text: &str) -> Vec<Invocation> {
    let mut out = Vec::new();
    let mut in_fence = false;
    let mut pending: Option<(usize, String)> = None;
    for (n, raw) in text.lines().enumerate() {
        if raw.starts_with("```") {
            in_fence = FENCES.iter().any(|f| raw.starts_with(f));
            pending = None;
            continue;
        }
        if !in_fence {
            continue;
        }
        let line = raw.trim_end();
        let stripped = line.trim_start().trim_start_matches("$ ").to_string();
        match pending.take() {
            Some((at, mut so_far)) => {
                so_far.push(' ');
                so_far.push_str(stripped.trim_end_matches('\\').trim());
                if line.ends_with('\\') {
                    pending = Some((at, so_far));
                } else {
                    out.push(Invocation {
                        page: page.to_string(),
                        line: at,
                        command: so_far,
                    });
                }
            }
            None if stripped == "day" || stripped.starts_with("day ") => {
                let body = stripped.trim_end_matches('\\').trim().to_string();
                if line.ends_with('\\') {
                    pending = Some((n + 1, body));
                } else {
                    out.push(Invocation {
                        page: page.to_string(),
                        line: n + 1,
                        command: body,
                    });
                }
            }
            None => {}
        }
    }
    out
}

/// **The exclusion rule, stated here rather than kept as a skip list.** A
/// hand-maintained list of exceptions drifts; a rule can be read and argued
/// with. Only one exclusion is needed: a template is not an invocation.
fn skip_reason(command: &str) -> Option<&'static str> {
    for (i, c) in command.char_indices() {
        if c == '<' && command[i..].contains('>') {
            return Some("a template with a <placeholder>, not a runnable example");
        }
    }
    None
}

/// zsh where available, `sh` otherwise. **The shell matters and is the bug**:
/// day#83 fails in zsh and passes in `sh`, so running the corpus through `sh`
/// would have missed the defect this whole file exists for.
fn shell() -> &'static str {
    if Command::new("zsh")
        .arg("-c")
        .arg("exit 0")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        "zsh"
    } else {
        "sh"
    }
}

/// A parse failure, as a **positive signal** rather than as "did not exit 0".
///
/// day exits non-zero for real semantic reasons — `assess` uses exit 2 — so an
/// exit code cannot distinguish "this command is malformed" from "this command
/// ran and reported something". Keyed instead on the three things that only ever
/// mean the invocation never got off the ground.
fn parse_failure(out: &std::process::Output) -> Option<String> {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    for marker in [
        // clap's own banner on an unrecognised flag or a missing value.
        "For more information, try '--help'",
        // zsh refusing to run a command whose glob matched nothing. This is
        // day#83 exactly.
        "no matches found",
        "command not found",
    ] {
        if text.contains(marker) {
            return Some(format!("{marker}\n{text}"));
        }
    }
    None
}

/// A scratch directory with `day` and a stub `kan` on PATH, and nothing else.
struct Sandbox {
    dir: tempfile::TempDir,
    kan: std::path::PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("a scratch dir");
        let bin = dir.path().join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_day"), bin.join("day")).unwrap();
        Command::new("git")
            .args(["init", "-q", "-b", "main"])
            .current_dir(dir.path())
            .output()
            .expect("git should be runnable");
        // An empty log: every documented example must at least PARSE against a
        // project that has declared nothing, which is the state a reader copying
        // from the page is in.
        //
        // Written ONCE. The first version rebuilt the stub per invocation, which
        // reset its own append log, so the check that writes reached the stub
        // read zero every time — a fixture that erases the evidence it is about
        // to be asked for.
        let kan = write_kan_stub(dir.path(), &[]);
        Self { dir, kan }
    }

    fn run(&self, command: &str) -> std::process::Output {
        Command::new(shell())
            .arg("-c")
            .arg(command)
            .current_dir(self.dir.path())
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.dir.path().join("bin").display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("DAY_KAN_BIN", &self.kan)
            .stdin(std::process::Stdio::null())
            .output()
            .expect("the shell should be runnable")
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }
}

/// AC-17, AC-19 — **every documented `day` invocation parses and runs.**
#[test]
fn every_documented_day_invocation_parses_and_runs() {
    let sandbox = Sandbox::new();
    let mut ran = 0;
    let mut skipped = 0;
    let mut failures = Vec::new();

    for page in PAGES {
        let text = std::fs::read_to_string(repo_root().join(page))
            .unwrap_or_else(|e| panic!("{page} should be readable: {e}"));
        for inv in invocations(page, &text) {
            if let Some(why) = skip_reason(&inv.command) {
                // Every skip is printed with its reason. A corpus that shrinks
                // silently reads as coverage it does not have.
                println!(
                    "skip {}:{} — {why}\n     {}",
                    inv.page, inv.line, inv.command
                );
                skipped += 1;
                continue;
            }
            ran += 1;
            if let Some(detail) = parse_failure(&sandbox.run(&inv.command)) {
                failures.push(format!(
                    "{}:{}\n  {}\n  {detail}",
                    inv.page, inv.line, inv.command
                ));
            }
        }
    }

    // **An exact expectation, not a floor.** `ran >= 8` passed with a third of
    // the corpus missing, which is the failure `capture-block-corpus.sh` had
    // twice: a generator whose failure mode is less output needs an exhaustive
    // expectation. Changing this number is a decision about coverage, and the
    // message says so.
    // 13/5. The five skips include `commands/witness-interview.md`'s two, which
    // is honest rather than disappointing: both of its `day` examples take the
    // telos slug being interviewed, so both are genuinely templates. The page
    // is in `PAGES` so a *runnable* example added to it later is executed
    // rather than silently uncovered.
    //
    // The thirteenth run is `docs/CONVENTIONS.md`'s `--witness-any` example,
    // added with the flag. That it counts as a *run* rather than a skip is the
    // point of this check: the documented form of a brand-new flag is executed
    // against a stub rather than trusted to be right.
    const EXPECTED_RUN: usize = 13;
    const EXPECTED_SKIPPED: usize = 5;
    assert_eq!(
        (ran, skipped),
        (EXPECTED_RUN, EXPECTED_SKIPPED),
        "the documented corpus changed size. If an example was added or removed \
         deliberately, update these numbers in the same commit; if not, the \
         extractor stopped seeing something it used to see."
    );
    assert!(
        failures.is_empty(),
        "documented invocations that do not run under {}:\n\n{}",
        shell(),
        failures.join("\n\n")
    );

    // AC-19: nothing reached a real log — asserted from the *stub's* side,
    // which is a positive signal, rather than from the absence of a change to
    // this repo's `.kan/`. Several documented examples append claims; if any of
    // them had escaped the sandbox, the stub would have no record of them.
    //
    // The discipline behind this is not hypothetical: verifying day#83's own fix
    // left a `telos/zzz-probe` claim that had to be retracted in the same breath.
    // An assessment that pollutes the record it assesses is measuring its own
    // footprint.
    let appended = common::appends(sandbox.path());
    assert!(
        appended.iter().any(|a| a.contains("v05-shipped")),
        "the documented `telos declare` examples must have reached the STUB. \
         Nothing recorded means either they did not run or they wrote somewhere \
         else; got {} append(s)",
        appended.len()
    );
}

/// AC-18 — **the page list is complete**, asserted against what is on disk
/// rather than against a number.
#[test]
fn the_corpus_covers_every_page_that_carries_a_shell_block() {
    let mut found: Vec<String> = Vec::new();
    let mut candidates = vec![repo_root().join("README.md"), repo_root().join("CLAUDE.md")];
    for dir in ["docs", "commands"] {
        for entry in std::fs::read_dir(repo_root().join(dir)).unwrap().flatten() {
            candidates.push(entry.path());
        }
    }
    for path in candidates {
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        if !invocations(&rel, &text).is_empty() {
            found.push(rel);
        }
    }
    found.sort();
    let mut expected: Vec<String> = PAGES.iter().map(|p| p.to_string()).collect();
    expected.sort();

    assert_eq!(
        found, expected,
        "the documented-invocation corpus and the pages that actually carry one \
         have diverged. Add the page to PAGES — a page that quietly drops out is \
         coverage lost with no error."
    );
}

/// AC-17 — **the check can fail, shown on day#83 itself.**
///
/// The page has been fixed, so the corpus above passes and would pass whether or
/// not this check works. Feeding the runner the unquoted form is what
/// distinguishes "the docs are correct" from "the checker sees nothing" — and
/// those are the two readings this milestone exists to separate.
#[test]
fn the_check_catches_day_83s_unquoted_glob() {
    if shell() != "zsh" {
        // **Could-not-check**, and libtest captures BOTH streams for a passing
        // test — so neither `println!` nor `eprintln!` makes this visible on its
        // own, and an earlier comment here claiming stdout specifically was
        // wrong. The stream is not what fixes it.
        //
        // The resolution is that suite's, exactly: the skip stays legitimate on
        // a developer machine that has no zsh, and **CI installs zsh** so the
        // criterion is never skipped where it counts. What must not happen is
        // the skip becoming permanent without anyone noticing, so the CI
        // guarantee is itself asserted, below.
        eprintln!(
            "COULD NOT CHECK: zsh is not available, and `sh` does not fail on an \
             unmatched glob, so day#83 is unreachable here. This is not a pass. \
             CI installs zsh so this criterion always runs there."
        );
        let ci = std::fs::read_to_string(repo_root().join(".github/workflows/ci.yml")).unwrap();
        assert!(
            ci.contains("install -y -qq zsh"),
            "the local skip is only acceptable because CI installs zsh; without \
             that, this criterion is dead everywhere and day#83 has no test"
        );
        return;
    }
    let sandbox = Sandbox::new();

    let unquoted = "day telos declare v05-shipped \"day v0.5 is published.\" \
                    --witness published-artifact --scope published-artifact=v0.5*";
    assert!(
        parse_failure(&sandbox.run(unquoted)).is_some(),
        "the unquoted `--scope …=v0.5*` form must be caught; it is day#83, and \
         it shipped in docs/CONVENTIONS.md through several releases"
    );

    let quoted = "day telos declare v05-shipped \"day v0.5 is published.\" \
                  --witness published-artifact --scope 'published-artifact=v0.5*'";
    assert!(
        parse_failure(&sandbox.run(quoted)).is_none(),
        "the quoted form is what the page now carries and must pass"
    );
}
