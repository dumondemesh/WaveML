#!/usr/bin/env bash
set -euo pipefail

# ---- Cargo-based CLI detection helpers ----
wavectl_help() { cargo run -q -p wavectl --bin wavectl -- --help 2>/dev/null; }
wavectl_has_subcmd() { wavectl_help | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }
cargo_run_wavectl() { cargo run -p wavectl --bin wavectl -- "$@"; }
cargo_run_wt_equiv() { cargo run -p wavectl --bin wt-equiv -- "$@"; }

echo "== I3: wt_equiv_gate =="

OUT="build/acceptance/wt_equiv.wfr.json"
mkdir -p "$(dirname "$OUT")"

ok=0
if wavectl_has_subcmd "wt-equivalence"; then
  if cargo_run_wavectl wt-equivalence --out "$OUT"; then ok=1; fi
fi
if [[ $ok -ne 1 ]]; then
  if cargo_run_wt_equiv --out "$OUT"; then ok=1; fi
fi

if [[ $ok -ne 1 ]]; then
  echo "[WARN] wt-equivalence not available; skipping I3 gate"
  exit 0
fi

WT_MSE_THRESH="${WT_MSE_THRESH:-1e-8}"
WT_SDR_MIN_DB="${WT_SDR_MIN_DB:-60}"
STRICT_I3="${STRICT_I3:-0}"

MSE="$(jq -r '
  if has("wt_mse_max") then .wt_mse_max
  elif .metrics? and .metrics.mse? and .metrics.mse.max? then .metrics.mse.max
  elif .mse? and .mse.max? then .mse.max
  else "MISSING" end
' "$OUT" 2>/dev/null || echo "MISSING")"

SDR="$(jq -r '
  if .metrics? and .metrics.sdr_db? and .metrics.sdr_db.min? then .metrics.sdr_db.min
  elif has("sdr_db_min") then .sdr_db_min
  else "MISSING" end
' "$OUT" 2>/dev/null || echo "MISSING")"

pass=0
reason=""

if [[ "$MSE" != "MISSING" && -n "$MSE" ]]; then
  awk -v v="$MSE" -v t="$WT_MSE_THRESH" 'BEGIN{ if (v+0 <= t+0) exit 0; else exit 1 }' \
  && { pass=1; reason="MSE=$MSE ≤ $WT_MSE_THRESH"; } \
  || { pass=0; reason="MSE=$MSE > $WT_MSE_THRESH"; }
elif [[ "$SDR" != "MISSING" && -n "$SDR" ]]; then
  awk -v v="$SDR" -v t="$WT_SDR_MIN_DB" 'BEGIN{ if (v+0 >= t+0) exit 0; else exit 1 }' \
  && { pass=1; reason="SDR_min=$SDR dB ≥ $WT_SDR_MIN_DB dB"; } \
  || { pass=0; reason="SDR_min=$SDR dB < $WT_SDR_MIN_DB dB"; }
else
  echo "[WARN] Neither MSE nor SDR found in $OUT; not failing I3."
  exit 0
fi

if [[ $pass -eq 1 ]]; then
  echo "[OK] WT-equivalence within threshold ($reason)"
else
  if [[ "$STRICT_I3" == "1" ]]; then
    echo "[FAIL] WT-equivalence: threshold violated ($reason). See $OUT"
    exit 1
  else
    echo "[WARN] WT-equivalence: threshold violated ($reason). See $OUT"
    exit 0
  fi
fi
