# Фаза F2 (P2): Swaps & MDL-структура

## Цели
- Ввести структурную метрику `L_struct` для канонизированного графа STRICT-NF.
- Определить орбиты допустимых свопов (swap orbits) и гейт: ΔL_struct ≤ 0.
- Подключить CLI-утилиты для моделирования свопов и генерации отчётов WFR v1.x.
- Сделать детерминированные property-тесты на эквивалентность/различия и свопы.

## Инварианты
- I2≈1.0: При допустимых свопах (`allow`) должно выполняться ΔL_struct ≤ 0, NF-ID сохраняется.
- Запрещённые свопы (`forbid`) → либо NF-ID меняется, либо линтер даёт FAIL.

## Deliverables (DoD)
- `crates/waveforge/src/l_struct.rs` — стабильная реализация L_struct.
- `crates/wavectl` — подкоманда `simulate-swaps`, `swaps-gate`.
- `acceptance/tests_i2.yaml` — позитив/негатив кейсы.
- `scripts/ci/swaps_gate.sh` — гейт I2; подключён в `run_all_gates.sh`.
- `.wfr` отчёты v1.x (cert.i2_delta_l_le_0=true для PASS-кейсов).
