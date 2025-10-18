# F2 / P2 — I2 Wrap-up

Дата: 2025-10-17T22-48-46Z

## Результат
- **I2 ≈ 1.0**: гейт `swaps_gate.sh` — PASS на планах из `acceptance/tests_i2.yaml`.
- WFR-отчёты с `cert.i2_delta_l_le_0` сформированы в `build/acceptance_i2/*.wfr.json`.
- Метрика `L_struct` рассчитывается детерминированно.

## Логи команд
```bash
cargo build                       # OK
bash scripts/ci/swaps_gate.sh     # PASS (advisory validate-wfr warnings ok)
```
Пример вывода:
```
[I2] Swaps Gate: PASS
```

## Известные предупреждения
- `validate-wfr` отсутствует/не активирован → предупреждение в логе. Не блокирует PASS.
  - Варианты: (а) добавить сабкоманду `validate-wfr` в wavectl; (б) оставить advisory.

## DoD подтверждён
- Орбиты-допуски: ΔL_struct ≤ 0, NF-ID сохраняется.
- Орбиты-запреты: FAIL линтера/контракта.

## Следующая фаза
Перейти к **F3 / P2 (DSP-эквивалентность)**: rustfft, WT-MSE/SDR гейт.
