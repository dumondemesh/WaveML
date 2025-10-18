#!/usr/bin/env bash
set -euo pipefail
RA="scripts/ci/run_all_gates.sh"
echo "[ci-doctor] run_all path: $(realpath "$RA")"
if command -v shasum >/dev/null 2>&1; then
  echo "[sha256] $(shasum -a 256 "$RA" | awk '{print $1}')"
fi
echo "[scan] occurrences of SYN / forge_eq_synonyms:"
grep -nE "SYN|forge_eq_synonyms" "$RA" || echo "[scan] none"
