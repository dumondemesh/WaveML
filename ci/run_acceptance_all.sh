#!/usr/bin/env bash
set -euo pipefail

BASE_IN="${1:-build}"
BASE_MIG="${2:-build_migrated}"
BASE_I23="${3:-build_i23}"

bash ci/run_wfr_migration_and_check.sh "$BASE_IN" "$BASE_MIG"
bash ci/forge_gate.sh
bash ci/run_i23_acceptance.sh "$BASE_I23"
python3 tools/verify_i23.py "$BASE_I23"
python3 tools/acceptance_runner.py acceptance/tests_i2_i3.json
bash ci/publish_overview.sh "$BASE_MIG"
