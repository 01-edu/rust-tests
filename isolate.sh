#!/usr/bin/env bash
# Use:
#   EXERCISE="" bash -c 'cargo --config '\''target."cfg(all())".runner="./isolate.sh"'\'' test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"'

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
"$bin" "${filtered[@]}" 2>&1 | tee "$logfile"
rc="${PIPESTATUS[0]}"

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

# 4) Decide overall success/failure
sort -u "$expected" -o "$expected"
sort -u "$actual"   -o "$actual"

missing=0
if comm -23 "$expected" "$actual" | read -r _; then
  echo "Some tests weren't ran for the exercise \`$EXERCISE\`. Perhaps the solution forcefully exits?"
  missing=1
fi

if [[ "$rc" -eq 0 ]] && grep -Eq '^test result: ok\.' "$logfile" && [[ $missing -eq 0 ]]; then
  exit 0
fi

exit 1
