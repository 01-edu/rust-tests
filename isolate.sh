#!/usr/bin/env bash
# Use:
#   cargo --config 'target."cfg(all())".runner="./isolate.sh"' test [args]

set -u

bin="${1:-}"; shift || true
[[ -n "${bin:-}" ]] || { echo "[wrapper] missing test binary" >&2; exit 2; }

# Strip Cargo-only flags that confuse the test binary (e.g., --message-format)
filter_args() {
  local -a out=(); local skip=0
  for a in "$@"; do
    if [[ $skip -eq 1 ]]; then skip=0; continue; fi
    case "$a" in
      --message-format|--format) skip=1 ;;   # drop the next token
      --message-format=*|--format=*) ;;      # drop inline forms
      *) out+=("$a") ;;
    esac
  done
  printf '%s\n' "${out[@]}"
}
mapfile -t filtered < <(filter_args "$@")

tmpdir="$(mktemp -d -t strictwrap.XXXXXX)"; trap 'rm -rf "$tmpdir"' EXIT
expected="$tmpdir/expected.txt"
actual="$tmpdir/actual.txt"
logfile="$tmpdir/run.out"

# 1) Ask the harness which tests it *will* run (respects filters in "$@")
if ! "$bin" --list "${filtered[@]}" >"$tmpdir/list.txt" 2>/dev/null; then
  echo "[wrapper] unable to list tests; cannot verify harness" >&2
  exit 1
fi
# Lines look like: `path::to::name: test` (sometimes with " (ignored)")
awk -F': test' '/: test/{print $1}' "$tmpdir/list.txt" | sed 's/[[:space:]]*$//' > "$expected"

# 2) Run the suite normally and capture output + real exit code
(
  "$bin" "${filtered[@]}" 2>&1
  echo "__RC__$?"
) | tee "$logfile" >/dev/null
rc="$(awk -F'__RC__' '/__RC__/ {v=$2} END{print v+0}' "$logfile")"

# 3) Collect tests that actually produced a result line:
#    Matches lines like: `test foo::bar ... ok|ignored|FAILED`
#    Normalize names by stripping the " - should panic[ with `...`]" suffix.
awk '
  /^test[[:space:]][^ ]/ {
    line=$0
    sub(/^test[[:space:]]+/, "", line)            # drop leading "test "
    sub(/[[:space:]]+\.\.\..*$/, "", line)        # drop " ... ok/FAILED/ignored"
    # Remove should_panic annotation added to pretty output:
    sub(/[[:space:]]+- should panic( with `[^`]*`)?$/, "", line)
    print line
  }
' "$logfile" | sed 's/[[:space:]]*$//' > "$actual"

# 4) Decide: require (a) rc==0, (b) final summary ok, (c) every expected test appeared
if [[ "$rc" -eq 0 ]] && grep -Eq '^test result: ok\.' "$logfile"; then
  sort -u "$expected" -o "$expected"
  sort -u "$actual"   -o "$actual"
  # If no tests were expected (filters matched none), that's fine too.
  if comm -23 "$expected" "$actual" | read -r _; then
    # there were missing tests → failure
    echo "Some tests weren't ran for the exercise `$EXERCISE`. Perhaps the solution forcefully exits?"
    exit 1
  else
    exit 0
  fi
fi

exit 1
