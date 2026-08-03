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
    #[error("{subject}: {source} (claim {cid})")]
    Block {
        subject: String,
        cid: String,
        #[source]
        source: BlockError,
    },
}

/// The metadata key naming the reader version a block requires.
///
/// Underscore-prefixed to mark it as metadata rather than declared content: a
/// project's own block could legitimately want a field called `v` or
/// `version`, and this must never collide with one. Deliberately not
/// `$`-prefixed, because declared block schemas (day#74) may end up expressed
/// as JSON Schema, where `$` names are reserved.
pub const VERSION_KEY: &str = "_version";

/// The version an absent [`VERSION_KEY`] means. Every block written before
/// versioning existed is a v1 block, so absence has to be the first version
/// rather than an error.
pub const IMPLICIT_VERSION: u64 = 1;

/// The block version a type understands, declared **per block type** so a
/// reader fails only on the block that actually changed rather than on the
/// whole vocabulary.
///
/// This is the honest half of refusing unknown fields. `deny_unknown_fields`
/// *detects* that a block says more than this day can read; the version is what
/// lets the message say **why** — "this day reads `day-atom` v1, this block
/// declares v2, upgrade day" rather than a parse error that reads as the
/// project's mistake. day#60's lesson was that the v0.6 binary failed loudly
/// and misdirected the reader; detection without an actionable message repeats
/// it.
pub trait Versioned {
    /// The highest version of this block type this build can read.
    const SUPPORTED_VERSION: u64;
    /// The fence info string this block is declared under, for diagnostics.
    const FENCE: &'static str;

    /// Structural invariants **serde cannot express**, checked immediately after
    /// the typed parse so a block that is well-typed but meaningless is refused
    /// in the same place, and with the same diagnostics, as one that will not
    /// deserialize.
    ///
    /// This exists because `deny_unknown_fields` catches a block saying *more*
    /// than the type allows, and nothing caught a block saying *less than it
    /// needs to mean anything*. day#20 is the case: `{"any": []}` in a bridge
    /// plan is valid JSON and a valid `Vec<Node>`, and an empty alternative set
    /// contributed nothing and reported nothing. A plan grammar day writes can
    /// never produce one, but a hand-written block can — and hand-written blocks
    /// are supported deliberately, which makes this a real path rather than a
    /// hypothetical one.
    ///
    /// Default is `Ok`: most blocks have no invariant beyond their types, and a
    /// trait method nobody implements is cheaper than a second mechanism.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The fence a [`BlockError`] concerns.
///
/// `Cow` because day's own seven fences are `&'static str` constants and a
/// project-declared block's name is not. Borrowed costs nothing for the
/// built-ins; owned avoids the `Box::leak` the first implementation used, which
/// was bounded only because the leaking function was unreachable — and day ships
/// a long-running MCP server, so a per-call leak on a reachable path is a real
/// one.
pub type Fence = std::borrow::Cow<'static, str>;

/// Why a fenced block could not be read into its type.
#[derive(Debug, thiserror::Error)]
pub enum BlockError {
    /// The block declares a version newer than this build reads. **Not the
    /// project's mistake** — the reader is behind, and the message says so.
    #[error(
        "`{fence}` block declares {VERSION_KEY} {declared}, but this day reads \
         up to {supported} — upgrade day to read it"
    )]
    TooNew {
        fence: Fence,
        declared: u64,
        supported: u64,
    },
    /// The block is malformed at a version this build does read — invalid
    /// JSON, or a field the block type does not declare. **This one is the
    /// claim's problem**, and the message points at the claim.
    #[error("`{fence}` block could not be read: {source}")]
    Malformed {
        fence: Fence,
        #[source]
        source: serde_json::Error,
    },
    /// The block deserialized, but violates an invariant its type cannot
    /// encode — see [`Versioned::validate`]. Also the claim's problem, not the
    /// reader's, so it is reported the same way `Malformed` is rather than as
    /// version skew.
    #[error("`{fence}` block is not a valid {fence}: {reason}")]
    Invalid { fence: Fence, reason: String },
}

impl BlockError {
    /// Whether the reader is behind the log, rather than the log being wrong.
    /// The two need different actions from different people, which is the
    /// whole reason they are distinct variants.
    pub fn is_version_skew(&self) -> bool {
        matches!(self, BlockError::TooNew { .. })
    }
}

/// An atom's declared interface. `inputs`/`outputs` are free-form type
/// names — day checks that they *match*, deliberately not what they mean;
/// the vocabulary of type names is the project's to choose and evolve.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    #[serde(rename = "in", default)]
    pub inputs: Vec<String>,
    #[serde(rename = "out", default)]
    pub outputs: Vec<String>,
    /// Atoms this one declares it composes into, by slug. **Forward only**:
    /// the relation is "what follows this", never "what this can send you back
    /// to". See [`revisits`](Self::revisits) for the other one, and
    /// [`Forward`] for the acyclic view every ordering is read through.
    ///
    /// Read this field raw only to render the declaration as written. Anything
    /// that treats it as an *order* goes through [`Forward`], which is enforced
    /// by a source scan in `tests/plugin.rs` — see [`Forward`] for why.
    #[serde(default)]
    pub next: Vec<String>,
    /// Atoms a negative outcome here sends you **back** to, by slug. The
    /// feedback half of what `next` used to carry alone: an adversarial review
    /// blocking sends you back to the build, and that is a real relation with
    /// its own uses ("what work can this atom invalidate?") rather than a
    /// dumping ground for edges that break the DAG.
    ///
    /// May be cyclic; is never an ordering, never contributes to input
    /// coverage, and is never walked by anything that wants a partial order.
    /// That is the whole point of it being a separate field (day#113): with
    /// feedback out of `next`, `next` alone is a guaranteed DAG, and
    /// reachability, topological ordering and partial-order reporting go from
    /// unavailable to trivial.
    ///
    /// Additive, the same mechanism `done` uses: `skip_serializing_if` keeps
    /// every block written before this existed byte-identical. A block that
    /// *does* use it declares `_version: 2` — see [`Interface::to_claim_text`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub revisits: Vec<String>,
    /// Witness types that would evidence this atom is **done**, resolved
    /// through the same `schema/witness` probes teloi use. `in`/`out`/`next`
    /// say what an atom consumes, produces, and leads to; this says how you
    /// know it finished. Absent means no completion criteria are declared —
    /// reported as such, never treated as met.
    ///
    /// Additive: `skip_serializing_if` keeps every block written before this
    /// existed byte-identical, the same mechanism `Witnesses::scope` uses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub done: Vec<String>,
}

/// The `day-atom` version that introduced [`Interface::revisits`], and the
/// version a block declares **only when it uses that field**.
///
/// Conditional on purpose. [`VERSION_KEY`] is documented as the reader version
/// a block *requires*, and a block with no `revisits` requires nothing this
/// day's predecessors could not already read — so stamping every block a new
/// day writes would turn a compatible vocabulary into an incompatible one for
/// no reason.
pub const INTERFACE_VERSION_REVISITS: u64 = 2;

impl crate::atoms::Versioned for Interface {
    /// An atom's interface. v1 is every block written before versioning
    /// existed, which an absent `_version` still means; v2 added `revisits`.
    const SUPPORTED_VERSION: u64 = INTERFACE_VERSION_REVISITS;
    const FENCE: &'static str = FENCE_INFO;

    /// One slug cannot be both a successor and a revisit. Decidable from a
    /// single block, which is why it lives here rather than in [`check`] — the
    /// *return* rule (a revisit's target must reach this atom through `next`)
    /// needs the whole atom set and belongs there instead.
    ///
    /// No block written before `revisits` existed can trip this, so it refuses
    /// nothing that upgrading a project would newly break — the reason a
    /// `next` cycle is a finding and this is a refusal.
    fn validate(&self) -> Result<(), String> {
        let both: Vec<&str> = self
            .next
            .iter()
            .filter(|n| self.revisits.iter().any(|r| r == *n))
            .map(String::as_str)
            .collect();
        if both.is_empty() {
            return Ok(());
        }
        Err(format!(
            "[{}] appear in both `next` and `revisits` — an edge is either what follows \
             this atom or what it sends you back to, and declaring both says neither",
            both.join(", ")
        ))
    }
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
    ///
    /// Stamps [`VERSION_KEY`] **only when the block uses a field an older day
    /// could not read** — today, only `revisits`. An older day handed a
    /// stamped block reports [`BlockError::TooNew`] ("upgrade day to read
    /// it"), which is true and actionable; handed an unstamped one it reports
    /// [`BlockError::Malformed`], which would blame the claim for the reader
    /// being behind. See [`INTERFACE_VERSION_REVISITS`] for why this is not
    /// stamped unconditionally.
    pub fn to_claim_text(&self, slug: &str, note: Option<&str>) -> String {
        let json = self.to_block_json();
        let note = note
            .map(|n| format!("{n}\n\n"))
            .unwrap_or_else(|| format!("The {slug} atom.\n\n"));
        format!("{note}```{FENCE_INFO}\n{json}\n```\n")
    }

    /// The block body alone, version-stamped per [`to_claim_text`]'s rule.
    ///
    /// Split out so the stamping rule has one home and can be asserted
    /// directly, rather than only through the claim text that wraps it.
    ///
    /// [`to_claim_text`]: Self::to_claim_text
    pub fn to_block_json(&self) -> String {
        let body = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        if self.revisits.is_empty() {
            return body;
        }
        // Spliced onto the serialized struct rather than rebuilt through a
        // `serde_json::Value`. serde_json's `Map` is a `BTreeMap` unless
        // `preserve_order` is on, so a round-trip through `Value` would
        // re-sort every key and change the bytes of blocks that are supposed
        // to be untouched by this feature. Splicing keeps every other byte
        // exactly as the unstamped path produced it, which is what makes
        // "the stamp is the only difference" an assertable property rather
        // than a hope.
        //
        // `in`/`out`/`next` are never skipped, so a serialized interface is
        // always a non-empty object; the `else` is unreachable in practice and
        // returns the unstamped body rather than inventing a shape.
        match body.strip_prefix('{') {
            Some(rest) if rest != "}" => {
                format!("{{\"{VERSION_KEY}\":{INTERFACE_VERSION_REVISITS},{rest}")
            }
            _ => body,
        }
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
    /// True when this finding is a declaration day **could not read at all**,
    /// as opposed to one it read and found wrong.
    ///
    /// This is the flag that decides whether the rest of day's report is
    /// *partial*. A dangling `next` edge is day reporting something it did read;
    /// an unreadable block means the vocabulary is incomplete and every
    /// conclusion drawn from it is qualified.
    pub unreadable: bool,
    /// True when this finding is a declaration **this build is too old to
    /// read**, rather than one that is wrong.
    ///
    /// Carried as a flag rather than left for a caller to grep out of
    /// `message`, because the two need different actions from different people
    /// — upgrade the binary, or fix the claim — and a caller deciding that by
    /// substring match would break the first time the wording changed.
    pub version_skew: bool,
    /// True when day **could not perform this check**, as opposed to performing
    /// it and finding a fault. Excluded from [`crate::doctor::Report::is_healthy`],
    /// and therefore from the exit code.
    ///
    /// The distinction against `unreadable`, which *does* fail: an unreadable
    /// block is not legal at any version, whereas a cycle in `next` is a
    /// perfectly legal declaration in a vocabulary written before `revisits`
    /// existed (day#113). Failing on one would break every such project on
    /// upgrade, before its author had touched anything — and day is advisory,
    /// so an existing project gets told, not broken.
    ///
    /// Never a way to be quiet: an unchecked finding is still a finding, still
    /// rendered, and still says which check could not run. Could-not-check
    /// outranks checked-and-clean; it does not outrank being reported.
    pub unchecked: bool,
}

impl Finding {
    /// A fault: day ran the check and the vocabulary is wrong.
    fn fault(atoms: Vec<String>, message: String) -> Self {
        Finding {
            atoms,
            message,
            unreadable: false,
            version_skew: false,
            unchecked: false,
        }
    }

    /// A check day could not run. See [`Finding::unchecked`].
    fn unchecked(atoms: Vec<String>, message: String) -> Self {
        Finding {
            unchecked: true,
            ..Finding::fault(atoms, message)
        }
    }
}

/// One cycle in the declared `next` relation, and the edges [`Forward`] had to
/// drop because of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    /// Every atom on the cycle, sorted, so the same cycle renders identically
    /// however it was discovered.
    pub atoms: Vec<String>,
    /// The `(from, to)` edges excluded from the ordering, sorted.
    pub dropped: Vec<(String, String)>,
}

impl Cycle {
    pub fn message(&self) -> String {
        format!(
            "{} are in a cycle through `next` ({}), so day cannot order them — `next` is \
             forward-only, and the edge that sends you back belongs in `revisits`",
            self.atoms
                .iter()
                .map(|a| format!("{ATOM_PREFIX}{a}"))
                .collect::<Vec<_>>()
                .join(" and "),
            self.dropped
                .iter()
                .map(|(from, to)| format!("{from} -> {to}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// The forward relation over `next`, **guaranteed acyclic**, together with the
/// cycles that had to be dropped to guarantee it.
///
/// This type is the day#113 deliverable. `next` used to carry both sequence
/// ("review follows build") and feedback ("a review sends you back to fix"),
/// which made every consumer that read it as an ordering wrong on any
/// vocabulary with a feedback edge — including day's own. Feedback now lives in
/// [`Interface::revisits`], so a *migrated* vocabulary has nothing to drop and
/// this view is simply the declaration. For one that has not migrated, the
/// cyclic edges are excluded and **reported**, never silently skipped.
///
/// The guarantee is structural, not documentary: [`cycles`](Self::cycles) sits
/// on the same value as [`successors`](Self::successors), so a caller cannot
/// obtain the ordering without also holding what could not be ordered. That is
/// the shape day#101 keeps asking for — push the guarantee into the mechanism
/// rather than wiring a check at a call site — and the same one
/// [`crate::position::infer`] uses in taking a whole `WitnessSchema` rather
/// than the half a caller happened to want.
///
/// It is backed by a source scan. `tests/plugin.rs` fails the build if
/// `interface.next` is read outside this module without an adjacent
/// `dag-not-required: <why>` comment, because CLAUDE.md records five separate
/// occasions on which a rule stated in one module's doc comment did not reach
/// the others. Rendering the declaration as written is a legitimate raw read;
/// treating it as an order is not.
#[derive(Debug, Clone)]
pub struct Forward<'a> {
    successors: std::collections::BTreeMap<&'a str, Vec<&'a str>>,
    cycles: Vec<Cycle>,
}

impl<'a> Forward<'a> {
    /// Drops every edge that participates in a cycle, and records the cycles.
    ///
    /// An edge `u -> v` is cyclic iff `v` reaches `u` through declared `next`
    /// edges, which covers a self-loop (`u -> u`) with no special case. Cycles
    /// are the non-trivial strongly connected components, computed as the
    /// mutual-reachability partition rather than by Tarjan: a vocabulary is
    /// single digits of atoms, and `docs/CONVENTIONS.md` asks the composition
    /// check to be boring and obviously right rather than asymptotically
    /// clever.
    ///
    /// An edge naming an atom that does not exist is **kept**. It is not part
    /// of any cycle, and dropping it here would take the dangling-edge finding
    /// in [`check`] with it — the check would go quiet on exactly the
    /// declaration it exists to report.
    pub fn build(atoms: &'a [Atom]) -> Self {
        let declared: std::collections::BTreeMap<&str, &[String]> = atoms
            .iter()
            // dag-not-required: this is the one place the raw declaration is
            // read *in order to* build the acyclic view from it.
            .map(|a| (a.name.as_str(), a.interface.next.as_slice()))
            .collect();

        let reaches = |from: &str, to: &str| -> bool {
            let mut frontier: Vec<&str> = declared
                .get(from)
                .map(|next| next.iter().map(String::as_str).collect())
                .unwrap_or_default();
            let mut seen: std::collections::BTreeSet<&str> = frontier.iter().copied().collect();
            while let Some(current) = frontier.pop() {
                if current == to {
                    return true;
                }
                for successor in declared.get(current).copied().unwrap_or(&[]) {
                    if seen.insert(successor.as_str()) {
                        frontier.push(successor.as_str());
                    }
                }
            }
            false
        };

        let mut successors: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
        let mut dropped: Vec<(&str, &str)> = Vec::new();
        for atom in atoms {
            let mut kept = Vec::new();
            for successor in declared.get(atom.name.as_str()).copied().unwrap_or(&[]) {
                if reaches(successor, &atom.name) {
                    dropped.push((atom.name.as_str(), successor.as_str()));
                } else {
                    kept.push(successor.as_str());
                }
            }
            successors.insert(atom.name.as_str(), kept);
        }

        // Mutual reachability partitions the atoms touched by a dropped edge
        // into their strongly connected components. A self-loop puts one atom
        // in a component alone, which is a real cycle and reported as one.
        let mut on_a_cycle: Vec<&str> =
            dropped.iter().flat_map(|(from, to)| [*from, *to]).collect();
        on_a_cycle.sort_unstable();
        on_a_cycle.dedup();

        let mut cycles: Vec<Cycle> = Vec::new();
        let mut assigned: std::collections::BTreeSet<&str> = Default::default();
        for atom in &on_a_cycle {
            if !assigned.insert(atom) {
                continue;
            }
            let mut component = vec![*atom];
            for other in &on_a_cycle {
                if other != atom && reaches(atom, other) && reaches(other, atom) {
                    assigned.insert(other);
                    component.push(*other);
                }
            }
            component.sort_unstable();
            let mut edges: Vec<(String, String)> = dropped
                .iter()
                .filter(|(from, _)| component.contains(from))
                .map(|(from, to)| (from.to_string(), to.to_string()))
                .collect();
            edges.sort();
            cycles.push(Cycle {
                atoms: component.into_iter().map(str::to_string).collect(),
                dropped: edges,
            });
        }

        Forward { successors, cycles }
    }

    /// What follows `name`, with no cyclic edge in it.
    pub fn successors(&self, name: &str) -> &[&'a str] {
        self.successors.get(name).map_or(&[], Vec::as_slice)
    }

    /// Every atom that can reach `name` through forward edges — the transitive
    /// upstream closure input coverage is checked against.
    ///
    /// Terminating because the graph is acyclic. The visited set is an
    /// optimisation now; before day#113 it was the only thing keeping this
    /// from hanging on day's own vocabulary.
    pub fn ancestors(&self, atoms: &'a [Atom], name: &str) -> Vec<&'a Atom> {
        let mut found: Vec<&Atom> = Vec::new();
        let mut frontier = vec![name];
        let mut seen: std::collections::BTreeSet<&str> = [name].into_iter().collect();

        while let Some(current) = frontier.pop() {
            for candidate in atoms {
                if !self.successors(&candidate.name).contains(&current) {
                    continue;
                }
                if !seen.insert(candidate.name.as_str()) {
                    continue;
                }
                frontier.push(candidate.name.as_str());
                found.push(candidate);
            }
        }

        found.sort_by(|a, b| a.name.cmp(&b.name));
        found
    }

    /// The cycles that had to be dropped. Empty for a migrated vocabulary.
    ///
    /// A consumer holding a [`Forward`] holds this too; that is the guarantee.
    pub fn cycles(&self) -> &[Cycle] {
        &self.cycles
    }
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
            // The `BlockError` already names the fence and says whether this
            // day is behind the log or the block is wrong, so this wrapper adds
            // only the subject and the claim. Restating it here is what made
            // the message say "not valid interface JSON (not valid day-atom
            // JSON: …)" — the same thing twice, in two vocabularies.
            Some((claim, Err(e))) => findings.push(Finding {
                atoms: vec![name.clone()],
                // Every `BlockError` variant means unreadable, including
                // `Invalid` — a structurally-empty plan node is as unreadable as
                // a malformed one. Set from the error's type rather than by
                // matching its wording, which is the bug this replaced: callers
                // grepped for "could not be read", so `Invalid` ("is not a
                // valid …") slipped past and day#20's refusal reached no hook
                // channel at all.
                unreadable: true,
                version_skew: e.is_version_skew(),
                unchecked: false,
                message: format!("{subject}: {e} — claim {}", claim.cid),
            }),
            None => findings.push(Finding::fault(
                vec![name.clone()],
                format!(
                    "{subject}: no `{FENCE_INFO}` interface block on any live claim, so it can't be composition-checked"
                ),
            )),
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
pub fn extract_fenced<T: serde::de::DeserializeOwned + Versioned>(
    text: &str,
) -> Option<Result<T, BlockError>> {
    // The fence comes from `T::FENCE`, not from a parameter. It used to be both:
    // a caller passed a fence string to locate the block, and the diagnostics
    // reported `T::FENCE` — two sources of truth for one fact, so a mismatch
    // would find a block by one name and blame another. Every call site passed a
    // constant equal to the type's own fence, so nothing was broken; removing
    // the parameter makes that true by construction instead of by nine call
    // sites continuing to agree. Diagnostics naming the right cause is the whole
    // point of the error split.
    let open = format!("```{}", T::FENCE);
    let start = text.find(&open)? + open.len();
    let rest = &text[start..];
    let end = rest.find("```")?;
    Some(parse_block::<T>(rest[..end].trim()))
}

/// The version gate, then the typed parse.
///
/// The version gate itself lives in [`version_gate`], shared with the
/// project-declared path so a project's own vocabulary inherits the same
/// diagnostics rather than growing a parallel set.
pub(crate) fn parse_block<T: serde::de::DeserializeOwned + Versioned>(
    json: &str,
) -> Result<T, BlockError> {
    let value = version_gate(json, Fence::Borrowed(T::FENCE), T::SUPPORTED_VERSION)?;
    let parsed: T = serde_json::from_value(value).map_err(|source| BlockError::Malformed {
        fence: Fence::Borrowed(T::FENCE),
        source,
    })?;
    parsed.validate().map_err(|reason| BlockError::Invalid {
        fence: Fence::Borrowed(T::FENCE),
        reason,
    })?;
    Ok(parsed)
}

/// Parses a block body, applies the version gate, and hands back the remainder
/// with [`VERSION_KEY`] removed.
///
/// **Shared by the typed path and the project-declared one** (`src/blocks.rs`),
/// which is what makes a declared block inherit versioning and the
/// `BlockError` diagnostics rather than growing its own. A project's own
/// vocabulary gets the same "this day reads v1, this block declares v2 — upgrade
/// day" message day's built-ins get, because it is literally the same message.
///
/// The version is read and then **removed** before anything typed happens. Three
/// reasons, and the third decides it:
///
/// 1. No block type needs a `_version` field, so nothing day already writes
///    changes shape and no round-trip becomes non-byte-identical.
/// 2. `deny_unknown_fields` and `serde(flatten)` do not compose, so a shared
///    metadata struct was never available.
/// 3. [`crate::telos::WitnessSchema`] and `blocks::BlockSchemas` are both
///    `transparent` over a map. A `_version` *field* there would be read as a
///    map entry literally named `_version`; stripping it first is the only
///    approach that works for a block whose body is a map rather than a struct.
pub(crate) fn version_gate(
    json: &str,
    fence: Fence,
    supported: u64,
) -> Result<serde_json::Value, BlockError> {
    let malformed = |source| BlockError::Malformed {
        fence: fence.clone(),
        source,
    };
    let mut value: serde_json::Value = serde_json::from_str(json).map_err(malformed)?;

    // A non-object block (a bare array, say) carries no metadata and cannot be
    // version-gated; hand it on, and let the typed parse or the field check
    // reject it.
    if let Some(object) = value.as_object_mut() {
        if let Some(declared) = object.remove(VERSION_KEY) {
            let Some(declared) = declared.as_u64() else {
                // A `_version` that is not a number is a malformed block rather
                // than a version this day cannot read: day cannot tell whether
                // it is behind, so it must not claim to be.
                return Err(malformed(serde::de::Error::custom(format!(
                    "{VERSION_KEY} must be a positive integer, found `{declared}`"
                ))));
            };
            if declared > supported {
                return Err(BlockError::TooNew {
                    fence: fence.clone(),
                    declared,
                    supported,
                });
            }
        }
    }
    Ok(value)
}

/// [`extract_fenced`] specialized to an atom's `day-atom` interface block.
pub fn extract_interface(text: &str) -> Option<Result<Interface, BlockError>> {
    extract_fenced(text)
}

/// Reads the newest claim on `subject` carrying a `fence` block, returning
/// the parsed value with the CID of the claim it came from. The
/// newest-wins rule every kan-backed vocabulary in day uses.
pub fn newest_fenced<T: serde::de::DeserializeOwned + Versioned>(
    client: &KanClient,
    subject: &str,
) -> Result<Option<(String, T)>, Error> {
    let claims = client.show(subject)?;
    for claim in claims.iter().rev() {
        let Some(text) = claim.text.as_deref() else {
            continue;
        };
        match extract_fenced::<T>(text) {
            Some(Ok(value)) => return Ok(Some((claim.cid.clone(), value))),
            // An unreadable block on the newest claim is not silently skipped
            // in favour of an older good one — that would hide the error, and
            // would silently resolve an *older* declaration as though it were
            // current, which is worse than failing.
            Some(Err(source)) => {
                return Err(Error::Block {
                    subject: subject.to_string(),
                    cid: claim.cid.clone(),
                    source,
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
///
/// Since day#113 the closure is [`Forward`]'s, not the raw declaration's, so a
/// cycle in `next` cannot make every atom on it vacuously upstream of every
/// other. Where that exclusion is what left an input uncovered, the finding
/// says so and does not fail — see [`Finding::unchecked`].
pub fn check(atoms: &[Atom]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let forward = Forward::build(atoms);
    let exists = |slug: &String| atoms.iter().any(|a| &a.name == slug);

    for atom in atoms {
        // dag-not-required: the dangling-edge check is about what the claim
        // *declares*, so it must see an edge even when the ordering dropped it.
        for successor in &atom.interface.next {
            if !exists(successor) {
                findings.push(Finding::fault(
                    vec![atom.name.clone(), successor.clone()],
                    format!(
                        "{} declares next: {successor}, but no {ATOM_PREFIX}{successor} subject exists in the live vocabulary",
                        atom.subject()
                    ),
                ));
            }
        }
        for target in &atom.interface.revisits {
            if !exists(target) {
                findings.push(Finding::fault(
                    vec![atom.name.clone(), target.clone()],
                    format!(
                        "{} declares revisits: {target}, but no {ATOM_PREFIX}{target} subject exists in the live vocabulary",
                        atom.subject()
                    ),
                ));
                continue;
            }
            // A revisit is a *return*: it names work this atom's negative
            // outcome sends you back to, so the target must be somewhere the
            // path already went. One that is not a return is almost always a
            // forward edge filed in the wrong field — the exact confusion
            // day#113 removed — so it is reported rather than accepted.
            //
            // A finding, not a refusal: day read the declaration and
            // understood it, and refusing what it understood is how an
            // advisory tool becomes a blocking one.
            if !forward
                .ancestors(atoms, &atom.name)
                .iter()
                .any(|a| &a.name == target)
            {
                findings.push(Finding::fault(
                    vec![atom.name.clone(), target.clone()],
                    format!(
                        "{} revisits {target}, but {ATOM_PREFIX}{target} does not reach it through `next` — a revisit that is not a return has no defined meaning; if {target} follows this atom, declare it in `next`",
                        atom.subject()
                    ),
                ));
            }
        }
    }

    for cycle in forward.cycles() {
        findings.push(Finding::unchecked(cycle.atoms.clone(), cycle.message()));
    }

    for atom in atoms {
        let upstream = forward.ancestors(atoms, &atom.name);
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
            // Would the *declared* closure have covered it? If so, the answer
            // is unknown because a cycle was excluded, not because the
            // vocabulary is wrong — and a project whose vocabulary predates
            // `revisits` must not start failing on upgrade for that.
            let declared_covers = {
                let declared: Vec<&str> = declared_ancestors(atoms, &atom.name)
                    .iter()
                    .flat_map(|a| a.interface.outputs.iter().map(String::as_str))
                    .collect();
                missing.iter().all(|input| declared.contains(input))
            };
            let detail = format!(
                "needs input(s) [{}] that nothing upstream produces (upstream {} produce [{}])",
                missing.join(", "),
                upstream
                    .iter()
                    .map(|a| a.subject())
                    .collect::<Vec<_>>()
                    .join(", "),
                available.join(", "),
            );
            findings.push(if declared_covers {
                Finding::unchecked(
                    implicated,
                    format!(
                        "{}: day could not check that its interfaces compose — {detail}. Every missing input is produced by an atom day had to exclude from the ordering because it is on a cycle through `next`, so this is unknown rather than wrong",
                        atom.subject(),
                    ),
                )
            } else {
                Finding::fault(
                    implicated,
                    format!("{}: interfaces do not compose — {detail}", atom.subject()),
                )
            });
        }
    }

    findings
}

/// Every atom that can reach `name` through the **declared** `next` edges,
/// cycles and all.
///
/// Not an ordering and never used as one: its single purpose is to tell
/// [`check`] whether excluding a cycle is what left an input uncovered, so the
/// difference between "wrong" and "unknown" can be reported honestly. The
/// visited set is what keeps it terminating, since this closure is exactly the
/// one that may be cyclic.
fn declared_ancestors<'a>(atoms: &'a [Atom], name: &str) -> Vec<&'a Atom> {
    let mut found: Vec<&Atom> = Vec::new();
    let mut frontier = vec![name.to_string()];
    let mut seen: Vec<String> = vec![name.to_string()];

    while let Some(current) = frontier.pop() {
        for candidate in atoms {
            // dag-not-required: this closure is deliberately the raw one — it
            // exists to be compared against `Forward`'s.
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

/// `.design/honest-reads.md` REQ-1 and REQ-2, at the level the contract lives:
/// one gate, exercised against **every** block type day owns.
///
/// These are unit tests rather than seven end-to-end runs on purpose. Each block
/// type is read by a different verb (`doctor`, `assess telos`, `bridge check`,
/// `design check`, `assess docs`), so driving all seven through their verbs would
/// test the verbs, at seven subprocesses per case, while testing the gate once.
/// The gate is what has to hold for all seven.
#[cfg(test)]
mod version_gate {
    use super::*;
    use crate::{bridge, docs, schema, telos, tension};

    /// One row per block type day owns. `strict` records whether the body is a
    /// **struct**, where an unrecognised key is a field the type does not
    /// declare and must be refused — or a **map**, where every key is data and
    /// refusing unknown ones would refuse the project's own vocabulary.
    ///
    /// Adding a block type to day means adding a row here, which is the point:
    /// this table is the inventory, and a new block type that is neither strict
    /// nor deliberately lax cannot be added without someone deciding which.
    fn parse_all_seven(
        mutate: impl Fn(&str) -> String,
    ) -> Vec<(&'static str, bool, Result<(), String>)> {
        fn attempt<T: serde::de::DeserializeOwned + Versioned>(json: &str) -> Result<(), String> {
            parse_block::<T>(json)
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        vec![
            (
                "day-atom",
                true,
                attempt::<Interface>(&mutate(r#"{"in":["a"],"out":["b"]}"#)),
            ),
            (
                "day-telos",
                true,
                attempt::<bridge::Witnesses>(&mutate(r#"{"witnesses":["a"]}"#)),
            ),
            (
                "day-bridge",
                true,
                attempt::<bridge::Plan>(&mutate(
                    r#"{"telos":"t","have":["intent"],"plan":{"atom":"design"}}"#,
                )),
            ),
            (
                // A map from witness type to probe, so any key is a witness
                // type this project declared. Strictness lives one level down,
                // on the probe — see the `Versioned` impl for `WitnessSchema`.
                "day-witness",
                false,
                attempt::<telos::WitnessSchema>(&mutate(r#"{"code-change":{"path":"src/*"}}"#)),
            ),
            (
                "day-schema",
                true,
                attempt::<schema::Schema>(&mutate(
                    r#"{"sections":["Summary"],"requirement_prefix":"REQ-","criterion_prefix":"AC-","min_requirements":1,"min_criteria":1,"placeholders":[],"paths_section":"Architecture"}"#,
                )),
            ),
            (
                "day-docs",
                true,
                attempt::<docs::DocsSchema>(&mutate(
                    r#"{"version_source":"Cargo.toml","version_key":"version"}"#,
                )),
            ),
            (
                "day-tension",
                true,
                attempt::<tension::Tension>(&mutate(r#"{"between":["a","b"]}"#)),
            ),
        ]
    }

    /// AC-1's negative control, and it has to come first: if the untouched
    /// bodies did not parse, every assertion below would pass for the wrong
    /// reason.
    #[test]
    fn every_block_type_parses_its_own_minimal_body() {
        for (fence, _, result) in parse_all_seven(str::to_string) {
            assert!(result.is_ok(), "{fence} rejected a valid body: {result:?}");
        }
    }

    /// AC-1: an unrecognised field is **refused**, not dropped, for every
    /// struct-shaped block.
    ///
    /// Before this, all of them parsed and silently discarded the field — which
    /// for a vocabulary that exists to *constrain* something is a false
    /// certification rather than lost information.
    #[test]
    fn every_struct_shaped_block_refuses_an_unrecognised_field() {
        let inject = |json: &str| json.replacen('{', r#"{"nonsense_field":1,"#, 1);
        let rows = parse_all_seven(inject);
        assert!(
            rows.iter().filter(|(_, strict, _)| *strict).count() >= 6,
            "the strict set should not have quietly shrunk"
        );
        for (fence, strict, result) in rows {
            if !strict {
                continue;
            }
            let err = result.expect_err(&format!("{fence} silently dropped an unknown field"));
            assert!(
                err.contains("nonsense_field"),
                "{fence} should name the field it refused: {err}"
            );
        }
    }

    /// The `day-witness` map's contract, which is deliberately *not* the one
    /// above and would be wrong if it were: an unrecognised key is a witness
    /// type this project declared, and refusing it would refuse the project's
    /// own vocabulary. What must not happen is the probe being dropped
    /// silently — it is set aside and reported, so a reader never mistakes
    /// "unreadable here" for "no probe declared".
    #[test]
    fn an_unreadable_probe_is_set_aside_and_reported_not_dropped() {
        let schema = parse_block::<telos::WitnessSchema>(
            r#"{"code-change":{"path":"src/*"},"exotic":{"future-kind":{"x":1}}}"#,
        )
        .expect("a witness type day has never heard of is the project's business");

        assert!(schema.probes.contains_key("code-change"));
        assert!(
            !schema.probes.contains_key("exotic"),
            "an unreadable probe must not land in the usable set"
        );
        assert!(
            schema.unsupported.contains_key("exotic"),
            "and must be reported rather than dropped: {schema:?}"
        );
    }

    /// AC-3: an absent `_version` and an explicit `_version: 1` are the same
    /// block, so nothing written before versioning existed needs touching.
    #[test]
    fn an_absent_version_means_the_first_version() {
        let implicit = parse_block::<Interface>(r#"{"in":["a"],"out":["b"]}"#).unwrap();
        let explicit =
            parse_block::<Interface>(r#"{"_version":1,"in":["a"],"out":["b"]}"#).unwrap();
        assert_eq!(implicit, explicit);
        assert_eq!(IMPLICIT_VERSION, 1);
    }

    /// AC-3: `_version` is accepted on every block type, not only the one it
    /// was implemented against — including `day-witness`, whose body is a *map*
    /// and where a `_version` field would otherwise read as a witness type
    /// literally named `_version`.
    #[test]
    fn the_version_key_is_stripped_from_every_block_type() {
        let inject = |json: &str| json.replacen('{', r#"{"_version":1,"#, 1);
        for (fence, _, result) in parse_all_seven(inject) {
            assert!(
                result.is_ok(),
                "{fence} should accept an explicit v1: {result:?}"
            );
        }
        // The map case specifically: `_version` must not survive as a key.
        let schema =
            parse_block::<telos::WitnessSchema>(r#"{"_version":1,"code-change":{"path":"src/*"}}"#)
                .unwrap();
        assert!(
            !schema.probes.contains_key(VERSION_KEY)
                && !schema.unsupported.contains_key(VERSION_KEY),
            "the version key leaked into the witness map: {schema:?}"
        );
        assert!(schema.probes.contains_key("code-change"));
    }

    /// AC-4: a block this day is too old to read reports **the reader is
    /// behind**, distinguishably from a block that is simply wrong. The two
    /// need different actions from different people, which is why they are
    /// different variants rather than one message.
    ///
    /// The "too new" version is derived from `SUPPORTED_VERSION`, never
    /// hardcoded. It was hardcoded as `2`, and day#113 made 2 a version day
    /// reads — so the test that proves version skew is reported would have
    /// started proving it against a block that parses fine. A fixture pinned to
    /// a literal cannot stay ahead of the thing it is meant to be ahead of.
    #[test]
    fn a_too_new_block_blames_the_reader_and_a_broken_one_blames_the_claim() {
        let ahead = Interface::SUPPORTED_VERSION + 1;
        let too_new =
            parse_block::<Interface>(&format!(r#"{{"_version":{ahead},"in":["a"]}}"#)).unwrap_err();
        assert!(too_new.is_version_skew());
        let rendered = too_new.to_string();
        assert!(
            rendered.contains(&ahead.to_string()),
            "names the declared version: {rendered}"
        );
        assert!(
            rendered.contains(&Interface::SUPPORTED_VERSION.to_string()),
            "and the supported one: {rendered}"
        );
        assert!(
            rendered.contains("upgrade day"),
            "and says whose problem it is: {rendered}"
        );

        // Malformed, at a version this day does read.
        let broken = parse_block::<Interface>(r#"{"in":["a"],}"#).unwrap_err();
        assert!(!broken.is_version_skew());
        assert!(
            !broken.to_string().contains("upgrade day"),
            "a broken block must not tell the reader to upgrade: {broken}"
        );

        // An unknown field is the claim's problem too, not version skew — it is
        // only skew when the block *says* it needs a newer reader. Getting this
        // backwards would tell every project their day was out of date.
        let unknown = parse_block::<Interface>(r#"{"in":["a"],"requires":["x"]}"#).unwrap_err();
        assert!(!unknown.is_version_skew(), "{unknown}");
    }

    /// A `_version` that is not a number is the claim's problem, not the
    /// reader's: day cannot tell whether it is behind, so it must not claim to
    /// be. The tempting alternative — treat anything unparseable as "probably
    /// newer" — would send every reader to upgrade over a typo.
    #[test]
    fn a_non_numeric_version_is_malformed_not_skew() {
        let e = parse_block::<Interface>(r#"{"_version":"two","in":["a"]}"#).unwrap_err();
        assert!(!e.is_version_skew(), "{e}");
        assert!(e.to_string().contains(VERSION_KEY), "{e}");
    }

    /// Versions are **per block type**, so a `day-atom` this day cannot read
    /// does not make it unable to read a `day-telos`. One shared version bumped
    /// for one block would invalidate all seven for an older reader, which is
    /// the whole-vocabulary blast radius the smallest-unit rule exists to avoid.
    #[test]
    fn a_too_new_block_of_one_type_does_not_affect_another() {
        let ahead = Interface::SUPPORTED_VERSION + 1;
        assert!(
            parse_block::<Interface>(&format!(r#"{{"_version":{ahead},"in":["a"]}}"#)).is_err()
        );
        assert!(parse_block::<bridge::Witnesses>(r#"{"witnesses":["a"]}"#).is_ok());
    }
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
                revisits: vec![],
                done: vec![],
            },
        }
    }

    fn revisiting(mut atom: Atom, revisits: &[&str]) -> Atom {
        atom.interface.revisits = revisits.iter().map(|s| s.to_string()).collect();
        atom
    }

    #[test]
    fn a_written_interface_reads_back_identical() {
        let interface = Interface {
            inputs: vec!["design-doc".into()],
            outputs: vec!["code-change".into()],
            next: vec!["adversarial-review".into()],
            revisits: vec![],
            done: vec![],
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
        // dag-not-required: asserting what the block deserialized to, which is
        // the declaration itself and not an ordering over it.
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

    /// A cyclic `next` terminates, and — since day#113 — says so.
    ///
    /// It used to assert `check` found *nothing*, which was true and was the
    /// problem: the vocabulary was one day could not order, and every consumer
    /// downstream was told it was clean. The finding is `unchecked`, so
    /// `doctor` still exits zero; a legal declaration written before `revisits`
    /// existed gets told, not broken.
    #[test]
    fn a_cyclic_vocabulary_terminates_and_reports_that_it_could_not_be_ordered() {
        let atoms = vec![
            atom("design", &["drift-report"], &["design-doc"], &["drift"]),
            atom("drift", &["design-doc"], &["drift-report"], &["design"]),
        ];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(findings[0].unchecked, "{findings:#?}");
        assert_eq!(findings[0].atoms, vec!["design", "drift"]);
        assert!(findings[0].message.contains("revisits"), "{findings:#?}");
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

    /// AC-3, first half: the 2-cycle day's own vocabulary had. Both edges go,
    /// because "upstream" is undefined for either of them, and the cycle names
    /// both atoms and both edges so a reader can see which one to move.
    #[test]
    fn a_two_cycle_drops_both_edges_and_is_reported_once() {
        let atoms = vec![
            atom("build", &[], &["code-change"], &["review"]),
            atom("review", &[], &["verdict"], &["build"]),
        ];
        let forward = Forward::build(&atoms);

        assert!(forward.successors("build").is_empty());
        assert!(forward.successors("review").is_empty());
        assert_eq!(forward.cycles().len(), 1);
        assert_eq!(forward.cycles()[0].atoms, vec!["build", "review"]);
        assert_eq!(
            forward.cycles()[0].dropped,
            vec![
                ("build".to_string(), "review".to_string()),
                ("review".to_string(), "build".to_string()),
            ]
        );
    }

    /// AC-3, second half. An acyclic vocabulary is passed through untouched —
    /// the guarantee must cost a migrated project nothing, or nobody migrates.
    #[test]
    fn an_acyclic_vocabulary_keeps_every_edge_and_reports_no_cycle() {
        let atoms = vec![
            atom("design", &[], &["design-doc"], &["build"]),
            atom("build", &[], &["code-change"], &["review"]),
            atom("review", &[], &["verdict"], &[]),
        ];
        let forward = Forward::build(&atoms);

        assert_eq!(forward.successors("design"), ["build"]);
        assert_eq!(forward.successors("build"), ["review"]);
        assert!(forward.successors("review").is_empty());
        assert!(forward.cycles().is_empty());
        assert_eq!(
            forward
                .ancestors(&atoms, "review")
                .iter()
                .map(|a| a.name.as_str())
                .collect::<Vec<_>>(),
            ["build", "design"]
        );
    }

    /// AC-4: a self-loop is a cycle of one. Handled by the same rule as every
    /// other cycle — `u -> u` is an edge whose target reaches `u` — rather than
    /// by a special case, which is the point of phrasing the rule that way.
    #[test]
    fn a_self_loop_is_a_cycle_of_one() {
        let atoms = vec![atom("build", &[], &["code-change"], &["build"])];
        let forward = Forward::build(&atoms);

        assert!(forward.successors("build").is_empty());
        assert_eq!(forward.cycles().len(), 1);
        assert_eq!(forward.cycles()[0].atoms, vec!["build"]);
        assert_eq!(
            forward.cycles()[0].dropped,
            vec![("build".to_string(), "build".to_string())]
        );
    }

    /// An edge naming an atom that does not exist survives the ordering, so the
    /// dangling-edge finding still fires. Dropping unknown targets in `Forward`
    /// would silence `check` on exactly the declaration it exists to report —
    /// a fix in one place going quiet in another.
    #[test]
    fn a_dangling_edge_is_not_swallowed_by_the_ordering() {
        let atoms = vec![atom("design", &[], &["design-doc"], &["nonexistent"])];
        let forward = Forward::build(&atoms);

        assert_eq!(forward.successors("design"), ["nonexistent"]);
        assert!(forward.cycles().is_empty());
        assert_eq!(check(&atoms).len(), 1);
    }

    /// AC-9: a coverage question day cannot answer **because it excluded a
    /// cycle** is reported as unanswered, not as a failure.
    ///
    /// The premise is asserted, not assumed: the test proves the fixture
    /// actually reaches the state — that the declared closure covers `x` and
    /// the acyclic one does not — before asserting how the finding is
    /// classified. Without that, the test passes on any fixture that produces
    /// some finding, which is the "fixture cannot reach the mode" trap.
    #[test]
    fn coverage_lost_only_to_a_dropped_cycle_is_unchecked_not_a_failure() {
        let atoms = vec![
            atom("design", &[], &["design-doc"], &["a"]),
            atom("a", &["x"], &["design-doc"], &["b"]),
            atom("b", &[], &["x"], &["a"]),
        ];

        // The premise. `b` produces `x` and is an ancestor of `a` only through
        // the cycle, so excluding the cycle is exactly what makes `x`
        // uncoverable.
        let declared: Vec<&str> = declared_ancestors(&atoms, "a")
            .iter()
            .map(|at| at.name.as_str())
            .collect();
        assert!(declared.contains(&"b"), "declared closure: {declared:?}");
        let forward = Forward::build(&atoms);
        let acyclic: Vec<&str> = forward
            .ancestors(&atoms, "a")
            .iter()
            .map(|at| at.name.as_str())
            .collect();
        assert!(!acyclic.contains(&"b"), "acyclic closure: {acyclic:?}");

        let findings = check(&atoms);
        let coverage = findings
            .iter()
            .find(|f| f.message.contains("[x]"))
            .unwrap_or_else(|| panic!("no coverage finding about `x`: {findings:#?}"));
        assert!(coverage.unchecked, "{coverage:#?}");
        assert!(
            coverage.message.contains("could not check"),
            "{coverage:#?}"
        );
    }

    /// The other side of AC-9: coverage that fails for a reason unrelated to
    /// any cycle is still a **fault**. Without this, marking every coverage
    /// finding `unchecked` would pass the test above and silently disable the
    /// composition check.
    #[test]
    fn coverage_that_no_cycle_explains_is_still_a_failure() {
        let atoms = vec![
            atom("design", &[], &["design-doc"], &["build"]),
            atom("build", &["nothing-makes-this"], &["code-change"], &[]),
        ];
        let findings = check(&atoms);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(!findings[0].unchecked, "{findings:#?}");
        assert!(
            findings[0].message.contains("do not compose"),
            "{findings:#?}"
        );
    }

    /// AC-10, both halves, plus the case that must stay silent.
    #[test]
    fn a_revisit_is_checked_for_existence_and_for_being_a_return() {
        let dangling = vec![revisiting(
            atom("review", &[], &["verdict"], &[]),
            &["nonexistent"],
        )];
        let findings = check(&dangling);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(
            findings[0].message.contains("no atom/nonexistent subject"),
            "{findings:#?}"
        );

        // Exists, but nothing reaches `review` from it: this is a forward edge
        // in the wrong field.
        let not_a_return = vec![
            revisiting(atom("review", &[], &["verdict"], &[]), &["release"]),
            atom("release", &[], &["published-artifact"], &[]),
        ];
        let findings = check(&not_a_return);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(
            findings[0].message.contains("does not reach it through"),
            "{findings:#?}"
        );

        // day's own shape: build precedes review, and review sends you back.
        let genuine = vec![
            atom("build", &[], &["code-change"], &["review"]),
            revisiting(
                atom("review", &["code-change"], &["verdict"], &[]),
                &["build"],
            ),
        ];
        assert_eq!(check(&genuine), vec![], "a real return must be silent");
    }

    /// A `revisits` edge is never an ordering: it must not make its target an
    /// ancestor, or input coverage would be satisfied by going backwards around
    /// the loop — which is the vacuous coverage `next`'s cycles used to give.
    #[test]
    fn a_revisit_does_not_make_its_target_an_ancestor() {
        let atoms = vec![
            atom("build", &[], &["code-change"], &["review"]),
            revisiting(
                atom("review", &["code-change"], &["verdict"], &[]),
                &["build"],
            ),
        ];
        let forward = Forward::build(&atoms);
        assert!(forward.cycles().is_empty());
        assert!(
            forward.ancestors(&atoms, "build").is_empty(),
            "a revisit pointed `review -> build`; it must not make review an ancestor of build"
        );
    }

    /// AC-11: one slug cannot be both a successor and a revisit.
    #[test]
    fn an_edge_declared_both_forward_and_backward_is_refused() {
        let err = parse_block::<Interface>(r#"{"next":["x"],"revisits":["x"]}"#).unwrap_err();
        assert!(
            matches!(err, BlockError::Invalid { .. }),
            "expected Invalid, got {err:?}"
        );
        assert!(err.to_string().contains('x'), "{err}");
        // And not version skew: this is the claim's problem, not the reader's.
        assert!(!err.is_version_skew(), "{err}");
    }

    /// AC-1 and AC-2: the stamp appears when — and only when — `revisits` is
    /// used, and adding the field changed nothing about how an interface
    /// without it serializes.
    #[test]
    fn the_version_stamp_is_the_only_difference_a_revisit_makes() {
        let plain = Interface {
            inputs: vec!["design-doc".into()],
            outputs: vec!["code-change".into()],
            next: vec!["review".into()],
            revisits: vec![],
            done: vec![],
        };
        assert_eq!(
            plain.to_block_json(),
            r#"{"in":["design-doc"],"out":["code-change"],"next":["review"]}"#,
            "an interface with no revisits must serialize exactly as it did before the field existed"
        );

        let with_revisit = Interface {
            revisits: vec!["build".into()],
            ..plain.clone()
        };
        let stamped = with_revisit.to_block_json();
        assert_eq!(
            stamped,
            format!(
                r#"{{"_version":{INTERFACE_VERSION_REVISITS},"in":["design-doc"],"out":["code-change"],"next":["review"],"revisits":["build"]}}"#
            )
        );
        // And it round-trips through the reader that will see it.
        assert_eq!(parse_block::<Interface>(&stamped).unwrap(), with_revisit);
    }
}
