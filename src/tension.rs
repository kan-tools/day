//! Tensions between teloi — the relationship, and where its reason lives.
//!
//! Several teloi normally apply to one project at once and usually pull
//! against each other. That tension is information, not a bug, and the
//! *reason* is the part a reader needs: "these two conflict" is much less
//! useful than "these two conflict because compelling the records legibility
//! needs would make day the kind of tool people route around".
//!
//! A kan relation carries no narrative body, so the reason has to be a claim
//! somewhere. It used to be a claim on one of the telos subjects, and that
//! was wrong in a way that took a while to see: everywhere day renders a
//! telos it shows the newest claim carrying text, so a recorded tension
//! *displaced the telos statement* in injected session context and in
//! assessments (day#32). Four of six teloi were affected.
//!
//! So the reason lives here, on `tension/<a>--<b>`, cited by both edges. A
//! telos subject carries its declaration and its edges and nothing else,
//! which means the newest text claim on a telos is the telos again — no
//! heuristic needed to tell a statement from commentary about it.
//!
//! Moving information must not make it unfindable, so this module also
//! provides the read back: [`for_telos`] answers "what does this pull
//! against, and why", and both `session_context` and `day assess telos`
//! surface it.

use serde::{Deserialize, Serialize};

use crate::atoms::{self, prose_only, TELOS_PREFIX};
use crate::kan_client::KanClient;

/// Subject-name prefix for a tension between two teloi.
pub const TENSION_PREFIX: &str = "tension/";
/// Fence info string marking a tension's participants inside a claim.
pub const FENCE_INFO: &str = "day-tension";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Atoms(#[from] atoms::Error),
    #[error(transparent)]
    Kan(#[from] crate::kan_client::Error),
}

/// Which two teloi a tension relates.
///
/// day locates tensions by reading this block, **never by parsing the
/// slug**. Telos slugs contain hyphens themselves — `no-store-of-its-own`
/// has four — so `tension/foo-bar--baz` is not reliably decomposable. The
/// slug is a name; this is the data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tension {
    /// The two telos slugs, sorted. Always two.
    pub between: Vec<String>,
}

impl Tension {
    /// Canonical, so one relationship has one subject. `day telos tension b a`
    /// and `day telos tension a b` describe the same thing, and recording it
    /// twice in opposite orders must not produce two subjects.
    pub fn new(a: &str, b: &str) -> Self {
        let mut between = vec![a.to_string(), b.to_string()];
        between.sort();
        Self { between }
    }

    pub fn slug(&self) -> String {
        self.between.join("--")
    }

    pub fn subject(&self) -> String {
        format!("{TENSION_PREFIX}{}", self.slug())
    }

    /// The other telos in this tension, given one of them.
    pub fn other(&self, telos: &str) -> Option<&str> {
        match self.between.as_slice() {
            [a, b] if a == telos => Some(b),
            [a, b] if b == telos => Some(a),
            _ => None,
        }
    }

    pub fn to_claim_text(&self, why: &str) -> String {
        let json = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        format!("{why}\n\n```{FENCE_INFO}\n{json}\n```\n")
    }
}

/// A tension as recorded: who it relates, and why.
#[derive(Debug, Clone)]
pub struct Recorded {
    pub subject: String,
    pub tension: Tension,
    /// The prose reason, with the fenced block stripped.
    pub why: Option<String>,
}

/// Every tension recorded in this log.
pub fn all(client: &KanClient) -> Result<Vec<Recorded>, Error> {
    let mut out = Vec::new();
    for subject in client.subjects()? {
        if !subject.starts_with(TENSION_PREFIX) {
            continue;
        }
        // One `show` per subject, not two: `newest_fenced` would re-read
        // what this already has. Each read is a `kan` subprocess, and day
        // makes one per subject per command, so a duplicate here is a
        // duplicate in every surface that reports tensions.
        let claims = client.show(&subject)?;
        let Some(tension) = claims.iter().rev().find_map(|c| {
            c.text
                .as_deref()
                .and_then(|t| atoms::extract_fenced::<Tension>(t, FENCE_INFO))
                .and_then(Result::ok)
        }) else {
            continue;
        };
        let why = claims
            .iter()
            .rev()
            .find_map(|c| c.text.as_deref().map(prose_only))
            .filter(|s| !s.is_empty());
        out.push(Recorded {
            subject,
            tension,
            why,
        });
    }
    Ok(out)
}

/// What a given telos pulls against, and why.
///
/// Takes the bare slug (`legible-process`), not the subject.
pub fn for_telos(client: &KanClient, slug: &str) -> Result<Vec<Recorded>, Error> {
    Ok(all(client)?
        .into_iter()
        .filter(|r| r.tension.other(slug).is_some())
        .collect())
}

/// One line per tension, for the surfaces that report them.
pub fn render_for(slug: &str, recorded: &[Recorded]) -> Vec<String> {
    recorded
        .iter()
        .filter_map(|r| {
            let other = r.tension.other(slug)?;
            Some(match &r.why {
                Some(why) => format!("in tension with {TELOS_PREFIX}{other}: {why}"),
                None => format!("in tension with {TELOS_PREFIX}{other} ({})", r.subject),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// REQ-2. Argument order must not decide which subject a tension lands
    /// on, or the same relationship gets recorded twice under two names.
    #[test]
    fn the_slug_is_canonical_regardless_of_argument_order() {
        let forward = Tension::new("legible-process", "affordance-not-enforcement");
        let reverse = Tension::new("affordance-not-enforcement", "legible-process");
        assert_eq!(forward, reverse);
        assert_eq!(forward.slug(), reverse.slug());
        assert_eq!(
            forward.slug(),
            "affordance-not-enforcement--legible-process"
        );
    }

    /// REQ-3. The reason this is a block and not a slug convention: telos
    /// slugs contain hyphens, so splitting `a--b` is guesswork the moment a
    /// slug contains `--` or ends in `-`.
    #[test]
    fn participants_come_from_the_block_not_from_splitting_the_slug() {
        let tension = Tension::new("no-store-of-its-own", "composable-process");
        let text = tension.to_claim_text("Richer structure pulls toward day-owned state.");
        let parsed: Tension = atoms::extract_fenced(&text, FENCE_INFO).unwrap().unwrap();
        assert_eq!(parsed, tension);
        assert_eq!(
            parsed.other("no-store-of-its-own"),
            Some("composable-process")
        );
        assert_eq!(
            parsed.other("composable-process"),
            Some("no-store-of-its-own")
        );
        assert_eq!(parsed.other("something-else"), None);
    }

    #[test]
    fn the_reason_is_the_prose_without_the_block() {
        let tension = Tension::new("a", "b");
        let text = tension.to_claim_text("They pull apart.");
        assert_eq!(prose_only(&text), "They pull apart.");
        assert!(!prose_only(&text).contains("day-tension"));
    }

    #[test]
    fn rendering_names_the_other_telos_and_the_reason() {
        let recorded = vec![Recorded {
            subject: "tension/a--b".into(),
            tension: Tension::new("a", "b"),
            why: Some("they pull apart".into()),
        }];
        let lines = render_for("a", &recorded);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("telos/b"), "{lines:?}");
        assert!(lines[0].contains("they pull apart"), "{lines:?}");
        // And from the other side, since the relation is symmetric.
        assert!(render_for("b", &recorded)[0].contains("telos/a"));
        // A telos not party to it gets nothing.
        assert!(render_for("c", &recorded).is_empty());
    }
}
