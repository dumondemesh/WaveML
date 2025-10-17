# Usage

## 1) Миграция legacy → WFR 1.0.0
```bash
python3 tools/migrate_wfr.py --src build --dst build_migrated
```

- Скрипт просканирует `--src` (`build` по умолчанию), найдёт все `*.wfr.json`.
- Legacy-отчёты будут преобразованы в v1.0.0 и записаны в `--dst`, сохраняя относительные пути.
- Новые отчёты (у которых уже есть `schema_version`) копируются как есть, но при необходимости дополняются пустыми `cert/w_perf/metrics`.

## 2) Обзор после миграции
```bash
python3 tools/overview_v3.py --base build_migrated
```

- Видно, где пустые поля в `cert`, `metrics`, `w_perf`, чтобы закрыть P0/P1 задачами.

## 3) Интеграция с CI (предложение)
Добавьте отдельный job:
- запустить `migrate_wfr.py` в артефактах билда,
- проверять схему `spec/WFR-1.0.0.schema.json`,
- падать, если ключи отсутствуют.
