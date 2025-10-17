# Hotfix: tools/acceptance_overview.py

- Исправлена ошибка `'NoneType' object has no attribute 'get'` при чтении legacy-отчётов.
- Скрипт теперь умеет обрабатывать старый и новый форматы WFR (v1.x и legacy).
- Доп. флаг `--only-new` покажет только новые отчёты (с `schema_version`).

Примеры:
```bash
python3 tools/acceptance_overview.py
python3 tools/acceptance_overview.py --only-new
python3 tools/acceptance_overview.py --base build
```
