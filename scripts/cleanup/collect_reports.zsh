#!/usr/bin/env zsh
set -euo pipefail

outdir="build/reports"
mkdir -p "$outdir"

echo "== Collect .wfr.json reports → $outdir =="
while IFS= read -r -d '' f; do
  base="$(basename "$f")"
  dest="$outdir/$base"
  if [[ -e "$dest" ]]; then
    suf=1
    while [[ -e "$outdir/${base%.json}_$suf.json" ]]; do ((suf++)); done
    dest="$outdir/${base%.json}_$suf.json"
  fi
  echo "→ $dest"
  mv "$f" "$dest"
done < <(find . -type f -name "*.wfr.json" -print0)

echo "== Done =="
