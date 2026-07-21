//! Harness hooks — day's actual integration into dev flow.
//!
//! **Advisory, never blocking.** Everything here returns text for a harness
//! to inject as context and exits 0, including on every error path. day
//! deliberately does not port crosslink's blocking hooks: agents act, the
//! record is made legible, drift surfaces as data. A hook that can reject an
//! action is a different (and, in crosslink's experience, worse) tool — see
//! `docs/TELOS.md` and kan's own affordance-not-enforcement house rule.

use crate::atoms::{self, TELOS_PREFIX};
use crate::doctor;
use crate::kan_client::KanClient;

/// Longest telos line day will inline before truncating, so a verbose telos
/// claim can't crowd out the rest of the session context.
const TELOS_EXCERPT: usize = 240;

/// Assembles the session-start context block. Infallible by construction:
/// any failure degrades to a short explanatory note, because a broken
/// process layer must not be able to derail a coding session.
pub fn session_start(client: &KanClient) -> String {
    let mut out = String::from("## day — process layer\n\n");

    if let Err(e) = client.probe() {
        out.push_str(&format!(
            "kan is not reachable, so no telos or atom context is available this session.\n{e}\n",
        ));
        return out;
    }

    let subjects = match client.subjects() {
        Ok(s) => s,
        Err(e) => {
            out.push_str(&format!(
                "kan is installed but its log could not be read here ({e}).\nIf this repo isn't tracked by kan yet, that's expected.\n",
            ));
            return out;
        }
    };

    out.push_str(&render_teloi(client, &subjects));
    out.push('\n');
    out.push_str(&render_atoms(client));
    out.push_str(&render_open(client));
    out.push_str(PRACTICE);
    out
}

/// What is still unresolved. This lives at session *start* rather than
/// session end because only `UserPromptSubmit`, `UserPromptExpansion`, and
/// `SessionStart` add hook stdout to the model's context — every
/// end-of-session event writes to the debug log instead (verified against
/// Claude Code's hook documentation, not assumed). See [`session_end`].
fn render_open(client: &KanClient) -> String {
    match client.issues() {
        Ok(open) if open.is_empty() => String::new(),
        Ok(open) => format!(
            "\nStill open ({}): {}\n",
            open.len(),
            open.iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Err(_) => String::new(),
    }
}

fn render_teloi(client: &KanClient, subjects: &[String]) -> String {
    let mut teloi: Vec<&String> = subjects
        .iter()
        .filter(|s| s.starts_with(TELOS_PREFIX))
        .collect();
    teloi.sort();

    if teloi.is_empty() {
        return "No teloi are recorded for this project yet. A telos is a desired state of \
                the world held up to weak equivalence — declare one with `kan decide \
                \"<statement>\" --subject telos/<slug>` when the purpose of a piece of work \
                is worth making durable.\n"
            .to_string();
    }

    let mut out = format!("Teloi in play ({}):\n", teloi.len());
    for subject in teloi {
        let claims = client.show(subject).unwrap_or_default();
        // The newest narrative claim is often commentary *about* the telos
        // — a recorded tension, an assessment — not the telos itself. The
        // declared title is what the subject is; show it first so a telos
        // stays identifiable no matter what was last said about it.
        let title = claims.iter().rev().find_map(|c| c.title.clone());
        let latest = claims.iter().rev().find_map(|c| c.text.clone());

        let line = match (title, latest) {
            (Some(title), Some(text)) => format!("{title} — {}", excerpt(&text)),
            (Some(title), None) => title,
            (None, Some(text)) => excerpt(&text),
            (None, None) => "(no claims yet)".to_string(),
        };
        out.push_str(&format!("- {subject}: {line}\n"));
    }
    out.push_str(
        "\nThese are in tension with each other by design; when work trades one off against \
         another, record that with `kan decide --subject <telos subject>` rather than \
         resolving it silently.\n",
    );
    out
}

fn render_atoms(client: &KanClient) -> String {
    match doctor::run(client) {
        Ok(report) if report.atoms.is_empty() => String::from(
            "No process atoms are declared yet, so there is no composition to check.\n",
        ),
        Ok(report) => {
            let names: Vec<String> = report.atoms.iter().map(|a| a.subject()).collect();
            let mut out = format!("Process atoms ({}): {}\n", names.len(), names.join(", "));
            if !report.findings.is_empty() {
                out.push_str(&format!(
                    "\nDrift warnings ({}) — advisory, nothing is blocked:\n",
                    report.findings.len()
                ));
                for finding in &report.findings {
                    out.push_str(&format!("- {}\n", finding.message));
                }
            }
            out
        }
        Err(e) => format!("Atom vocabulary could not be read ({e}).\n"),
    }
}

fn excerpt(text: &str) -> String {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if single_line.chars().count() <= TELOS_EXCERPT {
        return single_line;
    }
    let truncated: String = single_line.chars().take(TELOS_EXCERPT).collect();
    format!("{truncated}…")
}

/// The one prescriptive block day injects — process opinions are this
/// tool's whole job, which is exactly why they live here and not in kan
/// (ADR-18). Kept short: a session-start hook competes for the same
/// attention budget as the user's actual request.
const PRACTICE: &str = "\nWorking practice for this session:\n\
    - Before non-trivial work, name which telos it serves. If none fits, that is itself \
      worth recording — an unstated telos is how drift enters.\n\
    - Record durable findings, choices, and outcomes in kan as you go (`observe`, \
      `decide`, `result`), citing the claims they build on. Recording is cheap; \
      reconstructing intent later is not.\n\
    - Assess against material evidence — builds, tests, diffs — not against your own \
      summary of what you did.\n";

/// Which harness events day answers. Kept as an explicit list so an unknown
/// event is a clear error rather than silent empty output.
pub fn dispatch(event: &str, client: &KanClient) -> Result<String, UnknownEvent> {
    match event {
        "session-start" => Ok(session_start(client)),
        "session-end" => Ok(session_end(client)),
        other => Err(UnknownEvent(other.to_string())),
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown hook event `{0}` (known events: session-start, session-end)")]
pub struct UnknownEvent(pub String);

/// An end-of-session report, for a human to run by hand.
///
/// **It is deliberately not registered as a `SessionEnd` hook.** The original
/// design wanted this to prompt the agent before its context was lost, and
/// that is not achievable: only `UserPromptSubmit`, `UserPromptExpansion`,
/// and `SessionStart` add hook stdout to the model's context, and every
/// end-of-session event writes to the debug log instead. The one mechanism
/// that *would* deliver text at that moment is `Stop`'s blocking decision —
/// which is exactly what `telos/affordance-not-enforcement` forbids, so the
/// only route to the goal is one day will not take. That tension, recorded
/// abstractly on the telos subjects, turned out to have a concrete instance.
///
/// The useful half — what is still open — moved to [`session_start`], where
/// injection works and the agent can still act on it. This stays as a
/// command because running it by hand is genuinely useful; it just is not
/// wired to an event that would silently do nothing.
///
/// It reports what is **open**, not what changed during this session: day
/// has no store and therefore no session state, and acquiring one would
/// trade `telos/no-store-of-its-own` for a reminder.
///
/// Infallible and non-blocking, like every hook here.
pub fn session_end(client: &KanClient) -> String {
    let mut out = String::from("## day — before this session ends\n\n");

    if client.probe().is_err() {
        out.push_str("kan is not reachable, so there is nothing to check.\n");
        return out;
    }

    match client.issues() {
        Ok(open) if open.is_empty() => {
            out.push_str("No subjects are left open.\n");
        }
        Ok(open) => {
            out.push_str(&format!("Still open ({}):\n", open.len()));
            for subject in &open {
                out.push_str(&format!("- {subject}\n"));
            }
            out.push('\n');
        }
        Err(e) => {
            out.push_str(&format!("Open subjects could not be read ({e}).\n\n"));
        }
    }

    if let Ok(subjects) = client.subjects() {
        let teloi: Vec<&String> = subjects
            .iter()
            .filter(|s| s.starts_with(TELOS_PREFIX))
            .collect();
        if !teloi.is_empty() {
            out.push_str(&format!(
                "Teloi this work was meant to serve: {}\n\n",
                teloi
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    out.push_str(CLOSING_PRACTICE);
    out
}

/// The one prescriptive block at session end. Deliberately short and
/// specific: a generic "remember to record things" is the kind of
/// boilerplate that gets skipped after the second time.
const CLOSING_PRACTICE: &str = "\
    Before the context holding this session is gone, record what would otherwise be \
    lost:\n\
    - Outcomes of what you actually did (`kan result`), and resolutions for anything \
      finished (`kan resolve`).\n\
    - Findings you would have to re-derive next time (`kan observe`), and choices \
      whose reasoning is not obvious from the diff (`kan decide`).\n\
    - Cite the claims each one builds on. An uncited claim is findable; an uncited \
      chain of reasoning is not reconstructable.\n\
    Nothing here blocks ending the session.\n";

/// Re-exported for the composition check's callers; keeps `atoms` in this
/// module's public surface for hook consumers that want the raw set.
pub use atoms::Atom;
