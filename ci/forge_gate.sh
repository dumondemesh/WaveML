#!/usr/bin/env bash
set -euo pipefail

echo "== Forge Gate =="

BIN="${BIN:-target/debug/wavectl}"
EX_DIR="${EX_DIR:-examples/graph}"

eq_a="${EX_DIR}/forge_eq_A.json"
eq_b="${EX_DIR}/forge_eq_B.json"
eq_syn="${EX_DIR}/forge_eq_synonyms.json"

diff_center_a="${EX_DIR}/forge_diff_center_false.json"
diff_center_b="${EX_DIR}/forge_diff_center_true.json"

diff_pad_a="${EX_DIR}/forge_diff_pad_reflect.json"
diff_pad_b="${EX_DIR}/forge_diff_pad_toeplitz.json"

# ensure binary exists
if [ ! -x "$BIN" ]; then
  echo "Building wavectl..."
  cargo build >/dev/null
fi

id_a=$("$BIN" forge --input "$eq_a" --print-id | tail -n1 | tr -d '\r\n')
id_b=$("$BIN" forge --input "$eq_b" --print-id | tail -n1 | tr -d '\r\n')
id_syn=$("$BIN" forge --input "$eq_syn" --print-id | tail -n1 | tr -d '\r\n')

echo "[forge-gate] NF-ID(A)   = $id_a"
echo "[forge-gate] NF-ID(B)   = $id_b"
echo "[forge-gate] NF-ID(SYN) = $id_syn"

if [[ "$id_a" != "$id_b" || "$id_a" != "$id_syn" ]]; then
  echo "[forge-gate] FAIL: Equivalent graphs must have identical NF-ID"
  exit 1
fi

id_center_false=$("$BIN" forge --input "$diff_center_a" --print-id | tail -n1 | tr -d '\r\n')
id_center_true=$("$BIN" forge --input "$diff_center_b" --print-id | tail -n1 | tr -d '\r\n')

echo "[forge-gate] NF-ID(center=false) = $id_center_false"
echo "[forge-gate] NF-ID(center=true)  = $id_center_true"

if [[ "$id_center_false" == "$id_center_true" ]]; then
  echo "[forge-gate] FAIL: center must affect NF-ID (STRICT-NF key includes center)"
  exit 1
fi

id_pad_reflect=$("$BIN" forge --input "$diff_pad_a" --print-id | tail -n1 | tr -d '\r\n')
id_pad_toeplitz=$("$BIN" forge --input "$diff_pad_b" --print-id | tail -n1 | tr -d '\r\n')

echo "[forge-gate] NF-ID(pad=reflect)  = $id_pad_reflect"
echo "[forge-gate] NF-ID(pad=toeplitz) = $id_pad_toeplitz"

if [[ "$id_pad_reflect" == "$id_pad_toeplitz" ]]; then
  echo "[forge-gate] FAIL: pad_mode must affect NF-ID (STRICT-NF key includes pad_mode)"
  exit 1
fi

echo "== Forge Gate: PASS =="
