# CHECKLIST — перед RC

- [ ] `wavectl forge --print-id` печатает ровно 64 hex и `\n`.
- [ ] `schema_gate.sh` валидирует WMLB-1.1 и WFR-1.0.0, падает при несоответствии.
- [ ] `property_gate.sh` включает SYN-фикстуры и запрещённые композиции (линтеры).
- [ ] `swaps_gate.sh` формирует валидные `.wfr` без `null/false` заглушек.
- [ ] `wt_equiv_gate.sh` пишет `W.params/W.perf/metrics` и проходит пороги.
- [ ] `perf_gate.sh` показывает стабильный вывод при `--jobs` и фиксирует `elapsed_sec`.
- [ ] `docs/release/PHASE_STATUS.md` отражает реальный статус CI.
