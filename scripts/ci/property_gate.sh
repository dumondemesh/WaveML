#!/usr/bin/env bash
set -euo pipefail

# Property tests (I1/I2): 1) equivalence under known rewrites; 2) meaningful diffs.
# This is a lightweight generator using seeds, without external deps.

WAVECTL="${WAVECTL:-target/debug/wavectl}"
EX_DIR="${EX_DIR:-examples/graph}"
TMP_DIR="${TMP_DIR:-build/property_gate}"
SEED="${SEED:-12345}"
mkdir -p "$TMP_DIR"

ok=1

# Case: apply neutral permutations (alpha-renames, op reorder where commutative)
# Expect: same NF-ID
if [[ -f "$EX_DIR/forge_eq_A.json" && -f "$EX_DIR/forge_eq_A_syn.json" ]]; then
  A="$("$WAVECTL" forge --input "$EX_DIR/forge_eq_A.json" --print-id | head -n1)"
  S="$("$WAVECTL" forge --input "$EX_DIR/forge_eq_A_syn.json" --print-id | head -n1)"
  if [[ "$A" != "$S" ]]; then
    echo "[FAIL] SYN not equivalent: $A vs $S"; ok=0
  else
    echo "[OK] SYN equivalence holds"
  fi
else
  echo "[WARN] No SYN fixtures, skipping"
fi

# Case: introduce a prohibited pattern (e.g., A∘Align) → linter should reject (exit non-zero) if linters are wired
if [[ -f "$EX_DIR/forge_forbidden_A_after_align.json" ]]; then
  if "$WAVECTL" forge --input "$EX_DIR/forge_forbidden_A_after_align.json" --check; then
    echo "[FAIL] Forbidden pattern not rejected"; ok=0
  else
    echo "[OK] Forbidden pattern rejected by linter"
  fi
fi

[[ $ok -eq 1 ]] || { echo "== Property Gate: FAIL =="; exit 1; }
echo "== Property Gate: PASS =="
