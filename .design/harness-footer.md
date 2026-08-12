# Feature: the harness footer

## Summary

Redesign what day renders into the Claude Code status line: one `☀️` instead of
`day` repeated on every line, a repo/branch/sync/worktree context line, the
session's signing identity, and every message kind collapsed into one tray with
a glyph key. day#179 is the issue; the mockup there is the target.

## Requirements

- REQ-1: The footer renders **every** state `Status::render_line` in
  `src/status.rs` can produce. There are nine: four mutually exclusive position
  states (`setup: declare schema/witness`, `no atom in play`, `atom:` for one
  inferred, `atom?` for several), four message kinds (transition, unrecorded
  boundary, unrecorded kind, off-sequence), and the partial-read report from
  `Status::render_unreadable`. A glyph key that covers only the mockup's four
  leaves five states unrenderable.

- REQ-2: The distinction `atom:` versus `atom?` is preserved. It is load-bearing
  and documented as such at `src/status.rs:499-503` — the separator *is* the
  state, `:` meaning one atom inferred and `?` meaning several the evidence does
  not distinguish. Whatever replaces the separator must still express it.

- REQ-3: `day` is not repeated per line. The word becomes one `☀️` at the left of
  the first line.

- REQ-4: The context line reports **branch**, **sync**, and **which checkout**.
  `src/git.rs` implements none of these today — its public surface is tags,
  cycle boundaries, tracked and changed files, and the position fingerprint — so
  each is a new method. Branch and paths come from `rev-parse`, ahead/behind and
  dirtiness from `status`, both already permitted by
  `ac1_day_never_invokes_a_mutating_git_subcommand` in `tests/assess.rs`.

- REQ-12: The repo is named from the **remote when there is one**, and from the
  main checkout's directory otherwise. `git remote get-url origin` yields
  `kan-tools/day`; with no remote — a fresh `git init`, which is the population
  `telos/v1.0` names — the fallback is the parent of `rev-parse
  --git-common-dir`. **Not `rev-parse --show-toplevel`**, which names the
  *current* directory and therefore renders `day-oss` inside a worktree of
  `day`: measured, and self-defeating in a footer whose job includes showing you
  are in a worktree.

- REQ-13: `remote` is added to the permitted git subcommands, taking the
  whitelist from seven entries to eight, and the reason is recorded at the
  whitelist rather than only here. It is unambiguously a read; the cost is that
  the list is deliberately narrow and every addition widens what `src/git.rs`
  may reach.

- REQ-14: Turning a remote URL into `org/name` handles the forms a remote
  actually takes — `https://host/org/name.git`, `git@host:org/name.git`, either
  without the `.git` suffix, a non-GitHub host, and a local filesystem path —
  and **falls back to the REQ-12 directory name rather than guessing** when the
  URL is not of a recognised shape. A wrong repo name is worse than a plain one.

- REQ-15: The checkout segment always renders, in one of three forms: an icon
  when this *is* the main checkout; the path **relative to the main checkout**
  when the worktree lives under it (`.claude/worktrees/abcd`); and an
  abbreviated, width-bounded path when it lives anywhere else. Measured on a
  real machine, every worktree was in the third case and one abbreviated to
  `…/day-behaviour-0009e02f9dcb/tree`, so the bound is a requirement rather than
  a nicety.

- REQ-5: Sync state is reported with **distinct marks rather than a rollup**:
  clean, dirty tree, ahead, behind, and their combinations. One `✗` covering all
  of them hides which, and commit, push and pull are different remedies.

- REQ-6: The footer reports the **session's signing identity**: the name of the
  declared kan role whose DID matches the active one. day invents no vocabulary
  for this — `director` is kan's own example in `kan identity role --help`, the
  active identity comes from `KanClient::identity()` in `src/kan_client.rs`, and
  the declared roles come from `kan identity role list --json`.

- REQ-7: **A segment day cannot fill is omitted, and a narrowing is never
  omitted.** No declared roles, no identity, a kan that does not support the
  verb, or any error reading it: the identity segment does not render, because a
  missing segment honestly says "nothing to report". A *narrowed view* is the
  opposite — it is something to report — so when `claims_withheld_from_view()`
  is non-zero the footer says so, and that indicator has no error path that
  blanks it.

- REQ-8: Messages collapse into **one tray** rather than one line each, and the
  tray **never silently drops** a message. When it truncates it says so, the way
  `src/status.rs:567` already collapses with `(+N more)`.

- REQ-9: The footer renders without emoji. Terminals vary in whether they render
  emoji at all and in how wide they draw them, so a plain-text rendering exists
  and carries the same nine states.

- REQ-16: Which rendering is used resolves through three layers, **most specific
  first**: an explicit environment override for this session, then a declared
  preference in kan, then automatic detection. Session beats project, the same
  precedence `KAN_IDENTITY_FILE` has over a workspace's default identity.

- REQ-17: Automatic detection **uses only signals day can know, and claims no
  positive**. Plain is chosen when the locale is not UTF-8 (`LC_ALL`, `LC_CTYPE`
  or `LANG`), when `TERM` is `dumb` or unset, or when `NO_COLOR` is set; emoji
  otherwise. There is **no allowlist of terminal programs** — `TERM_PROGRAM`
  matching is a classifier keyed on a positive list that stops matching as new
  terminals appear and reports "supported" by having found nothing to object to,
  which is a defect class this repo has recorded repeatedly.

- REQ-18: The distinction detection can and cannot make is stated where the code
  makes it: day can know whether emoji are **encodable**, never whether they are
  **renderable**. A UTF-8 locale with no emoji font renders `▯▯` and reports
  nothing day can see. This is why REQ-16 has an override at all, and why RQ-4's
  flat layout matters — it removes width variance as a failure mode rather than
  detecting it.

- REQ-19: The **declared layer is specified but not built in v1**. A
  `schema/*` claim for the footer would be a loader whose contract is "absent
  means use the shipped default", which is the case day#160 records
  `atoms::newest_fenced`'s withheld-read guard as over-broad for. Adding it now
  makes a seventh such loader — the thing `.design/day-config.md` RQ-1 sequenced
  `day config` behind `.design/read-visibility.md` to avoid. v1 ships the
  override and the detection; the declared layer lands with the rest.

- REQ-10: Every element the footer adds is read in the **session-start hook**,
  never in the status line itself. Claude Code cancels an in-flight status line
  at 300 ms, which is the entire reason `.day/` exists (`src/cache.rs`), and the
  cache may be read only to display, never to decide.

- REQ-11: The footer is **display only**. It gates nothing, and reading the
  cache to render it does not make the cache a decision input — the boundary
  `tests/plugin.rs` enforces by scanning the rest of `src/` for `.day/` access.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) A test drives all nine states through the renderer and
      asserts nine distinct outputs. A state that renders identically to another
      fails it.

- [ ] AC-2: (REQ-2) One inferred atom and several candidate atoms render
      differently, and a test asserts the difference is present in both the
      emoji and the plain-text renderings.

- [ ] AC-3: (REQ-3) The rendered footer contains the literal `day` at most once,
      asserted over every one of the nine states.

- [ ] AC-4: (REQ-4) Against a fixture repo, the context line reports the branch
      and the ahead/behind counts, and reports them from `status` rather than
      from a second source that could disagree.

- [ ] AC-5: (REQ-13) `ac1_day_never_invokes_a_mutating_git_subcommand` passes
      with a permitted list of exactly eight entries including `remote`, and the
      test names why the eighth was added. A ninth added without a stated reason
      fails it.

- [ ] AC-14: (REQ-12) With a remote configured the repo renders as `org/name`;
      with the remote removed it renders as the main checkout's directory name;
      and **inside a worktree both answers are unchanged**, which is the
      assertion that fails if `--show-toplevel` is used.

- [ ] AC-15: (REQ-14) Each remote URL form — https, ssh, suffixed and
      unsuffixed, a non-GitHub host, and a local path — is driven through the
      parser, and an unrecognised form yields the directory-name fallback rather
      than a mangled `org/name`.

- [ ] AC-16: (REQ-15) The three checkout cases render distinguishably: the main
      checkout as the icon, a worktree under the main root as a relative path,
      and a worktree elsewhere as an abbreviated path no longer than the
      declared bound. A path exceeding the bound is truncated visibly.

- [ ] AC-6: (REQ-5) Clean, dirty, ahead, behind, and dirty-and-ahead-and-behind
      each render distinguishably; a test asserts five distinct outputs rather
      than asserting the presence of a phrase.

- [ ] AC-7: (REQ-6) With a declared role whose DID is the active one, the footer
      names that role. With a role declared but a different DID active, it does
      not name it.

- [ ] AC-8: (REQ-7) With no declared roles, the identity segment is absent from
      the output entirely — not rendered as empty, not rendered as a placeholder.
      With the identity read failing outright, the rest of the footer still
      renders and the process still exits zero.

- [ ] AC-9: (REQ-7) With `claims_withheld_from_view()` non-zero, the footer
      reports the narrowing; and a test asserts there is no input for which the
      narrowing is non-zero and the indicator is absent.

- [ ] AC-10: (REQ-8) With more messages than the tray can show, the output ends
      in the truncation mark and the count of what was dropped; with a tray that
      fits, no truncation mark appears.

- [ ] AC-11: (REQ-9) The plain-text rendering carries all nine states, asserted
      by the same table AC-1 drives.

- [ ] AC-17: (REQ-17) Each negative signal independently forces the plain
      rendering: `LANG=C`, an `LC_ALL` without UTF-8, `TERM=dumb`, `TERM` unset,
      and `NO_COLOR=1`. Each is a separate case, so one of them being handled
      does not make the others look handled.

- [ ] AC-18: (REQ-17) With a UTF-8 locale and none of the negative signals, the
      emoji rendering is chosen — and a source scan asserts no `TERM_PROGRAM`
      allowlist exists, so the positive case can never come from a list that
      rots.

- [ ] AC-19: (REQ-16) The environment override forces **both** directions
      against detection: plain where detection would choose emoji, and emoji
      where a negative signal would otherwise force plain. An override that only
      works one way is half a control.

- [ ] AC-20: (REQ-18, REQ-19) With no override and no declaration, the footer
      still renders in every locale tested, and `day status-line` exits zero —
      detection has no failure mode that produces no footer. A test asserts v1
      reads no `schema/*` subject for this, so the seventh
      absent-means-default loader is not introduced by accident.

- [ ] AC-12: (REQ-10) `day status-line` makes **zero** `kan` and **zero** `git`
      invocations, asserted against counting stubs. This is the existing
      guarantee and the criterion that keeps it true as the footer grows.

- [ ] AC-13: (REQ-11) A source scan asserts the footer renderer reads the cache
      only to produce output — no branch, no early return, no comparison keyed
      on a cached value.

## Architecture

**Where the line comes from now.** `Status::render_line` in `src/status.rs`
builds a multi-line string; `day hook session-start` writes it to `.day/statusline`
through `cache::write_status_line` (`src/cache.rs`); `day status-line` reads that
file and prints it. The wiring is a user-installed settings entry —
`{"statusLine": {"type": "command", "command": "day status-line"}}`, emitted by
`src/cli/mod.rs:964` — because a plugin cannot declare a top-level `statusLine`,
which `docs/ROADMAP.md:467` records as verified rather than assumed.

**The renderer is where the change lands**, not the cache and not the reader.
The cache is an opaque string and stays one; `day status-line` keeps reading a
file and printing it, which is what keeps AC-12 true and the 300 ms budget
intact. Everything new is computed in the hook.

**git.rs grows three read methods and no new subcommand.** Branch and toplevel
come from `rev-parse`, ahead/behind and dirtiness from `status`, both already in
the whitelist `tests/assess.rs` enforces. The worktree segment renders only when
the checkout is not the main one, which is why it is absent for this repo and
present in the mockup.

**Identity is a read over kan's existing surface.** `KanClient::identity()` runs
`kan identity did`; `kan identity role list --json` gives the declared roles with
their DIDs; the footer renders the name of the role whose DID matches. Nothing is
declared by day, and no `schema/*` subject is invented — kan#115 shipped this
mechanism and `kan identity role --help` uses `director` as its own example.

**The identity segment and the narrowing indicator are one fact seen twice.**
`TrustBase::Solo` is `author == trusted`, so signing as a role narrows what the
log returns — reproduced in kan#121, where two identities on one workspace each
read a complete-looking view, neither mentioning the other, exit 0 both times.
day already computes the log-wide count as
`KanClient::claims_withheld_from_view()`, whose doc comment states that every
surface which *enumerates* has to carry it. The footer is such a surface. That is
why REQ-7 splits the degradation rule: the identity may vanish, the narrowing may
not.

**What the mockup asked for and this drops.** The horizontal rule costs a whole
line for decoration, which on a three-line footer is a quarter of the budget. And
emoji are double-width inconsistently across terminals, so the indented mockup
will not align under its first line; the rendering is flat rather than indented
for that reason.

## Resolved Questions

- RQ-1: The identity segment reads kan's **declared roles**, not a day-owned
  vocabulary and not the bare DID. `director` is kan's own example; the segment
  shows the role whose DID is active.

- RQ-2: **Absent and error both omit the segment; a narrowing never omits.** A
  segment day cannot fill says nothing by not appearing, which is honest for a
  display surface. A partial view is information, so it renders whenever the
  withheld count is non-zero.

- RQ-3: **Sync uses distinct marks, not a rollup**, because clean, dirty, ahead
  and behind have different remedies and one glyph cannot name which.

- RQ-4: **The horizontal rule is dropped**, and the footer is flat rather than
  indented, because emoji width varies by terminal and an indented layout will
  not align.

- RQ-5: **The tray truncates visibly or not at all.** A tray that silently drops
  a warning is worse than a third line, so truncation carries a mark and a count.

- RQ-6: **Both, rather than either.** The repo is named from the remote when one
  exists and from the main checkout's directory otherwise, and the checkout
  segment is relativised against the main checkout with an icon for being in it.
  `remote` is therefore added to the git whitelist as an eighth entry, taken as
  a deliberate boundary decision with its reason recorded at the whitelist.

- RQ-7: **`rev-parse --show-toplevel` is not the fallback**, `--git-common-dir`
  is. Measured: in a worktree of `day`, `--show-toplevel` renders `day-oss` and
  `--git-common-dir`'s parent renders `day`. The naive read names a different
  project precisely in the case this footer exists to make visible, which is why
  AC-14 asserts the answer is unchanged inside a worktree.

- RQ-8: **Emoji unless a signal day can actually know says otherwise, plus an
  override.** Detection is asymmetric — a non-UTF-8 locale, `TERM=dumb` and
  `NO_COLOR` are knowable negatives; there is no knowable positive, because font
  coverage is invisible to the environment. So day claims only the negative, and
  an environment override exists for the case it gets wrong. The declared layer
  is specified and deferred, per REQ-19.

## Corrections from the adversarial review

Three cold reviews of the implementation returned BLOCK, and two findings were
against **this document** rather than against the code. Recorded here, with the
verdict on `harness-footer` (`bafyreiccsprqo…`), because a design doc that is
wrong is a worse defect than an implementation that is — it will be built from
again.

- **REQ-1 conflates two states, so it names nine and there are ten.** "The
  partial-read report from `Status::render_unreadable`" folds together a log day
  read *partially* — `Status.unreadable` is non-empty, so the position rendered
  beside it was computed over a vocabulary day knows is incomplete — and a log
  day could not read *at all*. They call for different responses and get
  different glyphs. Only the second was built first time round, and the
  requirement's wording is why nobody noticed: the footer rendered a confident
  `atom: build · 1/2 done` over an incomplete read while the *model* channel was
  told the report was partial, which is the day#60 asymmetry `src/hooks.rs`
  already records as mattering most in practice. Both render now, and the
  partial-read report joins the narrowing in the never-elided class.

- **AC-3 states a property of the output that is false in day's own repo.**
  "The rendered footer contains the literal `day` at most once" passed only
  because the test drove an empty context: with a repo name the plain footer
  reads `day setup: …` / `kan-tools/day - on main - …`, twice, because the repo
  is *named* `kan-tools/day`. REQ-3's actual requirement is that day does not
  stamp its own name on every line — an anchor property — and that is what is
  asserted now, against a fixture deliberately named `kan-tools/day` so the test
  runs against the string that falsified its predecessor.

- **AC-1's criterion invited the defect it was meant to prevent.** "Nine
  distinct outputs" is satisfied by nine distinct *wrong* outputs, and three
  mutations walked straight through it: off-sequence findings wearing the
  unrecorded glyph, `behind` rendered as `ahead`, and the setup line naming a
  subject that resolves nothing. Distinctness is cheap; the states are now
  asserted by content, per style, with distinctness on top.

Two requirements gained scope from the same review, and are built:

- **The footer must fit the terminal.** Measured at ~112 columns against a
  status bar Claude Code documents as width-limited, on a rubric that
  standardizes an 80-column capture. `COLUMNS` is set for the status-line
  command (v2.1.153+) and was read by nothing. The hook now renders several
  width variants and the status line picks; elision is visible, and the two
  caveats are pinned against it.

- **`DAY_FOOTER` must work where a person sets it.** It resolved at hook time
  and was baked into the cache, so `DAY_FOOTER=plain day status-line` did
  nothing while README and CONVENTIONS both documented it as an override. Same
  fix: the choice moves to where the status line runs.

## Out of Scope

- **The session's signing identity being *set*.** This footer *reports* the
  active identity; making a harness session mint and use its own is day#186, and
  it is blocked on kan#121 because a fresh identity under `TrustBase::Solo`
  reads an empty log.
- **The broader rendered-output pass.** day#172 covers day's other surfaces and
  kan#199 the short-CID form; the footer should not invent a CID rendering that
  kan#198 is still deciding.
- **`day hook session-start`'s injected context.** Its reader is a model, and
  day#172 records that as the exception to the readability rules. The footer's
  reader is a human; the two must not be conflated.
- **Making the cache anything but opaque.** The renderer changes; `.day/` stays a
  rendered string that is never read to decide anything.
