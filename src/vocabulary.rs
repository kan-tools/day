//! The one writer behind every vocabulary-declaring verb.
//!
//! Teloi, process atoms, and design-doc schemas are all "a subject carrying a
//! declaration, revised by appending a later claim that cites the earlier
//! one". `day telos declare`, `day telos tension`, and `day atom declare`
//! differ only in subject prefix, kan verb, and body text — so they are thin
//! surfaces over this, not three parallel implementations.
//!
//! **There is no `revise` verb anywhere in day.** kan is append-only, so a
//! revision *is* a later claim; naming it separately would name an operation
//! the data model does not have. `declare` cites the newest live claim when
//! one exists, and that is the whole of revision.

use crate::kan_client::{self, KanClient, Write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Kan(#[from] kan_client::Error),
    #[error("no subject `{0}` exists yet — declare it before referring to it")]
    NoSuchSubject(String),
}

/// What kind of act a claim records — all [`Outcome::render`] needs in order
/// to describe it honestly.
///
/// The distinction is not cosmetic. A tension recorded on a telos that
/// already has claims is *not* a revision of that telos, even though a prior
/// claim exists; reporting it as one tells the reader something false about
/// what just happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    /// Declaring the subject, or revising it by declaring again.
    Declare,
    /// Recording a relationship between this subject and another.
    Relate { what: &'static str },
}

pub struct Declaration<'a> {
    pub subject: &'a str,
    /// The kan write verb to invoke (`decide` for teloi, `observe` for atoms).
    pub verb: &'a str,
    pub text: &'a str,
    pub title: Option<&'a str>,
    pub kind: Option<&'a str>,
    /// Other subjects whose newest live claim should also be cited. Used by
    /// `telos tension`, which relates two subjects; each must already exist.
    pub also_cite: &'a [String],
    pub act: Act,
}

#[derive(Debug)]
pub struct Outcome {
    pub cid: String,
    pub subject: String,
    /// True when this claim superseded an earlier one on the same subject.
    /// Only meaningful for [`Act::Declare`].
    pub revised: bool,
    pub cites: Vec<String>,
    pub act: Act,
}

impl Outcome {
    pub fn render(&self) -> String {
        let what = match (self.act, self.revised) {
            (Act::Relate { what }, _) => format!("recorded {what} on"),
            (Act::Declare, true) => "revised".to_string(),
            (Act::Declare, false) => "declared".to_string(),
        };
        let mut out = format!("{what} `{}` ({})\n", self.subject, self.cid);
        if !self.cites.is_empty() {
            out.push_str(&format!("  cites {}\n", self.cites.join(", ")));
        }
        out
    }
}

/// The CID of the newest live claim on `subject`, if it has any.
pub fn newest_claim(client: &KanClient, subject: &str) -> Result<Option<String>, Error> {
    Ok(client.show(subject)?.last().map(|c| c.cid.clone()))
}

pub fn declare(client: &KanClient, d: Declaration<'_>) -> Result<Outcome, Error> {
    let prior = newest_claim(client, d.subject)?;
    let revised = prior.is_some();

    let mut cites: Vec<String> = prior.into_iter().collect();
    for subject in d.also_cite {
        // A referenced subject must already exist: citing nothing would
        // silently produce a claim that asserts a relationship to something
        // that was never declared.
        let cid =
            newest_claim(client, subject)?.ok_or_else(|| Error::NoSuchSubject(subject.clone()))?;
        cites.push(cid);
    }

    let mut write = Write::new(d.verb, d.subject, d.text).cites(&cites);
    if let (Some(title), Some(kind)) = (d.title, d.kind) {
        write = write.declaring(title, kind);
    }
    let cid = client.append(write)?;

    Ok(Outcome {
        cid,
        subject: d.subject.to_string(),
        revised,
        cites,
        act: d.act,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_referenced_subject_is_named() {
        let err = Error::NoSuchSubject("telos/nope".to_string());
        assert!(err.to_string().contains("telos/nope"));
    }

    /// Found by dogfooding: recording a tension against a telos that already
    /// had claims reported "revised", which is false — the telos was not
    /// revised, a relationship was recorded on it.
    #[test]
    fn a_relation_on_an_existing_subject_is_not_reported_as_a_revision() {
        let outcome = Outcome {
            cid: "bafy".into(),
            subject: "telos/a".into(),
            revised: true,
            cites: vec![],
            act: Act::Relate { what: "tension" },
        };
        let rendered = outcome.render();
        assert!(rendered.contains("recorded tension on"), "{rendered}");
        assert!(!rendered.contains("revised"), "{rendered}");
    }
}
