# docs/acceptance — как читать отчёты

- Основные отчёты складываются в `build/acceptance/*.wfr.json`.
- Для I2 ожидается поле `ΔL_struct` или, альтернативно, `metrics.delta_L_struct.max`.
- Для I3 — `metrics.mse.max` или `metrics.sdr_db.min`.
- В отсутствии полей гейты переходят в **WARN** (не блокируют Ф4).
