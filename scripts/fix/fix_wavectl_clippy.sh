#!/usr/bin/env bash
set -euo pipefail

FILE="crates/wavectl/src/main.rs"

if [[ ! -f "$FILE" ]]; then
  echo "[fix] File not found: $FILE" >&2
  exit 1
fi

ts=$(date +"%Y%m%d_%H%M%S")
cp "$FILE" "${FILE}.bak.${ts}"

# 1) remove unused `json` import in serde_json
#    from: use serde_json::{json, Map, Value};
#    to:   use serde_json::{Map, Value};
if grep -q 'use serde_json::{json, Map, Value};' "$FILE"; then
  sed -i '' 's/use serde_json::{json, Map, Value};/use serde_json::{Map, Value};/' "$FILE"
fi

# 2) remove unnecessary mut on canonicalize_graph_object result
#    let mut g = canonicalize_graph_object(&graph)?; -> let g = canonicalize_graph_object(&graph)?;
sed -i '' 's/let[[:space:]]\+mut[[:space:]]\+g[[:space:]]*=\s*canonicalize_graph_object(\&graph)\?;/let g = canonicalize_graph_object(&graph)?;/' "$FILE"

# 3) unwrap_or_else(|| vec![]) -> unwrap_or_default()
sed -i '' 's/unwrap_or_else(\(\) => vec!\[\]\)/unwrap_or_default()/g' "$FILE"

# A safer variant for occurrences like: ... .unwrap_or_else(|| vec![]);
sed -i '' 's/unwrap_or_else( *\( *\) *=> *vec!\[\] *)/unwrap_or_default()/g' "$FILE"

# And a generic replacement for redundant closure creating vec![]
sed -i '' 's/unwrap_or_else( *\(.*\) *=> *vec!\[\] *)/unwrap_or_default()/g' "$FILE"

# 4) needless borrow on `op` in canon_params(&op, ... ) -> canon_params(op, ...)
sed -i '' 's/canon_params(&op, /canon_params(op, /' "$FILE"

# 5) if_same_then_else: if other.is_empty() { "W" } else { "W" } -> "W"
sed -i '' 's/if[[:space:]]\+other\.is_empty()[[:space:]]*{[[:space:]]*"W"[[:space:]]*}[[:space:]]*else[[:space:]]*{[[:space:]]*"W"[[:space:]]*}/"W"/' "$FILE"

# 6) redundant guard: `other if other.is_empty() => "Hann",` -> `"" => "Hann",`
#    Keep it narrow to avoid touching other matches arms.
sed -i '' 's/other[[:space:]]\+if[[:space:]]\+other\.is_empty()[[:space:]]*=>[[:space:]]*"Hann",/"" => "Hann",/' "$FILE"

echo "[fix] Patched clippy findings in $FILE"
echo "[fix] Backup: ${FILE}.bak.${ts}"
echo "[fix] Now run: cargo clippy --workspace --all-targets -- -D warnings
cargo build
bash scripts/ci/run_all_gates.sh"
