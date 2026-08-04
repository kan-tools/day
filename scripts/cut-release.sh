#!/bin/sh
# Cut a release: verify, record, tag. In that order, as one step.
#
# WHY THIS EXISTS, and why it is a script rather than a CI check.
#
# v0.7.0-beta.3 and v0.8.0-beta.1 were both published without a `release` claim.
# Neither was noticed for a day. The debrief (kan subject `process-model`) found
# the cause, and it was not carelessness: at the beta.3 boundary the release
# ceremony DID run, and was spent entirely on the previous release's paperwork,
# because that was the part with a gate attached. What was mechanized got done;
# what was ritual got dropped, at exactly the moment the gate consumed the
# attention the ritual needed.
#
# The obvious response was a CI gate. IT IS NOT BUILDABLE. `.kan/` is gitignored
# and this repo publishes no `.claims/` tree, so a workflow checking out the
# repo cannot see the log at all. A CI step asserting "a release claim exists"
# would be a check with nothing to check — green forever, for the wrong reason,
# which is the exact defect class the milestone that produced this script was
# about. Better no gate than a gate that cannot fail.
#
# So the gate goes where the log actually is: here, on the machine cutting the
# tag. This is day#101's rule applied to a process instead of to code — do not
# add a check beside the step, make the step impossible to do without it. The
# claim is not something you remember after tagging; it is part of tagging.
#
# Usage:  scripts/cut-release.sh v0.9.0-beta.1
#
# Refuses rather than proceeds on anything it cannot verify. Nothing here is
# advisory: `day` is advisory to an AGENT mid-flow, which is a different thing
# from a human deliberately publishing an artifact (the same distinction
# docs/ROADMAP.md draws for `day plugin install`).

set -eu

die() { printf '%s\n' "error: $*" >&2; exit 1; }

[ $# -eq 1 ] || die "usage: $0 <tag>   e.g. $0 v0.9.0-beta.1"
tag="$1"

case "$tag" in
  v*.*.*) ;;
  *) die "tag must look like v<major>.<minor>.<patch>[-pre]; got '$tag'" ;;
esac

command -v kan >/dev/null 2>&1 || die "kan is not on PATH; the release claim cannot be recorded"
command -v day >/dev/null 2>&1 || die "day is not on PATH; run 'cargo install --path .' first"
# Guarded like the others. Without this, a missing jq exits under `set -e` with
# no message at all, mid-script, after the checks above have passed.
command -v jq  >/dev/null 2>&1 || die "jq is not on PATH; it is needed to read the Cargo.toml version"

# --- 1. the tree is what it says it is -------------------------------------

[ -z "$(git status --porcelain)" ] || die "working tree is dirty; commit or stash first"

branch="$(git branch --show-current 2>/dev/null || true)"
[ "$branch" = "main" ] || die "releases are cut from main; you are on '${branch:-a detached HEAD}'"

# A stale local main is still `main`. Compare against the remote rather than
# trusting the branch name — tagging a commit the remote does not have produces
# a release nobody can check out. Read-only: `ls-remote` fetches nothing.
#
# WHAT THIS PROVES NARROWED IN v0.11, and it is worth stating rather than
# leaving to be re-derived. Step 4b below MAKES A COMMIT — the migration-matrix
# row for the tag being cut — so the commit that ends up tagged is by
# construction one the remote has never seen. This check therefore now proves
# that the tagged commit's PARENT is on origin, not the tagged commit itself.
# The residual risk is one commit this script wrote and printed, and the closing
# instruction pushes `main` and the tag together so the two cannot separate.
# That is a real weakening of a guard that was itself a review finding, taken
# deliberately in exchange for the tag containing its own expectation row.
#
# THE FIRST VERSION OF THIS COULD NOT FAIL. It was
#   remote_main="$(git ls-remote origin ... 2>/dev/null | cut -f1)"
# and a pipeline's status is its LAST command's, so a failed `ls-remote`
# (offline, auth, no origin) left the variable empty, the `-n` test skipped the
# check, and the script proceeded. Could-not-check reported as
# checked-and-clean, in a release gate, in the commit that cites that rule.
#
# The three cases are now distinguished, because they want different answers:
# no `origin` at all is a legitimate skip and says so; a reachable origin is
# compared; an origin that cannot be reached REFUSES, because at that point the
# script does not know whether the tag would be based on a stale commit.
# FOUR cases, and each of the previous two versions got a different one wrong.
# Round 1 used `2>/dev/null | cut`, so a pipeline's status is `cut`'s and the
# check COULD NOT FAIL. Round 2 used `2>&1`, which folded stderr INTO the value
# being compared — an ordinary first-connect SSH warning ("Permanently added
# 'github.com' …") became part of the sha and the script refused a correct
# release. Could-not-fail replaced by cannot-succeed.
#
# stdout and stderr are therefore captured SEPARATELY, via a temp file for
# stderr, so the compared value is only ever the sha.
if git remote get-url origin >/dev/null 2>&1; then
  ls_remote_err="$(mktemp)"
  if remote_out="$(git ls-remote origin refs/heads/main 2>"$ls_remote_err")"; then
    remote_main="$(printf '%s' "$remote_out" | cut -f1)"
    rm -f "$ls_remote_err"
    if [ -z "$remote_main" ]; then
      # Reachable, but the branch is not there. Not a skip: the script cannot
      # confirm HEAD is pushed, and silently proceeding is the could-not-check
      # -reported-as-clean shape this block exists to remove.
      die "origin is reachable but has no refs/heads/main; verify by hand that this commit is pushed"
    fi
    if [ "$remote_main" != "$(git rev-parse HEAD)" ]; then
      die "local main is not origin/main ($remote_main); pull or push first"
    fi
  else
    err="$(cat "$ls_remote_err" 2>/dev/null)"
    rm -f "$ls_remote_err"
    die "could not reach origin to compare main: ${err:-no detail}
re-run when the remote is reachable, or verify by hand that HEAD is pushed"
  fi
else
  printf 'note: no `origin` remote configured; skipping the origin/main check.\n' >&2
fi

# An `if` rather than `... | grep -q . && die`. That form is safe under `set -e`
# (the left side of an AND-OR list is exempt from errexit) but only obviously so
# to someone who has checked, and a release script is the wrong place to make a
# reader re-derive a shell subtlety.
if git tag --list "$tag" | grep -q .; then
  die "tag $tag already exists locally"
fi

# Same pipeline-status class as above: a failed `cargo metadata` would leave
# this empty and the script would die naming the wrong cause ("tag does not
# match version ''") instead of the real one.
if ! cargo_meta="$(cargo metadata --no-deps --format-version=1)"; then
  die "could not read cargo metadata; fix the manifest before releasing"
fi
cargo_version="$(printf '%s' "$cargo_meta" | jq -r '.packages[0].version')"
[ -n "$cargo_version" ] && [ "$cargo_version" != "null" ] \
  || die "could not read a version out of cargo metadata"
[ "${tag#v}" = "$cargo_version" ] \
  || die "tag $tag does not match Cargo.toml version $cargo_version — bump one of them"

# --- 1b. every EARLIER release already has its expectation row ---------------
#
# Braces to step 4b's belt (day#118). Step 4b measures and commits this release's
# row, so in the normal case this check has nothing to find; it exists for the
# case where a row was lost — reverted, dropped in a rebase, or written by a
# version of this script that did not do 4b. The alternative is where that
# absence used to surface: a red `migration-matrix` on somebody's tag push a
# whole release later, about a version they did not ship, needing a historical
# binary to diagnose. This lands it here instead, before anything is built.
#
# Deliberately BEFORE the ten-minute verify block, so the cheapest failure is
# also the earliest.

expectations="tests/fixtures/migration-expectations.tsv"
[ -f "$expectations" ] || die "$expectations is missing; the migration matrix cannot be checked"

missing=""
for released in $(git tag --list 'v*.*.*'); do
  # `awk`'s exit status, not a pipeline's: the pipeline-status defect this
  # script already carries two comments about is one `| grep -q` away here.
  if ! awk -v t="$released" '$1 == t { found = 1 } END { exit !found }' "$expectations"; then
    missing="$missing $released"
  fi
done
if [ -n "$missing" ]; then
  die "these released tags have no row in $expectations:$missing

A row is measured, never assumed. For each one:
    git worktree add --detach /tmp/reader <tag> && (cd /tmp/reader && cargo build --release)
    scripts/run-migration-cell.sh /tmp/reader/target/release/day
and append '<tag><TAB><outcome>' to $expectations."
fi

# --- 2. it builds and passes -------------------------------------------------
#
# Re-run here rather than trusting a green CI badge: the point of this script is
# that the release is one verified step, and a cross-workflow status is a
# different run of a different tree.

cargo build --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check

# --- 3. the docs match, per day's own assessment -----------------------------

day assess docs || die "'day assess docs' failed; fix the docs before releasing"

# --- 3b. MEASURE THIS RELEASE'S MIGRATION ROW, BEFORE THE TAG EXISTS ---------
#
# day#118. Adding a row used to be an ungated ritual, done by hand after the
# tag, and it was dropped three releases running — with a failure mode worse
# than "a step gets forgotten": a version is excluded from the matrix on the tag
# push that releases it, so its missing row cannot fail until the NEXT release.
# The absence was undetectable while anyone was looking at it.
#
# So the row is measured here and committed BEFORE the tag, which means the
# tagged tree contains its own row and `migration-matrix.yml` no longer needs to
# exclude the tag being released. One invariant, no window: every released tag
# has a measured row.
#
# "A version cannot be a historical reader of its own release" is still true and
# is not what this contradicts. This cell asks a well-defined question — what
# does the binary being released do with the block shapes this commit writes —
# and the next release re-asks it against THAT commit's corpus. If the shapes
# moved in between, the matrix reports that the blast radius moved, which is the
# failure it exists to produce rather than one it should be deferring.
#
# Release profile, matching what `migration-matrix.yml` builds for every other
# cell. Measuring the debug binary would be cheaper and would introduce an
# assumption (that the classification is profile-independent) whose failure mode
# is a red matrix on the next release; building it removes the assumption
# instead of stating it.

printf 'Measuring the migration-matrix row for %s.\n' "$tag"
cargo build --release --bin day

if ! outcome="$(scripts/run-migration-cell.sh target/release/day)"; then
  die "run-migration-cell.sh failed for the release binary; nothing has been tagged.
That is a harness fault, not an outcome — do not record a row for it."
fi

case "$outcome" in
  refused-honestly|silently-widened|protocol-mismatch) ;;
  errored|unbuildable)
    # The tsv's own header says a limit of the harness must never be filed as a
    # fact about a released version, and `errored` from a binary this script
    # just built and tested is a harness fault by elimination.
    die "the migration cell reported '$outcome' for the binary just built.
Nothing has been tagged. Diagnose it before releasing — recording this row
would file a broken invocation as a characterization of $tag." ;;
  *)
    die "the migration cell reported an unrecognised outcome '$outcome'.
Nothing has been tagged." ;;
esac

printf '%s\t%s\n' "$tag" "$outcome" >> "$expectations"
git add "$expectations"
git commit -q -m "migration matrix: the measured row for $tag" -m \
"Measured by scripts/cut-release.sh against the release binary built from this
tree, before the tag. day#118: the row used to be added by hand after the tag,
where its absence could not fail until the next release."
printf 'recorded row: %s\t%s (committed)\n' "$tag" "$outcome"

# --- 4. RECORD THE RELEASE, BEFORE THE TAG EXISTS ----------------------------
#
# Before, deliberately. A claim recorded after the tag is a claim that can be
# forgotten between the two, which is precisely what happened twice. Recorded
# first, the failure mode inverts: a claim with no tag is loud (day's own
# `assess docs` reports "a boundary nobody cut") where a tag with no claim was
# silent until somebody happened to look.

printf 'Recording the release claim for %s.\n' "$tag"
# `cat` reads to EOF, so the prompt must say EOF and not "empty line" — the
# first version said the latter, and a blank line does not terminate it. Stating
# the key sequence because "EOF" is not what a person types.
printf 'What shipped, verified against the artifact.\n' >&2
printf 'Finish with Ctrl-D on a blank line:\n' >&2

notes="$(cat)"
[ -n "$notes" ] || die "a release claim with no text is not a record; aborting"

cid="$(kan result release "$tag — $notes")" \
  || die "recording the release claim failed; nothing has been tagged"
printf 'recorded %s\n' "$cid"

# --- 5. only now, the tag ----------------------------------------------------

git tag -a "$tag" -m "$tag"
printf '\nTagged %s locally. Nothing has been pushed.\n' "$tag"
# Both, in one command, deliberately. The tagged commit is the row commit this
# script made in step 3b, so it is not on origin; pushing the tag alone would
# publish a tag whose commit the remote does not have.
printf 'Push the branch and the tag together:\n\n    git push origin main %s\n\n' "$tag"
printf 'Then verify against the ARTIFACT, not the workflow exit code:\n'
printf '  cargo install day --version %s --locked --root /tmp/day-verify\n' "${tag#v}"
printf '  /tmp/day-verify/bin/day doctor\n'
