# F3 / P2 — I3 Wrap-up

Дата: 2025-10-17T22-53-23Z

## Результат
- **I3 ≈ 1.0**: `wt_equiv_gate.sh` — PASS на `sine` и `sweep`.
- Отчёты WFR с метриками: `metrics.mse`, `metrics.snr_db`, `cert.i3_conservative_functors=true` в:
  - `build/wt_equiv/i3_sine.wfr.json`
  - `build/wt_equiv/i3_sweep.wfr.json`

## Команды (факт прогона)
```bash
cargo build
target/debug/wt-equiv --signal sine  --out build/wt/sine.wfr.json
target/debug/wt-equiv --signal sweep --out build/wt/sweep.wfr.json
bash scripts/ci/wt_equiv_gate.sh   # PASS
```

## Пороговые условия (фиксированы)
- `MSE ≤ 1e-9`
- `SDR ≥ 60 dB`

## Следующие шаги
Перейти к **Ф4 / P2 (Perf/DX)**:
- `--jobs` для `nf-batch` с детерминированным порядком вывода,
- стабильные логи JSON и README (1 экран),
- добавить вызов WT-гейта в общий раннер CI.

