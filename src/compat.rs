//! Which kan versions this day works with (day#94).
//!
//! day and kan are separately released crates with a versioned contract
//! between them, and until now nothing recorded which pairings work: `day
//! doctor` reported `kan: reachable`, and reachable is not compatible. A user
//! running day against a kan that predates a read day depends on found out by
//! getting a wrong answer.
//!
//! **This is not a store.** The supported range is a compiled-in constant —
//! code, exactly like [`crate::atoms::Versioned::SUPPORTED_VERSION`] is for
//! block schemas — so `telos/no-store-of-its-own` is untouched. Nothing about
//! the pairing is written to disk, and day asks kan its version rather than
//! remembering it.
//!
//! **The range is measured, not asserted.** `.github/workflows/kan-compat.yml`
//! runs `tests/kan_conformance.rs` against every released kan and commits the
//! outcome to `tests/fixtures/kan-compat.tsv`; [`OLDEST_SUPPORTED`] is set from
//! that table. Writing the constant first and calling it a requirement would be
//! a claim about kan that day never checked — the same nominal-requirement
//! failure that blocked `.design/declared-blocks.md`'s first implementation.

use std::fmt;

/// A parsed semantic version, with the pre-release tag kept because every kan
/// release so far is one (`0.8.0-beta.1`) — a comparison that ignored it would
/// treat the whole of kan's history as identical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl Version {
    /// Parses `kan 0.8.0-beta.1`, `0.8.0-beta.1`, or `v0.8.0`.
    ///
    /// Tolerant on purpose: this reads another program's output, and the
    /// caller's contract is that an unreadable version is [`Compat::Unknown`]
    /// rather than a mismatch.
    pub fn parse(text: &str) -> Option<Self> {
        let token = text.split_whitespace().find(|t| {
            t.trim_start_matches('v')
                .starts_with(|c: char| c.is_ascii_digit())
        })?;
        let token = token.trim_start_matches('v');

        let (core, pre) = match token.split_once('-') {
            Some((core, pre)) => (core, Some(pre.to_string())),
            None => (token, None),
        };

        let mut parts = core.split('.');
        let mut next = || parts.next()?.parse::<u64>().ok();
        let (major, minor, patch) = (next()?, next()?, next()?);
        // A fourth component means this is not the shape we think it is.
        if parts.next().is_some() {
            return None;
        }
        Some(Version {
            major,
            minor,
            patch,
            pre,
        })
    }

    /// Precedence ignoring the pre-release tag — `0.8.0-beta.1` and `0.8.0`
    /// order together.
    ///
    /// Deliberately **not** `Ord`: semver says a pre-release precedes its
    /// release, and day has no reason to encode that subtlety for a
    /// range check. Keeping it a named method rather than an operator means a
    /// caller cannot reach for `<` and silently get a rule nobody chose.
    fn release_order(&self) -> (u64, u64, u64) {
        (self.major, self.minor, self.patch)
    }

    /// Full semver precedence, **including** the pre-release tag.
    ///
    /// [`Self::release_order`] deliberately orders `0.13.0-beta.1` and `0.13.0`
    /// together, which is right for the FLOOR — a stable release of a version
    /// whose beta day supports is supported — and wrong for the CEILING. Every
    /// kan day has ever measured is a pre-release, so a ceiling compared on
    /// release order alone reports a future stable `0.13.0` as measured when
    /// the newest artifact anyone ran is `0.13.0-beta.1`. day's own wording is
    /// "the newest kan this day was measured against", which is a claim about
    /// an artifact rather than about a version family.
    ///
    /// Found by a cold review, which noted the design chose release-order
    /// comparison on purpose and that the choice contradicts the sentence day
    /// prints. Semver rules, not an approximation: a pre-release precedes its
    /// release, numeric identifiers compare numerically and rank below
    /// alphanumeric ones, and a shorter identifier list precedes a longer one
    /// where all preceding identifiers are equal — so `beta.10` sorts after
    /// `beta.4`, which a string comparison gets backwards.
    fn cmp_precedence(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        self.release_order()
            .cmp(&other.release_order())
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (Some(a), Some(b)) => cmp_pre_release(a, b),
            })
    }
}

/// Semver pre-release precedence between two dot-separated identifier lists.
fn cmp_pre_release(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let mut left = a.split('.');
    let mut right = b.split('.');
    loop {
        let ord = match (left.next(), right.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => match (x.parse::<u64>(), y.parse::<u64>()) {
                (Ok(xn), Ok(yn)) => xn.cmp(&yn),
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => x.cmp(y),
            },
        };
        if ord != Ordering::Equal {
            return ord;
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
    }
}

/// The oldest kan `tests/kan_conformance.rs` passes against, from
/// `tests/fixtures/kan-compat.tsv`.
pub const OLDEST_SUPPORTED: Version = Version {
    major: 0,
    minor: 9,
    patch: 1,
    pre: None,
};

/// The newest kan this day was measured against. Exceeding it is **not** an
/// error — kan's read surface is additive, so a newer kan normally serves an
/// older day fine. It is reported only so a user diagnosing something odd can
/// see that they are past the tested edge.
pub fn newest_measured() -> Version {
    Version {
        major: 0,
        minor: 13,
        patch: 0,
        // The artifact, not the family. A `const` cannot hold an
        // `Option<String>`, and carrying the pre-release is the whole point of
        // this bound — so it is a function.
        pre: Some("beta.1".to_string()),
    }
}

/// The verdict on a kan/day pairing.
///
/// The two skew directions are separate variants because they call for
/// opposite responses, and collapsing them into one "mismatch" would make the
/// benign case as loud as the real one — which is how a warning gets ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Compat {
    /// Within the measured range.
    Supported,
    /// kan predates a read day depends on. The user's fix is to upgrade kan.
    TooOld,
    /// kan is newer than day was measured against. Usually fine; advisory only.
    Newer,
    /// kan's version could not be read. **Not** a mismatch — day says it does
    /// not know, because claiming incompatibility from a failed read would
    /// break day against any kan whose `--version` format shifts.
    Unknown,
}

/// Classifies a kan version against the measured range.
///
/// `None` — kan's version could not be determined — is [`Compat::Unknown`].
pub fn classify(kan: Option<&Version>) -> Compat {
    let Some(kan) = kan else {
        return Compat::Unknown;
    };
    if kan.release_order() < OLDEST_SUPPORTED.release_order() {
        Compat::TooOld
    } else if kan.cmp_precedence(&newest_measured()) == std::cmp::Ordering::Greater {
        Compat::Newer
    } else {
        Compat::Supported
    }
}

/// The line `day doctor` prints. Advisory, never a refusal — day degrades and
/// says so, exactly as an unreadable declaration produces an `Unreadable`
/// rather than an error exit. A day that refused to run against an
/// unrecognised kan would break on every kan release that outpaces it.
pub fn render(kan: Option<&Version>) -> String {
    match classify(kan) {
        Compat::Supported => format!(
            "kan: {} (supported: {}..={})\n",
            kan.expect("Supported implies a version"),
            OLDEST_SUPPORTED,
            newest_measured(),
        ),
        Compat::TooOld => format!(
            "kan: {} — OLDER than this day supports ({}..={}).\n     \
             day reads kan through its public CLI, and a read day depends on may\n     \
             be missing or shaped differently. Upgrade kan.\n",
            kan.expect("TooOld implies a version"),
            OLDEST_SUPPORTED,
            newest_measured(),
        ),
        Compat::Newer => format!(
            "kan: {} — newer than this day was measured against (through {}).\n     \
             Normally fine: kan's read surface is additive. Noted only so a\n     \
             surprise here is legible.\n",
            kan.expect("Newer implies a version"),
            newest_measured(),
        ),
        Compat::Unknown => format!(
            "kan: reachable, version unknown (supported: {}..={}).\n     \
             day could not read `kan --version`. This is not a mismatch — day\n     \
             cannot tell, and says so rather than guessing.\n",
            OLDEST_SUPPORTED,
            newest_measured(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_shape_kan_actually_prints() {
        let v = Version::parse("kan 0.8.0-beta.1").expect("kan's real output");
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 8);
        assert_eq!(v.patch, 0);
        assert_eq!(v.pre.as_deref(), Some("beta.1"));
        assert_eq!(v.to_string(), "0.8.0-beta.1");
    }

    #[test]
    fn parses_the_bare_and_prefixed_forms() {
        assert_eq!(Version::parse("0.7.1"), Version::parse("v0.7.1"));
        assert!(Version::parse("v0.7.1").is_some());
    }

    /// The failure that must not become a false mismatch.
    #[test]
    fn unreadable_output_is_unknown_not_incompatible() {
        assert!(Version::parse("").is_none());
        assert!(Version::parse("kan").is_none());
        assert!(Version::parse("kan version unknown").is_none());
        assert!(Version::parse("kan 0.8").is_none(), "two components");
        assert!(Version::parse("kan 0.8.0.1").is_none(), "four components");
        assert_eq!(classify(None), Compat::Unknown);
    }

    #[test]
    fn the_two_skew_directions_are_distinguished() {
        let old = Version::parse("0.1.0").unwrap();
        let new = Version::parse("9.0.0").unwrap();
        assert_eq!(classify(Some(&old)), Compat::TooOld);
        assert_eq!(classify(Some(&new)), Compat::Newer);
        assert_eq!(classify(Some(&newest_measured())), Compat::Supported);
        assert_eq!(classify(Some(&OLDEST_SUPPORTED)), Compat::Supported);
    }

    /// A pre-release of a supported version is supported. Every kan release so
    /// far is a `-beta.N`, so a rule that excluded them would reject all of kan.
    ///
    /// Pinned to 0.9.0-beta.1 since day#71: the floor moved to 0.9.0 when
    /// `ClaimLog` adopted `show --all`. The version here has to be a
    /// pre-release *of a supported release* or the test asserts the floor
    /// rather than the pre-release rule it is named for.
    #[test]
    fn a_prerelease_of_a_supported_version_is_supported() {
        let beta = Version::parse("kan 0.9.1-beta.1").unwrap();
        assert_eq!(classify(Some(&beta)), Compat::Supported);
    }

    /// The floor's other side, so the test above cannot pass by the range being
    /// wide open.
    #[test]
    fn a_kan_below_the_floor_is_too_old() {
        let below = Version::parse("kan 0.8.0-beta.1").unwrap();
        assert_eq!(classify(Some(&below)), Compat::TooOld);
    }
}
