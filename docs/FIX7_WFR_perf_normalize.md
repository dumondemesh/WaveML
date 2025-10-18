# Fix7 — WFR sanitizer: normalize w_perf

This update ensures `w_perf` fields always satisfy the schema:
- `threads`: coerced to integer (default 0)
- `frames`: coerced to integer (default 0)
- `wall_ms`: coerced to float (default 0.0)

Also applied to nested `W.perf`. Combine with previous fixes for `w_params` and `mdl`.
