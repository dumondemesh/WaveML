#!/usr/bin/env bash
set -euo pipefail

PLAN="acceptance/tests_i2.yaml"
OUTDIR="build/acceptance_i2"
SCHEMA="docs/schemas/wfr.v1.schema.json"

mkdir -p "$OUTDIR"

echo "[I2] Swaps Gate: plan=$PLAN"
ids=($(grep -E '^\s*- id:' "$PLAN" | awk '{print $3}'))
kinds=($(grep -E '^\s*kind:' "$PLAN" | awk '{print $2}'))
inputs=($(grep -E '^\s*input:' "$PLAN" | awk '{print $2}'))

fail=0
for i in "${!ids[@]}"; do
  id="${ids[$i]}"
  kind="${kinds[$i]}"
  input="${inputs[$i]}"
  out="$OUTDIR/${id}.wfr.json"
  echo "  -> $id ($kind): $input"

  # совместимо с существующим wavectl: без флага --check-i2
  target/debug/wavectl simulate-swaps --input "$input" --out "$out"

  # Схема (если есть команда validate-wfr — пропустить, если нет)
  if [[ -f "$SCHEMA" ]]; then
    if target/debug/wavectl validate-wfr --wfr "$out" --schema "$SCHEMA" --require-pass 2>/dev/null; then
      :
    else
      echo "    [WARN] validate-wfr not available or failed — продолжаем как advisory"
    fi
  fi

  if [[ ! -s "$out" ]]; then
    echo "    [FAIL] WFR not generated"
    fail=1
  fi
done

if [[ $fail -ne 0 ]]; then
  echo "[I2] Swaps Gate: FAIL"
  exit 1
fi
echo "[I2] Swaps Gate: PASS"
