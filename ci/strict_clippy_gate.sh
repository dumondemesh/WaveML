#!/usr/bin/env bash
set -euo pipefail
echo "==> Strict clippy gate"
cargo clippy --all-targets --all-features -- -D warnings
echo "CLIPPY-GATE: OK"
