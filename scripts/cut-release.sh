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

# --- 1. the tree is what it says it is -------------------------------------

[ -z "$(git status --porcelain)" ] || die "working tree is dirty; commit or stash first"

branch="$(git branch --show-current 2>/dev/null || true)"
[ "$branch" = "main" ] || die "releases are cut from main; you are on '${branch:-a detached HEAD}'"

# An `if` rather than `... | grep -q . && die`. That form is safe under `set -e`
# (the left side of an AND-OR list is exempt from errexit) but only obviously so
# to someone who has checked, and a release script is the wrong place to make a
# reader re-derive a shell subtlety.
if git tag --list "$tag" | grep -q .; then
  die "tag $tag already exists locally"
fi

cargo_version="$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')"
[ "${tag#v}" = "$cargo_version" ] \
  || die "tag $tag does not match Cargo.toml version $cargo_version — bump one of them"

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

# --- 4. RECORD THE RELEASE, BEFORE THE TAG EXISTS ----------------------------
#
# Before, deliberately. A claim recorded after the tag is a claim that can be
# forgotten between the two, which is precisely what happened twice. Recorded
# first, the failure mode inverts: a claim with no tag is loud (day's own
# `assess docs` reports "a boundary nobody cut") where a tag with no claim was
# silent until somebody happened to look.

printf 'Recording the release claim for %s.\n' "$tag"
printf 'What shipped, verified against the artifact (empty line to finish):\n' >&2

notes="$(cat)"
[ -n "$notes" ] || die "a release claim with no text is not a record; aborting"

cid="$(kan result release "$tag — $notes")" \
  || die "recording the release claim failed; nothing has been tagged"
printf 'recorded %s\n' "$cid"

# --- 5. only now, the tag ----------------------------------------------------

git tag -a "$tag" -m "$tag"
printf '\nTagged %s locally. Nothing has been pushed.\n' "$tag"
printf 'Push it yourself when ready:\n\n    git push origin %s\n\n' "$tag"
printf 'Then verify against the ARTIFACT, not the workflow exit code:\n'
printf '  cargo install day --version %s --locked --root /tmp/day-verify\n' "${tag#v}"
printf '  /tmp/day-verify/bin/day doctor\n'
