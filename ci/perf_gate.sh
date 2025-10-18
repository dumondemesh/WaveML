#!/usr/bin/env bash
set -euo pipefail

LIST="examples/batch/list.txt"
OUT1="build/perf/nf_serial.json"
OUTP="build/perf/nf_parallel.json"
mkdir -p build/perf examples/batch

# Bootstrap list if missing
if [[ ! -f "$LIST" ]]; then
  printf "examples/graph/forge_eq_A.json
examples/graph/forge_eq_B.json
" > "$LIST"
fi

measure() {
python3 - "$@" <<'PY'
import json, subprocess, sys, time
mode, list_path, out_path, jobs = sys.argv[1:5]
cmd = ["target/debug/wavectl", "nf-batch", "--list", list_path, "--format", "json", "--out", out_path, "--jobs", jobs]
t0 = time.time()
subprocess.run(cmd, check=True)
dt = time.time() - t0
print(f"{mode}:{dt:.6f}")
PY
}

echo "[perf] serial run (--jobs 1)"
TS=$(measure "serial" "$LIST" "$OUT1" "1")
echo "[perf] $TS"

echo "[perf] parallel run (--jobs 8)"
TP=$(measure "parallel" "$LIST" "$OUTP" "8")
echo "[perf] $TP"

echo "[perf] verify deterministic output ordering"
python3 - <<'PY'
import json, sys
with open("build/perf/nf_serial.json") as f: a=json.load(f)
with open("build/perf/nf_parallel.json") as f: b=json.load(f)
ain=[r["input"] for r in a]; bin=[r["input"] for r in b]
assert ain==bin, "order differs"
print("[perf] order OK")
PY

echo "[perf] gate: PASS (advisory)"
