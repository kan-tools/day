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
use crate::probe::{self, Authorization, ClaimLog, ClaimShape, Probe, Verdict};
use crate::schema::SCHEMA_PREFIX;

/// Subject slug day looks for: `schema/witness`.
pub const WITNESS_SLUG: &str = "witness";
/// Fence info string marking a witness-probe map inside a claim's text.
pub const FENCE_INFO: &str = "day-witness";

/// **The one place day says what to do about a telos declaring no witnesses.**
///
/// Two call sites rendered this independently — `assess telos` and `bridge
/// check` — with wording that had already drifted apart, which is how a third
/// arrives unnoticed. `CLAUDE.md` records the rule this follows: a guarantee
/// about what day reports belongs in the mechanism, never in a caller, because
/// a check added at a call site looks complete when the author's test drives
/// the call site they were thinking about.
///
/// **What changed with the collapse is the advice, not just its home.** Both
/// sites used to print `day telos declare <slug> "..." --witness <type>`, which
/// hands the reading agent a command that invites it to guess a witness alone.
/// day#86 records why that is worse than the state it purports to fix: a
/// trivially satisfiable witness reports the telos met forever, which is the
/// failure `telos/v05-shipped` taught, and a bad witness is worse than none. So
/// the remedy names the interview instead — a pass that asks a human what would
/// evidence this, because that is not inferable from a slug.
///
/// It points at the slash command rather than at `atom/witness-interview`,
/// deliberately. The command ships with the plugin and therefore exists
/// wherever day is installed; the atom is a kan claim that a fresh repo has
/// not declared. Pointing at the atom would be a remedy that does not remedy —
/// day#108's finding, which `src/status.rs` already acted on once.
///
/// The phrase "declares no witnesses" is written literally here rather than
/// pulled from a shared constant, and `tests/plugin.rs`'s scan writes it
/// literally too. That looks like duplication and is the opposite: a scan
/// asserting a fact about *source text* must own the text it matches, or a
/// rename carries both sides along together and the scan quietly checks
/// something else. It also keeps the scan compiling when this function does
/// not, which is what lets a reversion demonstrate that it fires.
pub fn unwitnessed_remedy(slug: &str, consequence: &str) -> String {
    format!(
        "  {}{slug} declares no witnesses, so {consequence}\n  \
         What would evidence it is a question for a person, not a guess -- and a\n  \
         witness that cannot fail is worse than none (day#86). Establish one:\n    \
         /witness-interview {slug}\n",
        atoms::TELOS_PREFIX,
    )
}

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
    // Every variant that concerns one subject **names that subject**, so a
    // caller which already knows it does not have to guess whether to prefix.
    // Guessing is what printed `telos/bad: telos/bad: …`.
    #[error("{}{}: no such telos is declared", atoms::TELOS_PREFIX, .0)]
    NoSuchTelos(String),
    #[error("no atom `{0}` is declared")]
    NoSuchAtom(String),
    #[error(
        "no witness schema is declared for this project (expected a `{FENCE_INFO}` block on \
         subject `{SCHEMA_PREFIX}{WITNESS_SLUG}`).\n\nWhat would evidence a witness type is \
         this project's choice — day ships no built-in mapping, because what counts as a \
         published artifact differs by project. Record a starter with:\n\n{starter}"
    )]
    NotDeclared { starter: String },
}

/// What would evidence each witness type, declared per project.
///
/// Deserialization is **tolerant of probe kinds this day does not know**: an
/// unreadable entry is set aside in [`Self::unsupported`] and the rest of the
/// map still loads. day requires exactly this of kan — `kan_client`'s tests
/// assert that a field day has never heard of cannot break it — and did not
/// offer it in return until a `claim` probe recorded on this repo made the
/// installed v0.6 binary fail the *whole* schema, and with it every hook and
/// status line in the session. A newer probe kind now degrades a single
/// witness to "no probe", which is an honest state day already renders,
/// rather than taking the project's process surface down with it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WitnessSchema {
    /// The **material** witness for each type: what would show the artifact
    /// exists in the world. Every declaration written before the paired form
    /// lands here unchanged, which is what makes the widening below invisible
    /// to the eleven call sites that read this map.
    pub probes: BTreeMap<String, Probe>,
    /// The **record** witness, for types that declare one: what would show the
    /// artifact was *written down*.
    ///
    /// Empty for every single-probe declaration, so a project that never opts
    /// in is never affected. A type present in both maps can be asked a
    /// question neither map alone can answer — the thing exists *and* the log
    /// says so, or it exists and the log does not — which is day#103
    /// generalised out of the `release` subject it was found on.
    pub records: BTreeMap<String, Probe>,
    /// Witness types whose declared probe this version could not read, with
    /// the reason. Reported, never silently dropped: a reader must not think
    /// a witness is unprobed when it is merely unreadable *here*.
    ///
    /// Not serialized — see the [`Serialize`] impl for why.
    pub unsupported: BTreeMap<String, String>,
}

/// The paired form: `{"material": {...}, "record": {...}}`.
///
/// `deny_unknown_fields` on purpose. A pair is opt-in and both halves are
/// required, so a typo'd key must be an *unsupported witness* rather than a
/// silent degradation to material-only — the latter would reintroduce exactly
/// the blindness the pair exists to remove, and would do it quietly.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PairedWitness {
    material: Probe,
    record: Probe,
}

/// Emits the same shapes [`WitnessSchema`]'s deserializer accepts: a bare probe
/// for a material-only witness, `{"material": …, "record": …}` for a pair.
///
/// Hand-written because this type used to be `#[serde(transparent)]` over
/// `probes`, and leaving it that way once `records` existed would have printed a
/// starter that silently dropped every record half — `starter_command()` is what
/// a project copies into its own `schema/witness` claim, so a lossy render here
/// would hand people a declaration that quietly does less than the one day
/// suggested. Round-tripping is asserted rather than assumed.
///
/// `unsupported` is deliberately not emitted: it is what *this build* could not
/// read, not something the project declared, and writing it back would launder a
/// reader's limitation into the project's record.
impl Serialize for WitnessSchema {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.probes.len()))?;
        for (witness, material) in &self.probes {
            match self.records.get(witness) {
                Some(record) => {
                    let mut pair = BTreeMap::new();
                    pair.insert("material", material);
                    pair.insert("record", record);
                    map.serialize_entry(witness, &pair)?;
                }
                None => map.serialize_entry(witness, material)?,
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for WitnessSchema {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Parsed one entry at a time through `Value`, so a kind this day
        // cannot read costs that witness and nothing else. Parsing the map
        // straight into `Probe` is what made a single unknown kind fatal.
        let raw = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut probes = BTreeMap::new();
        let mut records = BTreeMap::new();
        let mut unsupported = BTreeMap::new();
        for (witness, value) in raw {
            // The discriminator is the *author's evident intent*, not whether
            // the pair happens to parse: an entry naming either half is a pair,
            // and then both halves must read or the witness is unsupported.
            //
            // Keying on "does it parse as a pair" instead would mean a record
            // half with a typo fell through to the single-probe branch, failed
            // there too, and landed in `unsupported` with a message about the
            // wrong shape — or worse, if only `material` were misspelled,
            // parsed as a single probe and silently lost the record half. A
            // witness that quietly stops asking half its question is the
            // failure mode this whole milestone is about.
            let declares_pair = value.get("material").is_some() || value.get("record").is_some();

            if declares_pair {
                match serde_json::from_value::<PairedWitness>(value) {
                    Ok(pair) => {
                        probes.insert(witness.clone(), pair.material);
                        records.insert(witness, pair.record);
                    }
                    Err(e) => {
                        unsupported
                            .insert(witness, format!("declares a material/record pair but {e}"));
                    }
                }
                continue;
            }

            match serde_json::from_value::<Probe>(value) {
                Ok(probe) => {
                    probes.insert(witness, probe);
                }
                Err(e) => {
                    unsupported.insert(witness, e.to_string());
                }
            }
        }
        Ok(Self {
            probes,
            records,
            unsupported,
        })
    }
}

impl crate::atoms::Versioned for WitnessSchema {
    /// The witness map. v1 is every block written before versioning existed,
    /// which an absent `_version` still means.
    ///
    /// **No `deny_unknown_fields` here, and that is not an omission.** This
    /// block is `transparent` over a map from witness type to probe, so every
    /// key is *data* — an unrecognised key is a witness type day has never
    /// heard of, which is a project's business and not an error. The strictness
    /// lives one level down, on the probe each key maps to, and the per-entry
    /// deserializer above already reports a probe this build cannot read
    /// without losing the rest of the map.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;
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
        // The two claim-shaped witnesses. Neither is a file or a tag: a
        // verdict is what `day review record` appends, an assessment is what
        // `kan result` records — so before the `claim` probe kind existed,
        // both were unprobeable and left position permanently ambiguous
        // (day#60).
        //
        // Both are narrowed by more than `kind`, and day#70 found out why by
        // running them: `{kind: Decision, contains: …}` matched the very
        // decision that *defined* the marker, and a bare `{kind: Result}`
        // matched release notes and session handoffs as readily as an atom
        // assessment. Each is narrowed on the dimension that actually
        // separates the real thing — a verdict by the prefix `record::review`
        // anchors at the start of its text, an assessment by the `atom/*`
        // namespace it is recorded on.
        probes.insert(
            "verdict".to_string(),
            Probe::Claim(ClaimShape {
                kind: "Decision".to_string(),
                contains: None,
                starts_with: Some("adversarial review of".to_string()),
                subject: None,
                block: None,
            }),
        );
        probes.insert(
            "assessment".to_string(),
            Probe::Claim(ClaimShape {
                kind: "Result".to_string(),
                contains: None,
                starts_with: None,
                // Not `starts_with`: an assessment's text is free prose, so
                // there is no prefix to anchor. And no subject scope on
                // `verdict` for the mirror reason — a verdict lands on
                // whatever subject was reviewed.
                subject: Some(format!("{}*", atoms::ATOM_PREFIX)),
                block: None,
            }),
        );
        // NO PAIRED WITNESS IS SUGGESTED HERE, and the reason is the finding
        // that writing this milestone produced.
        //
        // The obvious starter pair is the one day#103 was about:
        // `published-artifact`, material `{tag: "v*"}`, record a `release`
        // claim. It does not work, and it fails *silently*, which is the one
        // way it must not fail.
        //
        // A `tag` probe under a cycle boundary means "created strictly after
        // the boundary" (`position::resolve_collecting`), and the boundary IS
        // the newest tag. So in the cycle a release opens, `published-artifact`
        // is Absent by construction — day's own v0.7.0-beta.1 release claim
        // records observing exactly this, cutting a tag and watching the
        // witness go absent. The comparison "material present and record
        // absent" therefore never fires for the case that motivated it, and a
        // project adopting the suggestion would get a permanently quiet check
        // that looks like a clean bill of health.
        //
        // What the release case actually needs is *correspondence* — does a
        // record exist that refers to THIS material instance — which is what
        // `docs::reconcile_boundary` does with `text.contains(tag)` and what a
        // `ClaimShape` cannot yet express. That is a real extension and it is
        // not this milestone's; filed rather than half-built.
        //
        // The pair mechanism below is still correct and still shipped: for any
        // artifact type whose material witness is not boundary-degenerate — a
        // `path` probe, for instance — "it exists this cycle and the log does
        // not mention it" is exactly right. It is only the tag/boundary
        // interaction that defeats it, and day should not suggest the one shape
        // it cannot answer.
        Self {
            probes,
            records: BTreeMap::new(),
            unsupported: BTreeMap::new(),
        }
    }

    pub fn starter_command() -> String {
        let json = serde_json::to_string_pretty(&Self::starter()).unwrap_or_default();
        format!(
            "  kan observe \"$(cat <<'EOF'\nWitness probes for this project.\n\n\
             ```{FENCE_INFO}\n{json}\n```\nEOF\n)\" --subject {SCHEMA_PREFIX}{WITNESS_SLUG}"
        )
    }

    /// The verdict for a witness whose declared probe this version cannot
    /// read.
    ///
    /// [`Verdict::Error`] rather than `Unsatisfied`, and the distinction is
    /// the whole point: the evidence was not *checked*, not found *absent*.
    /// `Error` already means "the probe could not be evaluated at all" and
    /// does not count against a telos, so an unreadable probe cannot fail a
    /// build — it says go look.
    pub fn unreadable(&self, witness: &str) -> Option<Verdict> {
        self.unsupported.get(witness).map(|reason| {
            Verdict::Error(format!(
                "`{witness}` declares a probe kind this version of day cannot read \
                 ({reason}) — upgrade day, or this witness goes unchecked here"
            ))
        })
    }

    pub fn load(client: &KanClient) -> Result<Self, Error> {
        let subject = format!("{SCHEMA_PREFIX}{WITNESS_SLUG}");
        newest_fenced::<Self>(client, &subject)?
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
        // A scope replaces *the* pattern argument, and a claim probe has two
        // fields rather than one — so there is no single thing to replace.
        // Overwriting `contains` would be the wrong guess in the dangerous
        // direction: a schema's marker is usually narrower than a telos's
        // scope, so honouring it could *widen* which claims count. Reported
        // rather than silently dropped, per the same rule as the command
        // arm: a reader must never believe a narrowing took effect that did
        // not.
        Probe::Claim(_) => (
            probe.clone(),
            Some(format!(
                "scope `{scope}` ignored: a claim probe is narrowed by its own `contains` \
                 marker, and replacing that from a telos could widen which claims count \
                 rather than narrow it"
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
    /// The declared witness structure. Findings are per *type*, because that
    /// is what a probe answers; the verdict is per *group*, because that is
    /// what the telos declared. [`Report::is_clean`] needs both.
    pub groups: Vec<bridge::Group>,
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
    ///
    /// **Folded per group, not per finding.** A group counts against the telos
    /// only when *every* member failed: one satisfied alternative is precisely
    /// what a disjunction declares to be enough. Before groups existed this was
    /// "any finding failed", which is the same thing when every group has one
    /// member — so the reading of an old declaration is unchanged.
    ///
    /// The member rule itself is untouched. Only `Unsatisfied` counts, so a
    /// not-run or timed-out member cannot fail its group either.
    pub fn is_clean(&self) -> bool {
        let failed = |witness: &str| {
            self.findings.iter().any(|f| {
                f.witness == witness && f.verdict.as_ref().is_some_and(Verdict::is_failure)
            })
        };
        !self
            .groups
            .iter()
            .any(|group| group.members().into_iter().all(failed))
    }

    pub fn render(&self) -> String {
        let mut out = format!("Telos assessment — {}{}\n", atoms::TELOS_PREFIX, self.telos);
        if let Some(statement) = &self.statement {
            out.push_str(&format!("  {statement}\n"));
        }
        out.push('\n');

        if !self.checkable {
            out.push_str(&unwitnessed_remedy(
                &self.telos,
                "whether work landed inside its\n  equivalence class cannot be checked \
                 mechanically.",
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
            // Any-of groups are stated after the per-type verdicts, because
            // without them the verdicts do not add up: a reader seeing one
            // `[MISSING]` above a clean exit would otherwise be looking at what
            // reads as a contradiction.
            for group in &self.groups {
                if group.members().len() > 1 {
                    out.push_str(&format!(
                        "  any of [{}] satisfies this telos; they are alternatives, not \
                         a checklist\n",
                        group.label()
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
        let plan = newest_fenced::<bridge::Plan>(client, &subject)?;
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

    let declared = newest_fenced::<Witnesses>(client, &subject)?
        .map(|(_cid, w)| w)
        .unwrap_or_default();
    // Two different lists, deliberately. `groups` carries the declared
    // structure and decides the verdict; `types` is the flattened, deduplicated
    // set that actually gets probed, because a type resolves to one probe and
    // one verdict however many groups offer it as an alternative.
    let groups = declared.witnesses.clone();
    let witnesses = declared.types();

    // The same fold `hooks::render_teloi` uses, and for the same reason.
    //
    // This read "the newest narrative claim" and it produced the defect on the
    // verb that CAUSES it: `assess telos` prints `kan result telos/<slug>` as
    // the way to record an assessment, and the next run rendered that
    // assessment in the telos-statement slot. The reader and the writer were
    // the same command, disagreeing with itself.
    //
    // Fixing `render_teloi` alone fixed the surface where it was observed and
    // left the one that instructs the write. Prefer the declaration; fall back
    // to any claim that is not an assessment; never let a `Result` stand in for
    // the statement.
    //
    // fallback: telos-without-a-declaration
    let statement = crate::fold::declaration(&claims);
    // Loaded only when there is something to check, so a telos with no
    // witnesses reports that rather than a missing-schema error it cannot
    // act on.
    let schema = if witnesses.is_empty() {
        WitnessSchema::default()
    } else {
        WitnessSchema::load(client)?
    };

    // One read of the log, shared by every claim probe. Lazy, so a telos
    // whose witnesses are all files and tags never touches kan for this.
    let log = ClaimLog::new(client);

    let mut findings = Vec::new();
    for witness in &witnesses {
        let mut scope_note = None;
        let verdict = schema
            .probes
            .get(witness)
            .map(|probe| {
                let (effective, note) = effective_probe(probe, declared.scope.get(witness));
                scope_note = note;
                // `probe::evaluate`, never `position::resolve`: an assessment is
                // cumulative. A release, a review, or an assessment from any
                // cycle is real evidence that work landed inside this telos's
                // equivalence class, and scoping that to the current cycle would
                // make last cycle's shipped telos start reporting as unmet.
                probe::evaluate(&effective, git, &log, auth)
            })
            // A witness whose probe this version cannot read is reported as
            // unchecked, not as unprobed — the project declared something,
            // day just could not read it.
            .or_else(|| schema.unreadable(witness));
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
        checkable: !groups.is_empty(),
        groups,
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

/// One atom's `done` criteria, evaluated against the project's probes.
///
/// The gateable counterpart to `day status`: `status` displays where you are,
/// this answers "is this atom finished" with an exit code, so CI and a human
/// can gate on it. Enforcement at the artifact level, never the action level.
pub struct AtomReport {
    pub atom: String,
    pub findings: Vec<WitnessFinding>,
    /// True when the atom declares no `done` criteria — reported, not treated
    /// as met.
    pub no_criteria: bool,
    /// CID of the atom's newest claim, so the printed `kan result` cites the
    /// declaration it assesses. `None` only if the atom somehow has no claim.
    pub newest_cid: Option<String>,
}

impl AtomReport {
    /// A declared criterion that ran and was not satisfied fails the check.
    /// Not-run, timed-out, no-probe, and no-criteria are all "unknown, go
    /// look", not failure — the same rule `assess telos` uses, for the same
    /// reason: exiting non-zero on absence of evidence would make every
    /// default run look broken.
    pub fn is_clean(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|f| f.verdict.as_ref().is_some_and(Verdict::is_failure))
    }

    pub fn render(&self) -> String {
        let mut out = format!("Atom assessment — {}{}\n", atoms::ATOM_PREFIX, self.atom);
        if self.no_criteria {
            out.push_str(
                "  no `done` criteria declared, so completion cannot be checked mechanically.\n                   Declare them: day atom declare <slug> --done <witness-type>\n",
            );
            return out;
        }
        out.push_str("Completion criteria:\n");
        for finding in &self.findings {
            match &finding.verdict {
                Some(verdict) => out.push_str(&format!(
                    "  [{}] {}: {}\n",
                    verdict.label(),
                    finding.witness,
                    verdict.detail()
                )),
                None => out.push_str(&format!(
                    "  [NO PROBE] {}: no probe declared for this witness type\n",
                    finding.witness
                )),
            }
        }

        // Close the loop the same way `assess telos` does. Recording an atom
        // assessment is what gives `day status` a transition baseline (REQ-10):
        // day reads that claim and never writes it, so the runnable command is
        // how the human, not the tool, records having checked. Without this
        // prompt atom assessments never get recorded and transitions stay dark
        // on any real log.
        let cites = self.newest_cid.as_deref().unwrap_or("<cid>");
        out.push_str(&format!(
            "\nThis assessment was performed, not recorded — those are separate acts.\n\
             To record it (and give `day status` a transition baseline):\n  \
             kan result {}{} \"<what you concluded, citing the evidence above>\" \\\n    \
             --cites {cites}\n",
            atoms::ATOM_PREFIX,
            self.atom
        ));
        out
    }
}

/// Assesses whether an atom's `done` criteria are met. Runs command probes
/// only under `Authorization::Run`, matching `assess telos`.
pub fn assess_atom(
    client: &KanClient,
    git: &Git,
    slug: &str,
    auth: Authorization,
) -> Result<AtomReport, Error> {
    let (atoms, _) = atoms::load(client)?;
    let atom = atoms
        .iter()
        .find(|a| a.name == slug)
        .ok_or_else(|| Error::NoSuchAtom(slug.to_string()))?;

    let done = &atom.interface.done;
    if done.is_empty() {
        return Ok(AtomReport {
            atom: slug.to_string(),
            findings: vec![],
            no_criteria: true,
            newest_cid: Some(atom.cid.clone()),
        });
    }

    let schema = WitnessSchema::load(client)?;
    let log = ClaimLog::new(client);
    let findings = done
        .iter()
        .map(|witness| WitnessFinding {
            witness: witness.clone(),
            verdict: schema
                .probes
                .get(witness)
                // Cumulative, like `assess`: "were these criteria ever met",
                // not "were they met this cycle".
                .map(|probe| probe::evaluate(probe, git, &log, auth))
                .or_else(|| schema.unreadable(witness)),
            asserted_by: None,
            scope_note: None,
        })
        .collect();

    Ok(AtomReport {
        atom: slug.to_string(),
        findings,
        no_criteria: false,
        newest_cid: Some(atom.cid.clone()),
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
        let parsed: WitnessSchema = atoms::extract_fenced(&command)
            .expect("the starter command should carry a block")
            .expect("it should parse");
        assert_eq!(parsed, WitnessSchema::starter());
    }

    /// The paired arm of the hand-written `Serialize`, which the starter no
    /// longer exercises.
    ///
    /// `starter()` used to suggest a paired `published-artifact`, so
    /// `the_starter_round_trips_through_its_own_block` covered this. Removing
    /// that suggestion (day#107 — the pair cannot answer for a boundary-degenerate
    /// tag witness) left the arm unreachable in production AND untested, while
    /// its doc comment still claimed "round-tripping is asserted rather than
    /// assumed". A comment claiming a property needs a test named after it.
    ///
    /// Kept rather than deleted because the arm goes live the moment a project
    /// declares a pair or day#107 lets the starter suggest one again — and a
    /// missing arm would silently flatten every record half on output, which is
    /// the exact failure the hand-written impl exists to prevent.
    #[test]
    fn a_paired_witness_round_trips_through_serialize() {
        let mut probes = BTreeMap::new();
        probes.insert("published-artifact".to_string(), Probe::Tag("v*".into()));
        probes.insert("design-doc".to_string(), Probe::Path(".design/*.md".into()));

        let mut records = BTreeMap::new();
        records.insert(
            "published-artifact".to_string(),
            Probe::Claim(ClaimShape {
                kind: "Result".to_string(),
                contains: None,
                starts_with: None,
                subject: Some("release".to_string()),
                block: None,
            }),
        );

        let schema = WitnessSchema {
            probes,
            records,
            unsupported: BTreeMap::new(),
        };

        let json = serde_json::to_string(&schema).expect("a schema should serialize");
        assert!(
            json.contains("\"material\"") && json.contains("\"record\""),
            "a paired witness must serialize in the paired form, not flattened to \
             its material half: {json}"
        );

        let back: WitnessSchema = serde_json::from_str(&json).expect("and parse back");
        assert_eq!(back.probes, schema.probes);
        assert_eq!(
            back.records, schema.records,
            "the record half must survive the round trip — losing it here is the \
             silent flattening this impl was hand-written to prevent"
        );

        // The unpaired entry must NOT gain a wrapper on the way out.
        assert!(
            json.contains("\"design-doc\":{\"path\""),
            "a material-only witness must still serialize as a bare probe: {json}"
        );
    }

    /// REQ-3 — every declaration written before the paired form keeps its exact
    /// meaning. This is the shape on *this repo's own* `schema/witness` claim.
    ///
    /// The assertion is not merely that it parses: it is that `records` stays
    /// empty, because an empty `records` is what makes the material/record
    /// comparison a no-op for projects that never opted in. If a single probe
    /// ever started populating both maps, every such project would begin
    /// getting done-but-unrecorded findings it never asked for.
    #[test]
    fn a_single_probe_declaration_is_material_only_and_gains_no_record() {
        let json = r#"{
            "published-artifact": {"tag": "v*"},
            "design-doc": {"path": ".design/*.md"},
            "assessment": {"claim": {"kind": "Result", "subject": "atom/*"}}
        }"#;
        let schema: WitnessSchema = serde_json::from_str(json).unwrap();

        assert_eq!(
            schema.probes.get("published-artifact"),
            Some(&Probe::Tag("v*".into()))
        );
        assert!(
            schema.records.is_empty(),
            "a single-probe declaration must declare no record witness, or every \
             pre-existing project starts getting findings it did not ask for: {:?}",
            schema.records
        );
        assert!(schema.unsupported.is_empty(), "{:?}", schema.unsupported);
    }

    /// REQ-2 — the paired form splits into the two maps.
    #[test]
    fn a_paired_witness_declares_both_halves() {
        let json = r#"{
            "published-artifact": {
                "material": {"tag": "v*"},
                "record": {"claim": {"kind": "Result", "subject": "release"}}
            }
        }"#;
        let schema: WitnessSchema = serde_json::from_str(json).unwrap();

        assert_eq!(
            schema.probes.get("published-artifact"),
            Some(&Probe::Tag("v*".into())),
            "the material half must land where every existing reader looks"
        );
        assert!(
            matches!(
                schema.records.get("published-artifact"),
                Some(Probe::Claim(_))
            ),
            "the record half must be kept: {:?}",
            schema.records
        );
        assert!(schema.unsupported.is_empty(), "{:?}", schema.unsupported);
    }

    /// REQ-3's honest-reads clause, and the reason the parser keys on the
    /// author's evident intent rather than on whether the pair happens to parse.
    ///
    /// Each of these names one half of a pair and gets the other wrong. None may
    /// quietly become a material-only witness: that would leave the project
    /// believing it declared a comparison day is not making, which is a silent
    /// downgrade of exactly the kind `telos/honest-reads` exists to refuse.
    #[test]
    fn a_broken_pair_is_unsupported_rather_than_silently_material_only() {
        for (name, json) in [
            (
                "record half misspelled",
                r#"{"published-artifact": {"material": {"tag": "v*"}, "recrod": {"claim": {"kind": "Result"}}}}"#,
            ),
            (
                "material half misspelled",
                r#"{"published-artifact": {"materal": {"tag": "v*"}, "record": {"claim": {"kind": "Result"}}}}"#,
            ),
            (
                "record half missing entirely",
                r#"{"published-artifact": {"material": {"tag": "v*"}}}"#,
            ),
            (
                "record half is not a probe day knows",
                r#"{"published-artifact": {"material": {"tag": "v*"}, "record": {"telepathy": "x"}}}"#,
            ),
        ] {
            let schema: WitnessSchema = serde_json::from_str(json).unwrap();
            assert!(
                schema.unsupported.contains_key("published-artifact"),
                "{name}: should be reported as unsupported, got probes={:?} records={:?}",
                schema.probes,
                schema.records
            );
            assert!(
                !schema.probes.contains_key("published-artifact"),
                "{name}: must NOT degrade to a material-only witness — the project \
                 asked for a comparison and would never be told it is not happening"
            );
        }
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
            groups: vec![crate::bridge::Group::One("w".into())],
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
            groups: vec![crate::bridge::Group::One("published-artifact".into())],
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
            groups: vec![],
            prompts: vec![],
            record_command: String::new(),
        };
        let rendered = report.render();
        assert!(rendered.contains("declares no witnesses"), "{rendered}");
        // The remedy is the interview, not `--witness <type>`. Asserted as the
        // positive string rather than as "does not contain the old one": a
        // negative assertion passes when the whole block is missing.
        assert!(rendered.contains("/witness-interview t"), "{rendered}");
    }

    /// AC-2 — **one satisfied alternative makes the group clean**, and that is
    /// the whole point of declaring one.
    ///
    /// Driven through `is_clean` rather than through the render, because the
    /// conjunction lived in the verdict and not in the output: the render
    /// already listed witnesses independently while this and-ed them, which is
    /// exactly why the gap was invisible until someone tried to declare a
    /// disjunction.
    #[test]
    fn one_satisfied_member_clears_an_any_of_group() {
        let finding = |witness: &str, verdict: Verdict| WitnessFinding {
            witness: witness.into(),
            verdict: Some(verdict),
            asserted_by: None,
            scope_note: None,
        };
        let report = |findings: Vec<WitnessFinding>| Report {
            telos: "t".into(),
            statement: None,
            findings,
            checkable: true,
            groups: vec![crate::bridge::Group::Any(vec!["a".into(), "b".into()])],
            prompts: vec![],
            record_command: String::new(),
        };

        assert!(
            report(vec![
                finding("a", Verdict::Unsatisfied("no".into())),
                finding("b", Verdict::Satisfied("yes".into())),
            ])
            .is_clean(),
            "one satisfied member is what the group declared to be enough"
        );
        assert!(
            !report(vec![
                finding("a", Verdict::Unsatisfied("no".into())),
                finding("b", Verdict::Unsatisfied("no".into())),
            ])
            .is_clean(),
            "a group fails only when every member does -- and it must still fail then"
        );
        // The member rule is unchanged: not-run is absence of evidence, so it
        // cannot fail its member and therefore cannot fail the group either.
        assert!(
            report(vec![
                finding("a", Verdict::Unsatisfied("no".into())),
                finding("b", Verdict::NotRun("needs --run".into())),
            ])
            .is_clean(),
            "an unrun member leaves the group unknown, not failed"
        );
    }

    /// AC-2 — a plain list still and-s, so an existing telos reads the same.
    #[test]
    fn separate_groups_remain_a_conjunction() {
        let report = Report {
            telos: "t".into(),
            statement: None,
            findings: vec![
                WitnessFinding {
                    witness: "a".into(),
                    verdict: Some(Verdict::Satisfied("yes".into())),
                    asserted_by: None,
                    scope_note: None,
                },
                WitnessFinding {
                    witness: "b".into(),
                    verdict: Some(Verdict::Unsatisfied("no".into())),
                    asserted_by: None,
                    scope_note: None,
                },
            ],
            checkable: true,
            groups: vec![
                crate::bridge::Group::One("a".into()),
                crate::bridge::Group::One("b".into()),
            ],
            prompts: vec![],
            record_command: String::new(),
        };
        assert!(
            !report.is_clean(),
            "two separate witnesses are both required, exactly as before groups existed"
        );
    }
}
