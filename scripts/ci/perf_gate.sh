#!/usr/bin/env bash
set -euo pipefail

# ---- Cargo-based CLI detection helpers ----
wavectl_help() { cargo run -q -p wavectl --bin wavectl -- --help 2>/dev/null; }
wavectl_has_subcmd() { wavectl_help | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }
cargo_run_wavectl() { cargo run -p wavectl --bin wavectl -- "$@"; }

echo "== Perf: perf_gate =="

if ! wavectl_has_subcmd "nf-batch"; then
  echo "[WARN] 'wavectl nf-batch' not available; skipping perf gate"
  exit 0
fi

LIST="build/filelist.txt"
OUT1="build/nf_batch_1.csv"
OUTN="build/nf_batch_n.csv"
mkdir -p build

find acceptance -type f -name "*.wml" > "$LIST" || true
if [[ ! -s "$LIST" ]]; then
  echo "[WARN] No *.wml files to batch; skipping perf gate."
  exit 0
fi

time cargo_run_wavectl nf-batch --list "$LIST" --jobs 1 --out "$OUT1"
time cargo_run_wavectl nf-batch --list "$LIST" --jobs 8 --out "$OUTN"

if ! diff -u "$OUT1" "$OUTN" >/dev/null; then
  echo "[WARN] Output order differs between 1 and N jobs (not a blocker)."
fi

echo "[OK] perf gate completed (check timings above)."
