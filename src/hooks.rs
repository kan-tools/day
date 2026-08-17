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
///
/// fallback: hook-degrades-when-kan-cannot-read
pub fn session_start(client: &KanClient, root: &Path) -> String {
    session_start_with_source(client, root, None)
}

/// [`session_start`] with the harness's declared SessionStart source.
///
/// Only `compact` changes the framing today. The underlying reads stay the
/// same: this is a prompt to reconstruct from the durable record after context
/// loss, not a second inference system or a claim that day knows what the
/// compacted session was doing (day#93).
pub fn session_start_with_source(client: &KanClient, root: &Path, source: Option<&str>) -> String {
    let mut out = String::from("## day — process layer\n\n");
    if source == Some("compact") {
        out.push_str(POST_COMPACTION);
    }

    if let Err(e) = client.probe() {
        out.push_str(&format!(
            "kan is not reachable, so no telos or atom context is available this session.\n{e}\n",
        ));
        // **The bar is told too.** These early returns are the paths a broken
        // kan actually takes, and for one release they were the paths on
        // which no footer was written at all — so the status line kept
        // showing the previous session's confident position while `day
        // status` correctly reported the log could not be read. The
        // could-not-read state existed and was unreachable: its only caller
        // sat downstream of both of these returns.
        write_unreadable_footer(client, root);
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
            write_unreadable_footer(client, root);
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

const POST_COMPACTION: &str = "Post-compaction reorientation: earlier working context was \
summarized. Before continuing, re-read the active design or handoff, verify its factual \
coordinates against the current record, and treat the summary as a pointer rather than \
evidence. The process context below is freshly read for this repository.\n\n";

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
        Err(error) => format!(
            "\nStill open: ⚠ could not read kan issues, so this list is unavailable ({error})\n"
        ),
    }
}

fn render_teloi(client: &KanClient, subjects: &[String]) -> String {
    let mut teloi: Vec<&String> = subjects
        .iter()
        .filter(|s| s.starts_with(TELOS_PREFIX))
        .collect();
    teloi.sort();

    if teloi.is_empty() {
        // **"None here" and "none this view admits" are different facts, and
        // this said the first when the second was true** (day#120, found by a
        // cold review of the fix meant to close it).
        //
        // The per-subject guards in `KanClient::show` cannot help here: this
        // reader ENUMERATES `subjects()`, and kan omits a fully-withheld
        // subject from `status --json` too, so the loop never produces it and
        // never calls `show`. In a plain clone of a repo publishing `.claims/`,
        // with no `--trust` flag anywhere, this printed "No teloi are recorded
        // for this project yet" over six withheld claims — on the channel that
        // reaches the model, which is the one `telos/honest-reads` names, to
        // the population `telos/v1.0` names.
        //
        // `render_teloi`'s own history is this defect once already: a failed
        // read became an empty list here and an unreadable telos vanished from
        // the list and from its count. It came back by a different route.
        let withheld = client.claims_withheld_from_view();
        if withheld > 0 {
            return format!(
                "No teloi are visible in this view, and {withheld} claim(s) in this log are \
                 withheld from it — so day cannot tell whether none are recorded or none \
                 are admitted by this trust base. Use kan directly with a trust base that \
                 admits the relevant authors; day has no trust-selection flag.\n"
            );
        }
        return "No teloi are recorded for this project yet. A telos is a desired state of \
                the world held up to weak equivalence — declare one with `kan decide \
                \"<statement>\" --subject telos/<slug>` when the purpose of a piece of work \
                is worth making durable.\n"
            .to_string();
    }

    let mut lines = Vec::new();
    let mut unreadable = Vec::new();
    for subject in teloi {
        // A read that FAILED used to become an empty claim list here, which fell
        // through to the `(None, None) => continue` arm below — so an unreadable
        // telos silently vanished from the list *and* from its count, in the one
        // place day is read by the model. Same defect as day#81, on the surface
        // where it costs most. The caveat is attached to the item it undermines
        // rather than put in a footer, because a footer is easy not to connect
        // to the list above it.
        let claims = match client.show(subject) {
            Ok(crate::kan_client::Read::Present(claims)) => claims,
            Ok(crate::kan_client::Read::Absent) => Vec::new(),
            Ok(crate::kan_client::Read::Withheld { count }) => {
                unreadable.push(subject.clone());
                lines.push(format!(
                    "- {subject}: ⚠ unreadable ({count} claim(s) withheld from this view)"
                ));
                continue;
            }
            Ok(crate::kan_client::Read::Indeterminate { log_wide }) => {
                unreadable.push(subject.clone());
                lines.push(format!(
                    "- {subject}: ⚠ may be absent or omitted; {log_wide} claim(s) are withheld without subject attribution"
                ));
                continue;
            }
            Err(e) => {
                unreadable.push(subject.clone());
                lines.push(format!(
                    "- {subject}: ⚠ this day could not read this telos ({e}) — it is \
                     in play but its declaration and witnesses are unknown here"
                ));
                continue;
            }
        };
        // A SUBJECT IS A CLAIM LOG, AND ITS CURRENT STATE IS A FOLD OVER IT BY
        // ROLE — not "whatever text arrived last".
        //
        // This used to take the newest claim carrying text, justified by: since
        // day#32 a tension's reason lives on `tension/<a>--<b>`, so the newest
        // text on a telos is the telos again. That held only while exactly one
        // KIND of claim ever landed here, and nothing enforces that. Recording
        // an assessment with `kan result telos/<slug>` — which `day assess
        // telos` itself instructs — made the assessment render AS the telos, on
        // the one surface the model reads every session. Two of day's own
        // surfaces in contradiction: one told you to write a claim the other
        // could not read.
        //
        // Each kind now has a role, so a kind nobody anticipated has NO role
        // rather than accidentally becoming the statement:
        //   Decision   -> the statement (newest live one wins; `kan decide` is
        //                 what `day telos declare` emits)
        //   Subject    -> the title
        //   Result     -> an assessment; surfaced as a suffix, never as the text
        //   other      -> context, not the statement
        let title = crate::fold::title(&claims);
        // Prefer the declaration; fall back to any claim that is not an
        // assessment. Filtering strictly to `Decision` was too strict: `kan
        // decide` is the documented way to declare a telos and what `day telos
        // declare` emits, but a hand-written one may be an `Observation`, and
        // rendering nothing for it would trade this defect for a worse one.
        // What must never happen is a `Result` becoming the statement, which is
        // the whole of F12.
        //
        // fallback: telos-without-a-declaration
        let statement = crate::fold::declaration(&claims);

        // An assessment enriches the line instead of replacing it — which is
        // what recording one was supposed to do.
        let assessed = crate::fold::is_assessed(&claims);
        let suffix = if assessed { "  [assessed]" } else { "" };

        match (title, statement) {
            (Some(title), Some(text)) => {
                lines.push(format!("- {subject}: {title} — {}{suffix}", excerpt(&text)))
            }
            (Some(title), None) => lines.push(format!("- {subject}: {title}{suffix}")),
            (None, Some(text)) => lines.push(format!("- {subject}: {}{suffix}", excerpt(&text))),
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

    // The count is over what is *listed*, including the unreadable ones, so it
    // cannot disagree with the list. A count that silently excluded them is what
    // made "Teloi in play (1)" true of a project with two.
    let mut out = format!("Teloi in play ({}):\n", lines.len());
    if !unreadable.is_empty() {
        out.push_str(&format!(
            "  ({} of these could not be read by this day, so treat this list as \
             partial — `day doctor` for detail)\n",
            unreadable.len()
        ));
    }
    let withheld = client.claims_withheld_from_view();
    if withheld > 0 {
        out.push_str(&format!(
            "  (⚠ {withheld} additional claim(s) are withheld without subject attribution; visible teloi are not a complete inventory)\n"
        ));
    }
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
        // Empty **and** nothing to report. The guard used to be
        // `atoms.is_empty()` alone, which matched first and discarded the
        // findings — so a project whose only atom day could not read was told
        // "no process atoms are declared yet", which is not a degraded answer
        // but a false one.
        // The same guard needs the same third case as `render_teloi`: empty,
        // nothing to report, AND nothing withheld. `atoms::load` enumerates
        // too, so a withheld `atom/*` subject is not merely unreadable — it is
        // absent from the list day iterates, and no per-subject check can see
        // it. Saying "not an error" here was the more emphatic of the two false
        // reassurances a cold review reproduced.
        Ok(report)
            if report.atoms.is_empty()
                && report.findings.is_empty()
                && client.claims_withheld_from_view() > 0 =>
        {
            format!(
                "No process atoms are visible in this view, and {} claim(s) in this log are \
                 withheld from it — so this is day unable to see the vocabulary, not a \
                 project without one. Use kan directly with an admitting trust base; day \
                 has no trust-selection flag.\n",
                client.claims_withheld_from_view()
            )
        }
        Ok(report) if report.atoms.is_empty() && report.findings.is_empty() => String::from(
            "No process atoms are declared yet, so there is no composition to check.\n",
        ),
        Ok(report) => {
            let names: Vec<String> = report.atoms.iter().map(|a| a.subject()).collect();
            let mut out = if names.is_empty() {
                // Declared but unreadable: say that, rather than either of the
                // two available lies ("none declared" or a count of zero
                // presented as the whole vocabulary).
                String::from(
                    "Process atoms: none could be read by this day, though the log \
                     declares some — the vocabulary below is not a composition check.\n",
                )
            } else {
                format!("Process atoms ({}): {}\n", names.len(), names.join(", "))
            };
            if !report.findings.is_empty() {
                out.push_str(&format!(
                    "\nDrift warnings ({}) — advisory, nothing is blocked:\n",
                    report.findings.len()
                ));
                for finding in &report.findings {
                    out.push_str(&format!("- {}\n", finding.message));
                }
            }
            if report.withheld > 0 {
                out.push_str(&format!(
                    "⚠ {} additional claim(s) are withheld without subject attribution; visible atoms are not a complete inventory.\n",
                    report.withheld
                ));
            }
            out
        }
        Err(e) => format!("Atom vocabulary could not be read ({e}).\n"),
    }
}

/// The fingerprint both cache writers use, in one place so they cannot drift.
///
/// F4: session-start wrote `git.position_fingerprint()` while user-prompt
/// compared `{git}:{log}`, so the two never matched and the first prompt of
/// every session paid for a recompute whose answer it already had. Two writers
/// of one value in different formats is the same shape as a guarantee wired at
/// a call site — the fix is one function, not two matching edits.
///
/// `None` when git cannot be read. An unreadable kan log deliberately yields an
/// *unmatchable* value rather than a matching one, so the next prompt recomputes
/// and reports the failure instead of going quiet — treating a log day could not
/// read as "nothing moved" would be the carve-out abuse and an honest-reads
/// violation at once.
fn position_cache_fingerprint(git: &Git, client: &KanClient) -> Option<String> {
    let git_fp = git.position_fingerprint().ok()?;
    Some(match client.log_fingerprint() {
        Ok(log_fp) => format!("{git_fp}:{log_fp}"),
        Err(_) => format!("{git_fp}:unreadable"),
    })
}

/// Assembles everything the footer renders besides the position: repo,
/// branch, sync and checkout from git; the active role and the withheld
/// count from kan (`.design/harness-footer.md` REQ-4..7).
///
/// **This is the only place those reads happen** (REQ-10): the footer is
/// rendered here in the hook and cached; `day status-line` reads the cache
/// and invokes nothing, which is what keeps it inside Claude Code's 300 ms
/// cancellation budget. Every read degrades to an omitted segment rather
/// than an error — a hook must not fail over a decoration.
///
/// fallback: footer-reads-degrade
fn footer_surround(client: &KanClient, git: &Git) -> crate::footer::Surround {
    crate::footer::Surround {
        context: footer_context(git),
        // **The cost, stated rather than left implicit**, because a cold
        // review objected to exactly this and the objection is good: the
        // segment renders identically for no declared roles, a kan too old
        // for the verb, a kan that errored, and output day could not parse —
        // which is "day could not look" spelled like "there is nothing here",
        // the shape this scan exists to catch. What makes it survivable is
        // that the segment is a decoration, while the caveats qualifying the
        // *report* — the narrowing and the partial-read count — are pinned
        // separately and cannot be omitted at all, for width or for anything
        // else. The objection is recorded on `harness-footer` against RQ-2
        // rather than resolved here.
        //
        // kan-read-may-degrade: RQ-2 decided that for the identity segment
        // specifically, absent and error both omit — recorded as a decision
        // before this was built, so not this fix round's to reverse quietly.
        role: client.active_role(),
        withheld: client.claims_withheld_from_view(),
    }
}

/// The git half of the footer's context: repo name, branch, sync, checkout.
/// Public so the acceptance tests can drive it against fixture repositories
/// — AC-14's worktree assertion in particular is about *this* assembly, not
/// about the renderer.
pub fn footer_context(git: &Git) -> crate::footer::Context {
    let canonical = |p: std::path::PathBuf| std::fs::canonicalize(&p).unwrap_or(p);

    let mut context = crate::footer::Context::default();
    if let Ok(sync) = git.sync_state() {
        // Real porcelain-v2 output always carries the branch header, even
        // detached or unborn. Output without it is not a sync state day
        // read — it is a git that printed nothing — and rendering it would
        // claim a clean tree from an empty read.
        if sync.branch.is_some() {
            context.branch = sync.branch.clone();
            context.sync = Some(crate::footer::Sync {
                dirty: sync.dirty,
                ahead_behind: sync.ahead_behind,
            });
        }
    }

    // The main checkout's root: the parent of the common git dir — not
    // `--show-toplevel`, which names the *current* checkout and therefore a
    // worktree's own directory, precisely in the case the footer exists to
    // make visible (RQ-7).
    let main_root = git
        .common_dir()
        .ok()
        .map(canonical)
        .and_then(|c| c.parent().map(std::path::Path::to_path_buf));

    // Repo name: the remote when there is one, the main checkout's directory
    // otherwise (REQ-12); an unrecognised remote URL falls back rather than
    // guessing (REQ-14).
    //
    // fallback: unrecognised-remote
    context.repo = git
        .remote_url()
        .ok()
        .flatten()
        .and_then(|url| crate::footer::repo_from_remote(&url))
        .or_else(|| {
            main_root
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().into_owned())
        });

    if let (Some(top), Some(main_root)) = (git.toplevel().ok().map(canonical), main_root) {
        context.checkout = Some(if top == main_root {
            crate::footer::Checkout::Main
        } else if let Ok(rel) = top.strip_prefix(&main_root) {
            crate::footer::Checkout::UnderMain(rel.display().to_string())
        } else {
            crate::footer::Checkout::Elsewhere(top.display().to_string())
        });
    }

    context
}

/// Renders every footer variant for the current state and writes them to the
/// cache, for `day status-line` to pick from.
///
/// **All variants, not one**, because the two things the choice depends on —
/// the terminal width and the user's `DAY_FOOTER` — are known where the
/// status line runs and not here. See `crate::footer` for why that keeps the
/// `.day/` carve-out intact.
fn write_footer(client: &KanClient, git: &Git, root: &Path, status: &crate::status::Status) {
    let surround = footer_surround(client, git);
    // Display-only, latency-only. Best-effort: if it fails the status line
    // simply shows nothing until the next session start.
    let _ = crate::cache::write_status_line(
        root,
        &crate::footer::render(
            status,
            &surround,
            crate::footer::Style::Emoji,
            crate::footer::FALLBACK_BUDGET,
        ),
        &crate::footer::render_variants(status, &surround),
    );
}

/// The could-not-read-the-log-at-all footer, for the paths where there is no
/// `Status` to render because kan itself failed.
///
/// Takes no `Git` of its own on purpose: git may be perfectly readable when
/// kan is not, and the context line is exactly the part still worth showing
/// (REQ-7 — a partial view is information).
fn write_unreadable_footer(client: &KanClient, root: &Path) {
    let git = Git::new(root);
    let surround = footer_surround(client, &git);
    let _ = crate::cache::write_status_line(
        root,
        &crate::footer::render_unreadable(
            &surround,
            crate::footer::Style::Emoji,
            crate::footer::FALLBACK_BUDGET,
        ),
        &crate::footer::render_unreadable_variants(&surround),
    );
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
///
/// fallback: hook-degrades-when-kan-cannot-read
fn render_position(client: &KanClient, root: &Path) -> String {
    let git = Git::new(root);
    let status = match crate::status::compute(client, &git) {
        Ok(s) => s,
        Err(error) => {
            // The bar still gets a truthful rendering: leaving the cache
            // holding an earlier session's position would display
            // confidently from a read that just failed.
            write_unreadable_footer(client, root);
            return format!("day could not refresh its reading: {error}\n");
        }
    };

    write_footer(client, &git, root, &status);

    // And what the per-prompt hook needs, so it can re-display without repeating
    // this read. Recorded here because this is the one place that already pays
    // for the expensive computation and has time to. A failed write costs the
    // next prompt a recompute, which is correct-but-slower — never wrong.
    if let Some(fingerprint) = position_cache_fingerprint(&git, client) {
        // The cadence comes off `status`, which resolved it with the other
        // declarations and reported it if unreadable. Loading it here instead
        // meant an unreadable `schema/injection` silently became the default —
        // the same defect as day#81, on a value nobody would notice was wrong.
        let cadence = status.cadence;
        let _ = crate::cache::write_standing(
            root,
            &crate::cache::Standing {
                fingerprint,
                unreadable: status.unreadable.len(),
                cadence,
                standing_notice: status.standing_notice(),
            },
        );
    }

    // The caveat comes before the early return, not after it. A schema whose
    // every probe is a kind this build cannot read makes position `uncheckable`
    // — and that is exactly the state where staying silent is worst, because
    // "position could not be inferred" and "this day could not read your witness
    // schema" call for completely different responses and only the second names
    // a cause. This is day#60's state, and the human channel reported it while
    // the model channel did not, which is the asymmetry that mattered least in
    // theory and most in practice.
    let mut out = String::new();
    if !status.unreadable.is_empty() {
        out.push_str(&format!(
            "\n⚠ {} declaration(s) could not be read by this day, so the process \
             context above is partial:\n",
            status.unreadable.len()
        ));
        for item in &status.unreadable {
            out.push_str(&format!("- {}\n", item.message));
        }
    }

    if status.uncheckable {
        if !out.is_empty() {
            out.push_str(
                "Position cannot be inferred at all as a result — treat any statement \
                 about where the work sits as unknown rather than as \"nothing to do\".\n",
            );
        }
        return out;
    }

    out.push_str("\nProcess position (inferred from artifacts, not tracked):\n");
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
    dispatch_with_input(event, client, root, None)
}

/// Dispatches a hook with the harness JSON delivered on stdin.
///
/// Invalid, empty, or unrelated input deliberately means "no known source"
/// and preserves the historical output. Hook metadata is advisory context;
/// losing it must not make a session fail.
pub fn dispatch_with_input(
    event: &str,
    client: &KanClient,
    root: &Path,
    input: Option<&str>,
) -> Result<String, UnknownEvent> {
    match event {
        "session-start" => {
            let source = input
                .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
                .and_then(|value| {
                    value
                        .get("source")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                });
            Ok(session_start_with_source(client, root, source.as_deref()))
        }
        "session-notice" => Ok(session_notice(client, root)),
        "user-prompt" => Ok(user_prompt(client, root)),
        "session-end" => Ok(session_end(client)),
        other => Err(UnknownEvent(other.to_string())),
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown hook event `{0}` (known events: session-start, session-notice, user-prompt, session-end)")]
pub struct UnknownEvent(pub String);

/// The transition/off-sequence notice, as a `systemMessage`-only JSON payload
/// for the human — or empty output when there is nothing to mark.
///
/// **A separate SessionStart hook from [`session_start`], on purpose.** That
/// hook prints markdown that becomes the model's context, and its output is
/// left byte-for-byte unchanged. This one emits only `{"systemMessage": …}`,
/// which Claude Code shows to the *human* (verified live this session on a
/// hook) and adds nothing to the model's context. Splitting them means adding
/// a human notice cannot regress the context injection — the two audiences are
/// served by two hooks rather than one hook switched to a riskier output
/// shape.
///
/// **Advisory, like everything here.** A `systemMessage` is a notice, never a
/// decision; there is no blocking construct, so `tests/plugin.rs` stays green.
/// Emits nothing when there is no transition and no off-sequence finding, so a
/// quiet session sees no notice at all. Infallible: any failure degrades to
/// empty output.
///
/// The status line already shows a transition persistently (`day ⤳ …`); this
/// is the once-per-session *event* marker on top of that, and the status line
/// is the visibility floor if `systemMessage` ever renders differently on
/// SessionStart than it did where it was verified.
///
/// fallback: hook-degrades-when-kan-cannot-read
pub fn session_notice(client: &KanClient, root: &Path) -> String {
    let mut notices: Vec<String> = Vec::new();

    // Compat first, because a pairing problem explains anything reported under
    // it. Kept ahead of `status::compute` for a second reason: that call is the
    // one that degrades to nothing when kan cannot be read, and a version
    // mismatch day *can* state should not be suppressed by a status day cannot.
    //
    // fallback: notice-degrades-when-kan-cannot-read
    if let Some(pairing) = compat_notice(client) {
        notices.push(pairing);
    }

    let git = Git::new(root);
    if let Ok(status) = crate::status::compute(client, &git) {
        notices.extend(status.notice());
    }

    if notices.is_empty() {
        return String::new();
    }
    serde_json::json!({ "systemMessage": notices.join("\n\n") }).to_string()
}

/// The kan-pairing line, when the pairing is worth saying something about.
///
/// **Only [`Compat::TooOld`] and [`Compat::Newer`].** [`Compat::Supported`] is
/// the quiet case by design, and [`Compat::Unknown`] is deliberately skipped
/// here even though `day doctor` reports it: `render` phrases that case as
/// "kan: reachable, version unknown", and reachability is precisely what this
/// call site has *not* established. `doctor` earns that wording by calling
/// `client.probe()` first; this hook does not probe, so emitting it would be
/// day asserting something it never checked — `telos/honest-reads`, in the
/// direction that is easy to miss because the text sounds harmless.
///
/// The absent-binary case is not day's to report at all: a `day` that cannot be
/// found cannot say so. The portable install skill covers recovery, which is why that
/// one script is exempt from "every SessionStart command is a `day hook`".
///
/// Wording comes from [`compat::render`] rather than being restated, so the
/// range and its explanation live in exactly one place.
fn compat_notice(client: &KanClient) -> Option<String> {
    let kan = client.version();
    match crate::compat::classify(kan.as_ref()) {
        crate::compat::Compat::TooOld | crate::compat::Compat::Newer => {
            Some(crate::compat::render(kan.as_ref()).trim_end().to_string())
        }
        crate::compat::Compat::Supported | crate::compat::Compat::Unknown => None,
    }
}

/// day's **mid-session channel to the model**, on `UserPromptSubmit`.
///
/// Designed in v0.6 (`docs/ROADMAP.md`, *Situated injection*), allowlisted in
/// `tests/plugin.rs`, and never wired until now. The roadmap already answers the
/// obvious objection: day#30 found that a *general standing rule* injected always
/// becomes background, and argues that a **specific, state-triggered** notice is
/// a different thing — that treating them as the same is over-learning from one
/// failure.
///
/// Two triggers, deliberately separate mechanisms:
///
/// 1. **A state transition** — an event. Emitted whenever position has moved
///    past the last recorded assessment, or a step looks skipped.
/// 2. **A standing condition** — that day's report is *partial* because a
///    declaration could not be read. Not an event; it stays true until someone
///    fixes it, so it is re-displayed on a bounded cadence rather than every
///    turn. This is the one closest to day#30's failure mode, so it carries only
///    conditions affecting the correctness of what day already said — never
///    practice advice.
///
/// **What this deliberately does not do is recompute the expensive half.** A
/// `UserPromptSubmit` hook runs on every prompt, and `day status` measures 2.76 s
/// on day's own log of which 1.99 s is 41 `kan` invocations. So position here is
/// read from what session-start already computed; the live, git-gated
/// recomputation waits on day#71's bulk read. Saying "as of session start" is the
/// honest version of a claim day cannot currently make live.
///
/// Emits nothing when there is nothing to say, and cannot fail: every error path
/// degrades to empty output, because a hook that breaks a prompt would be the
/// gate `telos/affordance-not-enforcement` forbids.
///
/// fallback: hook-degrades-when-kan-cannot-read
pub fn user_prompt(client: &KanClient, root: &Path) -> String {
    let git = Git::new(root);

    // The cheap gate, and the whole point of this function's shape. The
    // expensive path costs 3.0s on day's own log; this costs 0.03s. An earlier
    // version of this hook simply called `status::compute` on every prompt while
    // its own doc comment, `hooks/hooks.json`, and the design all said it did
    // not — a 3-second-per-turn regression that three artifacts described as its
    // opposite, which is the failure this whole milestone exists to stop day
    // committing.
    // day#111: git AND kan, because position depends on both.
    //
    // The git half alone was the whole gate, and `Git::position_fingerprint`
    // reads no kan — so recording a claim moved the position and left the
    // fingerprint byte-identical, this function took its early return, and the
    // status line kept serving the previous render. On this repo that is the
    // dominant workflow: a session that records a design pass, a verdict and an
    // assessment changes position repeatedly while touching no tracked file.
    // The bar was reliable; it updated on exactly one of the two ways position
    // moves.
    //
    // Costs one kan invocation on a quiet turn (0.06s measured, against the
    // ~1.4s recompute this gate exists to avoid) and **none** on a turn that
    // goes on to recompute, because `ClaimLog` memoizes the bulk read and the
    // two share it.
    let fingerprint = position_cache_fingerprint(&git, client);
    let cached = crate::cache::standing(root);

    // Unchanged git state AND a cached reading: nothing a `path`/`tag` probe
    // sees has moved, so the expensive read cannot have changed the answer.
    // Re-display the standing condition on the cadence and read nothing.
    //
    // A missing cache means *recompute*, never "all clear" — that is what keeps
    // deleting `.day/` a cost in redundant work rather than a change in answer.
    if let (Some(fp), Some(standing)) = (&fingerprint, &cached) {
        if *fp == standing.fingerprint {
            // Both standing conditions re-display on the cadence, from the
            // cache, without recomputing. The done-but-unrecorded findings were
            // missing here, and unifying the fingerprint writers (F4) turned
            // that from latent into live: the first prompt began hitting the
            // cache, so the model stopped being told about an unrecorded
            // release entirely.
            if crate::cache::cadence_allows(root, standing.cadence) {
                let mut parts = Vec::new();
                if let Some(notice) = &standing.standing_notice {
                    parts.push(notice.clone());
                }
                if standing.unreadable > 0 {
                    parts.push(format!(
                        "day: {} declaration(s) could not be read at session start, so \
                         day's telos and atom lists are partial — `day doctor` for detail.",
                        standing.unreadable
                    ));
                }
                if !parts.is_empty() {
                    return format!("{}\n", parts.join("\n"));
                }
            }
            return String::new();
        }
    }

    // Git state moved (or there is nothing cached): pay for the real answer
    // once, and re-cache it so the next prompts are cheap again.
    let status = match crate::status::compute(client, &git) {
        Ok(status) => status,
        Err(error) => {
            // **The bar is told here too, and this is the half that matters
            // most.** The first fix for the stale-confident-bar defect covered
            // `session_start`'s two early returns and left this one, which is
            // the worse omission: session-start runs once, this runs every
            // prompt. A kan that breaks *after* session start therefore left
            // the bar showing a confident position for the rest of the
            // session, while `day status` correctly reported the log could not
            // be read — the day#60 asymmetry, on the surface a human watches.
            //
            // Found by a cold review that drove all four kan-failure paths;
            // the test written for the first fix drove one of them.
            write_unreadable_footer(client, root);
            return format!(
                "day could not refresh its process reading: {error}. `day doctor` for detail.\n"
            );
        }
    };
    let cadence = status.cadence;

    // day#97: re-render the line from the reading just paid for.
    //
    // This path recomputed the position and cached the *standing* while leaving
    // `statusline` holding whatever session-start wrote, so the bar showed
    // session-start state for the whole session — observed four hours and three
    // atoms behind on a repo that had advanced through three atoms, three
    // assessments and four commits. `day status` and the line disagreed, and the
    // line is the surface a human actually watches.
    //
    // Reaching this point means the fingerprint moved — so the write happens
    // exactly when the answer can have changed and never on the cheap path
    // above, which stays at zero kan invocations (the property REQ-4 pins,
    // rather than a duration, which would measure the machine). The footer's
    // own reads (one `kan identity role list`, a few git reads) are paid only
    // here, on the path already paying for a full recompute.
    write_footer(client, &git, root, &status);

    if let Some(fp) = fingerprint {
        let _ = crate::cache::write_standing(
            root,
            &crate::cache::Standing {
                fingerprint: fp,
                unreadable: status.unreadable.len(),
                cadence,
                standing_notice: status.standing_notice(),
            },
        );
    }

    let mut parts = Vec::new();

    // The event half. Always emitted when true — an event that fired is not
    // something to ration, and reaching here means the state genuinely moved.
    if let Some(notice) = status.notice_for_model() {
        parts.push(notice);
    }

    // THE CADENCE IS CONSULTED EXACTLY ONCE PER PROMPT, and every standing
    // condition shares that one decision.
    //
    // `cadence_allows` ADVANCES A COUNTER on each call, so consulting it per
    // condition does not ration them independently — it makes them compete. Two
    // calls advanced the counter by two per prompt, and whichever ran last
    // always landed on the threshold and reset it, so the other could never
    // reach it. The done-but-unrecorded notice reached the model *zero* times in
    // 22 prompts on any repo that also had an unreadable declaration — which is
    // the degraded repo, exactly where both notices matter most.
    //
    // Verified in the one-condition case at the time, which is the two-mode trap
    // `CLAUDE.md` names: the mode this repo happened to be in worked, and the
    // other one was dead.
    let repeat_standing = crate::cache::cadence_allows(root, cadence);

    // Both are conditions rather than events: true until somebody acts, so a git
    // change is not a reason to repeat them.
    if repeat_standing {
        if let Some(standing) = status.standing_notice() {
            parts.push(standing);
        }
        if !status.unreadable.is_empty() {
            parts.push(format!(
                "day: {} declaration(s) could not be read, so day's telos and atom lists \
                 are partial — `day doctor` for detail.",
                status.unreadable.len()
            ));
        }
    }

    if parts.is_empty() {
        return String::new();
    }
    // Plain text on stdout: `UserPromptSubmit` adds it to the model's context.
    // No `hookSpecificOutput`, no decision field — there is deliberately no
    // shape here that could deny a prompt.
    format!("{}\n", parts.join("\n"))
}

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

    // A failed read used to drop this whole section silently — `if let Ok(..)`
    // with no `else`, twenty lines below the `client.issues()` read in this same
    // function that reports its failure properly. One read here was honest and
    // the next was not, and the scan named for exactly this rule passed on both
    // because it only knew the `.unwrap_or*`/`.ok()` shapes.
    match client.subjects() {
        Ok(subjects) => {
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
        Err(e) => {
            out.push_str(&format!(
                "Teloi could not be read ({e}), so this handoff does not say what \
                 the work was meant to serve.\n\n"
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
