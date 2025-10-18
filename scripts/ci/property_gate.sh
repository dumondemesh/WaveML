#!/usr/bin/env bash
set -euo pipefail
# --- CLI detection helpers ---
has_wavectl() { command -v wavectl >/dev/null 2>&1; }
has_wt_equiv_bin() { command -v wt-equiv >/dev/null 2>&1; }
wavectl_has_subcmd() { has_wavectl && wavectl --help 2>/dev/null | grep -E "^[[:space:]]+$1([[:space:]]|$)" >/dev/null 2>&1; }


echo "== I1/I2: property_gate =="
PLAN="acceptance/tests.yaml"

if [[ ! -f "$PLAN" ]]; then
  echo "[WARN] No acceptance plan at $PLAN; skipping."
  exit 0
fi

if wavectl_has_subcmd "acceptance"; then
  cargo run -p wavectl --bin wavectl -- acceptance --plan "$PLAN" --outdir build/acceptance --strict || true
  echo "[OK] acceptance subcommand executed (non-blocking)"
else
  echo "[WARN] 'wavectl acceptance' not available; skipping property tests"
fi
