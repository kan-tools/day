//! Atoms: the composable process units of the telos-driven model
//! (`docs/TELOS.md`), and the composition check `day doctor` runs over them.
//!
//! Per `docs/CONVENTIONS.md`, an atom lives entirely in kan as claims on an
//! `atom/<slug>` subject, and its interface is a fenced `day-atom` JSON
//! block inside the claim text. The vocabulary is **per-atom additive**:
//! each atom is its own subject, "the current vocabulary" is just the live
//! non-retracted fold, and the newest interface-bearing claim on a subject
//! is that atom's current interface. day never mutates or deletes anything
//! to revise an atom — a newer claim supersedes an older one, kan's own
//! append/fold/retract pattern rather than a second versioning mechanic.

use serde::{Deserialize, Serialize};

use crate::kan_client::{self, KanClient};

/// Subject-name prefix for atom declarations.
pub const ATOM_PREFIX: &str = "atom/";
/// Subject-name prefix for telos declarations.
pub const TELOS_PREFIX: &str = "telos/";
/// Fence info string marking an interface block inside a claim's text.
pub const FENCE_INFO: &str = "day-atom";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] kan_client::Error),
    #[error("{subject}: fenced block on claim {cid} is not valid JSON: {source}")]
    Malformed {
        subject: String,
        cid: String,
        #[source]
        source: serde_json::Error,
    },
}

/// An atom's declared interface. `inputs`/`outputs` are free-form type
/// names — day checks that they *match*, deliberately not what they mean;
/// the vocabulary of type names is the project's to choose and evolve.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interface {
    #[serde(rename = "in", default)]
    pub inputs: Vec<String>,
    #[serde(rename = "out", default)]
    pub outputs: Vec<String>,
    /// Atoms this one declares it composes into, by slug.
    #[serde(default)]
    pub next: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom {
    /// Slug, i.e. the subject name minus the `atom/` prefix.
    pub name: String,
    /// CID of the claim carrying this interface — the evidence pointer, so
    /// a finding can always be traced back to the claim that caused it.
    pub cid: String,
    pub interface: Interface,
}

impl Atom {
    pub fn subject(&self) -> String {
        format!("{ATOM_PREFIX}{}", self.name)
    }
}

impl Interface {
    /// Renders the claim text `day atom declare` appends — the exact shape
    /// [`extract_interface`] reads back. Write and read share the
    /// `Interface` type and this one function, so a hand-written block and a
    /// day-written block cannot mean different things.
    pub fn to_claim_text(&self, slug: &str, note: Option<&str>) -> String {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        let note = note
            .map(|n| format!("{n}\n\n"))
            .unwrap_or_else(|| format!("The {slug} atom.\n\n"));
        format!("{note}```{FENCE_INFO}\n{json}\n```\n")
    }
}

/// Something wrong with the live atom set. Advisory: day reports these and
/// exits non-zero, it never rewrites the log to "fix" them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Every atom implicated, so callers can render both sides of a
    /// mismatch rather than just the source.
    pub atoms: Vec<String>,
    pub message: String,
}

/// Reads every `atom/<slug>` subject's live claims and takes the newest
/// interface-bearing claim as that atom's current interface.
pub fn load(client: &KanClient) -> Result<(Vec<Atom>, Vec<Finding>), Error> {
    let mut atoms = Vec::new();
    let mut findings = Vec::new();

    let mut subjects: Vec<String> = client
        .subjects()?
        .into_iter()
        .filter(|s| s.starts_with(ATOM_PREFIX))
        .collect();
    subjects.sort();

    for subject in subjects {
        let name = subject[ATOM_PREFIX.len()..].to_string();
        let claims = client.show(&subject)?;
        // Latest interface-bearing claim wins: `kan show` prints a
        // subject's live claims oldest-first, so the last match is current.
        let latest = claims.iter().rev().find_map(|c| {
            c.text
                .as_deref()
                .and_then(extract_interface)
                .map(|r| (c, r))
        });

        match latest {
            Some((claim, Ok(interface))) => atoms.push(Atom {
                name,
                cid: claim.cid.clone(),
                interface,
            }),
            Some((claim, Err(e))) => findings.push(Finding {
                atoms: vec![name.clone()],
                message: format!(
                    "{subject}: `{FENCE_INFO}` block is not valid interface JSON ({e}) — claim {}",
                    claim.cid
                ),
            }),
            None => findings.push(Finding {
                atoms: vec![name.clone()],
                message: format!(
                    "{subject}: no `{FENCE_INFO}` interface block on any live claim, so it can't be composition-checked"
                ),
            }),
        }
    }

    Ok((atoms, findings))
}

/// A claim's prose with fenced blocks removed.
///
/// Both uses found by dogfooding: rendering a telos statement printed the
/// whole `day-telos` block back at the reader, and — worse — the witness
/// scan matched every witness type against the block that *declares* it, so
/// every telos reported its own declaration as a prose assertion that the
/// witness had been satisfied. A declaration is not an assertion of success.
pub fn prose_only(text: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.trim().to_string()
}

/// Pulls the first fenced block with the given info string out of a claim's
/// text and deserializes it. Returns `None` when the claim carries no such
/// block at all (most claims don't), `Some(Err(..))` when it carries one
/// that doesn't parse — the difference matters, since the second is a real
/// finding and the first isn't.
///
/// Shared by atoms (`day-atom`) and design-doc schemas (`day-schema`): one
/// embedded-block convention, not two, so a project learns the pattern once.
pub fn extract_fenced<T: serde::de::DeserializeOwned>(
    text: &str,
    fence: &str,
) -> Option<Result<T, serde_json::Error>> {
    let open = format!("```{fence}");
    let start = text.find(&open)? + open.len();
    let rest = &text[start..];
    let end = rest.find("```")?;
    Some(serde_json::from_str(rest[..end].trim()))
}

/// [`extract_fenced`] specialized to an atom's `day-atom` interface block.
pub fn extract_interface(text: &str) -> Option<Result<Interface, serde_json::Error>> {
    extract_fenced(text, FENCE_INFO)
}

/// Reads the newest claim on `subject` carrying a `fence` block, returning
/// the parsed value with the CID of the claim it came from. The
/// newest-wins rule every kan-backed vocabulary in day uses.
pub fn newest_fenced<T: serde::de::DeserializeOwned>(
    client: &KanClient,
    subject: &str,
    fence: &str,
) -> Result<Option<(String, T)>, Error> {
    let claims = client.show(subject)?;
    for claim in claims.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        match extract_fenced::<T>(text, fence) {
            Some(Ok(value)) => return Ok(Some((claim.cid.clone(), value))),
            // A malformed block on the newest claim is not silently skipped
            // in favour of an older good one — that would hide the error.
            Some(Err(e)) => {
                return Err(Error::Malformed {
                    subject: subject.to_string(),
                    cid: claim.cid.clone(),
                    source: e,
                })
            }
            None => continue,
        }
    }
    Ok(None)
}

/// The composition check: every declared `next` edge must name an atom that
/// exists, and every atom that has upstream atoms must have its inputs
/// covered by what those upstreams produce. A derived read over kan's fold —
/// the same category of computation as kan's own status fold, computing
/// nothing into the log.
///
/// Coverage is checked against the **transitive** upstream closure, not just
/// the immediate predecessor. Artifacts accumulate along a bridging path
/// rather than being consumed by the next step: an adversarial review needs
/// both the design doc and the code change, and the design doc is still
/// there when the review runs even though the build step in between didn't
/// re-emit it. Checking only immediate predecessors reports that as a
/// failure, which is how this rule was found — day's own two atoms tripped
/// it the first time `day doctor` ran against a real log.
///
/// An atom with no upstream atoms is a source; its inputs come from outside
/// the vocabulary and are not checked.
pub fn check(atoms: &[Atom]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for atom in atoms {
        for successor in &atom.interface.next {
            if !atoms.iter().any(|a| &a.name == successor) {
                findings.push(Finding {
                    atoms: vec![atom.name.clone(), successor.clone()],
                    message: format!(
                        "{} declares next: {successor}, but no {ATOM_PREFIX}{successor} subject exists in the live vocabulary",
                        atom.subject()
                    ),
                });
            }
        }
    }

    for atom in atoms {
        let upstream = ancestors(atoms, &atom.name);
        if upstream.is_empty() {
            continue;
        }

        let available: Vec<&str> = upstream
            .iter()
            .flat_map(|a| a.interface.outputs.iter().map(String::as_str))
            .collect();
        let missing: Vec<&str> = atom
            .interface
            .inputs
            .iter()
            .filter(|input| !available.contains(&input.as_str()))
            .map(String::as_str)
            .collect();

        if !missing.is_empty() {
            let mut implicated: Vec<String> = upstream.iter().map(|a| a.name.clone()).collect();
            implicated.push(atom.name.clone());
            findings.push(Finding {
                atoms: implicated,
                message: format!(
                    "{}: interfaces do not compose — needs input(s) [{}] that nothing upstream produces (upstream {} produce [{}])",
                    atom.subject(),
                    missing.join(", "),
                    upstream
                        .iter()
                        .map(|a| a.subject())
                        .collect::<Vec<_>>()
                        .join(", "),
                    available.join(", "),
                ),
            });
        }
    }

    findings
}

/// Every atom that can reach `name` through `next` edges. Breadth-first with
/// a visited set, so a cyclic vocabulary terminates rather than hanging —
/// cycles are legal here (a drift-evaluation atom feeding back into design
/// is a real pattern), they just must not be walked twice.
fn ancestors<'a>(atoms: &'a [Atom], name: &str) -> Vec<&'a Atom> {
    let mut found: Vec<&Atom> = Vec::new();
    let mut frontier = vec![name.to_string()];
    let mut seen: Vec<String> = vec![name.to_string()];

    while let Some(current) = frontier.pop() {
        for candidate in atoms {
            if !candidate.interface.next.contains(&current) {
                continue;
            }
            if seen.contains(&candidate.name) {
                continue;
            }
            seen.push(candidate.name.clone());
            frontier.push(candidate.name.clone());
            found.push(candidate);
        }
    }

    found.sort_by(|a, b| a.name.cmp(&b.name));
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atom(name: &str, inputs: &[&str], outputs: &[&str], next: &[&str]) -> Atom {
        Atom {
            name: name.to_string(),
            cid: format!("bafy-{name}"),
            interface: Interface {
                inputs: inputs.iter().map(|s| s.to_string()).collect(),
                outputs: outputs.iter().map(|s| s.to_string()).collect(),
                next: next.iter().map(|s| s.to_string()).collect(),
            },
        }
    }

    #[test]
    fn a_written_interface_reads_back_identical() {
        let interface = Interface {
            inputs: vec!["design-doc".into()],
            outputs: vec!["code-change".into()],
            next: vec!["adversarial-review".into()],
        };
        let text = interface.to_claim_text("generative-build", None);
        let parsed = extract_interface(&text)
            .expect("the written text should contain a block")
            .expect("the written block should be valid");
        assert_eq!(parsed, interface);
    }

    #[test]
    fn extracts_a_fenced_interface_block() {
        let text = "The build atom.\n\n```day-atom\n{\"in\": [\"design-doc\"], \"out\": [\"code-change\"]}\n```\n";
        let interface = extract_interface(text)
            .expect("block present")
            .expect("valid json");
        assert_eq!(interface.inputs, vec!["design-doc"]);
        assert_eq!(interface.outputs, vec!["code-change"]);
        assert!(interface.next.is_empty());
    }

    #[test]
    fn claim_text_with_no_block_is_not_a_finding() {
        assert!(extract_interface("just an ordinary observation").is_none());
    }

    #[test]
    fn malformed_block_is_distinguishable_from_absent_block() {
        let text = "```day-atom\n{not json}\n```";
        assert!(extract_interface(text).expect("block present").is_err());
    }

    #[test]
    fn composing_interfaces_pass() {
        let atoms = vec![
            atom("design", &["idea"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &[]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn mismatched_interfaces_name_both_atoms() {
        let atoms = vec![
            atom("design", &["idea"], &["design-doc"], &["build"]),
            atom("build", &["verified-spec"], &["code-change"], &[]),
        ];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].atoms, vec!["design", "build"]);
        assert!(findings[0].message.contains("verified-spec"));
    }

    #[test]
    fn an_artifact_stays_available_past_the_step_that_did_not_re_emit_it() {
        // The case day's own vocabulary hit: review needs the design doc
        // and the code change; build sits in between and only produces the
        // code change. The design doc is still there when review runs.
        let atoms = vec![
            atom("design", &["intent"], &["design-doc"], &["build"]),
            atom("build", &["design-doc"], &["code-change"], &["review"]),
            atom("review", &["design-doc", "code-change"], &["verdict"], &[]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn a_cyclic_vocabulary_terminates() {
        let atoms = vec![
            atom("design", &["drift-report"], &["design-doc"], &["drift"]),
            atom("drift", &["design-doc"], &["drift-report"], &["design"]),
        ];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn a_source_atoms_inputs_are_not_checked() {
        let atoms = vec![atom("design", &["intent"], &["design-doc"], &[])];
        assert_eq!(check(&atoms), vec![]);
    }

    #[test]
    fn dangling_successor_is_a_finding() {
        let atoms = vec![atom("design", &["idea"], &["design-doc"], &["nonexistent"])];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1);
        assert!(findings[0]
            .message
            .contains("no atom/nonexistent subject exists"));
    }
}
