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

install_lines=
case " $missing " in
  *" day "*) install_lines="${install_lines}\\n    cargo install day" ;;
esac
case " $missing " in
  *" kan "*) install_lines="${install_lines}\\n    cargo install kan" ;;
esac

printf '{"systemMessage":"%s"}\n' "\
⚠  $banner\\n\\n\
  $body\\n\\n\
  day is a thin CLI over these binaries — its skills and its MCP server both\\n\
  shell out to them, so the plugin does nothing useful until they exist:\\n\
$install_lines\\n\\n\
  Then run /reload-plugins, or start a new session. Verify with: day doctor"
