<!--
One PR per milestone, off `main`. Merged with `--merge` rather than squashed,
so the milestone's internal commits stay visible — the demonstration census
walks that history.

If this changes what day DOES and there is no issue and no design pass behind
it, expect to be asked to go back through that first. See CONTRIBUTING.md.
-->

## What this changes

<!-- And which issue or milestone it closes. -->

Closes #

## What you ran, and what it printed

<!--
Paste it. "Tests pass" is not evidence; the output is. This repo's own rule
is that assessment goes against material evidence — builds, tests, diffs —
rather than against a summary of them.
-->

```
```

## Checklist

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [ ] `cargo fmt --all -- --check` is clean
- [ ] `day doctor` still reports the atom vocabulary as composing

### The two properties this must not break

- [ ] **day stores nothing of its own** — every durable thing is an ordinary
      kan claim, read back through kan's public CLI. No config store, no
      sidecar database, no state file. (`.day/` is the one carve-out and is
      never read to decide anything, only to display.)
- [ ] **Advisory, never blocking** — no hook gates, denies, or rejects an
      action.

### If this adds a check

- [ ] I have seen it **fail**, not just pass. A check that cannot fail
      converts an unchecked area into one that merely looks checked.
- [ ] It distinguishes could-not-check from checked-and-clean, and
      could-not-check does not degrade to a pass.

### If this fixes a review finding

- [ ] The fix ships with a test that fails without it, **in the same commit**,
      verified by reverting the fix — not by a demonstration in a terminal.
- [ ] `Demonstrated-by:` trailer present
      (`python3 scripts/revert-demo.py --tests <target>::<test>`), or an
      exemption stated with the outcome the harness actually reported for it.

### Boundary

- [ ] This does not require a change to kan's data model. (If it does, that is
      a kan design question first — see CONTRIBUTING.md.)
