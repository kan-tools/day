#!/bin/sh
# Tell a human, once per session, when day's binaries are not on PATH.
#
# WHY THIS IS NOT A `day hook` SUBCOMMAND, which is the rule every other hook
# here follows. It is the one check whose whole subject is day being *absent*:
# `day hook missing-binaries` cannot report that `day` is missing, because the
# shell never finds it to ask. So this is a bundled script, and
# `tests/plugin.rs` was narrowed rather than relaxed to allow exactly this one
# and assert every other SessionStart command is still a `day hook` invocation.
#
# WHAT IT FIXES. The plugin installs cleanly with no binary present — the
# marketplace ships prose and configuration, and `cargo install day` is a
# separate act nothing enforces. What the user sees then is an MCP server that
# fails to start and two hooks that exit non-zero, none of which says the words
# `cargo install day`. This turns that into an instruction.
#
# WHY IT CHECKS kan TOO. day writes only by shelling out to `kan`, so a present
# day with an absent kan is still broken, and it fails at the first write rather
# than at startup — later, and further from the cause.
#
# ADVISORY, like every hook here (`telos/affordance-not-enforcement`). It emits
# a `systemMessage` and nothing else: a notice to the human, never a decision,
# and no `hookSpecificOutput`. It exits 0 unconditionally, including when it has
# found something wrong — a hook that fails the session because a dependency is
# missing would be the gate this project exists not to build.
#
# Emits NOTHING when both binaries are present, matching `day hook
# session-notice`: a healthy session sees no injection at all.

missing=
command -v day >/dev/null 2>&1 || missing=day
command -v kan >/dev/null 2>&1 || missing="${missing:+$missing }kan"

[ -z "$missing" ] && exit 0

# Built as one line with literal backslash-n sequences: `printf %s` does not
# process escapes in its argument, so these reach stdout as the two characters
# JSON wants, and the payload stays a single valid line. No double quote appears
# in the text, deliberately — there is nothing here to escape.
banner='day plugin: required binaries are not on PATH'
body='Missing: '"$missing"

# The install commands are DERIVED, never hand-written. Every published day is
# a pre-release, so a plain `cargo install day` errors; a plain `cargo install
# kan` silently installs the one stable kan, 0.1.0, which is below day's
# measured floor — the worse failure because it looks like it worked (day#50).
# This script ships inside the plugin next to the two files that know the right
# pins, so it reads them the same way tests/install_docs.rs derives the
# README's: the day pin is this crate's version, the kan pin is the newest kan
# tests/fixtures/kan-compat.tsv records as `ok`.
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
day_version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/Cargo.toml" 2>/dev/null | head -n 1)
kan_version=$(awk -F'	' '$2 == "ok" {v = $1} END {if (v != "") print substr(v, 2)}' \
  "$root/tests/fixtures/kan-compat.tsv" 2>/dev/null)

install_lines=
case " $missing " in
  *" day "*)
    if [ -n "$day_version" ]; then
      install_lines="${install_lines}\\n    cargo install day --version $day_version"
    else
      # The derivation failed; say where the pin lives rather than print a
      # command the README documents as broken.
      install_lines="${install_lines}\\n    cargo install day --version <see the Install section of the plugin README>"
    fi ;;
esac
case " $missing " in
  *" kan "*)
    if [ -n "$kan_version" ]; then
      install_lines="${install_lines}\\n    cargo install kan --version $kan_version"
    else
      install_lines="${install_lines}\\n    cargo install kan --version <see the Install section of the plugin README>"
    fi ;;
esac

printf '{"systemMessage":"%s"}\n' "\
⚠  $banner\\n\\n\
  $body\\n\\n\
  day is a thin CLI over these binaries — its skills and its MCP server both\\n\
  shell out to them, so the plugin does nothing useful until they exist:\\n\
$install_lines\\n\\n\
  Then run /reload-plugins, or start a new session. Verify with: day doctor"
