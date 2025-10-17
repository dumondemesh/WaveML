#!/usr/bin/env bash
set -euo pipefail

echo "== WaveML CI gates =="

echo "[1/5] build"
cargo build

echo "[2/5] clippy strict"
if [ -f "ci/strict_clippy_gate.sh" ]; then
  bash ci/strict_clippy_gate.sh
else
  cargo clippy --all-targets --all-features -- -D warnings
fi

echo "[3/5] forge canonicalization"
bash ci/forge_gate.sh

echo "[4/5] COLA -> WFR"
mkdir -p build/reports
target/debug/wavectl cola --n-fft 1024 --hop 512 --window hann --mode amp --out build/reports/auto_amp.wfr.json

echo "[5/5] validate WFR (require-pass)"
target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass

echo "== Gates: PASS =="
