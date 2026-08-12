# The review exit bar for `harness-footer` — preregistered

**Written before round 3 runs, so it cannot be fitted to the result.** Same
discipline as `eval/legible-surface.rubric.v1.md`, applied to the review loop
itself rather than to a telos: a bar stated after seeing the findings is a bar
chosen to be cleared.

Two rounds have returned BLOCK. Each round's fixes introduced the next round's
findings, and nothing in the loop says what would end it — so "fix the findings"
has no completion condition except "a review finds nothing", which is
unfalsifiable in the direction that matters. A round that finds nothing may mean
the work is clean, or may mean the reviewer looked where the last round already
fixed.

## What makes this branch mergeable

1. **Zero severity-1 findings.** Currently two, both open: the `user-prompt`
   path that still leaves a stale confident bar, and the `sync_state` pathspec
   that reports a clean tree from a subdirectory.
2. **Every severity-2 finding either fixed or recorded as accepted**, on
   `harness-footer`, with a reason and the cost stated. Accepted is a legitimate
   outcome; silently unaddressed is not.
3. **Every claim the fix round makes about its own verification is reproduced by
   a tool, not by hand** — `scripts/mutate.py` for mutations,
   `scripts/revert-demo.py` for demonstrations, `scripts/behaviour-diff.py` for
   regression. Both rounds so far failed this, in opposite directions.
4. **The mutation table is produced by an agent that did not write the fixes.**
   See below.
5. **`scripts/behaviour-diff.py` reports `IDENTICAL`, `CHANGED-AS-DECLARED`, or
   a `COVERAGE-UNKNOWN` whose unreached sources are named in the commit.** A
   bare pass is not required; an unexamined one is disqualifying.

## What does NOT block a merge

- Findings against `.design/harness-footer.md` itself. Two rounds have found
  real ones. A design defect routes to `/design`, not to another fix round —
  patching the code to match a wrong requirement is how a milestone converges on
  the wrong thing while every round looks productive.
- Severity-3 and below, if recorded.
- The eval harness's remaining degeneracies. The witness is honestly absent and
  no attestation exists; a harness that cannot yet score is not a blocker for the
  code it would score, and pretending otherwise couples two things that fail
  independently.

## Round 3's method, fixed in advance

- **The mutation table is produced by an independent agent**, given the list of
  claimed-fixed defects and told to construct each mutation itself. Both of
  round 2's worst findings — the collision-not-swap mutation and the exemption
  asserted about an unrun tool — would have been caught by this alone. The
  author of a fix is the least able person to see what the fix missed
  (kan ADR-52), and that argument does not stop applying at the verification.
- **Reviewers are handed the DERIVED SET, not a list of fixes** (`practice`
  item: a review's findings are file:line by format, so handing a reviewer
  instances gets instances back and the loop runs as long as the class has
  members). State the class, state each member's disposition, and let the review
  attack the enumeration.
- **The bar above is quoted to the reviewers**, so a verdict is rendered against
  it rather than against an unstated standard.

## If round 3 also returns BLOCK on new severity-1 findings

Stop and redesign rather than fix again. Three rounds of instance-fixing without
convergence is evidence about the generator, not about the instances — and the
generator here is a design whose requirements two rounds have already found
defective.
