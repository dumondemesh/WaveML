#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RA="$ROOT_DIR/scripts/ci/run_all_gates.sh"
echo "[ci-doctor] run_all:  $RA"
if command -v shasum >/dev/null 2>&1; then echo "[sha256] $(shasum -a 256 "$RA" | awk '{print $1}')"; fi
head -n 60 "$RA" || true
