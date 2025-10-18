#!/usr/bin/env bash
set -euo pipefail

OUTDIR="${1:-build_i23}"
mkdir -p "${OUTDIR}"

echo "==> I2/I3 scaffold acceptance"
python3 tools/simulate_swaps.py --input examples/graph/i23_pass.json     --out "${OUTDIR}/i23_pass.wfr.json"     --force pass
python3 tools/simulate_swaps.py --input examples/graph/i23_fail_i2.json  --out "${OUTDIR}/i23_fail_i2.wfr.json"  --force fail-i2
python3 tools/simulate_swaps.py --input examples/graph/i23_fail_i3.json  --out "${OUTDIR}/i23_fail_i3.wfr.json"  --force fail-i3

echo "==> Quick checks"
python3 - "${OUTDIR}" <<'PY'
import json, sys, pathlib
base = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "build_i23")

def load(rel):
    p = base / rel
    with p.open("r", encoding="utf-8") as f:
        return json.load(f)

ok = True
ok &= load("i23_pass.wfr.json")["cert"].get("i2") is True
ok &= load("i23_pass.wfr.json")["cert"].get("i3") is True
ok &= load("i23_fail_i2.wfr.json")["cert"].get("i2") is False
ok &= load("i23_fail_i2.wfr.json")["cert"].get("i3") is True
ok &= load("i23_fail_i3.wfr.json")["cert"].get("i2") is True
ok &= load("i23_fail_i3.wfr.json")["cert"].get("i3") is False

print("ACCEPTANCE:", "PASS" if ok else "FAIL")
raise SystemExit(0 if ok else 1)
PY

echo "OK"
