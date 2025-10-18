#!/usr/bin/env bash
set -euo pipefail
BASE=${1:-build_migrated}
OUT=${2:-out}
mkdir -p "$OUT"
echo "==> Packaging overview files from $BASE"
if [[ -f "$BASE/overview.md" ]]; then cp "$BASE/overview.md" "$OUT/overview.md"; fi
if [[ -f "$BASE/overview.html" ]]; then cp "$BASE/overview.html" "$OUT/overview.html"; fi
echo "==> Packaging WFRs (tar.gz)"
tar -czf "$OUT/wfr_bundle.tgz" $(find "$BASE" -type f -name '*.wfr.json') 2>/dev/null || true
echo "RELEASE-ARTIFACTS: OK → $OUT"
