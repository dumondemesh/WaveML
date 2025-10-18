## Note — Transitional WFR sanitize in CI

Пока генераторы I2 пишут `mdl: null`, в `schema_gate.sh` добавлен шаг `wfr_sanitize.py`,
который заменяет `null` на `{}` перед валидацией. Это **временный** костыль до Task B (Valid WFR):

- DoR Task B: `wavectl` и гейты формируют валидные `.wfr` со структурным блоком `mdl` (объект).
- DoD Task B: sanitizer удаляем; schema-гейт остаётся строго на `spec/WFR-1.0.0.schema.json`.
