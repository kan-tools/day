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

#[test]
fn ac7_and_ac8_the_plugin_ships_both_atoms_as_commands() {
    for (file, must_contain) in [
        ("commands/design.md", "design document"),
        ("commands/adversarial-review.md", "APPROVE WITH FOLLOW-UPS"),
    ] {
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
/// marker makes the exception visible in review instead.
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
            let text: String = std::fs::read_to_string(&path)
                .unwrap()
                .lines()
                .map(|l| match l.find("//") {
                    Some(i) => l[..i].to_string(),
                    None => l.to_string(),
                })
                .collect::<Vec<_>>()
                .join("\n");

            for read in reads {
                let mut from = 0;
                while let Some(at) = text[from..].find(read) {
                    let start = from + at + read.len();
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
                    if swallows.iter().any(|s| expr.contains(s)) {
                        let line = text[..start].matches('\n').count() + 1;
                        offenders.push(format!("{}:{line}", path.display()));
                    }
                }
            }
        }
    }

    // The marker opts a file out, and has to say why.
    let allowed: Vec<String> = offenders
        .iter()
        .filter(|o| {
            let file = o.split(':').next().unwrap_or_default();
            std::fs::read_to_string(file)
                .map(|t| t.contains(MARKER))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let unexplained: Vec<&String> = offenders.iter().filter(|o| !allowed.contains(o)).collect();
    assert!(
        unexplained.is_empty(),
        "a kan read's failure is swallowed at {unexplained:?}.\n\n\
         \"day could not read this\" and \"there is nothing here\" must not be \
         spelled the same way — that has been five separate defects. Either \
         propagate the error and report it (see `status::unreadable_from`), or \
         mark the site `{MARKER} <why this one is genuinely different>`."
    );
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
    for rel in ["commands/design.md", "commands/adversarial-review.md"] {
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
    // matching, every assertion above would pass by checking nothing. This is
    // the count the two files carry today.
    assert_eq!(
        checked, 13,
        "expected 13 `!` preamble commands across the two command files; found \
         {checked}. If a line was added or removed, update this number — if it \
         dropped to zero the parse broke and this test was asserting nothing."
    );
}
