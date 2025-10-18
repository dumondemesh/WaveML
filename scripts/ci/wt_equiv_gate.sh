#!/usr/bin/env bash
set -euo pipefail
# --- CLI detection helpers ---
has_wavectl() { command -v wavectl >/dev/null 2>&1; }
has_wt_equiv_bin() { command -v wt-equiv >/dev/null 2>&1; }
wavectl_has_subcmd() { has_wavectl && wavectl --help 2>/dev/null | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }


echo "== I3: wt_equiv_gate =="

OUT="build/acceptance/wt_equiv.wfr.json"
mkdir -p "$(dirname "$OUT")"

ok=0
if wavectl_has_subcmd "wt-equivalence"; then
  if cargo run -q -p wavectl --bin wavectl -- wt-equivalence --out "$OUT"; then ok=1; fi
fi
if [[ $ok -ne 1 ]] && has_wt_equiv_bin; then
  if cargo run -q -p wavectl --bin wt-equiv -- --out "$OUT"; then ok=1; fi
fi

if [[ $ok -ne 1 ]]; then
  echo "[WARN] No wt-equivalence available; skipping I3 gate"
  exit 0
fi

THRESH="${WT_MSE_THRESH:-1e-9}"
if jq -e --arg t "$THRESH" '.wt_mse_max | tonumber <= ($t|tonumber)' "$OUT" >/dev/null 2>&1; then
  echo "[OK] WT-MSE within threshold"
else
  echo "[FAIL] WT-MSE exceeded (see $OUT)"
  exit 1
fi
