//! Harness hooks — day's actual integration into dev flow.
//!
//! **Advisory, never blocking.** Everything here returns text for a harness
//! to inject as context and exits 0, including on every error path. day
//! deliberately does not port crosslink's blocking hooks: agents act, the
//! record is made legible, drift surfaces as data. A hook that can reject an
//! action is a different (and, in crosslink's experience, worse) tool — see
//! `docs/TELOS.md` and kan's own affordance-not-enforcement house rule.

use std::path::Path;

use crate::atoms::{self, TELOS_PREFIX};
use crate::doctor;
use crate::git::Git;
use crate::kan_client::KanClient;

/// Longest telos line day will inline before truncating, so a verbose telos
/// claim can't crowd out the rest of the session context.
const TELOS_EXCERPT: usize = 240;

/// Assembles the session-start context block. Infallible by construction:
/// any failure degrades to a short explanatory note, because a broken
/// process layer must not be able to derail a coding session.
///
/// It also does the work the status line cannot afford to: it runs position
/// inference (path/tag probes only — never a command, [`crate::position`]
/// holds that by construction) and writes the rendered status line into the
/// `.day/` cache, so the status line can render instantly instead of being
/// cancelled mid-shell-out at Claude Code's 300ms cutoff. This is where the
/// AC-5 guarantee earns real coverage: inference genuinely runs here.
pub fn session_start(client: &KanClient, root: &Path) -> String {
    let mut out = String::from("## day — process layer\n\n");

    if let Err(e) = client.probe() {
        out.push_str(&format!(
            "kan is not reachable, so no telos or atom context is available this session.\n{e}\n",
        ));
        // Still injected: nothing in SAFETY depends on kan, and a session
        // where day's process layer is degraded is not a session where it is
        // safe to stage blindly. Returning early here would drop the
        // guidance exactly where the tooling is already thinnest.
        out.push_str(SAFETY);
        return out;
    }

    let subjects = match client.subjects() {
        Ok(s) => s,
        Err(e) => {
            out.push_str(&format!(
                "kan is installed but its log could not be read here ({e}).\nIf this repo isn't tracked by kan yet, that's expected.\n",
            ));
            out.push_str(SAFETY);
            return out;
        }
    };

    out.push_str(&render_teloi(client, &subjects));
    out.push('\n');
    out.push_str(&render_atoms(client));
    out.push_str(&render_position(client, root));
    out.push_str(&render_open(client));

    // A project's own practice can extend day's blocks or replace them. day
    // is advisory and disposable by design, so an opinion a project cannot
    // refuse would be the wrong kind of opinion — but a replacement is a
    // recorded claim, and day says when one is in effect rather than letting
    // guidance vanish silently.
    let projected = crate::practice::project(client);
    if projected.replaces.practice {
        out.push_str(REPLACED_PRACTICE);
    } else {
        out.push_str(PRACTICE);
    }
    if projected.replaces.safety {
        out.push_str(REPLACED_SAFETY);
    } else {
        out.push_str(SAFETY);
    }
    out.push_str(&projected.render());
    out
}

/// Shown in place of [`PRACTICE`] when a project replaced it. The
/// replacement is visible in the thing being replaced: transparency rather
/// than enforcement.
const REPLACED_PRACTICE: &str =
    "\nWorking practice: this project replaced day's default process practice with its \
     own, below.\n";

/// Shown in place of [`SAFETY`] when a project replaced it.
///
/// Deliberately states what was dropped. The block exists because of a real
/// incident, and its absence is otherwise silent until something
/// irreversible happens — so a reader should be able to see that it is gone.
const REPLACED_SAFETY: &str =
    "\nOperational safety: this project replaced day's default safety guidance (credential \
     handling, explicit staging, unchained commit and push, verified rotation) with its \
     own, below.\n";

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

    let mut lines = Vec::new();
    for subject in teloi {
        let claims = client.show(subject).unwrap_or_default();
        // Since day#32 a tension's reason lives on `tension/<a>--<b>`, not
        // here, so the newest text claim on a telos is the telos again. The
        // declared title still leads, because a subject's name is an rkey
        // and this is what it is called.
        let title = claims.iter().rev().find_map(|c| c.title.clone());
        let latest = claims
            .iter()
            .rev()
            .find_map(|c| c.text.as_deref().map(atoms::prose_only))
            .filter(|s| !s.is_empty());

        match (title, latest) {
            (Some(title), Some(text)) => {
                lines.push(format!("- {subject}: {title} — {}", excerpt(&text)))
            }
            (Some(title), None) => lines.push(format!("- {subject}: {title}")),
            (None, Some(text)) => lines.push(format!("- {subject}: {}", excerpt(&text))),
            // Nothing left to say about it. kan never destroys a subject, so
            // a fully-retracted telos still exists and still appears in
            // `status` — but a telos whose every claim has been retracted is
            // not "in play", and listing it as one would make retraction
            // look like it had not worked.
            (None, None) => continue,
        }
    }

    if lines.is_empty() {
        return "Every recorded telos has been retracted, so none are in play.\n".to_string();
    }

    let mut out = format!("Teloi in play ({}):\n", lines.len());
    for line in lines {
        out.push_str(&line);
        out.push('\n');
    }
    // The tensions themselves, read from their own subjects. Without this,
    // moving the reason off the telos (day#32) would have made it invisible
    // in the one place day is most read.
    if let Ok(tensions) = crate::tension::all(client) {
        let mut lines: Vec<String> = tensions
            .iter()
            .map(|r| match &r.why {
                Some(why) => format!(
                    "- {} vs {}: {}",
                    r.tension.between[0],
                    r.tension.between[1],
                    excerpt(why)
                ),
                None => format!("- {} vs {}", r.tension.between[0], r.tension.between[1]),
            })
            .collect();
        lines.sort();
        if !lines.is_empty() {
            out.push_str(&format!("\nIn tension ({}):\n", lines.len()));
            for line in lines {
                out.push_str(&line);
                out.push('\n');
            }
        }
    }

    out.push_str(
        "\nThese are in tension with each other by design; when work trades one off against \
         another, record that with `day telos tension <a> <b> \"<why>\"` rather than \
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

/// Runs position inference, writes the status-line cache, and returns a short
/// block naming where the work sits for the model.
///
/// Two things happen here that matter beyond the returned text:
/// - **The cache is written.** The status line reads it and never shells out,
///   which is the whole latency story ([`crate::cache`]).
/// - **Inference actually runs.** `AC-5` asserts it executes no command probe
///   on session start; that assertion is only real coverage because this call
///   exists — [`crate::status::compute`] uses `Authorization::Report`, so the
///   guarantee holds by construction rather than by the hook happening not to
///   ask.
///
/// Infallible like the rest of the hook: a failed computation degrades to
/// nothing rather than derailing the session, and a failed cache write leaves
/// the status line showing its documented empty state.
fn render_position(client: &KanClient, root: &Path) -> String {
    let git = Git::new(root);
    let status = match crate::status::compute(client, &git) {
        Ok(s) => s,
        Err(_) => return String::new(),
    };

    // Display-only, latency-only. The write is best-effort: if it fails the
    // status line simply shows nothing until the next session start.
    let _ = crate::cache::write_status_line(root, &status.render_line());

    if status.uncheckable {
        return String::new();
    }

    let mut out = String::from("\nProcess position (inferred from artifacts, not tracked):\n");
    out.push_str(&status.render_line());
    out.push('\n');
    out
}

fn excerpt(text: &str) -> String {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if single_line.chars().count() <= TELOS_EXCERPT {
        return single_line;
    }
    let truncated: String = single_line.chars().take(TELOS_EXCERPT).collect();
    format!("{truncated}…")
}

/// day's **process** opinions — the ones this tool exists to hold, which is
/// exactly why they live here and not in kan (ADR-18). Kept short: a
/// session-start hook competes for the same attention budget as the user's
/// actual request.
///
/// One of *two* prescriptive blocks since day#30. [`SAFETY`] is the other,
/// and they are kept separate deliberately: they are different kinds of
/// guidance, justify their context cost on different grounds, and should be
/// trimmable independently.
const PRACTICE: &str = "\nWorking practice for this session:\n\
    - Before non-trivial work, name which telos it serves. If none fits, that is itself \
      worth recording — an unstated telos is how drift enters.\n\
    - Record durable findings, choices, and outcomes in kan as you go (`observe`, \
      `decide`, `result`), citing the claims they build on. Recording is cheap; \
      reconstructing intent later is not.\n\
    - Assess against material evidence — builds, tests, diffs — not against your own \
      summary of what you did.\n";

/// Operational safety, injected alongside the process practice (day#30).
///
/// Every other opinion day injects is about *process* — name the telos,
/// record as you go, assess against evidence. This one is about actions that
/// cannot be taken back, and it exists because of a real incident: a
/// provisioning script printed a password into the conversation, a blanket
/// `git add -A` chained to a push swept the file holding it into a commit
/// that reached the remote, and the force-push that followed left the object
/// still served by SHA. Four well-known footguns, none of them surfaced
/// anywhere in that session's injected context.
///
/// **Unconditional, not gated on repo signals.** Detecting an
/// "infra-shaped" repo would save context budget and would fail silently in
/// exactly the repo that needed it. These four rules are near-universally
/// correct and cost less than being wrong once.
///
/// It earns its share of the attention budget on a different basis than
/// [`PRACTICE`] does: a missed telos is recoverable from the record, while a
/// pushed credential means rotation and history rewrite. Kept to four lines
/// regardless.
///
/// **Still advisory.** This is guidance in context at the moment the agent
/// would otherwise reach for `git add -A` — not a gate, not a pre-commit
/// hook. `telos/affordance-not-enforcement` governs it exactly as it governs
/// everything else day injects.
const SAFETY: &str = "\nOperational safety for this session:\n\
    - Never print a credential into conversation. If a step produces one, write it \
      somewhere the user controls and report only that it was stored. A secret in a \
      transcript is a secret that now has to be rotated.\n\
    - Stage explicitly. `git add -A`/`-u` in a repo holding secrets, config, or \
      generated output will eventually commit something nobody read. Name paths.\n\
    - Do not chain commit and push. They are separate decisions, and chaining them \
      removes the last checkpoint where a bad commit is still cheap.\n\
    - Rotation and redaction are not done until verified. Confirm the old credential \
      fails and the new one works; after a history rewrite, confirm the object is gone \
      from the remote — a force-push alone does not remove it.\n";

/// Which harness events day answers. Kept as an explicit list so an unknown
/// event is a clear error rather than silent empty output.
pub fn dispatch(event: &str, client: &KanClient, root: &Path) -> Result<String, UnknownEvent> {
    match event {
        "session-start" => Ok(session_start(client, root)),
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

#[cfg(test)]
mod safety_tests {
    use super::*;

    /// day#30. The four rules are the whole point; a paraphrase that drops
    /// one is a regression the prose would hide.
    #[test]
    fn the_safety_block_covers_all_four_footguns_from_the_incident() {
        for rule in [
            "Never print a credential",
            "Stage explicitly",
            "Do not chain commit and push",
            "not done until verified",
        ] {
            assert!(SAFETY.contains(rule), "SAFETY should cover {rule:?}");
        }
        // The specific failure that made this worth filing: a force-push
        // looked like it had worked, and the object was still served by SHA.
        assert!(
            SAFETY.contains("force-push alone does not remove it"),
            "the verification rule should name the force-push case that motivated it"
        );
    }

    /// Advisory, like everything else day injects. If this block ever starts
    /// telling an agent it is forbidden to act, it has become the kind of
    /// tool `telos/affordance-not-enforcement` exists to prevent.
    #[test]
    fn the_safety_block_advises_and_never_forbids() {
        let lower = SAFETY.to_lowercase();
        for forbidden in ["you must not", "is forbidden", "do not proceed", "blocked"] {
            assert!(
                !lower.contains(forbidden),
                "SAFETY reads as enforcement ({forbidden:?}); day injects practice, not gates"
            );
        }
    }

    /// Nothing in the safety guidance depends on kan, and a session where
    /// day's process layer is degraded is not one where it is safe to stage
    /// blindly. Dropping it on the error paths would remove the guidance
    /// exactly where the tooling is thinnest.
    #[test]
    fn safety_is_injected_even_when_kan_is_unreachable() {
        let dir = tempfile::tempdir().unwrap();
        let client = KanClient::with_bin(dir.path(), "definitely-not-a-real-kan-binary");
        let out = session_start(&client, dir.path());
        assert!(out.contains("kan is not reachable"), "{out}");
        assert!(
            out.contains("Operational safety"),
            "safety guidance should survive kan being unavailable: {out}"
        );
    }

    /// Budget discipline, asserted rather than intended: this competes with
    /// the user's actual request for attention. Four rules, and a ceiling so
    /// the block cannot quietly grow into a policy document.
    #[test]
    fn the_safety_block_stays_terse() {
        let bullets = SAFETY.lines().filter(|l| l.starts_with("- ")).count();
        assert_eq!(bullets, 4, "four rules; adding a fifth is a deliberate act");
        assert!(
            SAFETY.len() < 1200,
            "SAFETY is {} bytes and competes with the user's request for attention",
            SAFETY.len()
        );
    }
}
