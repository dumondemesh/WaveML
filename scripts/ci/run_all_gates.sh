#!/usr/bin/env bash
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="$HERE/..:$PATH"

echo "== Gates: I1/I2/I3 + Perf =="
bash "$HERE/forge_gate.sh"
bash "$HERE/schema_gate.sh"
bash "$HERE/property_gate.sh"
bash "$HERE/swaps_gate.sh"
bash "$HERE/wt_equiv_gate.sh"
bash "$HERE/perf_gate.sh"
echo "== Gates: PASS (non-zero exit → fail) =="
