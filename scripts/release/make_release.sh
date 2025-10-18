#!/usr/bin/env bash
set -euo pipefail

VER="${1:-v0.2.0-rc1}"
OUTDIR="build/release"
mkdir -p "$OUTDIR"

echo "== Building =="
cargo build

echo "== Running CI =="
bash scripts/ci/run_all_gates.sh

ts=$(date +"%Y-%m-%d_%H-%M-%S")
ARCHIVE="$OUTDIR/WaveML_${VER}_${ts}.zip"
echo "== Packing $ARCHIVE =="
zip -r "$ARCHIVE" . -x "target/*" "build/release/*" ".git/*" || true
shasum -a 256 "$ARCHIVE" > "${ARCHIVE}.sha256"

echo "== Release ready: $ARCHIVE =="
