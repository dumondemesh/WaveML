#!/usr/bin/env bash
set -euo pipefail

# Forge / NF-ID determinism and NF-diff distinctions (I1).

WAVECTL="${WAVECTL:-target/debug/wavectl}"
EX_DIR="${EX_DIR:-examples/graph}"
TMP_DIR="${TMP_DIR:-build/forge_gate}"
mkdir -p "$TMP_DIR"

# 1) Deterministic ID on the same file (repeatability)
FILE_A="${EX_DIR}/forge_eq_A.json"
if [[ ! -f "$FILE_A" ]]; then
  echo "[WARN] Missing $FILE_A, skipping repeatability check"
else
  ID1="$("$WAVECTL" forge --input "$FILE_A" --print-id | head -n1)"
  ID2="$("$WAVECTL" forge --input "$FILE_A" --print-id | head -n1)"
  if [[ "$ID1" != "$ID2" ]]; then
    echo "[FAIL] NF-ID not deterministic"; exit 1
  fi
  [[ ${#ID1} -eq 64 ]] || { echo "[FAIL] ID length not 64 hex"; exit 1; }
  echo "[OK] Deterministic NF-ID: $ID1"
fi

# 2) Equivalence A==B (same NF-ID)
FILE_B="${EX_DIR}/forge_eq_B.json"
if [[ -f "$FILE_A" && -f "$FILE_B" ]]; then
  IDA="$("$WAVECTL" forge --input "$FILE_A" --print-id | head -n1)"
  IDB="$("$WAVECTL" forge --input "$FILE_B" --print-id | head -n1)"
  if [[ "$IDA" != "$IDB" ]]; then
    echo "[FAIL] Equivalence broken: $IDA != $IDB"; exit 1
  fi
  echo "[OK] Equivalence A==B: $IDA"
else
  echo "[WARN] Missing A/B files — skip equivalence"
fi

# 3) Meaningful difference (center/pad_mode)
FILE_DIFF1="${EX_DIR}/forge_diff_center.json"
FILE_DIFF2="${EX_DIR}/forge_diff_pad.json"
if [[ -f "$FILE_DIFF1" ]]; then
  if "$WAVECTL" nf-diff --a "$FILE_A" --b "$FILE_DIFF1" --fail-on-diff; then
    echo "[FAIL] center differs but nf-diff returned 0"; exit 1
  else
    echo "[OK] center differs — nf-diff detected"
  fi
fi
if [[ -f "$FILE_DIFF2" ]]; then
  if "$WAVECTL" nf-diff --a "$FILE_A" --b "$FILE_DIFF2" --fail-on-diff; then
    echo "[FAIL] pad_mode differs but nf-diff returned 0"; exit 1
  else
    echo "[OK] pad_mode differs — nf-diff detected"
  fi
fi

echo "== Forge Gate: PASS =="
