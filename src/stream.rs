//! One honest inventory of the visible handoff streams.
//!
//! The report is deliberately narrower than a task tracker: it folds live kan
//! claims under `agents/handoff/*` and never infers another stream's branch,
//! worktree, position, or staleness.

use std::collections::BTreeMap;

use crate::kan_client::{Claim, Error, KanClient, PublishedReadDiagnostics};

const PREFIX: &str = "agents/handoff/";
const PREVIEW_CHARS: usize = 120;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    pub name: String,
    pub visible_claims: usize,
    /// Present only when every live claim on the stream carried a timestamp.
    pub last_recorded_at: Option<i64>,
    pub preview: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub streams: Vec<Stream>,
    pub withheld_claims: u64,
    pub unaccounted_subjects: Vec<String>,
    pub published_reads: PublishedReadDiagnostics,
}

impl Report {
    pub fn is_complete(&self) -> bool {
        self.withheld_claims == 0
            && self.unaccounted_subjects.is_empty()
            && self.published_reads.count == Some(0)
            && self
                .published_reads
                .errors
                .as_ref()
                .is_some_and(Vec::is_empty)
    }

    pub fn render(&self) -> String {
        let mut out = format!("{} visible live handoff stream(s)\n", self.streams.len());

        if self.is_complete() {
            out.push_str("inventory: complete under the reported kan view\n");
        } else {
            let mut reasons = Vec::new();
            if self.withheld_claims > 0 {
                reasons.push(format!(
                    "{} claim(s) withheld by the view",
                    self.withheld_claims
                ));
            }
            if !self.unaccounted_subjects.is_empty() {
                reasons.push(format!(
                    "{} subject(s) listed but absent from the bulk read: {}",
                    self.unaccounted_subjects.len(),
                    self.unaccounted_subjects.join(", ")
                ));
            }
            match self.published_reads.count {
                Some(0) => {}
                Some(count) => {
                    reasons.push(format!("kan reports {count} published-claim read error(s)"))
                }
                None => reasons.push(
                    "kan omitted `published_read_error_count`; completeness is unknown".to_string(),
                ),
            }
            match &self.published_reads.errors {
                Some(errors) if errors.is_empty() => {}
                Some(errors) => {
                    reasons.push(format!("published-read diagnostics: {}", errors.join("; ")))
                }
                None => reasons.push(
                    "kan omitted `published_read_errors`; completeness is unknown".to_string(),
                ),
            }
            out.push_str(&format!("inventory: INCOMPLETE — {}\n", reasons.join("; ")));
        }

        if self.streams.is_empty() {
            out.push_str("\nNo visible live handoff streams.\n");
            return out;
        }

        for stream in &self.streams {
            let when = stream
                .last_recorded_at
                .map(|at| format!("last recorded_at {at} µs"))
                .unwrap_or_else(|| "recorded time unknown".to_string());
            out.push_str(&format!(
                "\n  {}  {} visible claim(s)  {}\n",
                stream.name, stream.visible_claims, when
            ));
            if let Some(preview) = &stream.preview {
                out.push_str(&format!("    {preview}\n"));
            }
        }

        out
    }
}

pub fn list(client: &KanClient) -> Result<Report, Error> {
    let claims = client.show_all()?;
    Ok(from_claims(
        claims,
        client.claims_withheld_from_view(),
        client.unaccounted_subjects(),
        client.published_read_diagnostics(),
    ))
}

fn from_claims(
    claims: Vec<(String, Claim)>,
    withheld_claims: u64,
    unaccounted_subjects: Vec<String>,
    published_reads: PublishedReadDiagnostics,
) -> Report {
    let mut grouped: BTreeMap<String, Vec<Claim>> = BTreeMap::new();
    for (subject, claim) in claims {
        let Some(name) = subject.strip_prefix(PREFIX) else {
            continue;
        };
        if name.is_empty() || name.contains('/') {
            continue;
        }
        grouped.entry(name.to_string()).or_default().push(claim);
    }

    let mut streams: Vec<Stream> = grouped
        .into_iter()
        .map(|(name, claims)| {
            let all_timed = claims.iter().all(|claim| claim.recorded_at.is_some());
            let last_recorded_at = all_timed
                .then(|| claims.iter().filter_map(|claim| claim.recorded_at).max())
                .flatten();
            let preview = claims
                .iter()
                .rev()
                .find_map(|claim| claim.text.as_deref())
                .map(preview);
            Stream {
                name,
                visible_claims: claims.len(),
                last_recorded_at,
                preview,
            }
        })
        .collect();

    streams.sort_by(|a, b| {
        b.last_recorded_at
            .cmp(&a.last_recorded_at)
            .then_with(|| a.name.cmp(&b.name))
    });

    Report {
        streams,
        withheld_claims,
        unaccounted_subjects,
        published_reads,
    }
}

fn preview(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = flat.chars();
    let prefix: String = chars.by_ref().take(PREVIEW_CHARS).collect();
    if chars.next().is_some() {
        format!("{prefix}…")
    } else {
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claim(text: &str, recorded_at: Option<i64>) -> Claim {
        Claim {
            cid: format!("cid-{text}"),
            kind: "Observation".into(),
            text: Some(text.into()),
            title: None,
            author: Some("did:key:test".into()),
            recorded_at,
        }
    }

    #[test]
    fn inventory_is_visible_bounded_and_never_infers_position() {
        let report = from_claims(
            vec![
                ("agents/handoff/older".into(), claim("old", Some(1))),
                (
                    "agents/handoff/main".into(),
                    claim(&"long ".repeat(40), Some(3)),
                ),
                ("agents/handoff/main".into(), claim("new handoff", Some(4))),
                ("unrelated".into(), claim("ignore", Some(9))),
            ],
            0,
            vec![],
            PublishedReadDiagnostics {
                count: Some(0),
                errors: Some(vec![]),
            },
        );

        assert_eq!(report.streams.len(), 2);
        assert_eq!(report.streams[0].name, "main");
        assert_eq!(report.streams[0].visible_claims, 2);
        assert_eq!(report.streams[0].last_recorded_at, Some(4));
        assert_eq!(report.streams[0].preview.as_deref(), Some("new handoff"));
        let rendered = report.render();
        assert!(rendered.contains("visible live handoff"));
        for forbidden in ["branch ", "worktree", "position", "stale"] {
            assert!(
                !rendered.contains(forbidden),
                "invented {forbidden:?}: {rendered}"
            );
        }
    }

    #[test]
    fn missing_or_narrowed_diagnostics_make_completeness_explicit() {
        let report = from_claims(
            vec![("agents/handoff/main".into(), claim("x", None))],
            2,
            vec!["agents/handoff/dropped".into()],
            PublishedReadDiagnostics {
                count: None,
                errors: None,
            },
        );
        let rendered = report.render();
        assert!(!report.is_complete());
        assert!(rendered.contains("INCOMPLETE"));
        assert!(rendered.contains("2 claim(s) withheld"));
        assert!(rendered.contains("agents/handoff/dropped"));
        assert!(rendered.contains("published_read_error_count"));
        assert!(rendered.contains("published_read_errors"));
        assert!(rendered.contains("recorded time unknown"));
    }

    #[test]
    fn one_missing_timestamp_prevents_a_false_last_recorded_time() {
        let report = from_claims(
            vec![
                ("agents/handoff/main".into(), claim("timed", Some(10))),
                ("agents/handoff/main".into(), claim("untimed", None)),
            ],
            0,
            vec![],
            PublishedReadDiagnostics {
                count: Some(0),
                errors: Some(vec![]),
            },
        );
        assert_eq!(report.streams[0].last_recorded_at, None);
        assert!(report.render().contains("recorded time unknown"));
    }

    #[test]
    fn previews_are_unicode_safe_and_bounded() {
        let text = "é ".repeat(200);
        let rendered = preview(&text);
        assert!(rendered.ends_with('…'));
        assert_eq!(rendered.chars().count(), PREVIEW_CHARS + 1);
    }
}
