#!/usr/bin/env bash
set -euo pipefail
# Normalize WFR JSON in-place (or to stdout) to guarantee presence & types of keys:
# - mdl: object
# - w_params: object with keys: n_fft, hop, window, mode|pad_mode
# - w_perf: object with numeric fields (threads, frames, wall_ms)
# - metrics: keep as-is; ensure mse present if exists
# Usage:
#   tools/wfr-normalize.sh in.json > out.json
# or in-place:
#   tools/wfr-normalize.sh -i in.json

INPLACE=0
if [[ "${1:-}" == "-i" ]]; then
  INPLACE=1
  shift
fi
f="${1:-}"
if [[ -z "$f" || ! -f "$f" ]]; then
  echo "Usage: $0 [-i] file.wfr.json" >&2
  exit 2
fi

tmp="$(mktemp)"
jq '
  def ensure_obj: if type=="object" then . else {} end;
  def num(x): if (x|type)=="number" then x else (try (x|tonumber) catch 0) end;
  .mdl = (.mdl | ensure_obj) |
  .w_params = ( .w_params | ensure_obj
    | (if has("n_fft") then . else . + {"n_fft": (..|.n_fft? // 0)} end)
    | (if has("hop") then . else . + {"hop": (..|.hop? // 0)} end)
    | (if has("window") then . else . + {"window": (..|.window? // "Hann")} end)
    | (if has("mode") or has("pad_mode") then . else . + {"mode":"amp"} end)
  ) |
  .w_perf = ( .w_perf | ensure_obj
    | .threads = num(.threads//1)
    | .frames  = num(.frames//0)
    | .wall_ms = (.wall_ms | if type=="number" then . else (try (.|tonumber) catch 0.0) end)
  ) |
  .metrics = (.metrics // {})' "$f" > "$tmp"

if [[ $INPLACE -eq 1 ]]; then
  mv "$tmp" "$f"
  echo "[OK] normalized $f"
else
  cat "$tmp"
  rm -f "$tmp"
fi
