# WaveML v0.2.0-rc1 — Release Notes (Фаза 5)

## Highlights
- CI стал самодостаточным: forge, schema, swaps (I2), WT‑equiv (I3), perf‑детерминизация — одним раном.
- `.wfr` нормализуется **до** проверки схем: без null‑полей, предсказуемая структура разделов `mdl`, `w_params`, `w_perf`.
- WT‑порог MSE обязателен, SDR/CoLA опциональны (включаются флагами в thresholds.yaml).

## Breaking/Behavior
- Гейт WT читает пороги из `acceptance/thresholds.yaml`, не из CLI‑флагов.
- Порядок вывода батча стабилизирован сортировкой (advisory → ok).

## Next
- Перенос логики заполнения `.wfr` из normalize‑утилиты в `wavereport` (убрать шаг нормализации).
- Добавить SYN‑фикстуры для property‑тестов канонизации.
