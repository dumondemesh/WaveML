#!/usr/bin/env bash
set -euo pipefail

# WaveML CI — unified runner by phases:
# F1: I1 canon/schema/stdout/property
# F2: I2 swaps/ΔL_struct
# F3: I3 WT equivalence (MSE/SDR)
# F4: perf/dx (advisory unless CI_STRICT_PERF=1)

ROOT_DIR="${ROOT_DIR:-$(pwd)}"
CI_DIR="${CI_DIR:-scripts/ci}"
LOG_DIR="${LOG_DIR:-build/ci_logs}"
mkdir -p "$LOG_DIR"

echo "== CI: Forge/NF-ID gate (I1) =="
bash "$CI_DIR/forge_gate.sh" | tee "$LOG_DIR/forge_gate.log"

echo "== CI: Schema gate (I1) =="
bash "$CI_DIR/schema_gate.sh" | tee "$LOG_DIR/schema_gate.log"

echo "== CI: Property gate (I1/I2) =="
bash "$CI_DIR/property_gate.sh" | tee "$LOG_DIR/property_gate.log"

echo "== CI: Swaps gate (I2) =="
bash "$CI_DIR/swaps_gate.sh" | tee "$LOG_DIR/swaps_gate.log"

echo "== CI: WT equivalence gate (I3) =="
bash "$CI_DIR/wt_equiv_gate.sh" | tee "$LOG_DIR/wt_equiv_gate.log"

echo "== CI: Perf gate (P2 advisory) =="
if bash "$CI_DIR/perf_gate.sh" | tee "$LOG_DIR/perf_gate.log"; then
  echo "[PERF] advisory PASS"
else
  if [[ "${CI_STRICT_PERF:-0}" == "1" ]]; then
    echo "[PERF] FAIL (strict mode)"
    exit 1
  else
    echo "[PERF] advisory FAIL — not blocking"
  fi
fi

echo "== CI: ALL DONE =="
