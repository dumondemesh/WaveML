# WaveML — RC Capsule (1 экран)

**Repo:** (ваш)  •  **Baseline:** Freeze v1.0  •  **Active:** Фаза 4 (Perf/DX)  
**Last done:** Forge/NF-ID gate, Schema gate, WT-equivalence on synthetic  
**Next up:** nf-batch (--jobs) + стабильный порядок вывода; логи JSON; README

**Open risks:** несоответствие схем (graph vs WMLB-1.1); плейсхолдеры .wfr для I2; канонизация в CLI вместо либы

**Artifacts:** `scripts/ci/run_all_gates.sh`, `acceptance/thresholds.yaml`, `docs/adr/ADR-0003-stdout-and-schema.md`

**How to run (local):**
```bash
cargo build
bash scripts/ci/run_all_gates.sh
```
