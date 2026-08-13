#!/bin/sh
# Classify every plan-bearing subject as reviewed, exempt, or open.
set -eu

kan_bin="${KAN_BIN:-kan}"
input="${DAY_CENSUS_JSON:-}"
if [ -n "$input" ]; then
  json="$(cat "$input")"
else
  json="$($kan_bin show --all --json)"
fi

printf '%s' "$json" | jq -r '
  def live: (.claims // []) | map(select((.retracted // false) | not));
  [.subjects[]
   | .subject as $subject
   | (live) as $claims
   | select(any($claims[]; .kind == "Plan"))
   | if any($claims[]; (.kind == "Decision" or .kind == "Verdict") and ((.text // "") | test("adversarial.review"; "i")))
     then [$subject, "reviewed", "adversarial-review verdict recorded"]
     elif any($claims[]; .kind == "Decision" and ((.text // "") | test("review.exempt"; "i")))
     then [$subject, "exempt", ([ $claims[] | select(.kind == "Decision" and ((.text // "") | test("review.exempt"; "i"))) | .text ][-1])]
     else [$subject, "open", "no review verdict or exemption"] end]
  | sort_by(.[0])
  | .[] | @tsv
' | awk -F '\t' '
  { count[$2]++; total++; print $0 }
  END {
    printf "process census: %d reviewed, %d exempt, %d open, %d total\n", count["reviewed"]+0, count["exempt"]+0, count["open"]+0, total+0 > "/dev/stderr"
  }
'
