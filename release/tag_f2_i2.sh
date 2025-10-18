#!/usr/bin/env bash
set -euo pipefail
TAG="f2-i2-done-$(date -u +%Y-%m-%d)"
echo "[tag] ${TAG}"
git tag -f "${TAG}"
echo "[tag] created. Push with: git push -f origin ${TAG}"
