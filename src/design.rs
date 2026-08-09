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
                continue;
            }
            // day#123: an id *followed* by a qualifier, then the colon —
            // `- [ ] AC-1 (REQ-1): …`. The strict parser drops it, and the
            // rule above does not catch it because the next character is a
            // space. It is the shape day's own coverage warning talks an
            // author into writing, so fifteen criteria became zero with
            // nothing naming the cause.
            //
            // The discriminator is a **qualifier, not prose**. A loose mention
            // (`- REQ-1 is also relevant here`) stays a mention, which is what
            // `valid_ids_and_loose_mentions_are_not_flagged_as_malformed`
            // fixes in place: the line must be a list item, it must carry a
            // colon, and everything between the id and that colon must be free
            // of ordinary lowercase words. `(REQ-1)` and `— REQ-2` qualify;
            // `is also relevant` does not.
            // The strict form itself. Checked here rather than relied on from
            // the branch above, which only ever saw ids that *continued*.
            if after.starts_with(':') {
                continue;
            }
            if is_list_item(line) {
                let Some((gap, _)) = after.split_once(':') else {
                    continue;
                };
                let reads_as_prose = gap.split_whitespace().any(|word| {
                    let word = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
                    word.len() >= 2 && word.chars().all(|c| c.is_ascii_lowercase())
                });
                if !reads_as_prose {
                    out.push(format!("{prefix}{num}{}", gap.trim_end()));
                }
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

    /// Backtick-quoted tokens excluded from [`Self::quoted_paths`] **only**
    /// because they sit in one of day's own subject namespaces (day#136).
    ///
    /// Collected so the caller can report the ones that are ambiguous. `telos/`,
    /// `schema/` and `atom/` are kan namespaces, and they are also ordinary
    /// directory names: a repo with a real `schema/` directory had
    /// `schema/order.v2.json` dropped from the count with nothing said, which is
    /// a report asserting a completeness it did not verify — and day#84, decided
    /// seventy lines away, spends an `[UNCHECKED]` line to avoid exactly that.
    fn subject_shaped_citations(text: &str) -> Vec<String> {
        let mut out = Vec::new();
        for (i, chunk) in text.split('`').enumerate() {
            if i % 2 != 1 {
                continue;
            }
            let candidate = chunk
                .trim_end_matches(['.', ',', ')'])
                .split(':')
                .next()
                .unwrap_or_default();
            // Excluded by the namespace rule and by nothing else — a template
            // or a slash command would fail `looks_like_path` regardless, and
            // reporting those would be the crying-wolf this check avoids.
            if is_path_shaped(candidate) && in_a_subject_namespace(candidate) {
                out.push(candidate.to_string());
            }
        }
        out
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
                // The remedy is split by *which* malformation this is, because
                // day#123 is a case of day's own advice being wrong and
                // repeating "renumber it" here would repeat that.
                // A continued id (`REQ-11a`) genuinely wants renumbering; an
                // id followed by a qualifier (`AC-1 (REQ-1):`) wants the
                // qualifier moved past the colon, which parses and still
                // satisfies the coverage check.
                message: if bad.contains(char::is_whitespace) || bad.contains('(') {
                    format!(
                        "`{bad}` looks like a declaration but its id is not the strict form \
                         `{prefix}<n>:` (e.g. `{prefix}1:`) — it is not counted or \
                         coverage-checked. Move what follows the id to after the colon \
                         (`{prefix}1: …`), which parses and still satisfies the coverage check"
                    )
                } else {
                    format!(
                        "`{bad}` looks like a declaration but its id is not the strict form \
                         `{prefix}<n>:` (e.g. `{prefix}1:`) — it is not counted or \
                         coverage-checked. Renumber it, or the requirement it names is invisible \
                         to validation"
                    )
                },
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
            let subject_citations = Document::subject_shaped_citations(body);
            // day#84: a citation under a declared external root is not this
            // repo's to resolve. Partitioned BEFORE the existence test rather
            // than filtered out of `missing` afterwards, so an external path
            // also stops counting toward "references no existing file" — a doc
            // citing only the other repo would otherwise FAIL for having
            // grounded itself precisely.
            //
            // day#84's follow-up: matched on a **path segment boundary**, not
            // on the raw string. `starts_with("kan")` also matched
            // `kanban/src/store.rs`, so declaring one root silently excluded a
            // sibling directory and turned a missing in-repo file into a PASS
            // at exit 0 — the verdict moving the wrong way, which is worse than
            // the warning day#84 set out to remove.
            let (external, paths): (Vec<String>, Vec<String>) = paths.into_iter().partition(|p| {
                schema.paths_external.iter().any(|root| {
                    let root = root.strip_suffix('/').unwrap_or(root);
                    !root.is_empty()
                        && (p == root
                            || p.strip_prefix(root)
                                .is_some_and(|rest| rest.starts_with('/')))
                })
            });
            let missing: Vec<&String> = paths.iter().filter(|p| !base.join(p).exists()).collect();
            // The rule is grounding, not omniscience: a design must point at
            // code that exists, but an Architecture section naming files it
            // intends to *create* is doing its job — so an unresolved path
            // warns, and only a total absence of real ones fails.
            if paths.is_empty() && !external.is_empty() {
                // day#84, the half its own commit message claimed and did not
                // implement: partitioning empties `paths`, so `paths.is_empty()`
                // failed the document anyway — identical verdict and exit to a
                // day with no `paths_external` at all. A cross-repo contract
                // document citing only the other repo was FAILED for having
                // grounded itself precisely, which is the pressure day#84
                // exists to remove, surviving inside its own fix.
                //
                // `Unchecked` rather than `Pass`: day did not verify these, and
                // saying it did would be the same false completeness in the
                // other direction.
                findings.push(Finding {
                    verdict: Verdict::Unchecked,
                    message: format!(
                        "{} references only external path(s), under declared root(s) {} — \
                         grounded, but in a repo day cannot resolve, so nothing here was \
                         checked",
                        schema.paths_section,
                        schema.paths_external.join(", ")
                    ),
                });
            } else if paths.is_empty() || missing.len() == paths.len() {
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
            // day#136's exclusion, reported where it is ambiguous.
            //
            // `telos/`, `schema/` and `atom/` are kan namespaces AND ordinary
            // directory names, so excluding them unconditionally dropped
            // `schema/order.v2.json` from the count with nothing said, in a repo
            // that has a `schema/` directory. Silence is right for
            // `telos/v1.0` — that is a subject, and warning about it is the
            // pressure day#136 removed — so the discriminator is whether the
            // first segment is a REAL DIRECTORY HERE. If it is, day cannot tell
            // a subject citation from a path and says so instead of choosing.
            let ambiguous: Vec<&String> = subject_citations
                .iter()
                .filter(|c| {
                    c.split_once('/')
                        .is_some_and(|(head, _)| base.join(head).is_dir())
                })
                .collect();
            if !ambiguous.is_empty() {
                findings.push(Finding {
                    verdict: Verdict::Unchecked,
                    message: format!(
                        "{} citation(s) sit in one of day's kan namespaces AND under a real \
                         directory of that name, so day cannot tell a subject from a path and \
                         checked neither: {}",
                        ambiguous.len(),
                        ambiguous
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                });
            }
            // Reported, not silently dropped. An exclusion a reader cannot see
            // is one they cannot correct — and if a root is declared wrongly,
            // this line is the only place that says so.
            if !external.is_empty() {
                findings.push(Finding {
                    // `Unchecked`, which already means exactly this: a check
                    // that could not be run, as distinct from one that ran and
                    // found nothing. Not a new variant — day#105 put that
                    // distinction here for the same reason, and an external
                    // path is the textbook case of it.
                    verdict: Verdict::Unchecked,
                    message: format!(
                        "{} external path(s) not checked, under declared root(s) {}: {}",
                        external.len(),
                        schema.paths_external.join(", "),
                        external.join(", ")
                    ),
                });
            }
        }
    }

    // day#135: a resolved-questions section whose resolutions are written as
    // `### Qn:` headings yields no bullets, so `day design record` appends no
    // `decide` claims — and nothing said so. Five substantive resolutions in
    // `.design/witness-interview.md` and one in v0.11's own design doc reached
    // the log as nothing at all. The decisions and their reasoning are the most
    // valuable output of a design pass, and they are the part that silently did
    // not record.
    //
    // Keyed on the POSITIVE signal — a sub-heading under that section, which is
    // something structurally trying to be a resolution — rather than on the
    // absence of `RQ-`. CLAUDE.md records a classifier keyed on a phrase's
    // absence being suppressed by an unrelated finding; this is the same trap
    // one section over.
    if let Some(body) = doc.section(&schema.resolved_section) {
        let subheadings = body
            .lines()
            .filter(|l| l.trim_start().starts_with('#'))
            .count();
        if doc.bullets(&schema.resolved_section).is_empty() && subheadings > 0 {
            findings.push(Finding {
                verdict: Verdict::Warn,
                message: format!(
                    "{} carries {subheadings} sub-heading(s) and no bullets, so \
                     `day design record` will record no decisions from it — \
                     resolutions are read from `- ` bullets only. Rewrite them as \
                     `- {}1: …` bullets, or the reasoning stays in the document \
                     and never reaches the log",
                    schema.resolved_section, schema.resolution_prefix
                ),
            });
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
/// Whether `line` is a markdown list item — the position in which an id at the
/// head of the line is a declaration rather than a mention (day#123).
fn is_list_item(line: &str) -> bool {
    line.trim_start().starts_with(['-', '*', '+'])
}

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

/// Whether `s` is in one of day's own kan namespaces.
fn in_a_subject_namespace(s: &str) -> bool {
    SUBJECT_PREFIXES.iter().any(|p| s.starts_with(p))
}

/// Whether `s` has the *shape* of a path, ignoring whose namespace it sits in.
///
/// Split from [`looks_like_path`] so day#136's exclusion can be **reported**
/// where it is ambiguous rather than only applied. The first attempt tried to
/// bypass the namespace test by rewriting the string, which changed the answer
/// for an unrelated reason — it removed the slash the shape test requires, so
/// every citation read as not-a-path and the report was silent. Two predicates,
/// each answering one question, is what makes both callers correct.
fn looks_like_path(s: &str) -> bool {
    is_path_shaped(s) && !in_a_subject_namespace(s)
}

fn is_path_shaped(s: &str) -> bool {
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

    /// **day#123 — the id followed by a qualifier, which is what day's own
    /// coverage warning talks an author into writing.**
    ///
    /// `[WARN] REQ-1 is not named by any acceptance criterion … explicit is
    /// better` leads to `- [ ] AC-1 (REQ-1): …`, which the strict parser drops.
    /// Fifteen criteria became zero, `[FAIL] acceptance criteria: 0` was loud
    /// about the wrong thing, and no line was named.
    ///
    /// Asserted on the finding, not on the count: the count was already correct
    /// (it failed at zero, which is how this was noticed), so a test on the
    /// count alone would have passed before the fix.
    #[test]
    fn an_id_followed_by_a_qualifier_is_named_rather_than_dropped() {
        let doc = "# Feature: thing\n\n## Summary\nIt does a thing.\nMore summary.\n\n\
             ## Requirements\n- REQ-1: first\n- REQ-2: second\n\n\
             ## Acceptance Criteria\n- [ ] AC-1 (REQ-1): checks first\n\
             - [ ] AC-2 — REQ-2: checks second\n\n\
             ## Architecture\nTouches `src/design.rs`.\n";
        let parsed = Document::parse(doc);

        // The id format stays strict: this is still not a declaration.
        assert_eq!(
            parsed.declared_ids("AC-").len(),
            0,
            "the strict form is unchanged; the fix is about the silence, not the parser"
        );

        assert_eq!(
            parsed.malformed_ids("AC-"),
            vec!["AC-1 (REQ-1)", "AC-2 — REQ-2"],
            "both a parenthesised and a dashed qualifier are declarations whose \
             id did not parse"
        );

        let render = check(&parsed, &schema(), Path::new("x.md")).render();
        assert!(
            render.contains("AC-1 (REQ-1)"),
            "the dropped line must be named: {render}"
        );
        // day#123 is a case of day's advice being wrong, so the remedy must fit
        // this malformation rather than repeat "renumber it".
        assert!(
            render.contains("Move what follows the id to after the colon"),
            "the remedy must be the one that works here: {render}"
        );
    }

    /// The day#123 fix must not turn prose into findings. A bullet that
    /// *mentions* an id, with or without a colon later in the sentence, is
    /// still a mention — the discriminator is a qualifier versus prose, not
    /// the mere presence of a colon.
    #[test]
    fn a_bullet_mentioning_an_id_in_prose_is_still_not_malformed() {
        for line in [
            "- REQ-1 is also relevant here",
            "- REQ-1 is relevant: see the note above",
            "- REQ-2 and the surrounding text: both discussed below",
        ] {
            let text = format!("{DOC}\n{line}\n");
            let doc = Document::parse(&text);
            assert!(
                doc.malformed_ids("REQ-").is_empty(),
                "{line:?} reads as prose, not as a declaration whose id failed \
                 to parse — flagging it would make the check cry wolf on \
                 ordinary writing"
            );
        }
    }

    /// **day#135 — resolutions written as headings record nothing, silently.**
    ///
    /// `day design record` reads one `decide` per *bullet*. A `### Qn:` heading
    /// yields no bullets, so five substantive resolutions in
    /// `.design/witness-interview.md` and one in v0.11's own design doc reached
    /// the log as nothing, with `design check` reporting `[PASS]` throughout.
    ///
    /// Worse than day#123 in the way that matters: that failure was loud, if
    /// misleadingly so. This one is silent, and it fails `telos/legible-process`
    /// on the repo that defines it.
    #[test]
    fn resolutions_written_as_headings_are_reported_rather_than_recording_nothing() {
        let headings = DOC.replace(
            "## Resolved Questions\n- **Q1 — a**: chose a\n- **Q2 — b**: chose b\n",
            "## Resolved Questions\n\n### Q1 — a\n\nChose a, because of the thing.\n\n\
             ### Q2 — b\n\nChose b.\n",
        );
        let doc = Document::parse(&headings);

        // Premise: this really is the shape that records nothing. Asserted
        // rather than assumed, so the test cannot quietly stop measuring.
        assert!(
            doc.bullets("Resolved Questions").is_empty(),
            "premise broken: the fixture is supposed to yield no bullets"
        );

        let render = check(&doc, &schema(), Path::new("x.md")).render();
        assert!(
            render.contains("2 sub-heading(s) and no bullets"),
            "the drop must be named, and counted from the positive signal: {render}"
        );
    }

    /// The day#135 warning must not fire on the form that works. Keyed on a
    /// positive signal, an ordinary bullet-form section is silent — including
    /// when the bullets carry no `RQ-` ids, which is a different question
    /// (day#119) and not this one's to answer.
    #[test]
    fn a_bullet_form_resolved_section_is_not_reported_as_recording_nothing() {
        for doc in [
            Document::parse(DOC),
            Document::parse(&DOC.replace("- **Q1 — a**: chose a", "- RQ-1: chose a")),
        ] {
            let render = check(&doc, &schema(), Path::new("x.md")).render();
            assert!(
                !render.contains("sub-heading(s) and no bullets"),
                "a section that does yield bullets records fine: {render}"
            );
        }
    }

    /// **day#84 — a path in a sibling repo is not this repo's to resolve.**
    ///
    /// The whole coordination surface between day and kan is documents in one
    /// repo about code in the other, so citing `kan/src/workspace.rs` is the
    /// precise thing to do — and it drew `referenced path does not exist yet`.
    /// The issue's substance is that the warning **changed what got written**:
    /// the path was replaced by a symbol name to silence it, leaving the
    /// document less precise than with no check at all.
    ///
    /// The declared root is asserted three ways, because two of them are the
    /// ways a naive fix goes wrong.
    #[test]
    fn a_path_under_a_declared_external_root_is_unchecked_not_missing() {
        // Declared the way a project declares it — as the JSON of a
        // `day-schema` block — rather than by naming the field in Rust.
        //
        // That is not cosmetic. A test that touches `schema.paths_external`
        // makes every revert of this change fail to COMPILE, and
        // `revert-demo.py` then reports DID-NOT-COMPILE, which says nothing
        // about whether anything asserts the fix. Going through the declaration
        // keeps the demonstration possible, and `Schema` is
        // `deny_unknown_fields`, so a tree without the field refuses this
        // document loudly instead of ignoring the key.
        let mut declared = serde_json::to_value(Schema::starter()).expect("starter serializes");
        declared["paths_external"] = serde_json::json!(["kan/"]);
        let schema: Schema =
            serde_json::from_value(declared).expect("a schema declaring an external root");

        let doc = Document::parse(
            "# Feature: contract\n\n## Summary\nWhat day needs from kan.\n\n\
             ## Requirements\n- REQ-1: a\n- REQ-2: b\n\n## Acceptance Criteria\n\
             - [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
             Touches `src/design.rs`, and the cost lives in `kan/src/workspace.rs`.\n",
        );
        let render = check(&doc, &schema, Path::new(env!("CARGO_MANIFEST_DIR"))).render();

        assert!(
            !render.contains("referenced path does not exist yet: kan/src/workspace.rs"),
            "a declared external root must not be reported as missing — that \
             warning is what argued the path out of the document: {render}"
        );
        assert!(
            render.contains("external path(s) not checked"),
            "and it must be REPORTED as unchecked, not silently dropped: an \
             exclusion a reader cannot see is one they cannot correct: {render}"
        );
        // The in-repo path is still counted, and still the thing that grounds
        // the document. Partitioning before the existence test is what makes
        // this hold: filtering `missing` afterwards would leave the external
        // path in `paths`, so a doc citing ONLY the other repo would fail for
        // "references no existing file" having grounded itself precisely.
        assert!(
            render.contains("1 of 1 referenced path(s) exist"),
            "the external path must leave the in-repo count alone: {render}"
        );
    }

    /// **day#84's own commit message claimed this and the code did not do it.**
    ///
    /// Partitioning empties `paths`, so `paths.is_empty()` failed the document
    /// anyway — a cross-repo contract doc citing only the other repo was FAILED
    /// for having grounded itself precisely, which is the pressure day#84
    /// exists to remove, surviving inside its own fix. The original test never
    /// caught it because its fixture always included an in-repo path.
    #[test]
    fn a_document_citing_only_external_paths_is_unchecked_not_ungrounded() {
        let mut declared = serde_json::to_value(Schema::starter()).expect("starter serializes");
        declared["paths_external"] = serde_json::json!(["kan/"]);
        let schema: Schema = serde_json::from_value(declared).expect("a schema with a root");

        let doc = Document::parse(
            "# Feature: contract\n\n## Summary\nWhat day needs from kan.\n\n\
             ## Requirements\n- REQ-1: a\n- REQ-2: b\n\n## Acceptance Criteria\n\
             - [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
             The cost lives in `kan/src/workspace.rs`.\n",
        );
        let report = check(&doc, &schema, Path::new(env!("CARGO_MANIFEST_DIR")));
        let render = report.render();

        assert!(
            !render.contains("references no existing file"),
            "a document grounded entirely in a declared external repo is \
             grounded; failing it is day#84 reproduced inside day#84's fix: \
             {render}"
        );
        assert!(
            render.contains("references only external path(s)"),
            "and day must say it checked nothing rather than passing silently — \
             `Unchecked` is not `Pass`: {render}"
        );
        assert!(
            report.is_clean(),
            "an unchecked path is not a defect in the document: {render}"
        );
    }

    /// **day#84's root matched a raw string prefix, so `kan` swallowed
    /// `kanban/`.** The verdict moved from WARN to PASS on a genuinely missing
    /// in-repo file — the wrong direction, and worse than the warning day#84
    /// set out to remove.
    #[test]
    fn an_external_root_matches_a_path_segment_not_a_string_prefix() {
        let mut declared = serde_json::to_value(Schema::starter()).expect("starter serializes");
        // Declared WITHOUT a trailing slash, which is how a person writes it
        // and is what made the collision reachable.
        declared["paths_external"] = serde_json::json!(["kan"]);
        let schema: Schema = serde_json::from_value(declared).expect("a schema with a root");

        let doc = Document::parse(
            "# Feature: c\n\n## Summary\ns\n\n## Requirements\n- REQ-1: a\n- REQ-2: b\n\n\
             ## Acceptance Criteria\n- [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
             Touches `src/design.rs`, `kan/src/workspace.rs`, and `kanban/src/store.rs`.\n",
        );
        let render = check(&doc, &schema, Path::new(env!("CARGO_MANIFEST_DIR"))).render();

        // **Asserted on WHICH line it appears on, not on whether it appears.**
        // A first version used `contains("kanban/src/store.rs")` and did not
        // fail under revert: with the string-prefix bug the file still appears
        // in the render — in the EXTERNAL list — so `contains` cannot tell the
        // two outcomes apart. `revert-demo.py` reported it as not failing,
        // which is the harness doing the job the assertion did not.
        assert!(
            render.contains("referenced path does not exist yet: kanban/src/store.rs"),
            "`kanban/` is a different directory from the declared root `kan`, so \
             its missing file must be reported as MISSING — with the raw-prefix \
             match it was excluded as external instead, moving a WARN to a PASS: \
             {render}"
        );
        let external_line = render
            .lines()
            .find(|l| l.contains("external path(s) not checked"))
            .unwrap_or_default();
        assert!(
            !external_line.contains("kanban/src/store.rs"),
            "and it must not be listed as external: {external_line}"
        );
        // The genuinely-external one still is, so this narrows the match rather
        // than removing the feature.
        assert!(
            external_line.contains("kan/src/workspace.rs"),
            "the real external path is still unchecked: {external_line}"
        );
    }

    /// **day#136's exclusion was silent, always-on, and nobody opted into it.**
    ///
    /// `schema/`, `telos/` and `atom/` are kan namespaces AND ordinary
    /// directory names. In a repo with a real `schema/` directory, citations
    /// under it vanished from the count with nothing said — a report asserting
    /// a completeness it did not verify, while day#84 seventy lines away spends
    /// a line to avoid exactly that.
    ///
    /// The discriminator is whether the first segment is a real directory
    /// **here**, so `telos/v1.0` in this repo stays silent — warning about that
    /// is the pressure day#136 correctly removed.
    #[test]
    fn a_subject_citation_under_a_real_directory_of_that_name_is_reported() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("schema")).unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/design.rs"), "// fixture\n").unwrap();

        let doc = Document::parse(
            "# Feature: c\n\n## Summary\ns\n\n## Requirements\n- REQ-1: a\n- REQ-2: b\n\n\
             ## Acceptance Criteria\n- [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
             Touches `src/design.rs`, the validator `schema/order.v2.json`, \
             and the telos `telos/v1.0`.\n",
        );
        let render = check(&doc, &Schema::starter(), dir.path()).render();

        assert!(
            render.contains("schema/order.v2.json"),
            "a citation under a REAL `schema/` directory is ambiguous and must \
             be reported rather than silently dropped from the count: {render}"
        );
        assert!(
            !render.contains("telos/v1.0"),
            "and `telos/` is not a directory here, so that citation is \
             unambiguously a subject and stays silent — reporting it would be \
             the noise day#136 removed: {render}"
        );
    }

    /// The negative control. With no root declared — every project today, since
    /// the starter ships none — the behaviour is exactly what it was, warning
    /// included. Without this the test above passes against a check that has
    /// stopped looking at paths at all.
    #[test]
    fn an_undeclared_external_root_still_warns_exactly_as_before() {
        let doc = Document::parse(
            "# Feature: contract\n\n## Summary\nWhat day needs from kan.\n\n\
             ## Requirements\n- REQ-1: a\n- REQ-2: b\n\n## Acceptance Criteria\n\
             - [ ] AC-1: a\n- [ ] AC-2: b\n\n## Architecture\n\
             Touches `src/design.rs`, and the cost lives in `kan/src/workspace.rs`.\n",
        );
        let render = check(
            &doc,
            &Schema::starter(),
            Path::new(env!("CARGO_MANIFEST_DIR")),
        )
        .render();

        assert!(
            render.contains("referenced path does not exist yet: kan/src/workspace.rs"),
            "with nothing declared, day#84's warning is unchanged — the fix is \
             opt-in per project, not a quiet narrowing for everyone: {render}"
        );
        assert!(
            !render.contains("external path(s) not checked"),
            "and nothing is reported as external: {render}"
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
