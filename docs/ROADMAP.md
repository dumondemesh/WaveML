# ROADMAP.md — Фазы ↔ Приоритеты ↔ Инварианты

## Карта фаз
| Фаза | Цель                      | Приоритеты      | Инварианты |
|-----:|---------------------------|------------------|------------|
| Ф0   | Baseline (заморозка)      | —                | —          |
| **Ф1** | Canon/IO (API, stdin/glob, schema, stdout) | **P1** | **I1=1.0** |
| **Ф2** | Swaps/MDL‑структура      | **P2**           | **I2≈1.0** |
| **Ф3** | DSP‑эквивалентность      | **P2**           | **I3≈1.0** |
| **Ф4** | Perf/DX (`--jobs`, логи) | **P2**           | **I1–I3 стабильны** |
| Ф5   | Release (артефакты/релиз) | P1/P2            | I1–I3 замкнуты |
| Ф6   | Train/MDL                 | P3               | I4/I5      |

## Борд задач (Kanban/labels)
Три дорожки:
- **CORE/P1**: `api-stable`, `stdout-contract`, `stdin/glob`, `schema`.
- **QUALITY/P2**: `l_struct`, `swaps-orbits`, `property-tests`, `perf-batch`, `logs`.
- **GROWTH/P3**: `Σ-extension`, `wfr-mdl`, `train-toy`.

Примеры лейблов: `P1`, `P2`, `P3`, `I1`, `I2`, `I3`, `CI`, `docs`.

## Sprint-календарь (2 недели → RC)
- **Дни 1–2 (Ф1/P1)**: API‑stable, stdout‑контракт, stdin/glob, schema + unit/CLI/property.  
- **Дни 3–4 (Ф2/P2)**: `L_struct`, таблица свопов, swaps‑gate.  
- **Дни 5–6 (Ф3/P2)**: `rustfft`, WT‑MSE/SDR, `wt_equiv_gate`.  
- **Дни 7–8 (Ф4/P2)**: `--jobs rayon`, стабильный вывод, логи, README 1‑экран.  
- **День 9 (Ф5)**: Golden‑набор, релиз `v0.2.0-rc1`.  
- **День 10**: дефекты/полировка.

## DoR / DoD
**DoR:** формулировка «как протестирую», метка инварианта/приоритета, где появится лог/отчёт.  
**DoD:** тесты + гейты в CI, абзац в docs/README.md, измерение/порог в PR.
