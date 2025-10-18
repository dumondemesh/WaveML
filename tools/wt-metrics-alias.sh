#!/usr/bin/env bash
set -euo pipefail
f="${1:-}"; [[ -z "$f" || ! -f "$f" ]] && { echo "Usage: $0 file.wfr.json" >&2; exit 2; }
tmp="$(mktemp)"
jq '
  .metrics = (.metrics // {}) |
  (if (.metrics.sdr_db|type)=="number" then .
   elif (.metrics.snr_db|type)=="number" then .metrics.sdr_db = .metrics.snr_db | .
   else . end)
' "$f" > "$tmp"
mv "$tmp" "$f"
echo "[OK] aliased sdr_db from snr_db (if needed) in $f"
