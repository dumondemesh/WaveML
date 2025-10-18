# WaveML — CHECKLIST (2025-10-17)

## Done
- simulate-swaps реализован, Acceptance I2/I3 — PASS.
- RustFFT интеграция + COLA-нормировка (Hann/Hamming/Blackman; `center`, `pad_mode=reflect`) — в рантайме.
- Линтеры R7/R8/R9 — PASS, базовые acceptance наборы — PASS.

## P0 — Сейчас
- [WFR] Полная унификация формата: v1.x (cert, mdl, phase, swap, w_params, w_perf, metrics, log[*]).
- [Logging] Единая подсистема логирования (`tracing`) со слоями `error|warn|info|debug|trace`,
  флаги CLI `--log-level`, `--log-format {compact|full|json}`; ключевые события и тайминги дублируем в `.wfr.log[*]`.
- [Clippy] `-D warnings` по workspace, `anyhow::Context` вместо `unwrap`.
- [Docs] Обновить `docs/STRICT-NF.md` (ключ сортировки: `(op, n_fft, hop, window, center, pad_mode)`),
  `docs/versions.md` (таблицы W.params / W.perf / metrics).

## P1 — Следом
- [NF] Перегенерировать примеры NF-ID; обновить Acceptance.
- [Perf/Metrics] Замер `wall_ms`, `backend`, `version`, `frames`, `threads?`; метрики `mse, rel_mse, snr_db, cola_max_dev`;
  пороги — в `acceptance/thresholds.yaml`.
- [Waveform v0.3] INDEX/ALIGN/подпись, уточнить schema.
- [Acceptance Gate] One-pager (MD→HTML) + CI-gate (clippy + acceptance + wfr-validate).

## P2
- [Runner] Phase-Lock / Φ-свопы (каркас+логирование в .wfr).
- [Train] Toy-десцент (I4/I5).
- [Dialects] .Audio/.Text/.GSP пресеты окон/нормировок.

## Решения по размещению
- `CHECKLIST.md` — в **корне** (видимость для всех). Ссылки на детали — в `docs/`.
