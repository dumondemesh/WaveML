#!/usr/bin/env bash
set -euo pipefail

echo "==> Forge NF-ID stability gate"
EQ_A="examples/graph/forge_eq_A.json"
EQ_B="examples/graph/forge_eq_B.json"

if [[ ! -f "$EQ_A" || ! -f "$EQ_B" ]]; then
  # Try to auto-discover two 'forge_eq_*.json' files
  MAP=($(ls examples/graph/forge_eq_*.json 2>/dev/null || true))
  if [[ ${#MAP[@]} -ge 2 ]]; then
    EQ_A="${MAP[0]}"
    EQ_B="${MAP[1]}"
  else
    echo "SKIP: no forge_eq_* pair found; gate not enforced"
    exit 0
  fi
fi

ID_A=$(./target/debug/wavectl forge --input "$EQ_A" --print-id | tr -d '\r')
ID_B=$(./target/debug/wavectl forge --input "$EQ_B" --print-id | tr -d '\r')

echo "ID(A)=$ID_A"
echo "ID(B)=$ID_B"

if [[ "$ID_A" != "$ID_B" ]]; then
  echo "FORGE GATE: FAIL (NF-ID mismatch)"; exit 2
fi
echo "FORGE GATE: PASS"
