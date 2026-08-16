#!/usr/bin/env bash
set -euo pipefail
repo_root=${DAY_RFC_ROOT:-$(git rev-parse --show-toplevel)}
cd "$repo_root"
fail() { echo "RFC/ADR CHECK FAILED: $*" >&2; exit 1; }

rfc_sections=(Summary Motivation Terminology 'Denotational target' 'Operational profile v1' 'Approximation map' 'Canonicalization and equivalence' 'Resolution or processing algorithm' 'Authority and trust model' 'Security considerations' Compatibility 'Alternatives considered' 'Reference test vectors' 'Unresolved questions' 'Deferred questions' 'Implementation status')
adr_sections=(Context Decision Rationale Consequences Evidence 'Alternatives considered' Supersession)
rfc_fields=(Status Authors Created Discussion Review-period-ends Review-override Supersedes Superseded-by Profile-relationship Implementation)
adr_fields=(Status Date Authors Supersedes Superseded-by Related-RFC)

field_value() { sed -n "s/^- $2: //p" "$1"; }
require_fields() {
  local file=$1; shift
  local field value
  for field in "$@"; do
    value=$(field_value "$file" "$field")
    [[ -n "$value" ]] || fail "$file lacks metadata: $field"
  done
}
require_sections() {
  local file=$1; shift
  local section
  for section in "$@"; do
    grep -Fqx "## $section" "$file" || fail "$file lacks section: $section"
  done
}
check_rfc_shape() {
  local file=$1 status relationship discussion review_end
  require_fields "$file" "${rfc_fields[@]}"
  require_sections "$file" "${rfc_sections[@]}"
  status=$(field_value "$file" Status)
  case "$status" in Draft|Review|Accepted|Implemented|Rejected|Withdrawn|Superseded) ;; *) fail "$file has unrecognized status: $status" ;; esac
  relationship=$(field_value "$file" Profile-relationship)
  case "$relationship" in not-applicable|approximation) ;; *) fail "$file has unrecognized Profile-relationship: $relationship" ;; esac
  case "$status" in
    Accepted|Implemented|Superseded)
      discussion=$(field_value "$file" Discussion)
      review_end=$(field_value "$file" Review-period-ends)
      [[ "$discussion" =~ ^https://[^[:space:]]+$ ]] || fail "$file is $status but Discussion is not an https address"
      [[ "$review_end" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] || fail "$file is $status but Review-period-ends is not RFC3339 UTC"
      python3 - "$review_end" <<'PY' || fail "$file is $status before its review period ended"
import datetime, sys
end = datetime.datetime.strptime(sys.argv[1], "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=datetime.timezone.utc)
raise SystemExit(0 if end <= datetime.datetime.now(datetime.timezone.utc) else 1)
PY
      override=$(field_value "$file" Review-override)
      [[ "$override" == None || "$override" =~ ^unanimous:https://github\.com/[^[:space:]]+/pull/[0-9]+@[0-9a-f]{40}$ ]] || fail "$file is $status but Review-override is malformed"
      ;;
  esac
}
check_adr_shape() {
  local file=$1 status
  require_fields "$file" "${adr_fields[@]}"
  require_sections "$file" "${adr_sections[@]}"
  status=$(field_value "$file" Status)
  case "$status" in Proposed|Accepted|Rejected|Deprecated|Superseded) ;; *) fail "$file has unrecognized status: $status" ;; esac
}

[[ -f rfcs/README.md ]] || fail 'missing rfcs/README.md'
[[ -f rfcs/template.md ]] || fail 'missing rfcs/template.md'
[[ -f rfcs/numbers.tsv ]] || fail 'missing rfcs/numbers.tsv'
[[ -f adrs/README.md ]] || fail 'missing adrs/README.md'
[[ -f adrs/template.md ]] || fail 'missing adrs/template.md'
check_rfc_shape rfcs/template.md
check_adr_shape adrs/template.md
[[ -x scripts/check-rfc1-vectors.py ]] || fail 'scripts/check-rfc1-vectors.py is not executable'
[[ -x scripts/check-rfc0-publication.py ]] || fail 'scripts/check-rfc0-publication.py is not executable'
scripts/check-rfc0-publication.py
scripts/check-rfc1-vectors.py rfcs/vectors/1-process-model.json

rfc_numbers=''; rfc_count=0
for file in rfcs/[0-9]*-*.md; do
  base=${file#rfcs/}; number=${base%%-*}
  [[ "$number" =~ ^(0|[1-9][0-9]*)$ ]] || fail "$file does not use shortest decimal numbering"
  case " $rfc_numbers " in *" $number "*) fail "duplicate RFC number $number" ;; esac
  rfc_numbers="$rfc_numbers $number"; rfc_count=$((rfc_count + 1))
  check_rfc_shape "$file"
  heading=$(sed -n '1s/^# RFC [0-9][0-9]*: //p' "$file")
  heading_number=$(sed -n '1s/^# RFC \([0-9][0-9]*\): .*/\1/p' "$file")
  [[ "$heading_number" == "$number" ]] || fail "$file heading number differs from filename"
  status=$(field_value "$file" Status)
  expected="- [RFC $number: $heading]($base) — $status"
  [[ $(grep -Fxc -- "$expected" rfcs/README.md || true) -eq 1 ]] || fail "$file index row is missing or disagrees with title/status"
  registry=$(awk -F '\t' -v n="$number" '$1 == n { print $2 "\t" $3 }' rfcs/numbers.tsv)
  [[ "$registry" == "$base	$heading" ]] || fail "$file disagrees with rfcs/numbers.tsv allocation"
done
registry_count=$(awk -F '\t' 'NR > 1 && $1 != "" { count++ } END { print count+0 }' rfcs/numbers.tsv)
[[ "$registry_count" -eq "$rfc_count" ]] || fail 'rfcs/numbers.tsv contains stale or missing allocations'
baseline_registry=${DAY_RFC_BASE_REGISTRY:-}
if [[ -n "$baseline_registry" ]]; then
  baseline_contents=$(cat "$baseline_registry")
elif git cat-file -e main:rfcs/numbers.tsv 2>/dev/null; then
  baseline_contents=$(git show main:rfcs/numbers.tsv)
else
  baseline_contents=''
fi
while IFS=$'\t' read -r old_number old_file old_title; do
  [[ "$old_number" == number || -z "$old_number" ]] && continue
  [[ $(awk -F '\t' -v n="$old_number" -v f="$old_file" -v t="$old_title" '$1 == n && $2 == f && $3 == t { count++ } END { print count+0 }' rfcs/numbers.tsv) -eq 1 ]] || fail "historical RFC allocation changed: $old_number -> $old_file"
done <<< "$baseline_contents"
index_count=$(grep -Ec '^- \[RFC [0-9]+: .+\]\([0-9]+-[^)]+\.md\) — .+$' rfcs/README.md || true)
[[ "$index_count" -eq "$rfc_count" ]] || fail 'rfcs/README.md contains stale or missing RFC rows'

adr_numbers=''; adr_count=0; shopt -s nullglob
for file in adrs/[0-9]*-*.md; do
  base=${file#adrs/}; number=${base%%-*}
  [[ "$number" =~ ^[1-9][0-9]*$ ]] || fail "$file does not use positive shortest decimal numbering"
  case " $adr_numbers " in *" $number "*) fail "duplicate ADR number $number" ;; esac
  adr_numbers="$adr_numbers $number"; adr_count=$((adr_count + 1))
  check_adr_shape "$file"
  heading=$(sed -n '1s/^# ADR [0-9][0-9]*: //p' "$file")
  heading_number=$(sed -n '1s/^# ADR \([0-9][0-9]*\): .*/\1/p' "$file")
  [[ "$heading_number" == "$number" ]] || fail "$file heading number differs from filename"
  status=$(field_value "$file" Status)
  expected="- [ADR $number: $heading]($base) — $status"
  [[ $(grep -Fxc -- "$expected" adrs/README.md || true) -eq 1 ]] || fail "$file index row is missing or disagrees with title/status"
done
adr_index_count=$(grep -Ec '^- \[ADR [0-9]+: .+\]\([0-9]+-[^)]+\.md\) — .+$' adrs/README.md || true)
[[ "$adr_index_count" -eq "$adr_count" ]] || fail 'adrs/README.md contains stale or missing ADR rows'

if [[ ${1:-} == --self-test ]]; then
  fixture=$(mktemp -d "${TMPDIR:-/tmp}/day-rfc-check.XXXXXX"); trap 'rm -rf "$fixture"' EXIT
  reset_fixture() { rm -rf "$fixture/rfcs" "$fixture/adrs" "$fixture/scripts"; cp -R rfcs adrs scripts "$fixture/"; cp rfcs/numbers.tsv "$fixture/base-numbers.tsv"; }
  expect_rejected() {
    local label=$1 expected=$2
    if DAY_RFC_ROOT="$fixture" DAY_RFC_BASE_REGISTRY="$fixture/base-numbers.tsv" "$fixture/scripts/check-rfcs-adrs.sh" >"$fixture/output" 2>&1; then fail "self-test accepted $label mutation"; fi
    grep -Fq "$expected" "$fixture/output" || fail "self-test $label failed for the wrong reason"
    echo "RFC/ADR self-test: $label mutation rejected"
  }
  reset_fixture; perl -0pi -e 's/## Security considerations/## Security notes/' "$fixture/rfcs/template.md"; expect_rejected rfc-template-section 'rfcs/template.md lacks section: Security considerations'
  reset_fixture; perl -0pi -e 's/## Evidence/## Material/' "$fixture/adrs/template.md"; expect_rejected adr-template-section 'adrs/template.md lacks section: Evidence'
  reset_fixture; cp "$fixture/adrs/template.md" "$fixture/adrs/1-test.md"; perl -0pi -e 's/# ADR N: Title/# ADR 1: Test/; s/- Authors: Name or identity\n//' "$fixture/adrs/1-test.md"; printf '\n- [ADR 1: Test](1-test.md) — Proposed\n' >> "$fixture/adrs/README.md"; expect_rejected adr-metadata 'adrs/1-test.md lacks metadata: Authors'
  reset_fixture; printf '\n- [RFC 99: Stale](99-stale.md) — Draft\n' >> "$fixture/rfcs/README.md"; expect_rejected stale-index 'rfcs/README.md contains stale or missing RFC rows'
  reset_fixture; perl -0pi -e 's/1-frame-indexed-process-model.md\) — Draft/1-frame-indexed-process-model.md) — Accepted/' "$fixture/rfcs/README.md"; expect_rejected status-mismatch 'index row is missing or disagrees with title/status'
  reset_fixture; perl -0pi -e 's/# RFC 1: Frame-indexed/# RFC 2: Frame-indexed/' "$fixture/rfcs/1-frame-indexed-process-model.md"; expect_rejected heading-number 'heading number differs from filename'
  reset_fixture; perl -0pi -e 's/Frame-indexed process model/Unrelated replacement/' "$fixture/rfcs/numbers.tsv"; expect_rejected allocation-reuse 'disagrees with rfcs/numbers.tsv allocation'
  reset_fixture; perl -0pi -e 's/- Status: Draft/- Status: Accepted/' "$fixture/rfcs/0-rfc-and-adr-process.md"; perl -0pi -e 's/0-rfc-and-adr-process.md\) — Draft/0-rfc-and-adr-process.md) — Accepted/' "$fixture/rfcs/README.md"; expect_rejected accepted-metadata 'Discussion is not an https address'
  reset_fixture; mv "$fixture/rfcs/1-frame-indexed-process-model.md" "$fixture/rfcs/2-frame-indexed-process-model.md"; perl -0pi -e 's/# RFC 1:/# RFC 2:/' "$fixture/rfcs/2-frame-indexed-process-model.md"; perl -0pi -e 's/RFC 1: Frame-indexed process model\]\(1-frame-indexed-process-model\.md\)/RFC 2: Frame-indexed process model](2-frame-indexed-process-model.md)/' "$fixture/rfcs/README.md"; perl -0pi -e 's/^1\t1-frame-indexed-process-model\.md/2\t2-frame-indexed-process-model.md/m' "$fixture/rfcs/numbers.tsv"; expect_rejected historical-renumber 'historical RFC allocation changed: 1 -> 1-frame-indexed-process-model.md'
  reset_fixture; perl -0pi -e 's/- Status: Draft/- Status: Accepted/; s/- Discussion: Not opened/- Discussion: x/; s/- Review-period-ends: Not scheduled/- Review-period-ends: not-a-date/; s/- Review-override: None/- Review-override: forged/' "$fixture/rfcs/0-rfc-and-adr-process.md"; perl -0pi -e 's/0-rfc-and-adr-process.md\) — Draft/0-rfc-and-adr-process.md) — Accepted/' "$fixture/rfcs/README.md"; expect_rejected forged-review 'Discussion is not an https address'
  reset_fixture; perl -0pi -e 's/- Profile-relationship: approximation/- Profile-relationship: full-implementation/' "$fixture/rfcs/1-frame-indexed-process-model.md"; expect_rejected profile-relationship 'unrecognized Profile-relationship: full-implementation'
  reset_fixture; perl -0pi -e 's/"expected": "not-certified"/"expected": "certified"/' "$fixture/rfcs/vectors/1-process-model.json"; expect_rejected coherence-vector 'wrong witness result: coordinate-mismatch'
  reset_fixture; perl -0pi -e 's/- Status: Draft/- Kan-claim: bafyrecursive\n- Status: Draft/' "$fixture/rfcs/0-rfc-and-adr-process.md"; expect_rejected recursive-publication 'normative RFC bytes contain a claim-CID backlink'
  scripts/check-rfc1-vectors.py rfcs/vectors/1-process-model.json --self-test
  exit 0
fi
echo "RFC/ADR check: $rfc_count RFC(s), $adr_count ADR(s), templates, allocation registry, and indexes valid"
