#!/usr/bin/env bash
set -euo pipefail
# Local dev convenience: end-to-end acceptance + gates
BUILD_DIR="${1:-build}"
MIGRATED_DIR="${2:-build_migrated}"
I23_DIR="${3:-build_i23}"

echo "==> cargo build"
cargo build

echo "==> acceptance all"
bash ci/run_acceptance_all.sh "$BUILD_DIR" "$MIGRATED_DIR" "$I23_DIR"

echo "==> schema gate"
python3 ci/wfr_schema_gate.py "$MIGRATED_DIR"

echo "==> publish overview"
bash ci/publish_overview.sh "$MIGRATED_DIR"
bash ci/overview_gate.sh "$MIGRATED_DIR"

echo "==> DONE"
