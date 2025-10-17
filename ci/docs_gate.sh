#!/usr/bin/env bash
set -euo pipefail
echo "==> Docs gate"
missing=0
need=('docs/STRICT-NF.md')
for f in "${need[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "Missing: $f"
    missing=1
  fi
done
if [[ $missing -ne 0 ]]; then
  echo "DOCS-GATE: FAIL"
  exit 1
fi
echo "DOCS-GATE: OK"
