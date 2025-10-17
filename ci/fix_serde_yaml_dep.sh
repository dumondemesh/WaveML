#!/usr/bin/env bash
set -euo pipefail
PYTHON=${PYTHON:-python3}
$PYTHON - <<'PY'
import os, io, sys, re

paths = [
    "crates/wavectl/Cargo.toml",
]

changed = False
for p in paths:
    if not os.path.exists(p):
        continue
    s = open(p, "r", encoding="utf-8").read()
    ns = re.sub(r'(serde_yaml\s*=\s*)"0\.9\.34\+deprecated"', r'\1"0.9.34"', s)
    if ns != s:
        open(p, "w", encoding="utf-8").write(ns)
        print(f"Updated serde_yaml dep in {p}")
        changed = True

print("No change" if not changed else "Done")
PY
