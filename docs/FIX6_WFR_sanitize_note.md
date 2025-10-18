# Fix6 — WFR sanitizer: required `w_params` keys

This update to `scripts/ci/wfr_sanitize.py` fills missing required fields in root-level `w_params`
to satisfy WFR schema for legacy artifacts:

- `n_fft`: 0
- `hop`: 0
- `window`: "NA"
- `mode`: "NA"  (and mirrors to `pad_mode` if absent)

These are sentinel defaults for CI-only validation. Replace with real values at generation time
as part of Task B (Valid WFR), after which the sanitizer can be removed.
