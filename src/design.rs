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

    /// Prose with inline code spans removed, for the checks that must not
    /// fire on a document *quoting* a marker rather than carrying one.
    ///
    /// [`Self::prose`] already excludes fenced blocks, and its comment states
    /// the intent — "a doc explaining `TODO` markers is not itself
    /// unfinished". Inline spans are the case that intent missed, and it bit:
    /// `commands/design.md` tells an author to end every open question with
    /// "remove the `<!-- OPEN -->` marker", so each genuine open question
    /// contributed *two* matches — its real `<!-- OPEN: Qn -->` opener and
    /// the literal marker quoted in the instruction. A doc with two open
    /// questions reported four.
    ///
    /// Spans are matched per line, so an unbalanced backtick swallows at most
    /// the rest of that line rather than the remainder of the document. Nested
    /// or multi-backtick delimiters (``` `` ` `` ```) are not handled; they do
    /// not occur in a design document, and guessing at them would cost more
    /// than it returns.
    fn prose_outside_code_spans(&self) -> String {
        let mut out = String::with_capacity(self.prose.len());
        for line in self.prose.lines() {
            let mut in_span = false;
            for c in line.chars() {
                match c {
                    '`' => in_span = !in_span,
                    _ if !in_span => out.push(c),
                    _ => {}
                }
            }
            out.push('\n');
        }
        out
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
            let rest = strip_list_prefix(line);
            if let Some(rest) = rest.strip_prefix(prefix) {
                let num: String = rest.chars().take_while(char::is_ascii_digit).collect();
                if !num.is_empty() && rest[num.len()..].starts_with(':') {
                    ids.insert(format!("{prefix}{num}"));
                }
            }
        }
        ids
    }

    /// Lines that *look* like a declaration — a list item beginning with
    /// `<prefix><digits>` — but whose id the strict `<prefix><n>:` form cannot
    /// parse, because a non-colon continues the id (`REQ-11a`, `REQ-11.1`).
    ///
    /// day#55: such a line was **silently dropped** — not rejected, dropped —
    /// so the count came up short and coverage passed vacuously for a
    /// requirement no criterion named. It bit twice writing this milestone's
    /// own design doc. The fix keeps the id format strict and makes the drop
    /// loud: the caller turns each of these into a finding. Returned as the
    /// full offending token (`REQ-11a`) so the finding can name the exact line.
    pub fn malformed_ids(&self, prefix: &str) -> Vec<String> {
        let mut out = Vec::new();
        for line in self.prose.lines() {
            let rest = strip_list_prefix(line);
            let Some(rest) = rest.strip_prefix(prefix) else {
                continue;
            };
            let num: String = rest.chars().take_while(char::is_ascii_digit).collect();
            if num.is_empty() {
                continue;
            }
            let after = &rest[num.len()..];
            // A colon is the valid form; a space or other punctuation is a
            // loose mention, not a declaration. Only an id that *continues*
            // into a letter or dot is a declaration the strict form dropped.
            if after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '.') {
                let tail: String = after
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '.')
                    .collect();
                out.push(format!("{prefix}{num}{tail}"));
            }
        }
        out
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
    /// A check that could not be run, as distinct from one that ran and found
    /// nothing. `docs::Level` has carried this distinction since day#81; this
    /// enum did not, so `design check`'s only way to describe an unanswerable
    /// question was to answer it. day#105 is what that cost: coverage reported
    /// `[PASS]` for a document with no acceptance criteria to cover anything.
    ///
    /// Deliberately **not** a failure. `is_clean` stays false only for `Fail`,
    /// because an unrunnable check is not a defect in the document — it is a
    /// gap in what was asserted about it, and saying so is the whole point.
    Unchecked,
}

impl Verdict {
    fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
            Self::Unchecked => "UNCHECKED",
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
        // The unchecked count belongs here rather than only in the rendered
        // output: this string is what the *claim* carries, and a summary that
        // reports "0 failed" while silently omitting that a check could not run
        // is the same overstatement day#105 was about, one level up. A record
        // that says how much was verified must also say how much was not.
        let unchecked = self
            .findings
            .iter()
            .filter(|f| f.verdict == Verdict::Unchecked)
            .count();
        format!(
            "validation: {} check(s), {failed} failed, {warned} warning(s), \
             {unchecked} unchecked, {} open question(s)",
            self.findings.len(),
            self.open_questions,
        )
    }
}

/// Runs every schema rule over a parsed document. `base` is the directory
/// file-existence checks resolve against (the repo root, normally).
/// day#41: findings about decisions **already recorded on the subject** that
/// the document may not cover.
///
/// Found during kan's v0.7 release, where it cost two decided requirements: five
/// items were decided in conversation and recorded as `decide` claims, the
/// release was re-scoped, a new design doc was written **from the session rather
/// than from the log**, and two of the five fell through. Nothing rejected them.
/// One was recovered by accident, hours later, when the defect it fixed caused a
/// false data-loss alarm.
///
/// **Advisory, and deliberately not a matcher.** Matching a decision's prose to
/// a requirement is inexact, and a check that guessed would either miss the case
/// it exists for or cry wolf until it was switched off. So: where a decision
/// carries a resolution id (day#36) the coverage check is *exact*; where it does
/// not, day lists what is on the record and asks the reader to confirm. The two
/// issues compose — ids are what make this precise rather than a heuristic.
pub fn check_against_record(doc: &Document, schema: &Schema, recorded: &[String]) -> Vec<Finding> {
    if recorded.is_empty() {
        return Vec::new();
    }

    let declared: std::collections::BTreeSet<String> = doc
        .bullets(&schema.resolved_section)
        .iter()
        .filter_map(|b| crate::record::resolution_id(b, &schema.resolution_prefix))
        .collect();

    let mut identified = Vec::new();
    let mut unidentified = 0usize;
    for text in recorded {
        match crate::record::resolution_id(text, &schema.resolution_prefix) {
            Some(id) => {
                if !declared.contains(&id) {
                    identified.push(id);
                }
            }
            None => unidentified += 1,
        }
    }

    let mut out = Vec::new();
    if !identified.is_empty() {
        out.push(Finding {
            verdict: Verdict::Warn,
            message: format!(
                "{} decision(s) recorded on this subject are not in the document: {}. \
                 A design written from the session rather than from the log is how \
                 decided items get dropped",
                identified.len(),
                identified.join(", ")
            ),
        });
    }
    if unidentified > 0 {
        out.push(Finding {
            verdict: Verdict::Warn,
            message: format!(
                "{unidentified} decision(s) on this subject carry no `{}` id, so day \
                 cannot tell whether this document covers them — check by hand, or \
                 give them ids",
                schema.resolution_prefix
            ),
        });
    }
    out
}

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

    // day#55: a declaration-shaped line whose id the strict form cannot parse
    // used to vanish, taking its count and its coverage requirement with it.
    // Surface each one so the silence is impossible — the id format stays
    // strict, the checker just stops swallowing what it cannot read.
    for prefix in [&schema.requirement_prefix, &schema.criterion_prefix] {
        for bad in doc.malformed_ids(prefix) {
            findings.push(Finding {
                verdict: Verdict::Warn,
                message: format!(
                    "`{bad}` looks like a declaration but its id is not the strict form \
                     `{prefix}<n>:` (e.g. `{prefix}1:`) — it is not counted or \
                     coverage-checked. Renumber it, or the requirement it names is invisible \
                     to validation"
                ),
            });
        }
    }

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
    //
    // day#105: this used to run on `!requirements.is_empty()` alone, and
    // computes coverage from *mentions* of requirement ids inside the criteria
    // section — never from whether a criterion was declared. A document with
    // six requirements and no parseable criteria therefore got
    // "every requirement is referenced by an acceptance criterion" in the same
    // run that failed the criteria count at zero: two findings about one
    // absence, disagreeing.
    //
    // With no criteria the question is not satisfied, it is unanswerable, so it
    // reports unchecked. This is `CLAUDE.md`'s rule for verification tooling —
    // could-not-check outranks checked-and-clean — and it is day#55's other
    // half: that fix stopped a malformed id from vanishing out of the *count*
    // and left it vanishing out of *coverage*, which is what the comment above
    // the malformed-id loop already says it was supposed to prevent.
    if !requirements.is_empty() && criteria.is_empty() {
        findings.push(Finding {
            verdict: Verdict::Unchecked,
            message: format!(
                "requirement coverage not checked: {} requirement(s) declared and no \
                 acceptance criteria, so there is nothing to cover them",
                requirements.len()
            ),
        });
    } else if !requirements.is_empty() {
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

    // Quoted markers are examples, not defects — the same rule fenced blocks
    // already got, extended to inline spans. A design doc that writes
    // `` `TODO` `` while explaining the convention is not unfinished.
    let unquoted = doc.prose_outside_code_spans();

    let present: Vec<&String> = schema
        .placeholders
        .iter()
        .filter(|p| contains_token(&unquoted, p))
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
        open_questions: unquoted.matches("<!-- OPEN").count(),
    }
}

/// Strips a line's leading list punctuation and checkbox, so `- [ ] REQ-1:`
/// and `REQ-1:` both present their id at the start. Shared by [`declared_ids`]
/// and [`malformed_ids`] so the two agree on what a declaration line looks
/// like.
///
/// [`declared_ids`]: Document::declared_ids
/// [`malformed_ids`]: Document::malformed_ids
fn strip_list_prefix(line: &str) -> &str {
    line.trim_start()
        .trim_start_matches(['-', '*', '+'])
        .trim_start()
        .trim_start_matches("[ ]")
        .trim_start_matches("[x]")
        .trim_start()
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
/// day's own subject namespaces, taken from the constants that define them
/// rather than re-typed here.
///
/// day#136: `telos/v1.0` was warned about as a missing file, in a design
/// document whose subject *is* that telos. It reaches the extension test below
/// because `v1.0` looks exactly like a filename with an extension — and the
/// pressure that creates is the one day#84 describes, where the cheapest way to
/// silence the warning is to write "the v1.0 telos" and lose the subject a
/// later reader would run `kan show` against.
///
/// Sourced from the constants because a second hand-written list is this
/// repo's most-repeated defect; `every_subject_prefix_is_excluded_from_paths`
/// scans `src/` and fails the build when a sixth namespace is added without
/// being added here.
const SUBJECT_PREFIXES: &[&str] = &[
    crate::atoms::ATOM_PREFIX,
    crate::atoms::TELOS_PREFIX,
    crate::bridge::BRIDGE_PREFIX,
    crate::schema::SCHEMA_PREFIX,
    crate::tension::TENSION_PREFIX,
];

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
        // A kan subject in one of day's own namespaces. No file will ever
        // exist there, and day defines the namespace.
        || SUBJECT_PREFIXES.iter().any(|p| s.starts_with(p))
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

    /// day#105 — the pair, not either line alone.
    ///
    /// `design check` computes coverage from *mentions* of requirement ids
    /// inside the acceptance-criteria section, never from whether a criterion
    /// was declared. A document whose criteria are written in a form the strict
    /// parser rejects therefore declared zero criteria and still passed
    /// coverage, because the rejected lines left their `REQ-n` references
    /// behind in the section text.
    ///
    /// The assertion is deliberately on **both** findings at once. The criteria
    /// count was already correct — it failed at zero, which is how this was
    /// noticed — so a test checking only the count would have passed before the
    /// fix and proves nothing. What was wrong was the two findings coexisting:
    /// one saying there are no criteria, one saying every requirement has one.
    #[test]
    fn coverage_is_unchecked_rather_than_passed_when_no_criteria_are_declared() {
        // Criteria in the `- AC-1 (REQ-1): …` form: a real shape a person
        // writes, rejected by the strict `AC-<n>:` parser, and not flagged by
        // `malformed_ids` either — a space after the digits is treated as a
        // loose mention by design, which is correct on its own and is exactly
        // what let this through.
        let doc = "# Feature: thing\n\n## Summary\nIt does a thing.\nMore summary.\n\n\
             ## Requirements\n- REQ-1: first\n- REQ-2: second\n\n\
             ## Acceptance Criteria\n- AC-1 (REQ-1): checks first\n- AC-2 (REQ-2): checks second\n\n\
             ## Architecture\nTouches `src/design.rs`.\n";

        let report = check(&Document::parse(doc), &schema(), Path::new("x.md"));

        // Precondition: this document really does declare no criteria. If the
        // parser ever starts accepting this form the test is measuring nothing,
        // so assert the premise rather than assuming it.
        assert_eq!(
            Document::parse(doc).declared_ids("AC-").len(),
            0,
            "premise broken: the fixture is supposed to declare zero parseable criteria"
        );

        let coverage: Vec<&Finding> = report
            .findings
            .iter()
            .filter(|f| f.message.contains("coverage") || f.message.contains("referenced by"))
            .collect();

        assert!(
            !coverage.iter().any(|f| f.verdict == Verdict::Pass),
            "coverage must not PASS when nothing was declared to provide it; findings were: {:?}",
            coverage
                .iter()
                .map(|f| (f.verdict, &f.message))
                .collect::<Vec<_>>()
        );
        assert!(
            coverage.iter().any(|f| f.verdict == Verdict::Unchecked),
            "coverage should report UNCHECKED, so the gap is visible rather than absent; \
             findings were: {:?}",
            coverage
                .iter()
                .map(|f| (f.verdict, &f.message))
                .collect::<Vec<_>>()
        );

        // And the summary that lands in the claim must carry it, or the record
        // says the document was validated more thoroughly than it was.
        assert!(
            report.summary().contains("1 unchecked"),
            "summary should count the unchecked finding: {}",
            report.summary()
        );
    }

    /// The other side of the same rule: with criteria actually declared, the
    /// coverage check still runs and still passes. Without this, the fix could
    /// be "never report coverage at all" and the test above would be satisfied.
    #[test]
    fn coverage_still_passes_when_criteria_are_declared() {
        let report = check(&Document::parse(DOC), &schema(), Path::new("x.md"));
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.verdict == Verdict::Pass && f.message.contains("referenced by")),
            "a document with real criteria should still get the coverage PASS"
        );
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.verdict == Verdict::Unchecked),
            "nothing should be unchecked in a well-formed document"
        );
    }

    #[test]
    fn declared_ids_ignore_mere_mentions() {
        let doc = Document::parse(DOC);
        assert_eq!(doc.declared_ids("REQ-").len(), 2);
        // AC-1 mentions REQ-1 but does not declare it.
        assert_eq!(doc.declared_ids("AC-").len(), 2);
    }

    /// day#55: `- REQ-11a: …` parses as `11` followed by `a:`, so the strict
    /// form dropped it — silently. Now it surfaces as a warning naming the
    /// token, and the count reflects that it was not counted.
    #[test]
    fn a_sub_numbered_id_is_reported_not_silently_dropped() {
        let text = DOC.replace(
            "- REQ-2: second",
            "- REQ-2: second\n- REQ-11a: sub-numbered",
        );
        let doc = Document::parse(&text);

        // It is not counted as a declared id (the strict form is unchanged).
        assert_eq!(doc.declared_ids("REQ-").len(), 2);
        // But it is now named as malformed rather than vanishing.
        assert_eq!(doc.malformed_ids("REQ-"), vec!["REQ-11a"]);

        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(
            report.render().contains("REQ-11a"),
            "the dropped id must be named: {}",
            report.render()
        );
        assert!(
            report.render().contains("[WARN]"),
            "a malformed id warns: {}",
            report.render()
        );
    }

    /// The fix must not cry wolf. A valid id, and a loose *mention* of an id in
    /// a bullet (no colon, followed by a space) are both fine — only a
    /// declaration-shaped line with a continued id token is malformed.
    #[test]
    fn valid_ids_and_loose_mentions_are_not_flagged_as_malformed() {
        let doc = Document::parse(DOC);
        assert!(doc.malformed_ids("REQ-").is_empty());
        assert!(doc.malformed_ids("AC-").is_empty());

        // A bullet that mentions REQ-1 without declaring it (space after the
        // number) is not a malformed declaration.
        let text = format!("{DOC}\n- REQ-1 is also relevant here\n");
        let doc = Document::parse(&text);
        assert!(
            doc.malformed_ids("REQ-").is_empty(),
            "a space after the number is a mention, not a malformed id"
        );
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

    /// **day#136 — a subject whose last segment looks like a filename.**
    ///
    /// The test above passes for `telos/composable-process` for the wrong
    /// reason: that slug has no dot, so it fails the extension test rather than
    /// being recognised as a subject. `telos/v1.0` does have one, and `v1.0`
    /// is indistinguishable from `lib.rs` by shape alone — so the exclusion has
    /// to be about the *namespace*, not about what the slug looks like.
    #[test]
    fn a_subject_whose_slug_looks_like_a_filename_is_still_not_a_path() {
        for not_a_path in [
            "telos/v1.0",
            "atom/v1.0",
            "bridge/v0.7-beta2",
            "schema/design-doc",
            "tension/a--b",
        ] {
            assert!(
                !looks_like_path(not_a_path),
                "{not_a_path:?} is a kan subject in one of day's own namespaces, \
                 not a file — warning about it argues an author into dropping \
                 the exact subject a reader would `kan show`"
            );
        }
    }

    /// **The exclusion list is derived, not remembered.**
    ///
    /// `SUBJECT_PREFIXES` re-lists constants that live in five other modules,
    /// which is exactly the shape that drifts: a sixth namespace gets a
    /// constant and nothing points out that this list did not grow. Scanning
    /// `src/` for the declarations means adding one fails the build here
    /// instead of quietly reintroducing day#136 for the new namespace.
    #[test]
    fn every_subject_prefix_is_excluded_from_paths() {
        let mut declared: Vec<(String, String)> = Vec::new();
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![src];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("src/ should be readable") {
                let path = entry.expect("a readable dir entry").path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "rs") {
                    continue;
                }
                let text = std::fs::read_to_string(&path).expect("a readable source file");
                for line in text.lines() {
                    // `pub const FOO_PREFIX: &str = "foo/";`
                    let Some(rest) = line.trim().strip_prefix("pub const ") else {
                        continue;
                    };
                    let Some((name, value)) = rest.split_once(": &str = ") else {
                        continue;
                    };
                    if !name.ends_with("_PREFIX") {
                        continue;
                    }
                    let value = value.trim_end_matches(';').trim_matches('"');
                    if value.ends_with('/') {
                        declared.push((name.to_string(), value.to_string()));
                    }
                }
            }
        }

        // A scan that finds nothing would pass vacuously, which is the defect
        // class this repo names most often. Five namespaces exist today; the
        // floor is deliberately a floor, so adding one does not fail *here*.
        assert!(
            declared.len() >= 5,
            "the scan found only {} `*_PREFIX` constants — it has stopped \
             matching the source it is supposed to read, and would pass \
             whatever the list said",
            declared.len()
        );

        // Asserted through `looks_like_path`, NOT against `SUBJECT_PREFIXES`.
        //
        // Membership is the implementation; "a citation in this namespace is
        // not reported as a missing file" is the property, and it is the one a
        // reader of a design doc actually meets. Going through the function
        // also keeps this test *revertible*: a test naming the constant makes
        // every revert of the fix fail to compile, and `revert-demo.py` then
        // reports DID-NOT-COMPILE — honest, and silent about whether anything
        // asserts the fix.
        for (name, prefix) in declared {
            // A slug shaped like a filename, which is the day#136 case: a
            // namespace excluded only by its slugs happening to lack a dot is
            // not excluded at all.
            let citation = format!("{prefix}v1.0");
            assert!(
                !looks_like_path(&citation),
                "{name} declares the subject namespace {prefix:?}, but \
                 {citation:?} is still read as a file path — so `day design \
                 check` will warn that citing it is a missing file (day#136), \
                 and the cheapest way to silence that is to stop naming the \
                 subject. Add {prefix:?} to `SUBJECT_PREFIXES`."
            );
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

    /// Uses the block shape `commands/design.md` actually emits, including
    /// its closing instruction line. The previous version of this test built
    /// a block *without* that line, so it asserted on text the command never
    /// produces — and the miscount below survived it.
    #[test]
    fn open_question_markers_warn_without_failing() {
        let text = format!(
            "{DOC}\n<!-- OPEN: Q3 -->\n### Q3: still deciding\ncontext and options\n\
             **To resolve**: Edit this section with your decision and remove the \
             `<!-- OPEN -->` marker.\n<!-- /OPEN -->\n"
        );
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert_eq!(report.open_questions, 1);
        assert!(report.is_clean(), "an explicit unknown is not a defect");
    }

    /// Regression for the miscount that `.design/assess-telos.md` exposed:
    /// two genuine open questions reported as four, because each block quotes
    /// the literal `<!-- OPEN -->` marker in the instruction telling an author
    /// to remove it. Invisible until a design doc was recorded with unresolved
    /// blocks still in it, which had never happened before.
    #[test]
    fn a_quoted_open_marker_is_not_counted_as_an_open_question() {
        let block = |n: u8| {
            format!(
                "<!-- OPEN: Q{n} -->\n### Q{n}: a real question\n\
                 **To resolve**: Edit this section and remove the `<!-- OPEN -->` marker.\n\
                 <!-- /OPEN -->\n"
            )
        };
        let text = format!("{DOC}\n{}\n{}\n", block(1), block(2));
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert_eq!(
            report.open_questions, 2,
            "two blocks, each quoting the marker once, must count as two"
        );
    }

    /// The same class, one rule over: a doc explaining the `TODO` convention
    /// inline is not itself unfinished. `prose` already excluded fenced
    /// blocks for this reason; inline spans are the case that intent missed.
    #[test]
    fn a_placeholder_inside_an_inline_code_span_does_not_count() {
        let text = format!("{DOC}\nWe treat `TODO` and `TBD` as placeholder markers.\n");
        let doc = Document::parse(&text);
        let report = check(&doc, &schema(), Path::new(env!("CARGO_MANIFEST_DIR")));
        assert!(report.is_clean(), "{}", report.render());
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
