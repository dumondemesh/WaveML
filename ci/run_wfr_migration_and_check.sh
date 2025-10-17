#!/usr/bin/env bash
set -euo pipefail
SRC="${1:-build}"
DST="${2:-build_migrated}"

echo "==> Step 1: migrate WFRs to v1.0.0 into ${DST}"
python3 tools/migrate_wfr.py --src "${SRC}" --dst "${DST}" --overwrite

echo "==> Step 2: fill metrics (amp, periodic Hann) and ensure w_perf keys"
bash ci/run_fill_metrics.sh "${DST}"

echo "==> Done"
