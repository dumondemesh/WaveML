#!/usr/bin/env bash
set -euo pipefail
echo "==> Unit tests (workspace)"
cargo test --all
echo "UNIT-TEST-GATE: OK"
