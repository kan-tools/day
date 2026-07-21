//! `day doctor` — kan reachability plus the atom-composition check, shared
//! verbatim by the CLI verb and the MCP tool so the two surfaces can never
//! disagree about the state of the process layer.

use crate::atoms::{self, Atom, Finding};
use crate::kan_client::KanClient;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
}

#[derive(Debug)]
pub struct Report {
    pub atoms: Vec<Atom>,
    pub findings: Vec<Finding>,
}

impl Report {
    /// True when the live atom vocabulary composes cleanly. Drives the CLI
    /// exit code, so "healthy" is a single, testable predicate.
    pub fn is_healthy(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("kan: reachable\n");

        if self.atoms.is_empty() {
            out.push_str(
                "atoms: none declared yet — the process vocabulary is empty, which is a\n       valid starting state, not an error. See docs/CONVENTIONS.md.\n",
            );
        } else {
            out.push_str(&format!("atoms: {} declared\n", self.atoms.len()));
            for atom in &self.atoms {
                out.push_str(&format!(
                    "  {}  in[{}] out[{}]{}\n",
                    atom.subject(),
                    atom.interface.inputs.join(", "),
                    atom.interface.outputs.join(", "),
                    if atom.interface.next.is_empty() {
                        String::new()
                    } else {
                        format!(" -> {}", atom.interface.next.join(", "))
                    },
                ));
            }
        }

        if self.findings.is_empty() {
            out.push_str("composition: ok\n");
        } else {
            out.push_str(&format!(
                "composition: {} finding(s)\n",
                self.findings.len()
            ));
            for finding in &self.findings {
                out.push_str(&format!("  ! {}\n", finding.message));
            }
        }
        out
    }
}

/// Probes kan, then folds the live atom set and checks that it composes.
/// Reads only — a failed check is reported, never repaired.
pub fn run(client: &KanClient) -> Result<Report, Error> {
    client.probe()?;
    let (atoms, mut findings) = atoms::load(client)?;
    findings.extend(atoms::check(&atoms));
    Ok(Report { atoms, findings })
}
