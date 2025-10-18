# RC1g Final — Unified CI, workspace resolver=2, stub bins

## Steps
```bash
unzip -o WaveML_F5_RC1g_Final.zip -d .
chmod +x scripts/ci/* tools/* scripts/repair/* || true
bash scripts/ci/run_all_gates_unified.sh
```
Ожидается: валидные отчёты I2/I3 + perf-манифест без варнингов гейтов.
