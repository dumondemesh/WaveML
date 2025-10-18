#!/usr/bin/env bash
set -euo pipefail
echo "== Gates (RC1g unified) =="

# Workspace fix (resolver=2, existing crates only)
bash scripts/repair/fix_workspace.sh || true

echo "== build =="
cargo build
cargo clippy --workspace --all-targets -- -D warnings

echo "== I2: swaps (stub-enabled) =="
target/debug/wavectl simulate-swaps --input "${SWAPS_INPUT:-acceptance/data/sample1.wml}" --out build/acceptance/swaps_report.wfr.json
python3 tools/delta-l-augment.py build/acceptance/swaps_report.wfr.json || true
echo "[DUMP] mdl.i2:"
jq '.mdl.i2' build/acceptance/swaps_report.wfr.json || true

echo "== I3: wt-equiv (stub-enabled) =="
target/debug/wt-equiv --out build/acceptance/wt_equiv.wfr.json
bash tools/wt-metrics-alias.sh build/acceptance/wt_equiv.wfr.json || true
echo "[DUMP] metrics:"
jq '.metrics' build/acceptance/wt_equiv.wfr.json || true

echo "== Perf: deterministic manifest =="
bash scripts/ci/perf_determinize.sh || true
cat build/reports/manifest.txt 2>/dev/null || true

echo "== RC1g unified: DONE =="
