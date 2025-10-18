#!/usr/bin/env bash
set -euo pipefail
echo "==> Deps gate: forbid '+deprecated' serde_yaml pins"
if grep -R --line-number --include='Cargo.toml' '\+deprecated' crates || true; then
  if grep -R --line-number --include='Cargo.toml' 'serde_yaml\s*=\s*".*\+deprecated' crates ; then
    echo "DEPS-GATE: FAIL (serde_yaml has +deprecated)"
    exit 1
  fi
fi
echo "DEPS-GATE: OK"
