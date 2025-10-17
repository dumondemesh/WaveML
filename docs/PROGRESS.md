# WaveML / SMOT — PROGRESS (Phase 1) — 2025-10-17

> Коротко: этот файл служит «одним экраном» статуса. P0 — срочно, P1 — ближайшее, P2 — среднесрочно.

## ✅ Сделано
- Workspace крейтов: wmlb, waveform, wavereport, waveeval, waveforge, linters, wavectl, waverunner (скелет).
- WaveForge → STRICT-NF + детерминированный NF-ID; I1 (tests_forge.yaml) — PASS.
- Линтеры R7/R8/R9 (Sta/Toeplitz; запрет zero-pad; запрет A∘Align) — PASS.
- COLA: waveeval + wavectl cola/report-cola; окна Hann/Rect — PASS.
- RustFFT интегрирован в рантайм (STFT/iSTFT), COLA-нормировка.
- simulate-swaps: моделирование допустимых свопов — I2/I3 acceptance — PASS.
- WFR v1.x: новый формат используется частично (`cert, mdl, phase, swap, w_params, w_perf, metrics`).

## ⏭️ В работе / TODO
### P0
- WFR **унификация**: везде писать **единый** v1.x (см. spec/WFR-1.0.0.schema.json) и обновить валидатор.
- Clippy **strict** по workspace (`-D warnings`): убрать `unwrap`, `div_ceil`, лишние `&`, добавить `anyhow::Context`.
- README↔️CLI: синхронизировать Quick Start с текущими командами (или вернуть `compile/run`).

### P1
- STRICT-NF канон: включить `center` и `pad_mode` в ключ сортировки NF-ID.
- WFR `w_perf/metrics`: писать `{backend, rustfft_ver, wall_ms, frames, n_fft, hop, threads?}` и `{mse, rel_mse, snr_db, cola_max_dev}`; пороги — в acceptance.
- Waveform v0.3: довести `INDEX/ALIGN`, паспорт/подпись, zero-copy для WaveBit.

### P2
- WaveRunner: Phase-Lock/Φ-свопы — логирование в `.wfr`.
- WaveTrain: MDL-контур (L = L_struct + L_params + L_fit + λ·L_coh); реальные I4/I5.
- Acceptance one-pager: автоген MD/HTML из результатов.

## 📌 Риски
- Два формата WFR одновременно → расхождения пайплайнов.
- NF-ID без (`center`, `pad_mode`) → риск ложной эквивалентности графов.
- Нет Phase-Lock логирования → I2/I3/I4/I5 неполные.

## 🔧 Быстрые команды
```bash
# Линтеры
./target/debug/wavectl lint --input examples/graph/demo_w.json

# COLA и отчёт
./target/debug/wavectl cola --n-fft 1024 --hop 512 --window Hann --mode amp --tol 1e-12
./target/debug/wavectl report-from-graph --input examples/graph/demo_w.json \
  --out build/reports/auto_amp.wfr.json --mode amp --tol 1e-12
./target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass

# STRICT-NF ID
./target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-id

# I2/I3 (свопы)
./target/debug/wavectl simulate-swaps --input examples/graph/i23_pass.json --out build/i23_pass.wfr.json

# Приёмка целиком
make all
```
