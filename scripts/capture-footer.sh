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
#   2  a prerequisite is missing (kan, tmux, day) — NOT a failed eval
set -eu

OUT="${1:?usage: scripts/capture-footer.sh <outdir>}"
mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

for tool in kan tmux git; do
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

# --- the fixture: a repo in a known state -----------------------------------
FIXTURE="$(mktemp -d)/eval-fixture"
mkdir -p "$FIXTURE"
cd "$FIXTURE"
git init -q -b eval-branch
git -c user.email=eval@example.invalid -c user.name=eval commit -q --allow-empty -m init
git remote add origin https://github.com/kan-tools/eval-fixture.git
echo "uncommitted" > dirty.txt   # a dirty tree, so Q4 has a definite answer

# A real kan log: a witness schema (so position is inferable rather than the
# setup state) and a declared role whose DID is active (so Q6 has an answer).
kan observe --subject schema/witness "Witness probes for the eval fixture.

\`\`\`day-witness
{\"design-doc\": {\"path\": \".design/*.md\"}}
\`\`\`
" >/dev/null
kan identity role add director >/dev/null 2>&1 || true

# --- the footer, computed where it always is: the session-start hook --------
"$DAY_BIN" hook session-start >/dev/null

# --- ground truth, derived from the fixture ---------------------------------
# The position is derived from the SUBSTRATE (the kan log), never from day's
# own reading of it — scoring day against day's answer would make the eval
# circular. This fixture declares no atoms, so the true position is that none
# is in play; a future fixture that declares atoms derives this differently.
ATOMS="$(kan show --all --json | python3 -c '
import json,sys
e=json.load(sys.stdin)
print(sum(1 for s in e.get("subjects",[]) if s.get("subject","").startswith("atom/")))
')"
[ "$ATOMS" = "0" ] || {
  echo "capture-footer: the fixture unexpectedly declares $ATOMS atoms — the derived position below would be wrong" >&2
  exit 2
}
BRANCH="$(git branch --show-current)"
DIRTY="$(test -n "$(git status --porcelain)" && echo true || echo false)"
ROLE="$(kan identity role list --json | python3 -c '
import json,sys
e=json.load(sys.stdin)
print(next((r["name"] for r in e.get("roles",[]) if r["did"]==e.get("active")), ""))
')"
cat > "$OUT/ground-truth.json" <<EOF
{
  "rubric": "v1",
  "position": "no atom in play",
  "position_ambiguous": false,
  "repo": "kan-tools/eval-fixture",
  "branch": "$BRANCH",
  "dirty": $DIRTY,
  "upstream": null,
  "checkout": "main",
  "role": "$ROLE",
  "withheld": 0
}
EOF

# --- the capture: the standardized pane -------------------------------------
SESSION="day-eval-$$"
tmux new-session -d -s "$SESSION" -x 80 -y 24 \
  -e LANG=en_US.UTF-8 -e TERM=xterm-256color -e NO_COLOR= -e DAY_FOOTER= \
  "cd '$FIXTURE' && '$DAY_BIN' status-line < /dev/null; sleep 60"
# Give the pane a moment to render, then take it exactly as shown.
sleep 1
tmux capture-pane -t "$SESSION" -p > "$OUT/capture.txt"
tmux kill-session -t "$SESSION"

echo "capture written to $OUT/capture.txt"
echo "ground truth written to $OUT/ground-truth.json"
echo "next: score the capture against eval/legible-surface.rubric.v1.md;"
echo "a pass is attested with a kan result on eval/legible-surface (see the rubric)."
