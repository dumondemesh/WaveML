# ADR-0003 — Строгий stdout и schema

**Решение.** 
- `--print-id` печатает **только** NF-ID (hex, без перевода строки). 
- `--print-nf` печатает STRICT-NF в каноническом JSON (JCS).
- Все сложные объяснения и метрики выводятся в `.wfr.json` и/или stderr JSON.

**Мотивация.** Стабильные пайплайны, детерминизм, удобство `cut/xargs` и сравнения.

**Последствия.** Обновить CLI/README, добавить schema в `docs/graph.schema.json`, гейт `schema_gate`.
