# ADR: Унификация формата WFR → v1.x

## Решение
Единая схема WFR v1.x: { cert, mdl, phase, swap, w_params, w_perf, metrics, log[*] }.
Сериализация: JSON (JCS), schema_semver.

## Мотивация
Снятие разночтений старого/нового формата, прямой импорт в BI/CI и стабильная валидация.

## Последствия
- Единый валидатор.
- CLI и рантайм обязаны писать `w_perf` + ключевые события `log[*]`.
- Acceptance-пороги вынесены в `acceptance/thresholds.yaml`.
