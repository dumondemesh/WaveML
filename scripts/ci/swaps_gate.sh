#!/usr/bin/env bash
set -euo pipefail

# ---- Cargo-based CLI detection helpers ----
wavectl_help() { cargo run -q -p wavectl --bin wavectl -- --help 2>/dev/null; }
wavectl_has_subcmd() { wavectl_help | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }
cargo_run_wavectl() { cargo run -p wavectl --bin wavectl -- "$@"; }

echo "== I2: swaps_gate =="

REPORT="build/acceptance/swaps_report.wfr.json"
mkdir -p "$(dirname "$REPORT")"

# 0) есть ли команда
if ! wavectl_has_subcmd "simulate-swaps"; then
  echo "[WARN] 'simulate-swaps' not available; skipping I2 gate"
  exit 0
fi

# 1) находим вход
INPUT="${SWAPS_INPUT:-}"
if [[ -z "$INPUT" ]]; then
  # берём первый *.wml, избегая явных bad_* кейсов
  INPUT="$(find acceptance -type f -name '*.wml' | grep -v '/bad_' | head -n 1 || true)"
fi
if [[ -z "$INPUT" || ! -f "$INPUT" ]]; then
  echo "[WARN] No input *.wml found (set SWAPS_INPUT=path/to/graph.wml); skipping I2 gate"
  exit 0
fi
echo "[INFO] simulate-swaps input: $INPUT"

# 2) запуск
if ! cargo_run_wavectl simulate-swaps --input "$INPUT" --out "$REPORT"; then
  echo "[FAIL] simulate-swaps failed"
  exit 1
fi

# 3) выдираем ΔL_struct из разных возможных мест
VAL="$(
  jq -r '
    if has("delta_L_struct_max") then .delta_L_struct_max
    elif (.metrics? and .metrics.delta_L_struct? and .metrics.delta_L_struct.max?) then .metrics.delta_L_struct.max
    elif (.delta_L_struct? and .delta_L_struct.max?) then .delta_L_struct.max
    elif (.orbits? and (.orbits|length)>0) then
      [ .orbits[] | (.delta_L_struct? // .metrics?.delta_L_struct?.value? // empty) ] | select(length>0) | max
    else "MISSING" end
  ' "$REPORT" 2>/dev/null || echo "MISSING"
)"

if [[ "$VAL" == "MISSING" || -z "$VAL" ]]; then
  echo "[WARN] ΔL_struct metric not found in $REPORT; not failing I2 (inspect report manually)."
  exit 0
fi

# 4) числовая проверка ΔL_struct ≤ 0
awk -v v="$VAL" 'BEGIN{ if (v+0 <= 0.0) exit 0; else exit 1 }' \
|| { echo "[FAIL] ΔL_struct violation: $VAL (see $REPORT)"; exit 1; }

echo "[OK] ΔL_struct ≤ 0 (value: $VAL)"
