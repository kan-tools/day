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

/// A width no fixture in this file reaches, so a test that is not *about*
/// elision never accidentally measures it.
const ROOMY: usize = 200;

/// One state the footer must render, and **what it must say** — not merely
/// that it differs from the others.
///
/// The distinction is the whole point. The first version of this table
/// asserted only that the states were pairwise distinct, and three mutations
/// walked through it: off-sequence findings wearing the unrecorded glyph,
/// `behind` rendered as `ahead`, and the setup line naming the wrong subject
/// all keep nine outputs distinct while making one of them **wrong**. Nine
/// distinct wrong strings satisfy distinctness; only content catches this.
struct State {
    name: &'static str,
    status: Status,
    /// Substrings the rendering must contain, per style. Written per style
    /// because the glyph is the state's identity here — that is what REQ-1's
    /// "a glyph key that covers only the mockup's four" is about.
    emoji: &'static [&'static str],
    plain: &'static [&'static str],
}

impl State {
    fn expected(&self, style: Style) -> &'static [&'static str] {
        match style {
            Style::Emoji => self.emoji,
            Style::Plain => self.plain,
        }
    }
}

/// The ten states the footer renders: four position forms, four message
/// kinds, the partial-read report, and the could-not-read-at-all report.
///
/// **Ten, not REQ-1's nine.** REQ-1 named nine and folded two distinct facts
/// into "the partial-read report": a log day read *partially* (`Status.
/// unreadable` is non-empty, so the position beside it is computed over an
/// incomplete vocabulary) and a log day could not read *at all*. Only the
/// second was built first time round, and the requirement's wording is why
/// nobody noticed. Recorded on `harness-footer`.
fn all_states() -> Vec<State> {
    let with = |f: &dyn Fn(&mut Status)| {
        let mut s = blank_status();
        f(&mut s);
        s
    };
    vec![
        State {
            name: "setup",
            status: with(&|s| s.uncheckable = true),
            // Names the subject that resolves it (day#108). Asserted by name
            // so renaming the target fails here rather than drifting.
            emoji: &["setup: declare schema/witness"],
            plain: &["setup: declare schema/witness"],
        },
        State {
            name: "no-atom",
            status: blank_status(),
            emoji: &["no atom in play"],
            plain: &["no atom in play"],
        },
        State {
            name: "one-atom",
            status: with(&|s| s.here = vec![here("build", 1, 2, &["review"])]),
            emoji: &["atom: build", "1/2 done", "next: review"],
            plain: &["atom: build", "1/2 done", "next: review"],
        },
        State {
            name: "many-atoms",
            status: with(&|s| s.here = vec![here("build", 0, 0, &[]), here("release", 0, 0, &[])]),
            emoji: &["atom? build | release"],
            plain: &["atom? build | release"],
        },
        State {
            name: "transition",
            status: with(&|s| {
                s.here = vec![here("review", 0, 0, &[])];
                s.transition = Some(Transition {
                    from: "build".into(),
                    to: vec!["review".into()],
                });
            }),
            emoji: &["⤳ ", "past `build`"],
            plain: &["moved: ", "past `build`"],
        },
        State {
            name: "boundary",
            status: with(&|s| {
                s.here = vec![here("review", 0, 0, &[])];
                s.unrecorded_boundary = Some("v1.0.0 is tagged but unrecorded".into());
            }),
            emoji: &["🏷 ", "v1.0.0"],
            plain: &["tag: ", "v1.0.0"],
        },
        State {
            name: "unrecorded",
            status: with(&|s| {
                s.here = vec![here("review", 0, 0, &[])];
                s.unrecorded = vec!["code-change".into()];
            }),
            emoji: &["✍ ", "code-change"],
            plain: &["unrecorded: ", "code-change"],
        },
        State {
            name: "off-sequence",
            status: with(&|s| {
                s.here = vec![here("review", 0, 0, &[])];
                s.off_sequence = vec!["a step was skipped".into()];
            }),
            emoji: &["❗ ", "a step was skipped"],
            plain: &["skipped: ", "a step was skipped"],
        },
        State {
            name: "partial-read",
            status: with(&|s| {
                s.here = vec![here("review", 0, 0, &[])];
                s.unreadable = vec![day::status::Unreadable {
                    message: "an atom block could not be read".into(),
                    cause: day::status::Cause::Malformed,
                }];
            }),
            emoji: &["◐ ", "1 unreadable"],
            plain: &["? ", "1 unreadable"],
        },
        State {
            name: "unreadable",
            status: blank_status(), // rendered by render_unreadable, below
            emoji: &["⛔ ", "kan could not be read"],
            plain: &["!! ", "kan could not be read"],
        },
    ]
}

/// Renders every state, using the total-failure renderer for the state that
/// has one. Returns (name, rendered) so a failure names the state.
fn render_all(surround: &Surround, style: Style, budget: usize) -> Vec<(&'static str, String)> {
    all_states()
        .into_iter()
        .map(|s| {
            let rendered = if s.name == "unreadable" {
                footer::render_unreadable(surround, style, budget)
            } else {
                footer::render(&s.status, surround, style, budget)
            };
            (s.name, rendered)
        })
        .collect()
}

/// AC-1 and AC-11: every state renders, says **what it means**, and is
/// distinguishable from every other — in both renderings, from one table, so
/// a state one style can express and the other cannot fails here rather than
/// in a terminal.
#[test]
fn ac1_ac11_every_state_renders_its_own_content_in_both_styles() {
    for style in Style::ALL {
        let states = all_states();
        let rendered = render_all(&Surround::default(), style, ROOMY);
        assert_eq!(rendered.len(), states.len());

        // Content: the assertion distinctness cannot make.
        for (state, (name, out)) in states.iter().zip(&rendered) {
            assert_eq!(state.name, *name);
            for needle in state.expected(style) {
                assert!(
                    out.contains(needle),
                    "{name} ({style:?}) must contain {needle:?} — a state that \
                     renders someone else's glyph is still 'distinct':\n{out}"
                );
            }
        }

        // And distinctness on top, which content alone does not give.
        for (i, (name_a, out_a)) in rendered.iter().enumerate() {
            assert!(!out_a.is_empty(), "{name_a} rendered empty ({style:?})");
            for (name_b, out_b) in &rendered[i + 1..] {
                assert_ne!(
                    out_a, out_b,
                    "{name_a} and {name_b} render identically in {style:?}"
                );
            }
        }
    }
}

/// AC-11's other half, and the one the first version left to inference:
/// **the plain rendering is plain.** Filling `PLAIN`'s glyph table with emoji
/// kept nine distinct non-empty strings and the entire suite green, so the
/// requirement that exists because "terminals vary in whether they render
/// emoji at all" was asserted by nothing.
#[test]
fn ac11_the_plain_rendering_is_pure_ascii() {
    let populated = Surround {
        context: Context {
            repo: Some("kan-tools/day".into()),
            branch: Some("main".into()),
            sync: Some(Sync {
                dirty: true,
                ahead_behind: Some((3, 2)),
            }),
            checkout: Some(Checkout::UnderMain(".claude/worktrees/abcd".into())),
        },
        role: Some("director".into()),
        withheld: 2,
    };
    for (name, out) in render_all(&populated, Style::Plain, ROOMY) {
        assert!(
            out.is_ascii(),
            "{name} rendered non-ASCII in the plain style, which exists for \
             terminals that cannot draw emoji at all:\n{out}"
        );
    }
    // Non-vacuity: the same fixture in the emoji style must NOT be ASCII, or
    // the assertion above would pass on a footer that renders nothing.
    let emoji: String = render_all(&populated, Style::Emoji, ROOMY)
        .into_iter()
        .map(|(_, out)| out)
        .collect();
    assert!(
        !emoji.is_ascii(),
        "the emoji rendering must actually use emoji, or the ASCII assertion \
         above proves nothing"
    );
}

/// AC-2: one inferred atom and several candidates render differently, and
/// the difference — `atom:` vs `atom?`, the separator *is* the state — is
/// present in both renderings.
#[test]
fn ac2_atom_ambiguity_survives_both_renderings() {
    for style in Style::ALL {
        let states = render_all(&Surround::default(), style, ROOMY);
        let one = &states.iter().find(|(n, _)| *n == "one-atom").unwrap().1;
        let many = &states.iter().find(|(n, _)| *n == "many-atoms").unwrap().1;
        assert!(one.contains("atom: "), "{style:?}: {one}");
        assert!(!one.contains("atom? "), "{style:?}: {one}");
        assert!(many.contains("atom? "), "{style:?}: {many}");
        assert!(!many.contains("atom: "), "{style:?}: {many}");
    }
}

/// AC-3 / REQ-3: **`day` is not repeated per line** — the word becomes one
/// anchor at the left of the first line.
///
/// Stated as the anchor property, not as "the literal `day` appears at most
/// once in the output", which is what this asserted first time round and is
/// **false in day's own repo**: the fixture was an empty `Surround`, so no
/// repo name ever rendered, and driven for real the plain footer reads
/// `day setup: …` / `kan-tools/day - on main - …` — twice, because the repo
/// is *named* `kan-tools/day`. A repo may legitimately contain the word; the
/// requirement is about day stamping its own name on every line.
///
/// So the fixture below deliberately names the repo `kan-tools/day`: the
/// test now runs against the string that falsified its predecessor.
#[test]
fn ac3_the_anchor_is_not_repeated_per_line() {
    let named_day = Surround {
        context: Context {
            repo: Some("kan-tools/day".into()),
            branch: Some("day-branch".into()),
            sync: Some(Sync::default()),
            checkout: Some(Checkout::Main),
        },
        role: None,
        withheld: 0,
    };
    for style in Style::ALL {
        for (name, rendered) in render_all(&named_day, style, ROOMY) {
            let anchored = rendered
                .lines()
                .filter(|l| l.starts_with("day ") || l.starts_with("☀️"))
                .count();
            assert!(
                anchored <= 1,
                "{name} ({style:?}) stamps the anchor on {anchored} lines — \
                 REQ-3 is that it appears once, at the left of the first:\n{rendered}"
            );
            if style == Style::Emoji {
                assert!(
                    !rendered.lines().any(|l| l.starts_with("day ")),
                    "{name}: the emoji rendering's anchor is ☀️, not the word:\n{rendered}"
                );
            }
        }
    }
}

/// AC-6: clean, dirty, ahead, behind, and dirty-and-ahead-and-behind each
/// render distinguishably (REQ-5) — because commit, push and pull are
/// different remedies and one glyph cannot name which.
///
/// **The counts are symmetric on purpose.** The first version used `(3, 0)`
/// for ahead and `(0, 2)` for behind, so the five outputs differed because
/// `3 != 2` — and swapping the `ahead`/`behind` glyphs (making the footer say
/// "pull" when you should push, which is exactly REQ-5's stated worry) left
/// the whole suite green. A fixture that is symmetric under the mutation it
/// fears cannot detect it, so `(3, 0)` and `(0, 3)` differ *only* by which
/// mark is used.
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
    for style in Style::ALL {
        let cases = [
            ("clean", sync(false, Some((0, 0)))),
            ("dirty", sync(true, Some((0, 0)))),
            ("ahead", sync(false, Some((3, 0)))),
            ("behind", sync(false, Some((0, 3)))),
            ("all", sync(true, Some((3, 3)))),
        ];
        let outputs: Vec<(&str, String)> = cases
            .iter()
            .map(|(name, s)| (*name, footer::render(&blank_status(), s, style, ROOMY)))
            .collect();
        for (i, (name_a, a)) in outputs.iter().enumerate() {
            for (name_b, b) in &outputs[i + 1..] {
                assert_ne!(
                    a, b,
                    "{name_a} and {name_b} render alike in {style:?} — with equal \
                     counts, only the mark can tell them apart:\n{a}"
                );
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
    let crowded = Context {
        repo: Some("kan-tools/day".into()),
        branch: Some("a-rather-long-branch-name-here".into()),
        sync: Some(Sync {
            dirty: true,
            ahead_behind: Some((12, 34)),
        }),
        checkout: Some(Checkout::Elsewhere(
            "/Users/m/code/worktrees/day-behaviour-0009e02f9dcb/tree".into(),
        )),
    };
    for style in Style::ALL {
        for role in [None, Some("director".to_string())] {
            for context in [Context::default(), crowded.clone()] {
                // Every budget, including the narrowest: a caveat elided to
                // save room is a caveat that lied, so width pressure must
                // never be a route to dropping it.
                for budget in [ROOMY, 72, 48, 20] {
                    let surround = Surround {
                        context: context.clone(),
                        role: role.clone(),
                        withheld: 3,
                    };
                    for (name, rendered) in render_all(&surround, style, budget) {
                        assert!(
                            rendered.contains("3 withheld"),
                            "{name} ({style:?}, budget {budget}) hides a non-zero \
                             narrowing:\n{rendered}"
                        );
                    }
                }
            }
        }
    }
}

/// REQ-1's ninth state, and the companion to AC-9: **the partial-read report
/// is pinned too.** `Status.unreadable` means the position rendered beside it
/// was computed over a vocabulary day knows it could not fully read, which is
/// a caveat on everything else on the footer — so it survives every budget,
/// exactly as the narrowing does.
///
/// It reached the model channel and not the human's bar, which is the day#60
/// asymmetry `src/hooks.rs` already records as mattering most in practice.
#[test]
fn the_partial_read_report_reaches_the_bar_and_is_never_elided() {
    let mut status = blank_status();
    status.here = vec![here("build", 1, 2, &["review"])];
    status.unreadable = vec![
        day::status::Unreadable {
            message: "an atom block could not be read".into(),
            cause: day::status::Cause::Malformed,
        },
        day::status::Unreadable {
            message: "a witness probe kind is too new".into(),
            cause: day::status::Cause::VersionSkew,
        },
    ];
    let crowded = Surround {
        context: Context {
            repo: Some("kan-tools/day".into()),
            branch: Some("a-rather-long-branch-name-here".into()),
            sync: Some(Sync {
                dirty: true,
                ahead_behind: Some((12, 34)),
            }),
            checkout: Some(Checkout::UnderMain(".claude/worktrees/abcd".into())),
        },
        role: Some("director".into()),
        withheld: 0,
    };
    for style in Style::ALL {
        for budget in [ROOMY, 72, 48, 20] {
            let out = footer::render(&status, &crowded, style, budget);
            assert!(
                out.contains("2 unreadable"),
                "the partial-read report must reach the bar at every width — \
                 a confident position computed over a vocabulary day could not \
                 read is the inversion telos/honest-reads forbids ({style:?}, \
                 budget {budget}):\n{out}"
            );
        }
        // And it says nothing when there is nothing to say.
        let mut clean = blank_status();
        clean.here = vec![here("build", 1, 2, &["review"])];
        let out = footer::render(&clean, &crowded, style, ROOMY);
        assert!(
            !out.contains("unreadable"),
            "a fully-readable log must produce no partial-read report:\n{out}"
        );
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

    for style in Style::ALL {
        let rendered = footer::render(&overfull, &Surround::default(), style, ROOMY);
        assert!(
            rendered.contains("2 more)"),
            "five messages over a tray of {} must say what was dropped \
             ({style:?}):\n{rendered}",
            footer::TRAY_MAX
        );
        // **And the tray actually truncated.** Asserting only the mark let a
        // tray render ALL five items *and* claim `(+2 more)` — a tray that
        // both overflows its bound and lies about the count — with this test
        // passing. RQ-5's untested third state was "claims to truncate and
        // does not".
        let tray = rendered.lines().last().unwrap();
        for dropped in ["a step was skipped", "design-doc"] {
            assert!(
                !tray.contains(dropped),
                "the tray claims to have dropped items and still shows \
                 {dropped:?} ({style:?}):\n{tray}"
            );
        }

        let rendered = footer::render(&fits, &Surround::default(), style, ROOMY);
        assert!(
            !rendered.contains("more)"),
            "a tray that fits must carry no truncation mark ({style:?}):\n{rendered}"
        );
    }
}

/// RQ-5's rule, generalised to the whole footer: **what is dropped for width
/// is dropped visibly.** A segment silently missing because the terminal was
/// narrow is indistinguishable from a segment day could not fill, and REQ-7
/// makes that distinction load-bearing.
#[test]
fn width_elision_is_always_visible() {
    let full = Surround {
        context: Context {
            repo: Some("kan-tools/day".into()),
            branch: Some("build-harness-footer".into()),
            sync: Some(Sync {
                dirty: true,
                ahead_behind: Some((2, 1)),
            }),
            checkout: Some(Checkout::UnderMain(".claude/worktrees/abcd".into())),
        },
        role: Some("director".into()),
        withheld: 0,
    };
    let mut status = blank_status();
    status.here = vec![here("build", 1, 2, &["review"])];

    for style in Style::ALL {
        let roomy = footer::render(&status, &full, style, ROOMY);
        let narrow = footer::render(&status, &full, style, 40);
        assert!(
            day::footer::display_width(narrow.lines().nth(1).unwrap()) <= 40,
            "the narrow rendering must fit its budget ({style:?}):\n{narrow}"
        );
        assert!(
            narrow.len() < roomy.len(),
            "a 40-column budget must actually drop something ({style:?}) — \
             otherwise the assertion below is vacuous:\n{narrow}"
        );
        let marker = if style == Style::Emoji { "…+" } else { "+" };
        assert!(
            narrow.contains(marker),
            "something was elided for width and the footer does not say so \
             ({style:?}) — a silently missing segment reads as 'day has \
             nothing to report':\n{narrow}"
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
    for style in Style::ALL {
        let main = footer::render(&blank_status(), &with(Checkout::Main), style, ROOMY);
        let under = footer::render(
            &blank_status(),
            &with(Checkout::UnderMain(".claude/worktrees/abcd".into())),
            style,
            ROOMY,
        );
        let elsewhere = footer::render(
            &blank_status(),
            &with(Checkout::Elsewhere(long.into())),
            style,
            ROOMY,
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
        // AC-16's "no longer than the declared bound", which the first
        // version stated and did not assert — and the rendering exceeded it,
        // because the ellipsis was added *after* the bound was applied.
        let segment = elsewhere
            .lines()
            .nth(1)
            .expect("the context line renders")
            .trim();
        assert!(
            day::footer::display_width(segment) <= footer::CHECKOUT_BOUND + 4,
            "the checkout segment is {} wide, over its declared bound of {} \
             (+ the glyph) ({style:?}): {segment}",
            day::footer::display_width(segment),
            footer::CHECKOUT_BOUND
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
        columns: None,
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
        for (name, rendered) in render_all(&Surround::default(), style, signals.width()) {
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
    let renderer = renderer_sources();
    for (path, text) in &renderer {
        for loader in [
            "::load",
            "KanClient",
            "InjectionSchema",
            "BlockSchemas",
            "blocks::",
        ] {
            assert!(
                !text.contains(loader),
                "{} contains `{loader}` — reading a declaration here would be \
                 the seventh absent-means-default loader, and the declared \
                 preference layer is deliberately deferred (REQ-19)",
                path.display()
            );
        }
    }
}

/// Every source file the footer renderer lives in, **found rather than
/// named**, with a positive control that the scan reached the real thing.
///
/// `read_to_string("src/footer.rs")` is what the first version of the two
/// scans below did, and it fails open: split the renderer into
/// `src/footer/render.rs` with `src/footer.rs` reduced to `pub mod render;`
/// — a routine refactor — and both scans keep passing while the code they
/// exist to constrain goes unscanned. A scan that reports clean by having
/// found nothing is the defect class this repo has recorded repeatedly, so
/// the list is derived and the control asserts it is non-empty *and*
/// contains the renderer.
/// **Comments are stripped**, and that is not a convenience. The rule these
/// scans enforce is about what the renderer *does*; the module's own doc
/// comment has to be able to explain why it may not touch `.day/` without
/// tripping a scan looking for `.day`. `tests/fallbacks.rs`'s premise check
/// strips comments for the mirror-image reason — there, prose must not
/// satisfy an assertion; here, prose must not violate one.
fn renderer_sources() -> Vec<(PathBuf, String)> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let found: Vec<(PathBuf, String)> = walk(&src)
        .into_iter()
        .filter(|p| {
            let name = p.to_string_lossy();
            name.contains("/footer.rs") || name.contains("/footer/")
        })
        .map(|p| {
            let text = std::fs::read_to_string(&p).unwrap();
            let code = text
                .lines()
                .map(|l| match l.find("//") {
                    Some(i) => &l[..i],
                    None => l,
                })
                .collect::<Vec<_>>()
                .join("\n");
            (p, code)
        })
        .collect();
    assert!(
        !found.is_empty(),
        "the renderer scan found no sources under src/ — it would report \
         clean by having looked at nothing"
    );
    assert!(
        found.iter().any(|(_, t)| t.contains("pub fn render")),
        "the renderer scan found files but not the renderer itself, so it is \
         scanning the wrong thing"
    );
    found
}

/// AC-13: the renderer is display-only — it reads neither kan, nor git, nor
/// the cache, so nothing it produces can be a decision input. The scan is
/// blunt on purpose: the module referencing the cache *at all* is the
/// boundary being crossed, whatever the surrounding code intends.
#[test]
fn ac13_the_renderer_reads_nothing() {
    for (path, text) in renderer_sources() {
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
                !text.contains(forbidden),
                "{} references `{forbidden}` — the footer renderer is pure \
                 display and must not read or spawn anything (REQ-10/11)",
                path.display()
            );
        }
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

    // One commit here → ahead 1. **Asserted before the upstream commit**,
    // because every other state this test asserted — (0,0) and (1,1) — is
    // symmetric under swapping the two counts, so exchanging ahead for
    // behind in `sync_state` left it green. The asymmetric state was created
    // and thrown away; this is the assertion that makes the fixture able to
    // detect the mutation it exists to detect.
    std::fs::write(clone.join("b.txt"), "b\n").unwrap();
    git_in(&clone, &["add", "b.txt"]);
    git_in(&clone, &["commit", "-q", "-m", "local"]);
    let sync = git.sync_state().unwrap();
    assert_eq!(
        sync.ahead_behind,
        Some((1, 0)),
        "one local commit is one AHEAD and zero behind — the direction is the \
         whole point, since commit, push and pull are different remedies: {sync:?}"
    );

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

    // **The premise, read from git.** Deleting the `remote add` above left
    // this test passing, because "unrecognised remote → directory name"
    // (REQ-14) and "no remote at all → directory name" (REQ-12) have the same
    // answer — so it could not tell the two requirements apart and was a
    // duplicate of `fallback_no_remote`. Asserting the fixture's *state*, from
    // the substrate rather than from the parser under test, is what separates
    // them.
    let configured = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&configured.stdout).trim(),
        "/some/local/path",
        "premise: a remote must be configured and be of an unrecognised shape \
         — with none, this measures REQ-12's fallback instead of REQ-14's"
    );

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

/// REQ-7 / AC-9 **as delivered**, not as rendered.
///
/// The renderer-side test above drives ten states × two styles × four
/// budgets and is still the wrong side of the boundary: the one line that
/// supplies the count (`footer_surround`) could be hardcoded to `0` with the
/// entire suite green. A narrowing day never *fetches* is a narrowing the
/// renderer is never asked to show, and kan#121 — two identities on one
/// workspace, each reading a complete-looking view, neither mentioning the
/// other, exit 0 both times — is what that reproduces.
///
/// So this drives the shipped binary against a kan that withholds, and reads
/// the cache the status line will print.
#[test]
fn the_narrowing_is_fetched_not_merely_renderable() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    common::write_stub_withheld(dir.path(), 3);

    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let cached = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        cached.contains("3 withheld"),
        "the log-wide narrowing must reach the cache the bar prints — \
         asserting it only inside the renderer leaves the fetch unguarded:\n{cached}"
    );

    // The other direction, so the assertion above is not satisfied by a
    // footer that always says it: a log with nothing withheld says nothing.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let cached = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        !cached.contains("withheld"),
        "a log with nothing withheld must not claim a narrowing:\n{cached}"
    );
}

/// The partial-read report, **as delivered** — same argument as the test
/// above. An unreadable declaration must reach the bar, not only the model
/// channel it already reached (day#60).
#[test]
fn the_partial_read_report_is_fetched_not_merely_renderable() {
    let dir = tempfile::tempdir().unwrap();
    // An atom block from a newer day: readable as a claim, unreadable as a
    // declaration, which is exactly `Status.unreadable`'s subject.
    let mut claims = minimal_log();
    claims.push(claim(
        "atom/future",
        "bafyfuture",
        &format!(
            "A future atom.\n\n```day-atom\n{}\n```\n",
            common::too_new_atom_body()
        ),
    ));
    let kan = write_kan_stub(dir.path(), &claims);

    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let cached = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        cached.contains("unreadable"),
        "a declaration day could not read must reach the bar — the position \
         beside it was computed over an incomplete vocabulary:\n{cached}"
    );

    // And a fully-readable log says nothing.
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let cached = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        !cached.contains("unreadable"),
        "a fully-readable log must not claim a partial read:\n{cached}"
    );
}

/// The tenth state, **end to end**: with kan broken, the bar says so rather
/// than keeping the previous session's confident position.
///
/// Its renderer had exactly one caller, reachable only after `client.probe()`
/// and `client.subjects()` had both succeeded — so the two paths a broken kan
/// actually takes never wrote it, and the protection its own comment claimed
/// did not operate. Both tests covering it called the renderer by hand.
#[test]
fn a_broken_kan_replaces_the_bar_rather_than_leaving_it_stale() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());

    // A good session first, so there is a confident position to go stale.
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);
    let fresh = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        !fresh.contains("could not be read"),
        "premise: the first session must produce an ordinary footer:\n{fresh}"
    );

    // Now kan breaks.
    let broken = dir.path().join("broken-kan.sh");
    std::fs::write(&broken, "#!/bin/sh\necho boom >&2\nexit 3\n").unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&broken, std::fs::Permissions::from_mode(0o755)).unwrap();

    let out = day_cmd(dir.path(), &broken, &["hook", "session-start"]);
    assert!(out.status.success(), "the hook must still exit zero");

    let after = std::fs::read_to_string(dir.path().join(".day/statusline.variants")).unwrap();
    assert!(
        after.contains("could not be read"),
        "with kan broken the bar must say so — leaving it holding the previous \
         session's position displays confidently from a read that just \
         failed:\n{after}"
    );
    // And the legacy single-rendering file too, since that is what an older
    // day (or anything that just prints the file) shows.
    let legacy = std::fs::read_to_string(dir.path().join(".day/statusline")).unwrap();
    assert!(
        legacy.contains("could not be read"),
        "the single-rendering cache must be replaced too:\n{legacy}"
    );
}

/// `DAY_FOOTER` where a person actually sets it: on the status line.
///
/// It resolved at hook time and was baked into the cache, so
/// `DAY_FOOTER=plain day status-line` did nothing at all while README and
/// CONVENTIONS both documented it as an override. The variants exist partly
/// so this works.
#[test]
fn the_style_override_applies_where_the_status_line_runs() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    // The hook runs with NO override and an emoji-friendly environment.
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);

    let plain = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["status-line"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .env("DAY_FOOTER", "plain")
        .env("COLUMNS", "100")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    let plain = String::from_utf8_lossy(&plain.stdout).to_string();
    assert!(
        plain.is_ascii() && !plain.is_empty(),
        "DAY_FOOTER=plain must take effect at status-line time, not only in \
         the hook's environment: {plain:?}"
    );

    let emoji = Command::new(env!("CARGO_BIN_EXE_day"))
        .args(["status-line"])
        .current_dir(dir.path())
        .env("DAY_KAN_BIN", &kan)
        .env("DAY_FOOTER", "emoji")
        .env("COLUMNS", "100")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    let emoji = String::from_utf8_lossy(&emoji.stdout).to_string();
    assert!(
        !emoji.is_ascii(),
        "and emoji the other way, from the same cache: {emoji:?}"
    );
}

/// The width the terminal reports is honoured, from the same cache.
#[test]
fn a_narrow_terminal_gets_a_narrower_footer_from_the_same_cache() {
    let dir = tempfile::tempdir().unwrap();
    let kan = write_kan_stub(dir.path(), &minimal_log());
    day_cmd(dir.path(), &kan, &["hook", "session-start"]);

    let at = |columns: &str| {
        let out = Command::new(env!("CARGO_BIN_EXE_day"))
            .args(["status-line"])
            .current_dir(dir.path())
            .env("DAY_KAN_BIN", &kan)
            .env("COLUMNS", columns)
            .stdin(std::process::Stdio::null())
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).to_string()
    };
    for (columns, line) in [("100", at("100")), ("40", at("40"))] {
        let width = line
            .lines()
            .map(day::footer::display_width)
            .max()
            .unwrap_or(0);
        let budget: usize = columns.parse().unwrap();
        assert!(
            width <= budget,
            "at COLUMNS={columns} the footer rendered {width} columns wide:\n{line}"
        );
    }
}

/// day must not report dirt **it created**.
///
/// Untracked entries count as dirty and day writes `.day/statusline` at every
/// session start, so in any repo that has not gitignored the cache — every
/// fresh `git init`, which is the population REQ-12 and `telos/v1.0` both
/// name — the sync mark stuck on "dirty" from the second session onward with
/// the user having done nothing. A display whose stated justification is
/// "dirty means commit" must not be counting its own artifacts.
#[test]
fn the_cache_day_writes_is_not_reported_as_dirtiness() {
    let dir = tempfile::tempdir().unwrap();
    let root = fixture_repo(dir.path());
    let git = day::git::Git::new(&root);

    assert!(
        !git.sync_state().unwrap().dirty,
        "premise: the fixture must start clean, or this cannot show the cache \
         causing dirtiness"
    );

    // Exactly what a session start leaves behind.
    std::fs::create_dir_all(root.join(day::cache::CACHE_DIR)).unwrap();
    std::fs::write(
        root.join(day::cache::CACHE_DIR).join("statusline"),
        "day - build",
    )
    .unwrap();
    let porcelain = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&porcelain.stdout).contains(".day"),
        "premise: git must actually see the cache as untracked — if the \
         fixture gitignores it, this measures nothing"
    );

    assert!(
        !git.sync_state().unwrap().dirty,
        "day reported dirtiness caused by its own render cache"
    );

    // And a real edit still reads dirty, so the exclusion did not blind it.
    std::fs::write(root.join("real.txt"), "edited\n").unwrap();
    assert!(
        git.sync_state().unwrap().dirty,
        "excluding the cache must not stop day seeing the user's own changes"
    );
}
