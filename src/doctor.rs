//! `day doctor` — kan reachability plus the atom-composition check, shared
//! verbatim by the CLI verb and the MCP tool so the two surfaces can never
//! disagree about the state of the process layer.

use crate::atoms::{self, Atom, Finding};
use crate::compat::{self, Version};
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
    /// kan's version, or `None` when it could not be read (day#94). Carried
    /// rather than rendered on the spot so the MCP surface and the CLI report
    /// the same pairing.
    pub kan: Option<Version>,
}

impl Report {
    /// True when the live atom vocabulary composes cleanly. Drives the CLI
    /// exit code, so "healthy" is a single, testable predicate.
    ///
    /// **A kan version mismatch deliberately does not enter this.** The
    /// pairing is reported in [`render`](Self::render) and left out of the
    /// exit code for two reasons: this predicate is documented as the
    /// composition check and something may already script it, and a
    /// non-`Supported` verdict is usually [`Compat::Newer`] — a kan that
    /// outpaced day's measurements, which is normal and not a failure.
    /// Advisory, as everywhere else.
    ///
    /// [`Compat::Newer`]: crate::compat::Compat::Newer
    pub fn is_healthy(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&compat::render(self.kan.as_ref()));

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
    // Before the reads, so a report against an unsupported kan says which kan
    // produced it. `version` returns `None` rather than erroring: an
    // unreadable version must never turn a working day into a failing one.
    let kan = client.version();
    let (atoms, mut findings) = atoms::load(client)?;
    findings.extend(atoms::check(&atoms));
    Ok(Report {
        atoms,
        findings,
        kan,
    })
}
