#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-build_migrated}"
echo "==> Filling metrics in ${BASE} (amp mode, v2.1)"
python3 tools/fill_metrics.py --base "${BASE}" --rel-tol 1e-12

echo "==> Selecting WFRs with w_params.n_fft/hop for validation"
VAL_LIST=$(python3 tools/list_validatable_wfrs.py --base "${BASE}")
if [ -z "${VAL_LIST}" ]; then
  echo "No validatable WFRs found (with w_params.n_fft/hop)."
  exit 0
fi

echo "==> Re-validate selected reports"
while IFS= read -r f; do
  echo "Validate: $f"
  ./target/debug/wavectl validate-wfr --wfr "$f" --require-pass
done <<< "${VAL_LIST}"
echo "OK"
