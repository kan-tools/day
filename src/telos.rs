//! `day assess telos` — did work land inside a telos's equivalence class?
//!
//! `day bridge check` asks whether a plan *could* reach a telos: it walks the
//! arrangement and sees whether the target's declared witnesses are produced
//! somewhere along it. That is a statement about a plan. This asks whether
//! they **were** produced, and by what evidence.
//!
//! The gap between those is that a witness is a *type*. `telos/v03-shipped`
//! declares `published-artifact`, and many concrete artifacts of that type
//! satisfy it equally — that is the weak equivalence the whole model exists
//! to preserve. Assessing means binding the type to an instance without
//! collapsing the telos onto it, which is what a **probe** does: it names
//! what would count, and the assessment reports which concrete thing
//! answered.
//!
//! Two tiers with different powers, the split `src/docs.rs` established. The
//! **material** tier runs probes and alone decides the exit code. The
//! **record** tier reports what the log says and only ever prompts — because
//! a claim asserting a telos was met is a narrative about the work, and
//! `docs/CONVENTIONS.md` already holds that such a claim is worth much less
//! than one citing an artifact. Keeping them visibly separate is the point:
//! the report must never let prose read as evidence.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::atoms::{self, newest_fenced, prose_only};
use crate::bridge::{self, Witnesses};
use crate::git::Git;
use crate::kan_client::KanClient;
use crate::probe::{self, Authorization, Probe, Verdict};
use crate::schema::SCHEMA_PREFIX;

/// Subject slug day looks for: `schema/witness`.
pub const WITNESS_SLUG: &str = "witness";
/// Fence info string marking a witness-probe map inside a claim's text.
pub const FENCE_INFO: &str = "day-witness";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error(transparent)]
    Bridge(#[from] bridge::Error),
    #[error(transparent)]
    Tension(#[from] crate::tension::Error),
    #[error("no telos `{0}` is declared")]
    NoSuchTelos(String),
    #[error(
        "no witness schema is declared for this project (expected a `{FENCE_INFO}` block on \
         subject `{SCHEMA_PREFIX}{WITNESS_SLUG}`).\n\nWhat would evidence a witness type is \
         this project's choice — day ships no built-in mapping, because what counts as a \
         published artifact differs by project. Record a starter with:\n\n{starter}"
    )]
    NotDeclared { starter: String },
}

/// What would evidence each witness type, declared per project.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WitnessSchema {
    pub probes: BTreeMap<String, Probe>,
}

impl WitnessSchema {
    /// What day suggests when a project has none. Offered, never applied —
    /// the same contract `Schema::starter` and `DocsSchema::starter` hold.
    pub fn starter() -> Self {
        let mut probes = BTreeMap::new();
        probes.insert(
            "published-artifact".to_string(),
            Probe::Tag("v*".to_string()),
        );
        probes.insert(
            "design-doc".to_string(),
            Probe::Path(".design/*.md".to_string()),
        );
        probes.insert("code-change".to_string(), Probe::Path("src/*".to_string()));
        Self { probes }
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::starter()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nWitness probes for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{WITNESS_SLUG}"
        )
    }

    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{WITNESS_SLUG}");
        newest_fenced::<Self>(client, &subject, FENCE_INFO)?
            .map(|(_cid, schema)| schema)
            .ok_or_else(|| Error::NotDeclared {
                starter: Self::starter_command(),
            })
    }
}

/// Applies a telos's scope to the project's probe for a witness (day#34).
///
/// The scope narrows **which instances count**; the project map keeps
/// deciding **which kind of probe runs**. So a scope replaces the pattern
/// argument of a `path` or `tag` probe and leaves its kind alone.
///
/// **A `command` probe is returned unchanged**, with a note. Honouring a
/// scope there would mean a telos claim determining what day executes, and
/// commands must originate only from `schema/witness` — one subject to
/// review rather than every `telos/*` in the log. That is the widening the
/// day#34 decision rejected, and this is where it becomes code.
///
/// Lives here rather than in `src/probe.rs` deliberately: `probe.rs` is the
/// module the no-shell guardrail greps, and scoping is a policy decision
/// about which instances count, not a change to how a probe executes.
fn effective_probe(probe: &Probe, scope: Option<&String>) -> (Probe, Option<String>) {
    let Some(scope) = scope else {
        return (probe.clone(), None);
    };
    match probe {
        Probe::Path(_) => (Probe::Path(scope.clone()), None),
        Probe::Tag(_) => (Probe::Tag(scope.clone()), None),
        Probe::Command(_) => (
            probe.clone(),
            Some(format!(
                "scope `{scope}` ignored: a command probe is not narrowed by a telos, \
                 because that would let a telos claim decide what runs"
            )),
        ),
    }
}

/// One witness type and what became of it.
#[derive(Debug)]
pub struct WitnessFinding {
    pub witness: String,
    pub verdict: Option<Verdict>,
    /// A claim on the telos subject that mentions this witness type.
    /// Reported **separately from** the verdict and never counted as
    /// material: it is the project's own account of its work, which is
    /// exactly what an assessment is supposed to be checkable against
    /// rather than founded on.
    pub asserted_by: Option<String>,
    /// Why a declared scope was not applied, when it was not. Reported so a
    /// reader is never left believing a narrowing took effect that did not.
    pub scope_note: Option<String>,
}

#[derive(Debug)]
pub struct Report {
    pub telos: String,
    pub statement: Option<String>,
    pub findings: Vec<WitnessFinding>,
    /// Absent entirely — the telos declares no witnesses, so nothing about
    /// it is mechanically checkable.
    pub checkable: bool,
    /// Record-tier observations. Prompts, never failures.
    pub prompts: Vec<String>,
    /// The command a reader can run to record this assessment.
    pub record_command: String,
}

impl Report {
    /// Only the material tier decides this. A not-run probe is absence of
    /// evidence rather than evidence of absence, and a timeout leaves the
    /// evidence unknown — treating either as failure would make the default
    /// invocation look broken and push people toward `--run` reflexively.
    pub fn is_clean(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|f| f.verdict.as_ref().is_some_and(Verdict::is_failure))
    }

    pub fn render(&self) -> String {
        let mut out = format!("Telos assessment — {}{}\n", atoms::TELOS_PREFIX, self.telos);
        if let Some(statement) = &self.statement {
            out.push_str(&format!("  {statement}\n"));
        }
        out.push('\n');

        if !self.checkable {
            out.push_str(&format!(
                "  {}{} declares no witnesses, so whether work landed inside its\n  \
                 equivalence class cannot be checked mechanically. Declare what would\n  \
                 evidence it:\n    day telos declare {} \"...\" --witness <type>\n",
                atoms::TELOS_PREFIX,
                self.telos,
                self.telos
            ));
        } else {
            out.push_str("Material evidence:\n");
            for finding in &self.findings {
                match &finding.verdict {
                    Some(verdict) => out.push_str(&format!(
                        "  [{}] {}: {}\n",
                        verdict.label(),
                        finding.witness,
                        verdict.detail()
                    )),
                    None => out.push_str(&format!(
                        "  [NO PROBE] {}: no probe is declared for this witness type, so \
                         nothing\n             material was checked\n",
                        finding.witness
                    )),
                }
                // Rendered under the witness but visibly not part of the
                // verdict, so a reader cannot mistake the log agreeing with
                // itself for evidence.
                if let Some(note) = &finding.scope_note {
                    out.push_str(&format!("             {note}\n"));
                }
                if let Some(claim) = &finding.asserted_by {
                    out.push_str(&format!(
                        "             asserted in prose by {claim} — not material evidence\n"
                    ));
                }
            }
        }

        if !self.prompts.is_empty() {
            out.push_str("\nWhat the record says:\n");
            for prompt in &self.prompts {
                out.push_str(&format!("  {prompt}\n"));
            }
        }

        out.push_str(&format!(
            "\nThis assessment was performed, not recorded — those are separate acts.\n\
             To record it:\n{}\n",
            self.record_command
        ));
        out.push_str(
            "\n  Assessed within a single frame. Cross-frame reconciliation\n  \
             (docs/TELOS.md) is not checked and is not implied.\n",
        );
        out
    }
}

/// The record tier: what the log says, as prompts a reader weighs.
fn record_tier(
    client: &KanClient,
    slug: &str,
    claims: &[crate::kan_client::Claim],
    prompts: &mut Vec<String>,
) -> Result<(), Error> {
    let assessments = claims.iter().filter(|c| c.kind == "Result").count();
    if assessments == 0 {
        prompts
            .push("no assessment (`kan result`) has been recorded on this telos yet".to_string());
    } else {
        prompts.push(format!(
            "{assessments} assessment(s) already recorded on this telos"
        ));
    }

    // REQ-4: the reason lives on a tension subject now, so day reads it back
    // here. Moving information off the telos must not make it unfindable.
    for line in crate::tension::render_for(slug, &crate::tension::for_telos(client, slug)?) {
        prompts.push(format!(
            "{line}\n    work satisfying this telos may have traded against that one"
        ));
    }

    // A bridge aimed here already computed whether its plan *could* reach the
    // telos. That is a different question from this one, and saying so keeps
    // the two from being read as one.
    for subject in client.subjects()? {
        let Some(bridge_slug) = subject.strip_prefix(bridge::BRIDGE_PREFIX) else {
            continue;
        };
        let plan = newest_fenced::<bridge::Plan>(client, &subject, bridge::FENCE_INFO)?;
        if plan.is_some_and(|(_cid, p)| p.telos == slug) {
            let reachable = bridge::check(client, bridge_slug)
                .map(|r| r.is_reachable())
                .unwrap_or(false);
            prompts.push(format!(
                "{subject} targets this telos and its plan {} reach it — but a plan that \
                 could is not work that did",
                if reachable { "could" } else { "could not" }
            ));
        }
    }
    Ok(())
}

/// Assesses one telos. Reads kan, reads git, runs declared probes when
/// authorized, and prints. **Appends nothing** — recording an assessment is
/// a separate act, and conflating "I checked" with "I recorded that I
/// checked" would let the tool manufacture its own evidence.
pub fn assess(
    client: &KanClient,
    git: &Git,
    slug: &str,
    auth: Authorization,
) -> Result<Report, Error> {
    let subject = format!("{}{slug}", atoms::TELOS_PREFIX);
    let claims = client.show(&subject)?;
    if claims.is_empty() {
        return Err(Error::NoSuchTelos(slug.to_string()));
    }

    let declared = newest_fenced::<Witnesses>(client, &subject, bridge::TELOS_FENCE)?
        .map(|(_cid, w)| w)
        .unwrap_or_default();
    let witnesses = declared.witnesses.clone();

    // The newest narrative claim is the closest thing to "what this telos
    // currently says". It is not always the declaration — see day#32 — so it
    // is shown as context rather than labelled as the statement.
    let statement = claims
        .iter()
        .rev()
        .find_map(|c| c.text.as_deref().map(prose_only))
        .filter(|s| !s.is_empty());

    // Loaded only when there is something to check, so a telos with no
    // witnesses reports that rather than a missing-schema error it cannot
    // act on.
    let schema = if witnesses.is_empty() {
        WitnessSchema::default()
    } else {
        WitnessSchema::load(client)?
    };

    let mut findings = Vec::new();
    for witness in &witnesses {
        let mut scope_note = None;
        let verdict = schema.probes.get(witness).map(|probe| {
            let (effective, note) = effective_probe(probe, declared.scope.get(witness));
            scope_note = note;
            probe::evaluate(&effective, git, auth)
        });
        // Searched against prose only. The `day-telos` block naming this
        // witness is the *declaration* that it would count as evidence, not
        // a claim that it was produced — matching it would make every
        // witnessed telos report itself as prose-asserted.
        let asserted_by = claims
            .iter()
            .rev()
            .find(|c| {
                c.text
                    .as_deref()
                    .is_some_and(|t| prose_only(t).contains(witness))
            })
            .map(|c| c.cid.clone());
        findings.push(WitnessFinding {
            witness: witness.clone(),
            verdict,
            asserted_by,
            scope_note,
        });
    }

    let mut prompts = Vec::new();
    record_tier(client, slug, &claims, &mut prompts)?;

    let newest = claims.last().map(|c| c.cid.as_str()).unwrap_or("<cid>");
    Ok(Report {
        telos: slug.to_string(),
        statement,
        findings,
        checkable: !witnesses.is_empty(),
        prompts,
        // `kan result` takes its subject POSITIONALLY, unlike observe/plan/
        // decide. Getting this wrong is what day#27 and kan#78 are about, and
        // tests/kan_conformance.rs runs this exact shape against a real kan.
        record_command: format!(
            "  kan result {subject} \"<what you concluded, citing the evidence above>\" \\\n    \
             --cites {newest}"
        ),
    })
}

/// Every declared telos, for `--all`.
pub fn all_slugs(client: &KanClient) -> Result<Vec<String>, Error> {
    let mut slugs: Vec<String> = client
        .subjects()?
        .into_iter()
        .filter_map(|s| s.strip_prefix(atoms::TELOS_PREFIX).map(str::to_string))
        .collect();
    slugs.sort();
    Ok(slugs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_starter_round_trips_through_its_own_block() {
        let command = WitnessSchema::starter_command();
        let parsed: WitnessSchema = atoms::extract_fenced(&command, FENCE_INFO)
            .expect("the starter command should carry a block")
            .expect("it should parse");
        assert_eq!(parsed, WitnessSchema::starter());
    }

    /// The shape a project actually writes: a bare object mapping witness
    /// type to probe, with no wrapper key.
    #[test]
    fn a_witness_schema_parses_the_declared_shape() {
        let json =
            r#"{"published-artifact":{"tag":"v*"},"passing-tests":{"command":"cargo test"}}"#;
        let schema: WitnessSchema = serde_json::from_str(json).unwrap();
        assert_eq!(
            schema.probes.get("published-artifact"),
            Some(&Probe::Tag("v*".into()))
        );
        assert_eq!(
            schema.probes.get("passing-tests"),
            Some(&Probe::Command("cargo test".into()))
        );
    }

    /// Not-run and timed-out probes must not fail the assessment, or every
    /// invocation without `--run` would look like the telos was missed.
    #[test]
    fn only_a_probe_that_ran_and_found_nothing_fails_the_assessment() {
        let report = |verdict: Option<Verdict>| Report {
            telos: "t".into(),
            statement: None,
            findings: vec![WitnessFinding {
                witness: "w".into(),
                verdict,
                asserted_by: None,
                scope_note: None,
            }],
            checkable: true,
            prompts: vec![],
            record_command: String::new(),
        };
        assert!(report(Some(Verdict::Satisfied("x".into()))).is_clean());
        assert!(report(Some(Verdict::NotRun("x".into()))).is_clean());
        assert!(report(Some(Verdict::TimedOut("x".into()))).is_clean());
        assert!(report(Some(Verdict::Error("x".into()))).is_clean());
        assert!(
            report(None).is_clean(),
            "no probe means nothing was checked"
        );
        assert!(!report(Some(Verdict::Unsatisfied("x".into()))).is_clean());
    }

    /// REQ-10: a claim mentioning a witness is reported, but never counted.
    #[test]
    fn prose_assertion_is_rendered_separately_from_the_verdict() {
        let report = Report {
            telos: "t".into(),
            statement: None,
            findings: vec![WitnessFinding {
                witness: "published-artifact".into(),
                verdict: Some(Verdict::Unsatisfied("no tag matches".into())),
                asserted_by: Some("bafyclaim".into()),
                scope_note: None,
            }],
            checkable: true,
            prompts: vec![],
            record_command: String::new(),
        };
        let rendered = report.render();
        assert!(rendered.contains("[MISSING]"), "{rendered}");
        assert!(rendered.contains("not material evidence"), "{rendered}");
        assert!(
            !report.is_clean(),
            "a prose assertion must not rescue a failing probe"
        );
    }

    /// Found by running the tool on day's own log: a telos declaring
    /// `published-artifact` in its `day-telos` block reported that block as
    /// prose asserting the witness had been produced, and printed the raw
    /// JSON back as the telos statement.
    #[test]
    fn a_fenced_declaration_is_neither_the_statement_nor_a_prose_assertion() {
        let text = "day v0.5 is published.\n\n```day-telos\n                    {\"witnesses\":[\"published-artifact\"]}\n```\n";
        let prose = prose_only(text);
        assert_eq!(prose, "day v0.5 is published.");
        assert!(
            !prose.contains("published-artifact"),
            "a witness declaration must not read as an assertion that it was produced"
        );
    }

    #[test]
    fn a_telos_without_witnesses_says_so_rather_than_passing_silently() {
        let report = Report {
            telos: "t".into(),
            statement: None,
            findings: vec![],
            checkable: false,
            prompts: vec![],
            record_command: String::new(),
        };
        let rendered = report.render();
        assert!(rendered.contains("declares no witnesses"), "{rendered}");
        assert!(rendered.contains("--witness"), "{rendered}");
    }
}
