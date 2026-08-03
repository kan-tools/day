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
    /// **A finding day could not check deliberately does not enter this
    /// either** (day#113). A cycle in `next` is a legal declaration in a
    /// vocabulary written before `revisits` existed, so failing on one would
    /// break every such project on upgrade before its author had touched
    /// anything. It is still reported — see [`Finding::unchecked`].
    ///
    /// [`Compat::Newer`]: crate::compat::Compat::Newer
    /// [`Finding::unchecked`]: crate::atoms::Finding::unchecked
    pub fn is_healthy(&self) -> bool {
        self.findings.iter().all(|f| f.unchecked)
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
                    edges(atom),
                ));
            }
        }

        // Faults and could-not-checks are counted and rendered separately, and
        // `composition: ok` is about the faults alone. Collapsing them would
        // make a cycle — a legal declaration day merely cannot order — read as
        // a broken vocabulary, which is the `src/compat.rs` lesson day#113
        // quotes: a warning as loud as a failure is one a reader learns to
        // dismiss. `?` rather than `!` for the same reason, one level down.
        let (unchecked, faults): (Vec<&Finding>, Vec<&Finding>) =
            self.findings.iter().partition(|f| f.unchecked);

        if faults.is_empty() {
            out.push_str("composition: ok\n");
        } else {
            out.push_str(&format!("composition: {} finding(s)\n", faults.len()));
            for finding in &faults {
                out.push_str(&format!("  ! {}\n", finding.message));
            }
        }

        if !unchecked.is_empty() {
            out.push_str(&format!(
                "could not check: {} finding(s)\n",
                unchecked.len()
            ));
            for finding in &unchecked {
                out.push_str(&format!("  ? {}\n", finding.message));
            }
        }
        out
    }
}

/// An atom's declared edges, as the claim states them.
///
/// dag-not-required: `doctor` dumps the declaration, so it must show an edge
/// even when [`atoms::Forward`] dropped it from the ordering — a reader
/// comparing this against the cycle report below needs both sides.
fn edges(atom: &Atom) -> String {
    // Read once, into a local. Two reads under one marker used to pass the
    // scan, which meant the second inherited an exemption nobody wrote for it;
    // the marker now binds to the next read only, so a site that genuinely
    // needs the declaration twice says so once and uses it twice.
    let declared_next = &atom.interface.next;
    let mut out = String::new();
    if !declared_next.is_empty() {
        out.push_str(&format!(" -> {}", declared_next.join(", ")));
    }
    if !atom.interface.revisits.is_empty() {
        out.push_str(&format!(
            " (revisits {})",
            atom.interface.revisits.join(", ")
        ));
    }
    out
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
