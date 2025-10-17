#!/usr/bin/env bash
set -euo pipefail

TAG="p1-i1-done-$(date +%Y-%m-%d)"
MSG="WaveML: Phase P1/I1 finalized (STRICT-NF, NF-ID, CI green)"

if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "[tag_p1_i1] Not a git repo here. Init first: git init && git add -A && git commit -m 'init'"
  exit 1
fi

# commit everything if there are staged/unstaged changes
if ! git diff --quiet || ! git diff --cached --quiet; then
  git add -A
  git commit -m "P1/I1: finalize (STRICT-NF, NF-ID, green gates)" || true
fi

# create annotated tag if not exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "[tag_p1_i1] Tag $TAG already exists."
else
  git tag -a "$TAG" -m "$MSG"
  echo "[tag_p1_i1] Created tag $TAG"
fi

# Try to push tags if remote is configured
if git remote >/dev/null 2>&1; then
  git push --tags || true
fi

echo "[tag_p1_i1] Done."
