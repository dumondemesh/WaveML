## [1.1] - 2025-10-16
### Added
- `W.params`: `n_fft`, `hop`, `window`, `center`, `pad_mode=reflect` (R7 guard).
- New operator `Align` (`mode: xcorr_soft|xcorr_hard`, `radius`).
- Reports: include `ops[].params` for `W` and perf hint `{fft_backend, frame_count, n_fft}`.

### Changed
- W/T now use rustfft backend (O(N log N)), roundtrip RMSE ≤ 1e-8 @ (1024,256).

### Lint/Acceptance
- Safety forbids `A∘Align` (allows `Align∘A`).
- Added acceptance tests for W params, WT roundtrip, and Align PASS/FAIL.

---
(older entries omitted)
