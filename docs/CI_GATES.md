# CI_GATES.md — Гейты и переменные окружения

Запуск всего набора:
```bash
bash scripts/test/run_global_test.sh
```

Гейты вызываются в `scripts/ci/run_all_gates.sh`:

1. **forge_gate (I1)** — детерминизм. Порядок fallback:
   - `nf-batch` (если доступен);
   - `simulate-swaps` (2 прогона, канонизация отчёта);
   - `wt-equiv` (2 прогона, `RAYON_NUM_THREADS=1`, канонизация + квантизация чисел, сравнение `.metrics`).  
   Переменные:
   - `STRICT_I1=0|1` — делать FAIL при расхождении (по умолчанию WARN в режиме уборки).
   - `I1_NUM_EPS=1e-9` — квантование чисел перед сравнением.

2. **schema_gate (I1)** — наличие `docs/graph.schema.json` и корректный JSON.

3. **property_gate (I1/I2)** — если нет `wavectl acceptance`, гейт **OK** при наличии фикстур (`acceptance/*.wml`).

4. **swaps_gate (I2)** — `wavectl simulate-swaps --input … --out …`.  
   - Ищет `ΔL_struct` в отчёте, при отсутствии — WARN (не фейлит).  
   - Переменная: `SWAPS_INPUT=path/to/*.wml` — вход по умолчанию.

5. **wt_equiv_gate (I3)** — `wt-equiv --out …`.  
   - Поддерживает MSE/SDR; при отсутствии метрик — WARN.  
   - `WT_MSE_THRESH` (по умолчанию `1e-8`), `WT_SDR_MIN_DB` (по умолчанию `60`), `STRICT_I3=0|1`.

6. **perf_gate** — пропускается, если нет `nf-batch`.

## Канонизация отчётов
Удаляем volatile‑поля (`timestamp`, `uuid`, `seed`, `hostname`, времена), сортируем ключи/массивы, квантуем числа до `I1_NUM_EPS`.

## Примеры
```bash
# строгий I1 с жёстким сравнением после квантования
STRICT_I1=1 I1_NUM_EPS=1e-9 bash scripts/ci/forge_gate.sh

# ослабленные пороги для I3 (на этапе уборки)
WT_MSE_THRESH=1e-6 WT_SDR_MIN_DB=50 STRICT_I3=0 bash scripts/ci/wt_equiv_gate.sh
```
