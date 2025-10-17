#!/usr/bin/env bash
set -euo pipefail
BASE="${1:-build_migrated}"
echo "==> Build acceptance overview for $BASE → $BASE/overview.md"
python3 tools/overview_v3.py --base "$BASE" --out "$BASE/overview.md"
echo "Overview written to $BASE/overview.md"
