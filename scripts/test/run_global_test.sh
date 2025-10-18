#!/usr/bin/env bash
set -euo pipefail

echo "== WaveML Global Test (Phase 4 Freeze) — v3 =="
mkdir -p build/reports

echo "-- Build & Clippy"
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings

echo "-- Acceptance (non-blocking, if available)"
if wavectl --help 2>/dev/null | grep -E "^[[:space:]]+acceptance([[:space:]]|$)" >/dev/null 2>&1; then
  cargo run -p wavectl --bin wavectl -- acceptance --plan acceptance/tests.yaml --outdir build/acceptance --strict || true
else
  echo "[WARN] wavectl acceptance not available; skipping"
fi

echo "-- Gates"
bash scripts/ci/run_all_gates.sh

SUM="build/reports/global_test_summary.md"
{
  echo "# Global Test Summary"
  echo
  echo "- Date: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
  echo "- Build: OK"
  echo "- Clippy: OK"
  echo "- Acceptance: see logs (may be skipped)"
  echo "- Gates: see console logs and build/acceptance/*.wfr.json"
} > "$SUM"

echo "== DONE. Summary: $SUM =="
