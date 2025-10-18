#!/usr/bin/env bash
set -euo pipefail
# Validate that WFR has no nulls in critical sections and required keys exist.
f="${1:-}"
if [[ -z "$f" || ! -f "$f" ]]; then
  echo "Usage: $0 file.wfr.json" >&2
  exit 2
fi

fail=0
check_keys='["mdl","w_params","w_perf","metrics"]'
for k in $(echo "$check_keys" | jq -r '.[]'); do
  v=$(jq -r --arg k "$k" '.[$k]' "$f")
  if [[ "$v" == "null" ]]; then
    echo "[FAIL] $f: key $k is null"
    fail=1
  fi
done
# required w_params keys
for k in n_fft hop window; do
  ok=$(jq -r --arg k "$k" '.w_params|has($k)' "$f")
  if [[ "$ok" != "true" ]]; then
    echo "[FAIL] $f: w_params.$k is missing"
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "[OK] $f valid"
else
  exit 3
fi
