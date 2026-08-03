//! **A subject is a claim log, and its current state is a fold over that log
//! by role — never "whatever text arrived last."** (day#115)
//!
//! Every vocabulary subject day reads has the same shape: a `telos/<slug>`, an
//! `atom/<slug>`, a `tension/<a>--<b>`, `practice`. "The current declaration"
//! is not a field on any of them; it is a computation over their claims, and
//! which computation is a decision day has been making ad hoc, per call site,
//! four different ways.
//!
//! ## The defect this exists to close
//!
//! A fold keyed on **recency alone** is correct exactly while one *kind* of
//! claim ever lands on a subject, and nothing enforces that. `render_teloi`
//! took the newest claim carrying text as the telos statement; recording an
//! assessment with `kan result telos/<slug>` — which `day assess telos` itself
//! instructs you to do — made the assessment render *as* the telos, on the one
//! surface the model reads every session. The reader and the writer were the
//! same tool, disagreeing with itself.
//!
//! That was fixed for the telos statement. It was **not** fixed for
//! [`crate::tension`]'s reason or [`crate::practice`]'s items, which had the
//! same defect independently and kept it for two more milestones — a `Result`
//! on `tension/<a>--<b>` became the reason those teloi pull apart, and a
//! `Result` on `practice` became injected practice guidance. Four instances of
//! one defect in unrelated modules is the argument for a mechanism rather than
//! a fifth point fix.
//!
//! ## Why per *field* rather than per subject kind
//!
//! Because one subject needs several folds at once. A `telos/<slug>` resolves
//! its statement by role, its witnesses by fenced block, its title from the
//! newest `Subject` claim, and "has this been assessed" from any claim at all.
//! A rule keyed on the subject prefix cannot express that; a small set of
//! combinators applied per field can, and composes when a new subject kind
//! arrives.
//!
//! ## The one invariant
//!
//! **An assessment is never a declaration.** Every function here honours it,
//! and it is the property `an_assessment_is_never_a_declaration` asserts across
//! every vocabulary subject day reads rather than on the one that produced the
//! bug. The rest of a kind's meaning is left to the caller, deliberately: a
//! rule that tried to assign every kind a role everywhere would have to invent
//! meanings day does not have.
//!
//! ## Where this sits against kan
//!
//! Wholly in day, and no kan change is implied. The *mechanism* — give me the
//! newest claim of kind K — is trivial over what `kan show --json` already
//! returns, and kan's ADR-18 claim on "a pure read/fold over the claim graph"
//! is about folds kan itself exposes. What lives here is the **interpretation**:
//! that a `Decision` on a `telos/*` is the statement and a `Result` is an
//! assessment of it. That is process vocabulary, which is day's remit — no new
//! `ClaimBody`, `ClaimKind`, `Anchor` or `RelationKind` variant, so the
//! boundary in `CLAUDE.md` puts it here.
//!
//! Deliberately **not** project-declarable, though `telos/vocabulary-substrate`
//! makes that a fair question. Those declarations — witnesses, block schemas,
//! verdict vocabularies, injection cadence — say what a project's *own*
//! vocabulary contains. A fold rule says what kan's claim kinds *mean*, which
//! is one level down and shared by everyone reading the same log. And the
//! practical argument decides it: a misdeclared fold silently changes what
//! every surface reports, with no error anywhere — which is precisely the
//! failure mode this module exists to remove, so making the fix declarable
//! would make it declarable-wrong.

use crate::atoms::prose_only;
use crate::kan_client::Claim;

/// kan's `ClaimKind` for an assessment, as `kan show --json` renders it.
///
/// Named once rather than spelled at each comparison: four call sites tested
/// `kind == "Result"` by hand, and the one that forgot is how `tension`'s
/// reason came to be an assessment.
pub const ASSESSMENT: &str = "Result";

/// kan's `ClaimKind` for a decision — what `kan decide` and `day telos declare`
/// write, and therefore the preferred carrier of a declaration.
pub const DECISION: &str = "Decision";

/// Whether this claim is an assessment *of* the subject rather than a statement
/// *by* it.
///
/// The single predicate the whole module is built on. A caller that needs to
/// exclude assessments asks this rather than comparing a string, so there is
/// one place to change if kan's rendering ever moves.
pub fn is_assessment(claim: &Claim) -> bool {
    claim.kind == ASSESSMENT
}

/// True when anything on this subject has assessed it.
///
/// Deliberately over *all* claims including retracted-then-superseded ones as
/// `kan show` returns them: "has this ever been assessed" is genuinely an
/// any-claim question, unlike the folds above it.
pub fn is_assessed(claims: &[Claim]) -> bool {
    claims.iter().any(is_assessment)
}

/// The claim's prose, fences stripped, if it has any.
fn prose(claim: &Claim) -> Option<String> {
    claim
        .text
        .as_deref()
        .map(prose_only)
        .filter(|s: &String| !s.is_empty())
}

/// The statement a vocabulary subject currently makes.
///
/// Prefer the declaration ([`DECISION`], which is what `kan decide` and
/// `day telos declare` write), fall back to any claim that is not an
/// assessment, and never let one stand in for the statement.
///
/// **The fallback is load-bearing, not laxity.** Filtering strictly to
/// `Decision` is correct in principle and renders nothing for a hand-written
/// telos recorded with `kan observe` — and a fold that returns "no statement"
/// for a subject that plainly has one is a worse defect than the one being
/// fixed. That is the constraint day#115 names explicitly, and it is the part
/// a tidier exhaustive-match design gets wrong.
pub fn declaration(claims: &[Claim]) -> Option<String> {
    claims
        .iter()
        .rev()
        .filter(|c| c.kind == DECISION)
        .find_map(prose)
        .or_else(|| {
            claims
                .iter()
                .rev()
                .filter(|c| !is_assessment(c))
                .find_map(prose)
        })
}

/// Every claim on a subject whose prose is content *of* that subject, oldest
/// first — the accumulating fold, for subjects where each claim is an item
/// rather than a revision.
///
/// `practice` is the case: each live claim is one projected practice item, so
/// there is nothing to supersede and no "newest wins". It still must not sweep
/// up an assessment, which it did — a `kan result practice "…"` was injected
/// into every session as practice guidance.
pub fn items(claims: &[Claim]) -> impl Iterator<Item = (&Claim, String)> {
    claims
        .iter()
        .filter(|c| !is_assessment(c))
        .filter_map(|c| prose(c).map(|text| (c, text)))
}

/// The subject's declared title, from the newest claim carrying one.
///
/// Only a `Subject` claim carries a title, so this is already role-keyed by
/// kan's shape rather than by a filter here. Wrapped anyway so the four folds
/// read as one vocabulary at the call sites, and so that if kan ever lets
/// another kind carry a title this has a home to be fixed in.
pub fn title(claims: &[Claim]) -> Option<String> {
    claims.iter().rev().find_map(|c| c.title.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claim(kind: &str, text: &str) -> Claim {
        Claim {
            cid: format!("bafy-{kind}-{}", text.len()),
            kind: kind.to_string(),
            text: Some(text.to_string()),
            title: None,
            author: None,
            recorded_at: None,
        }
    }

    /// The fold, asserted on the case that produced it.
    ///
    /// Both surfaces shipped this logic with NO test — twice. Round 1 added the
    /// `render_teloi` fold untested; round 2 added the `assess telos` fold
    /// untested, inside the commit whose stated theme was "four delivery sites
    /// were unasserted". Reverting either to "newest text wins" SURVIVED.
    ///
    /// One shared function means one line to mutate, which is most of why it
    /// was extracted — and now that `tension` and `practice` are callers too,
    /// that one line covers four surfaces instead of two.
    #[test]
    fn a_result_never_stands_in_for_a_declaration() {
        let decl = claim(DECISION, "The telos itself.");
        // Newer than the declaration, and what `day assess telos` instructs you
        // to write — the exact claim that used to hijack the statement slot.
        let assessment = claim(ASSESSMENT, "ASSESSMENT OF telos/x: material evidence ...");

        let folded = declaration(&[decl.clone(), assessment.clone()])
            .expect("a declared telos must yield a statement");
        assert_eq!(
            folded, "The telos itself.",
            "a newer Result must not become the statement — that is the defect \
             `day assess telos` inflicts on itself by instructing `kan result \
             telos/<slug>`"
        );

        // The fallback: a telos recorded with `kan observe` still renders,
        // because a fold returning nothing for a subject that plainly has a
        // statement is worse than the defect being fixed.
        let observed = claim("Observation", "A hand-written telos.");
        assert_eq!(
            declaration(&[observed, assessment.clone()]).as_deref(),
            Some("A hand-written telos."),
            "an Observation-declared telos must still render"
        );

        // And a subject carrying only assessments has no statement to give.
        assert_eq!(
            declaration(&[assessment]),
            None,
            "nothing but Results means no statement, not the newest Result"
        );
    }

    #[test]
    fn items_accumulate_but_skip_assessments() {
        let claims = [
            claim("Observation", "first practice"),
            claim(ASSESSMENT, "an assessment of practice"),
            claim("Decision", "second practice"),
        ];
        let got: Vec<String> = items(&claims).map(|(_, text)| text).collect();
        assert_eq!(got, ["first practice", "second practice"]);
    }

    #[test]
    fn assessment_is_detected_for_the_suffix_even_though_it_never_declares() {
        assert!(is_assessed(&[claim(ASSESSMENT, "x")]));
        assert!(!is_assessed(&[claim(DECISION, "x")]));
    }
}
