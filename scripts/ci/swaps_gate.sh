#!/usr/bin/env bash
set -euo pipefail
# --- CLI detection helpers ---
has_wavectl() { command -v wavectl >/dev/null 2>&1; }
has_wt_equiv_bin() { command -v wt-equiv >/dev/null 2>&1; }
wavectl_has_subcmd() { has_wavectl && wavectl --help 2>/dev/null | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }


echo "== I2: swaps_gate =="

REPORT="build/acceptance/swaps_report.wfr.json"
mkdir -p "$(dirname "$REPORT")"

if wavectl_has_subcmd "simulate-swaps"; then
  if ! cargo run -p wavectl --bin wavectl -- simulate-swaps --out "$REPORT"; then
    echo "[FAIL] simulate-swaps failed"
    exit 1
  fi
  if jq -e '.delta_L_struct_max <= 0' "$REPORT" >/dev/null 2>&1; then
    echo "[OK] ΔL_struct ≤ 0"
  else
    echo "[FAIL] ΔL_struct violation (see $REPORT)"
    exit 1
  fi
else
  echo "[WARN] 'simulate-swaps' not available; skipping I2 gate"
fi
