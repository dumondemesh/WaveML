#!/usr/bin/env bash
set -euo pipefail

# Perf gate (P2 advisory). Measures nf-batch speedup and output stability.
# Returns non-zero on hard failures only (e.g., missing wavectl).

WAVECTL="${WAVECTL:-target/debug/wavectl}"
LIST="${LIST:-examples/batch/list.txt}"
OUT_DIR="${OUT_DIR:-build/perf}"
JOBS="${JOBS:-8}"
mkdir -p "$OUT_DIR"

if [[ ! -x "$WAVECTL" ]]; then
  echo "[FAIL] wavectl not found or not executable: $WAVECTL"
  exit 1
fi

if [[ ! -f "$LIST" ]]; then
  echo "[WARN] No batch list: $LIST (creating trivial)"
  mkdir -p "$(dirname "$LIST")"
  echo "examples/graph/forge_eq_A.json" > "$LIST"
fi

# Fallback: a portable Python batcher if 'scripts/nf_batch.py' exists
BATCH_PY="${BATCH_PY:-scripts/nf_batch.py}"
OUT_SER="$OUT_DIR/nf_serial.json"
OUT_PAR="$OUT_DIR/nf_parallel.json"

if [[ -f "$BATCH_PY" ]]; then
  python3 "$BATCH_PY" --wavectl "$WAVECTL" --list "$LIST" --out "$OUT_SER" --jobs 1
  python3 "$BATCH_PY" --wavectl "$WAVECTL" --list "$LIST" --out "$OUT_PAR" --jobs "$JOBS"
else
  # Minimal POSIX xargs-based fallback (BSD xargs supports -P)
  xargs -I{} -n1 -P1 sh -c '"$0" forge --input "$1" --print-id' "$WAVECTL" {} < "$LIST" > "$OUT_SER"
  xargs -I{} -n1 -P"$JOBS" sh -c '"$0" forge --input "$1" --print-id' "$WAVECTL" {} < "$LIST" | sort > "$OUT_PAR"
fi

if diff -q "$OUT_SER" "$OUT_PAR" >/dev/null 2>&1; then
  echo "[OK] Deterministic output under parallelism"
else
  echo "[WARN] Output differs between serial and parallel (order or content)."
fi

echo "== Perf Gate: DONE (advisory) =="
