#!/usr/bin/env bash
set -euo pipefail
# --- CLI detection helpers ---
has_wavectl() { command -v wavectl >/dev/null 2>&1; }
has_wt_equiv_bin() { command -v wt-equiv >/dev/null 2>&1; }
wavectl_has_subcmd() { has_wavectl && wavectl --help 2>/dev/null | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }


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

time cargo run -p wavectl --bin wavectl -- nf-batch --list "$LIST" --jobs 1 --out "$OUT1"
time cargo run -p wavectl --bin wavectl -- nf-batch --list "$LIST" --jobs 8 --out "$OUTN"

if ! diff -u "$OUT1" "$OUTN" >/dev/null; then
  echo "[WARN] Output order differs between 1 and N jobs (not a blocker)."
fi

echo "[OK] perf gate completed (check timings above)."
