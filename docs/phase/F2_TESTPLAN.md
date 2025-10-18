# Test Plan — F2 / P2

## Test Matrix
1) Allowed Orbits (PASS):
   - Swap(W₁,W₂) где коммутативность легальна по R7/R8/R9 и табл. орбит.
   - Ожидание: NF-ID неизменен; ΔL_struct ≤ 0; `.wfr.cert.i2_delta_l_le_0 = true`.

2) Forbidden Patterns (FAIL):
   - A∘Align (R9), D∘W без AA (R8), W∘T при edge=zero/zero-pad (R7).
   - Ожидание: либо NF-ID меняется, либо линтер FAIL; `.wfr.cert.i2_delta_l_le_0 = false`.

3) MDL Envelope Check:
   - Несколько законных свопов подряд — суммарная ΔL_struct ≤ 0.

## Acceptance Files
- `acceptance/tests_i2.yaml` — перечень сценариев.
- Генерация WFR: `wavectl simulate-swaps --input ... --out build/i2_*.wfr.json`
- Валидация: `wavectl validate-wfr --wfr ... --require-pass --schema docs/schemas/wfr.v1.schema.json`

## CI Gates
- `scripts/ci/swaps_gate.sh`:
  - Пробег по `acceptance/tests_i2.yaml`
  - Контроль порогов: ΔL_struct ≤ 0 (allowed), FAIL (forbidden)
  - В конце — суммарный PASS/FAIL

## DoR
- Описаны ожидаемые исходы по каждой задаче.
- Готовы примеры входных графов в `examples/graph/i2_*`.

## Команды
```bash
# Моделирование одной орбиты:
target/debug/wavectl simulate-swaps   --input examples/graph/i2_allow_commute_wt.json   --out build/i2_allow_commute_wt.wfr.json   --check-i2

# Гейт всех кейсов I2:
bash scripts/ci/swaps_gate.sh
```
