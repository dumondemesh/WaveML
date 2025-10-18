#!/usr/bin/env bash
set -euo pipefail
CARGO_FILE="${1:-Cargo.toml}"
BACKUP="${CARGO_FILE}.bak_rc1e_$(date +%s)"
[[ -f "$CARGO_FILE" ]] || { echo "[ERR] $CARGO_FILE not found"; exit 2; }

cp "$CARGO_FILE" "$BACKUP"
echo "[OK] backup → $BACKUP"

# Extract prefix/suffix around [workspace] table; rebuild members array entirely.
prefix="$(awk 'BEGIN{p=1}/^\[workspace\]/{print; p=0; next} p{print}' "$CARGO_FILE")"
wsblk="$(awk 'f{print} /^\[workspace\]/{f=1}' "$CARGO_FILE")"

# Build members list from existing crates/*/Cargo.toml
members=()
while IFS= read -r d; do
  [[ -f "$d/Cargo.toml" ]] && members+=("$d")
done < <(find crates -mindepth 1 -maxdepth 1 -type d | LC_ALL=C sort)

# Compose new Cargo.toml
{
  echo "$prefix"
  echo "[workspace]"
  echo "members = ["
  for m in "${members[@]}"; do
    echo "  \"$m\","
  done
  echo "]"
} > "${CARGO_FILE}.tmp"

mv "${CARGO_FILE}.tmp" "$CARGO_FILE"
echo "[OK] workspace members rewritten to: ${#members[@]} entries"
for m in "${members[@]}"; do echo " - $m"; done
