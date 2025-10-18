#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-build_migrated}"
OUT="$BASE/overview.md"
if [[ ! -f "$OUT" ]]; then
  echo "OVERVIEW-GATE: overview.md not found under $BASE" >&2
  exit 1
fi
if [[ ! -s "$OUT" ]]; then
  echo "OVERVIEW-GATE: overview.md is empty" >&2
  exit 2
fi
echo "OVERVIEW-GATE: OK ($OUT)"
