#!/usr/bin/env bash
set -euo pipefail

VER="${1:-}"
if [[ -z "$VER" ]]; then
  echo "Usage: $0 vX.Y.Z-rcN" >&2
  exit 2
fi

echo "== Release ${VER} =="
# Create tag if git repo
if git rev-parse --git-dir >/dev/null 2>&1; then
  git add -A
  git commit -m "chore(release): ${VER} (F5 RC) [skip ci]" || true
  git tag -f "${VER}"
  echo "[OK] git tag ${VER}"
else
  echo "[WARN] not a git repo — tag not created"
fi

out="WaveML_${VER}_artifacts.zip"
mkdir -p build/release
zip -q -r "build/release/${out}" build/reports acceptance/docs 2>/dev/null || true
echo "[OK] release bundle → build/release/${out}"
