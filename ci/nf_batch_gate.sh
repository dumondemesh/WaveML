#!/usr/bin/env bash
set -euo pipefail

echo "== NF-BATCH Gate =="
BIN="${WAVECTL_BIN:-target/debug/wavectl}"

# 1) JSON: должны быть одинаковые NF-ID
JSON="$("$BIN" nf-batch \
  --input examples/graph/forge_eq_A.json \
  --input examples/graph/forge_eq_B.json \
  --input examples/graph/forge_eq_synonyms.json \
  --json)"

uniq_cnt="$(printf "%s" "$JSON" | jq -r '.items[].nf_id' | sort -u | wc -l | tr -d ' ')"
if [[ "$uniq_cnt" != "1" ]]; then
  echo "[nf-batch-gate] FAIL: NF-ID in JSON differ"
  printf "%s\n" "$JSON" | jq .
  exit 1
fi
echo "[nf-batch-gate] JSON OK"

# 2) CSV: проверяем хедер и количество непустых строк данных
CSV="$("$BIN" nf-batch \
  --input examples/graph/forge_eq_A.json \
  --input examples/graph/forge_eq_B.json \
  --csv)"

header="$(printf "%s" "$CSV" | head -n1 | tr -d '\r')"
rows="$(printf "%s" "$CSV" | tail -n +2 | awk 'NF>0{c++} END{print c+0}')"

if [[ "$header" != "input,nf_id" || "$rows" -lt 2 ]]; then
  echo "[nf-batch-gate] FAIL: CSV invalid"
  echo "[debug] header=<$header>"
  echo "[debug] rows=$rows"
  printf "%s\n" "$CSV"
  exit 1
fi
echo "[nf-batch-gate] CSV OK"

echo "== NF-BATCH Gate: PASS =="
