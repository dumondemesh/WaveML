#!/usr/bin/env bash
set -euo pipefail

echo "== WaveML CI gates =="
echo "[1/4] build"
cargo build

echo "[2/4] clippy strict"
cargo clippy --workspace --all-targets -- -D warnings

echo "[3/4] COLA -> WFR"
target/debug/wavectl cola --n-fft 1024 --hop 512 --window Hann --mode amp --out build/reports/auto_amp.wfr.json

echo "[4/4] validate WFR (require-pass)"
target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass

echo "== Gates: PASS =="
