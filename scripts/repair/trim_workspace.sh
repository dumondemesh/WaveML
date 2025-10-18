#!/usr/bin/env bash
set -euo pipefail
if [[ ! -f Cargo.toml ]]; then
  echo "[ERR] Cargo.toml not found in current directory"; exit 2
fi
python3 tools/trim_workspace.py Cargo.toml
