//! `.design/scaffold.md` AC-4, AC-5, AC-7, AC-8, AC-9 — the shipped plugin
//! is well-formed, its hooks cannot block, and the skills and conventions it
//! promises are actually present.

mod common;

use common::repo_root;
use serde_json::Value;

fn read_json(rel: &str) -> Value {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should exist and be readable: {e}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{} should be valid JSON: {e}", path.display()))
}

#[test]
fn ac4_plugin_manifest_is_valid_json_with_name_and_description() {
    let manifest = read_json(".claude-plugin/plugin.json");
    assert_eq!(manifest["name"].as_str(), Some("day"));
    assert!(
        !manifest["description"].as_str().unwrap_or("").is_empty(),
        "plugin.json needs a non-empty description"
    );
}

#[test]
fn ac4_mcp_registration_declares_the_day_stdio_server() {
    let mcp = read_json(".mcp.json");
    let day = &mcp["mcpServers"]["day"];
    assert_eq!(day["command"].as_str(), Some("day"));
    assert_eq!(day["args"][0].as_str(), Some("mcp"));
}

/// REQ-4's guardrail, not a one-time check: day's hooks are advisory by
/// construction, and a blocking hook must never be able to land here
/// unnoticed. These are the values a Claude Code hook uses to deny, block,
/// or halt — none may appear in day's shipped hook config.
#[test]
fn ac5_shipped_hooks_declare_no_blocking_decisions() {
    let path = repo_root().join("hooks/hooks.json");
    let raw = std::fs::read_to_string(&path).expect("hooks.json should exist");
    let parsed: Value = serde_json::from_str(&raw).expect("hooks.json should be valid JSON");

    // Parse first so a malformed file fails loudly rather than passing the
    // substring check by accident. Every registered event is covered, so a
    // future hook cannot be added outside this guardrail.
    let events = parsed["hooks"]
        .as_object()
        .expect("hooks should be an object");
    assert!(!events.is_empty());
    for (event, entries) in events {
        assert!(entries.is_array(), "{event} should hold an array");
    }

    let hook_commands = raw.to_lowercase();
    for forbidden in [
        "\"decision\": \"block\"",
        "\"decision\":\"block\"",
        "\"permissiondecision\": \"deny\"",
        "\"permissiondecision\":\"deny\"",
        "\"continue\": false",
        "\"continue\":false",
        "pretooluse",
    ] {
        assert!(
            !hook_commands.contains(forbidden),
            "day's hooks must stay advisory; found blocking construct {forbidden:?} in hooks.json"
        );
    }
}

#[test]
fn ac5_the_session_start_hook_invokes_day_and_nothing_else() {
    let hooks = read_json("hooks/hooks.json");
    let groups = hooks["hooks"]["SessionStart"]
        .as_array()
        .expect("SessionStart should be an array");

    // Every SessionStart command must be a `day hook …` invocation — the
    // "nothing else" guarantee — and the two day registers must both be
    // present: the context hook and the human-facing notice hook.
    let commands: Vec<&str> = groups
        .iter()
        .flat_map(|g| g["hooks"].as_array().into_iter().flatten())
        .filter_map(|h| h["command"].as_str())
        .collect();
    assert!(!commands.is_empty(), "at least one SessionStart command");
    for command in &commands {
        assert!(
            command.starts_with("day hook "),
            "SessionStart must invoke day and nothing else; found {command:?}"
        );
    }
    assert!(commands.contains(&"day hook session-start"), "{commands:?}");
    assert!(
        commands.contains(&"day hook session-notice"),
        "{commands:?}"
    );
}

/// Guards the adversarial review's blocking finding. Only
/// `UserPromptSubmit`, `UserPromptExpansion`, and `SessionStart` add hook
/// stdout to the model's context; every other event writes to the debug log.
/// Registering a hook whose whole purpose is to say something to the agent
/// on any other event ships a feature that silently reaches nobody.
#[test]
fn hooks_are_only_registered_on_events_that_deliver_stdout_to_the_model() {
    let hooks = read_json("hooks/hooks.json");
    let registered = hooks["hooks"].as_object().expect("hooks object");
    for event in registered.keys() {
        assert!(
            ["UserPromptSubmit", "UserPromptExpansion", "SessionStart"].contains(&event.as_str()),
            "{event} does not add hook stdout to the model's context, so a prompt \
             registered there would reach nobody. If this hook is for a side effect \
             rather than for saying something, widen this test deliberately."
        );
    }
}

/// Every command the plugin ships, read from the directory rather than listed.
///
/// **Derived, not hand-maintained**, and that is a fix rather than a style
/// choice. A cold review found the third command absent from three separate
/// enumerations in this file — its preambles unchecked, its frontmatter
/// unasserted — while `tests/documented_invocations.rs`, which globs, picked it
/// up immediately and made the build say so.
///
/// The pattern the review named across the whole milestone: every requirement
/// whose artifact is Rust was met, and every requirement living in a
/// hand-maintained list was skipped, because nothing fails when a list does not
/// grow. A list that reads the directory cannot fail to grow.
fn shipped_commands() -> Vec<String> {
    let dir = repo_root().join("commands");
    let mut out: Vec<String> = std::fs::read_dir(&dir)
        .expect("commands/ should exist — it is what the plugin ships")
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if path.extension().is_some_and(|x| x == "md") {
                path.file_name()
                    .map(|n| format!("commands/{}", n.to_string_lossy()))
            } else {
                None
            }
        })
        .collect();
    out.sort();
    assert!(
        out.len() >= 3,
        "expected at least the three shipped commands, found {out:?}"
    );
    out
}

#[test]
fn ac7_and_ac8_the_plugin_ships_both_atoms_as_commands() {
    // Markers are per-command and stay explicit; the FILE LIST is derived, so a
    // new command cannot ship unasserted — it fails here until it is named.
    let markers = [
        ("commands/design.md", "design document"),
        ("commands/adversarial-review.md", "APPROVE WITH FOLLOW-UPS"),
        ("commands/witness-interview.md", "what would evidence"),
    ];
    for file in shipped_commands() {
        let file = file.as_str();
        let must_contain = markers
            .iter()
            .find(|(f, _)| *f == file)
            .map(|(_, m)| *m)
            .unwrap_or_else(|| {
                panic!(
                    "{file} ships with the plugin and no marker asserts what it is. \
                     Add one here — a command nothing checks is a command that can \
                     rot silently, which is what this test exists to stop."
                )
            });
        let text = std::fs::read_to_string(repo_root().join(file))
            .unwrap_or_else(|e| panic!("{file} should ship with the plugin: {e}"));
        assert!(text.starts_with("---"), "{file} needs command frontmatter");
        assert!(
            text.contains(must_contain),
            "{file} should contain {must_contain:?}"
        );
        assert!(
            text.contains("```day-atom"),
            "{file} should declare its atom interface"
        );
    }
}

#[test]
fn ac8_the_review_atom_declares_all_four_verdicts() {
    let text = std::fs::read_to_string(repo_root().join("commands/adversarial-review.md")).unwrap();
    for verdict in ["APPROVE", "APPROVE WITH FOLLOW-UPS", "REDIRECT", "BLOCK"] {
        assert!(text.contains(verdict), "missing verdict {verdict:?}");
    }
    // The verdict is still recorded into kan, but through `day review
    // record`, which enforces the closed verdict set and the citation
    // rather than trusting the prompt to.
    assert!(
        text.contains("day review record"),
        "the verdict should be recorded through day's verb"
    );
    assert!(
        text.contains("--cites"),
        "a verdict must cite the claim it audits"
    );
}

/// `.design/design-atom-backing.md` AC-9. Composition is data: the atom
/// graph in kan says what follows what, so a project can insert a step by
/// changing a claim rather than editing day's prompts. A command naming
/// another command would quietly hard-code a pipeline and undo that.
#[test]
fn ac9_neither_command_hardcodes_an_invocation_of_the_other() {
    let cases = [
        ("commands/design.md", "/adversarial-review"),
        ("commands/adversarial-review.md", "/design"),
    ];
    for (file, forbidden) in cases {
        let text = std::fs::read_to_string(repo_root().join(file)).unwrap();
        for line in text.lines() {
            // A prose mention is fine; an instruction to run it is not.
            let invokes = line.contains(forbidden)
                && (line.contains("Run ")
                    || line.contains("run ")
                    || line.trim_start().starts_with('$'));
            assert!(
                !invokes,
                "{file} should reach the next step via `day next`, not by naming \
                 {forbidden}: {line}"
            );
        }
        assert!(
            text.contains("day next"),
            "{file} should end by asking the atom graph what comes next"
        );
    }
}

/// `.design/vocabulary-verbs.md` AC-11. The conventions are the contract,
/// not day's verbs: a hand-written claim following this page must stay as
/// valid as one day wrote, or day has quietly become required.
#[test]
fn ac11_conventions_state_that_hand_written_claims_remain_valid() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    assert!(
        text.contains("hand-written claim"),
        "CONVENTIONS.md should say hand-written claims remain valid"
    );
    assert!(
        text.contains("no `revise` verb") || text.contains("no revise verb"),
        "CONVENTIONS.md should explain that revision is just a later claim"
    );
}

#[test]
fn ac9_conventions_document_the_prefixes_the_code_actually_reads() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md"))
        .expect("docs/CONVENTIONS.md should exist");
    // The doc and the code must agree, so read the constants rather than
    // hardcoding the strings twice.
    for token in [
        day::atoms::ATOM_PREFIX,
        day::atoms::TELOS_PREFIX,
        day::atoms::FENCE_INFO,
        day::schema::SCHEMA_PREFIX,
        day::schema::FENCE_INFO,
        day::bridge::BRIDGE_PREFIX,
        day::bridge::FENCE_INFO,
        day::bridge::TELOS_FENCE,
        day::docs::FENCE_INFO,
        day::docs::DOCS_SLUG,
        day::telos::FENCE_INFO,
        day::telos::WITNESS_SLUG,
        day::tension::TENSION_PREFIX,
        day::tension::FENCE_INFO,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }
}

/// `.design/assess-telos.md` AC-15's second half. Command probes make the
/// shell day's third substrate; `CLAUDE.md` is where a future session learns
/// that before adding a fourth spawn site, so it has to say so.
#[test]
fn ac15_claude_md_records_the_command_substrate_and_its_guardrails() {
    let text =
        std::fs::read_to_string(repo_root().join("CLAUDE.md")).expect("CLAUDE.md should exist");
    assert!(
        text.contains("three substrates"),
        "CLAUDE.md should record that day now has three substrates"
    );
    assert!(
        text.contains("src/probe.rs"),
        "CLAUDE.md should name where command execution is confined"
    );
    for guardrail in ["no shell", "--run", "MCP", "timeout"] {
        assert!(
            text.contains(guardrail),
            "CLAUDE.md should record the {guardrail:?} guardrail on command probes"
        );
    }
}

/// `.design/telos-subject-shape.md` AC-11's second half. The page must not
/// still instruct recording a tension's reason on a telos subject — a
/// conventions doc describing the shape day no longer implements is worse
/// than one that says nothing, because it is followed.
#[test]
fn ac11_conventions_no_longer_put_a_tension_reason_on_a_telos_subject() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    assert!(
        text.contains("carries its declaration and its edges"),
        "CONVENTIONS should state what a telos subject carries"
    );
    assert!(
        text.contains("scope"),
        "CONVENTIONS should document witness scoping"
    );
    // The old instruction, which day#32 was filed against.
    assert!(
        !text.contains("day telos tension interface-legibility feature-depth \\\n  \"Every verb")
            || text.contains("tension/<a>--<b>"),
        "the tension example should sit under the tension subject convention"
    );
}

/// `.design/claim-probe-narrowing.md` AC-7. The conventions page documents
/// both new claim-shape predicates, **and** no longer carries the day#70
/// cautions they resolved.
///
/// The negative half is the load-bearing one. A caution telling a reader that
/// `{"kind": "Result"}` matches release notes describes behaviour day's own
/// starter no longer has, and the failure mode of a conventions doc is that it
/// is followed: someone would narrow a probe day already narrows, or trust a
/// warning about a marker matching mid-text after the anchored predicate that
/// fixes it shipped.
#[test]
fn ac7_conventions_document_the_claim_predicates_and_drop_the_resolved_cautions() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    for token in [
        "`starts_with`",
        "`subject`",
        "glob-lite",
        // The predicates as a claim shape actually declares them, so the doc
        // shows a form that parses rather than prose about one.
        r#""starts_with": "adversarial review of""#,
        r#""subject": "atom/*""#,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }
    // And the conjunction, which is what makes "declare two predicates and
    // both must hold" true rather than a guess.
    assert!(
        text.contains("conjunction of independent predicates"),
        "CONVENTIONS.md should state that a claim shape is a conjunction"
    );

    // The two day#70 cautions, now resolved rather than warned about.
    for caution in [
        "The decision that *defined* `\"adversarial review of\"` is",
        "matches every `kan result`, not only atom",
    ] {
        assert!(
            !text.contains(caution),
            "CONVENTIONS.md still carries the day#70 caution {caution:?}, \
             which the narrowed starter resolved"
        );
    }
}

/// `.design/repo-defined-injection.md` AC-9. The trust list (REQ-9) is
/// designed and not built, and the property that keeps adding it later from
/// becoming a rewrite is that exactly one function decides whether a claim
/// may reach a model's context.
#[test]
fn ac9_one_function_decides_whether_a_claim_may_be_injected() {
    let practice = std::fs::read_to_string(repo_root().join("src").join("practice.rs"))
        .expect("src/practice.rs should exist");

    // The decision itself.
    assert!(
        practice.contains("fn accepts("),
        "authorship resolution should live in one named function"
    );

    // And nothing else compares an author. A second comparison anywhere is
    // a second place the trust list would have to be added.
    let comparisons = practice.matches("author").filter(|_| true).count();
    let in_accepts = practice
        .split("fn accepts(")
        .nth(1)
        .and_then(|rest| rest.split("\n}").next())
        .map(|body| body.matches("author").count())
        .unwrap_or(0);
    assert!(
        in_accepts > 0,
        "accepts() should be the function that inspects the author"
    );
    assert!(
        comparisons >= in_accepts,
        "sanity: accepts() is part of the file"
    );

    // The call site is single: the projection asks accepts(), and no other
    // module reaches for it.
    let src = repo_root().join("src");
    for entry in std::fs::read_dir(&src).unwrap().flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "rs") && path.file_name().unwrap() != "practice.rs"
        {
            let text = std::fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("accepts("),
                "{} decides injectability; that belongs only in practice.rs",
                path.display()
            );
        }
    }
}

/// `.design/rigor-as-artifact.md` AC-12: the conventions page documents the
/// `done` key and the cache path, checked against the code's own constant so
/// the doc and code cannot drift; and CLAUDE.md states the cache is derived
/// and not a store.
#[test]
fn ac12_docs_document_the_done_field_and_the_cache() {
    let conventions = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    assert!(
        conventions.contains("`done`"),
        "CONVENTIONS.md should document the atom `done` field"
    );
    assert!(
        conventions.contains(day::cache::CACHE_DIR),
        "CONVENTIONS.md should name the cache path {:?}",
        day::cache::CACHE_DIR
    );
    // The inference rule, in the form it actually takes since the `claim`
    // probe: the line is **read vs. execute**, not a fixed list of kinds.
    // This assertion used to name `path` and `tag` literally and had to be
    // revisited when a third read kind arrived — so it now pins the
    // invariant rather than the enumeration, which is what it was always
    // for. If a fourth read kind appears, this should still pass; if
    // `command` ever becomes runnable at inference time, it must not.
    assert!(
        conventions.contains("It **reads; it never executes.**"),
        "CONVENTIONS.md should document the inference rule (reads run, commands never do)"
    );
    assert!(
        conventions.contains("`command` does not"),
        "CONVENTIONS.md should say plainly that inference does not run a command probe"
    );
    // And the cycle boundary, the other rule bounding inference.
    assert!(
        conventions.contains("cycle boundary"),
        "CONVENTIONS.md should document the cycle boundary position resolves against"
    );

    let claude_md = std::fs::read_to_string(repo_root().join("CLAUDE.md")).unwrap();
    assert!(
        claude_md.contains(day::cache::CACHE_DIR) && claude_md.contains("not a store"),
        "CLAUDE.md should record that the render cache is derived and not a store"
    );
}

/// `.design/rigor-as-artifact.md` AC-9, the load-bearing cache guardrail:
/// the render cache is touched in **exactly one module**, and only for
/// display. If any other module reads or writes `.day/`, "display only" has
/// started to decay into "and also decides things" — the precise line
/// `telos/no-store-of-its-own` draws. So the literal cache path may appear in
/// `src/cache.rs` and nowhere else in `src/`; every other module reaches the
/// cache through that module's two functions, whose results only ever print.
#[test]
fn ac9_the_render_cache_is_touched_in_exactly_one_module() {
    let src = repo_root().join("src");
    let cache_dir_literal = format!("\"{}\"", day::cache::CACHE_DIR);

    let mut offenders = Vec::new();
    let mut cache_rs_has_it = false;
    let mut stack = vec![src.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            if !text.contains(&cache_dir_literal) {
                continue;
            }
            if path.file_name().unwrap() == "cache.rs" {
                cache_rs_has_it = true;
            } else {
                offenders.push(path.display().to_string());
            }
        }
    }

    assert!(
        cache_rs_has_it,
        "src/cache.rs should own the cache path constant {cache_dir_literal}"
    );
    assert!(
        offenders.is_empty(),
        "the cache path {cache_dir_literal} appears outside src/cache.rs ({offenders:?}); \
         the cache must be touched in exactly one module so 'display only' stays enforceable"
    );
}

/// AC-10. The conventions page and the code must agree on the subject name
/// and the replace token, checked against the constants rather than retyped.
#[test]
fn ac10_conventions_document_the_practice_subject_and_replace_token() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md")).unwrap();
    for token in [
        day::practice::PRACTICE_SUBJECT,
        day::practice::REPLACE_TOKEN,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }
    assert!(
        text.contains("locally signed") || text.contains("locally-signed"),
        "CONVENTIONS should state that projected practice is locally signed"
    );
}

/// **A kan read that failed must never be spelled the same way as a kan read
/// that found nothing.**
///
/// This is a source scan rather than prose because prose did not work. The rule
/// is stated plainly in `src/probe.rs` — *"a subject day cannot read is an
/// error, never a silently empty result"* — and `CLAUDE.md` now states it too,
/// and the defect still recurred **five times in five places**:
///
/// - `docs.rs`, folding a failed `show` into "no release recorded" (day#81)
/// - `hooks.rs`'s `render_teloi`, where an unreadable telos vanished from the
///   model's context *and* from its count
/// - `status::compute`, discarding `atoms::load`'s findings so position was
///   computed over a vocabulary day knew was incomplete
/// - `status::compute` again, `BlockSchemas::load(...).unwrap_or_default()`
/// - `hooks.rs`, `InjectionSchema::load(...).unwrap_or(DEFAULT_CADENCE)` — the
///   fifth, written *after* the rule was added to `CLAUDE.md`
///
/// A rule a human has to remember is not a constraint. This one fails the build.
///
/// **The escape hatch is deliberate and requires a reason.** A test with no way
/// out gets deleted the first time it is wrong; one that demands an explicit
/// marker makes the exception visible in review instead. It is per *site*, not
/// per file — a file-level opt-out exempts every later read in the same file for
/// free.
///
/// **Sixth instance, and a gap in this scan.** The four method shapes above were
/// the five instances this test was built from, and nothing else. `let Ok(x) =
/// read else { … }` and `if let Ok(x) = read { … }` discard an `Err` just as
/// completely, and both were live and unmarked: `record.rs` (deliberate, failing
/// toward *more* recording, now marked) and `hooks.rs`'s `session_end`, which
/// dropped a whole section silently twenty lines below a read in the same
/// function that reports its failure properly. The giveaway for those sits
/// *before* the read, which is why looking only at the expression after it
/// missed them.
///
/// **Still not caught, stated so this does not overclaim:** a `match` whose
/// `Err` arm is empty. That is a semantic judgement rather than a shape, and a
/// scan cannot tell an `Err` arm that reports from one that shrugs. The rule
/// this test enforces is narrower than the rule `CLAUDE.md` states, and the gap
/// is named here rather than left for someone to discover as instance seven.
#[test]
fn a_failed_kan_read_is_never_swallowed() {
    const MARKER: &str = "kan-read-may-degrade:";
    let reads = [
        "client.show(",
        "client.subjects(",
        "client.issues(",
        "::load(client)",
    ];
    let swallows = [
        ".unwrap_or_default(",
        ".unwrap_or(",
        ".unwrap_or_else(",
        ".ok(",
    ];

    let mut offenders = Vec::new();
    let mut stack = vec![repo_root().join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            // Comments are stripped first. Without that, a comment *describing*
            // a past instance — as `docs.rs` has, recording what day#81 was —
            // reads as a live one, and a scan that cries wolf about its own
            // documentation gets switched off.
            let raw = std::fs::read_to_string(&path).unwrap();
            // Markers are read from the RAW lines and reads from the stripped
            // ones. Stripping first and then looking for the marker in the
            // result finds nothing, because the marker lives in a comment —
            // which is what the first version of this did, reporting the one
            // site that had just been marked. Per-line stripping keeps the two
            // line-indexed the same.
            let raw_lines: Vec<&str> = raw.lines().collect();
            let text: String = raw_lines
                .iter()
                .map(|l| match l.find("//") {
                    Some(i) => l[..i].to_string(),
                    None => (*l).to_string(),
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mut swallowed: Vec<usize> = Vec::new();
            for read in reads {
                let mut from = 0;
                while let Some(at) = text[from..].find(read) {
                    let found = from + at;
                    let start = found + read.len();
                    from = start;
                    // The expression ends at the next `;` or `?`.
                    let tail = &text[start..text.len().min(start + 240)];
                    let end = tail
                        .find(';')
                        .into_iter()
                        .chain(tail.find('?'))
                        .min()
                        .unwrap_or(tail.len());
                    let expr = &tail[..end];
                    // **The pattern shapes are checked on the prefix, not the
                    // expression.** A `let Ok(x) = read else { … }` swallows the
                    // `Err` just as completely as `.unwrap_or_default()`, and so
                    // does `if let Ok(x) = read { … }` — but the giveaway sits
                    // *before* the read, and the four method shapes below sit
                    // after it. Detecting only the latter is what let two live
                    // instances stand: `record.rs`'s (deliberate, and now marked)
                    // and `hooks.rs`'s (not deliberate — it dropped a whole line
                    // from `session_end` twenty lines below a read in the same
                    // function that reports its failure properly).
                    //
                    // `let Ok(` covers `if let` and `while let` too.
                    let line_start = text[..found].rfind('\n').map_or(0, |i| i + 1);
                    let pattern_binding = text[line_start..found].contains("let Ok(");
                    if pattern_binding || swallows.iter().any(|s| expr.contains(s)) {
                        swallowed.push(text[..start].matches('\n').count());
                    }
                }
            }
            swallowed.sort_unstable();
            swallowed.dedup();

            // The marker is **per site**, not per file. A file-level opt-out
            // exempts every later read in the same file for free, which is the
            // hole `an_ordering_is_never_read_off_the_raw_next` was just fixed
            // for — leaving it here would be the same defect twice, knowingly.
            // Nothing used this hatch before now, so tightening it costs nothing.
            const WINDOW: usize = 8;
            let markers: Vec<usize> = raw_lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains(MARKER))
                .map(|(index, _)| index)
                .collect();
            for (nth, &line) in swallowed.iter().enumerate() {
                let previous = nth.checked_sub(1).map(|p| swallowed[p]);
                let marked = markers.iter().any(|&marker| {
                    marker <= line && line - marker <= WINDOW && previous.is_none_or(|p| p < marker)
                });
                if !marked {
                    offenders.push(format!("{}:{}", path.display(), line + 1));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "a kan read's failure is swallowed at {offenders:?}.\n\n\
         \"day could not read this\" and \"there is nothing here\" must not be \
         spelled the same way — that has been five separate defects. Either \
         propagate the error and report it (see `status::unreadable_from`), or \
         mark the site `{MARKER} <why this one is genuinely different>`."
    );
}

/// `.design/forward-only-next.md` AC-14 — **an ordering is read through
/// [`day::atoms::Forward`], never off the raw declaration.**
///
/// The DAG guarantee day#113 buys is only worth what enforces it. `next` is
/// acyclic *by construction* now, but nothing in the type system stops a future
/// consumer walking `interface.next` and quietly reinheriting the assumption
/// that broke the off-sequence check for every milestone day has ever run —
/// which is day#101's shape exactly: a guarantee that holds at the call sites
/// its author was thinking about.
///
/// A scan rather than prose for the reason
/// [`a_failed_kan_read_is_never_swallowed`] is one: CLAUDE.md records five
/// occasions on which a rule stated in one module's doc comment did not reach
/// the others, and the fifth was written after the rule was added to CLAUDE.md.
///
/// **Reading the declaration raw is legitimate and stays possible.** `doctor`
/// dumps what the claim says and must show an edge the ordering dropped; the
/// dangling-edge check must see an edge that names nothing. Those sites say so
/// with a marker, which is the point — the exception becomes visible in review
/// rather than indistinguishable from an oversight. Per-site, not per-file: a
/// file-level opt-out would exempt the next unmarked read in the same file for
/// free.
///
/// **What this catches, measured rather than assumed.** The first version was
/// line-local and window-based, and a cold review of this branch evaded it three
/// ways — each reproduced by compiling a probe file, not by reading:
///
/// - `cargo fmt`'s own multi-line chain (`atom\n.interface\n.next`). This was
///   the serious one: `cargo fmt --check` gates CI, so an offending consumer was
///   *formatted into* the evading shape automatically. Fixed by matching over
///   the whole comment-stripped file instead of per line.
/// - a local binding (`let iface = &atom.interface; iface.next`), which the
///   direct matcher cannot see because the type is not in the text. Fixed by
///   [`interface_bindings`].
/// - a second read inheriting the marker written for the first. Fixed by
///   binding a marker to the **next read only**; `doctor::edges` was
///   restructured to read once rather than have the rule bent for it.
///
/// **What it still does not catch, stated so the check does not overclaim:**
/// destructuring (`let Interface { next, .. } = …`), which needs a parser to
/// tell from the struct literal this repo uses in a dozen places; and a second
/// raw read on the *same line* as an explained one, which is one expression the
/// marker's author had in front of them. Both are could-not-checks. A scan that
/// reported "clean" while meaning "clean of the shapes I happen to know" would
/// be the same defect it exists to prevent, one level up.
#[test]
fn an_ordering_is_never_read_off_the_raw_next() {
    const MARKER: &str = "dag-not-required:";
    /// How far above a read the marker may sit — enough for a doc comment on
    /// the enclosing function. It is a *ceiling*, not the whole rule: a marker
    /// also binds to the **next read only** (see below), so distance alone
    /// never decides.
    const WINDOW: usize = 8;

    let mut offenders = Vec::new();
    let mut stack = vec![repo_root().join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).unwrap();
            let lines: Vec<&str> = raw.lines().collect();
            // Comments are stripped before matching, so a doc comment that
            // *names* `interface.next` — as `Forward`'s does at length — is not
            // itself an offence. A scan that flags its own documentation is one
            // that gets switched off.
            //
            // Stripped per line and then rejoined, so a byte offset in the
            // result still maps to a line by counting newlines, while the
            // matcher gets to see **across** lines.
            let code_lines: Vec<String> = lines
                .iter()
                .map(|l| match l.find("//") {
                    Some(at) => l[..at].to_string(),
                    None => (*l).to_string(),
                })
                .collect();
            let code = code_lines.join("\n");

            let mut reads: Vec<usize> = raw_next_offsets(&code)
                .into_iter()
                .map(|offset| code[..offset].matches('\n').count())
                .collect();
            // ...and the same field reached through a local binding, which the
            // direct matcher cannot see because the type is not in the text.
            for name in interface_bindings(&code_lines) {
                for (index, line) in code_lines.iter().enumerate() {
                    if reads_binding_next(line, &name) {
                        reads.push(index);
                    }
                }
            }
            reads.sort_unstable();
            reads.dedup();

            let markers: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| l.contains(MARKER))
                .map(|(index, _)| index)
                .collect();

            for (nth, &line) in reads.iter().enumerate() {
                let previous = nth.checked_sub(1).map(|p| reads[p]);
                let marked = markers.iter().any(|&marker| {
                    marker <= line
                        && line - marker <= WINDOW
                        // **A marker binds to the next read only.** Without
                        // this, a second read added below an explained one
                        // inherits its exemption silently — which is how an
                        // escape hatch becomes a hole. Reads are sorted, so it
                        // is enough that no earlier read sits at or after the
                        // marker.
                        && previous.is_none_or(|p| p < marker)
                });
                if !marked {
                    offenders.push(format!("{}:{}", path.display(), line + 1));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "`interface.next` is read raw at {offenders:?}.\n\n\
         `next` is a guaranteed DAG only because every ordering goes through \
         `atoms::Forward`, which hands back the cycles it had to drop. Walking \
         the declaration directly reinstates the assumption day#113 removed. \
         Use `Forward::successors`/`ancestors`, or — if this site renders the \
         declaration as written rather than treating it as an order — mark it \
         `{MARKER} <why>`."
    );
}

/// True when `c` could continue a Rust identifier, so a match can be required
/// to sit on a token boundary rather than inside a longer name.
fn ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Byte offsets of every `interface` followed by `.next` as a field access.
///
/// Runs over the **whole comment-stripped file**, not line by line, because
/// `cargo fmt` splits a long chain across lines:
///
/// ```ignore
/// atom
///     .interface
///     .next
///     .iter()
/// ```
///
/// The line-local version of this missed exactly that shape — and since
/// `cargo fmt --check` gates CI, an offending consumer is not merely able to
/// evade the scan, it is *formatted into* the evading shape automatically. A
/// check that its own toolchain routes around is worse than none, because it
/// reports clean. `trim_start` already spans newlines, so whole-file matching
/// is the entire fix.
fn raw_next_offsets(code: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find("interface") {
        let start = from + at;
        let after = start + "interface".len();
        from = after;
        // Token boundaries on both sides: `extract_interface(` is not a read,
        // and neither is `.interfaces`.
        if code[..start].ends_with(ident_char) || code[after..].starts_with(ident_char) {
            continue;
        }
        let Some(rest) = code[after..].trim_start().strip_prefix('.') else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix("next") else {
            continue;
        };
        // `next` must end here: `interface.next_thing` is a different field.
        if !rest.starts_with(ident_char) {
            out.push(start);
        }
    }
    out
}

/// Names bound to an `Interface` — `let iface = &atom.interface;` — so that a
/// later `iface.next` is caught too.
///
/// The direct matcher cannot see this: after the binding, the text says nothing
/// about the type. Bindings whose value already reaches `.next` are excluded,
/// because the binding line is itself a read the direct matcher reports, and
/// the resulting local is a `Vec<String>` rather than an `Interface`.
///
/// **Known blind spot, stated rather than papered over:** destructuring
/// (`let Interface { next, .. } = …`) is not detected. Distinguishing a pattern
/// from the struct literal this repo uses in a dozen places needs a parser, and
/// a scan that guessed would fire on legitimate construction. This is a
/// could-not-check, and naming it is the difference between a bounded check and
/// one that overclaims.
fn interface_bindings(code_lines: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for line in code_lines {
        let Some(at) = line.find("let ") else {
            continue;
        };
        let Some((pattern, value)) = line[at + "let ".len()..].split_once('=') else {
            continue;
        };
        if raw_next_offsets(value).is_empty() && interface_offsets(value).is_empty() {
            continue;
        }
        if !raw_next_offsets(value).is_empty() {
            continue;
        }
        let name: String = pattern
            .trim()
            .trim_start_matches("mut ")
            .trim()
            .chars()
            .take_while(|c| ident_char(*c))
            .collect();
        if !name.is_empty() {
            out.push(name);
        }
    }
    out
}

/// Offsets of `.interface` as a whole-token field access.
fn interface_offsets(code: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find(".interface") {
        let start = from + at;
        let after = start + ".interface".len();
        from = after;
        if !code[after..].starts_with(ident_char) {
            out.push(start);
        }
    }
    out
}

/// `<name>.next` as a field access on a binding known to be an `Interface`.
fn reads_binding_next(code: &str, name: &str) -> bool {
    let mut from = 0;
    while let Some(at) = code[from..].find(name) {
        let start = from + at;
        let after = start + name.len();
        from = after;
        if code[..start].ends_with(ident_char) {
            continue;
        }
        let Some(rest) = code[after..].trim_start().strip_prefix('.') else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix("next") else {
            continue;
        };
        if !rest.starts_with(ident_char) {
            return true;
        }
    }
    false
}

/// `.design/declared-blocks.md` AC-7. A mechanism a project is meant to *use*
/// is documented or it does not exist, and the reserved-fence list is exactly
/// the kind of rule someone hits at the worst moment.
///
/// Reads the constants rather than restating the strings, for the reason
/// [`ac9_conventions_document_the_prefixes_the_code_actually_reads`] does: a
/// doc check that hardcodes what it checks can pass while the code moves.
#[test]
fn ac7_conventions_document_the_declared_block_mechanism() {
    let text = std::fs::read_to_string(repo_root().join("docs/CONVENTIONS.md"))
        .expect("docs/CONVENTIONS.md should exist");

    for token in [
        day::blocks::FENCE_INFO,
        day::blocks::INJECTION_FENCE,
        day::blocks::CYCLE_FENCE,
        day::blocks::VERDICTS_FENCE,
    ] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document {token:?}"
        );
    }

    // Every reserved fence must appear, or a project learns the list by
    // being refused.
    for fence in day::blocks::RESERVED_FENCES {
        assert!(
            text.contains(*fence),
            "docs/CONVENTIONS.md should name {fence:?} as reserved"
        );
    }

    // The field-spec vocabulary a project actually writes.
    for token in ["required", "optional", "schema/blocks"] {
        assert!(
            text.contains(token),
            "docs/CONVENTIONS.md should document the field spec's {token:?}"
        );
    }

    // REQ-7's second half, and the part a summary would drop: *why* day's own
    // blocks are not declared this way. Asserted on the reasoning rather than
    // a heading, because the heading is not the content.
    assert!(
        text.contains("Why day's own blocks are not declared this way"),
        "CONVENTIONS.md should answer why the built-ins are struct-defined"
    );
    assert!(
        text.contains("no compiler between them"),
        "the reason is that a declaration beside a struct has no compiler \
         between them — CONVENTIONS.md should say so, since that is the whole \
         argument"
    );

    // The `block` predicate is the read path that makes declared blocks
    // matter; documenting the declaration without it would describe a
    // vocabulary nothing consults.
    assert!(
        text.contains("\"block\": \"research-claim\""),
        "CONVENTIONS.md should show the claim probe's `block` predicate"
    );
}

/// day#99 — every `` !`…` `` line in a shipped command's preamble must exit
/// zero, because the harness treats a non-zero preamble command as a load
/// failure and aborts the whole skill before the model sees any of it. One
/// unguarded `ls` of four orientation files (three of which do not exist in
/// this repo) made `/adversarial-review` unloadable here, and nothing noticed:
/// `tests/plugin.rs` checked what the command files SAY and never ran what
/// they DO.
///
/// Run in two working directories on purpose. The repo root alone is the
/// trap CLAUDE.md names — "a mechanism with two modes gets tested in whichever
/// mode this repo is in" — and for these lines the interesting mode is the one
/// where nothing exists: no `.design/`, no `docs/`, no git. That is every fresh
/// clone and every repo day is installed into, which is exactly the population
/// `telos/v1.0`'s bar names. A guard that only works where the files already
/// exist is not a guard.
#[test]
fn command_preambles_exit_zero_even_where_nothing_exists() {
    let shell = ["zsh", "bash", "sh"]
        .into_iter()
        .find(|s| {
            std::process::Command::new("command")
                .args(["-v", s])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
                || std::process::Command::new(s)
                    .arg("-c")
                    .arg("exit 0")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        })
        .expect("a POSIX shell must be available to check command preambles");

    let empty = std::env::temp_dir().join(format!("day-preamble-{}", std::process::id()));
    std::fs::create_dir_all(&empty).expect("temp dir for the nothing-exists case");

    let mut checked = 0usize;
    for rel in shipped_commands() {
        let rel = rel.as_str();
        let text = std::fs::read_to_string(repo_root().join(rel)).unwrap();

        for (n, line) in text.lines().enumerate() {
            let Some(rest) = line.split_once("!`") else {
                continue;
            };
            let Some((cmd, _)) = rest.1.split_once('`') else {
                panic!("{rel}:{} has an unterminated !` command", n + 1);
            };
            checked += 1;

            for (mode, cwd) in [
                ("repo root", repo_root()),
                ("nothing exists", empty.clone()),
            ] {
                let out = std::process::Command::new(shell)
                    .arg("-c")
                    .arg(cmd)
                    .current_dir(&cwd)
                    .output()
                    .unwrap_or_else(|e| panic!("{rel}:{} could not run under {shell}: {e}", n + 1));

                assert!(
                    out.status.success(),
                    "{rel}:{} exits {:?} under {shell} in the `{mode}` case, which \
                     aborts the skill load before the model sees anything.\n  \
                     command: {cmd}\n  stderr: {}",
                    n + 1,
                    out.status.code(),
                    String::from_utf8_lossy(&out.stderr).trim(),
                );
            }
        }
    }

    let _ = std::fs::remove_dir_all(&empty);

    // A generator whose failure mode is "less output" needs an exhaustive
    // expectation, not a trusting loop: if the `!` parse silently stopped
    // matching, every assertion above would pass by checking nothing.
    //
    // **The count stays hand-written; the FILE LIST does not.** Those are
    // different failure modes and only one of them is caught by a number. This
    // said 13 "across the two command files" while three shipped, because the
    // list was a literal and a third command could be added without touching
    // it — so the new file's four preambles went unchecked and the count still
    // matched. Deriving the list from `commands/` turned that into a red build
    // the moment it was wrong, which is how 17 got here.
    assert_eq!(
        checked, 17,
        "expected 17 `!` preamble commands across the shipped command files; found \
         {checked}. If a line was added or removed, update this number — if it \
         dropped to zero the parse broke and this test was asserting nothing."
    );
}

/// `.design/position-honesty.md` AC-8 — the boundary check is wired at the
/// **mechanism**, not at a caller.
///
/// This is day#101's rule as a scan. Three milestones produced three instances
/// of the same defect: a check that existed, was correct, and was called from
/// one of the paths that needed it. Each time the author's test drove the call
/// site they were thinking about, so each time it looked complete.
///
/// day#103 is the instance this guards. The boundary reconciliation existed and
/// was reachable only from `day assess docs` — a manual verb *downstream of the
/// step it detects* — so skipping the step skipped the alarm, and two
/// consecutive releases went unrecorded until the verb was run for an unrelated
/// reason. `status::compute` is the one place position is computed for every
/// channel, so a check there is inherited by the hooks, the status line and the
/// long form alike. Nothing but a scan would notice it being unwired.
///
/// **Scoped to what a scan can actually catch.** An earlier version of this test
/// also asserted that `position::infer` takes the whole `WitnessSchema` rather
/// than just its probes — the property that stops a channel computing a position
/// with the record witness silently skipped. That assertion is decoration: the
/// parameter type is compile-enforced, so no mutation can break the property
/// without breaking the build, and mutating it SURVIVED for exactly that reason.
/// Asserting it here would let this test claim credit the compiler earns, which
/// is the "test asserts a proxy" failure `CLAUDE.md` records. The invariant is
/// real and is held by `infer`'s signature; it does not need a scan, and saying
/// so is more useful than a green assertion that cannot fail.
///
/// Carries the same escape hatch as `a_failed_kan_read_is_never_swallowed`, for
/// the same reason: a test with no way out gets deleted the first time it is
/// wrong, and then the rule goes with it.
#[test]
fn the_boundary_check_is_wired_where_every_channel_reads() {
    const MARKER: &str = "position-guarantee-may-degrade:";

    let status = std::fs::read_to_string(repo_root().join("src/status.rs"))
        .expect("src/status.rs should be readable");

    // Comments stripped first: this repo deliberately keeps comments describing
    // past defects, and a scan that cries wolf about its own documentation gets
    // switched off.
    let live: String = status
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        live.contains("unrecorded_boundary(client, git)") || live.contains(MARKER),
        "`status::compute` must ask `docs::unrecorded_boundary`. day#103 is that \
         this check existed and was reachable only from `day assess docs` — a verb \
         downstream of the release step, so skipping the step skipped the alarm \
         and two consecutive releases went unrecorded. If it moved somewhere that \
         still reaches every channel, mark it `{MARKER} <why>`."
    );
}

/// Where a file's trailing `#[cfg(test)] mod` begins, or `lines.len()`.
///
/// **`#[cfg(test)]` on anything other than a `mod` does not end the production
/// half.** The first version cut at the first line starting with the attribute,
/// so a single `#[cfg(test)] use std::…;` near the top of a file — an ordinary
/// thing to write — exempted every `pub fn` below it from the scan, silently and
/// for the whole file. Found by a cold review probing the scan rather than
/// reading it.
fn cfg_test_module_line(lines: &[&str]) -> usize {
    let declares_a_module = |l: &str| {
        let t = l.trim_start();
        t.starts_with("mod ") || t.starts_with("pub mod ")
    };
    lines
        .iter()
        .enumerate()
        .find(|(n, l)| {
            let line = l.trim_start();
            if !line.starts_with("#[cfg(test)]") {
                return false;
            }
            // **The attribute and the `mod` on ONE line** — `#[cfg(test)] mod
            // tests {` — is the shape that made the first version of this fix
            // worse than what it replaced: it matched no `mod` on a later line,
            // returned `lines.len()`, and classified an entire test module as
            // production, so test-only callers counted as production callers and
            // the scan silently stopped firing for that file. All nineteen `src/`
            // files use the two-line form today, so it was latent, and its
            // failure direction is the silent one this whole fix exists to end.
            if declares_a_module(line.trim_start_matches("#[cfg(test)]")) {
                return true;
            }
            lines[*n..]
                .iter()
                .skip(1)
                .find(|next| !next.trim_start().starts_with('#') && !next.trim().is_empty())
                .is_some_and(|next| declares_a_module(next))
        })
        .map_or(lines.len(), |(n, _)| n)
}

/// A file's code with `//` comments removed and its trailing `#[cfg(test)]`
/// module split off. Shared by the two scans below.
fn production_half(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let cut = cfg_test_module_line(&lines);
    lines[..cut]
        .iter()
        .map(|l| match l.find("//") {
            Some(i) => l[..i].to_string(),
            None => (*l).to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn test_half(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let cut = cfg_test_module_line(&lines);
    lines[cut..].join("\n")
}

/// Every `.rs` file under a directory, as `(repo-relative path, text)`.
fn rust_sources(rel: &str) -> Vec<(String, String)> {
    let base = repo_root().join(rel);
    let mut out = Vec::new();
    let mut stack = vec![base.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let name = path
                    .strip_prefix(repo_root())
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                out.push((name, std::fs::read_to_string(&path).unwrap()));
            }
        }
    }
    out.sort();
    out
}

const TEST_ONLY_HATCH: &str = "test-only-caller-ok:";

/// The text after `pub fn ` on a definition line, allowing the qualifiers Rust
/// puts between them.
///
/// **`pub fn ` alone was not enough.** `pub async fn`, `pub const fn` and
/// `pub unsafe fn` all define a `pub fn` the scan could not see, and `src/` has
/// two `pub async fn` today (`cli::run`, `mcp::serve`). A scan whose stated
/// blind spot is "a name that also occurs elsewhere" while its real blind spot
/// is "three of the four ways to write the thing it looks for" overclaims, which
/// is what the doc comment is for.
fn pub_fn_name_start(line: &str) -> Option<&str> {
    let mut rest = line.trim_start().strip_prefix("pub ")?;
    // `pub(crate)` is deliberately not matched: dead-code detection is not
    // suppressed for it, so clippy already covers that case.
    for qualifier in ["async ", "const ", "unsafe ", "extern \"C\" "] {
        while let Some(stripped) = rest.strip_prefix(qualifier) {
            rest = stripped;
        }
    }
    rest.strip_prefix("fn ")
}

/// The scan itself, over a corpus rather than over the filesystem, so the two
/// directions of AC-20 can be driven with synthetic inputs instead of by writing
/// probe files into `src/` — which is how a real module declaration got reverted
/// by a probe in an earlier session.
fn pub_fns_with_only_test_callers(src: &[(String, String)], test_texts: &[String]) -> Vec<String> {
    let production: String = src
        .iter()
        .map(|(_, text)| production_half(text))
        .collect::<Vec<_>>()
        .join("\n");
    let tests: String = src
        .iter()
        .map(|(_, text)| test_half(text))
        .chain(test_texts.iter().cloned())
        .collect::<Vec<_>>()
        .join("\n");

    let mentions = |corpus: &str, name: &str| -> usize {
        corpus
            .match_indices(name)
            .filter(|(at, _)| {
                let before = corpus[..*at].chars().next_back();
                let after = corpus[at + name.len()..].chars().next();
                !before.is_some_and(ident_char) && !after.is_some_and(ident_char)
            })
            .count()
    };

    let mut offenders = Vec::new();
    for (path, text) in src {
        let code = production_half(text);
        let lines: Vec<&str> = code.lines().collect();
        let raw_lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            let Some(rest) = pub_fn_name_start(line) else {
                continue;
            };
            let name: String = rest.chars().take_while(|c| ident_char(*c)).collect();
            if name.is_empty() {
                continue;
            }
            // The definition itself is one mention; anything above one means a
            // caller exists somewhere that is not a test.
            if mentions(&production, &name) > 1 || mentions(&tests, &name) == 0 {
                continue;
            }
            let hatched = raw_lines[n.saturating_sub(6)..=n]
                .iter()
                .any(|l| l.contains(TEST_ONLY_HATCH));
            if !hatched {
                offenders.push(format!("{path}:{}: {name}", n + 1));
            }
        }
    }
    offenders
}

/// day#101 — **a `pub fn` whose only callers are tests fails the build.**
///
/// `BlockSchemas::extract` and `Compat::is_notable` were both exactly this: a
/// check that existed, was tested, and was called from nowhere. Both were `pub`,
/// and `pub` suppresses dead-code detection, so clippy was silent for both. A
/// `pub fn` whose only callers are `#[cfg(test)]` is either dead or a
/// requirement about to go nominal, and day#101's whole point is that
/// `/adversarial-review` was the only thing catching either.
///
/// **Measured before it was written, and validated against the instance it was
/// written for.** Over `src/` today: 163 `pub fn` definitions, **0 offenders**.
/// Run over the tree at `1e02220^` — the commit that dropped the dead code — it
/// reports exactly one, `src/compat.rs: is_notable`, and nothing else. A scan
/// that has never been shown to fire is a scan nobody has reason to believe.
///
/// **What it does not catch, so it does not overclaim:** a `pub fn` whose name
/// also occurs in production for an unrelated reason (a short or common
/// identifier) is not flagged — the mention count cannot tell a call from a
/// coincidence without a parser. That is a false *negative*, which is the safe
/// direction for a guard whose false positives would be hatched away.
#[test]
fn a_pub_fn_whose_only_callers_are_tests_fails_the_build() {
    let src = rust_sources("src");
    let tests: Vec<String> = rust_sources("tests")
        .into_iter()
        .map(|(_, text)| text)
        .collect();
    let offenders = pub_fns_with_only_test_callers(&src, &tests);

    assert!(
        !src.is_empty(),
        "could not check: no sources were read from src/"
    );
    assert!(
        offenders.is_empty(),
        "these `pub fn`s are called only from tests: {offenders:?}\n\n\
         Either wire the guarantee into the path that needs it, delete it, or \
         mark the definition `{TEST_ONLY_HATCH} <why this one is genuinely \
         different>`. `pub` suppresses dead-code detection, so nothing else \
         will tell you (day#101)."
    );
}

/// AC-20 — **both directions**: the scan flags the shape, and the hatch clears
/// it.
///
/// Synthetic corpora rather than probe files under `src/`: a scan asserted only
/// against a tree that has no offenders is a scan that has never been observed
/// to fail, which is this milestone's subject.
#[test]
fn the_test_only_caller_scan_fires_and_can_be_hatched() {
    let offender = (
        "src/probe.rs".to_string(),
        "pub fn only_tests_call_me() {}\n\n#[cfg(test)]\nmod tests {\n\
         #[test] fn t() { super::only_tests_call_me(); }\n}\n"
            .to_string(),
    );
    assert_eq!(
        pub_fns_with_only_test_callers(std::slice::from_ref(&offender), &[]).len(),
        1,
        "the scan must flag a pub fn reached only from a #[cfg(test)] module"
    );

    let hatched = (
        offender.0.clone(),
        format!(
            "// {TEST_ONLY_HATCH} it is the public API a downstream crate calls\n{}",
            offender.1
        ),
    );
    assert!(
        pub_fns_with_only_test_callers(&[hatched], &[]).is_empty(),
        "a per-site hatch must clear it — a check with no way out gets deleted \
         the first time it is wrong"
    );

    let called = (
        "src/probe.rs".to_string(),
        "pub fn used() {}\npub fn caller() { used(); }\n\n#[cfg(test)]\nmod tests {\n\
         #[test] fn t() { super::used(); }\n}\n"
            .to_string(),
    );
    assert!(
        pub_fns_with_only_test_callers(&[called], &[]).is_empty(),
        "a pub fn with a production caller must not be flagged"
    );
}

/// The `pub fn` scan sees the qualified forms, and a stray `#[cfg(test)]`
/// attribute does not exempt the rest of a file.
///
/// Both were found by a cold review probing the scan rather than reading it:
/// `pub async fn` / `pub const fn` / `pub unsafe fn` were invisible, and one
/// `#[cfg(test)] use …` anywhere above a definition silently truncated the
/// production half. `src/` has two `pub async fn` today, so the first was not
/// hypothetical.
#[test]
fn the_test_only_caller_scan_sees_qualified_definitions() {
    for qualifier in ["", "async ", "const ", "unsafe "] {
        let src = (
            "src/probe.rs".to_string(),
            format!(
                "pub {qualifier}fn only_tests_call_me() {{}}\n\n#[cfg(test)]\nmod tests {{\n\
                 #[test] fn t() {{ super::only_tests_call_me(); }}\n}}\n"
            ),
        );
        assert_eq!(
            pub_fns_with_only_test_callers(std::slice::from_ref(&src), &[]).len(),
            1,
            "`pub {qualifier}fn` must be seen; it defines a pub fn like any other"
        );
    }

    // The attribute and the `mod` on ONE line — the shape the first version of
    // `cfg_test_module_line` missed, classifying a whole test module as
    // production and silently switching the scan off for that file.
    let one_line = (
        "src/probe.rs".to_string(),
        "pub fn only_tests_call_me() {}\n\n#[cfg(test)] mod tests {\n\
         #[test] fn t() { super::only_tests_call_me(); }\n}\n"
            .to_string(),
    );
    assert_eq!(
        pub_fns_with_only_test_callers(std::slice::from_ref(&one_line), &[]).len(),
        1,
        "`#[cfg(test)] mod tests {{` on one line must still end the production half"
    );

    // A `#[cfg(test)]` on something that is not a `mod` must not end the
    // production half.
    let stray = (
        "src/probe.rs".to_string(),
        "#[cfg(test)]\nuse std::fmt;\n\npub fn only_tests_call_me() {}\n\n#[cfg(test)]\n\
         mod tests {\n#[test] fn t() { super::only_tests_call_me(); }\n}\n"
            .to_string(),
    );
    assert_eq!(
        pub_fns_with_only_test_callers(std::slice::from_ref(&stray), &[]).len(),
        1,
        "one `#[cfg(test)] use` must not exempt every definition below it"
    );
}

/// **The historical validation is asserted, not recounted.**
///
/// `CLAUDE.md` said the scan "is asserted against the tree at `1e02220^`", and
/// nothing asserted it — a one-time manual measurement written in the grammar of
/// an enforced constraint, which is the shape this milestone exists to remove.
/// It is now run: the scan is pure text, so the historical tree needs a checkout
/// and no build.
///
/// It reports could-not-check rather than passing when the commit is not
/// reachable, which a shallow clone would cause.
#[test]
fn the_test_only_caller_scan_finds_the_instance_it_was_written_for() {
    const BEFORE_THE_FIX: &str = "1e02220^";

    let rev = std::process::Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            &format!("{BEFORE_THE_FIX}{{commit}}"),
        ])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    if !rev.status.success() {
        panic!(
            "could not check: {BEFORE_THE_FIX} is not reachable, so the one tree \
             known to contain an instance cannot be scanned. That is not a pass \
             — a shallow clone is the usual cause, and ci.yml fetches full \
             history for exactly this kind of reason."
        );
    }

    let work = tempfile::tempdir().expect("a scratch dir");
    let tree = work.path().join("tree");
    let added = std::process::Command::new("git")
        .args([
            "worktree",
            "add",
            "--detach",
            &tree.to_string_lossy(),
            BEFORE_THE_FIX,
        ])
        .current_dir(repo_root())
        .output()
        .expect("git should be runnable");
    assert!(
        added.status.success(),
        "could not check: the historical worktree could not be created: {}",
        String::from_utf8_lossy(&added.stderr)
    );

    let read_all = |sub: &str| -> Vec<(String, String)> {
        let mut out = Vec::new();
        let mut stack = vec![tree.join(sub)];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let name = path.strip_prefix(&tree).unwrap_or(&path);
                    out.push((
                        name.to_string_lossy().to_string(),
                        std::fs::read_to_string(&path).unwrap(),
                    ));
                }
            }
        }
        out.sort();
        out
    };
    let src = read_all("src");
    let tests: Vec<String> = read_all("tests").into_iter().map(|(_, t)| t).collect();
    let offenders = pub_fns_with_only_test_callers(&src, &tests);

    let _ = std::process::Command::new("git")
        .args(["worktree", "remove", "--force", &tree.to_string_lossy()])
        .current_dir(repo_root())
        .output();

    assert_eq!(
        offenders.len(),
        1,
        "the scan must find exactly the instance day#101 named at \
         {BEFORE_THE_FIX}, and nothing else; got {offenders:?}"
    );
    assert!(
        offenders[0].contains("compat.rs") && offenders[0].ends_with("is_notable"),
        "the one offender must be `Compat::is_notable`; got {offenders:?}"
    );
}

/// `.design/witness-interview.md` AC-4 — **the unwitnessed-telos remedy has
/// exactly one renderer.**
///
/// day printed this advice from two places, `assess telos` and `bridge check`,
/// each formatting its own prose. The wording had already drifted apart, which
/// is the condition under which a third arrives and nobody notices — day#101's
/// shape, and CLAUDE.md's standing rule that a guarantee about what day reports
/// belongs in the mechanism rather than at a call site.
///
/// **Keyed on the presence of the phrase, never on its absence.** CLAUDE.md
/// records a classifier that looked for `composition: ok` to mean "loaded it
/// anyway" and mis-filed a reader when an unrelated finding suppressed the
/// phrase. So this asserts where [`day::telos::UNWITNESSED`] *does* appear, not
/// where it does not.
///
/// **`src/status.rs` is deliberately not in scope, and that is a decision
/// rather than an oversight.** Its "no witness probes are declared" message
/// reports a *project-level* fact — `schema/witness` declares no readable probe
/// at all — which is upstream of any telos and independent of it, as
/// `position::unordered`'s comment already states. day#108 proposed routing that
/// reader to `day init` and it was rejected because that verb "records a
/// `schema/design-doc` starter and no witnesses at all — a remedy that does not
/// remedy this". Routing it to the interview would reintroduce exactly that,
/// since no telos is in question. The two separate cleanly on text that already
/// differs, which is why this scan can be precise.
///
/// **What this does not catch, stated so it does not overclaim:** prose that
/// conveys the same advice in different words. A scan matches text, not
/// meaning. It catches the concrete failure that occurred — a second site
/// emitting this phrase — and not a paraphrase of it.
///
/// **The exact string `--witness <type>` is reserved**, not merely discouraged.
/// It is the signature of the remedy that was removed, and the scan cannot tell
/// it apart from an innocent mention — it caught `parse_witness_any`'s arity
/// error, which was legitimately naming the flag to use instead. That message
/// now says `--witness` without the placeholder, which costs it nothing and
/// keeps this check able to key on a string with exactly one meaning. Prefer
/// rewording over the escape hatch when the site is not a remedy at all.
///
/// **The phrase is a literal here, not `day::telos::UNWITNESSED`.** The first
/// version imported a constant the fix introduced, and `revert-demo.py`
/// reported `DID-NOT-COMPILE` — reverting the fix took the constant with it, so
/// the test could not run and the demonstration said nothing about coverage. A
/// scan asserting a fact about source *text* has to own the text it matches, for
/// the same reason: sharing a constant means a rename moves both sides together
/// and the scan silently checks something else.
#[test]
fn the_unwitnessed_remedy_has_one_renderer() {
    const MARKER: &str = "unwitnessed-remedy-elsewhere:";
    const PHRASE: &str = "declares no witnesses";
    // The one legitimate emitter, located by its own definition rather than by
    // file. A second hand-rolled site inside `telos.rs` is caught the same way
    // one in another module is — the failure was three sites across three
    // files, and keying on the file would have exempted the worst case.
    const RENDERER: &str = "pub fn unwitnessed_remedy";

    let mut offenders = Vec::new();
    let mut solo_guess = Vec::new();
    let mut renderers = 0usize;
    let mut stack = vec![repo_root().join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).unwrap();
            // `production_half` rather than a hand-rolled `#[cfg(test)]` cut.
            // The first version of this cut at the first line carrying the
            // attribute, which is the exact defect `cfg_test_module_line`
            // documents: a single `#[cfg(test)] use std::…;` near the top would
            // exempt every line below it, silently and for the whole file. It
            // also strips comments, so this repo's habit of describing past
            // defects in prose does not read as a live one.
            let code = production_half(&raw);
            let raw_lines: Vec<&str> = raw.lines().collect();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            // The renderer's own body is the one place the phrase belongs.
            // Located by definition rather than by file, so a second site in
            // `telos.rs` is caught exactly as one elsewhere is.
            let span = code.find(RENDERER).map(|at| {
                let from = code[..at].lines().count();
                let rest = &code[at..];
                let len = rest
                    .find("\n}")
                    .map(|e| rest[..e].lines().count())
                    .unwrap_or_else(|| rest.lines().count());
                (from, from + len)
            });
            if span.is_some() {
                renderers += 1;
            }

            for (n, line) in code.lines().enumerate() {
                // Markers are read from the RAW lines: the marker lives in a
                // comment, and `production_half` has already stripped those.
                if raw_lines.get(n).is_some_and(|l| l.contains(MARKER)) {
                    continue;
                }
                if line.contains(PHRASE) && !span.is_some_and(|(a, b)| n >= a && n <= b) {
                    offenders.push(format!("{name}:{}", n + 1));
                }
                // The remedy this replaced. It handed the reader a command
                // inviting a solo guess at a witness, which day#86 records as
                // worse than the state it purports to fix.
                if line.contains("--witness <type>") {
                    solo_guess.push(format!("{name}:{}", n + 1));
                }
            }
        }
    }

    assert_eq!(
        renderers, 1,
        "expected exactly one `{RENDERER}` in src/, found {renderers}"
    );
    assert!(
        offenders.is_empty(),
        "the unwitnessed-telos remedy is rendered outside `telos::unwitnessed_remedy` at \
         {offenders:?}.\n\n\
         Two call sites rendered this independently and their wording had already drifted; \
         collapse the new site into that function, or mark it `{MARKER} <why this one is \
         genuinely a different fact>` — as `status.rs`'s project-level message would be."
    );
    assert!(
        solo_guess.is_empty(),
        "the solo-guess remedy `--witness <type>` is emitted at {solo_guess:?}.\n\n\
         Telling a reading agent to declare a witness itself is what this replaced: a \
         trivially satisfiable witness reports the telos met forever (day#86), and a bad \
         witness is worse than none. Point at `/witness-interview <slug>` instead."
    );
}

/// **A claim shape has exactly one evaluator.**
///
/// A cold review blocked this milestone on `every_subject` matching shapes with
/// `ClaimShape::matches_with` directly, which skips everything
/// `probe::claims_matching` adds around it — `block` resolution and
/// `mentions_material`. Both predicates were silently inert inside an `every`
/// probe, and inert toward *satisfied*: an identical shape reported
/// `[ERROR] … day cannot check it` through a claim probe and `[MATERIAL]`
/// through the universal.
///
/// **It had already been noticed and written down.** `src/probe.rs` carried a
/// comment saying block predicates were not resolved there, "stated because
/// silently ignoring a predicate is precisely what this milestone is about" —
/// and the comment did not mention `mentions_material` at all, which is the
/// second instance the first one predicted. CLAUDE.md's rule is exactly this:
/// prose in the right place is not a constraint, and a rule that matters wants a
/// source scan.
///
/// So the shape of the guarantee is the one that rule prescribes: not "call the
/// other predicates too", but **one place where a shape is evaluated at all**.
/// A second call site is a second evaluator, and it will drift.
#[test]
fn the_claim_shape_predicate_has_one_evaluator() {
    const MARKER: &str = "second-shape-evaluator:";
    let src = std::fs::read_to_string(repo_root().join("src/probe.rs")).unwrap();
    let code = production_half(&src);
    let raw: Vec<&str> = src.lines().collect();

    let mut definitions = 0usize;
    let mut callers = Vec::new();
    for (n, line) in code.lines().enumerate() {
        // A window, not the line itself. The marker lives in a comment and the
        // call it exempts is usually a few lines below it — the same shape
        // `a_pub_fn_with_only_test_callers_fails_the_build` uses, and getting it
        // wrong makes the hatch unusable rather than making the scan stricter.
        let hatched = raw[n.saturating_sub(8)..=n.min(raw.len() - 1)]
            .iter()
            .any(|l| l.contains(MARKER));
        if hatched {
            continue;
        }
        if line.contains("fn matches_with(") {
            definitions += 1;
        } else if line.contains(".matches_with(") {
            callers.push(n + 1);
        }
    }

    assert_eq!(
        definitions, 1,
        "expected exactly one `matches_with` definition, found {definitions}"
    );
    assert_eq!(
        callers.len(),
        1,
        "a claim shape is evaluated at {} sites in production code (lines {callers:?}), and \
         only `claims_matching` resolves `block` and `mentions_material`.\n\n\
         A second evaluator silently drops whichever predicates it does not know about, \
         toward satisfied — which is what a cold review blocked this milestone on. Route \
         the new site through `claims_matching`, or mark it `{MARKER} <why this one cannot \
         drop a predicate>`.",
        callers.len()
    );
}
