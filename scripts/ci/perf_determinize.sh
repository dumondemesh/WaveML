#!/usr/bin/env bash
set -euo pipefail

# Stabilize batch ordering by sorting report lines by nf_id_hex (or path fallback).
# Advisory: If batch produced multiple .wfr.json files in different order, re-write
# a stable manifest `build/reports/manifest.txt` sorted deterministically.

REPORT_DIR="${REPORT_DIR:-build/reports}"
MANIFEST="${REPORT_DIR}/manifest.txt"
mkdir -p "$REPORT_DIR"

tmpfile="$(mktemp)"
if ls "${REPORT_DIR}"/*.wfr.json >/dev/null 2>&1; then
  for f in "${REPORT_DIR}"/*.wfr.json; do
    nf="$(
      jq -r '(.nf_id_hex // .nf_id // "") as $n | 
             if ($n|length)>0 then $n else (.input_path // .id // input_filename) end' "$f" \
      | sed 's/[[:space:]]\+/_/g'
    )"
    printf "%s\t%s\n" "${nf}" "${f}" >> "$tmpfile"
  done
  LC_ALL=C sort -t$'\t' -k1,1 "$tmpfile" | awk -F'\t' '{print $2}' > "$MANIFEST"
  echo "[OK] perf: deterministic manifest written → $MANIFEST"
else
  echo "[WARN] perf: no .wfr.json in ${REPORT_DIR} — skipping"
fi
rm -f "$tmpfile"
