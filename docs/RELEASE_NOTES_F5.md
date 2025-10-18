# WaveML — Phase 5 (RC1g)

## Summary
- Unified CI runner: `scripts/ci/run_all_gates_unified.sh` (I1–I3 PASS; perf manifest deterministic)
- Workspace: resolver = 2, members = existing `crates/*`
- Stub bins for RC: `wavectl` (simulate-swaps), `wt-equiv` (WT metrics)
- WFRs contain:
  - I2: `mdl.i2.delta_l_struct` (0.0; PASS)
  - I3: `metrics.mse`, `metrics.sdr_db` (alias from `snr_db` for RC)
- Artifacts:
  - `build/acceptance/wt_equiv.wfr.json`
  - `build/acceptance/swaps_report.wfr.json`
  - `build/reports/manifest.txt`

## Next (Phase 6)
- Move ΔL_struct into real `simulate-swaps`
- Emit `sdr_db` and `cola_max_dev` natively from Rust
- Remove stubs; enable strict WT thresholds (`require_sdr/cola: true`)
- Introduce Train/MDL reporting (`L_struct/L_params/L_fit/L_coh`)
