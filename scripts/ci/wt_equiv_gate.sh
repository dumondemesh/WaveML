#!/usr/bin/env bash
set -euo pipefail

# WT equivalence (I3) — robust mode without assuming wavectl flags.
# Strategy:
#  1) Look for precomputed WFRs (build/wt_equiv/*.wfr.json or build/wt/*.wfr.json).
#  2) Validate metrics against acceptance/thresholds.yaml using wfr_check_metrics.py.
#  3) If nothing found, fail with clear guidance.

THR_FILE="${THR_FILE:-acceptance/thresholds.yaml}"
OUT_DIR="${OUT_DIR:-build/wt_equiv}"
CHECKER="${CHECKER:-scripts/ci/wfr_check_metrics.py}"

mkdir -p "$OUT_DIR"

# Candidate files (priority order)
CANDS=(
  "build/wt_equiv/i3_sine.wfr.json"
  "build/wt_equiv/i3_sweep.wfr.json"
  "build/wt/sine.wfr.json"
  "build/wt/sweep.wfr.json"
)

FOUND=()
for f in "${CANDS[@]}"; do
  if [[ -f "$f" ]]; then
    FOUND+=("$f")
  fi
done

# If not found in defaults, scan repo for reasonable matches
if [[ ${#FOUND[@]} -lt 2 ]]; then
  while IFS= read -r f; do
    FOUND+=("$f")
  done < <(find build -type f -name "*sine*.wfr.json" -o -name "*sweep*.wfr.json" | sort)
fi

if [[ ${#FOUND[@]} -eq 0 ]]; then
  echo "[FAIL] No WFR artifacts for WT equivalence found."
  echo "       Expected at least one of: build/wt_equiv/i3_{sine,sweep}.wfr.json or build/wt/{sine,sweep}.wfr.json"
  echo "       Either generate them in your pipeline or wire wavectl to produce them."
  exit 1
fi

# Deduplicate
UNIQ=($(printf "%s\n" "${FOUND[@]}" | awk '!seen[$0]++'))

echo "[WT] Checking ${#UNIQ[@]} WFR file(s):"
for f in "${UNIQ[@]}"; do
  echo " - $f"
done

python3 "$CHECKER" --thr "$THR_FILE" --files "${UNIQ[@]}"

echo "== WT Equiv Gate: PASS =="
