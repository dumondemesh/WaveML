#!/usr/bin/env bash
set -euo pipefail

echo "== NF-DIFF Gate =="

W=target/debug/wavectl

A=examples/graph/forge_eq_A.json
B=examples/graph/forge_eq_B.json
S=examples/graph/forge_eq_synonyms.json
CF=examples/graph/forge_diff_center_false.json
CT=examples/graph/forge_diff_center_true.json
PR=examples/graph/forge_diff_pad_reflect.json
PT=examples/graph/forge_diff_pad_toeplitz.json

fail() { echo "$@" 1>&2; exit 1; }

# Эквивалентные пары должны быть идентичны (exit=0)
"$W" nf-diff --left "$A" --right "$B"  >/dev/null || fail "[nf-diff-gate] A ~ B (должны быть эквивалентны) — FAIL"
echo "[nf-diff-gate] A ~ B (должны быть эквивалентны)"

"$W" nf-diff --left "$A" --right "$S"  >/dev/null || fail "[nf-diff-gate] A ~ SYN (синонимы) — FAIL"
echo "[nf-diff-gate] A ~ SYN (синонимы, тоже эквивалентны)"

# Различающиеся пары должны давать ошибку (exit≠0) при --fail-on-diff
set +e
"$W" nf-diff --left "$CF" --right "$CT" --fail-on-diff >/dev/null 2>&1
ec1=$?
"$W" nf-diff --left "$PR" --right "$PT" --fail-on-diff >/dev/null 2>&1
ec2=$?
set -e

if [[ $ec1 -ne 0 ]]; then
  echo "[OK] center различаются — nf-diff вернул ошибку как и ожидалось"
else
  fail "[FAIL] center различаются — но nf-diff вернул 0"
fi

if [[ $ec2 -ne 0 ]]; then
  echo "[OK] pad_mode различаются — nf-diff вернул ошибку как и ожидалось"
else
  fail "[FAIL] pad_mode различаются — но nf-diff вернул 0"
fi

echo "== NF-DIFF Gate: PASS =="
