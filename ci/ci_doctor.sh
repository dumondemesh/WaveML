#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FG="$ROOT_DIR/scripts/ci/forge_gate.sh"
RA="$ROOT_DIR/scripts/ci/run_all_gates.sh"
echo "[ci-doctor] forge_gate: $FG"
if command -v shasum >/dev/null 2>&1; then echo "[sha256] $(shasum -a 256 "$FG" | awk '{print $1}')"; fi
head -n 20 "$FG" || true
echo "-------------------"
echo "[ci-doctor] run_all:  $RA"
if command -v shasum >/dev/null 2>&1; then echo "[sha256] $(shasum -a 256 "$RA" | awk '{print $1}')"; fi
head -n 40 "$RA" || true
