#!/usr/bin/env bash
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$HERE/.." && pwd)"
SRC_DIR="$HERE/../crates/wavectl"
DST_DIR="$REPO_ROOT/crates/wavectl"

echo "[replace] removing old $DST_DIR"
rm -rf "$DST_DIR"
mkdir -p "$REPO_ROOT/crates"
echo "[replace] copying wavectl"
cp -a "$SRC_DIR" "$DST_DIR"
echo "[replace] done. sentinel should exist:"
grep -R "WAVECTL_SENTINEL_v030_MINIMAL_WFR_V1_ONLY" "$DST_DIR" >/dev/null && echo "[ok] sentinel found" || (echo "[warn] sentinel NOT found"; exit 1)
