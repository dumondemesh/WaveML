# CI Gate (P0)

Локальный прогон:
```bash
bash scripts/ci/run_all_gates.sh
```
Шаги:
1. build
2. clippy strict (`-D warnings`)
3. wavectl cola → WFR
4. validate-wfr (require-pass)
Результат: PASS/FAIL.
