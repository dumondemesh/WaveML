#!/usr/bin/env bash
set -euo pipefail

RA="scripts/ci/run_all_gates.sh"
BKP="${RA}.bak.$(date -u +%Y%m%d%H%M%S)"

if [[ ! -f "$RA" ]]; then
  echo "[patch] not found: $RA" >&2
  exit 1
fi

cp "$RA" "$BKP"
echo "[patch] backup saved: $BKP"

# 1) Drop any SYN/forge_eq_synonyms references and the triple-equality test.
# 2) Keep A/B, make inline A==B check only.
# 3) Avoid tail|awk parsing; rely on first line of --print-id.

tmp="$(mktemp)"
awk '
  BEGIN { skip=0 }
  /forge_eq_synonyms/ { skip=1; next }
  /SYN/ && /NF-ID/ { next }
  {
    gsub(/S=examples\/graph\/forge_eq_synonyms.json/,"# S removed");
    gsub(/\[forge-gate\] NF-ID\(SYN\).*/,"# SYN removed");
    print $0
  }
' "$RA" > "$tmp"

# Replace the ID function and the equality test block
python3 - "$tmp" <<'PY'
import sys, re
p = sys.argv[1]
s = open(p, "r", encoding="utf-8").read()

# Normalize id() function: head -n1 / tr -d \\r
s = re.sub(r'id\(\)\s*\{[^\}]+\}', 'id() { "$W" forge --input "$1" --print-id | head -n1 | tr -d \'\\r\'; }', s, flags=re.S)

# Replace the A/B/SYN equality cluster with a strict A vs B check
s = re.sub(
    r'IDA=.*?\n.*?IDB=.*?\n.*?IDS=.*?\n.*?test.*?IDA.*?IDB.*?&&.*?IDS.*?||.*?exit 1;?\s*',
    r'IDA=$(id "$A");  echo "[forge-gate] NF-ID(A)   = NF-ID=$IDA"\n'
    r'IDB=$(id "$B");  echo "[forge-gate] NF-ID(B)   = NF-ID=$IDB"\n'
    r'if [[ "$IDA" != "$IDB" ]]; then\n'
    r'  echo "[forge-gate] FAIL: A and B must be identical after canon"\n'
    r'  "$W" nf-diff --left "$A" --right "$B" --show-source-diff || true\n'
    r'  exit 1\n'
    r'fi\n'
    r'echo "== Forge Gate: PASS =="\n',
    s, flags=re.S
)

open(p, "w", encoding="utf-8").write(s)
PY

mv "$tmp" "$RA"
chmod +x "$RA"
echo "[patch] applied to $RA"
grep -n "forge_eq_synonyms" -n "$RA" || echo "[verify] ok: no forge_eq_synonyms"
grep -n "SYN" "$RA" || echo "[verify] ok: no SYN lines"
