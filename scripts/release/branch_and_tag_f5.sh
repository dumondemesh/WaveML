#!/usr/bin/env bash
set -euo pipefail

BRANCH="${1:-phase/f5}"
TAG="${2:-v0.2.0-rc1}"
MSG="${3:-WaveML Phase 5 RC (RC1g)}"

# Create/switch branch
if git rev-parse --verify "$BRANCH" >/dev/null 2>&1; then
  git checkout "$BRANCH"
else
  git checkout -b "$BRANCH"
fi

# Commit everything that changed (no-op if clean)
git add -A
if git diff --cached --quiet; then
  echo "[INFO] nothing to commit"
else
  git commit -m "feat(f5): finalize Phase 5 — unified CI (RC1g), resolver=2, I1–I3 green, perf manifest"
fi

# Tag (no-op if exists)
if git rev-parse --verify "$TAG" >/dev/null 2>&1; then
  echo "[INFO] tag exists: $TAG"
else
  git tag -a "$TAG" -m "$MSG"
fi

echo "[OK] Branch '$BRANCH' ready; tag '$TAG' present."
echo "Push with: git push -u origin $BRANCH --tags"
