#!/usr/bin/env bash
# Patch wavectl files to use threads: Some(0) instead of threads: 0
set -euo pipefail

FILES=(
  "crates/wavectl/src/cmd_cola.rs"
  "crates/wavectl/src/cmd_simulate_swaps.rs"
  "crates/wavectl/src/cmd_report_from_graph.rs"
)

for f in "${FILES[@]}"; do
  if [[ -f "$f" ]]; then
    echo "[patch] $f"
    cp "$f" "${f}.bak_threads_fix"
    if sed --version >/dev/null 2>&1; then
      sed -i 's/threads:[[:space:]]*0/threads: Some(0)/g' "$f"
    else
      sed -i '' 's/threads:[[:space:]]*0/threads: Some(0)/g' "$f"
    fi
    grep -n "threads: Some(0)" "$f" || { echo "[ERR] Replacement failed for $f"; exit 1; }
  else
    echo "[WARN] Missing file: $f (skipping)"
  fi
done

echo "[ok] threads literals replaced with Some(0). Try building now:"
echo "     cargo build"
