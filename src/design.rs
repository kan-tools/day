//! Design-document parsing and validation — the mechanically-checkable part
//! of the `/design` atom, moved out of the prompt.
//!
//! Every rule here was previously prose instructing a model to count things.
//! Models are poor at counting and excellent at reporting that they counted,
//! which is the same failure the adversarial-review atom exists to catch, one
//! level up. A linter cannot mis-report its own arithmetic.
//!
//! Parsing is deliberately shallow — heading lines, ID tokens, fenced-block
//! tracking, backtick-quoted paths. It is not a Markdown AST: anything that
//! needs real document understanding stays in the command's prose, where a
//! model belongs.

use std::collections::BTreeSet;
use std::path::Path;

use crate::schema::Schema;

/// A design document, sliced into sections by `##` headings.
#[derive(Debug, Clone)]
pub struct Document {
    pub title: Option<String>,
    sections: Vec<(String, String)>,
    /// Lines outside fenced code blocks, for checks that must not fire on
    /// examples (a doc explaining `TODO` markers is not itself unfinished).
    prose: String,
}

impl Document {
    pub fn parse(text: &str) -> Self {
        let mut title = None;
        let mut sections: Vec<(String, String)> = Vec::new();
        let mut prose = String::new();
        let mut in_fence = false;

        for line in text.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if !in_fence {
                prose.push_str(line);
                prose.push('\n');

                if let Some(rest) = trimmed.strip_prefix("## ") {
                    sections.push((rest.trim().to_string(), String::new()));
                    continue;
                }
                if title.is_none() {
                    if let Some(rest) = trimmed.strip_prefix("# ") {
                        title = Some(rest.trim().to_string());
                    }
                }
            }
            if let Some((_, body)) = sections.last_mut() {
                body.push_str(line);
                body.push('\n');
            }
        }

        Self {
            title,
            sections,
            prose,
        }
    }

    pub fn section(&self, heading: &str) -> Option<&str> {
        self.sections
            .iter()
            .find(|(h, _)| h.eq_ignore_ascii_case(heading))
            .map(|(_, body)| body.as_str())
    }

    /// IDs *declared* with the given prefix — a line whose first token after
    /// list punctuation is `<prefix><n>:`. Distinguished from mere mentions,
    /// which is what makes the coverage check meaningful.
    pub fn declared_ids(&self, prefix: &str) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        for line in self.prose.lines() {
            let t = line
                .trim_start()
                .trim_start_matches(['-', '*', '+'])
                .trim_start()
                .trim_start_matches("[ ]")
                .trim_start_matches("[x]")
                .trim_start();
            if let Some(rest) = t.strip_prefix(prefix) {
                let num: String = rest.chars().take_while(char::is_ascii_digit).collect();
                if !num.is_empty() && rest[num.len()..].starts_with(':') {
                    ids.insert(format!("{prefix}{num}"));
                }
            }
        }
        ids
    }

    /// Every occurrence of `<prefix><n>` anywhere in the given text.
    fn mentioned_ids(text: &str, prefix: &str) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        let mut rest = text;
        while let Some(at) = rest.find(prefix) {
            let after = &rest[at + prefix.len()..];
            let num: String = after.chars().take_while(char::is_ascii_digit).collect();
            if !num.is_empty() {
                ids.insert(format!("{prefix}{num}"));
            }
            rest = &rest[at + prefix.len()..];
        }
        ids
    }

    /// Backtick-quoted tokens that look like file paths.
    ///
    /// A slash alone is not enough: design docs are full of backticked
    /// things containing slashes that are not files — kan subject names
    /// (`telos/composable-process`), slash commands (`/design`), and
    /// templated placeholders (`schema/<slug>`). Requiring a file
    /// extension or a trailing slash, and rejecting the other three shapes,
    /// is what keeps this check from crying wolf on a doc's own prose.
    fn quoted_paths(text: &str) -> Vec<String> {
        let mut paths = Vec::new();
        for (i, chunk) in text.split('`').enumerate() {
            // Odd indices are the insides of backtick pairs.
            if i % 2 != 1 {
                continue;
            }
            // `src/store/log.rs:207` cites a line; the file is the part
            // that has to exist.
            let candidate = chunk
                .trim_end_matches(['.', ',', ')'])
                .split(':')
                .next()
                .unwrap_or_default();
            if looks_like_path(candidate) {
                paths.push(candidate.to_string());
            }
        }
        paths
    }

    /// Bullet lines under `heading`, each becoming one `decide` claim.
    pub fn bullets(&self, heading: &str) -> Vec<String> {
        let Some(body) = self.section(heading) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut current: Option<String> = None;
        for line in body.lines() {
            let t = line.trim_start();
            if let Some(rest) = t.strip_prefix("- ") {
                if let Some(done) = current.take() {
                    out.push(done);
                }
                current = Some(rest.trim().to_string());
            } else if let (false, Some(c)) = (t.is_empty(), current.as_mut()) {
                // Continuation of a wrapped bullet.
                c.push(' ');
                c.push_str(t);
            } else if t.is_empty() {
                if let Some(done) = current.take() {
                    out.push(done);
                }
            }
        }
        if let Some(done) = current {
            out.push(done);
        }
        out.into_iter()
            .map(|b| b.replace("**", "").trim().to_string())
            .filter(|b| !b.is_empty())
            .collect()
    }

    /// The first non-empty line of the Summary section, for claim text.
    pub fn summary_line(&self) -> Option<String> {
        let body = self.section("Summary")?;
        let mut sentence = String::new();
        for line in body.lines() {
            let t = line.trim();
            if t.is_empty() {
                if !sentence.is_empty() {
                    break;
                }
                continue;
            }
            if !sentence.is_empty() {
                sentence.push(' ');
            }
            sentence.push_str(t);
        }
        (!sentence.is_empty()).then_some(sentence)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Warn,
    Fail,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub verdict: Verdict,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub findings: Vec<Finding>,
    pub open_questions: usize,
}

impl Report {
    /// A document passes when nothing failed. Warnings (open questions) do
    /// not fail it — an explicitly-marked unknown is a feature of a design
    /// doc, not a defect in one.
    pub fn is_clean(&self) -> bool {
        !self.findings.iter().any(|f| f.verdict == Verdict::Fail)
    }

    pub fn render(&self) -> String {
        let mut out = String::from("Design doc validation:\n");
        for finding in &self.findings {
            out.push_str(&format!(
                "  [{}] {}\n",
                finding.verdict.label(),
                finding.message
            ));
        }
        if self.open_questions > 0 {
            out.push_str(&format!(
                "  [OPEN] {} unresolved open question(s) remain\n",
                self.open_questions
            ));
        }
        out
    }

    /// One-line form, for embedding in the claim a design is recorded as —
    /// so the record carries how well-formed the document was at the time,
    /// rather than that being ephemeral terminal output.
    pub fn summary(&self) -> String {
        let failed = self
            .findings
            .iter()
            .filter(|f| f.verdict == Verdict::Fail)
            .count();
        let warned = self
            .findings
            .iter()
            .filter(|f| f.verdict == Verdict::Warn)
            .count();
        format!(
            "validation: {} check(s), {failed} failed, {warned} warning(s), {} open question(s)",
            self.findings.len(),
            self.open_questions,
        )
    }
}

/// Runs every schema rule over a parsed document. `base` is the directory
/// file-existence checks resolve against (the repo root, normally).
pub fn check(doc: &Document, schema: &Schema, base: &Path) -> Report {
    let mut findings = Vec::new();

    for section in &schema.sections {
        match doc.section(section) {
            Some(body) if !body.trim().is_empty() => findings.push(Finding {
                verdict: Verdict::Pass,
                message: format!("section present: {section}"),
            }),
            Some(_) => findings.push(Finding {
                verdict: Verdict::Fail,
                message: format!("section is empty: {section}"),
            }),
            None => findings.push(Finding {
                verdict: Verdict::Fail,
                message: format!("section missing: {section}"),
            }),
        }
    }

    let requirements = doc.declared_ids(&schema.requirement_prefix);
    let criteria = doc.declared_ids(&schema.criterion_prefix);

    findings.push(count_finding(
        requirements.len(),
        schema.min_requirements,
        "requirements",
    ));
    findings.push(count_finding(
        criteria.len(),
        schema.min_criteria,
        "acceptance criteria",
    ));

    // Coverage: every declared requirement must be referenced somewhere in
    // the acceptance-criteria section.
    if !requirements.is_empty() {
        let ac_section = schema
            .sections
            .iter()
            .find(|s| s.to_lowercase().contains("acceptance"))
            .cloned()
            .unwrap_or_else(|| "Acceptance Criteria".to_string());
        let covered = doc
            .section(&ac_section)
            .map(|body| Document::mentioned_ids(body, &schema.requirement_prefix))
            .unwrap_or_default();
        let uncovered: Vec<&String> = requirements.difference(&covered).collect();
        if uncovered.is_empty() {
            findings.push(Finding {
                verdict: Verdict::Pass,
                message: "every requirement is referenced by an acceptance criterion".to_string(),
            });
        } else {
            // Warn rather than fail: many good docs map criteria to
            // requirements *positionally* (AC-1 covers REQ-1) instead of
            // naming them, and this check cannot see that convention.
            // Failing on something undetectable trains people to ignore the
            // tool; surfacing it is the honest strength of the signal.
            for id in uncovered {
                findings.push(Finding {
                    verdict: Verdict::Warn,
                    message: format!(
                        "{id} is not named by any acceptance criterion \
                         (fine if criteria map positionally; explicit is better)"
                    ),
                });
            }
        }
    }

    let present: Vec<&String> = schema
        .placeholders
        .iter()
        .filter(|p| contains_token(&doc.prose, p))
        .collect();
    for placeholder in &present {
        findings.push(Finding {
            verdict: Verdict::Fail,
            message: format!("placeholder text present: {placeholder}"),
        });
    }
    if !schema.placeholders.is_empty() && present.is_empty() {
        findings.push(Finding {
            verdict: Verdict::Pass,
            message: "no placeholder text".to_string(),
        });
    }

    if !schema.paths_section.is_empty() {
        if let Some(body) = doc.section(&schema.paths_section) {
            let paths = Document::quoted_paths(body);
            let missing: Vec<&String> = paths.iter().filter(|p| !base.join(p).exists()).collect();
            // The rule is grounding, not omniscience: a design must point at
            // code that exists, but an Architecture section naming files it
            // intends to *create* is doing its job — so an unresolved path
            // warns, and only a total absence of real ones fails.
            if paths.is_empty() || missing.len() == paths.len() {
                findings.push(Finding {
                    verdict: Verdict::Fail,
                    message: format!(
                        "{} references no existing file — a design should be grounded in real code",
                        schema.paths_section
                    ),
                });
            } else {
                findings.push(Finding {
                    verdict: Verdict::Pass,
                    message: format!(
                        "{} of {} referenced path(s) exist ({})",
                        paths.len() - missing.len(),
                        paths.len(),
                        schema.paths_section
                    ),
                });
            }
            {
                for path in missing {
                    findings.push(Finding {
                        verdict: Verdict::Warn,
                        message: format!("referenced path does not exist yet: {path}"),
                    });
                }
            }
        }
    }

    Report {
        findings,
        open_questions: doc.prose.matches("<!-- OPEN").count(),
    }
}

/// Whether `token` appears in `text` as a standalone word rather than
/// inside a longer one. Without this, the placeholder `TODO` matches the
/// filename `docs/SETUP-TODO.md` — found by running this check over kan's
/// own design docs.
fn contains_token(text: &str, token: &str) -> bool {
    let boundary = |c: Option<char>| match c {
        None => true,
        Some(c) => !c.is_alphanumeric() && c != '-' && c != '_',
    };
    let mut from = 0;
    while let Some(at) = text[from..].find(token) {
        let start = from + at;
        let end = start + token.len();
        let before = text[..start].chars().next_back();
        let after = text[end..].chars().next();
        if boundary(before) && boundary(after) {
            return true;
        }
        from = end;
    }
    false
}

/// Whether a backticked token is a repo-relative file path rather than a
/// subject name, slash command, or template placeholder.
fn looks_like_path(s: &str) -> bool {
    if s.is_empty()
        || !s.contains('/')
        || s.contains(char::is_whitespace)
        // A slash command, not a path.
        || s.starts_with('/')
        // A template like `schema/<slug>`, not a real file.
        || s.contains('<')
        || s.contains('>')
        // Outside the repo, or elided (`~/.cargo/.../lib.rs`) — real, but
        // not this repo's to resolve.
        || s.starts_with('~')
        || s.contains("...")
    {
        return false;
    }
    // A directory, or a file with an extension. A bare `a/b` is a subject
    // name as often as a path, so it does not qualify.
    s.ends_with('/')
        || s.rsplit('/')
            .next()
            .is_some_and(|last| last.contains('.') && !last.ends_with('.'))
}

fn count_finding(found: usize, min: usize, noun: &str) -> Finding {
    if found >= min {
        Finding {
            verdict: Verdict::Pass,
            message: format!("{noun}: {found}"),
        }
    } else {
        Finding {
            verdict: Verdict::Fail,
            message: format!("{noun}: {found} (schema requires at least {min})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = "# Feature: thing\n\n## Summary\nIt does a thing.\nMore summary.\n\n\
        ## Requirements\n- REQ-1: first\n- REQ-2: second\n\n\
        ## Acceptance Criteria\n- [ ] AC-1: checks first (REQ-1)\n- [ ] AC-2: checks second (REQ-2)\n\n\
        ## Architecture\nTouches `src/design.rs` and `src/schema.rs`.\n\n\
        ## Resolved Questions\n- **Q1 — a**: chose a\n- **Q2 — b**: chose b\n";

    fn schema() -> Schema {
        Schema::starter()
    }

    #[test]
    fn parses_sections_and_title() {
        let doc = Document::parse(DOC);
        assert_eq!(doc.title.as_deref(), Some("Feature: thing"));
        assert!(doc.section("Summary").unwrap().contains("does a thing"));
        assert!(
            doc.section("architecture").is_some(),
            "lookup is case-insensitive"
        );
    }

    #[test]
    fn declared_ids_ignore_mere_mentions() {
        let doc = Document::parse(DOC);
        assert_eq!(doc.declared_ids("REQ-").len(), 2);
        // AC-1 mentions REQ-1 but does not declare it.
        assert_eq!(doc.declared_ids("AC-").len(), 2);
    }

    #[test]
    fn a_clean_document_passes() {
        let doc = Document::parse(DOC);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.is_clean(), "{}", report.render());
    }

    #[test]
    fn an_uncovered_requirement_warns_and_is_named() {
        let text = DOC.replace(
            "- [ ] AC-2: checks second (REQ-2)",
            "- [ ] AC-2: checks second",
        );
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report
            .render()
            .contains("REQ-2 is not named by any acceptance criterion"));
        assert!(
            report.is_clean(),
            "warns rather than fails: positional AC/REQ correspondence is a \
             convention this check cannot see"
        );
    }

    #[test]
    fn a_missing_section_fails_and_is_named() {
        let text = DOC.replace("## Architecture", "## Design Notes");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.render().contains("section missing: Architecture"));
    }

    #[test]
    fn a_nonexistent_referenced_path_fails() {
        let text = DOC.replace("`src/schema.rs`", "`src/not_a_real_file.rs`");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.render().contains("src/not_a_real_file.rs"));
    }

    /// Found by running `day design check` on the very design doc that
    /// specified it: subject names, slash commands, and templates all
    /// contain slashes and were all being reported as missing files.
    #[test]
    fn subject_names_slash_commands_and_templates_are_not_paths() {
        for not_a_path in [
            "telos/composable-process",
            "schema/<slug>",
            "atom/<slug>",
            "/design",
            "/adversarial-review",
            "src/cli",
        ] {
            assert!(
                !looks_like_path(not_a_path),
                "{not_a_path:?} should not be treated as a file path"
            );
        }
        for path in [
            "src/design.rs",
            "docs/CONVENTIONS.md",
            ".design/scaffold.md",
            "src/transport/git_tree.rs",
            "src/cli/",
        ] {
            assert!(looks_like_path(path), "{path:?} should be a file path");
        }
    }

    /// Found by running this check over kan's own design docs: the
    /// placeholder `TODO` matched the filename `docs/SETUP-TODO.md`.
    #[test]
    fn a_placeholder_inside_a_longer_word_does_not_count() {
        assert!(contains_token("there is a TODO here", "TODO"));
        assert!(contains_token("TODO", "TODO"));
        assert!(!contains_token("see `docs/SETUP-TODO.md` for more", "TODO"));
        assert!(!contains_token("a TODOS list", "TODO"));
    }

    /// Also from kan's docs: a design doc's Architecture section legitimately
    /// names files it intends to create, so an unresolved path warns while a
    /// section with no real paths at all still fails.
    #[test]
    fn a_proposed_file_warns_but_a_grounded_section_still_passes() {
        // One real file, one the design proposes to create.
        let text = DOC.replace("`src/schema.rs`", "`src/not_yet_written.rs`");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.is_clean(), "{}", report.render());
        assert!(
            report.render().contains("does not exist yet"),
            "{}",
            report.render()
        );

        // No real files at all: the design is ungrounded.
        let text = DOC
            .replace("`src/design.rs`", "`src/nope.rs`")
            .replace("`src/schema.rs`", "`src/also_nope.rs`");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(
            !report.is_clean(),
            "a wholly ungrounded section should fail"
        );
    }

    #[test]
    fn a_line_reference_resolves_to_its_file() {
        let text = DOC.replace("`src/design.rs`", "`src/design.rs:207`");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.is_clean(), "{}", report.render());
    }

    #[test]
    fn placeholders_inside_fenced_blocks_do_not_count() {
        let text = format!("{DOC}\n```\nTODO: this is an example, not a real placeholder\n```\n");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.is_clean(), "{}", report.render());
    }

    #[test]
    fn open_question_markers_warn_without_failing() {
        let text = format!("{DOC}\n<!-- OPEN: Q3 -->\nstill deciding\n<!-- /OPEN -->\n");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert_eq!(report.open_questions, 1);
        assert!(report.is_clean(), "an explicit unknown is not a defect");
    }

    #[test]
    fn bullets_become_one_entry_each() {
        let doc = Document::parse(DOC);
        let bullets = doc.bullets("Resolved Questions");
        assert_eq!(bullets.len(), 2);
        assert!(bullets[0].starts_with("Q1"));
    }

    #[test]
    fn summary_line_joins_the_first_paragraph() {
        let doc = Document::parse(DOC);
        assert_eq!(
            doc.summary_line().as_deref(),
            Some("It does a thing. More summary.")
        );
    }
}
