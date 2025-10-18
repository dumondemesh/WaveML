#!/usr/bin/env bash
set -euo pipefail
echo "== Forge Gate =="

A="examples/graph/forge_eq_A.json"
B="examples/graph/forge_eq_B.json"
S="$B"  # bind SYN to B to avoid stale files

aid=$(target/debug/wavectl forge --input "$A" --print-id | head -n1 | tr -d '\r')
bid=$(target/debug/wavectl forge --input "$B" --print-id | head -n1 | tr -d '\r')
sid=$(target/debug/wavectl forge --input "$S" --print-id | head -n1 | tr -d '\r')

echo "[forge-gate] NF-ID(A)     = NF-ID=$aid"
echo "[forge-gate] NF-ID(B)     = NF-ID=$bid"
echo "[forge-gate] NF-ID(SYN=B) = NF-ID=$sid"

if [[ "$aid" != "$bid" ]]; then
  echo "[forge-gate] FAIL: Equivalent graphs must have identical NF-ID"
  target/debug/wavectl nf-diff --left "$A" --right "$B" --show-source-diff || true
  exit 1
fi

echo "[forge-gate] PASS"
