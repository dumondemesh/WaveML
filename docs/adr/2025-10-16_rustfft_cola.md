# ADR: RustFFT как основной FFT-бэкенд + COLA-нормировка в ISTFT

## Контекст
Требуется заменить O(N²) DFT на O(N log N) и зафиксировать инвариантную реконструкцию для пары W/T_inv с сохранением правил R7/R8 и инвариантов I1–I5.

## Решение
- Бэкенд: **rustfft** (pure Rust, планировщик, AVX/NEON автодетект). Для real-сигналов далее возможно подключение `realfft` (feature).
- Окна: Hann/Hamming/Blackman.
- Паддинг: только **reflect** (запрет zero-pad) при `center=true` — в рамках R7.
- ISTFT: **sum-of-squares** нормировка окна (OLA/COLA), независимая от hop.
- Метрики: логируем `mse`, `snr_db`, `cola_max_dev` в `.wfr.json`.

## Риски
- Нестандартные комбинации окна/hop → рост `cola_max_dev`. Контроль через acceptance (FAIL при превышении порога).
- Потенциальные MINOR-изменения в API rustfft: пин до `6.4` на Freeze ветке.

## Метрики успеха
- `rel_MSE` < `1e-20` (f64) / `1e-8` (f32) на корпусе (импульс/ступень/шум/чирп).
- `cola_max_dev` ≤ `1e-12` при `pad_mode=reflect`, `center=true`.

## Миграция
- **WMLB → 1.1 (MINOR)**: расширен `.wfr.json` полями `W.params`, `W.perf`, `W.metrics`.
  Обратная совместимость: старые поля `{certificate, ops, source}` сохранены.
