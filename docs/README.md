# WaveML — контекст-капсула (Phase 4 Freeze)

- **Baseline:** I1–I3 стабильны; Perf/DX выполнены частично (логи/--jobs).
- **Цель этого шага:** уборка → консолидация документации → зелёные гейты I1–I3 → RC-подготовка.
- **Глобальный прогон:** `scripts/test/run_global_test.sh` (создаёт `build/reports/global_test_summary.md`).

**Гейты:**
- I1/forge: канонизация STRICT-NF, стабильный NF-ID (детерминизм).
- I1/schema: валидация `graph.schema.json`.
- I1–I2/property: генеративные эквиваленты/различия (seed-locked).
- I2/swaps: ΔL_struct ≤ 0 по орбитам свопов.
- I3/wt_equiv: WT-MSE ≤ порога (rustfft/COLA).
- Perf: ускорение `nf-batch --jobs` и стабильный порядок вывода (не блокер релиза).

**Куда пишет:**
- NF-ID/NF → stdout.
- Объяснения/логика → stderr/JSON logs.
- Метрики (L_struct, WT-MSE/SDR) → `.wfr.json`.
- Batch → `build/*.csv|json` (детерминированные).

**Кнопки:**
1) `scripts/cleanup/clean_repo.zsh --dry-run`
2) `scripts/cleanup/collect_reports.zsh`
3) `make all`
4) `scripts/ci/run_all_gates.sh`
