#!/usr/bin/env zsh
set -euo pipefail

DRY=0
if [[ "${1:-}" == "--dry-run" ]]; then DRY=1; fi

echo "== WaveML Clean v3 =="

dirs=( "target" "build" ".idea" ".vscode" "node_modules" "dist" )
for d in "${dirs[@]}"; do
  if [[ -e "$d" ]]; then
    if [[ $DRY -eq 1 ]]; then echo "[DRY] rm -rf $d"; else echo "rm -rf $d"; rm -rf "$d"; fi
  fi
done

# Remove common junk
while IFS= read -r -d '' d; do
  if [[ $DRY -eq 1 ]]; then echo "[DRY] rm -rf $d"; else echo "rm -rf $d"; rm -rf "$d"; fi
done < <(find . -type d -name "__pycache__" -print0 2>/dev/null || true)

for pat in "*.pyc" "*.log" "*.tmp" ".DS_Store"; do
  while IFS= read -r -d '' f; do
    if [[ $DRY -eq 1 ]]; then echo "[DRY] rm -f $f"; else echo "rm -f $f"; rm -f "$f"; fi
  done < <(find . -type f -name "$pat" -print0 2>/dev/null || true)
done

echo "== Done =="
