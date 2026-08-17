#!/bin/sh
# Cut v0.13 from the exact SHA already qualified by every required workflow.

set -eu

die() { printf '%s\n' "error: $*" >&2; exit 1; }

[ "$#" -eq 1 ] || die "usage: $0 v0.13.0-beta.1"
tag="$1"
[ "$tag" = "v0.13.0-beta.1" ] || die "this exact-candidate cutter only serves v0.13.0-beta.1"

for command in git cargo gh kan; do
  command -v "$command" >/dev/null 2>&1 || die "$command is required"
done

[ -z "$(git status --porcelain)" ] || die "working tree is dirty; commit first"
[ "$(git branch --show-current 2>/dev/null || true)" = "main" ] \
  || die "v0.13 must be cut from main"
candidate="$(git rev-parse HEAD)"
printf '%s\n' "$candidate" | grep -Eq '^[0-9a-f]{40}$' || die "HEAD is not a full commit SHA"

git tag --list "$tag" | grep -q . && die "tag $tag already exists"

expectations="tests/fixtures/migration-expectations.tsv"
rows="$(awk -v t="$tag" '$1 == t { count++ } END { print count + 0 }' "$expectations")"
[ "$rows" -eq 1 ] || die "$expectations must contain exactly one prequalified row for $tag"
corpus="tests/fixtures/block-corpus/$tag.jsonl"
[ -s "$corpus" ] || die "$corpus must be generated and committed before qualification"

cargo run -p xtask -- release verify-candidate-v013 "$candidate"

for issue in 93 143 152 193 195 204; do
  state="$(gh issue view "$issue" --repo kan-tools/day --json state --jq .state)" \
    || die "could not read issue #$issue state"
  [ "$state" = "CLOSED" ] || die "issue #$issue is not closed"
  closing_prs="$(gh issue view "$issue" --repo kan-tools/day \
    --json closedByPullRequestsReferences --jq '.closedByPullRequestsReferences[]?.number')" \
    || die "could not read issue #$issue closing pull requests"
  merged=""
  for pr in $closing_prs; do
    merged_at="$(gh pr view "$pr" --repo kan-tools/day --json mergedAt --jq .mergedAt)" \
      || continue
    [ -n "$merged_at" ] && [ "$merged_at" != "null" ] && { merged="$pr"; break; }
  done
  [ -n "$merged" ] || die "issue #$issue has no merged closing pull request"
done

printf 'What shipped, verified at candidate %s. Finish with Ctrl-D:\n' "$candidate" >&2
notes="$(cat)"
[ -n "$notes" ] || die "a release claim with no text is not a record"
cid="$(kan result release "$tag candidate $candidate — $notes")" \
  || die "recording the release claim failed; nothing has been tagged"
printf 'recorded %s\n' "$cid"

# No source-producing command occurs after qualification. The tag names the
# same commit all workflow runs and evidence bundles addressed.
git tag -a "$tag" -m "$tag"
[ "$(git rev-list -n 1 "$tag")" = "$candidate" ] || die "tag target changed unexpectedly"
printf 'Tagged exact candidate %s. Push only the tag:\n\n    git push origin %s\n' "$candidate" "$tag"
