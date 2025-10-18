#!/usr/bin/env bash
set -euo pipefail

PLAN="acceptance/wt/tests_i3.yaml"
OUTDIR="build/wt_equiv"

mkdir -p "$OUTDIR"

ids=($(grep -E '^\s*- id:' "$PLAN" | awk '{print $3}'))
sigs=($(grep -E '^\s*signal:' "$PLAN" | awk '{print $2}'))
mse_maxs=($(grep -E '^\s*mse_max:' "$PLAN" | awk '{print $2}'))
sdr_mins=($(grep -E '^\s*sdr_min:' "$PLAN" | awk '{print $2}'))

fail=0
for i in "${!ids[@]}"; do
  id="${ids[$i]}"
  sig="${sigs[$i]}"
  mse_max="${mse_maxs[$i]}"
  sdr_min="${sdr_mins[$i]}"
  out="${OUTDIR}/${id}.wfr.json"
  echo "[WT] $id signal=$sig thresholds: mse<=${mse_max}, sdr>=${sdr_min}"

  target/debug/wt-equiv --signal "$sig" --out "$out" --mse-threshold "$mse_max" --sdr-min "$sdr_min"

  mse=$(jq -r '.metrics.mse' "$out")
  sdr=$(jq -r '.metrics.snr_db' "$out")
  ok=$(jq -r '.cert.i3_conservative_functors' "$out")

  awk -v m="$mse" -v mm="$mse_max" 'BEGIN{if(!(m<=mm)) exit 1}' || { echo "  [FAIL] MSE=$m > $mse_max"; fail=1; }
  awk -v s="$sdr" -v sm="$sdr_min" 'BEGIN{if(!(s>=sm)) exit 1}' || { echo "  [FAIL] SDR=$s < $sdr_min"; fail=1; }
  [[ "$ok" == "true" ]] || { echo "  [FAIL] cert.i3_conservative_functors=false"; fail=1; }
  echo "  [OK] $id"
done

if [[ $fail -ne 0 ]]; then
  echo "[WT] gate: FAIL"
  exit 1
fi
echo "[WT] gate: PASS"
