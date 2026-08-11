//! `.design/harness-footer.md` — the harness footer's acceptance criteria.
//!
//! The renderer half (AC-1, 2, 3, 6, 9, 10, 11, 16, 17, 18, 19, 20) drives
//! `day::footer` directly: the footer is pure by design (REQ-10), so its
//! states are constructible without a repo. The assembly half (AC-4, 14, 15)
//! drives `hooks::footer_context` against real fixture repositories, because
//! the worktree and remote-parsing claims are about git, and a stub answering
//! what the test expects would validate day against day. The identity half
//! (AC-7, 8) drives the shipped binary against the kan stub. AC-5 lives in
//! `tests/assess.rs` at the whitelist it is about; AC-12 in `tests/status.rs`
//! with the other cache-reader guarantees.

#![cfg(unix)]

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use day::footer::{self, Checkout, Context, EnvSignals, Style, Surround, Sync};
use day::probe::Verdict;
use day::status::{Criterion, Here, Status, Transition};

use common::{claim, write_kan_stub, write_stub_roles};

fn blank_status() -> Status {
    Status {
        here: vec![],
        off_sequence: vec![],
        unordered: vec![],
        unrecorded: vec![],
        unrecorded_boundary: None,
        transition: None,
        uncheckable: false,
        cadence: 10,
        unreadable: Vec::new(),
    }
}

fn here(atom: &str, done_met: usize, done_total: usize, next: &[&str]) -> Here {
    let done = (0..done_total)
        .map(|i| Criterion {
            witness: format!("w{i}"),
            verdict: Some(if i < done_met {
                Verdict::Satisfied("found".into())
            } else {
                Verdict::Unsatisfied("missing".into())
            }),
        })
        .collect();
    Here {
        atom: atom.to_string(),
        inputs_present: vec![],
        inputs_unknown: vec![],
        done,
        next: next.iter().map(|s| s.to_string()).collect(),
    }
}

/// The nine states REQ-1 names: four position forms, four message kinds, and
/// the could-not-read report. Returned as (name, rendered) so a failure names
/// the state rather than an index.
fn nine_states(surround: &Surround, style: Style) -> Vec<(&'static str, String)> {
    let mut states = Vec::new();

    let mut setup = blank_status();
    setup.uncheckable = true;
    states.push(("setup", footer::render(&setup, surround, style)));

    states.push(("no-atom", footer::render(&blank_status(), surround, style)));

    let mut one = blank_status();
    one.here = vec![here("build", 1, 2, &["review"])];
    states.push(("one-atom", footer::render(&one, surround, style)));

    let mut many = blank_status();
    many.here = vec![here("build", 0, 0, &[]), here("release", 0, 0, &[])];
    states.push(("many-atoms", footer::render(&many, surround, style)));

    let mut transition = blank_status();
    transition.here = vec![here("review", 0, 0, &[])];
    transition.transition = Some(Transition {
        from: "build".into(),
        to: vec!["review".into()],
    });
    states.push(("transition", footer::render(&transition, surround, style)));

    let mut boundary = blank_status();
    boundary.here = vec![here("review", 0, 0, &[])];
    boundary.unrecorded_boundary =
        Some("v1.0.0 is tagged but no `release` claim records it".into());
    states.push(("boundary", footer::render(&boundary, surround, style)));

    let mut unrecorded = blank_status();
    unrecorded.here = vec![here("review", 0, 0, &[])];
    unrecorded.unrecorded = vec!["code-change".into()];
    states.push(("unrecorded", footer::render(&unrecorded, surround, style)));

    let mut off_sequence = blank_status();
    off_sequence.here = vec![here("review", 0, 0, &[])];
    off_sequence.off_sequence =
        vec!["review produced its output but upstream build did not".into()];
    states.push((
        "off-sequence",
        footer::render(&off_sequence, surround, style),
    ));

    states.push(("unreadable", footer::render_unreadable(surround, style)));

    states
}

/// AC-1 and AC-11: all nine states through the renderer, nine distinct
/// outputs, in the emoji and the plain rendering alike — the same table
/// drives both, so a state one style can express and the other cannot fails
/// here rather than in a terminal.
#[test]
fn ac1_ac11_the_nine_states_render_distinctly_in_both_styles() {
    for style in [Style::Emoji, Style::Plain] {
        let states = nine_states(&Surround::default(), style);
        assert_eq!(states.len(), 9);
        for (i, (name_a, out_a)) in states.iter().enumerate() {
            assert!(!out_a.is_empty(), "{name_a} rendered empty ({style:?})");
            for (name_b, out_b) in &states[i + 1..] {
                assert_ne!(
                    out_a, out_b,
                    "{name_a} and {name_b} render identically in {style:?} — a \
                     state that cannot be told apart is a state the footer drops"
                );
            }
        }
    }
}

/// AC-2: one inferred atom and several candidates render differently, and
/// the difference — `atom:` vs `atom?`, the separator *is* the state — is
/// present in both renderings.
#[test]
fn ac2_atom_ambiguity_survives_both_renderings() {
    for style in [Style::Emoji, Style::Plain] {
        let states = nine_states(&Surround::default(), style);
        let one = &states.iter().find(|(n, _)| *n == "one-atom").unwrap().1;
        let many = &states.iter().find(|(n, _)| *n == "many-atoms").unwrap().1;
        assert!(one.contains("atom: "), "{style:?}: {one}");
        assert!(!one.contains("atom? "), "{style:?}: {one}");
        assert!(many.contains("atom? "), "{style:?}: {many}");
        assert!(!many.contains("atom: "), "{style:?}: {many}");
    }
}

/// AC-3: the literal `day` appears at most once, over every one of the nine
/// states — in the emoji rendering it appears zero times (the anchor is ☀️,
/// REQ-3), and the plain rendering spends its one occurrence on the anchor.
#[test]
fn ac3_the_literal_day_appears_at_most_once_in_every_state() {
    for style in [Style::Emoji, Style::Plain] {
        for (name, rendered) in nine_states(&Surround::default(), style) {
            let count = rendered.matches("day").count();
            assert!(
                count <= 1,
                "{name} ({style:?}) contains `day` {count} times:\n{rendered}"
            );
        }
    }
}

/// AC-6: clean, dirty, ahead, behind, and dirty-and-ahead-and-behind each
/// render distinguishably — five distinct outputs, not the presence of a
/// phrase, because commit, push and pull are different remedies (REQ-5).
#[test]
fn ac6_the_five_sync_states_render_distinctly() {
    let sync = |dirty, ab| Surround {
        context: Context {
            sync: Some(Sync {
                dirty,
                ahead_behind: ab,
            }),
            ..Context::default()
        },
        ..Surround::default()
    };
    for style in [Style::Emoji, Style::Plain] {
        let outputs: Vec<String> = [
            sync(false, Some((0, 0))), // clean
            sync(true, Some((0, 0))),  // dirty
            sync(false, Some((3, 0))), // ahead
            sync(false, Some((0, 2))), // behind
            sync(true, Some((3, 2))),  // all at once
        ]
        .iter()
        .map(|s| footer::render(&blank_status(), s, style))
        .collect();
        for (i, a) in outputs.iter().enumerate() {
            for b in &outputs[i + 1..] {
                assert_ne!(a, b, "two sync states render alike in {style:?}: {a}");
            }
        }
    }
}

/// AC-9: there is no input for which the narrowing is non-zero and the
/// indicator absent — driven across all nine states, both styles, with and
/// without the identity segment, because REQ-7 splits the degradation rule:
/// the identity may vanish, the narrowing may not.
#[test]
fn ac9_a_non_zero_narrowing_is_never_omitted() {
    for style in [Style::Emoji, Style::Plain] {
        for role in [None, Some("director".to_string())] {
            let surround = Surround {
                context: Context::default(),
                role,
                withheld: 3,
            };
            for (name, rendered) in nine_states(&surround, style) {
                assert!(
                    rendered.contains("3 withheld"),
                    "{name} ({style:?}) hides a non-zero narrowing:\n{rendered}"
                );
            }
        }
    }
}

/// AC-10: more messages than the tray shows ends in the truncation mark and
/// the count of what was dropped; a tray that fits carries no mark (REQ-8,
/// RQ-5 — visibly or not at all).
#[test]
fn ac10_the_tray_truncates_visibly_or_not_at_all() {
    let mut overfull = blank_status();
    overfull.here = vec![here("build", 0, 0, &[])];
    overfull.transition = Some(Transition {
        from: "design".into(),
        to: vec!["build".into()],
    });
    overfull.unrecorded_boundary = Some("v1 is tagged but unrecorded".into());
    overfull.unrecorded = vec!["code-change".into(), "design-doc".into()];
    overfull.off_sequence = vec!["a step was skipped".into()];

    let mut fits = blank_status();
    fits.here = vec![here("build", 0, 0, &[])];
    fits.unrecorded = vec!["code-change".into()];

    for style in [Style::Emoji, Style::Plain] {
        let overfull = footer::render(&overfull, &Surround::default(), style);
        assert!(
            overfull.contains("(+2 more)"),
            "five messages over a tray of {} must say what was dropped \
             ({style:?}):\n{overfull}",
            footer::TRAY_MAX
        );
        let fits = footer::render(&fits, &Surround::default(), style);
        assert!(
            !fits.contains("more)"),
            "a tray that fits must carry no truncation mark ({style:?}):\n{fits}"
        );
    }
}

/// AC-16: the three checkout forms render distinguishably, and a path
/// exceeding the bound is truncated visibly (REQ-15).
#[test]
fn ac16_the_three_checkout_forms_render_distinguishably() {
    let with = |checkout| Surround {
        context: Context {
            checkout: Some(checkout),
            ..Context::default()
        },
        ..Surround::default()
    };
    let long = "/Users/m/code/worktrees/day-behaviour-0009e02f9dcb/tree";
    for style in [Style::Emoji, Style::Plain] {
        let main = footer::render(&blank_status(), &with(Checkout::Main), style);
        let under = footer::render(
            &blank_status(),
            &with(Checkout::UnderMain(".claude/worktrees/abcd".into())),
            style,
        );
        let elsewhere = footer::render(
            &blank_status(),
            &with(Checkout::Elsewhere(long.into())),
            style,
        );
        assert_ne!(main, under, "{style:?}");
        assert_ne!(main, elsewhere, "{style:?}");
        assert_ne!(under, elsewhere, "{style:?}");
        assert!(
            under.contains(".claude/worktrees/abcd"),
            "a worktree under the main root renders its relative path \
             ({style:?}): {under}"
        );
        let ellipsis = if style == Style::Emoji { "…" } else { "..." };
        assert!(
            elsewhere.contains(ellipsis),
            "a path over the bound must be truncated visibly ({style:?}): {elsewhere}"
        );
        assert!(
            !elsewhere.contains(long),
            "a path over the bound must not render whole ({style:?}): {elsewhere}"
        );
    }
}

/// AC-17: each negative signal independently forces the plain rendering —
/// separate cases, so one being handled does not make the others look
/// handled (REQ-17).
#[test]
fn ac17_each_negative_signal_independently_forces_plain() {
    let healthy = || EnvSignals {
        style_override: None,
        lc_all: None,
        lc_ctype: None,
        lang: Some("en_US.UTF-8".into()),
        term: Some("xterm-256color".into()),
        no_color: None,
    };
    let cases: Vec<(&str, EnvSignals)> = vec![
        ("LANG=C", {
            let mut s = healthy();
            s.lang = Some("C".into());
            s
        }),
        ("LC_ALL without UTF-8", {
            let mut s = healthy();
            s.lc_all = Some("en_US.ISO8859-1".into());
            s
        }),
        ("TERM=dumb", {
            let mut s = healthy();
            s.term = Some("dumb".into());
            s
        }),
        ("TERM unset", {
            let mut s = healthy();
            s.term = None;
            s
        }),
        ("NO_COLOR=1", {
            let mut s = healthy();
            s.no_color = Some("1".into());
            s
        }),
    ];
    for (name, signals) in cases {
        assert_eq!(
            Style::resolve(&signals),
            Style::Plain,
            "{name} alone must force the plain rendering"
        );
    }
    assert_eq!(
        Style::resolve(&healthy()),
        Style::Emoji,
        "the healthy baseline must not be plain, or the cases above pass vacuously"
    );
}

/// AC-18: with a UTF-8 locale and no negative signal the emoji rendering is
/// chosen — and no `TERM_PROGRAM` allowlist exists anywhere in `src/`, so
/// the positive case can never come from a list that rots (REQ-17).
#[test]
fn ac18_emoji_is_the_default_and_no_term_program_allowlist_exists() {
    let signals = EnvSignals {
        lang: Some("en_US.UTF-8".into()),
        term: Some("xterm".into()),
        ..EnvSignals::default()
    };
    assert_eq!(Style::resolve(&signals), Style::Emoji);

    for path in walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src")) {
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(
            !text.contains("TERM_PROGRAM"),
            "{} consults TERM_PROGRAM — a terminal allowlist is a classifier \
             keyed on a positive list that stops matching as new terminals \
             appear (REQ-17 forbids it)",
            path.display()
        );
    }
}

/// AC-19: the override forces **both** directions against detection — an
/// override that only works one way is half a control (REQ-16).
#[test]
fn ac19_the_override_forces_both_directions() {
    let plain_by_detection = EnvSignals {
        lang: Some("C".into()),
        style_override: Some("emoji".into()),
        ..EnvSignals::default()
    };
    assert_eq!(Style::resolve(&plain_by_detection), Style::Emoji);

    let emoji_by_detection = EnvSignals {
        lang: Some("en_US.UTF-8".into()),
        term: Some("xterm".into()),
        style_override: Some("plain".into()),
        ..EnvSignals::default()
    };
    assert_eq!(Style::resolve(&emoji_by_detection), Style::Plain);
}

/// AC-20: detection has no failure mode that produces no footer — every
/// signal combination renders every state non-empty — and v1 reads no
/// `schema/*` subject for the choice, so the seventh absent-means-default
/// loader is not introduced by accident (REQ-18, REQ-19).
#[test]
fn ac20_every_locale_renders_and_no_schema_loader_exists() {
    let combos: Vec<EnvSignals> = vec![
        EnvSignals::default(), // nothing set at all
        EnvSignals {
            lang: Some("C".into()),
            ..EnvSignals::default()
        },
        EnvSignals {
            lc_all: Some("ja_JP.eucJP".into()),
            term: Some("dumb".into()),
            ..EnvSignals::default()
        },
        EnvSignals {
            lang: Some("en_US.UTF-8".into()),
            term: Some("xterm-256color".into()),
            ..EnvSignals::default()
        },
        EnvSignals {
            style_override: Some("nonsense".into()), // falls through, never fails
            ..EnvSignals::default()
        },
    ];
    for signals in combos {
        let style = Style::resolve(&signals);
        for (name, rendered) in nine_states(&Surround::default(), style) {
            assert!(
                !rendered.is_empty(),
                "{name} rendered empty under {signals:?}"
            );
        }
    }

    // The renderer consults env and status, never a declaration: a `schema/*`
    // read here would be the seventh loader whose absent-means-default
    // contract day#160 records as over-broad. The declared layer is specified
    // and lands with `day config` (REQ-19), not silently before it.
    //
    // The scan keys on loader *tokens*, not on the string `schema/` — the
    // setup state legitimately renders `setup: declare schema/witness` as
    // display text (the bar must name the subject that resolves it, day#108),
    // and reading a declaration requires a load through a client, which these
    // are the spellings of.
    let footer_rs =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/footer.rs"))
            .unwrap();
    for loader in [
        "::load",
        "KanClient",
        "InjectionSchema",
        "BlockSchemas",
        "blocks::",
    ] {
        assert!(
            !footer_rs.contains(loader),
            "src/footer.rs contains `{loader}` — reading a declaration here \
             would be the seventh absent-means-default loader, and the \
             declared preference layer is deliberately deferred (REQ-19)"
        );
    }
}

/// AC-13: the renderer is display-only — it reads neither kan, nor git, nor
/// the cache, so nothing it produces can be a decision input. The scan is
/// blunt on purpose: the module referencing the cache *at all* is the
/// boundary being crossed, whatever the surrounding code intends.
#[test]
fn ac13_the_renderer_reads_nothing() {
    let footer_rs =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/footer.rs"))
            .unwrap();
    for forbidden in [
        "crate::cache", // the render cache — display state, never an input
        "cache::",
        ".day",      // its directory
        "Command::", // no subprocess of any kind
        "kan_client",
        "Git::",
        "git::",
    ] {
        assert!(
            !footer_rs.contains(forbidden),
            "src/footer.rs references `{forbidden}` — the footer renderer is \
             pure display and must not read or spawn anything (REQ-10/11)"
        );
    }
}

// ---------------------------------------------------------------------------
// Assembly against real repositories (AC-4, AC-14, AC-15, AC-16's classifier).
// Real git, not the stub: these claims are about git's actual behaviour in
// worktrees, and a stub answering what the test expects would validate day
// against day's own idea of git.
// ---------------------------------------------------------------------------

fn git_in(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@example.invalid")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@example.invalid")
        .output()
        .expect("git should be runnable");
    assert!(
        out.status.success(),
        "git {args:?} failed in {}: {}",
        dir.display(),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A repo with one commit, named `day-fixture`, inside `parent`.
fn fixture_repo(parent: &Path) -> PathBuf {
    let root = parent.join("day-fixture");
    std::fs::create_dir_all(&root).unwrap();
    git_in(&root, &["init", "-q", "-b", "main"]);
    std::fs::write(root.join("a.txt"), "a\n").unwrap();
    git_in(&root, &["add", "a.txt"]);
    git_in(&root, &["commit", "-q", "-m", "init"]);
    root
}

/// AC-4: against a fixture repo, branch and ahead/behind come from one
/// `status` read — asserted by driving the state through commits and seeing
/// both move together, not by comparing against a second source.
#[test]
fn ac4_branch_and_ahead_behind_come_from_one_status_read() {
    let dir = tempfile::tempdir().unwrap();
    let upstream = fixture_repo(dir.path());
    let clone = dir.path().join("clone");
    git_in(
        dir.path(),
        &[
            "clone",
            "-q",
            upstream.to_str().unwrap(),
            clone.to_str().unwrap(),
        ],
    );

    let git = day::git::Git::new(&clone);
    let sync = git.sync_state().expect("a real repo must be readable");
    assert_eq!(sync.branch.as_deref(), Some("main"));
    assert_eq!(sync.ahead_behind, Some((0, 0)));
    assert!(!sync.dirty);

    // One commit here → ahead 1. One commit upstream, fetched → behind 1.
    std::fs::write(clone.join("b.txt"), "b\n").unwrap();
    git_in(&clone, &["add", "b.txt"]);
    git_in(&clone, &["commit", "-q", "-m", "local"]);
    std::fs::write(upstream.join("c.txt"), "c\n").unwrap();
    git_in(&upstream, &["add", "c.txt"]);
    git_in(&upstream, &["commit", "-q", "-m", "upstream"]);
    git_in(&clone, &["fetch", "-q"]);

    let sync = git.sync_state().unwrap();
    assert_eq!(sync.ahead_behind, Some((1, 1)), "{sync:?}");
    assert!(!sync.dirty);

    // And dirtiness is the same read.
    std::fs::write(clone.join("d.txt"), "d\n").unwrap();
    let sync = git.sync_state().unwrap();
    assert!(sync.dirty, "{sync:?}");
}

/// AC-14: with a remote the repo renders as `org/name`; without one, as the
/// main checkout's directory name; and **inside a worktree both answers are
/// unchanged** — the assertion that fails if `--show-toplevel` names the
/// repo (RQ-7).
#[test]
fn ac14_the_repo_name_is_stable_inside_a_worktree() {
    let dir = tempfile::tempdir().unwrap();
    let root = fixture_repo(dir.path());
    let wt = dir.path().join("elsewhere-worktree");
    git_in(
        &root,
        &["worktree", "add", "-q", wt.to_str().unwrap(), "-b", "wt"],
    );

    git_in(
        &root,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/kan-tools/day.git",
        ],
    );
    for checkout in [&root, &wt] {
        let ctx = day::hooks::footer_context(&day::git::Git::new(checkout));
        assert_eq!(
            ctx.repo.as_deref(),
            Some("kan-tools/day"),
            "with a remote, org/name — from {} too",
            checkout.display()
        );
    }

    git_in(&root, &["remote", "remove", "origin"]);
    for checkout in [&root, &wt] {
        let ctx = day::hooks::footer_context(&day::git::Git::new(checkout));
        assert_eq!(
            ctx.repo.as_deref(),
            Some("day-fixture"),
            "with no remote, the MAIN checkout's directory name — from {} \
             too, which is what `--show-toplevel` gets wrong",
            checkout.display()
        );
    }

    // And the checkout classifier tells the two apart (AC-16's assembly).
    let main = day::hooks::footer_context(&day::git::Git::new(&root));
    assert_eq!(main.checkout, Some(Checkout::Main));
    let elsewhere = day::hooks::footer_context(&day::git::Git::new(&wt));
    assert!(
        matches!(elsewhere.checkout, Some(Checkout::Elsewhere(_))),
        "{:?}",
        elsewhere.checkout
    );

    // A worktree *under* the main root is the middle form, relative to it.
    let under = root.join(".claude/worktrees/abcd");
    std::fs::create_dir_all(root.join(".claude/worktrees")).unwrap();
    git_in(
        &root,
        &[
            "worktree",
            "add",
            "-q",
            under.to_str().unwrap(),
            "-b",
            "wt2",
        ],
    );
    let ctx = day::hooks::footer_context(&day::git::Git::new(&under));
    assert_eq!(
        ctx.checkout,
        Some(Checkout::UnderMain(".claude/worktrees/abcd".into())),
        "a worktree under the main root renders relative to it"
    );
}

/// AC-15: an unrecognised remote yields the directory-name fallback rather
/// than a mangled `org/name`. (The recognised forms are unit-tested at the
/// parser in `src/footer.rs`; this is the fallback wired end-to-end.)
#[test]
fn ac15_an_unrecognised_remote_falls_back_to_the_directory_name() {
    let dir = tempfile::tempdir().unwrap();
    let root = fixture_repo(dir.path());
    git_in(&root, &["remote", "add", "origin", "/some/local/path"]);
    let ctx = day::hooks::footer_context(&day::git::Git::new(&root));
    assert_eq!(
        ctx.repo.as_deref(),
        Some("day-fixture"),
        "a local-path remote is not of a recognised shape and must fall back"
    );
}

// ---------------------------------------------------------------------------
// The identity segment, through the shipped binary (AC-7, AC-8).
// ---------------------------------------------------------------------------

fn day_cmd(dir: &Path, kan: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_day"))
        .args(args)
        .current_dir(dir)
        .env("DAY_KAN_BIN", kan)
        // Deterministic style whatever the harness environment looks like.
        .env("DAY_FOOTER", "emoji")
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run day")
}

fn minimal_log() -> Vec<common::StubClaim> {
    vec![claim(
        "schema/witness",
        "bafyreiw",
        "W.\n\n```day-witness\n{\"design-doc\":{\"path\":\".design/*.md\"}}\n```\n",
    )]
}

/// AC-7: with a declared role whose DID is the active one, the footer names
/// that role; with a role declared under a different DID, it does not.
#[test]
fn ac7_the_footer_names_the_role_whose_did_is_active() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    write_stub_roles(
        dir.path(),
        &format!(
            r#"{{"v":1,"active":"{did}","roles":[{{"name":"director","did":"{did}"}}]}}"#,
            did = common::STUB_AUTHOR
        ),
    );
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let line = std::fs::read_to_string(dir.path().join(".day/statusline")).unwrap();
    assert!(
        line.contains("director"),
        "the active role must be named: {line}"
    );

    let kan = write_kan_stub(dir.path(), &minimal_log());
    write_stub_roles(
        dir.path(),
        r#"{"v":1,"active":"did:key:zSomebodyElse","roles":[{"name":"director","did":"did:key:zNotThisOne"}]}"#,
    );
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let line = std::fs::read_to_string(dir.path().join(".day/statusline")).unwrap();
    assert!(
        !line.contains("director"),
        "a role under a different DID must not be named: {line}"
    );
}

/// AC-8: with no declared roles the identity segment is absent entirely —
/// not empty, not a placeholder — and with the identity read failing
/// outright the rest of the footer still renders and the hook exits zero.
#[test]
fn ac8_no_roles_omits_the_segment_and_a_failed_read_degrades() {
    let dir = tempfile::tempdir().unwrap();

    // No roles declared: the stub's default envelope has an empty role list.
    let kan = write_kan_stub(dir.path(), &minimal_log());
    let out = day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    assert!(out.status.success());
    let line = std::fs::read_to_string(dir.path().join(".day/statusline")).unwrap();
    assert!(
        !line.contains("🖋"),
        "no declared roles must omit the segment, not render it empty: {line}"
    );

    // The read failing outright: an envelope day cannot parse.
    let kan = write_kan_stub(dir.path(), &minimal_log());
    write_stub_roles(dir.path(), "this is not json at all");
    let out = day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    assert!(
        out.status.success(),
        "an unparseable identity read must not fail the hook"
    );
    let line = std::fs::read_to_string(dir.path().join(".day/statusline")).unwrap();
    assert!(
        !line.contains("🖋") && !line.is_empty(),
        "the rest of the footer must render without the segment: {line}"
    );
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out
}
