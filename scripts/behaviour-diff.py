#!/usr/bin/env python3
"""Run two builds of `day` over a fixture corpus and report what changed.

**The question the other two harnesses do not ask.** `mutate.py` asks whether
any test asserts a line. `revert-demo.py` asks whether the test written for a
finding fails when the finding returns. Neither asks *did this change alter
behaviour it was not meant to alter* -- and that is the question two cold
reviews answered, by hand, by building the pre-fix binary and diffing it.

Both times they found a regression the author had not: a fix that routed witness
shapes through one evaluator also, in the same line, dropped a declared scope and
re-read a subject name as a glob. The suite was green, clippy was clean, and the
`Demonstrated-by:` trailer was honest, because all three were asking a different
question.

Outcomes, never conflated, could-not-check outranking checked-and-clean:

  IDENTICAL           every fixture produced byte-identical output. Exit 0.
  CHANGED-AS-DECLARED every difference was named with --expect. Exit 0.
  CHANGED-UNEXPLAINED at least one difference nobody declared. THE finding.
  BASE-DID-NOT-BUILD  the comparison could not be made. Says nothing about
                      whether behaviour changed.
  CORPUS-EMPTY        no fixtures ran. A diff over nothing is IDENTICAL for the
                      wrong reason, which is the failure `capture-block-corpus`
                      had twice.

**The corpus is read from the directory, never listed here.** A hand-maintained
corpus is the defect class this repo spent v0.12 on: it does not fail when it
stops growing. `--expect-fixtures N` is still required, because a *derived* list
also silently shrinks if the glob breaks -- the list catches a missing member,
the count catches a broken reader, and neither substitutes for the other.

**No `--run`, ever.** Command probes execute, and one of day's own now makes a
network call. A corpus that shells out is a corpus that flakes, and a flaky
check gets switched off.

Usage:
  scripts/behaviour-diff.py [--since REV] [--expect FIXTURE:VERB]... [--expect-fixtures N]
"""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import shutil
import subprocess
import sys

IDENTICAL = "IDENTICAL"
CHANGED_AS_DECLARED = "CHANGED-AS-DECLARED"
CHANGED_UNEXPLAINED = "CHANGED-UNEXPLAINED"
BASE_DID_NOT_BUILD = "BASE-DID-NOT-BUILD"
CORPUS_EMPTY = "CORPUS-EMPTY"

class Unrunnable(Exception):
    """A fixture did not produce a verdict, so it cannot evidence agreement."""


ROOT = pathlib.Path(__file__).resolve().parent.parent
CORPUS = ROOT / "fixtures" / "behaviour"

# A kan stub answering only what a read-only day invocation asks. Deliberately
# tiny: the corpus supplies the log as one JSON file, so a fixture is reviewable
# in one read rather than assembled by a script nobody checks.
KAN_STUB = """#!/bin/sh
case "$1" in
  show)
    if [ "$2" = "--all" ]; then cat "$FIXTURE/log.json"; exit 0; fi
    printf '{"v":1,"subject":"%s","subjects":[],"claims":[],"inbound":[]}\\n' "$2"; exit 0 ;;
  identity) printf 'did:key:zFixtureAuthor\\n'; exit 0 ;;
  # `status --json` and `issues --json` are an envelope of subjects, the shape
  # `tests/common` builds. Answering them with prose is what made the first run
  # report IDENTICAL while checking nothing: day refused the output on BOTH
  # binaries, and two identical errors compare equal.
  status|issues) cat "$FIXTURE/status.json"; exit 0 ;;
  *) exit 0 ;;
esac
"""

# `git tag --list <pattern>` and `git ls-files -- <pathspec>` both put the
# pattern in $3, and `case` gives real glob matching -- the same shape
# `tests/assess_telos.rs` uses, and for the same reason: a stub that ignored the
# pattern would make a scoped probe and an unscoped one indistinguishable.
GIT_STUB = """#!/bin/sh
pattern="$3"
match() {
  for item in $1; do
    case "$item" in $pattern) printf '%s\\n' "$item" ;; esac
  done
}
case "$1" in
  tag)      match "$FIXTURE_TAGS" ;;
  ls-files) match "$FIXTURE_FILES" ;;
  rev-parse) printf '%s\\n' "$FIXTURE" ;;
  *) exit 0 ;;
esac
"""


def run(cmd: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=True, text=True, **kw)


def build_base(rev: str) -> pathlib.Path | None:
    """Build `day` at `rev` in a worktree, cached per rev.

    Cached because the base build is the whole cost of this check, and a check
    that is not nearly free is one people route around -- the condition
    `revert-demo.py` shipped under and the reason it stuck. Second run onward is
    seconds.
    """
    sha = run(["git", "rev-parse", rev], cwd=ROOT).stdout.strip()
    if not sha:
        return None
    cache = pathlib.Path(os.environ.get("TMPDIR", "/tmp")) / f"day-behaviour-{sha[:12]}"
    binary = cache / "target" / "debug" / "day"
    if binary.exists():
        return binary
    tree = cache / "tree"
    if not tree.exists():
        cache.mkdir(parents=True, exist_ok=True)
        r = run(["git", "worktree", "add", "--detach", str(tree), sha], cwd=ROOT)
        if r.returncode != 0:
            print(r.stderr.strip()[:400], file=sys.stderr)
            return None
    r = run(
        ["cargo", "build", "--bin", "day", "--target-dir", str(cache / "target")],
        cwd=tree,
    )
    if r.returncode != 0 or not binary.exists():
        print(r.stderr.strip()[-800:], file=sys.stderr)
        return None
    return binary


def fixtures(corpus: pathlib.Path) -> list[pathlib.Path]:
    if not corpus.is_dir():
        return []
    return sorted(p for p in corpus.iterdir() if (p / "case.json").is_file())


def observe(binary: pathlib.Path, fixture: pathlib.Path, work: pathlib.Path) -> dict:
    """Every declared invocation of one fixture, against one binary."""
    case = json.loads((fixture / "case.json").read_text())
    env = dict(os.environ)
    env["FIXTURE"] = str(fixture)
    env["FIXTURE_TAGS"] = " ".join(case.get("tags", []))
    env["FIXTURE_FILES"] = " ".join(case.get("tracked", []))
    env["DAY_KAN_BIN"] = str(work / "kan")
    env["DAY_GIT_BIN"] = str(work / "git")
    out = {}
    for verb in case["invocations"]:
        argv = [str(binary)] + verb.split()
        if "--run" in argv:
            raise SystemExit(f"{fixture.name}: --run is not allowed in the corpus")
        r = run(argv, cwd=work, env=env)
        # **A fixture that could not run is not a fixture that agreed.** Both
        # binaries erroring identically compares equal, so the diff reports
        # IDENTICAL -- clean, for the reason the whole harness exists to reject.
        # It happened on the first run: the kan stub answered `status --json`
        # with prose, day refused it on both sides, and two fixtures that reach
        # a real regression reported no difference.
        #
        # So a fixture must produce a verdict, not an error. Checked per
        # invocation rather than trusted, because the stub is the thing most
        # likely to rot and the failure is silent in the safe-looking direction.
        if r.returncode not in (0, 1) or "could not read" in r.stderr:
            raise Unrunnable(f"{fixture.name}:{verb} exited {r.returncode}: "
                             f"{(r.stderr or r.stdout).strip()[:200]}")
        out[verb] = f"exit={r.returncode}\n{r.stdout}"
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--since", default="HEAD~1")
    ap.add_argument("--expect", action="append", default=[],
                    help="FIXTURE:VERB whose output is expected to differ")
    ap.add_argument("--expect-fixtures", type=int, default=None)
    ap.add_argument("--corpus", default=None,
                    help="fixture directory (default fixtures/behaviour)")
    args = ap.parse_args()

    corpus = pathlib.Path(args.corpus) if args.corpus else CORPUS
    cases = fixtures(corpus)
    if args.expect_fixtures is not None and len(cases) != args.expect_fixtures:
        print(f"{CORPUS_EMPTY}: expected {args.expect_fixtures} fixture(s), found "
              f"{len(cases)}. A corpus that silently shrinks reports IDENTICAL "
              f"for the wrong reason.")
        return 2
    if not cases:
        print(f"{CORPUS_EMPTY}: no fixtures under {corpus}")
        return 2

    head = ROOT / "target" / "debug" / "day"
    if not head.exists():
        r = run(["cargo", "build", "--bin", "day"], cwd=ROOT)
        if r.returncode != 0:
            print(f"{BASE_DID_NOT_BUILD}: the current tree does not build.")
            return 2

    work = pathlib.Path(os.environ.get("TMPDIR", "/tmp")) / "day-behaviour-work"
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir(parents=True)
    for name, body in (("kan", KAN_STUB), ("git", GIT_STUB)):
        p = work / name
        p.write_text(body)
        p.chmod(0o755)

    # **The corpus is validated against the CURRENT binary before the base is
    # built.** Fail fast on a broken fixture rather than after paying for a
    # worktree build -- and, more importantly, this is the check that has to run
    # for the guard to mean anything, so it must not sit behind something that
    # can itself fail first.
    try:
        head_out = {fx: observe(head, fx, work) for fx in cases}
    except Unrunnable as e:
        print(f"{CORPUS_EMPTY}: {e}\n\nA fixture that errors compares equal "
              f"to itself, so this would otherwise have reported IDENTICAL "
              f"while checking nothing.")
        return 2

    base = build_base(args.since)
    if base is None:
        print(f"{BASE_DID_NOT_BUILD}: could not build `day` at {args.since}. "
              f"This says NOTHING about whether behaviour changed.")
        return 2

    declared = set(args.expect)
    changed, unexplained = [], []
    for fx in cases:
        try:
            before, after = observe(base, fx, work), head_out[fx]
        except Unrunnable as e:
            print(f"{CORPUS_EMPTY}: {e}\n\nA fixture that errors compares equal "
                  f"to itself, so this would otherwise have reported IDENTICAL "
                  f"while checking nothing.")
            return 2
        for verb in before:
            if before[verb] != after[verb]:
                key = f"{fx.name}:{verb}"
                changed.append(key)
                (unexplained, changed)[key in declared]  # noop, readability
                if key not in declared:
                    unexplained.append((key, before[verb], after[verb]))

    print(f"corpus: {len(cases)} fixture(s) against {args.since}")
    if not changed:
        print(IDENTICAL)
        return 0
    for key in changed:
        mark = "declared" if key in declared else "UNEXPLAINED"
        print(f"  {mark}: {key}")
    if not unexplained:
        print(CHANGED_AS_DECLARED)
        return 0
    for key, b, a in unexplained:
        print(f"\n--- {key}\nbefore:\n{b.rstrip()}\nafter:\n{a.rstrip()}")
    print(f"\n{CHANGED_UNEXPLAINED}: {len(unexplained)} change(s) nobody declared. "
          f"Declare them with --expect if intended; a fix that changes behaviour "
          f"it was not aimed at is the defect this exists to surface.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
