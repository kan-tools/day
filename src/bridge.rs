//! Bridges: a planned arrangement of atoms aimed at a target telos, and the
//! check for whether that arrangement could actually reach it.
//!
//! Atom composition is checkable because atoms declare typed inputs and
//! outputs. Telos satisfaction is not, because a telos is a weak-equivalence
//! invariant rather than a type. The bridge is that a telos may declare
//! **witnesses** — artifact *types* that would evidence it — which says what
//! kind of evidence counts while leaving open which concrete instance
//! provides it. Weak equivalence survives, and realizability computes by the
//! rule composition already uses.
//!
//! Realizability here is **frame-internal only**. `docs/TELOS.md` defines it
//! as two-fold — frame-internal continuity plus temporal coherence across
//! frames — and the second is vacuous with one actor. It is not checked, and
//! the output says so rather than letting a single-frame result read as a
//! settled global one.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::atoms::{self, Atom, Finding};
use crate::kan_client::KanClient;

/// Subject-name prefix for bridge plans.
pub const BRIDGE_PREFIX: &str = "bridge/";
/// Fence info string marking a bridge plan inside a claim's text.
pub const FENCE_INFO: &str = "day-bridge";
/// Fence info string marking a telos's declared witnesses.
pub const TELOS_FENCE: &str = "day-telos";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error("no bridge `{0}` is declared")]
    NoSuchBridge(String),
    #[error("plan syntax: {0}")]
    Syntax(String),
    #[error("plan references atoms that are not declared: {0}")]
    UndeclaredAtoms(String),
}

/// One entry in a witness list: a single type, or a set of types **any one of
/// which** suffices.
///
/// The list stays a conjunction of entries and an entry may now be a
/// disjunction, so a declaration reads as "all of these, where this one may be
/// satisfied any of these ways". That shape is not a preference: it makes the
/// change **backward compatible by construction**. A bare string parses as
/// [`Group::One`] and serializes back to a bare string, so every `day-telos`
/// block written before this keeps both its meaning and its bytes — which a
/// parallel `witnesses_any` key would not have, leaving two lists to hold in
/// agreement.
///
/// The gap this closes was found by running the interview in
/// `.design/witness-interview.md`: asked what would evidence day's own v1.0
/// bar, the author named three independently valid answers, and day could only
/// say "all of them". `is_clean` and [`check`]'s `uncovered` were both
/// conjunctions, so the render listed witnesses independently while the verdict
/// and-ed them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Group {
    /// Exactly this type. What every witness was before groups existed.
    One(String),
    /// Any one of these types. An empty list is vacuous and is refused at
    /// declare time rather than silently satisfying nothing.
    Any(Vec<String>),
}

impl Group {
    /// The types named in this entry, in declaration order.
    pub fn members(&self) -> Vec<&str> {
        match self {
            Group::One(w) => vec![w.as_str()],
            Group::Any(ws) => ws.iter().map(String::as_str).collect(),
        }
    }

    /// How the entry is named in output. A disjunction renders with `|` so a
    /// reader can tell at a glance which entries have alternatives — the
    /// verdict depends on it, so hiding it would make the report unreadable
    /// against its own conclusion.
    pub fn label(&self) -> String {
        match self {
            Group::One(w) => w.clone(),
            Group::Any(ws) => ws.join(" | "),
        }
    }
}

/// What a telos declares as evidence for itself.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Witnesses {
    /// Artifact types that would evidence this telos. Types, not instances:
    /// many concrete artifacts of a declared type satisfy the telos equally,
    /// which is the weak equivalence being preserved.
    ///
    /// Each entry is a [`Group`]: a type, or several any one of which counts.
    #[serde(default)]
    pub witnesses: Vec<Group>,
    /// Per-witness narrowing of *which instances count* (day#34). The
    /// project's `schema/witness` map still decides which kind of probe
    /// runs; this only tightens its pattern.
    ///
    /// It does not collapse the telos onto an instance, which is the thing
    /// the witness design exists to prevent: `v0.5*` still admits
    /// `v0.5.0-beta.1`, `v0.5.0` and `v0.5.1`, so it names a **narrower
    /// equivalence class, not a point**. `telos/v05-shipped` genuinely is
    /// about a narrower class than "day is published".
    ///
    /// `skip_serializing_if` keeps blocks written before this existed
    /// byte-identical, so the change is additive rather than versioned.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub scope: std::collections::BTreeMap<String, String>,
}

impl crate::atoms::Versioned for Witnesses {
    /// A telos's witness declaration. v1 is every block written before versioning
    /// existed, which an absent `_version` still means.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = TELOS_FENCE;
}

impl Witnesses {
    /// Every type named anywhere in the list, in declaration order, without
    /// repeats.
    ///
    /// Probing is per *type* — a type resolves to one probe and one verdict no
    /// matter how many entries name it — while the verdict is per *group*.
    /// Keeping the two apart is what stops a type appearing twice in the
    /// material section when two groups happen to offer the same alternative.
    pub fn types(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for group in &self.witnesses {
            for member in group.members() {
                if seen.insert(member.to_string()) {
                    out.push(member.to_string());
                }
            }
        }
        out
    }

    pub fn to_claim_text(&self, statement: &str) -> String {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        format!("{statement}\n\n```{TELOS_FENCE}\n{json}\n```\n")
    }
}

/// A node in a plan.
///
/// `Seq` and `All` are **not** the same, which the design pass originally
/// assumed. In `a > b`, `b` runs after `a` and may use what `a` produced. In
/// `a & b` the two are concurrent with no ordering, so `b` may *not* rely on
/// `a`'s outputs — only on what was available before either began. Collapsing
/// them would silently accept plans whose steps depend on work that has not
/// happened yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Node {
    Atom(String),
    Seq(Vec<Node>),
    All(Vec<Node>),
    Any(Vec<Node>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    /// Target telos slug (without the `telos/` prefix).
    pub telos: String,
    /// Artifact types already available where this bridge starts — "here",
    /// in "a path from here to a telos". Without it every source atom would
    /// report its inputs missing, since a source's inputs come from outside
    /// the vocabulary by definition.
    #[serde(default)]
    pub have: Vec<String>,
    pub plan: Node,
}

impl crate::atoms::Versioned for Plan {
    /// A bridge plan. v1 is every block written before versioning
    /// existed, which an absent `_version` still means.
    const SUPPORTED_VERSION: u64 = crate::atoms::IMPLICIT_VERSION;
    const FENCE: &'static str = FENCE_INFO;

    /// day#20: an empty `seq`, `all`, or `any` node is refused.
    ///
    /// The plan *grammar* can never produce one — `collapse` guarantees at least
    /// one child and `parse` rejects empty input — but a `day-bridge` block is
    /// JSON in a claim, and hand-written blocks are supported on purpose. So
    /// `{"any": []}` was reachable, and `walk` handled it by returning the
    /// incoming set unchanged: an empty alternative contributed nothing and
    /// reported nothing.
    ///
    /// Refused rather than reported as a finding, and the distinction matters.
    /// A finding says "day read your plan and it looks wrong"; a refusal says
    /// "this is not a plan." An empty `any` is the second — there is no reading
    /// of "either of these zero routes suffices" that a reachability check can
    /// act on, and quietly treating it as a no-op is how `bridge check` could
    /// report a plan reaches a telos on the strength of a branch that says
    /// nothing.
    fn validate(&self) -> Result<(), String> {
        fn check(node: &Node, path: &str) -> Result<(), String> {
            let (kind, children) = match node {
                Node::Atom(_) => return Ok(()),
                Node::Seq(children) => ("seq", children),
                Node::All(children) => ("all", children),
                Node::Any(children) => ("any", children),
            };
            if children.is_empty() {
                return Err(format!(
                    "`{kind}` at {path} has no children — an empty {kind} node says \
                     nothing about reachability, so it cannot be part of a plan"
                ));
            }
            for (i, child) in children.iter().enumerate() {
                check(child, &format!("{path}/{kind}[{i}]"))?;
            }
            Ok(())
        }
        check(&self.plan, "plan")
    }
}

impl Plan {
    pub fn to_claim_text(&self, slug: &str, note: Option<&str>) -> String {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        let note = note
            .map(|n| format!("{n}\n\n"))
            .unwrap_or_else(|| format!("The {slug} bridge, toward telos/{}.\n\n", self.telos));
        format!("{note}```{FENCE_INFO}\n{json}\n```\n")
    }
}

// ---------------------------------------------------------------- grammar

/// Parses the plan grammar: `a > b` sequence, `a & b` concurrent, `a | b`
/// alternatives, parentheses for grouping. Precedence binds `|` tightest,
/// then `&`, then `>` — so `a > b | c` is `a` followed by a choice between
/// `b` and `c`, which is how the shape reads aloud.
pub fn parse(input: &str) -> Result<Node, Error> {
    let tokens = tokenize(input)?;
    let mut pos = 0;
    let node = parse_seq(&tokens, &mut pos)?;
    if pos != tokens.len() {
        return Err(Error::Syntax(format!(
            "unexpected `{}` at token {pos}",
            tokens[pos]
        )));
    }
    Ok(node)
}

fn tokenize(input: &str) -> Result<Vec<String>, Error> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in input.chars() {
        match c {
            '>' | '&' | '|' | '(' | ')' => {
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
                tokens.push(c.to_string());
            }
            c if c.is_whitespace() => {
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current.clear();
            }
            c => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    if tokens.is_empty() {
        return Err(Error::Syntax("a plan cannot be empty".to_string()));
    }
    Ok(tokens)
}

fn parse_seq(tokens: &[String], pos: &mut usize) -> Result<Node, Error> {
    let mut items = vec![parse_all(tokens, pos)?];
    while tokens.get(*pos).map(String::as_str) == Some(">") {
        *pos += 1;
        items.push(parse_all(tokens, pos)?);
    }
    Ok(collapse(Node::Seq, items))
}

fn parse_all(tokens: &[String], pos: &mut usize) -> Result<Node, Error> {
    let mut items = vec![parse_any(tokens, pos)?];
    while tokens.get(*pos).map(String::as_str) == Some("&") {
        *pos += 1;
        items.push(parse_any(tokens, pos)?);
    }
    Ok(collapse(Node::All, items))
}

fn parse_any(tokens: &[String], pos: &mut usize) -> Result<Node, Error> {
    let mut items = vec![parse_leaf(tokens, pos)?];
    while tokens.get(*pos).map(String::as_str) == Some("|") {
        *pos += 1;
        items.push(parse_leaf(tokens, pos)?);
    }
    Ok(collapse(Node::Any, items))
}

fn parse_leaf(tokens: &[String], pos: &mut usize) -> Result<Node, Error> {
    let token = tokens
        .get(*pos)
        .ok_or_else(|| Error::Syntax("plan ended early".to_string()))?;
    if token == "(" {
        *pos += 1;
        let inner = parse_seq(tokens, pos)?;
        if tokens.get(*pos).map(String::as_str) != Some(")") {
            return Err(Error::Syntax("unclosed `(`".to_string()));
        }
        *pos += 1;
        return Ok(inner);
    }
    if ["(", ")", ">", "&", "|"].contains(&token.as_str()) {
        return Err(Error::Syntax(format!("expected an atom, found `{token}`")));
    }
    *pos += 1;
    Ok(Node::Atom(token.clone()))
}

/// A one-item group is just that item — keeps `a > b` from nesting pointless
/// single-child nodes.
fn collapse(build: fn(Vec<Node>) -> Node, mut items: Vec<Node>) -> Node {
    if items.len() == 1 {
        items.remove(0)
    } else {
        build(items)
    }
}

/// Every atom slug a plan references.
pub fn referenced(node: &Node) -> Vec<String> {
    match node {
        Node::Atom(name) => vec![name.clone()],
        Node::Seq(children) | Node::All(children) | Node::Any(children) => {
            children.iter().flat_map(referenced).collect()
        }
    }
}

// ---------------------------------------------------------- realizability

/// Walks a plan accumulating the artifact types available at each point, and
/// records a finding wherever an atom sits where its declared inputs are not
/// yet available.
///
/// Availability accumulates and is never consumed — the same rule the atom
/// composition check uses, and for the same reason: a design doc is still
/// there when a review runs even though the build between them did not
/// re-emit it.
fn walk(
    node: &Node,
    incoming: &BTreeSet<String>,
    atoms: &[Atom],
    findings: &mut Vec<Finding>,
) -> BTreeSet<String> {
    match node {
        Node::Atom(name) => {
            let Some(atom) = atoms.iter().find(|a| &a.name == name) else {
                // Undeclared atoms are rejected before the walk; reaching
                // here means the caller skipped that check.
                return incoming.clone();
            };
            let missing: Vec<&str> = atom
                .interface
                .inputs
                .iter()
                .filter(|i| !incoming.contains(*i))
                .map(String::as_str)
                .collect();
            if !missing.is_empty() {
                findings.push(Finding {
                    atoms: vec![name.clone()],
                    unreadable: false,
                    version_skew: false,
                    unchecked: false,
                    message: format!(
                        "{}{name} needs [{}] which nothing before it makes available",
                        atoms::ATOM_PREFIX,
                        missing.join(", ")
                    ),
                });
            }
            let mut out = incoming.clone();
            out.extend(atom.interface.outputs.iter().cloned());
            out
        }
        // Ordered: each step sees everything the steps before it produced.
        Node::Seq(children) => children.iter().fold(incoming.clone(), |available, child| {
            walk(child, &available, atoms, findings)
        }),
        // Concurrent: every child happens, but none may rely on another's
        // output, so each is checked against what was available before any
        // of them began.
        Node::All(children) => {
            let mut out = incoming.clone();
            for child in children {
                out.extend(walk(child, incoming, atoms, findings));
            }
            out
        }
        // Alternatives: only what *every* branch produces can be relied on
        // downstream, because a route that might not be taken cannot be
        // assumed to have produced anything. This intersection is what makes
        // an alternative different from concurrent work.
        Node::Any(children) => {
            let mut result: Option<BTreeSet<String>> = None;
            for child in children {
                let produced = walk(child, incoming, atoms, findings);
                result = Some(match result {
                    None => produced,
                    Some(acc) => acc.intersection(&produced).cloned().collect(),
                });
            }
            result.unwrap_or_else(|| incoming.clone())
        }
    }
}

#[derive(Debug)]
pub struct Report {
    pub bridge: String,
    pub telos: String,
    pub findings: Vec<Finding>,
    /// What the target telos declares as evidence for itself, one label per
    /// declared group — so a disjunction reads as `a | b` rather than as two
    /// independent requirements.
    pub witnesses: Vec<String>,
    /// Declared witness groups the plan does not produce, labelled the same
    /// way. A group counts as covered when **any** member is produced. Empty
    /// when the target declares none, in which case `checkable` is false.
    pub uncovered: Vec<String>,
    /// For each covered group offering alternatives, which member the plan
    /// actually produced — `"a | b -> b"`.
    ///
    /// REQ-3: a disjunction that reports "reaches" without saying *how* leaves
    /// the reader unable to tell a plan that satisfies the strong member from
    /// one that scrapes by on the weak one, and those are different plans.
    pub counted: Vec<String>,
    pub checkable: bool,
    pub available: BTreeSet<String>,
}

impl Report {
    pub fn is_reachable(&self) -> bool {
        self.findings.is_empty() && self.uncovered.is_empty()
    }

    pub fn render(&self) -> String {
        let mut out = format!("bridge/{} -> telos/{}\n", self.bridge, self.telos);
        for finding in &self.findings {
            out.push_str(&format!("  ! {}\n", finding.message));
        }

        if !self.checkable {
            out.push_str(&crate::telos::unwitnessed_remedy(
                &self.telos,
                "whether this plan reaches it\n  cannot be checked mechanically -- only that \
                 the steps compose.",
            ));
        } else if self.uncovered.is_empty() {
            out.push_str(&format!(
                "  reaches telos/{}: its declared witness(es) [{}] are produced\n",
                self.telos,
                self.witnesses.join(", ")
            ));
            for counted in &self.counted {
                out.push_str(&format!("    any-of satisfied by: {counted}\n"));
            }
        } else {
            out.push_str(&format!(
                "  does not reach telos/{}: witness(es) [{}] are never produced\n",
                self.telos,
                self.uncovered.join(", ")
            ));
        }

        out.push_str(
            "\n  Realizability is assessed within a single frame. Temporal coherence\n  \
             across frames (docs/TELOS.md) is not checked and is not implied.\n",
        );
        out
    }
}

/// Loads a bridge and its target telos, and checks realizability.
pub fn check(client: &KanClient, slug: &str) -> Result<Report, Error> {
    let subject = format!("{BRIDGE_PREFIX}{slug}");
    // not-per-key: a bridge's plan is the subject's OWN DECLARATION, the
    // fourth row of RQ-7's table. Redeclaring must REPLACE — merging a plan
    // across claims would compose a route nobody planned.
    let plan = atoms::newest_fenced::<Plan>(client, &subject)?
        .map(|(_cid, plan)| plan)
        .ok_or_else(|| Error::NoSuchBridge(slug.to_string()))?;

    let (atom_set, mut findings) = atoms::load(client)?;

    let undeclared: Vec<String> = referenced(&plan.plan)
        .into_iter()
        .filter(|name| !atom_set.iter().any(|a| &a.name == name))
        .collect();
    if !undeclared.is_empty() {
        return Err(Error::UndeclaredAtoms(undeclared.join(", ")));
    }

    let start: BTreeSet<String> = plan.have.iter().cloned().collect();
    let mut walk_findings = Vec::new();
    let available = walk(&plan.plan, &start, &atom_set, &mut walk_findings);
    findings.extend(walk_findings);

    let telos_subject = format!("{}{}", atoms::TELOS_PREFIX, plan.telos);
    // not-per-key: a telos's witnesses are its own declaration, as above.
    let witnesses = atoms::newest_fenced::<Witnesses>(client, &telos_subject)?
        .map(|(_cid, w)| w.witnesses)
        .unwrap_or_default();

    // Per group, not per type. A group is covered when the plan produces
    // **any** member; only a group with no member produced is uncovered.
    let mut uncovered = Vec::new();
    let mut counted = Vec::new();
    for group in &witnesses {
        match group.members().into_iter().find(|m| available.contains(*m)) {
            None => uncovered.push(group.label()),
            // Named only for a real alternative. Reporting "a satisfied by a"
            // for every single-member group would bury the lines that carry
            // information under lines that carry none.
            Some(member) if group.members().len() > 1 => {
                counted.push(format!("{} -> {member}", group.label()));
            }
            Some(_) => {}
        }
    }

    Ok(Report {
        bridge: slug.to_string(),
        telos: plan.telos,
        findings,
        witnesses: witnesses.iter().map(Group::label).collect(),
        uncovered,
        counted,
        checkable: !witnesses.is_empty(),
        available,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atoms::Interface;

    fn atom(name: &str, inputs: &[&str], outputs: &[&str]) -> Atom {
        Atom {
            name: name.to_string(),
            cid: format!("bafy-{name}"),
            interface: Interface {
                inputs: inputs.iter().map(|s| s.to_string()).collect(),
                outputs: outputs.iter().map(|s| s.to_string()).collect(),
                next: vec![],
                revisits: vec![],
                done: vec![],
            },
        }
    }

    fn run(plan: &str, have: &[&str], atoms: &[Atom]) -> (BTreeSet<String>, Vec<Finding>) {
        let node = parse(plan).expect("plan should parse");
        let start: BTreeSet<String> = have.iter().map(|s| s.to_string()).collect();
        let mut findings = Vec::new();
        let available = walk(&node, &start, atoms, &mut findings);
        (available, findings)
    }

    #[test]
    fn parses_the_three_operators_and_grouping() {
        assert_eq!(parse("a").unwrap(), Node::Atom("a".into()));
        assert_eq!(
            parse("a > b").unwrap(),
            Node::Seq(vec![Node::Atom("a".into()), Node::Atom("b".into())])
        );
        assert_eq!(
            parse("a & b").unwrap(),
            Node::All(vec![Node::Atom("a".into()), Node::Atom("b".into())])
        );
        assert_eq!(
            parse("a | b").unwrap(),
            Node::Any(vec![Node::Atom("a".into()), Node::Atom("b".into())])
        );
        // `|` binds tightest, so this is a followed by a choice.
        assert_eq!(
            parse("a > b | c").unwrap(),
            Node::Seq(vec![
                Node::Atom("a".into()),
                Node::Any(vec![Node::Atom("b".into()), Node::Atom("c".into())]),
            ])
        );
        assert_eq!(
            parse("a > (b | c) > d").unwrap(),
            Node::Seq(vec![
                Node::Atom("a".into()),
                Node::Any(vec![Node::Atom("b".into()), Node::Atom("c".into())]),
                Node::Atom("d".into()),
            ])
        );
    }

    #[test]
    fn malformed_plans_are_refused() {
        for bad in ["", "a >", "> a", "(a", "a b )"] {
            assert!(parse(bad).is_err(), "{bad:?} should not parse");
        }
    }

    #[test]
    fn a_plan_round_trips_through_its_own_block() {
        let plan = Plan {
            telos: "shipped".into(),
            have: vec!["intent".into()],
            plan: parse("design > build").unwrap(),
        };
        let text = plan.to_claim_text("v1", None);
        let parsed: Plan = atoms::extract_fenced(&text).unwrap().unwrap();
        assert_eq!(parsed, plan);
    }

    #[test]
    fn a_sequence_threads_availability_forward() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"]),
            atom("build", &["design-doc"], &["code-change"]),
        ];
        let (available, findings) = run("design > build", &["intent"], &atoms);
        assert!(findings.is_empty(), "{findings:?}");
        assert!(available.contains("code-change"));
    }

    #[test]
    fn an_artifact_survives_a_step_that_did_not_re_emit_it() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"]),
            atom("build", &["design-doc"], &["code-change"]),
            atom("review", &["design-doc", "code-change"], &["verdict"]),
        ];
        let (_, findings) = run("design > build > review", &["intent"], &atoms);
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn concurrent_steps_may_not_rely_on_each_other() {
        let atoms = [
            atom("design", &["intent"], &["design-doc"]),
            atom("build", &["design-doc"], &["code-change"]),
        ];
        // Sequenced this is fine; concurrent it is not, because `build` has
        // no ordering guarantee that `design` ran first.
        let (_, seq) = run("design > build", &["intent"], &atoms);
        assert!(seq.is_empty());
        let (_, all) = run("design & build", &["intent"], &atoms);
        assert_eq!(all.len(), 1, "{all:?}");
        assert!(all[0].message.contains("design-doc"));
    }

    #[test]
    fn an_alternative_only_offers_what_every_branch_produces() {
        let atoms = [
            atom("fast", &["intent"], &["code-change"]),
            atom("careful", &["intent"], &["code-change", "proof"]),
            atom("ship", &["code-change"], &["released"]),
            atom("certify", &["proof"], &["certificate"]),
        ];
        // Both branches produce code-change, so ship is satisfied.
        let (_, ok) = run("(fast | careful) > ship", &["intent"], &atoms);
        assert!(ok.is_empty(), "{ok:?}");
        // Only `careful` produces proof, so certify cannot rely on it.
        let (_, bad) = run("(fast | careful) > certify", &["intent"], &atoms);
        assert_eq!(bad.len(), 1, "{bad:?}");
        assert!(bad[0].message.contains("proof"));
    }

    #[test]
    fn a_missing_starting_artifact_is_reported() {
        let atoms = [atom("design", &["intent"], &["design-doc"])];
        let (_, findings) = run("design", &[], &atoms);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("intent"));
    }

    #[test]
    fn referenced_lists_every_atom_in_a_plan() {
        let node = parse("a > (b | c) & d").unwrap();
        let mut names = referenced(&node);
        names.sort();
        assert_eq!(names, vec!["a", "b", "c", "d"]);
    }
}

/// day#20: an empty `seq`/`all`/`any` node is refused at parse, not silently
/// treated as a no-op.
#[cfg(test)]
mod empty_nodes {
    use super::*;
    use crate::atoms::{parse_block, BlockError, Versioned};

    fn plan(json: &str) -> Result<Plan, BlockError> {
        parse_block::<Plan>(json)
    }

    #[test]
    fn an_empty_alternative_set_is_refused() {
        for body in [
            r#"{"telos":"t","have":[],"plan":{"any":[]}}"#,
            r#"{"telos":"t","have":[],"plan":{"seq":[]}}"#,
            r#"{"telos":"t","have":[],"plan":{"all":[]}}"#,
        ] {
            let e = plan(body).expect_err(&format!("should be refused: {body}"));
            assert!(
                matches!(e, BlockError::Invalid { .. }),
                "an empty node is invalid, not unreadable or version-skewed: {e:?}"
            );
            assert!(
                !e.is_version_skew(),
                "an empty node must never tell the reader to upgrade: {e}"
            );
        }
    }

    /// Nested, because the grammar is recursive and a check that only looked at
    /// the root would pass a plan whose *branch* says nothing.
    #[test]
    fn an_empty_node_nested_inside_a_valid_one_is_refused() {
        let e = plan(r#"{"telos":"t","have":[],"plan":{"seq":[{"atom":"a"},{"any":[]}]}}"#)
            .expect_err("a nested empty node should be refused");
        assert!(matches!(e, BlockError::Invalid { .. }), "{e:?}");
        // The path is named, so a deep plan does not require bisecting by hand.
        assert!(e.to_string().contains("seq[1]"), "{e}");
    }

    /// The negative control: every shape the plan grammar can actually produce
    /// still parses. Without this the assertions above would pass if `validate`
    /// rejected everything.
    #[test]
    fn plans_the_grammar_can_produce_are_unaffected() {
        for body in [
            r#"{"telos":"t","have":["intent"],"plan":{"atom":"design"}}"#,
            r#"{"telos":"t","have":[],"plan":{"seq":[{"atom":"a"},{"atom":"b"}]}}"#,
            r#"{"telos":"t","have":[],"plan":{"all":[{"atom":"a"},{"atom":"b"}]}}"#,
            r#"{"telos":"t","have":[],"plan":{"any":[{"atom":"a"},{"seq":[{"atom":"b"},{"atom":"c"}]}]}}"#,
        ] {
            assert!(plan(body).is_ok(), "valid plan was refused: {body}");
        }
    }

    /// And a plan day itself writes round-trips, so the invariant cannot be
    /// tightened past what day produces.
    #[test]
    fn a_plan_day_writes_passes_its_own_validation() {
        let node = parse("design > generative-build > (adversarial-review & assess-telos)")
            .expect("day's own plan syntax should parse");
        let p = Plan {
            telos: "t".into(),
            have: vec!["intent".into()],
            plan: node,
        };
        assert!(p.validate().is_ok(), "{:?}", p.validate());
    }

    /// AC-1 — **an existing declaration keeps both its meaning and its bytes.**
    ///
    /// The nested form was chosen over a parallel `witnesses_any` key precisely
    /// so this holds by construction rather than by care. Asserted on the
    /// serialized text, not just on the parsed value: a round-trip that changed
    /// `["a","b"]` into `[["a"],["b"]]` would still compare equal as a `Witnesses`
    /// while rewriting every telos claim day has ever recorded the next time one
    /// was revised.
    #[test]
    fn a_declaration_written_before_groups_existed_parses_and_reserializes_unchanged() {
        let before = r#"{"witnesses":["published-artifact","assessment"]}"#;
        let parsed: Witnesses = serde_json::from_str(before).expect("v1 block should parse");
        assert_eq!(
            parsed.witnesses,
            vec![
                Group::One("published-artifact".into()),
                Group::One("assessment".into())
            ],
            "a bare string is a one-member group"
        );
        assert_eq!(
            serde_json::to_string(&parsed).unwrap(),
            before,
            "reserializing a v1 block must be byte-identical"
        );
    }

    /// AC-1 — the disjunctive form survives its own round trip.
    #[test]
    fn an_any_of_group_round_trips_as_a_nested_array() {
        let text = r#"{"witnesses":["design-doc",["passing-tests","assessment"]]}"#;
        let parsed: Witnesses = serde_json::from_str(text).expect("group block should parse");
        assert_eq!(
            parsed.witnesses[1],
            Group::Any(vec!["passing-tests".into(), "assessment".into()])
        );
        assert_eq!(serde_json::to_string(&parsed).unwrap(), text);
        // Probing is per type and de-duplicated; the verdict is per group.
        assert_eq!(
            parsed.types(),
            vec!["design-doc", "passing-tests", "assessment"]
        );
    }

    /// AC-1 — a type named in two groups is probed once.
    #[test]
    fn a_type_offered_by_two_groups_is_probed_once() {
        let text = r#"{"witnesses":[["a","shared"],["b","shared"]]}"#;
        let parsed: Witnesses = serde_json::from_str(text).unwrap();
        assert_eq!(parsed.types(), vec!["a", "shared", "b"]);
    }
}
