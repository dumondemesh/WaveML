# STRUCTURE.md — Навигация по репозиторию

Основные директории:
- `crates/` — ядро и утилитные крейты (`waveforge`, `wavectl`, `wavereport`, `waveform`, `wmlb`, `linters`, `wave_logging`).
- `scripts/` — скрипты CI/cleanup/test. **Точки входа:**  
  - `scripts/test/run_global_test.sh` — глобальный тест (Ф4).  
  - `scripts/ci/*.sh` — гейты инвариантов.  
  - `scripts/cleanup/*.zsh` — уборка артефактов.
- `acceptance/` — входные файлы/план тестов (если используется `wavectl acceptance`).  
- `docs/` — этот каталог.
- `examples/` — примеры .wml/.wfm (опционально, вне критического пути).

Внутри `docs/`:
- `README.md` — быстрый старт и оглавление.
- `INVARIANTS.md` — I1–I5, правила и метрики.
- `CI_GATES.md` — спецификация гейтов, пороги, переменные окружения.
- `ROADMAP.md` — фазы/приоритеты/борд задач.
- `adr/` — принятые архитектурные решения (ADR-000x).
- `formats/`, `schemas/`, `spec/`, `templates/` — форматы, схемы и шаблоны.
- `graph.schema.json` — дублирующая корневая схема графа (канонический путь).

## Матрица «что куда пишет»
- **NF-ID/NF** → `stdout` (`--print-id`, `--print-nf`), строгий формат.
- **Объяснения** → `forge-explain` + `stderr`/логи JSON.
- **L_struct** → `forge-explain` + `.wfr`.
- **Числовые метрики (WT-MSE/SDR)** → только `.wfr`.
- **Batch-результаты** → CSV/JSON (детерминированные).
