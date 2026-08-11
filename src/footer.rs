//! The harness footer — the status-line rendering of [`crate::status::Status`],
//! plus the repo/identity context around it (day#179, `.design/harness-footer.md`).
//!
//! **Display only.** Everything here turns already-computed state into a
//! string; nothing reads kan, git, or the render cache, and nothing decides
//! — `tests/footer.rs` scans this module to keep that true. The expensive
//! reads happen in `day hook session-start` (REQ-10), which calls
//! [`render_variants`] and caches the result; `day status-line` picks one
//! with [`select`] and prints it, invoking nothing (AC-12).
//!
//! **Why the cache holds several renderings.** Two things the footer must
//! respect are knowable only when the status line *runs*, not when the hook
//! renders it: the terminal width (Claude Code sets `COLUMNS`/`LINES` before
//! invoking the command, since v2.1.153) and the user's `DAY_FOOTER`
//! preference, which lives in the status line's environment rather than the
//! hook's. Rendering at hook time and printing verbatim made both
//! unreachable — `DAY_FOOTER=plain day status-line` did nothing at all. So
//! the hook renders every variant and the status line *picks* one. That
//! keeps the `.day/` carve-out intact: the cache still holds nothing but
//! rendered display state, and choosing which pre-rendered string to show is
//! a display decision, not day deciding anything it reports.
//!
//! **Width is estimated, never known** — the same asymmetry REQ-18 draws for
//! emoji. [`display_width`] counts what the terminal *probably* draws;
//! emoji are double-width in some terminals and single in others, which RQ-4
//! already records as the reason the layout is flat rather than indented. So
//! budgets are deliberately conservative and elision is *visible*: a footer
//! that dropped something says so, exactly as the tray does (RQ-5 —
//! "truncates visibly or not at all"). A segment silently missing for width
//! would be indistinguishable from a segment day could not fill, and REQ-7
//! makes that distinction load-bearing.
//!
//! Ten states, not nine. REQ-1 names nine and conflates two of them under
//! "the partial-read report": a log day could read *partially* and a log day
//! could not read *at all* are different facts calling for different
//! responses, and only the second was built first time round. Both render
//! here, and the partial-read report joins the narrowing indicator in the
//! **never-elided** class — a caveat dropped for width is a caveat that lied.

use crate::status::Status;

/// The environment override for which rendering is used (REQ-16). `plain` or
/// `emoji`; any other value falls through to detection. Read where the
/// status line runs, so the obvious `DAY_FOOTER=plain day status-line` works.
pub const STYLE_ENV: &str = "DAY_FOOTER";

/// The width bound for a checkout path outside the main root (REQ-15).
/// Measured need: a real worktree abbreviated to `…/day-behaviour-0009e02f9dcb/tree`
/// — without a bound the segment eats the line.
pub const CHECKOUT_BOUND: usize = 24;

/// How many tray items render before the tray says it truncated (REQ-8).
/// The tray truncates visibly or not at all (RQ-5).
pub const TRAY_MAX: usize = 3;

/// The width budgets the hook pre-renders, widest first. [`select`] picks the
/// widest variant that actually fits the terminal, so these are granularity,
/// not a promise: a variant's *rendered* width is what gets compared.
///
/// Chosen against real terminal sizes rather than round numbers — 100 is a
/// comfortable wide pane, 72 a half-screen split, 48 a narrow sidebar. The
/// narrowest is the floor: when nothing fits, it is what prints, because a
/// too-wide footer is better than no footer.
pub const BUDGETS: [usize; 3] = [100, 72, 48];

/// The budget for the single rendering written to the legacy cache file, for
/// a reader that does not know about variants. Deliberately the narrowest
/// sensible one: it will be printed verbatim into a terminal of unknown
/// width, so it should fit the common case rather than the roomy one.
pub const FALLBACK_BUDGET: usize = 72;

/// The width assumed when `COLUMNS` is absent — a Claude Code older than
/// v2.1.153, or a hand-run `day status-line`. 80 is the conventional floor;
/// assuming a wide terminal would overflow the common case, and assuming the
/// narrowest would throw away information on every terminal that has room.
///
/// fallback: no-columns-assume-80
pub const ASSUMED_COLUMNS: usize = 80;

/// Which rendering the footer uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Emoji,
    Plain,
}

impl Style {
    /// How this style is tagged in the cache header. Parsed back by
    /// [`select`], so the two must agree; a round-trip test pins that.
    fn tag(self) -> &'static str {
        match self {
            Style::Emoji => "emoji",
            Style::Plain => "plain",
        }
    }

    /// Every style, so callers that must cover all of them cannot miss one
    /// by enumerating a literal pair (`.design/harness-footer.md`'s "a list
    /// that can be derived must be derived").
    pub const ALL: [Style; 2] = [Style::Emoji, Style::Plain];
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
    /// `COLUMNS`, which Claude Code sets to the terminal width before running
    /// the status-line command (documented, v2.1.153+).
    pub columns: Option<String>,
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
            columns: var("COLUMNS"),
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

    /// The terminal width to lay out against. An unset, empty, or
    /// unparseable `COLUMNS` falls back to [`ASSUMED_COLUMNS`] rather than to
    /// zero — laying out against zero would elide everything.
    ///
    /// fallback: no-columns-assume-80
    pub fn width(&self) -> usize {
        self.columns
            .as_deref()
            .and_then(|c| c.trim().parse::<usize>().ok())
            .filter(|c| *c > 0)
            .unwrap_or(ASSUMED_COLUMNS)
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
    /// there is no error path that blanks it, and it is never elided for
    /// width either — a caveat dropped to save room is a caveat that lied.
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
    /// Declarations day could not read — the partial-read state. Distinct
    /// from [`Self::unreadable`], which is "day could not read the log at
    /// all": one means the report is incomplete, the other that there is no
    /// report. A shared glyph made them the same fact on the surface.
    partial: &'static str,
    transition: &'static str,
    boundary: &'static str,
    unrecorded: &'static str,
    off_sequence: &'static str,
    unreadable: &'static str,
    ellipsis: &'static str,
    /// Marks that something was dropped for width, so elision is visible.
    elided: &'static str,
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
    partial: "◐ ",
    transition: "⤳ ",
    boundary: "🏷 ",
    unrecorded: "✍ ",
    off_sequence: "❗ ",
    unreadable: "⛔ ",
    ellipsis: "…",
    elided: "…+",
};

/// Pure ASCII (REQ-9): the negative signals that force this rendering — a
/// `C` locale, `TERM=dumb` — rule out `·` and `…` too, not only emoji. A
/// test asserts every rendered byte is ASCII, because "distinct" would be
/// satisfied by emoji here too.
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
    partial: "? ",
    transition: "moved: ",
    boundary: "tag: ",
    unrecorded: "unrecorded: ",
    off_sequence: "skipped: ",
    unreadable: "!! ",
    ellipsis: "...",
    elided: "+",
};

fn glyphs(style: Style) -> &'static Glyphs {
    match style {
        Style::Emoji => &EMOJI,
        Style::Plain => &PLAIN,
    }
}

/// What a terminal will *probably* draw this string as, in columns.
///
/// An estimate, and unavoidably one: the same emoji is double-width in some
/// terminals and single in others, which is RQ-4's recorded reason for a flat
/// layout. day can know what it *encoded*, never what the font *renders* —
/// REQ-18's asymmetry, applied to width instead of to coverage. Budgets are
/// therefore conservative and elision is visible, so an estimate that is
/// wrong costs a wrapped line rather than a silently dropped fact.
pub fn display_width(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            // Zero-width: the variation selector that asks for the emoji
            // presentation, and the joiner in a composed sequence.
            '\u{fe0f}' | '\u{200d}' | '\u{fe0e}' => {}
            // Emoji and pictographs: double-width essentially everywhere.
            '\u{1f300}'..='\u{1faff}' | '\u{1f000}'..='\u{1f2ff}' => width += 2,
            // Miscellaneous symbols and dingbats are ambiguous: drawn wide
            // when followed by U+FE0F asking for emoji presentation, narrow
            // otherwise. That is exactly how this file uses them — `☀️` wide,
            // `✔` narrow — so the rule matches the glyph tables above.
            '\u{2190}'..='\u{2bff}' => {
                if chars.peek() == Some(&'\u{fe0f}') {
                    width += 2;
                } else {
                    width += 1;
                }
            }
            // CJK and fullwidth forms, for a branch or repo name in one.
            '\u{1100}'..='\u{115f}'
            | '\u{2e80}'..='\u{a4cf}'
            | '\u{ac00}'..='\u{d7a3}'
            | '\u{f900}'..='\u{faff}'
            | '\u{fe30}'..='\u{fe6f}'
            | '\u{ff00}'..='\u{ff60}'
            | '\u{ffe0}'..='\u{ffe6}' => width += 2,
            _ => width += 1,
        }
    }
    width
}

/// One segment of the context line, with what it costs to keep.
struct Seg {
    text: String,
    /// Higher drops first. A segment day cannot fill never gets here at all
    /// (REQ-7); this orders what happens when day *can* fill it and the
    /// terminal has no room.
    drop_order: u8,
    /// Never dropped for width, whatever the budget. The narrowing and the
    /// partial-read report are caveats on everything else on the line, and a
    /// caveat elided to save room misreports the thing it qualifies.
    pinned: bool,
}

/// Every variant, tagged, for the cache. This is what
/// `day hook session-start` writes and [`select`] reads back.
///
/// Rendering all of them costs microseconds and buys the two things the hook
/// cannot know: the terminal width and the user's `DAY_FOOTER`.
pub fn render_variants(status: &Status, surround: &Surround) -> String {
    variants_from(|style, budget| render(status, surround, style, budget))
}

/// [`render_variants`] for the could-not-read-the-log-at-all state.
pub fn render_unreadable_variants(surround: &Surround) -> String {
    variants_from(|style, budget| render_unreadable(surround, style, budget))
}

fn variants_from(mut one: impl FnMut(Style, usize) -> String) -> String {
    let mut out = String::new();
    for style in Style::ALL {
        for budget in BUDGETS {
            let block = one(style, budget);
            out.push_str(&format!(
                "{HEADER} {} {}\n{block}\n",
                style.tag(),
                widest_line(&block)
            ));
        }
    }
    out
}

/// Marks a variant in the cache. Deliberately not a bare `#`: a rendered
/// footer line could begin with one.
const HEADER: &str = "#day-footer";

fn widest_line(block: &str) -> usize {
    block.lines().map(display_width).max().unwrap_or(0)
}

/// Picks the variant to print: the requested style, and the widest rendering
/// that fits `columns`.
///
/// **This is the whole of what the status line decides**, and it decides it
/// from the environment, never from the cache's content — the cache supplies
/// candidates and nothing else. Returns `None` only when the text carries no
/// variants at all, which the caller treats as "print it verbatim": a cache
/// written by an older day is a stale rendering, not an error.
///
/// fallback: cache-without-variants
pub fn select(cached: &str, columns: usize, style: Style) -> Option<String> {
    let mut best: Option<(usize, String)> = None;
    let mut narrowest: Option<(usize, String)> = None;
    let mut current: Option<(bool, usize)> = None;
    let mut block = String::new();

    // A trailing sentinel so the last block is flushed by the same arm as
    // every other one, rather than by a copy of the logic after the loop.
    for line in cached.lines().chain(std::iter::once(HEADER)) {
        if let Some(rest) = line.strip_prefix(HEADER) {
            if let Some((matches_style, width)) = current.take() {
                let block = std::mem::take(&mut block).trim_end().to_string();
                if matches_style && !block.is_empty() {
                    if width <= columns && best.as_ref().is_none_or(|(w, _)| width > *w) {
                        best = Some((width, block.clone()));
                    }
                    if narrowest.as_ref().is_none_or(|(w, _)| width < *w) {
                        narrowest = Some((width, block));
                    }
                }
            }
            let mut fields = rest.split_whitespace();
            let tag = fields.next();
            let width = fields.next().and_then(|w| w.parse().ok());
            current = match (tag, width) {
                (Some(tag), Some(width)) => Some((tag == style.tag(), width)),
                // A header this day cannot parse is skipped rather than
                // guessed at, and the remaining variants still serve.
                _ => None,
            };
            continue;
        }
        if current.is_some() {
            block.push_str(line);
            block.push('\n');
        }
    }

    // Nothing fits: print the narrowest anyway. A footer too wide for the
    // pane wraps; no footer at all tells the reader nothing, and "day is
    // running and has nothing to say" is a different claim from "the pane is
    // narrow".
    best.or(narrowest).map(|(_, block)| block)
}

/// One variant: position line, context line, message tray, laid out to fit
/// `budget` columns. Lines that have nothing to say are omitted entirely —
/// never rendered empty.
pub fn render(status: &Status, surround: &Surround, style: Style, budget: usize) -> String {
    let g = glyphs(style);
    let anchor = format!("{} ", g.anchor);
    let mut lines = vec![format!(
        "{anchor}{}",
        position_of(status, g, budget.saturating_sub(display_width(&anchor)))
    )];
    if let Some(context) = context_line(status, surround, g, budget) {
        lines.push(context);
    }
    if let Some(tray) = tray_line(status, g, budget) {
        lines.push(tray);
    }
    lines.join("\n")
}

/// The tenth state: kan could not be read **at all**, so no position is
/// reportable — said plainly, because "day could not look" and "nothing in
/// play" must not render alike. The context line still renders: git may be
/// fine, and a partial view is information (REQ-7).
///
/// Distinct from the partial-read report, which the ordinary [`render`]
/// carries. REQ-1 conflated the two; they call for different responses, so
/// they get different glyphs and different words.
pub fn render_unreadable(surround: &Surround, style: Style, budget: usize) -> String {
    let g = glyphs(style);
    let mut lines = vec![format!(
        "{} {}kan could not be read",
        g.anchor, g.unreadable
    )];
    // `status` is genuinely absent here, so the context line renders without
    // the partial-read segment — there is no partial read, there is no read.
    if let Some(context) = context_line_of(Vec::new(), surround, g, budget) {
        lines.push(context);
    }
    lines.join("\n")
}

/// The four position forms (REQ-1), with `atom:` vs `atom?` preserved in
/// both styles — the separator *is* the state (REQ-2, src/status.rs).
fn position_of(status: &Status, g: &Glyphs, budget: usize) -> String {
    if status.uncheckable {
        // Names the SUBJECT that resolves it, never `day init`, which
        // declares no witnesses (day#108). Built from the schema prefix and
        // the slug rather than written out, so a rename moves it — but note
        // `WITNESS_SLUG` is the bare `witness`, not the subject: composing
        // them wrongly renders `declare witness`, which names nothing a
        // reader can act on. `tests/footer.rs` asserts the full subject.
        return format!(
            "setup: declare {}{}",
            crate::schema::SCHEMA_PREFIX,
            crate::telos::WITNESS_SLUG
        );
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
            // Drop trailing detail rather than the atom name: which atom you
            // are in is the answer, `1/2 done` is colour.
            while parts.len() > 1 && display_width(&parts.join(g.seg)) > budget {
                parts.pop();
            }
            parts.join(g.seg)
        }
        many => {
            // Named, not counted: knowing you are in *either* build or
            // release tells you what to do next; knowing there are two does
            // not. Under width pressure the names go one at a time, and the
            // count of what went is shown rather than the list silently
            // shortening.
            let names: Vec<&str> = many.iter().map(|h| h.atom.as_str()).collect();
            let mut shown = names.len();
            loop {
                let line = render_ambiguous(&names, shown, g);
                if shown <= 1 || display_width(&line) <= budget {
                    return line;
                }
                shown -= 1;
            }
        }
    }
}

fn render_ambiguous(names: &[&str], shown: usize, g: &Glyphs) -> String {
    let dropped = names.len() - shown;
    let mut line = format!("atom? {}", names[..shown].join(" | "));
    if dropped > 0 {
        line.push_str(&format!(" {}{dropped}", g.elided));
    }
    line
}

/// The repo/branch/sync/checkout/identity line (REQ-4..7), plus the two
/// pinned caveats. `None` when every segment is empty — an all-absent
/// context earns no line.
fn context_line(status: &Status, surround: &Surround, g: &Glyphs, budget: usize) -> Option<String> {
    context_line_of(partial_segment(status, g), surround, g, budget)
}

/// The partial-read report (REQ-1's ninth state, as distinct from the
/// tenth): day read the log and could not read some of what it declares, so
/// everything else on this footer is computed over an incomplete vocabulary.
///
/// Pinned, like the narrowing: it is a caveat on the position beside it.
/// `status::compute` already computes this and `render_long` already shows
/// it; the footer dropping it meant the human's bar stayed confident while
/// the model was told the report was partial — the day#60 asymmetry.
fn partial_segment(status: &Status, g: &Glyphs) -> Vec<Seg> {
    if status.unreadable.is_empty() {
        return Vec::new();
    }
    vec![Seg {
        text: format!("{}{} unreadable", g.partial, status.unreadable.len()),
        drop_order: 0,
        pinned: true,
    }]
}

fn context_line_of(
    mut segments: Vec<Seg>,
    surround: &Surround,
    g: &Glyphs,
    budget: usize,
) -> Option<String> {
    let ctx = &surround.context;
    // Drop order, lowest information first. `Checkout::Main` leads because
    // it is the *unremarkable* case — "you are where you think you are" —
    // while a worktree is the thing the segment exists to surface (RQ-7), so
    // it is ordered with the rest rather than beside its own icon.
    if let Some(Checkout::Main) = &ctx.checkout {
        segments.push(Seg {
            text: g.main_checkout.to_string(),
            drop_order: 6,
            pinned: false,
        });
    }
    if let Some(role) = &surround.role {
        segments.push(Seg {
            text: format!("{}{role}", g.role),
            drop_order: 5,
            pinned: false,
        });
    }
    if let Some(repo) = &ctx.repo {
        segments.push(Seg {
            text: format!("{}{repo}", g.repo),
            drop_order: 4,
            pinned: false,
        });
    }
    match &ctx.checkout {
        Some(Checkout::Main) | None => {}
        Some(checkout) => segments.push(Seg {
            text: checkout_segment(checkout, g),
            drop_order: 3,
            pinned: false,
        }),
    }
    if let Some(sync) = &ctx.sync {
        segments.push(Seg {
            text: sync_segment(sync, g),
            drop_order: 2,
            pinned: false,
        });
    }
    if let Some(branch) = &ctx.branch {
        segments.push(Seg {
            text: format!("{}{branch}", g.branch),
            drop_order: 1,
            pinned: false,
        });
    }
    // REQ-7: this renders whenever the count is non-zero, unconditionally,
    // and is never elided for width.
    if surround.withheld > 0 {
        segments.push(Seg {
            text: format!("{}{} withheld", g.withheld, surround.withheld),
            drop_order: 0,
            pinned: true,
        });
    }
    if segments.is_empty() {
        return None;
    }
    segments.sort_by_key(|s| s.drop_order);
    Some(fit(segments, g, budget))
}

/// Joins segments, dropping the least informative until it fits and saying
/// how many went. Pinned segments are never dropped — if they alone exceed
/// the budget the line overflows, which is the correct failure: a caveat
/// that does not fit still has to be seen.
fn fit(segments: Vec<Seg>, g: &Glyphs, budget: usize) -> String {
    let mut kept: Vec<&Seg> = segments.iter().collect();
    let mut dropped = 0usize;
    loop {
        let line = join_kept(&kept, dropped, g);
        if display_width(&line) <= budget {
            return line;
        }
        // The last droppable segment in drop order, which sorting put last.
        let Some(pos) = kept.iter().rposition(|s| !s.pinned) else {
            return line;
        };
        kept.remove(pos);
        dropped += 1;
    }
}

fn join_kept(kept: &[&Seg], dropped: usize, g: &Glyphs) -> String {
    let mut parts: Vec<&str> = kept.iter().map(|s| s.text.as_str()).collect();
    let marker = format!("{}{dropped}", g.elided);
    if dropped > 0 {
        parts.push(&marker);
    }
    parts.join(g.seg)
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

/// One of three distinguishable forms (REQ-15): the icon for the main
/// checkout (rendered by the caller, since it sorts differently), the
/// relative path for a worktree under it, the bounded path for one anywhere
/// else.
fn checkout_segment(checkout: &Checkout, g: &Glyphs) -> String {
    match checkout {
        Checkout::Main => g.main_checkout.to_string(),
        Checkout::UnderMain(rel) => format!("{}{rel}", g.worktree),
        Checkout::Elsewhere(path) => format!(
            "{}{}",
            g.worktree,
            abbreviate(path, CHECKOUT_BOUND, g.ellipsis)
        ),
    }
}

/// Keeps the tail of a path within `bound` **rendered columns including the
/// ellipsis**, marking the cut visibly (REQ-15).
///
/// The bound covers the marker, which the first version did not: it kept
/// `bound` characters and *then* prefixed `…`, so the segment was always
/// wider than the bound it declared and the unit test codified the overshoot
/// by asserting a length of `bound + 1`.
fn abbreviate(path: &str, bound: usize, ellipsis: &str) -> String {
    if display_width(path) <= bound {
        return path.to_string();
    }
    let room = bound.saturating_sub(display_width(ellipsis));
    let chars: Vec<char> = path.chars().collect();
    let mut take = 0;
    let mut width = 0;
    for c in chars.iter().rev() {
        let w = display_width(&c.to_string());
        if width + w > room {
            break;
        }
        width += w;
        take += 1;
    }
    let keep: String = chars[chars.len() - take..].iter().collect();
    format!("{ellipsis}{keep}")
}

/// One tray for every message kind (REQ-8): transition, unrecorded boundary,
/// unrecorded artifact kinds, off-sequence findings. Never silently drops —
/// past [`TRAY_MAX`] items, or past the width budget, it ends in the count of
/// what it is not showing.
fn tray_line(status: &Status, g: &Glyphs, budget: usize) -> Option<String> {
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
    let total = items.len();
    let mut shown = total.min(TRAY_MAX);
    loop {
        let line = render_tray(&items, shown, total, g);
        if shown <= 1 || display_width(&line) <= budget {
            return Some(line);
        }
        shown -= 1;
    }
}

fn render_tray(items: &[String], shown: usize, total: usize, g: &Glyphs) -> String {
    let dropped = total - shown;
    let mut line = items[..shown].join(g.seg);
    if dropped > 0 {
        line.push_str(&format!(" ({}{dropped} more)", g.elided));
    }
    line
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

    /// The bound covers the marker — the defect the first version codified.
    #[test]
    fn abbreviation_is_visible_and_within_its_declared_bound() {
        let long = "/Users/m/code/worktrees/day-behaviour-0009e02f9dcb/tree";
        let out = abbreviate(long, CHECKOUT_BOUND, "…");
        assert!(out.starts_with('…'), "{out}");
        assert!(
            display_width(&out) <= CHECKOUT_BOUND,
            "{out} is {} wide, over the declared bound of {CHECKOUT_BOUND}",
            display_width(&out)
        );
        assert_eq!(abbreviate("short", CHECKOUT_BOUND, "…"), "short");
    }

    /// The width estimate has to treat the glyphs this file actually uses
    /// the way a terminal does, or every budget is wrong by a constant.
    #[test]
    fn width_counts_emoji_wide_and_ascii_narrow() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("☀️"), 2, "an emoji-presentation sun is wide");
        assert_eq!(display_width("✔"), 1, "a text-presentation check is narrow");
        assert_eq!(display_width("📁"), 2);
        assert_eq!(display_width("·"), 1);
    }
}
