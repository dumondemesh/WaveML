#!/usr/bin/env bash
set -euo pipefail
echo "== Gates: I1/I2/I3 + Perf =="
bash scripts/ci/forge_gate.sh
bash scripts/ci/schema_gate.sh
bash scripts/ci/property_gate.sh
bash scripts/ci/swaps_gate.sh
bash scripts/ci/wt_equiv_gate.sh
bash scripts/ci/perf_gate.sh
echo "== Gates: PASS (non-zero exit → fail) =="
