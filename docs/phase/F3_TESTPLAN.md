# Test Plan — F3 / P2 — WT Equivalence

## Набор тестов
1) `sine_1k`: синус 1 кГц, SR=48k, длительность 0.1s, N=512, hop=256, Hann, center=false.
2) `sweep`: линейный свип 200→4000 Гц, SR=48k, длительность 0.1s, те же параметры окна.

## Критерии прохождения
- `MSE ≤ 1e-9`
- `SDR ≥ 60 dB`
- Флаг `.cert.i3_conservative_functors = true`

## Команды
```bash
# Локальный запуск одиночного теста:
target/debug/wt-equiv --signal sine --out build/wt/sine.wfr.json

# Гейт:
bash scripts/ci/wt_equiv_gate.sh
```
