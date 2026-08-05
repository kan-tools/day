#!/usr/bin/env python3
"""Revert a fix, watch the test that closes the finding fail, restore, watch it
pass. One demonstration, honestly reported.

**Mutation and reversion are different questions**, and conflating them is what
let day#116's first instance through:

  * *Mutation* (`scripts/mutate.py`) asks: does **any** test assert this line?
  * *Reversion* asks: does **the test written for this finding** fail when the
    finding is reintroduced?

Mutating an adjacent line, or the feature around the finding, answers the first
and looks like the second. `CLAUDE.md` already says "mutate the exact line the
finding was about"; three review rounds showed that is not specific enough,
because the exact line is often ambiguous after a restructure. Reverting the
change is unambiguous.

Outcomes, never conflated, could-not-check outranking checked-and-clean:

  DEMONSTRATED     baseline green -> reverted -> named tests FAILED -> restored
                   -> named tests pass. The only outcome that exits 0.
  VACUOUS          the named tests still passed with the fix reverted. The
                   headline finding: the test does not observe its guarantee.
  BASELINE-RED     a named test was already failing. Could-not-check; nothing
                   was reverted.
  NO-SUCH-TEST     a name matched no test. Could-not-check -- a filter typo
                   otherwise reads as the strongest possible result.
  DID-NOT-COMPILE  the reverted tree does not build. Could-not-check; says
                   NOTHING about coverage. Reverting a fix that changed a
                   signature is a normal way to reach this, not an exotic one.
  REVERT-FAILED    there was nothing to revert, or the patch did not apply.
  NOT-RESTORED     the tree or the tests did not come back. Loud by design.

**The test half of the change is not reverted.** A fix commit normally carries
the fix *and* the test that closes the finding; reverting both deletes the test,
and a deleted test cannot fail. Hunks under `tests/` are dropped, and inside a
`src/*.rs` file, hunks at or after that file's `#[cfg(test)]` line are dropped.
The heuristic's failure modes are both loud and neither is `DEMONSTRATED`:
exclude too much and the fix is not really reverted (`VACUOUS`), exclude too
little and the named test is gone (`NO-SUCH-TEST`).

Usage:
  scripts/revert-demo.py --tests a,b [--rev REF] [--include P]... [--exclude P]...
  scripts/revert-demo.py --verify REV
"""
# macOS ships Python 3.9, where `int | None` in an evaluated annotation is a
# TypeError. Lazy annotations rather than `typing.Optional`, so the source reads
# the same on both.
from __future__ import annotations

import argparse
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

DEMONSTRATED = "DEMONSTRATED"
VACUOUS = "VACUOUS"
BASELINE_RED = "BASELINE-RED"
NO_SUCH_TEST = "NO-SUCH-TEST"
DID_NOT_COMPILE = "DID-NOT-COMPILE"
REVERT_FAILED = "REVERT-FAILED"
NOT_RESTORED = "NOT-RESTORED"

# Zero context, so a hunk never spans the `#[cfg(test)]` boundary.
#
# With git's default three lines of context, a fix and a test module a couple of
# lines apart come out as ONE hunk, and the boundary rule below can only drop it
# whole or keep it whole. Measured: it dropped nothing, the test was reverted
# along with the fix, and the run reported NO-SUCH-TEST -- loud, and still the
# wrong answer. Zero-context hunks are line-precise, and the patch is applied
# back to the same tree it was generated from moments earlier, so the context
# lines were verifying nothing that the byte-for-byte restore check does not.
UNIFIED_ZERO = "--unified=0"

TRAILER = "Demonstrated-by:"
# Deliberately strict, and asserted by `tests/revert_demo.rs`: a trailer is a
# claim about the work, and a grammar that accepts anything cannot refute one.
#
# `revert=HEAD` is literal rather than a field. It was a free-form value that was
# parsed and then discarded, so any string survived — a fabricable field that
# nothing read. A trailer is always about the commit carrying it, so there is
# nothing for it to vary over.
TRAILER_RE = re.compile(
    r"^Demonstrated-by:\s+revert=(?P<rev>HEAD)\s+tests=(?P<tests>\S+)\s+"
    r"outcome=(?P<outcome>[A-Z-]+)\s*$",
    re.MULTILINE,
)


class CouldNotCheck(Exception):
    """Raised with an outcome token that is not a statement about coverage."""

    def __init__(self, outcome: str, detail: str):
        super().__init__(detail)
        self.outcome = outcome
        self.detail = detail


def git(*args, cwd=None, check=True) -> str:
    r = subprocess.run(
        ["git", *args], cwd=cwd, capture_output=True, text=True
    )
    if check and r.returncode != 0:
        raise CouldNotCheck(REVERT_FAILED, f"git {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


# --- the patch, and which half of it to invert -------------------------------


def cfg_test_line(path: pathlib.Path) -> int | None:
    """The 1-based line where a file's trailing `#[cfg(test)] mod` begins, or None.

    Nineteen files in `src/` carry a trailing test module, which is why this
    exists.

    **`#[cfg(test)]` on anything other than a `mod` does not start the test
    half**, and getting that wrong was a live defect rather than a hypothetical.
    This returned the FIRST line carrying the attribute, so a single
    `#[cfg(test)] fn helper(...)` inside an `impl` near the top of a file made
    every hunk below it "test-side" -- the fix was left in place, its callers
    were reverted, and the run reported DID-NOT-COMPILE. Which is honest, and
    says nothing about coverage.

    `tests/plugin.rs`'s `cfg_test_module_line` had already been fixed for
    exactly this, for exactly this reason. A rule learned on one side of the
    repo did not reach the harness that checks the other -- the propagation
    failure CLAUDE.md records, one language over.

    Note also that the old docstring named the wrong failure mode. It said the
    blind spot surfaces as NO-SUCH-TEST (over-reverting); what actually
    happened was under-reverting into a tree that would not build. A harness
    that mis-describes its own failure modes is the thing this file exists to
    prevent, so the claim is now narrower: the boundary is a `mod`, and the
    remaining blind spot is a test `mod` that is not the last thing in a file.
    """
    try:
        lines = path.read_text().splitlines()
    except OSError:
        return None

    def declares_a_module(text: str) -> bool:
        stripped = text.strip()
        return stripped.startswith("mod ") or stripped.startswith("pub mod ")

    for n, line in enumerate(lines, start=1):
        stripped = line.strip()
        if not stripped.startswith("#[cfg(test)]"):
            continue
        # `#[cfg(test)] mod tests {` on one line.
        if declares_a_module(stripped[len("#[cfg(test)]") :]):
            return n
        # Otherwise the next non-attribute, non-blank line decides.
        for following in lines[n:]:
            if not following.strip() or following.strip().startswith("#"):
                continue
            if declares_a_module(following):
                return n
            break
    return None


def split_files(patch: str) -> list[tuple[str, str]]:
    """[(path, section)] for each file in a unified diff."""
    out = []
    current_path, buf = None, []
    for line in patch.splitlines(keepends=True):
        if line.startswith("diff --git "):
            if current_path is not None:
                out.append((current_path, "".join(buf)))
            # `diff --git a/X b/Y` -- take the b-side, which is the post-fix path.
            current_path = line.rstrip("\n").split(" b/", 1)[-1]
            buf = [line]
        elif current_path is not None:
            buf.append(line)
    if current_path is not None:
        out.append((current_path, "".join(buf)))
    return out


def split_hunks(section: str) -> tuple[str, list[tuple[int, str]]]:
    """(file header, [(new_start_line, hunk_text)])."""
    lines = section.splitlines(keepends=True)
    header, hunks, current, start = [], [], None, 0
    for line in lines:
        if line.startswith("@@"):
            if current is not None:
                hunks.append((start, "".join(current)))
            m = re.match(r"^@@ -\d+(?:,\d+)? \+(\d+)", line)
            start = int(m.group(1)) if m else 0
            current = [line]
        elif current is not None:
            current.append(line)
        else:
            header.append(line)
    if current is not None:
        hunks.append((start, "".join(current)))
    return "".join(header), hunks


def matches(path: str, patterns: list[str]) -> bool:
    return any(path == p or path.startswith(p.rstrip("/") + "/") for p in patterns)


def filter_patch(patch: str, root: pathlib.Path, include: list[str], exclude: list[str]):
    """Drop the test half. Returns (filtered patch, human-readable report)."""
    kept_sections, report = [], []
    for path, section in split_files(patch):
        if include and not matches(path, include):
            report.append(f"  skipped {path} (not in --include)")
            continue
        if matches(path, exclude):
            report.append(f"  skipped {path} (excluded)")
            continue
        # **`--include` overrides the `tests/` drop, which it did not.** The
        # default rule assumes the test half lives under `tests/`; that is a
        # default, not a law, and while it stood no fix under `tests/` could ever
        # be demonstrated. This milestone's own sixth defect was in
        # `tests/common/mod.rs`, so the tool could not have demonstrated the fix
        # for it. Naming a path explicitly is the author saying "this one is the
        # fix", and there is nothing left for the heuristic to decide.
        if not matches(path, include) and path.startswith("tests/"):
            report.append(f"  skipped {path} (test side; --include it to revert it)")
            continue

        header, hunks = split_hunks(section)
        cutoff = cfg_test_line(root / path) if path.endswith(".rs") else None
        kept = [h for start, h in hunks if cutoff is None or start < cutoff]
        dropped = len(hunks) - len(kept)
        if not kept:
            report.append(f"  skipped {path} (all {len(hunks)} hunk(s) test-side)")
            continue
        note = f" ({dropped} test-side hunk(s) kept in place)" if dropped else ""
        report.append(f"  reverting {path}: {len(kept)}/{len(hunks)} hunk(s){note}")
        kept_sections.append(header + "".join(kept))
    return "".join(kept_sections), report


# --- running the named tests -------------------------------------------------

# `ok` and `FAILED` only. **`ignored` is deliberately absent**: an `#[ignore]`d
# test did not run, and counting it as having run satisfies `require_ran` and
# then reports VACUOUS -- a finding *about the test* derived from an observation
# nobody made. That is day#114's shape mirrored: could-not-check dressed as the
# strongest available answer. An ignored named test now falls through to
# NO-SUCH-TEST, which is what it is.
RESULT_RE = re.compile(r"^test (?P<name>\S+) \.\.\. (?P<verdict>ok|FAILED)", re.M)


def cargo_args(spec: str) -> tuple[list[str], str]:
    """Turn `plugin::some_test` into the narrowest cargo invocation that runs it.

    **Qualifying the target is what makes the demonstration affordable.** An
    unqualified name means `--workspace`, which builds all twenty-odd integration
    targets three times over -- measured at 3m54s for one demonstration, which is
    not "nearly free" and would make the rule ceremony. `--test plugin` builds the
    library and one target. `lib::` runs a `#[cfg(test)]` module in `src/`.
    """
    if "::" not in spec:
        return ["cargo", "test", "--workspace", "--no-fail-fast", spec], spec
    target, filt = spec.split("::", 1)
    if target == "lib":
        return ["cargo", "test", "--lib", filt], filt
    return ["cargo", "test", "--test", target, filt], filt


def run_tests(specs: list[str], cwd: pathlib.Path, target_dir: str | None):
    """(per-spec {spec: (ran, failed)}, compiled) by libtest name.

    **Per spec, not aggregated.** Which tests a filter selected is the question
    the catcher rule below turns on, and a union across filters cannot answer it:
    one spec's selection would be credited to another whose filter happens to be
    a substring of it.

    `ran` is measured from libtest's own per-test lines rather than from the exit
    code, because a filter matching nothing exits 0 -- which is day#116's shape
    exactly: the absence of an observation reported as the strongest result.
    """
    env = {**os.environ, "CARGO_TARGET_DIR": target_dir} if target_dir else None
    results: dict[str, tuple[set[str], set[str]]] = {}
    for spec in specs:
        args, _ = cargo_args(spec)
        r = subprocess.run(
            # No `--quiet`: it reaches libtest as terse output, which prints dots
            # instead of `test <name> ... ok` lines -- and those lines are the
            # only evidence that the named test RAN. The first run of this
            # harness reported NO-SUCH-TEST for exactly that reason, which is the
            # premise check working before anything depended on it.
            args, cwd=cwd, capture_output=True, text=True, env=env,
        )
        out = r.stdout + r.stderr
        if "could not compile" in out or "\nerror[" in out or out.startswith("error["):
            return results, False
        ran, failed = set(), set()
        for m in RESULT_RE.finditer(out):
            ran.add(m.group("name"))
            if m.group("verdict") == "FAILED":
                failed.add(m.group("name"))
        results[spec] = (ran, failed)
    return results, True


def caught_by(results: dict) -> list[str]:
    """The specs whose EVERY selected test failed.

    A filter selects a set of tests; the demonstration is meaningful only if the
    set it selected failed. "At least one failed" credits a filter for a test it
    merely overlaps: with `demo_test` passing and `demo_test_two` failing, the
    filter `t::demo_test` selects both, and a substring rule put it in the
    trailer as a catcher. Found by a cold review probing the shape the two
    existing tests did not reach -- overlapping names where one PASSES.

    A filter that selected nothing catches nothing; `require_ran` reports that
    separately as NO-SUCH-TEST.
    """
    return [
        spec
        for spec, (ran, failed) in results.items()
        if ran and ran <= failed
    ]


def require_ran(specs: list[str], results: dict, when: str) -> None:
    missing = [s for s in specs if not results.get(s, (set(), set()))[0]]
    if missing:
        raise CouldNotCheck(
            NO_SUCH_TEST,
            f"{when}: no test matched {missing}. A filter that matches nothing "
            f"exits 0, so this can never be read as a pass.",
        )


# --- the demonstration -------------------------------------------------------


def demonstrate(root: pathlib.Path, patch: str, names: list[str], rev_label: str,
                include: list[str], exclude: list[str], target_dir: str | None):
    filtered, report = filter_patch(patch, root, include, exclude)
    print(f"What would be reverted ({rev_label}):")
    for line in report:
        print(line)
    if not filtered.strip():
        raise CouldNotCheck(
            REVERT_FAILED,
            "nothing left to revert once the test half was excluded. Either the "
            "change is test-only, or --include/--exclude excluded the fix.",
        )

    touched = [p for p, _ in split_files(filtered)]

    # Baseline BEFORE touching anything (day#114's rule, applied where the
    # harness is written rather than retrofitted onto it).
    results, compiled = run_tests(names, root, target_dir)
    if not compiled:
        raise CouldNotCheck(DID_NOT_COMPILE, "the tree does not build before any revert")
    require_ran(names, results, "baseline")
    failed = set().union(*(f for _, f in results.values())) if results else set()
    if failed:
        raise CouldNotCheck(
            BASELINE_RED,
            f"already failing before the revert: {sorted(failed)}. A demonstration "
            f"against a red baseline reports the strongest possible result for the "
            f"wrong reason.",
        )

    snapshot: dict[str, bytes | None] = {}
    for rel in touched:
        p = root / rel
        snapshot[rel] = p.read_bytes() if p.exists() else None

    def digest() -> dict[str, str | None]:
        return {
            rel: (hashlib.sha256((root / rel).read_bytes()).hexdigest()
                  if (root / rel).exists() else None)
            for rel in touched
        }

    before = digest()
    try:
        r = subprocess.run(
            ["git", "apply", "-R", "--unidiff-zero", "-"],
            cwd=root, input=filtered, capture_output=True, text=True,
        )
        if r.returncode != 0:
            raise CouldNotCheck(
                REVERT_FAILED, f"the reverse patch did not apply: {r.stderr.strip()}"
            )
        results, compiled = run_tests(names, root, target_dir)
        if not compiled:
            raise CouldNotCheck(
                DID_NOT_COMPILE,
                "the reverted tree does not build, so the tests could not run. This "
                "says nothing about whether they assert the fix.",
            )
        require_ran(names, results, "under revert")
        # **`caught` is the SPECS that caught it, not the libtest names.**
        #
        # A trailer names only the tests that caught it, so that it is a true
        # statement whatever was named on the command line, and `verify()` can
        # then require every test a trailer names to fail.
        #
        # `caught_by` decides "caught" as **every selected test failed**, which
        # is the third attempt at this rule. Comparing filters to libtest names
        # by equality broke prefix filters; comparing by substring credited a
        # filter for a test it merely overlapped. Selection is the thing that
        # actually matters, and it is now measured per spec.
        caught = caught_by(results)
        outcome = DEMONSTRATED if caught else VACUOUS
        quiet = [n for n in names if n not in caught]
        if outcome == DEMONSTRATED and quiet:
            print(f"note: named but did not fail under revert, so NOT in the "
                  f"trailer: {quiet}")
    finally:
        restore(root, snapshot)

    after = digest()
    if after != before:
        raise CouldNotCheck(
            NOT_RESTORED,
            "the working tree did not come back byte-for-byte after the revert",
        )
    results, compiled = run_tests(names, root, target_dir)
    still_failing = (
        set().union(*(f for _, f in results.values())) if results else set()
    )
    if not compiled or still_failing:
        raise CouldNotCheck(
            NOT_RESTORED,
            f"the named tests do not pass again after restoring: {sorted(still_failing)}",
        )
    return outcome, caught


def restore(root: pathlib.Path, snapshot: dict[str, bytes | None]) -> None:
    """Put every touched file back, with a fresh mtime.

    A fresh mtime, not a preserved one: `mutate.py` learned that preserving it
    hides the restore from cargo's change detection and corrupts the NEXT run
    rather than this one, which is much worse to diagnose.
    """
    for rel, content in snapshot.items():
        p = root / rel
        if content is None:
            p.unlink(missing_ok=True)
        else:
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_bytes(content)
            p.touch()


# --- modes -------------------------------------------------------------------


def patch_for_rev(rev: str, cwd: pathlib.Path) -> str:
    # `rev-list --parents` prints "<sha> <parent>...": one entry for a root
    # commit, two for an ordinary one, more for a merge.
    parents = git("rev-list", "--parents", "-n", "1", rev, cwd=cwd).split()
    if len(parents) > 2:
        raise CouldNotCheck(
            REVERT_FAILED,
            f"{rev} is a merge commit; a merge has no single change to invert",
        )
    if len(parents) == 2:
        return git("diff", UNIFIED_ZERO, f"{rev}^", rev, cwd=cwd)
    return git("show", "--format=", UNIFIED_ZERO, rev, cwd=cwd)


def read_trailer(rev: str, cwd: pathlib.Path):
    body = git("log", "-1", "--format=%B", rev, cwd=cwd)
    if TRAILER not in body:
        return None
    m = TRAILER_RE.search(body)
    if not m:
        raise CouldNotCheck(
            REVERT_FAILED,
            f"{rev} carries a `{TRAILER}` line that does not parse. Expected, "
            f"exactly:\n"
            f"  {TRAILER} revert=HEAD tests=<target::name>,... outcome=DEMONSTRATED\n"
            f"`revert=HEAD` and `outcome=DEMONSTRATED` are literal: a trailer is "
            f"always about the commit carrying it, and DEMONSTRATED is the only "
            f"outcome worth claiming. Run the harness and paste what it prints.",
        )
    return m.group("rev"), [t for t in m.group("tests").split(",") if t], m.group("outcome")


def verify(spec: str, root: pathlib.Path) -> int:
    """Re-derive what a commit's trailer claims, in a throwaway worktree.

    A worktree rather than the caller's tree: re-derivation in CI must not be
    able to touch what it is checking, and this repo already uses worktrees to
    build historical readers in `migration-matrix.yml`.

    **Two things this must not do, both found by running it rather than reading
    it, and both of which produced a confident wrong answer:**

    1. It must not carry a *symbolic* rev into the worktree. `HEAD~1` means one
       thing in the caller's repo and something else entirely inside a detached
       checkout, where `HEAD` is the commit being verified — so `--verify HEAD~1`
       silently verified `HEAD~2`'s diff against `HEAD~1`'s tree. Resolved to a
       sha here, once, before anything else sees it.
    2. It must not perturb the environment the tests run in. Sharing the
       caller's artifact cache through `CARGO_TARGET_DIR` was a real speedup and
       a real change of environment: it leaked into a test that shells out to
       cargo, which then built somewhere unexpected and failed for a reason that
       had nothing to do with the commit. A verifier whose own environment can
       change the answer reports could-not-check as checked. The worktree builds
       into its own `target/`; the cost is a cold build per verification.
    """
    rev = git("rev-parse", "--verify", f"{spec}^{{commit}}", cwd=root).strip()
    parsed = read_trailer(rev, root)
    if parsed is None:
        print(f"{spec}: no {TRAILER} trailer; nothing to verify")
        return 0
    _, names, claimed = parsed

    work = pathlib.Path(tempfile.mkdtemp(prefix="revert-demo-"))
    tree = work / "tree"
    git("worktree", "add", "--detach", str(tree), rev, cwd=root)
    try:
        patch = patch_for_rev(rev, cwd=tree)
        outcome, caught = demonstrate(tree, patch, names, rev, [], [], None)
        # A trailer names only the tests that caught it, so every one of them
        # must catch it again. Accepting "at least one" would let a trailer carry
        # passengers that never observed the finding.
        #
        # Compared as SPECS on both sides. Comparing a spec list against libtest
        # names made an honest trailer verify as VACUOUS whenever its filter also
        # matched a second failing test — a verifier reporting
        # checked-and-found-a-defect where it could not check, and telling the
        # author to fix a test that was fine.
        if outcome == DEMONSTRATED and sorted(names) != sorted(caught):
            outcome = VACUOUS
    finally:
        subprocess.run(["git", "worktree", "remove", "--force", str(tree)],
                       cwd=root, capture_output=True, text=True)
        shutil.rmtree(work, ignore_errors=True)

    # **Two conditions, not one.** Comparing the re-derived outcome to the
    # claimed one is necessary and was not sufficient: a trailer claiming
    # `outcome=VACUOUS` re-derived as VACUOUS, matched, and exited 0 — so the one
    # outcome the rule names as disqualifying passed the gate that enforces it.
    # A green check and a `Demonstrated-by:` trailer, for a commit whose test
    # does not observe its finding. day#116's shape inside the tool built to end
    # it, found by a cold review of this branch.
    #
    # DEMONSTRATED is the only outcome a trailer may carry, so both halves are
    # checked: what was claimed, and whether it is the thing worth claiming.
    if outcome != claimed:
        print(f"{spec}: *** {outcome} *** — the trailer claims {claimed}")
        return 1
    if outcome != DEMONSTRATED:
        print(
            f"{spec}: *** {outcome} *** — a trailer may only claim {DEMONSTRATED}.\n"
            f"  Re-deriving {outcome} confirms the claim and does not make it a "
            f"demonstration:\n  the fix was reverted and the named tests did not "
            f"observe it. Fix the test."
        )
        return 1
    print(f"{spec}: {outcome} (re-derived; caught by {', '.join(caught)})")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--tests", help="comma-separated test name filters")
    ap.add_argument("--rev", help="invert this commit instead of the working tree")
    ap.add_argument("--include", action="append", default=[])
    ap.add_argument("--exclude", action="append", default=[])
    ap.add_argument("--verify", metavar="REV", help="re-derive the trailer REV claims")
    args = ap.parse_args()

    root = pathlib.Path(git("rev-parse", "--show-toplevel").strip())

    try:
        if args.verify:
            return verify(args.verify, root)
        if not args.tests:
            ap.error("--tests is required unless --verify is given")
        names = [t for t in args.tests.split(",") if t]
        if args.rev:
            patch, label = patch_for_rev(args.rev, root), args.rev
        else:
            patch, label = git("diff", UNIFIED_ZERO, "HEAD", cwd=root), "worktree"
            untracked = [
                p for p in git("ls-files", "--others", "--exclude-standard",
                               cwd=root).split()
                if not p.startswith("tests/")
            ]
            if untracked:
                print(f"note: untracked and therefore NOT reverted: {untracked}")
        outcome, caught = demonstrate(
            root, patch, names, label, args.include, args.exclude, None
        )
        args_rev = args.rev
    except CouldNotCheck as e:
        print(f"{e.outcome}: {e.detail}")
        return 1

    if outcome == VACUOUS:
        print(f"\n*** {VACUOUS} *** — the fix was reverted and {names} still passed.")
        print("The test does not observe the finding it was written to close.")
        return 1

    print(f"\n{DEMONSTRATED} — failed under revert: {', '.join(caught)}")
    # A trailer belongs on the commit that carries the change it describes, so it
    # is printed only when this run inverted something that IS that commit: the
    # working tree (about to be committed) or HEAD. Under `--rev <older>` the
    # demonstration is a re-check of history, and printing `revert=HEAD` there
    # produced a paste-ready line that misstated what had been inverted.
    if args_rev in (None, "HEAD"):
        print("\nPaste into the commit message:\n")
        print(f"    {TRAILER} revert=HEAD "
              f"tests={','.join(caught)} outcome={DEMONSTRATED}")
    else:
        print(f"\nNo trailer printed: this demonstrated {args_rev}, which is not "
              f"the commit a trailer would land on.\nRe-run without `--rev` on the "
              f"change you are about to commit, or amend that commit.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
