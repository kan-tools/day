//! The harness footer — the status-line rendering of [`crate::status::Status`],
//! plus the repo/identity context around it (day#179, `.design/harness-footer.md`).
//!
//! **Display only.** Everything here turns already-computed state into a
//! string; nothing reads kan, git, or the render cache, and nothing decides
//! — `tests/footer.rs` scans this file to keep that true. The expensive
//! reads happen in `day hook session-start` (REQ-10), which calls [`render`]
//! and caches the result; `day status-line` prints it back without invoking
//! anything (AC-12).
//!
//! Two renderings carry the same nine states (REQ-1, REQ-9): four position
//! forms, four message kinds in the tray, and the could-not-read report.
//! [`Style::Emoji`] is chosen unless a signal day can actually *know* says
//! otherwise (REQ-17); [`Style::Plain`] is pure ASCII, because the negative
//! signals that force it (a `C` locale, `TERM=dumb`) also rule out `·` and
//! `…`, not just emoji.
//!
//! The layout is flat rather than indented, and there is no horizontal rule:
//! emoji are double-width in some terminals and not others, so an indented
//! mockup cannot align under its first line (RQ-4). Width variance is removed
//! as a failure mode rather than detected — day can know whether emoji are
//! *encodable*, never whether they are *renderable* (REQ-18), which is why the
//! override exists.

use crate::status::Status;

/// The environment override for which rendering is used (REQ-16). `plain` or
/// `emoji`; any other value falls through to detection. Session beats
/// detection in both directions (AC-19) — an override that only forced one
/// way would be half a control.
pub const STYLE_ENV: &str = "DAY_FOOTER";

/// The width bound for a checkout path outside the main root (REQ-15).
/// Measured need: a real worktree abbreviated to `…/day-behaviour-0009e02f9dcb/tree`
/// — without a bound the segment eats the line.
pub const CHECKOUT_BOUND: usize = 28;

/// How many tray items render before the tray says it truncated (REQ-8).
/// The tray truncates visibly or not at all (RQ-5).
pub const TRAY_MAX: usize = 3;

/// Which rendering the footer uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Emoji,
    Plain,
}

/// The environment signals detection may consult — a snapshot, so tests can
/// drive every combination without mutating process env (which races across
/// parallel tests).
///
/// Detection **claims no positive** (REQ-17): a non-UTF-8 locale, a dumb or
/// absent TERM, and NO_COLOR are knowable negatives; font coverage is
/// invisible to the environment, so there is deliberately no allowlist of
/// terminal programs — a positive list rots as new terminals appear and
/// reports "supported" by having found nothing to object to.
#[derive(Debug, Default, Clone)]
pub struct EnvSignals {
    pub style_override: Option<String>,
    pub lc_all: Option<String>,
    pub lc_ctype: Option<String>,
    pub lang: Option<String>,
    pub term: Option<String>,
    pub no_color: Option<String>,
}

impl EnvSignals {
    pub fn from_env() -> Self {
        let var = |k: &str| std::env::var(k).ok();
        Self {
            style_override: var(STYLE_ENV),
            lc_all: var("LC_ALL"),
            lc_ctype: var("LC_CTYPE"),
            lang: var("LANG"),
            term: var("TERM"),
            no_color: var("NO_COLOR"),
        }
    }

    /// The effective locale, with POSIX precedence: `LC_ALL` beats `LC_CTYPE`
    /// beats `LANG`.
    fn locale(&self) -> Option<&str> {
        self.lc_all
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(self.lc_ctype.as_deref().filter(|s| !s.is_empty()))
            .or(self.lang.as_deref().filter(|s| !s.is_empty()))
    }
}

impl Style {
    /// Resolves which rendering to use: the explicit override first, then
    /// detection (REQ-16). The declared layer is specified and deliberately
    /// not built in v1 (REQ-19) — nothing here reads a `schema/*` subject.
    pub fn resolve(signals: &EnvSignals) -> Style {
        match signals.style_override.as_deref() {
            Some("plain") => return Style::Plain,
            Some("emoji") => return Style::Emoji,
            _ => {}
        }
        Style::detect(signals)
    }

    /// Emoji unless a knowable negative says otherwise (REQ-17).
    fn detect(signals: &EnvSignals) -> Style {
        // A locale that is not UTF-8 cannot encode the emoji rendering at all.
        // An *unset* locale is not a negative — modern terminal environments
        // routinely leave it to the terminal — but a set, non-UTF-8 one is.
        if let Some(locale) = signals.locale() {
            let l = locale.to_ascii_lowercase();
            if !(l.contains("utf-8") || l.contains("utf8")) {
                return Style::Plain;
            }
        }
        // TERM unset or dumb: something that declared itself unable.
        match signals.term.as_deref() {
            None | Some("") | Some("dumb") => return Style::Plain,
            _ => {}
        }
        // NO_COLOR (set and non-empty, per the convention it comes from) is a
        // person asking for undecorated output.
        if signals.no_color.as_deref().is_some_and(|v| !v.is_empty()) {
            return Style::Plain;
        }
        Style::Emoji
    }
}

/// Where this checkout sits relative to the main one (REQ-15). Three forms,
/// all of which render distinguishably (AC-16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Checkout {
    /// This *is* the main checkout.
    Main,
    /// A worktree under the main root; carries the path relative to it.
    UnderMain(String),
    /// A worktree anywhere else; carries the full path, abbreviated at render.
    Elsewhere(String),
}

/// Working-tree sync state, from one `git status` read (AC-4) — one source,
/// so the counts cannot disagree with the dirtiness.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sync {
    pub dirty: bool,
    /// `None` when there is no upstream to count against — which is a real
    /// state, not zero-zero.
    pub ahead_behind: Option<(u64, u64)>,
}

/// The repo context line's inputs. Each field a segment; a field day could
/// not fill is `None` and its segment is omitted (REQ-7) — a missing segment
/// honestly says "nothing to report".
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// `org/name` from the remote, or the main checkout's directory name
    /// (REQ-12, REQ-14).
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub sync: Option<Sync>,
    pub checkout: Option<Checkout>,
}

/// `org/name` from the forms a remote URL actually takes (REQ-14), or `None`
/// for anything unrecognised — the caller falls back to the directory name,
/// because a wrong repo name is worse than a plain one.
///
/// fallback: unrecognised-remote
pub fn repo_from_remote(url: &str) -> Option<String> {
    let url = url.trim();
    if url.is_empty() {
        return None;
    }
    // scp-like ssh: git@host:org/name(.git)
    let path = if let Some((_, path)) = url.split_once(':').filter(|(userhost, path)| {
        userhost.contains('@') && !path.starts_with("//") && !userhost.contains('/')
    }) {
        path
    } else {
        // Everything else must carry a scheme day recognises; a local
        // filesystem path or any other shape yields `None` here, so the
        // caller falls back rather than guessing (REQ-14).
        //
        // fallback: unrecognised-remote
        let rest = url
            .strip_prefix("https://")
            .or_else(|| url.strip_prefix("http://"))
            .or_else(|| url.strip_prefix("ssh://"))
            .or_else(|| url.strip_prefix("git://"))?;
        // host/org/name(.git) — drop the host, keep the path.
        let (_host, path) = rest.split_once('/')?;
        path
    };
    let path = path
        .trim_matches('/')
        .strip_suffix(".git")
        .unwrap_or_else(|| path.trim_matches('/'));
    let mut segments = path.rsplit('/');
    let name = segments.next().filter(|s| !s.is_empty())?;
    let org = segments.next().filter(|s| !s.is_empty())?;
    Some(format!("{org}/{name}"))
}

/// Everything the footer renders besides the position itself. Assembled in
/// the session-start hook (REQ-10), consumed here.
#[derive(Debug, Clone, Default)]
pub struct Surround {
    pub context: Context,
    /// The declared kan role whose DID is the active one (REQ-6). `None`
    /// omits the segment (REQ-7) — no declared roles, no identity, a kan
    /// without the verb, and a failed read all say nothing by not appearing.
    pub role: Option<String>,
    /// Claims withheld from this view, log-wide. **A narrowing is never
    /// omitted** (REQ-7, AC-9): whenever this is non-zero the footer says so,
    /// and there is no error path that blanks it — the count is a plain
    /// integer the client already holds.
    pub withheld: u64,
}

/// The glyph set for one style. One table, so the two renderings cannot
/// cover different state sets — every state named here exists in both.
struct Glyphs {
    /// Replaces the per-line `day` (REQ-3). The literal `day` appears at most
    /// once across the whole footer (AC-3).
    anchor: &'static str,
    seg: &'static str,
    repo: &'static str,
    branch: &'static str,
    clean: &'static str,
    dirty: &'static str,
    /// Formatted with the count after it.
    ahead: &'static str,
    behind: &'static str,
    main_checkout: &'static str,
    worktree: &'static str,
    role: &'static str,
    withheld: &'static str,
    transition: &'static str,
    boundary: &'static str,
    unrecorded: &'static str,
    off_sequence: &'static str,
    unreadable: &'static str,
    ellipsis: &'static str,
}

const EMOJI: Glyphs = Glyphs {
    anchor: "☀️",
    seg: " · ",
    repo: "📁 ",
    branch: "🌿 ",
    clean: "✔",
    dirty: "✏️",
    ahead: "⇡",
    behind: "⇣",
    main_checkout: "🏠",
    worktree: "🌳 ",
    role: "🖋 ",
    withheld: "⚠ ",
    transition: "⤳ ",
    boundary: "🏷 ",
    unrecorded: "✍ ",
    off_sequence: "❗ ",
    unreadable: "⚠ ",
    ellipsis: "…",
};

/// Pure ASCII (REQ-9): the negative signals that force this rendering — a
/// `C` locale, `TERM=dumb` — rule out `·` and `…` too, not only emoji.
const PLAIN: Glyphs = Glyphs {
    anchor: "day",
    seg: " - ",
    repo: "",
    branch: "on ",
    clean: "clean",
    dirty: "dirty",
    ahead: "ahead ",
    behind: "behind ",
    main_checkout: "[main checkout]",
    worktree: "wt:",
    role: "as ",
    withheld: "! ",
    transition: "moved: ",
    boundary: "tag: ",
    unrecorded: "unrecorded: ",
    off_sequence: "skipped: ",
    unreadable: "! ",
    ellipsis: "...",
};

fn glyphs(style: Style) -> &'static Glyphs {
    match style {
        Style::Emoji => &EMOJI,
        Style::Plain => &PLAIN,
    }
}

/// The full footer: position line, context line, message tray. Lines that
/// have nothing to say are omitted entirely — never rendered empty.
pub fn render(status: &Status, surround: &Surround, style: Style) -> String {
    let g = glyphs(style);
    let mut lines = vec![format!("{} {}", g.anchor, position_of(status, g))];
    if let Some(context) = context_line(surround, g) {
        lines.push(context);
    }
    if let Some(tray) = tray_line(status, g) {
        lines.push(tray);
    }
    lines.join("\n")
}

/// The ninth state (REQ-1): kan could not be read at all, so no position is
/// reportable — said plainly, because "day could not look" and "nothing in
/// play" must not render alike. The context line still renders: git may be
/// fine, and a partial view is information (REQ-7).
///
/// Terse on purpose, with no remedy spelled out: `day status` carries the
/// diagnosis and the pointer to `doctor`, and naming a command here would put
/// a second literal `day` into the plain rendering (AC-3).
pub fn render_unreadable(surround: &Surround, style: Style) -> String {
    let g = glyphs(style);
    let mut lines = vec![format!(
        "{} {}kan could not be read",
        g.anchor, g.unreadable
    )];
    if let Some(context) = context_line(surround, g) {
        lines.push(context);
    }
    lines.join("\n")
}

/// The four position forms (REQ-1), with `atom:` vs `atom?` preserved in
/// both styles — the separator *is* the state (REQ-2, src/status.rs).
fn position_of(status: &Status, g: &Glyphs) -> String {
    if status.uncheckable {
        return "setup: declare schema/witness".to_string();
    }
    match status.here.as_slice() {
        [] => "no atom in play".to_string(),
        [here] => {
            let mut parts = vec![format!("atom: {}", here.atom)];
            let (met, total) = (
                here.done.iter().filter(|c| c.is_met()).count(),
                here.done.len(),
            );
            if total > 0 {
                parts.push(format!("{met}/{total} done"));
            }
            if let Some(next) = here.next.first() {
                let more = if here.next.len() > 1 { g.ellipsis } else { "" };
                parts.push(format!("next: {next}{more}"));
            }
            parts.join(g.seg)
        }
        many => {
            let names: Vec<&str> = many.iter().map(|h| h.atom.as_str()).collect();
            format!("atom? {}", names.join(" | "))
        }
    }
}

/// The repo/branch/sync/checkout/identity line (REQ-4..7). `None` when every
/// segment is empty — an all-absent context earns no line.
fn context_line(surround: &Surround, g: &Glyphs) -> Option<String> {
    let mut segments = Vec::new();
    let ctx = &surround.context;
    if let Some(repo) = &ctx.repo {
        segments.push(format!("{}{repo}", g.repo));
    }
    if let Some(branch) = &ctx.branch {
        segments.push(format!("{}{branch}", g.branch));
    }
    if let Some(sync) = &ctx.sync {
        segments.push(sync_segment(sync, g));
    }
    if let Some(checkout) = &ctx.checkout {
        segments.push(checkout_segment(checkout, g));
    }
    if let Some(role) = &surround.role {
        segments.push(format!("{}{role}", g.role));
    }
    // The narrowing, last so it reads as a caveat on everything before it.
    // REQ-7: this renders whenever the count is non-zero, unconditionally —
    // the identity segment may vanish, this may not.
    if surround.withheld > 0 {
        segments.push(format!(
            "{}{} withheld from this view",
            g.withheld, surround.withheld
        ));
    }
    (!segments.is_empty()).then(|| segments.join(g.seg))
}

/// Distinct marks, not a rollup (REQ-5): clean, dirty, ahead and behind have
/// different remedies — commit, push, pull — and one glyph cannot name which.
fn sync_segment(sync: &Sync, g: &Glyphs) -> String {
    let mut parts = vec![if sync.dirty { g.dirty } else { g.clean }.to_string()];
    if let Some((ahead, behind)) = sync.ahead_behind {
        if ahead > 0 {
            parts.push(format!("{}{ahead}", g.ahead));
        }
        if behind > 0 {
            parts.push(format!("{}{behind}", g.behind));
        }
    }
    parts.join(" ")
}

/// Always renders when present, in one of three distinguishable forms
/// (REQ-15): the icon for the main checkout, the relative path for a worktree
/// under it, the bounded path for one anywhere else.
fn checkout_segment(checkout: &Checkout, g: &Glyphs) -> String {
    match checkout {
        Checkout::Main => g.main_checkout.to_string(),
        Checkout::UnderMain(rel) => format!("{}{rel}", g.worktree),
        Checkout::Elsewhere(path) => {
            format!(
                "{}{}",
                g.worktree,
                abbreviate(path, CHECKOUT_BOUND, g.ellipsis)
            )
        }
    }
}

/// Keeps the tail of a path within `bound` characters, marking the cut
/// visibly (REQ-15): a path exceeding the bound is truncated with a leading
/// ellipsis rather than silently shortened.
fn abbreviate(path: &str, bound: usize, ellipsis: &str) -> String {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= bound {
        return path.to_string();
    }
    let keep: String = chars[chars.len() - bound..].iter().collect();
    format!("{ellipsis}{keep}")
}

/// One tray for every message kind (REQ-8): transition, unrecorded boundary,
/// unrecorded artifact kinds, off-sequence findings. Never silently drops —
/// past [`TRAY_MAX`] items it ends in the count of what it is not showing.
fn tray_line(status: &Status, g: &Glyphs) -> Option<String> {
    let mut items = Vec::new();
    if let Some(t) = &status.transition {
        items.push(format!("{}past `{}`", g.transition, t.from));
    }
    if let Some(finding) = &status.unrecorded_boundary {
        items.push(format!("{}{finding}", g.boundary));
    }
    for kind in &status.unrecorded {
        items.push(format!("{}{kind}", g.unrecorded));
    }
    for finding in &status.off_sequence {
        items.push(format!("{}{finding}", g.off_sequence));
    }
    if items.is_empty() {
        return None;
    }
    let dropped = items.len().saturating_sub(TRAY_MAX);
    items.truncate(TRAY_MAX);
    let mut line = items.join(g.seg);
    if dropped > 0 {
        line.push_str(&format!(" (+{dropped} more)"));
    }
    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_recognised_remote_yields_org_name_in_every_form() {
        for url in [
            "https://github.com/kan-tools/day.git",
            "https://github.com/kan-tools/day",
            "git@github.com:kan-tools/day.git",
            "git@github.com:kan-tools/day",
            "ssh://git@github.com/kan-tools/day.git",
            "https://codeberg.org/kan-tools/day.git",
            "git://github.com/kan-tools/day.git",
        ] {
            assert_eq!(
                repo_from_remote(url).as_deref(),
                Some("kan-tools/day"),
                "{url}"
            );
        }
    }

    #[test]
    fn an_unrecognised_remote_falls_back_rather_than_guessing() {
        for url in [
            "/Users/m/code/kan-tools/day",
            "../day",
            "file:///Users/m/code/day",
            "",
            "https://github.com/day.git", // no org segment
        ] {
            assert_eq!(repo_from_remote(url), None, "{url:?}");
        }
    }

    #[test]
    fn abbreviation_is_visible_and_bounded() {
        let long = "/Users/m/code/worktrees/day-behaviour-0009e02f9dcb/tree";
        let out = abbreviate(long, 28, "…");
        assert!(out.starts_with('…'), "{out}");
        assert_eq!(out.chars().count(), 29, "{out}");
        // Under the bound, untouched.
        assert_eq!(abbreviate("short", 28, "…"), "short");
    }
}
