#!/usr/bin/env bash
set -euo pipefail
TARGET="scripts/ci/run_all_gates.sh"
echo "[doctor] path: $(realpath "$TARGET")"
if command -v shasum >/dev/null 2>&1; then
  echo "[sha256] $(shasum -a 256 "$TARGET" | awk '{print $1}')"
fi
echo "[scan] forbidden tokens:"
grep -nE "SYN|forge_eq_synonyms|id\(" "$TARGET" || echo "[scan] none (ok)"
echo "[scan] ensure nfid() exists:"
grep -n "nfid()" "$TARGET" && echo "[ok] nfid() present"
