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

/// What a telos declares as evidence for itself.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Witnesses {
    /// Artifact types that would evidence this telos. Types, not instances:
    /// many concrete artifacts of a declared type satisfy the telos equally,
    /// which is the weak equivalence being preserved.
    #[serde(default)]
    pub witnesses: Vec<String>,
}

impl Witnesses {
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
    /// What the target telos declares as evidence for itself.
    pub witnesses: Vec<String>,
    /// Declared witnesses the plan does not produce. Empty when the target
    /// declares none, in which case `checkable` is false.
    pub uncovered: Vec<String>,
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
            out.push_str(&format!(
                "  telos/{} declares no witnesses, so whether this plan reaches it\n  \
                 cannot be checked mechanically — only that the steps compose.\n  \
                 Declare what would evidence it: day telos declare {} \"...\" --witness <type>\n",
                self.telos, self.telos
            ));
        } else if self.uncovered.is_empty() {
            out.push_str(&format!(
                "  reaches telos/{}: its declared witness(es) [{}] are produced\n",
                self.telos,
                self.witnesses.join(", ")
            ));
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
    let plan = atoms::newest_fenced::<Plan>(client, &subject, FENCE_INFO)?
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
    let witnesses = atoms::newest_fenced::<Witnesses>(client, &telos_subject, TELOS_FENCE)?
        .map(|(_cid, w)| w.witnesses)
        .unwrap_or_default();

    let uncovered: Vec<String> = witnesses
        .iter()
        .filter(|w| !available.contains(*w))
        .cloned()
        .collect();

    Ok(Report {
        bridge: slug.to_string(),
        telos: plan.telos,
        findings,
        witnesses: witnesses.clone(),
        uncovered,
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
        let parsed: Plan = atoms::extract_fenced(&text, FENCE_INFO).unwrap().unwrap();
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
