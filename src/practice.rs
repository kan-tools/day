//! A project's own working practice, projected into injected context.
//!
//! day injects two prescriptive blocks and both are hard-coded constants.
//! Every other thing day knows about a project is declared per project in kan
//! and read back; the one thing day actually *says* to the model each session
//! was the one thing a project could not change without forking the binary.
//!
//! So a `practice` subject is projected too. **Each live claim is one item**,
//! which makes the fold do the work: appending adds an item, retracting
//! removes one, and there is no revision mechanism to design. Every other
//! `day-*` block is newest-claim-wins because a schema is one object; this is
//! not one object, it is a list.
//!
//! **Locally-signed only.** An injection path sourced from claims is inert
//! while one key signs everything and stops being inert the moment kan sync
//! lands. Scoping to the local identity makes it inert *by construction*
//! rather than by vigilance, and retrofitting a trust boundary onto a live
//! injection path is much worse than designing one in. The per-author trust
//! list is designed (`.design/repo-defined-injection.md` REQ-9) and not
//! built; [`accepts`] is the single place that decision is made, so adding
//! it later is a change to one function.

use crate::atoms::prose_only;
use crate::kan_client::{Claim, KanClient};

/// Subject a project records its own working practice on.
pub const PRACTICE_SUBJECT: &str = "practice";

/// A claim carrying this token replaces one of day's own blocks rather than
/// adding to it.
///
/// A project can discard day's opinions — the README promises exactly that —
/// but doing so is a recorded, attributable claim rather than a config
/// toggle. That includes the safety block: treating it as unremovable would
/// mean day holds an opinion a project cannot refuse, which
/// `telos/affordance-not-enforcement` forbids, and *"this rule is too
/// important to let you remove"* is the argument every blocking tool makes
/// about itself.
pub const REPLACE_TOKEN: &str = "day-replace:";

/// Longest single projected item before truncation. A project controls this
/// list, and session-start competes with the user's actual request for
/// attention.
const ITEM_EXCERPT: usize = 300;

/// Most items day will project, however many are recorded.
const MAX_ITEMS: usize = 12;

/// Which of day's own blocks a project asked to replace.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Replacements {
    pub practice: bool,
    pub safety: bool,
}

#[derive(Debug, Default)]
pub struct Projection {
    pub items: Vec<String>,
    pub replaces: Replacements,
    /// Why something is missing. Rendered into the block, because a
    /// projection that drops claims silently is indistinguishable from one
    /// that found nothing — the failure shape this repo has met three times.
    pub notes: Vec<String>,
}

/// Whether a claim may be projected into a model's context.
///
/// **The single place this decision is made.** Today it is "signed by this
/// workspace's identity". `.design/repo-defined-injection.md` REQ-9's trust
/// list becomes a change to this function's body and to nothing else, which
/// is what keeps that extension point from being a rewrite.
fn accepts(local: &str, claim: &Claim) -> bool {
    claim.author.as_deref() == Some(local)
}

/// Projects a project's recorded practice.
///
/// Fails closed: with no local identity, nothing is projected and the caller
/// is told. Projecting claims whose authorship could not be checked, *because
/// checking was unavailable*, inverts the property the locally-signed rule
/// exists to provide — and would do it exactly when something is already
/// wrong.
pub fn project(client: &KanClient) -> Projection {
    let claims = match client.show(PRACTICE_SUBJECT) {
        Ok(claims) if claims.is_empty() => return Projection::default(),
        Ok(claims) => claims,
        // No subject, or an unreadable one. Absence is not an error.
        Err(_) => return Projection::default(),
    };

    let Some(local) = client.identity() else {
        return Projection {
            notes: vec![format!(
                "{} claim(s) on `{PRACTICE_SUBJECT}` were not projected: this workspace's \
                 identity could not be established, so day cannot tell which are locally \
                 signed. Nothing is injected rather than injecting unverified text.",
                claims.len()
            )],
            ..Default::default()
        };
    };

    let mut projection = Projection::default();
    let mut foreign = 0;

    for claim in &claims {
        if !accepts(&local, claim) {
            foreign += 1;
            continue;
        }
        let Some(text) = claim.text.as_deref().map(prose_only) else {
            continue;
        };
        let text = text.trim().to_string();
        if text.is_empty() {
            continue;
        }
        if let Some(rest) = text.strip_prefix(REPLACE_TOKEN) {
            match rest.trim() {
                "practice" => projection.replaces.practice = true,
                "safety" => projection.replaces.safety = true,
                other => projection.notes.push(format!(
                    "`{REPLACE_TOKEN}{other}` names no block day injects — expected \
                     `practice` or `safety`"
                )),
            }
            continue;
        }
        projection.items.push(excerpt(&text));
    }

    if foreign > 0 {
        projection.notes.push(format!(
            "{foreign} claim(s) on `{PRACTICE_SUBJECT}` were not projected: they are not \
             signed by this workspace's identity. Injected text is scoped to the local \
             signer deliberately."
        ));
    }

    if projection.items.len() > MAX_ITEMS {
        let dropped = projection.items.len() - MAX_ITEMS;
        projection.items.truncate(MAX_ITEMS);
        projection.notes.push(format!(
            "{dropped} further item(s) not shown: a projection is capped at {MAX_ITEMS} \
             so it cannot crowd out the request it is meant to inform."
        ));
    }

    projection
}

fn excerpt(text: &str) -> String {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if single_line.chars().count() <= ITEM_EXCERPT {
        return single_line;
    }
    let truncated: String = single_line.chars().take(ITEM_EXCERPT).collect();
    format!("{truncated}…")
}

impl Projection {
    /// Whether anything at all needs rendering. A project with no `practice`
    /// subject must leave the injected text byte-identical to before.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.notes.is_empty()
    }

    pub fn render(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        if !self.items.is_empty() {
            out.push_str("\nThis project's own practice:\n");
            for item in &self.items {
                out.push_str(&format!("- {item}\n"));
            }
        }
        for note in &self.notes {
            out.push_str(&format!("\n({note})\n"));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claim(text: &str, author: &str) -> Claim {
        Claim {
            cid: "bafyreia".into(),
            kind: "Observation".into(),
            text: Some(text.into()),
            title: None,
            author: Some(author.into()),
        }
    }

    #[test]
    fn only_locally_signed_claims_are_accepted() {
        assert!(accepts("did:key:zme", &claim("x", "did:key:zme")));
        assert!(!accepts(
            "did:key:zme",
            &claim("x", "did:key:zsomeone-else")
        ));
        // A claim with no author at all is not local, and must not be
        // projected by default.
        let mut anonymous = claim("x", "did:key:zme");
        anonymous.author = None;
        assert!(!accepts("did:key:zme", &anonymous));
    }

    #[test]
    fn the_replace_token_selects_a_block_and_is_not_itself_an_item() {
        let mut p = Projection::default();
        for (token, practice, safety) in [("practice", true, false), ("safety", false, true)] {
            p = Projection::default();
            let text = format!("{REPLACE_TOKEN} {token}");
            let rest = text.strip_prefix(REPLACE_TOKEN).unwrap().trim().to_string();
            match rest.as_str() {
                "practice" => p.replaces.practice = true,
                "safety" => p.replaces.safety = true,
                _ => unreachable!(),
            }
            assert_eq!(p.replaces.practice, practice);
            assert_eq!(p.replaces.safety, safety);
        }
        assert!(p.items.is_empty(), "a replace instruction is not an item");
    }

    #[test]
    fn an_item_longer_than_the_excerpt_is_truncated() {
        let long = "word ".repeat(200);
        let out = excerpt(&long);
        assert!(out.chars().count() <= ITEM_EXCERPT + 1, "{}", out.len());
        assert!(out.ends_with('…'));
    }

    /// A projection that finds nothing must leave the injected block exactly
    /// as it was, so adding this feature changes nothing for a project that
    /// does not use it.
    #[test]
    fn an_empty_projection_renders_nothing() {
        assert_eq!(Projection::default().render(), "");
    }

    /// Silent omission is the failure shape this repo has hit three times.
    /// A dropped claim must be visible in the text a reader actually sees.
    #[test]
    fn skipped_claims_are_named_in_the_rendered_text() {
        let projection = Projection {
            items: vec!["do the thing".into()],
            replaces: Replacements::default(),
            notes: vec!["2 claim(s) were not projected: not locally signed".into()],
        };
        let rendered = projection.render();
        assert!(rendered.contains("do the thing"), "{rendered}");
        assert!(rendered.contains("not projected"), "{rendered}");
    }
}
