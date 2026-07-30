#!/usr/bin/env python3
"""One mutation, honestly reported. The shape day#90 asks for.

Distinguishes the three outcomes that matter and never conflates them:
  CAUGHT            a test failed -> the mutation was detected
  SURVIVED          tests passed  -> nothing asserts this behaviour
  DID-NOT-COMPILE   the mutation was ill-formed; says NOTHING about coverage
  ANCHOR-MISSING    the text was not found; the mutation never happened

Restores from a backup in a finally block, so an interrupt cannot leave a
mutated tree — the failure that left src/status.rs mutated earlier in this
session.
"""
import pathlib
import shutil
import subprocess
import sys
import tempfile

def main() -> int:
    path, anchor, replacement, name = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    p = pathlib.Path(path)
    original = p.read_text()

    if anchor not in original:
        print(f"{name}: ANCHOR-MISSING (the mutation never happened)")
        return 0

    mutated = original.replace(anchor, replacement, 1)
    if mutated == original:
        print(f"{name}: ANCHOR-MISSING (replacement identical to anchor)")
        return 0

    backup = pathlib.Path(tempfile.mkdtemp()) / p.name
    shutil.copy2(p, backup)
    try:
        p.write_text(mutated)
        r = subprocess.run(
            ["cargo", "test", "--quiet", "--workspace"],
            capture_output=True, text=True,
        )
        out = r.stdout + r.stderr
        if "could not compile" in out or "\nerror[" in out or out.startswith("error["):
            print(f"{name}: DID-NOT-COMPILE (inconclusive — says nothing about coverage)")
        elif "FAILED" in out:
            failed = [l.strip() for l in out.splitlines() if l.strip().startswith("---- ")]
            print(f"{name}: CAUGHT")
            for line in failed[:4]:
                print(f"    {line}")
        else:
            print(f"{name}: *** SURVIVED *** — nothing asserts this")
    finally:
        # `shutil.copy` and an explicit touch, NOT `copy2`: copy2 preserves the
        # backup's mtime, so cargo's change detection does not see the restore
        # and the NEXT run reuses the artifact built from mutated source. That
        # does not corrupt the run doing the mutating — it corrupts the one
        # after, which is worse to diagnose.
        shutil.copy(backup, p)
        p.touch()
    return 0

if __name__ == "__main__":
    sys.exit(main())
