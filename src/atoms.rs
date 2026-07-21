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

/// Pulls the first fenced ```` ```day-atom ```` block out of a claim's text.
/// Returns `None` when the claim carries no block at all (most claims
/// don't), `Some(Err(..))` when it carries one that doesn't parse — the
/// difference matters, since the second is a real finding and the first
/// isn't.
pub fn extract_interface(text: &str) -> Option<Result<Interface, serde_json::Error>> {
    let open = format!("```{FENCE_INFO}");
    let start = text.find(&open)? + open.len();
    let rest = &text[start..];
    let end = rest.find("```")?;
    Some(serde_json::from_str(rest[..end].trim()))
}

/// The composition check: every declared `next` edge must name an atom that
/// exists, and the upstream atom's outputs must cover the downstream atom's
/// inputs. A derived read over kan's fold — the same category of
/// computation as kan's own status fold, computing nothing into the log.
pub fn check(atoms: &[Atom]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for atom in atoms {
        for successor in &atom.interface.next {
            let Some(downstream) = atoms.iter().find(|a| &a.name == successor) else {
                findings.push(Finding {
                    atoms: vec![atom.name.clone(), successor.clone()],
                    message: format!(
                        "{} declares next: {successor}, but no {ATOM_PREFIX}{successor} subject exists in the live vocabulary",
                        atom.subject()
                    ),
                });
                continue;
            };

            let missing: Vec<&str> = downstream
                .interface
                .inputs
                .iter()
                .filter(|input| !atom.interface.outputs.contains(input))
                .map(String::as_str)
                .collect();

            if !missing.is_empty() {
                findings.push(Finding {
                    atoms: vec![atom.name.clone(), downstream.name.clone()],
                    message: format!(
                        "{} -> {}: interfaces do not compose — {} needs input(s) [{}] that {} does not produce (it outputs [{}])",
                        atom.subject(),
                        downstream.subject(),
                        downstream.subject(),
                        missing.join(", "),
                        atom.subject(),
                        atom.interface.outputs.join(", "),
                    ),
                });
            }
        }
    }

    findings
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
    fn dangling_successor_is_a_finding() {
        let atoms = vec![atom("design", &["idea"], &["design-doc"], &["nonexistent"])];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1);
        assert!(findings[0]
            .message
            .contains("no atom/nonexistent subject exists"));
    }
}
