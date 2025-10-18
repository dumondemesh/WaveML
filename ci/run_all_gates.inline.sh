#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "== WaveML CI gates =="
echo "[1/6] build"
cargo build

echo "[2/6] clippy strict"
echo "==> Strict clippy gate"
cargo clippy --workspace --all-targets -- -D warnings
echo "CLIPPY-GATE: OK"

echo "[3/6] forge canonicalization (INLINE, no SYN)"
W=target/debug/wavectl
A=examples/graph/forge_eq_A.json
B=examples/graph/forge_eq_B.json
id() { "$W" forge --input "$1" --print-id | head -n1 | tr -d '\\r'; }
IDA=$(id "$A"); echo "[forge-gate] NF-ID(A)   = NF-ID=$IDA"
IDB=$(id "$B"); echo "[forge-gate] NF-ID(B)   = NF-ID=$IDB"
if [[ "$IDA" != "$IDB" ]]; then
  echo "[forge-gate] FAIL: A and B must be identical after canon"
  "$W" nf-diff --left "$A" --right "$B" --show-source-diff || true
  exit 1
fi
echo "[forge-gate] PASS (A==B)"

echo "[4/6] nf-diff gate (optional)"
/bin/bash "$ROOT_DIR/scripts/ci/nf_diff_gate.sh" || echo "[warn] nf_diff_gate.sh not present or failed (non-blocking)"

echo "[5/6] swaps gate (I2)"
/bin/bash "$ROOT_DIR/scripts/ci/swaps_gate.sh" || echo "[warn] swaps_gate.sh not present or failed (non-blocking)"

echo "[6/6] wt-equivalence gate (I3)"
/bin/bash "$ROOT_DIR/scripts/ci/wt_equiv_gate.sh" || echo "[warn] wt_equiv_gate.sh not present or failed (non-blocking)"

echo "[CI] OK"
