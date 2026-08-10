#!/usr/bin/env python3
"""One mutation, honestly reported. The shape day#90 asks for.

Distinguishes the outcomes that matter and never conflates them:
  CAUGHT            a test failed -> the mutation was detected
  SURVIVED          tests passed  -> nothing asserts this behaviour
  BASELINE-RED      the suite was ALREADY failing; nothing was mutated
  DID-NOT-COMPILE   the mutation was ill-formed; says NOTHING about coverage
  ANCHOR-MISSING    the text was not found; the mutation never happened

The exit code carries the same distinctions, because a scripted caller reads
nothing else. This harness printed them honestly and then exited 0 for all of
CAUGHT, SURVIVED, DID-NOT-COMPILE and ANCHOR-MISSING — so a loop over
mutations, or a CI step, read a survived mutation as green. That is the exact
rule this repo's practice states ("could-not-check outranks checked-and-clean
in the exit code... every tool in scripts/ reports its outcomes as distinct
named states"), violated by the tool the rule cites. The contract, matching
demonstration-census.py's shape:
  0  CAUGHT          checked, and the coverage is real
  1  SURVIVED        checked, and found the gap the caller asked about
  2  ANCHOR-MISSING / DID-NOT-COMPILE   could not check
  3  BASELINE-RED    could not check; the tree failed the precondition

Restores from a backup in a finally block, so an interrupt cannot leave a
mutated tree — the failure that left src/status.rs mutated earlier in this
session.

**BASELINE-RED is not a nicety** (day#114). Without it, a red suite reports every
mutation as CAUGHT — including ones that assert nothing — because a test failing
for an unrelated reason is indistinguishable from a test catching the mutation.
That happened here: two telos tests were already broken, a CID-hash mutation
reported CAUGHT, and only the obvious irrelevance of the failing names gave it
away. Had the mutation been in the same module as the pre-existing failure, the
false CAUGHT would have been invisible. It is the rule this harness exists to
enforce, missing from the harness: a run against a red baseline COULD NOT CHECK,
and reported the strongest possible result instead.
"""
import pathlib
import shutil
import subprocess
import sys
import tempfile

SUITE = ["cargo", "test", "--quiet", "--workspace", "--no-fail-fast"]


def failing_tests(out: str) -> list:
    return [l.strip() for l in out.splitlines() if l.strip().startswith("---- ")]


def main() -> int:
    path, anchor, replacement, name = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    p = pathlib.Path(path)
    original = p.read_text()

    if anchor not in original:
        print(f"{name}: ANCHOR-MISSING (the mutation never happened)")
        return 2

    mutated = original.replace(anchor, replacement, 1)
    if mutated == original:
        print(f"{name}: ANCHOR-MISSING (replacement identical to anchor)")
        return 2

    # The baseline, BEFORE the tree is touched. One extra suite run per
    # invocation, accepted deliberately: a mutation harness is not something you
    # run in a tight loop, and the alternative is an outcome vocabulary in which
    # "could not check" is spelled the same way as the strongest possible result.
    base = subprocess.run(SUITE, capture_output=True, text=True)
    base_out = base.stdout + base.stderr
    if ("could not compile" in base_out or "\nerror[" in base_out
            or base_out.startswith("error[")):
        print(f"{name}: BASELINE-RED (the tree does not build; nothing was mutated)")
        for line in base_out.splitlines()[:4]:
            print(f"    {line}")
        return 3
    if "FAILED" in base_out:
        print(f"{name}: BASELINE-RED (the suite was already failing; nothing was mutated)")
        for line in failing_tests(base_out)[:4]:
            print(f"    {line}")
        print("    Fix the baseline first — against a red suite every mutation reports CAUGHT.")
        return 3

    backup = pathlib.Path(tempfile.mkdtemp()) / p.name
    shutil.copy2(p, backup)
    try:
        p.write_text(mutated)
        r = subprocess.run(SUITE, capture_output=True, text=True)
        out = r.stdout + r.stderr
        if "could not compile" in out or "\nerror[" in out or out.startswith("error["):
            print(f"{name}: DID-NOT-COMPILE (inconclusive — says nothing about coverage)")
            code = 2
        elif "FAILED" in out:
            print(f"{name}: CAUGHT")
            code = 0
            # `--no-fail-fast` above is what makes this list complete. Without it
            # cargo stops at the first failing test BINARY, so a mutation caught
            # by tests in three targets reports one, and the author concludes the
            # coverage is thinner than it is.
            for line in failing_tests(out):
                print(f"    {line}")
        else:
            print(f"{name}: *** SURVIVED *** — nothing asserts this")
            code = 1
    finally:
        # `shutil.copy` and an explicit touch, NOT `copy2`: copy2 preserves the
        # backup's mtime, so cargo's change detection does not see the restore
        # and the NEXT run reuses the artifact built from mutated source. That
        # does not corrupt the run doing the mutating — it corrupts the one
        # after, which is worse to diagnose.
        shutil.copy(backup, p)
        p.touch()
        # And then REBUILD, because restoring the source is not the same as
        # restoring `target/` (day#114). A manual probe immediately after a run
        # otherwise drives a binary compiled from the mutant: that is how a
        # defect that had already been fixed got "measured" as still present.
        # Quiet, and its failure is reported rather than swallowed — a failed
        # rebuild means the tree on disk is fine but the artifacts are not.
        rebuild = subprocess.run(
            ["cargo", "build", "--quiet", "--workspace", "--all-targets"],
            capture_output=True, text=True,
        )
        if rebuild.returncode != 0:
            print("    warning: the post-restore rebuild failed; `target/` may still")
            print("    hold artifacts built from the mutant. Run `cargo build` before")
            print("    probing anything by hand.")
    return code


if __name__ == "__main__":
    sys.exit(main())
