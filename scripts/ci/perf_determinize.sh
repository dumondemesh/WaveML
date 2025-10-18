#!/usr/bin/env bash
set -euo pipefail
REPORT_DIR="${REPORT_DIR:-build/reports}"
MANIFEST="${REPORT_DIR}/manifest.txt"
mkdir -p "$REPORT_DIR"
tmp="$(mktemp)"
shopt -s nullglob
files=("build/acceptance"/*.wfr.json "$REPORT_DIR"/*.wfr.json)
for f in "${files[@]}"; do
  [[ -f "$f" ]] || continue
  nf="$( jq -r '(.nf_id_hex // .nf_id // "") as $n | if ($n|length)>0 then $n else (.input_path // .id // input_filename) end' "$f" 2>/dev/null | sed 's/[[:space:]]\+/_/g' )"
  printf "%s\t%s\n" "${nf:-unknown}" "$f" >> "$tmp"
done
if [[ -s "$tmp" ]]; then
  LC_ALL=C sort -t$'\t' -k1,1 "$tmp" | awk -F'\t' '{print $2}' > "$MANIFEST"
  echo "[OK] perf: deterministic manifest → $MANIFEST"
else
  echo "[INFO] perf: no reports to manifest"
fi
rm -f "$tmp"
