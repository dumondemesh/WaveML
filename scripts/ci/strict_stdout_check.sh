#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:-examples/graph/forge_eq_A.json}"

if ! command -v hexdump >/dev/null 2>&1; then
  echo "[strict_stdout_check] hexdump not found; skipping."
  exit 0
fi

OUT="$(target/debug/wavectl forge --input "$INPUT" --print-id | head -n1)"
LEN="${#OUT}"

if [[ "$LEN" -ne 64 ]]; then
  echo "[strict_stdout_check] FAIL: first line length=$LEN, expected 64"
  exit 1
fi

if ! [[ "$OUT" =~ ^[0-9a-f]{64}$ ]]; then
  echo "[strict_stdout_check] FAIL: first line is not 64 hex chars"
  echo "LINE=[$OUT]"
  exit 1
fi

echo "[strict_stdout_check] PASS: strict 64-hex on first line"
