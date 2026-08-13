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

/// Longest single projected item before truncation, when a project declares no
/// preference. A project controls this list, and session-start competes with
/// the user's actual request for attention.
///
/// The default, not the rule: the effective length is
/// [`InjectionSchema::max_practice_item_length`].
///
/// **This was a `const` while the count cap beside it was made declarable, and
/// the omission mattered more.** At 300, sixteen of day's own twenty-three
/// items arrived cut mid-sentence — including the rule telling a session which
/// review findings re-open the work, severed before its fourth kind, which is
/// therefore never injected at all. Nine of the eleven rules migrated out of
/// `CLAUDE.md` were among them, so the migration read as done and was
/// two-thirds real.
///
/// [`InjectionSchema::max_practice_item_length`]: crate::blocks::InjectionSchema::max_practice_item_length
pub const DEFAULT_ITEM_EXCERPT: usize = 300;

/// Most items day will project when a project declares no preference.
///
/// The default, not the rule: the effective cap is
/// [`InjectionSchema::max_practice_items`], read from `schema/injection`.
///
/// [`InjectionSchema::max_practice_items`]: crate::blocks::InjectionSchema::max_practice_items
pub const DEFAULT_MAX_ITEMS: usize = 12;

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
        Ok(crate::kan_client::Read::Absent) => return Projection::default(),
        Ok(crate::kan_client::Read::Present(claims)) if claims.is_empty() => {
            return Projection::default()
        }
        Ok(crate::kan_client::Read::Present(claims)) => claims,
        Ok(crate::kan_client::Read::Withheld { count }) => {
            return Projection {
                notes: vec![format!(
                    "`{PRACTICE_SUBJECT}` is unreadable: {count} claim(s) are withheld from this view"
                )],
                ..Projection::default()
            };
        }
        Ok(crate::kan_client::Read::Indeterminate { log_wide }) => {
            return Projection {
                notes: vec![format!(
                    "the log withholds {log_wide} claim(s) without subject attribution, so `{PRACTICE_SUBJECT}` may be absent or omitted"
                )],
                ..Projection::default()
            };
        }
        // **A read that failed is not an absence, and this arm used to say it
        // was.** The comment here read "No subject, or an unreadable one.
        // Absence is not an error" — true when the only `Err` meant kan was
        // broken, false the moment `kan_client::Error` gained
        // `PartiallyWithheld` and `AbsentUnderNarrowedTrust` for day#120. From
        // then on a withheld practice subject rendered as "no practice items",
        // and the whole projection vanished from `hook session-start` with no
        // note — in the same run that printed a ⚠ block enumerating three other
        // unreadable declarations, so the completeness warning was itself
        // incomplete.
        //
        // Neither the compiler nor the swallow scan could say so: every
        // consumer already handled `Result`, and this shape is a `match` arm,
        // which the scan did not read until it was widened for exactly this.
        //
        // Still fails open — a hook must never fail — but it says why.
        Err(e) => {
            return Projection {
                notes: vec![format!(
                    "`{PRACTICE_SUBJECT}` could not be read, so no practice items are \
                     projected — this is day unable to answer, not a project without \
                     practice: {e}"
                )],
                ..Projection::default()
            };
        }
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

    // day#115: an assessment of the practice is not a practice.
    //
    // This is the accumulating fold — each live claim is one item, so nothing
    // supersedes anything — but it swept up every kind, and `kan result
    // practice "…"` was injected into every session as guidance. Same rule as
    // the telos statement, different shape of fold, which is why it lives in
    // [`crate::fold`] and not in whichever module noticed it last.
    //
    // Routed through `fold::items` rather than re-implemented here. The first
    // fix inlined `fold::is_assessment` and left `items` a `pub fn` whose only
    // callers were its own tests — CLAUDE.md's "`pub` suppresses dead-code
    // detection… either dead or a requirement about to go nominal", introduced
    // in the commit that quotes it. `items` also owns the prose extraction and
    // the empty check, so inlining duplicated three things, not one.
    //
    // Assessments are filtered *before* the identity check, so a foreign
    // assessment is no longer counted as "not projected: not locally signed".
    // That count exists to explain an identity skip; an assessment was never
    // skipped for that reason, and saying so was the less accurate of the two.
    // Both caps, from one read. A read that fails must not silently become the
    // default — `src/probe.rs`'s rule, and the one this file's own
    // `InjectionSchema::load(…).unwrap_or(DEFAULT_CADENCE)` was cited for in
    // CLAUDE.md — so the failure is carried as a note and the projection says
    // it is working from settings it could not confirm.
    let (cap, excerpt_limit) = match crate::blocks::InjectionSchema::load(client) {
        Ok(schema) => (schema.max_practice_items, schema.max_practice_item_length),
        Err(e) => {
            projection.notes.push(format!(
                "`schema/injection` could not be read ({e}), so the item cap falls back \
                 to day's default of {DEFAULT_MAX_ITEMS} and the item length to \
                 {DEFAULT_ITEM_EXCERPT}. Declared values may be in effect that this \
                 projection did not apply."
            ));
            (DEFAULT_MAX_ITEMS, DEFAULT_ITEM_EXCERPT)
        }
    };
    // How much each item lost to the length cap, carried ALONGSIDE the item so
    // the count cap can discard both together. Summing during the loop counted
    // items the reader never received: with three short items delivered and
    // five long ones withheld by the count cap, the note said "5 projected
    // item(s) were cut" when none of the three delivered was, and pointed at a
    // setting that would reveal nothing. The comment below used to claim the
    // property this vector is what actually provides.
    let mut drops: Vec<usize> = Vec::new();

    for (claim, text) in crate::fold::items(&claims) {
        if !accepts(&local, claim) {
            foreign += 1;
            continue;
        }
        let text = text.trim().to_string();
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
        let (item, dropped) = excerpt(&text, excerpt_limit);
        projection.items.push(item);
        drops.push(dropped);
    }

    if foreign > 0 {
        projection.notes.push(format!(
            "{foreign} claim(s) on `{PRACTICE_SUBJECT}` were not projected: they are not \
             signed by this workspace's identity. Injected text is scoped to the local \
             signer deliberately."
        ));
    }

    if projection.items.len() > cap {
        let dropped = projection.items.len() - cap;
        projection.items.truncate(cap);
        drops.truncate(cap);
        projection.notes.push(format!(
            "{dropped} further item(s) not shown: a projection is capped at {cap} \
             so it cannot crowd out the request it is meant to inform."
        ));
    }

    // **Counted over the survivors, which is the whole point.** The two caps are
    // independent and both can fire, and an item the count cap withheld was not
    // "cut at N characters" — it was not delivered at all, and raising the
    // length would not reveal it. Reporting the pre-cap population told a
    // reader to change the setting that could not help them, which is a report
    // asserting something day had not established.
    let truncated_items = drops.iter().filter(|d| **d > 0).count();
    let truncated_chars: usize = drops.iter().sum();
    if truncated_items > 0 {
        projection.notes.push(format!(
            "{truncated_items} projected item(s) were cut at {excerpt_limit} characters \
             ({truncated_chars} character(s) not shown). Raise \
             `max_practice_item_length` on `schema/injection` to see them whole."
        ));
    }

    projection
}

/// Returns the excerpt and, when it truncated, how many characters it dropped.
///
/// **The count is returned rather than discarded**, because `Projection::notes`
/// says a projection that drops silently "is indistinguishable from one that
/// found nothing — the failure shape this repo has met three times", and this
/// function was the one path in the module that dropped without saying so. The
/// count cap reported what it withheld; the length cap did not, so
/// "23 items, nothing withheld" was true of one and false of the other.
fn excerpt(text: &str, limit: usize) -> (String, usize) {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let total = single_line.chars().count();
    if total <= limit {
        return (single_line, 0);
    }
    let truncated: String = single_line.chars().take(limit).collect();
    (format!("{truncated}…"), total - limit)
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
            recorded_at: None,
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
        let (out, dropped) = excerpt(&long, DEFAULT_ITEM_EXCERPT);
        assert!(
            out.chars().count() <= DEFAULT_ITEM_EXCERPT + 1,
            "{}",
            out.len()
        );
        assert!(out.ends_with('…'));
        // **The count, not just the ellipsis.** The ellipsis was already there
        // and told nobody how much was gone; a projection that drops silently
        // is indistinguishable from one that found nothing.
        assert_eq!(
            dropped,
            long.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .count()
                - DEFAULT_ITEM_EXCERPT
        );
        // A short item drops nothing and says so.
        assert_eq!(excerpt("short", DEFAULT_ITEM_EXCERPT).1, 0);
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
