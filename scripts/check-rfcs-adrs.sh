#!/usr/bin/env bash
set -euo pipefail

repo_root=${DAY_RFC_ROOT:-$(git rev-parse --show-toplevel)}
cd "$repo_root"

fail() {
  echo "RFC/ADR CHECK FAILED: $*" >&2
  exit 1
}

required_rfc_sections=(
  Summary Motivation Terminology 'Denotational target' 'Operational profile'
  'Approximation map' 'Canonicalization and equivalence'
  'Resolution or processing algorithm' 'Authority and trust model'
  'Security considerations' Compatibility 'Alternatives considered'
  'Reference test vectors' 'Unresolved questions' 'Deferred questions'
  'Implementation status'
)

check_rfc() {
  local file=$1 status
  for field in Status Authors Created Discussion Review-period-ends Review-override Supersedes Superseded-by Kan-claim Implementation; do
    grep -Eq "^- ${field}: .+" "$file" || fail "$file lacks metadata: $field"
  done
  status=$(sed -n 's/^- Status: //p' "$file")
  case "$status" in
    Draft|Review|Accepted|Implemented|Rejected|Withdrawn|Superseded) ;;
    *) fail "$file has unrecognized status: $status" ;;
  esac
  for section in "${required_rfc_sections[@]}"; do
    grep -Fqx "## $section" "$file" || fail "$file lacks section: $section"
  done
}

check_adr() {
  local file=$1 status
  status=$(sed -n 's/^- Status: //p' "$file")
  case "$status" in
    Proposed|Accepted|Rejected|Deprecated|Superseded) ;;
    *) fail "$file has unrecognized status: $status" ;;
  esac
  for section in Context Decision Rationale Consequences Evidence 'Alternatives considered' Supersession; do
    grep -Fqx "## $section" "$file" || fail "$file lacks section: $section"
  done
}

[[ -f rfcs/README.md ]] || fail "missing rfcs/README.md"
[[ -f rfcs/template.md ]] || fail "missing rfcs/template.md"
[[ -f adrs/README.md ]] || fail "missing adrs/README.md"
[[ -f adrs/template.md ]] || fail "missing adrs/template.md"

rfc_numbers=''
rfc_count=0
for file in rfcs/[0-9]*-*.md; do
  base=${file#rfcs/}
  number=${base%%-*}
  [[ "$number" =~ ^(0|[1-9][0-9]*)$ ]] || fail "$file does not use shortest decimal numbering"
  case " $rfc_numbers " in
    *" $number "*) fail "duplicate RFC number $number" ;;
  esac
  rfc_numbers="$rfc_numbers $number"
  rfc_count=$((rfc_count + 1))
  check_rfc "$file"
  links=$(grep -Ec "^- \[RFC ${number}: .+\]\(${base}\) — " rfcs/README.md || true)
  [[ "$links" -eq 1 ]] || fail "$file must have exactly one RFC index entry"
done

adr_numbers=''
adr_count=0
shopt -s nullglob
for file in adrs/[0-9]*-*.md; do
  base=${file#adrs/}
  number=${base%%-*}
  [[ "$number" =~ ^[1-9][0-9]*$ ]] || fail "$file does not use positive shortest decimal numbering"
  case " $adr_numbers " in
    *" $number "*) fail "duplicate ADR number $number" ;;
  esac
  adr_numbers="$adr_numbers $number"
  adr_count=$((adr_count + 1))
  check_adr "$file"
  links=$(grep -Ec "^- \[ADR ${number}: .+\]\(${base}\) — " adrs/README.md || true)
  [[ "$links" -eq 1 ]] || fail "$file must have exactly one ADR index entry"
done

if [[ ${1:-} == --self-test ]]; then
  fixture=$(mktemp -d "${TMPDIR:-/tmp}/day-rfc-check.XXXXXX")
  trap 'rm -rf "$fixture"' EXIT
  reset_fixture() {
    rm -rf "$fixture/rfcs" "$fixture/adrs" "$fixture/scripts"
    cp -R rfcs adrs scripts "$fixture/"
  }
  expect_rejected() {
    local label=$1 expected=$2
    if DAY_RFC_ROOT="$fixture" "$fixture/scripts/check-rfcs-adrs.sh" >"$fixture/output" 2>&1; then
      fail "self-test accepted $label mutation"
    fi
    grep -Fq "$expected" "$fixture/output" || fail "self-test $label failed for the wrong reason"
    echo "RFC/ADR self-test: $label mutation rejected"
  }

  reset_fixture
  perl -0pi -e 's/## Security considerations/## Security notes/' "$fixture/rfcs/0-rfc-and-adr-process.md"
  expect_rejected missing-section 'lacks section: Security considerations'

  reset_fixture
  perl -0pi -e 's/- Status: Draft/- Status: Maybe/' "$fixture/rfcs/0-rfc-and-adr-process.md"
  expect_rejected invalid-status 'unrecognized status: Maybe'

  reset_fixture
  cp "$fixture/rfcs/1-frame-indexed-process-model.md" "$fixture/rfcs/1-zduplicate.md"
  expect_rejected duplicate-number 'duplicate RFC number 1'

  reset_fixture
  cp "$fixture/rfcs/1-frame-indexed-process-model.md" "$fixture/rfcs/01-leading-zero.md"
  expect_rejected leading-zero 'does not use shortest decimal numbering'

  reset_fixture
  perl -ni -e 'print unless /1-frame-indexed-process-model/' "$fixture/rfcs/README.md"
  expect_rejected missing-index 'must have exactly one RFC index entry'
  exit 0
fi

echo "RFC/ADR check: $rfc_count RFC(s), $adr_count ADR(s), templates and indexes valid"
