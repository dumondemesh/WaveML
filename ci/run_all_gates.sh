#!/usr/bin/env bash
set -euo pipefail
echo "==> Run all local gates"
bash ci/strict_clippy_gate.sh
bash ci/unit_test_gate.sh
bash ci/deps_gate.sh
bash ci/docs_gate.sh
# existing gates assumed present in repo:
if [[ -f "ci/forge_gate.sh" ]]; then bash ci/forge_gate.sh; fi
if [[ -f "ci/overview_gate.sh" ]]; then bash ci/overview_gate.sh build_migrated; fi
if [[ -f "ci/wfr_schema_gate.py" ]]; then python3 ci/wfr_schema_gate.py build_migrated; fi
echo "ALL-GATES: OK"
