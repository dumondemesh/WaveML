#!/usr/bin/env bash
set -euo pipefail

echo "== WaveML CI gates =="

step() { echo "[$1] $2"; }

# [1/6] build
step "1/6" "build"
cargo build >/dev/null

# [2/6] clippy strict
step "2/6" "clippy strict"
echo "==> Strict clippy gate"
cargo clippy --workspace --all-targets -- -D warnings >/dev/null && echo "CLIPPY-GATE: OK"

# [3/6] forge canonicalization
step "3/6" "forge canonicalization"
echo "== Forge Gate =="

W=target/debug/wavectl

A=examples/graph/forge_eq_A.json
B=examples/graph/forge_eq_B.json
S=examples/graph/forge_eq_synonyms.json
CF=examples/graph/forge_diff_center_false.json
CT=examples/graph/forge_diff_center_true.json
PR=examples/graph/forge_diff_pad_reflect.json
PT=examples/graph/forge_diff_pad_toeplitz.json

id() { "$W" forge --input "$1" --print-id | tail -n1 | awk '{print $NF}'; }

IDA=$(id "$A");  echo "[forge-gate] NF-ID(A)   = $IDA"
IDB=$(id "$B");  echo "[forge-gate] NF-ID(B)   = $IDB"
IDS=$(id "$S");  echo "[forge-gate] NF-ID(SYN) = $IDS"
test "$IDA" = "$IDB" && test "$IDA" = "$IDS" || { echo "[forge-gate] FAIL: Equivalent graphs must have identical NF-ID"; exit 1; }

IDC=$(id "$CF"); echo "[forge-gate] NF-ID(center=false) = $IDC"
IDT=$(id "$CT"); echo "[forge-gate] NF-ID(center=true)  = $IDT"
test "$IDC" != "$IDT" || { echo "[forge-gate] FAIL: center must affect NF-ID"; exit 1; }

IDR=$(id "$PR");  echo "[forge-gate] NF-ID(pad=reflect)  = $IDR"
IDT2=$(id "$PT"); echo "[forge-gate] NF-ID(pad=toeplitz) = $IDT2"
test "$IDR" != "$IDT2" || { echo "[forge-gate] FAIL: pad_mode must affect NF-ID"; exit 1; }

echo "== Forge Gate: PASS =="

# [4/6] NF-DIFF gate (если есть отдельный скрипт — используем его)
if [[ -x scripts/ci/nf_diff_gate.sh ]]; then
  bash scripts/ci/nf_diff_gate.sh
else
  step "4/6" "nf-diff gate"
  echo "== NF-DIFF Gate =="
  set +e
  "$W" nf-diff --left "$A"  --right "$B"  >/dev/null; ok1=$?
  "$W" nf-diff --left "$A"  --right "$S"  >/dev/null; ok2=$?
  "$W" nf-diff --left "$CF" --right "$CT" --fail-on-diff >/dev/null 2>&1; diff1=$?
  "$W" nf-diff --left "$PR" --right "$PT" --fail-on-diff >/dev/null 2>&1; diff2=$?
  set -e
  [[ $ok1 -eq 0 && $ok2 -eq 0 && $diff1 -ne 0 && $diff2 -ne 0 ]] || { echo "NF-DIFF Gate failed"; exit 1; }
  echo "== NF-DIFF Gate: PASS =="
fi

# [5/6] COLA -> WFR
step "5/6" "COLA -> WFR"
mkdir -p build/reports
"$W" cola --n-fft 1024 --hop 512 --window Hann --mode amp --out build/reports/auto_amp.wfr.json >/dev/null
echo "[cola] Wrote \"build/reports/auto_amp.wfr.json\""

# [6/6] validate WFR
step "6/6" "validate WFR (require-pass)"
"$W" validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass >/dev/null
echo "[validate-wfr] OK: \"build/reports/auto_amp.wfr.json\""

# Дополнительно: NF-BATCH gate, если присутствует
if [[ -x scripts/ci/nf_batch_gate.sh ]]; then
  bash scripts/ci/nf_batch_gate.sh
fi
