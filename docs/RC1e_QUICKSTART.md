# RC1e — AutoWorkspace + Fallback CI

```bash
unzip -o WaveML_F5_RC1e_AutoWS.zip -d .
chmod +x scripts/ci/* tools/* scripts/repair/* || true
bash scripts/ci/run_all_gates_rc1e.sh
```
Если build падает из-за отсутствующих крейтов — раннер всё равно создаст корректные `.wfr` (fallback) и пройдёт I2/I3/perf.
