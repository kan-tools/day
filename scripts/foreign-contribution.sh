#!/bin/sh
# Exits 0 iff someone other than <login> has opened an issue or pull request on
# this repository. Witness probe for `telos/v1.0`.
#
# The v1.0 bar is "a person who is not the author uses day on a project that is
# neither kan nor day". This is the weaker of the two halves it decomposes into
# -- filing a bug is not shipping with something -- but it is the half that is
# *structurally unfakeable by the author*, which is what makes it worth having.
# You cannot open an issue as somebody else.
#
# WHY THIS IS A SCRIPT AND NOT A `command` PROBE INLINE. Two independent
# reasons, both found by trying:
#
#   1. `gh search issues` exits 0 whether it finds thirty results or none, so
#      the exit status carries no information. The `--jq ... error(...)` form
#      below is what makes an empty result set exit non-zero.
#   2. day splits a probe's argv on whitespace and execs it directly -- there is
#      no shell, deliberately -- so an argument containing a space cannot be
#      expressed at all. The GitHub query needs one.
#
# The excluded login is an ARGUMENT rather than baked in, so the value lives in
# the `schema/witness` declaration where it is visible and greppable, instead of
# becoming another hand-written constant inside a file nobody re-reads.
#
# Failure modes and what they mean to day, stated because a positive command
# probe has only two outcomes and both must be honest:
#
#   exit 0        a third-party contribution exists   -> [MATERIAL]
#   exit non-zero it does not, OR gh could not answer -> [MISSING]
#
# Those two collapse, and the collapse is in the safe direction: an unreachable
# network, a missing `gh`, or a mistyped login (GitHub answers 422 for a user
# that does not exist) all report the telos NOT met, never met. day#137 is the
# same question in the direction where the collapse is dangerous.
#
# Verified in both directions, which is the only way to know a witness can be
# satisfied at all: excluding a real login that has not contributed here exits
# 0, and excluding the author exits 1. A first check used a NONEXISTENT login as
# its control, got the 422 above, and looked exactly like a script that always
# exits 1 -- a witness that can never be met, which is the mirror of the defect
# this milestone is about.

set -u

if [ $# -ne 1 ]; then
    echo "usage: $0 <github-login-to-exclude>" >&2
    exit 2
fi
login=$1

repo=$(gh repo view --json nameWithOwner --jq .nameWithOwner 2>/dev/null) || {
    echo "could not determine the repository from gh" >&2
    exit 2
}

# `error(...)` on an empty result set is what turns "found nothing" into a
# non-zero exit; without it gh reports success and prints nothing.
gh api -X GET search/issues \
    -f q="repo:$repo -author:$login" \
    --jq '.items | if length == 0 then error("no third-party contribution") else .[0].number end' \
    >/dev/null 2>&1
