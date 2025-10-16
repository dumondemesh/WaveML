# CHANGELOG

## 1.1.0 — WT params + perf, Align
- ABI bump: WMLB 1.1 (см. docs/versions.md)
- Добавлены поля `W.params` в IR и запись в .wfr
- Включён отчёт `perf` (rmse, snr_db, elapsed_ms) для WT-roundtrip
- Реализован Align: xcorr_soft / xcorr_hard с `radius`
- Acceptance обновлён, auto-index в `build/acceptance/index.md`
