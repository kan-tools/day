#!/bin/sh
# Capture the harness footer under the standardized setup rubric v1 names
# (eval/legible-surface.rubric.v1.md), for scoring by an agent that sees only
# the capture.
#
# Usage: scripts/capture-footer.sh <outdir>
#
# Writes into <outdir>:
#   capture.txt        — the tmux pane, exactly as rendered
#   ground-truth.json  — the fixture's known state, DERIVED from the fixture
#                        (never hand-written; a hand-written expectation table
#                        is how this repo was wrong five rows out of eight)
#
# Exit codes, kept distinct because could-not-check outranks checked-and-clean:
#   0  capture written
#   2  a prerequisite is missing, or a derivation could not be made — NOT a
#      failed eval. The workflow translates this rather than reporting red.
set -eu

OUT="${1:?usage: scripts/capture-footer.sh <outdir>}"
mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

for tool in kan tmux git python3; do
  command -v "$tool" >/dev/null 2>&1 || {
    echo "capture-footer: $tool is not on PATH — cannot capture (this is not a failed eval)" >&2
    exit 2
  }
done
DAY_BIN="${DAY_BIN:-$(command -v day || true)}"
[ -n "$DAY_BIN" ] || {
  echo "capture-footer: day is not on PATH (set DAY_BIN) — cannot capture" >&2
  exit 2
}

# The fixture lives under the output directory rather than in a stray mktemp,
# so a run leaves nothing behind it does not name — the first version left a
# git repo, a kan log and a 0600 signing seed in /tmp permanently.
#
# **It holds a signing seed, so it must never be published.** Anything
# uploading results uploads `capture.txt` and `ground-truth.json` BY NAME,
# never this directory; `.github/workflows/legible-surface-eval.yml` says so
# at the step that would otherwise have done it.
FIXTURE="$OUT/fixture"
rm -rf "$FIXTURE"
mkdir -p "$FIXTURE"

# ---------------------------------------------------------------------------
# THE STANDARDIZED ENVIRONMENT APPLIES TO THE HOOK, NOT TO THE PANE.
#
# The style is decided and cached where the footer is RENDERED — the
# session-start hook — so setting LANG/TERM on the tmux pane controlled
# nothing: a `TERM=dumb` ambient shell produced a plain capture inside a pane
# declaring UTF-8, and the harness could not tell. On a CI runner that made
# the artifact attested as "rubric v1, detection path" possibly the forced
# plain rendering. The pane keeps the same values because the *width* is read
# there; both now carry the standardized environment.
# ---------------------------------------------------------------------------
STD_ENV="LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 TERM=xterm-256color"
# NO_COLOR and DAY_FOOTER must be ABSENT, not empty: an empty DAY_FOOTER falls
# through to detection, which is what rubric v1 wants, but an empty NO_COLOR
# is likewise ignored only because day checks for non-empty. Unsetting both
# states the intent rather than relying on that.
unset NO_COLOR DAY_FOOTER || true

# --- the fixture: a repo in a known, ADVERSARIAL state ----------------------
# Adversarial rather than convenient. The first fixture declared no atoms, had
# no upstream, was always the main checkout and rendered no tray — so five of
# the rubric's eight questions were answered by a footer that could not have
# got them wrong, against a 7-of-8 bar. A fixture that cannot fail is the
# vacuity this repo keeps recording, arriving in the thing built to measure it.
cd "$FIXTURE"

# An upstream, so ahead/behind is a real reading rather than "no upstream".
git init -q -b eval-branch upstream
git -C upstream -c user.email=eval@example.invalid -c user.name=eval \
  commit -q --allow-empty -m init
git clone -q upstream main-checkout
cd "$FIXTURE/main-checkout"
git config user.email eval@example.invalid
git config user.name eval
git remote set-url origin https://github.com/kan-tools/eval-fixture.git

# Ahead 1, behind 1 — asymmetric and non-zero, so a swapped mark is visible.
git commit -q --allow-empty -m local
git -C "$FIXTURE/upstream" -c user.email=eval@example.invalid -c user.name=eval \
  commit -q --allow-empty -m upstream
git remote set-url origin "$FIXTURE/upstream"
git fetch -q
git remote set-url origin https://github.com/kan-tools/eval-fixture.git

# What a real repo does with its tooling's workspaces, and what day's own
# repo does: gitignore them. Without this the fixture disagreed with itself —
# day reported dirty because `.kan/` was untracked while the ground truth,
# which excludes tooling artifacts, said clean. The disagreement was real and
# the fixture's fault: an operator does not call "kan initialized its log"
# a dirty working tree, and day should not be shown a repo no one would ship.
printf '.day/\n.kan/\n' > .gitignore
git add .gitignore && git commit -q -m "ignore tooling workspaces"

# A dirty tree, from the user's own file rather than from day's artifacts,
# and left UNCOMMITTED so the reading is genuine.
echo "uncommitted" > dirty.txt

# TWO atoms consistent with the evidence, so the position is genuinely
# ambiguous and Q1/Q2 have a wrong answer available.
env $STD_ENV kan observe --subject schema/witness "Witness probes for the eval fixture.

\`\`\`day-witness
{\"design-doc\": {\"path\": \".design/*.md\"}, \"code-change\": {\"path\": \"src/*.rs\"}}
\`\`\`
" >/dev/null
for slug in alpha beta; do
  env $STD_ENV kan observe --subject "atom/$slug" "The $slug atom.

\`\`\`day-atom
{\"in\": [\"design-doc\"], \"out\": [\"code-change\"]}
\`\`\`
" >/dev/null
done
mkdir -p .design && echo "# a design" > .design/thing.md
# Named paths, never `-A`: the whole point of the fixture is that `dirty.txt`
# stays uncommitted, and a blanket add would sweep it in and quietly make the
# dirtiness reading vacuous.
git add .design/thing.md && git commit -q -m "a design doc, so both atoms have their input"

env $STD_ENV kan identity role add director >/dev/null 2>&1 || true

# --- the footer, computed where it always is: the session-start hook --------
env $STD_ENV "$DAY_BIN" hook session-start >/dev/null

# --- ground truth, DERIVED — every field, or the run fails ------------------
# Six of nine fields were hand-written literals in a file whose own header
# forbids that. They are all read from the substrate now, and a field that
# cannot be derived exits 2 rather than being guessed: a ground truth that
# silently lies is worse than a capture nobody scores.
derive() {
  desc="$1"; value="$2"
  [ -n "$value" ] || { echo "capture-footer: could not derive $desc — refusing to hand-write it" >&2; exit 2; }
  printf '%s' "$value"
}

BRANCH="$(derive branch "$(git branch --show-current)")"
# Dirtiness EXCLUDING day's own artifacts, which is what day itself reports —
# computed the same way rather than from a `git status` that day has since
# dirtied by writing .day/.
DIRTY_FILES="$(git status --porcelain | wc -l | tr -d ' ')"
DIRTY="$([ "$DIRTY_FILES" -gt 0 ] && echo true || echo false)"
[ "$DIRTY" = "true" ] || {
  echo "capture-footer: the fixture's tree is CLEAN, so Q4's dirty reading cannot be wrong — a fixture that cannot fail answers for free" >&2
  exit 2
}
AHEAD="$(git rev-list --count '@{u}..HEAD' 2>/dev/null || echo "")"
BEHIND="$(git rev-list --count 'HEAD..@{u}' 2>/dev/null || echo "")"
AHEAD="$(derive ahead "$AHEAD")"
BEHIND="$(derive behind "$BEHIND")"
REPO="$(git remote get-url origin | sed -e 's#.*[/:]\([^/]*\/[^/]*\)$#\1#' -e 's#\.git$##')"
REPO="$(derive repo "$REPO")"
# The main checkout is where the common git dir's parent is — the same read
# day makes (RQ-7), so the ground truth cannot disagree with it by construction.
COMMON="$(cd "$(git rev-parse --git-common-dir)" && pwd)"
TOP="$(git rev-parse --show-toplevel)"
CHECKOUT="$([ "$(dirname "$COMMON")" = "$TOP" ] && echo main || echo worktree)"
ROLE="$(kan identity role list --json | python3 -c '
import json,sys
e=json.load(sys.stdin)
print(next((r["name"] for r in e.get("roles",[]) if r["did"]==e.get("active")), ""))
')"
ROLE="$(derive role "$ROLE")"
# The position, derived from the LOG rather than from day's reading of it —
# scoring day against day would make the eval circular. Both atoms have their
# input present and neither has produced its output, so both are in play.
ATOMS="$(kan show --all --json | python3 -c '
import json,sys
e=json.load(sys.stdin)
print(" ".join(sorted(s["subject"].split("/",1)[1] for s in e.get("subjects",[]) if s.get("subject","").startswith("atom/"))))
')"
ATOMS="$(derive atoms "$ATOMS")"
ATOM_COUNT="$(printf '%s' "$ATOMS" | wc -w | tr -d ' ')"
[ "$ATOM_COUNT" -ge 2 ] || {
  echo "capture-footer: the fixture must leave the position AMBIGUOUS (>=2 atoms), got $ATOM_COUNT — a fixture that cannot render \`atom?\` answers Q2 for free" >&2
  exit 2
}
WITHHELD="$(kan show --all --json | python3 -c 'import json,sys; print(json.load(sys.stdin).get("excluded_by_trust",0))')"

# Values go in as ARGUMENTS, not interpolated into the source: a shell
# `false` spliced into Python is a NameError, and interpolating a branch name
# would be an injection waiting for a branch with a quote in it.
python3 - "$OUT/ground-truth.json" "$ATOMS" "$REPO" "$BRANCH" "$DIRTY" \
  "$AHEAD" "$BEHIND" "$CHECKOUT" "$ROLE" "$WITHHELD" <<'PYEOF'
import json, sys
(out, atoms, repo, branch, dirty, ahead, behind, checkout, role, withheld) = sys.argv[1:11]
names = atoms.split()
json.dump(
    {
        "rubric": "v1",
        "position_atoms": names,
        "position_ambiguous": len(names) > 1,
        "repo": repo,
        "branch": branch,
        "dirty": dirty == "true",
        "ahead": int(ahead),
        "behind": int(behind),
        "checkout": checkout,
        "role": role,
        "withheld": int(withheld),
        # Q7 asks about warnings and had no field to be marked against, so it
        # could not be scored at all against a rubric that says every answer
        # is marked against the ground truth. This fixture produces none; the
        # field exists so "none" is a markable answer rather than a gap.
        "warnings": [],
    },
    open(out, "w"),
    indent=2,
    sort_keys=True,
)
PYEOF

# --- the capture: the standardized pane -------------------------------------
SESSION="day-eval-$$"
tmux new-session -d -s "$SESSION" -x 80 -y 24 \
  -e LANG=en_US.UTF-8 -e LC_ALL=en_US.UTF-8 -e TERM=xterm-256color \
  "cd '$FIXTURE/main-checkout' && '$DAY_BIN' status-line < /dev/null; sleep 60"
# Wait for the pane to actually render rather than assuming one second was
# enough: an empty capture at exit 0 is could-not-check reported as
# checked-and-clean, three lines below a header invoking that discipline.
i=0
while [ "$i" -lt 50 ]; do
  tmux capture-pane -t "$SESSION" -p > "$OUT/capture.txt" 2>/dev/null || true
  if [ -s "$OUT/capture.txt" ] && grep -q '[^[:space:]]' "$OUT/capture.txt"; then
    break
  fi
  i=$((i + 1))
  sleep 0.2
done
tmux kill-session -t "$SESSION" 2>/dev/null || true

grep -q '[^[:space:]]' "$OUT/capture.txt" 2>/dev/null || {
  echo "capture-footer: the pane rendered nothing within 10s — no capture to score (this is not a failed eval)" >&2
  exit 2
}

echo "capture written to $OUT/capture.txt"
echo "ground truth written to $OUT/ground-truth.json"
echo "next: score the capture against eval/legible-surface.rubric.v1.md;"
echo "a pass is attested with a kan result on eval/legible-surface (see the rubric)."
