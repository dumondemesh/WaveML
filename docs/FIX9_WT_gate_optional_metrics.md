# Fix9 — WT equivalence gate: SDR/COLA optional in F4

This patch relaxes `scripts/ci/wfr_check_metrics.py` so that:
- MSE is **required** and checked against `i3.wt_mse_max`.
- `sdr_db` and `cola_max_dev` are **optional by default** for Phase 4 (Perf/DX).
- You can enforce them by setting flags in `acceptance/thresholds.yaml`:
  ```yaml
  i3:
    wt_mse_max: 1e-9
    sdr_db_min: 60
    cola_max_dev: 1e-12
    require_sdr: true
    require_cola: true
  ```

This matches our F4 goal: keep CI robust while the metric emitters are being wired back.
