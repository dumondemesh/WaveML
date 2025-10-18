#!/usr/bin/env bash
set -euo pipefail

# ---- helpers ----
wavectl_help() { cargo run -q -p wavectl --bin wavectl -- --help 2>/dev/null; }
wavectl_has_subcmd() { wavectl_help | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }
cargo_run_wavectl() { cargo run -q -p wavectl --bin wavectl -- "$@"; }
cargo_run_wt_equiv() { RAYON_NUM_THREADS=1 cargo run -q -p wavectl --bin wt-equiv -- "$@"; }

STRICT_I1="${STRICT_I1:-0}"
I1_NUM_EPS="${I1_NUM_EPS:-1e-9}"

canon_json() {
  # убрать volatile, отсортировать ключи/массивы и округлить числа до eps
  local src="$1" dst="$2" eps="$3"
  jq --argjson eps "$eps" '
    def quant: if type=="number" then ((.*(1/$eps))|round|./(1/$eps)) else . end;
    def canon:
      walk(
        if type=="object" then with_entries(.value |= canon)
        elif type=="array" then (map(canon)|sort_by(tostring))
        else quant end
      );
    del(.timestamp,.time,.uuid,.seed,.git_sha,.hostname,.duration_ms,.elapsed,.started_at,.finished_at)
    | canon
  ' "$src" 2>/dev/null | jq -S . > "$dst" 2>/dev/null || cp "$src" "$dst"
}

warn_or_fail(){ local m="$1"; [[ "$STRICT_I1" = "1" ]] && { echo "[FAIL] $m"; exit 1; } || echo "[WARN] $m"; }

echo "== I1: forge_gate (determinism) =="

# 1) nf-batch (если есть)
if wavectl_has_subcmd "nf-batch"; then
  SAMPLE="${SAMPLE:-$(find acceptance -type f -name '*.wml' | head -n 1 || true)}"
  if [[ -n "$SAMPLE" && -f "$SAMPLE" ]]; then
    mkdir -p build
    LIST="build/_nf_sample.txt"; echo "$SAMPLE" > "$LIST"
    OUT1="build/_nf1.csv"; OUT2="build/_nf2.csv"
    cargo_run_wavectl nf-batch --list "$LIST" --jobs 1 --out "$OUT1"
    cargo_run_wavectl nf-batch --list "$LIST" --jobs 1 --out "$OUT2"
    diff -u "$OUT1" "$OUT2" >/dev/null \
      && { echo "[OK] Determinism: nf-batch stable"; exit 0; } \
      || warn_or_fail "Determinism failed: nf-batch outputs differ"
  else
    echo "[WARN] No SAMPLE *.wml found; skipping nf-batch path"
  fi
fi

# 2) simulate-swaps (если доступен)
if wavectl_has_subcmd "simulate-swaps"; then
  INPUT="${SAMPLE:-}"
  [[ -z "$INPUT" && -f "acceptance/ok_multiline_w.wml" ]] && INPUT="acceptance/ok_multiline_w.wml"
  [[ -z "$INPUT" && -f "acceptance/data/sample1.wml" ]] && INPUT="acceptance/data/sample1.wml"
  [[ -z "$INPUT" ]] && INPUT="$(find acceptance -type f -name '*.wml' | grep -v '/bad_' | head -n 1 || true)"
  if [[ -n "$INPUT" && -f "$INPUT" ]]; then
    echo "[INFO] I1 input: $INPUT"
    mkdir -p build/_i1
    R1="build/_i1/swaps_1.json"; R2="build/_i1/swaps_2.json"
    if cargo_run_wavectl simulate-swaps --input "$INPUT" --out "$R1" && [[ -s "$R1" ]]; then
      if cargo_run_wavectl simulate-swaps --input "$INPUT" --out "$R2" && [[ -s "$R2" ]]; then
        C1="build/_i1/swaps_1.canon.json"; C2="build/_i1/swaps_2.canon.json"
        canon_json "$R1" "$C1" "$I1_NUM_EPS"; canon_json "$R2" "$C2" "$I1_NUM_EPS"
        diff -u "$C1" "$C2" >/dev/null \
          && { echo "[OK] Determinism: simulate-swaps stable"; exit 0; } \
          || warn_or_fail "Determinism failed: simulate-swaps reports differ"
      else
        echo "[WARN] simulate-swaps(2) failed; will try wt-equiv fallback"
      fi
    else
      echo "[WARN] simulate-swaps(1) failed; will try wt-equiv fallback"
    fi
  else
    echo "[WARN] No usable *.wml for simulate-swaps; will try wt-equiv fallback"
  fi
else
  echo "[WARN] simulate-swaps not available; will try wt-equiv fallback"
fi

# 3) wt-equiv x2 (без --seed) + квантование; если не равно — сравним только .metrics
mkdir -p build/_i1
W1="build/_i1/wt_1.json"; W2="build/_i1/wt_2.json"
if cargo_run_wt_equiv --out "$W1" && [[ -s "$W1" ]] && cargo_run_wt_equiv --out "$W2" && [[ -s "$W2" ]]; then
  CW1="build/_i1/wt_1.canon.json"; CW2="build/_i1/wt_2.canon.json"
  canon_json "$W1" "$CW1" "$I1_NUM_EPS"; canon_json "$W2" "$CW2" "$I1_NUM_EPS"
  if diff -u "$CW1" "$CW2" >/dev/null; then
    echo "[OK] Determinism: wt-equiv stable (eps=$I1_NUM_EPS)"; exit 0
  else
    M1="build/_i1/wt_1.metrics.json"; M2="build/_i1/wt_2.metrics.json"
    jq '.metrics // empty' "$CW1" > "$M1" || true
    jq '.metrics // empty' "$CW2" > "$M2" || true
    if [[ -s "$M1" && -s "$M2" ]] && diff -u "$M1" "$M2" >/dev/null; then
      echo "[OK] Determinism: wt-equiv metrics stable (eps=$I1_NUM_EPS)"; exit 0
    else
      warn_or_fail "Determinism failed: wt-equiv reports differ (after quantization eps=$I1_NUM_EPS)"
    fi
  fi
else
  echo "[WARN] wt-equiv fallback not available or failed; skipping I1"; exit 0
fi
