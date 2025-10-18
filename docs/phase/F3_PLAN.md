# Фаза F3 (P2): DSP-эквивалентность (WT Equivalence)

## Цели
- Проверяем числовую корректность дискретного преобразования (STFT) и обратного преобразования (ISTFT) на синтетике.
- Вводим гейт `wt_equiv_gate.sh` с метриками `MSE` и `SDR`.
- Пороговые условия (по умолчанию):
  - `MSE ≤ 1e-9` на коротких синтетических сигналах,
  - `SDR ≥ 60 dB` (Signal-to-Distortion Ratio).

## Стратегия
- Референс — точный DFT/IDFT (O(N^2)) на малых окнах, что исключает погрешности реализации FFT.
- Проверяем консистентность окна Hann, hop, overlap-add.
- Профили: `center=false`, `pad=reflect` (как в R7).

## Deliverables (DoD)
- Отдельный бинарник `wt-equiv` (в составе `wavectl` как вторичный bin: `src/bin/wt-equiv.rs`).
- CI-скрипт `scripts/ci/wt_equiv_gate.sh`.
- Примеры сценариев `acceptance/wt/tests_i3.yaml`.
- `.wfr` отчёты с `metrics.mse`, `metrics.snr_db`, `cert.i3_conservative_functors`.
- PASS на синтетике (sine, sweep).
