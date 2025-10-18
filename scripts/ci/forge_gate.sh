#!/usr/bin/env bash
set -euo pipefail
# --- CLI detection helpers ---
has_wavectl() { command -v wavectl >/dev/null 2>&1; }
has_wt_equiv_bin() { command -v wt-equiv >/dev/null 2>&1; }
wavectl_has_subcmd() { has_wavectl && wavectl --help 2>/dev/null | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }


echo "== I1: forge_gate (determinism via nf-batch) =="

if ! has_wavectl; then
  echo "[WARN] wavectl not found; skipping determinism check"
  exit 0
fi

if ! wavectl_has_subcmd "nf-batch"; then
  echo "[WARN] 'wavectl nf-batch' not available; skipping"
  exit 0
fi

SAMPLE="${SAMPLE:-}"
if [[ -z "${SAMPLE}" ]]; then
  SAMPLE="$(find acceptance -type f -name "*.wml" | head -n 1 || true)"
fi
if [[ -z "${SAMPLE}" || ! -f "$SAMPLE" ]]; then
  echo "[WARN] No SAMPLE *.wml found; skipping"
  exit 0
fi

mkdir -p build
LIST="build/_nf_sample.txt"
echo "$SAMPLE" > "$LIST"

OUT1="build/_nf1.csv"
OUT2="build/_nf2.csv"

cargo run -q -p wavectl --bin wavectl -- nf-batch --list "$LIST" --jobs 1 --out "$OUT1"
cargo run -q -p wavectl --bin wavectl -- nf-batch --list "$LIST" --jobs 1 --out "$OUT2"

if ! diff -u "$OUT1" "$OUT2" >/dev/null; then
  echo "[FAIL] Determinism failed: nf-batch outputs differ"
  exit 1
fi

echo "[OK] Determinism: nf-batch stable"
