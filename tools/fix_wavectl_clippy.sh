#!/usr/bin/env bash
set -euo pipefail

FILE="crates/wavectl/src/main.rs"
if [[ ! -f "$FILE" ]]; then
  echo "ERROR: $FILE not found. Run from the repo root."
  exit 1
fi

python3 - "$FILE" <<'PY'
import sys
p = sys.argv[1]
with open(p, 'r', encoding='utf-8') as f:
    s = f.read()
orig = s

# 1) Remove needless borrow in File::open(&input)? -> File::open(input)?
s = s.replace('File::open(&input)?', 'File::open(input)?')

# 2) Replace comparison to empty string: op != "" -> !op.is_empty()
s = s.replace('op != ""', '!op.is_empty()')

if s != orig:
    with open(p, 'w', encoding='utf-8') as f:
        f.write(s)
    print(f"Applied clippy fixes to {p}")
else:
    print("No changes applied (patterns not found).")
PY
