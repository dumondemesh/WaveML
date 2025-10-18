#!/usr/bin/env bash
set -euo pipefail

SCHEMA="schemas/graph.schema.json"

if [[ ! -f "$SCHEMA" ]]; then
  echo "[schema_gate] Missing $SCHEMA"
  exit 1
fi
echo "[schema_gate] OK: schema present at $SCHEMA"

if [[ -d "examples/graph" ]]; then
  shopt -s nullglob
  for f in examples/graph/forge*.json; do
    [[ -e "$f" ]] || continue
    if ! jq -e 'has("op")' "$f" >/dev/null; then
      echo "[schema_gate] FAIL: $f has no 'op' field"
      exit 1
    fi
  done
  echo "[schema_gate] forge* examples basic check: PASS"
fi
