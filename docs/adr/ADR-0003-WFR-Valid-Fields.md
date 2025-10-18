# ADR-0003 — Valid WFR Emission (no sanitizer)

**Context.** На Ф4 поля `.wfr` нередко приходили как `null` (mdl, w_params, w_perf). 
Схема пропускала через санитайзер. Это ломает детерминизм и затрудняет гейты.

**Decision.** На Ф5 вводим явную нормализацию перед валидацией: 
`tools/wfr-normalize.sh` гарантирует структуру и типы. 
Пороговый файл `acceptance/thresholds.yaml` делает метрики явными.

**Consequences.** 
- CI стабилен на macOS/Linux.
- Следующий шаг — перенос логики в `wavereport` и удаление normalize.
