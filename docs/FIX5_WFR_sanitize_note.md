# Fix5 — WFR sanitizer: w_params/w_perf null -> {}

This patch extends `scripts/ci/wfr_sanitize.py` to coerce:
- root-level `w_params: null` -> `{}`
- root-level `w_perf: null` -> `{}`
- nested `W.params: null` -> `{}`
- nested `W.perf: null` -> `{}`
(in addition to the existing `mdl: null` -> `{}`)

Use:
    bash scripts/ci/schema_gate.sh
    bash scripts/ci/run_all_gates.sh
