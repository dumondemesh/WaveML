#!/usr/bin/env bash
set -euo pipefail

# Schema validation gate (I1) — smart validator for Graph + strict WFR with sanitize step.

SCHEMA_GRAPH_PRIMARY="${SCHEMA_GRAPH_PRIMARY:-schemas/graph.schema.json}"
SCHEMA_GRAPH_ALT="${SCHEMA_GRAPH_ALT:-spec/WMLB-1.1.schema.json}"
SCHEMA_WFR="${SCHEMA_WFR:-spec/WFR-1.0.0.schema.json}"
GRAPH_VALIDATOR="${GRAPH_VALIDATOR:-scripts/ci/smart_graph_validate.py}"
WFR_VALIDATOR="${WFR_VALIDATOR:-scripts/ci/jsonschema_validate.py}"
WFR_SANITIZER="${WFR_SANITIZER:-scripts/ci/wfr_sanitize.py}"

GRAPHS=( examples/graph/*.json build/forge/*.json )
ANY_GRAPH=0
for g in ${GRAPHS[@]}; do
  if [[ -f "$g" ]]; then
    if python3 "$GRAPH_VALIDATOR" --primary "$SCHEMA_GRAPH_PRIMARY" --alt "$SCHEMA_GRAPH_ALT" --target "$g"; then
      echo "[OK] graph schema: $g"
    else
      echo "[FAIL] graph schema: $g"; exit 1
    fi
    ANY_GRAPH=1
  fi
done
if [[ "$ANY_GRAPH" -eq 0 ]]; then
  echo "[WARN] No graph JSONs found"
fi

# Validate reports strictly against WFR schema, but sanitize 'mdl: null' -> {}
WFRS=( build/**/*.wfr.json build/*.wfr.json build/reports/*.wfr.json )
ANY_WFR=0
TMP_DIR="${TMP_DIR:-build/.schema_tmp}"
mkdir -p "$TMP_DIR"
shopt -s nullglob globstar
for r in ${WFRS[@]}; do
  if [[ -f "$r" ]]; then
    tmp="$TMP_DIR/$(basename "$r")"
    if python3 "$WFR_SANITIZER" --input "$r" --output "$tmp"; then
      python3 "$WFR_VALIDATOR" "$SCHEMA_WFR" "$tmp" || { echo "[FAIL] WFR schema: $r"; exit 1; }
      echo "[OK] WFR schema: $r"
    else
      echo "[FAIL] sanitizer failed for: $r"; exit 1
    fi
    ANY_WFR=1
  fi
done
if [[ "$ANY_WFR" -eq 0 ]]; then
  echo "[WARN] No WFR JSONs found"
fi

echo "== Schema Gate: PASS =="
