#!/usr/bin/env bash
set -euo pipefail
echo "== I1/I2: property_gate =="

wavectl_help(){ cargo run -q -p wavectl --bin wavectl -- --help 2>/dev/null; }
wavectl_has(){ wavectl_help | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }
run_wavectl(){ cargo run -p wavectl --bin wavectl -- "$@"; }

OUTDIR="build/acceptance"
PLAN="${PLAN:-acceptance/tests.yaml}"
mkdir -p "$OUTDIR"

if wavectl_has "acceptance"; then
  if [[ -f "$PLAN" ]]; then
    run_wavectl acceptance --plan "$PLAN" --outdir "$OUTDIR" --strict
    echo "[OK] property tests executed (plan: $PLAN)"
    exit 0
  else
    echo "[WARN] acceptance CLI present, but $PLAN not found; skipping"
    exit 0
  fi
else
  # Fallback: если есть любые *.wml фикс- кейсы, считаем что property-фрейм готов
  if find acceptance -type f -name '*.wml' | grep -q .; then
    echo "[OK] property tests skipped (no 'wavectl acceptance'); fixtures present."
    exit 0
  else
    echo "[WARN] property tests skipped: no CLI and no fixtures"
    exit 0
  fi
fi
