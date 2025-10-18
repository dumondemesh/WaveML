#!/usr/bin/env bash
set -euo pipefail

echo "== I1: schema_gate (graph.schema.json presence + JSON syntax checks) =="

SCHEMA="docs/graph.schema.json"
if [[ ! -f "$SCHEMA" ]]; then
  echo "[WARN] No schema file at $SCHEMA; skipping."
  exit 0
fi

examples=( $(ls acceptance/data/*.json 2>/dev/null || true) )
if [[ ${#examples[@]} -eq 0 ]]; then
  echo "[INFO] No JSON examples to validate; schema present."
  exit 0
fi

fail=0
for j in "${examples[@]}"; do
  if ! jq -e . "$j" >/dev/null 2>&1; then
    echo "[FAIL] invalid JSON: $j"
    fail=1
  fi
done
if [[ $fail -ne 0 ]]; then exit 1; fi

echo "[OK] JSON examples are syntactically valid (schema present)"
