#!/usr/bin/env bash
set -euo pipefail

FILE="crates/linters/src/lib.rs"

if [[ ! -f "$FILE" ]]; then
  echo "ERROR: $FILE not found. Run this from the repo root."
  exit 1
fi

cp "$FILE" "${FILE}.bak.linters_bool_fix"

# Replace: if !sta && !(edge == "reflect" || edge.eq_ignore_ascii_case("toeplitz"))
# With:    if !(sta || edge == "reflect" || edge.eq_ignore_ascii_case("toeplitz"))
# Be tolerant to whitespace.
perl -0777 -i -pe 's/if\s+!sta\s*&&\s*!\(\s*edge\s*==\s*\"reflect\"\s*\|\|\s*edge\.eq_ignore_ascii_case\(\"toeplitz\"\)\s*\)/if !(sta || edge == "reflect" || edge.eq_ignore_ascii_case("toeplitz"))/g' "$FILE"

echo "Applied boolean simplification to $FILE"
