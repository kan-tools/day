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
  prose          changes only prose (`.md`) — there is no behaviour to invert
  unaccounted    none of the above. THE FAILURE. A commit that changed code,
                 claims no demonstration, and gives no reason.

Only `unaccounted` is a verdict; the other three are a census. Whether an
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


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], capture_output=True, text=True, check=True
    ).stdout


def classify(sha: str) -> tuple[str, str]:
    body = git("log", "-1", "--format=%B", sha)
    subject = git("log", "-1", "--format=%s", sha).strip()
    if TRAILER_RE.search(body):
        return "demonstrated", subject
    if EXEMPTION_RE.search(body):
        return "exempt", subject
    files = [f for f in git("show", "--stat=200", "--format=", "--name-only", sha).split() if f]
    if files and all(f.endswith(".md") or f.endswith(".tsv") for f in files):
        return "prose", subject
    return "unaccounted", subject


def main() -> int:
    span = sys.argv[1] if len(sys.argv) > 1 else "main..HEAD"
    shas = git("rev-list", "--reverse", "--no-merges", span).split()
    if not shas:
        # Could-not-check, said plainly AND distinguishably. An empty range means
        # the census has nothing to be complete about, which is not the same as a
        # clean branch.
        #
        # Exit 2, not 1, and that is the point: a caller has to tell the two
        # apart, and the first version left it to tell them apart by grepping the
        # output for "could not check" — which a COMMIT SUBJECT on this very
        # branch contains ("a mutation run against a red baseline could not
        # check"). Keying on the absence of a phrase, in the check that exists to
        # stop hand-written evidence. Third occurrence in this milestone.
        print(f"COULD-NOT-CHECK: no commits in {span}")
        return 2

    buckets: dict[str, list[str]] = {
        "demonstrated": [],
        "exempt": [],
        "prose": [],
        "unaccounted": [],
    }
    for sha in shas:
        bucket, subject = classify(sha)
        buckets[bucket].append(f"{sha[:7]} {subject}")

    print(f"| bucket | count |\n| --- | --- |")
    for name in ("demonstrated", "exempt", "prose", "unaccounted"):
        print(f"| {name} | {len(buckets[name])} |")
    print(f"| **total** | **{len(shas)}** |")
    for name in ("demonstrated", "exempt", "prose", "unaccounted"):
        if buckets[name]:
            print(f"\n{name}:")
            for line in buckets[name]:
                print(f"  {line}")

    if buckets["unaccounted"]:
        print(
            "\nUNACCOUNTED: these commits changed something other than prose, "
            "carry no demonstration, and state no reason."
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
