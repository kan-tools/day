#!/usr/bin/env python3
"""Account for every commit on a branch under the demonstration rule.

`CLAUDE.md` says a commit closing a finding carries a `Demonstrated-by:` trailer.
Whether a branch actually does that is a fact about the branch, and it was being
written down **by hand** — a table in `.design/verification-that-can-fail.md`
counting commits and exemptions.

It was wrong three rounds running: first omitting the commit that introduces the
rule, then miscounting after the omission was fixed, then miscounting again after
that. Each round corrected the numbers and left the mechanism, which is the thing
`CLAUDE.md` already has a rule about:

    Generate expectation tables from a measurement run, then review them. The
    migration expectations were written from reasoning: eight rows, five wrong.

So this counts. Every commit lands in exactly one bucket:

  demonstrated   carries a `Demonstrated-by:` trailer that PARSES
  exempt         states a reason, in a `No trailer:` paragraph
  unaccounted    neither. THE FAILURE. A commit that claims no demonstration and
                 gives no reason — including a docs-only one, because this repo
                 executes its own documentation and "it is only prose" has been
                 the wrong guess twice.

Only `unaccounted` is a verdict; the other two are a census. Whether an
exemption is *true* is a judgement a script cannot make — see
`tests/harness_honesty.rs`, which asserts the count of unaccounted commits is
zero and leaves the reasons to review.

Usage:  scripts/demonstration-census.py [<base>..<head>]   (default: main..HEAD)
"""
from __future__ import annotations

import re
import subprocess
import sys

TRAILER_RE = re.compile(
    r"^Demonstrated-by:\s+revert=HEAD\s+tests=(\S+)\s+outcome=DEMONSTRATED\s*$",
    re.MULTILINE,
)
EXEMPTION_RE = re.compile(r"^No trailer:", re.MULTILINE)

# **Accounting for an earlier commit by APPENDING, because that is how this
# project supersedes anything.**
#
# A commit's message is immutable once pushed, and the only way to correct one
# is to rewrite history — which is precisely what kan refuses to do to a claim
# and what `CLAUDE.md` says day does not do to a subject: *"day never retracts
# or rejects. Superseding is done by appending, the same way kan does it."* A
# census that can only be satisfied by rebasing therefore demands, as its remedy,
# the one operation this repo's whole model is built to avoid.
#
# It arrived the obvious way: an exemption paragraph opening `No trailer,` with
# a comma instead of a colon. One character, a correct catch, and a remedy out
# of proportion to it.
#
# So a LATER commit may account for an earlier one by naming its sha and giving
# the reason. Deliberately narrow, because this is an escape hatch and an
# unbounded one is just the rule switched off:
#
#   - the sha must be given and must resolve to a commit IN THE SPAN, so this
#     cannot silently absolve something outside the range under review;
#   - a reason must follow on the same line — the same bargain `No trailer:`
#     and `kan-read-may-degrade:` make, and the reason is what review reads;
#   - it accounts as `exempt`, never as `demonstrated`. Appending a sentence is
#     not running the tool, and collapsing those two would let a commit claim a
#     demonstration it never performed.
ACCOUNTS_FOR_RE = re.compile(
    r"^Accounts-for:\s+([0-9a-f]{7,40})\s+(\S.*)$", re.MULTILINE
)

# **There is no `prose` bucket, and that is the third answer to this question.**
#
# The first version exempted a commit whose files were all `.md` or `.tsv`. The
# second narrowed that to an allowlist of paths "no test reads". Both were
# guesses about which files are invertible, and both were wrong: this repo
# EXECUTES the examples in `README.md`, `docs/CONVENTIONS.md` and `skills/*/SKILL.md`
# (`tests/documented_invocations.rs`), reads `CLAUDE.md` (`tests/plugin.rs`), and
# reads every `.md` under `docs/` by joining the directory — so the allowlist's
# own premise could not even be scanned for reliably, and the scan written to
# check it flagged five scratch fixtures instead. day#83 is a bug in prose.
#
# So the classifier stops guessing. A commit either demonstrates, or states a
# reason. "This is only a design document" is a perfectly good reason and costs
# one line; what it no longer does is get inferred from a file extension by a
# check that has been wrong every time it tried.

OK, UNACCOUNTED, COULD_NOT_CHECK, NOTHING_TO_CHECK = 0, 1, 2, 3


class CouldNotCheck(Exception):
    pass


def git(*args: str) -> str:
    r = subprocess.run(["git", *args], capture_output=True, text=True)
    if r.returncode != 0:
        # **Not an uncaught exception.** A `CalledProcessError` exits 1, which is
        # this script's code for "a commit is unaccounted" — so a git failure was
        # reported as a substantive finding about a commit that does not exist.
        # It fired the first time this ran outside the author's machine:
        # `actions/checkout` creates no local `main`, so `main..HEAD` exited 128
        # and CI accused a phantom commit. Could-not-check reported as
        # checked-and-found-a-defect, in the check written to end that.
        raise CouldNotCheck(f"git {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


def accounted_elsewhere(commits: list[str]) -> dict[str, str]:
    """sha -> the reason a later commit in the span gave for it.

    Full shas, resolved from whatever abbreviation was written, so `cfa9247`
    and its 40-character form are the same key. A sha naming something outside
    the span is ignored rather than honoured: an `Accounts-for:` that reaches
    past the range under review would let a branch absolve commits nobody is
    looking at.
    """
    in_span = set(commits)
    out: dict[str, str] = {}
    for sha in commits:
        for named, reason in ACCOUNTS_FOR_RE.findall(git("log", "-1", "--format=%B", sha)):
            try:
                full = git("rev-parse", f"{named}^{{commit}}").strip()
            except CouldNotCheck:
                continue
            if full in in_span and full != sha:
                out[full] = reason.strip()
    return out


def classify(sha: str, accounted: dict[str, str] | None = None) -> tuple[str, str]:
    body = git("log", "-1", "--format=%B", sha)
    subject = git("log", "-1", "--format=%s", sha).strip()
    if TRAILER_RE.search(body):
        return "demonstrated", subject
    if EXEMPTION_RE.search(body):
        return "exempt", subject
    # Accounted for by a later commit that named it. Never `demonstrated` —
    # appending a sentence is not running the tool.
    if accounted and sha in accounted:
        return "exempt", subject
    return "unaccounted", subject


def resolve_span(argv: list[str]) -> tuple[str, list[str]]:
    """(description, commits). Raises CouldNotCheck when the range is unknowable.

    The default is the merge base with `main` — resolved through `origin/main`
    first, because a CI checkout has the remote ref and no local branch.
    """
    if argv:
        span = argv[0]
        return span, git("rev-list", "--reverse", "--no-merges", span).split()
    for base_ref in ("refs/remotes/origin/main", "main"):
        # The whole attempt is guarded, not just the `rev-parse`. A ref can
        # resolve and still share no history with HEAD, and `merge-base` then
        # fails — which propagated straight out of a loop whose shape says "try
        # both", so the local `main` fallback was never reached.
        try:
            git("rev-parse", "--verify", f"{base_ref}^{{commit}}")
            base = git("merge-base", base_ref, "HEAD").strip()
            commits = git("rev-list", "--reverse", "--no-merges", f"{base}..HEAD").split()
        except CouldNotCheck:
            continue
        return f"{base[:7]}..HEAD", commits
    raise CouldNotCheck(
        "no `main` or `origin/main` to take a merge base from, so there is no "
        "range to account for"
    )


def main() -> int:
    try:
        span, shas = resolve_span(sys.argv[1:])
    except CouldNotCheck as e:
        print(f"COULD-NOT-CHECK: {e}")
        return COULD_NOT_CHECK

    if not shas:
        # Distinct from could-not-check, and from clean. On `main` after a merge
        # the range is legitimately empty; sharing a code with either of the
        # others made the check impossible to pass there, which would have
        # blocked the next release the first time anyone ran the suite on main.
        print(f"NOTHING-TO-CHECK: no commits in {span}")
        return NOTHING_TO_CHECK

    buckets: dict[str, list[str]] = {
        "demonstrated": [],
        "exempt": [],
        "unaccounted": [],
    }
    try:
        accounted = accounted_elsewhere(shas)
        for sha in shas:
            bucket, subject = classify(sha, accounted)
            note = f"  [accounted later: {accounted[sha]}]" if sha in accounted else ""
            buckets[bucket].append(f"{sha[:7]} {subject}{note}")
    except CouldNotCheck as e:
        print(f"COULD-NOT-CHECK: {e}")
        return COULD_NOT_CHECK

    print(f"span: {span}")
    print("| bucket | count |\n| --- | --- |")
    for name in ("demonstrated", "exempt", "unaccounted"):
        print(f"| {name} | {len(buckets[name])} |")
    print(f"| **total** | **{len(shas)}** |")
    for name in ("demonstrated", "exempt", "unaccounted"):
        if buckets[name]:
            print(f"\n{name}:")
            for line in buckets[name]:
                print(f"  {line}")

    if buckets["unaccounted"]:
        print(
            "\nUNACCOUNTED: these commits carry no demonstration and state no "
            "reason. A docs-only commit needs one too — this repo executes its "
            "own documentation."
        )
        return UNACCOUNTED
    return OK


if __name__ == "__main__":
    sys.exit(main())
