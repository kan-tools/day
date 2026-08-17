//! `.design/cross-harness-packaging.md` AC-1..AC-5, AC-8 — day is packaged as a
//! conformant Agent Plugins 1.0.0 plugin, its atoms are portable skills, and the
//! `` !`command` `` pre-execution that made day#99 and day#100 possible is gone.
//!
//! **Why these live in their own file rather than in `tests/plugin.rs`.** That
//! file asserts the *Claude Code* packaging, and REQ-7 is that Claude Code
//! behaviour is unchanged by this move — so the two sets have to be able to fail
//! independently. A single file would let a conformance fix be "verified" by a
//! Claude Code assertion, which is the measuring-the-wrong-thing-accurately
//! defect `CLAUDE.md` records for the kan-conformance floor.

use serde_json::Value;
use std::path::PathBuf;

/// The repo root. **Deliberately NOT `common::repo_root`**, though it is the
/// same one line: `mod common;` drags in `tests/common/mod.rs`, which calls
/// `std::os::unix::fs::symlink` — so importing it would make this target fail to
/// COMPILE on Windows. Nothing in this file needs unix, and this is the
/// conformance suite for a cross-harness portability milestone, so being the one
/// integration target that runs on every platform is the point rather than a
/// nicety. `.github/workflows/ci.yml`'s `windows` job runs exactly this target.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The ten top-level fields the closed 1.0.0 plugin schema permits.
/// `additionalProperties` is `false` at the root, verified against
/// <https://agent-plugins.org/schemas/1.0.0/plugin.schema.json>, so any
/// Claude Code-specific key belongs under `extensions` and never beside these.
const PERMITTED_PLUGIN_KEYS: &[&str] = &[
    "$schema",
    "name",
    "version",
    "description",
    "author",
    "homepage",
    "repository",
    "license",
    "keywords",
    "extensions",
];

fn read_json(rel: &str) -> Value {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should exist and be readable: {e}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{} should be valid JSON: {e}", path.display()))
}

/// Every skill the plugin ships, **read from the directory rather than listed**.
///
/// Derived for the reason `tests/plugin.rs` records for `commands/`: a cold
/// review found the third command absent from three separate hand-maintained
/// enumerations, its preambles unchecked, while the globbing test picked it up
/// immediately. A list that reads the directory cannot fail to grow.
///
/// Discovery matches §7.1: each immediate child of `skills/` holding a regular
/// file named exactly `SKILL.md` is one skill, and clients MUST NOT recurse
/// deeper. This walks exactly that far, so a skill nested one level too deep
/// fails here rather than being silently invisible to every client.
fn shipped_skills() -> Vec<String> {
    let dir = repo_root().join("skills");
    let mut out: Vec<String> = std::fs::read_dir(&dir)
        .expect("skills/ should exist — it is what the plugin ships")
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter(|e| e.path().join("SKILL.md").is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    out.sort();
    out
}

fn skill_text(name: &str) -> String {
    let path = repo_root().join("skills").join(name).join("SKILL.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} should be readable: {e}", path.display()))
}

/// The frontmatter block, and the body after it, split once at the closing `---`.
fn split_frontmatter(text: &str, name: &str) -> (String, String) {
    let rest = text
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("{name}/SKILL.md must open with YAML frontmatter"));
    let (front, body) = rest
        .split_once("\n---\n")
        .unwrap_or_else(|| panic!("{name}/SKILL.md frontmatter is not terminated by `---`"));
    (front.to_string(), body.to_string())
}

fn frontmatter_field(front: &str, key: &str) -> Option<String> {
    front.lines().find_map(|l| {
        l.strip_prefix(&format!("{key}:"))
            .map(|v| v.trim().to_string())
    })
}

/// AC-1 — `plugin.json` satisfies the constraints day depends on, **field by
/// field and by type**, not merely by key name.
///
/// **This test shipped a fatal manifest defect and could not see it.** Its first
/// version scanned top-level *key names* against the permitted set and stopped
/// there. `plugin.json` carried `"author": "Maxine Levesque"` — a string where
/// §5.4 defines an object of `name`/`email`/`url` — and every key name was
/// legal, so the scan passed. A cold review caught it with a real validator.
///
/// The mechanism is the part worth remembering. §5.2 names exactly two
/// non-fatal violations — an unknown top-level field, and a non-object
/// `extensions` — and makes everything else fatal: *"the client MUST reject the
/// plugin and MUST NOT discover or execute any of its components."* The scan was
/// aimed precisely at the non-fatal category and was structurally incapable of
/// reaching the fatal one, so it read as coverage while guaranteeing the
/// cheapest thing available. day would have loaded **nothing** on a conformant
/// client: no skills, no MCP server, on the milestone whose whole subject is
/// being loadable elsewhere.
///
/// **Scoped honestly.** This asserts the constraints *day's own manifest* has to
/// meet, which is what `CLAUDE.md` allows a cell like this to assert. It is not
/// a JSON Schema implementation and must not pretend to be one — the schemas use
/// `$ref`, `oneOf`, `not` and `propertyNames`, and a hand-rolled validator for
/// those would be its own defect surface. **The authoritative check is
/// `.github/workflows/agent-plugins.yml`**, which fetches both published schemas
/// and validates both manifests with a real validator on every push, so a
/// constraint this file does not model still fails the build.
#[test]
fn ac1_plugin_manifest_conforms_to_the_closed_1_0_0_schema() {
    let manifest = read_json("plugin.json");
    let obj = manifest
        .as_object()
        .expect("plugin.json must be a JSON object");

    assert_eq!(
        obj.get("$schema").and_then(|v| v.as_str()),
        Some("https://agent-plugins.org/schemas/1.0.0/plugin.schema.json"),
        "$schema is required by the 1.0.0 plugin schema and pins the version a \
         client validates against"
    );
    assert_eq!(obj.get("name").and_then(|v| v.as_str()), Some("day"));

    for key in obj.keys() {
        assert!(
            PERMITTED_PLUGIN_KEYS.contains(&key.as_str()),
            "plugin.json carries top-level key {key:?}, which the closed 1.0.0 \
             schema does not permit (additionalProperties is false). Client-specific \
             data belongs under `extensions`, keyed by a reverse-domain namespace — \
             never beside the ten permitted fields: {PERMITTED_PLUGIN_KEYS:?}"
        );
    }

    // EVERY field day ships, checked for TYPE — the half whose absence was the
    // defect. A key name being legal says nothing about the value being legal,
    // and only one of those two is fatal.
    // A flat list, not `(key, want_string)` pairs: every pair carried `true`, so
    // the parameter was machinery around `assert!(v.is_string())` that read as
    // though some field might legitimately not be a string. `author`,
    // `keywords` and `extensions` are the non-string fields and each is checked
    // against its own shape below.
    for key in [
        "$schema",
        "name",
        "version",
        "description",
        "homepage",
        "repository",
        "license",
    ] {
        if let Some(v) = obj.get(key) {
            assert!(
                v.is_string(),
                "plugin.json's {key:?} must be a string; got {v}. §5.2: any schema \
                 violation but an unknown top-level field or a non-object \
                 `extensions` is FATAL — the client rejects the whole plugin."
            );
        }
    }

    // §5.4, the exact constraint that shipped broken: `author` is an OBJECT, and
    // may contain only `name`, `email` and `url`, each a string.
    if let Some(author) = obj.get("author") {
        let author = author.as_object().unwrap_or_else(|| {
            panic!(
                "plugin.json's `author` must be an OBJECT of name/email/url, not \
                 {author}. This exact defect shipped: a bare string passed a \
                 key-name scan and would have made a conformant client reject \
                 every component day ships (§5.4, §5.2)."
            )
        });
        assert!(
            !author.is_empty(),
            "an empty `author` object says less than no `author` field at all"
        );
        for (k, v) in author {
            assert!(
                ["name", "email", "url"].contains(&k.as_str()),
                "plugin.json's `author` may contain only name/email/url; got {k:?}. \
                 §5.4: any other field makes the manifest invalid."
            );
            assert!(
                v.is_string(),
                "plugin.json's `author.{k}` must be a string; got {v}"
            );
        }
    }

    if let Some(keywords) = obj.get("keywords") {
        let arr = keywords
            .as_array()
            .unwrap_or_else(|| panic!("`keywords` must be an array; got {keywords}"));
        for k in arr {
            assert!(k.is_string(), "every keyword must be a string; got {k}");
        }
    }

    // §8.1: `extensions` is an object keyed by reverse-domain namespace whose
    // values are objects. A non-object `extensions` is one of the two NON-fatal
    // violations, which is exactly why it is worth failing here — CI's validator
    // would report it and a client would shrug it off.
    if let Some(ext) = obj.get("extensions") {
        let ext = ext
            .as_object()
            .unwrap_or_else(|| panic!("`extensions` must be an object; got {ext}"));
        for (ns, v) in ext {
            assert!(
                ns.contains('.'),
                "`extensions` keys are reverse-domain namespaces; {ns:?} is not one"
            );
            assert!(
                v.is_object(),
                "`extensions.{ns}` must be an object; got {v}"
            );
        }
    }

    // §5.5's name constraints, asserted rather than assumed: 1–64 chars,
    // lowercase alphanumeric plus hyphen and period, alphanumeric at both ends,
    // no `--` and no `..`.
    let name = obj["name"].as_str().unwrap();
    assert!(
        (1..=64).contains(&name.len()),
        "name must be 1–64 characters"
    );
    assert!(
        name.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.'),
        "name may only contain lowercase alphanumerics, hyphens and periods"
    );
    assert!(
        name.starts_with(|c: char| c.is_ascii_alphanumeric())
            && name.ends_with(|c: char| c.is_ascii_alphanumeric()),
        "name must start and end alphanumeric"
    );
    assert!(
        !name.contains("--") && !name.contains(".."),
        "name may not contain consecutive hyphens or periods"
    );
}

/// AC-2 — `mcp.json` conforms, and cannot drift from `.mcp.json`.
///
/// §7.2.2 is why the `$schema` assertion is not decoration: a client that finds
/// a malformed `mcp.json` "MUST disable MCP for that plugin and continue loading
/// other component types". The failure is silent and partial — the skills load,
/// the server does not — which is the shape this repo keeps recording as worse
/// than a crash.
///
/// day#157's claim that "the MCP half is done and already portable" is what
/// REQ-2 exists to correct: `.mcp.json` has no `$schema` and would be rejected
/// outright.
#[test]
fn ac2_mcp_manifest_conforms_and_matches_the_claude_code_one() {
    let mcp = read_json("mcp.json");
    let obj = mcp.as_object().expect("mcp.json must be a JSON object");

    assert_eq!(
        obj.get("$schema").and_then(|v| v.as_str()),
        Some("https://agent-plugins.org/schemas/1.0.0/mcp.schema.json"),
        "the 1.0.0 MCP schema requires $schema and pins its exact value"
    );
    let mut keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["$schema", "mcpServers"],
        "the 1.0.0 MCP schema permits exactly these two top-level keys \
         (additionalProperties is false); anything else disables MCP for the \
         whole plugin under §7.2.2"
    );

    // REQ-9: a bare executable name, resolved by the platform's search rules.
    // A plugin-relative command would be the right answer only if day bundled
    // the binary, which it deliberately does not — the crate ships to crates.io
    // and bundling would make the plugin a second release path able to drift
    // from `scripts/cut-release.sh`.
    let day = &mcp["mcpServers"]["day"];
    assert_eq!(day["type"].as_str(), Some("stdio"));
    assert_eq!(
        day["command"].as_str(),
        Some("day"),
        "REQ-9: the command is a bare executable name the user installs with \
         `cargo install day`, not a plugin-relative path"
    );

    // The anti-drift half. Two manifests is the intended shape — Agent Plugins
    // defines no discovery, so the marketplace entry that makes
    // `/plugin install day@kan-tools` work is not replaced by it — but two
    // manifests describing *different servers* is how the shape goes wrong.
    let claude = read_json(".mcp.json");
    assert_eq!(
        mcp["mcpServers"], claude["mcpServers"],
        "mcp.json and .mcp.json must describe the same server. They are separate \
         files on purpose (different schemas, different clients) and that is \
         exactly why nothing but a test stops them diverging."
    );
}

/// AC-3 — the skills are discoverable where the spec says to look, and each
/// declares the name its directory already gives it.
///
/// **The count and the list are different guarantees**, and `CLAUDE.md` records
/// what it cost to conflate them: a hand-written `checked == 13` still matched
/// while a third command file went entirely unchecked. The list is derived, so a
/// skill that was never added fails; the count is exact, so a `SKILL.md` glob
/// that silently stopped matching fails too. Neither substitutes for the other.
#[test]
fn ac3_every_skill_declares_the_name_of_its_directory() {
    let skills = shipped_skills();

    assert_eq!(
        skills,
        [
            "adversarial-review",
            "design",
            "handoff",
            "install",
            "wakeup",
            "witness-interview"
        ],
        "the five process atoms plus the portable installation skill"
    );
    assert_eq!(
        skills.len(),
        6,
        "expected exactly 6 skills; if this dropped to zero the SKILL.md \
         discovery broke and every assertion below was checking nothing"
    );

    for name in &skills {
        let text = skill_text(name);
        let (front, body) = split_frontmatter(&text, name);

        let declared = frontmatter_field(&front, "name").unwrap_or_else(|| {
            panic!(
                "skills/{name}/SKILL.md has no `name` in its frontmatter. The Agent \
                 Skills specification requires it and requires it to match the parent \
                 directory."
            )
        });
        assert_eq!(
            &declared, name,
            "skills/{name}/SKILL.md declares name {declared:?}, which must equal its \
             directory name"
        );

        // The spec's own constraints on `name`, so a rename cannot land invalid.
        assert!(
            declared
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "{name}: skill names are lowercase alphanumerics and hyphens only"
        );
        assert!(
            !declared.starts_with('-') && !declared.ends_with('-'),
            "{name}: a skill name may not start or end with a hyphen"
        );
        assert!(
            !declared.contains("--"),
            "{name}: a skill name may not contain consecutive hyphens"
        );

        let description = frontmatter_field(&front, "description")
            .unwrap_or_else(|| panic!("skills/{name}/SKILL.md needs a `description`"));
        assert!(
            !description.is_empty() && description.len() <= 1024,
            "{name}: description must be non-empty and at most 1024 characters"
        );

        if name != "install" {
            assert!(
                body.contains("```day-atom"),
                "skills/{name}/SKILL.md should still declare its atom interface — the \
                 packaging moved, the vocabulary did not"
            );
        } else {
            assert!(
                body.contains("Installation is portable plugin content"),
                "the support skill must state its harness-neutral boundary"
            );
        }
    }
}

/// AC-4 — **no `` !`command` `` survives anywhere in a skill body**, and this
/// scan carries no exemption hatch.
///
/// Deliberately unlike `a_failed_kan_read_is_never_swallowed`, which has a
/// documented escape because a rule with no way out gets deleted the first time
/// it is wrong. There is no such case here: a portable client does not strip the
/// syntax, it renders it literally, so the agent receives backtick text where
/// day promised data. That is day#100's exact shape exported to every harness,
/// and there is no repo in which it is the right thing to write.
///
/// This scan replaces `command_preambles_exit_zero_even_where_nothing_exists`,
/// which asserted the preambles exit zero. That test was right about its own
/// question and the question stopped existing.
#[test]
fn ac4_no_skill_body_pre_executes_a_command() {
    let mut scanned = 0usize;
    for name in shipped_skills() {
        let text = skill_text(&name);
        scanned += 1;
        for (n, line) in text.lines().enumerate() {
            assert!(
                !line.contains("!`"),
                "skills/{name}/SKILL.md:{} contains a `` !`command` `` preamble.\n  \
                 {}\n\
                 Pre-execution is removed outright (REQ-4), not conditioned on the \
                 target harness. A portable client renders the backticks literally, \
                 so the agent gets text where day promised data; and under Claude \
                 Code a non-zero preamble is a load failure that aborts the skill \
                 before the model sees any of it (day#99). Make the read an explicit \
                 instruction in the body, and say what to do when it fails.",
                n + 1,
                line.trim(),
            );
        }
    }
    assert_eq!(
        scanned, 6,
        "expected to scan 6 skill bodies; if this dropped the scan was asserting \
         nothing"
    );
}

/// AC-5 — every read a skill instructs says what to do when that read fails.
///
/// This is `telos/honest-reads` applied to the one channel its source scan
/// structurally cannot see: `a_failed_kan_read_is_never_swallowed` greps `src/`,
/// and a skill body is not `src/`. `src/probe.rs` states the rule — a subject
/// day cannot read is an error and never a silently empty result — and
/// `CLAUDE.md` records five violations of it *inside* `src/`, which is the
/// argument for not trusting prose to propagate here either.
///
/// The marker is a fixed string rather than a prose sniff on purpose: the
/// failure this guards is a bullet added later without one, and only an exact
/// token makes that a red build rather than a judgement call.
#[test]
fn ac5_every_instructed_read_names_its_failure_handling() {
    const MARKER: &str = "**If this read fails:**";

    let mut bullets_checked = 0usize;
    for name in shipped_skills() {
        let text = skill_text(&name);

        let context = text
            .split_once("## Context")
            .unwrap_or_else(|| {
                panic!(
                    "skills/{name}/SKILL.md has no `## Context` section. Every skill \
                     gathers state before it acts; the section is where the failure \
                     handling for those reads lives."
                )
            })
            .1;
        // Up to the next top-level heading.
        let context = context.split("\n## ").next().unwrap();

        // Each read is one `- **Label** — command.` bullet. Split on the bullet
        // marker at line start rather than on blank lines, since a bullet wraps.
        let bullets: Vec<&str> = context
            .split("\n- **")
            .skip(1) // prose before the first bullet
            .collect();
        assert!(
            !bullets.is_empty(),
            "skills/{name}/SKILL.md's Context section instructs no reads. If that is \
             genuinely true the section should say so; if a bullet shape changed, \
             this test just stopped checking anything."
        );

        for bullet in bullets {
            let label = bullet.split("**").next().unwrap_or("?").trim();
            assert!(
                bullet.contains('`'),
                "skills/{name}/SKILL.md, Context bullet {label:?}: a read bullet must \
                 name the command it runs, in backticks"
            );
            assert!(
                bullet.contains(MARKER),
                "skills/{name}/SKILL.md, Context bullet {label:?}: no {MARKER:?} \
                 clause.\n\
                 REQ-5: a read that fails is reported, never silently empty. day#100 \
                 is the instance — a telos read matched nothing, a fallback printed \
                 `none`, and every adversarial review in this repo measured against \
                 CLAUDE.md instead of nine live teloi, exit zero throughout. Every \
                 read this skill instructs must say what to do when it fails."
            );
            bullets_checked += 1;
        }
    }

    // **Exact, not a floor.** `CLAUDE.md`: "Keep the count exact — it catches a
    // parser that silently stopped matching — and derive the list, which catches
    // a member that was never added. Neither substitutes for the other." This
    // said `>= 20` against 28 actual bullets, so eight could have stopped being
    // parsed with the test still green — the count half of the rule doing none
    // of its job while looking like it was. The LIST is derived by
    // `shipped_skills()`; this is the other half.
    assert_eq!(
        bullets_checked, 35,
        "expected exactly 35 instructed reads across the six skills, found \
         {bullets_checked}. If a bullet was added or removed, update this number \
         — if it dropped sharply the bullet parse broke and every assertion above \
         was checking nothing."
    );
}

/// AC-5's named instance, asserted specifically rather than left to the general
/// rule above.
///
/// day#100 was not "a read failed". It was a read that failed *and produced the
/// text of a legitimate empty result*, so the review that consumed it could not
/// tell the difference. The three skills that read teloi must say that an
/// unreadable log is not "no teloi" — a generic failure clause does not carry
/// that, and `CLAUDE.md`'s rule is that a test covering a finding asserts the
/// finding rather than the feature around it.
#[test]
fn ac5_the_telos_reads_distinguish_unreadable_from_empty() {
    for name in ["adversarial-review", "design", "witness-interview"] {
        let text = skill_text(name);
        assert!(
            text.contains("telos/"),
            "skills/{name}/SKILL.md should instruct a read of the telos subjects"
        );
        // Normalised before matching: emphasis, sentence case and **line
        // wrapping** are prose decisions, and a matcher sensitive to any of
        // them fails on a rewording that changed nothing. These bodies are
        // hard-wrapped at 80 columns, so every phrase long enough to be worth
        // asserting spans a newline. Match the claim, not its typography.
        let flat = text.replace("**", "").to_lowercase();
        let flat = flat.split_whitespace().collect::<Vec<_>>().join(" ");
        let says_so = flat.contains("do not treat it as an empty log")
            || flat.contains("do not proceed as though the project has no teloi")
            || flat.contains("do not fall through");
        assert!(
            says_so,
            "skills/{name}/SKILL.md reads teloi and does not say that a FAILED read \
             is different from an empty one. That distinction is day#100 itself: \
             `none` was printed for nine live teloi and every review measured against \
             the wrong north star at exit zero."
        );
    }
}

/// AC-8 — no atom's prose exists in two places.
///
/// `CLAUDE.md` records rule-drift-across-two-locations as a defect class twice,
/// and this would be a third instance. It would also simply collide: Claude Code
/// folds `commands/` and `skills/` into one inventory keyed by name, so a file
/// at `commands/design.md` and a skill at `skills/design/` both create `/design`.
///
/// Asserted as "the directory does not exist, or shares no name", not as "the
/// directory does not exist" — the rule is about duplication, and a project
/// legitimately shipping an unrelated command should not fail here.
#[test]
fn ac8_no_atom_ships_as_both_a_command_and_a_skill() {
    let commands = repo_root().join("commands");
    if !commands.is_dir() {
        return;
    }
    let skills = shipped_skills();
    for entry in std::fs::read_dir(&commands)
        .expect("commands/ is readable")
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_none_or(|x| x != "md") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        assert!(
            !skills.contains(&stem),
            "{stem} ships as both commands/{stem}.md and skills/{stem}/SKILL.md. \
             One atom, one body: the two files are one inventory to Claude Code and \
             two sources of truth to everyone else."
        );
    }
}

/// The manifests' `version` fields are **derived from `Cargo.toml`, checked
/// here** — the one form of hand-written number this repo permits.
///
/// `CLAUDE.md` records a stale hand-maintained count being wrong in four
/// separate places, and a version string is the same failure with a worse blast
/// radius: without it, plugin versions resolve to commit SHAs and a user sees
/// `28599cf98e5b`. The field has to be a literal because neither manifest is
/// generated, so the derivation happens here instead.
#[test]
fn the_manifest_versions_track_the_crate_version() {
    let cargo = std::fs::read_to_string(repo_root().join("Cargo.toml")).unwrap();
    let version = cargo
        .lines()
        .find_map(|l| l.strip_prefix("version = \""))
        .and_then(|v| v.strip_suffix('"'))
        .expect("Cargo.toml should declare a package version");

    for manifest in ["plugin.json", ".claude-plugin/plugin.json"] {
        assert_eq!(
            read_json(manifest)["version"].as_str(),
            Some(version),
            "{manifest} declares a version that is not the crate's ({version}). \
             `scripts/cut-release.sh` bumps Cargo.toml; these two follow it, and \
             nothing but this test says so."
        );
    }
}

/// **`allowed-tools` is deliberately NOT in the specification's serialization**,
/// and this test exists so that nobody silently "fixes" it.
///
/// The Agent Skills specification defines `allowed-tools` as a *space-separated*
/// string (`Bash(git:*) Read`). day's five bodies carry Claude Code's
/// *comma-separated* form (`Bash(kan *), Read`), which the design document
/// asserted would "survive" unchanged — the assertion was wrong, and it was
/// wrong in the way `CLAUDE.md` names: a justification about a mechanism whose
/// own specification says otherwise.
///
/// Keeping the comma form is a decision, not an oversight. REQ-7 (Claude Code
/// behaviour unchanged) has an acceptance criterion; conformance of `allowed-tools`
/// has none, because the field is optional, marked experimental, and validated by
/// no schema — Agent Plugins defers the skill format entirely. And the divergence
/// fails safe: a strict client splitting on spaces gets tokens that match no
/// tool, so it grants *fewer* permissions and prompts, which is what
/// `telos/affordance-not-enforcement` would choose anyway. Recorded as RQ-5.
#[test]
fn the_allowed_tools_divergence_from_the_spec_is_deliberate() {
    for name in shipped_skills() {
        let text = skill_text(&name);
        let (front, _) = split_frontmatter(&text, &name);
        // **NOT a `let ... else { continue }`**, which is how this test was
        // first written and why it guaranteed nothing. RQ-5 weighs three
        // options — convert the field, DROP it, or keep the divergence — and a
        // `continue` on absence passes for exactly the one RQ-5 rejected:
        // removing `allowed-tools` from all five bodies left this green. That is
        // `CLAUDE.md`'s "a read that fails is never a silently empty result"
        // wearing a control-flow keyword.
        let tools = frontmatter_field(&front, "allowed-tools").unwrap_or_else(|| {
            panic!(
                "skills/{name}/SKILL.md declares no `allowed-tools`. Dropping the \
                 field is the option RQ-5 explicitly rejected: it degrades Claude \
                 Code from pre-approved to prompting, which is a REQ-7 change. If \
                 that is now the intent, change RQ-5 first."
            )
        });
        assert!(
            tools.contains(", "),
            "skills/{name}/SKILL.md's `allowed-tools` is no longer Claude Code's \
             comma-separated form. If that was a deliberate move to the Agent Skills \
             space-separated serialization, REQ-7 needs re-measuring first — \
             `.design/cross-harness-packaging.md` RQ-5 records why the comma form was \
             kept, and this test is the thing that makes changing it a decision."
        );
    }
}
