# WaveML — CI Gate (WFR v1.0.0) — 2025-10-17

Цель: закрепить **единый формат WFR 1.0.0** в CI и не допускать регресса.

## Что делает gate
1. Мигрирует все `build/**/*.wfr.json` → `build_migrated/**/*.wfr.json` в 1.0.0.
2. Валидирует по `spec/WFR-1.0.0.schema.json`.
3. Проверяет наличие ключей `cert`, `w_params`, `w_perf`, `metrics` (допускается пустота на P0).
4. (Опционально на P1) Проверяет non-empty для `w_perf/metrics` и пороги `metrics`.

## Быстрый запуск локально
```bash
bash ci/run_wfr_migration_and_check.sh
```

## Интеграция в GitHub Actions (пример шага)
```yaml
- name: WFR v1.0.0 gate
  run: bash ci/run_wfr_migration_and_check.sh
```
