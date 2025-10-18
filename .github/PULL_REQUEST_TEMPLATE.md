## Phase 5 (RC1g) — Checklist
- [ ] `cargo build && cargo clippy -D warnings` green
- [ ] `bash scripts/ci/run_all_gates_unified.sh` green
- [ ] `wt_equiv.wfr.json` has `metrics.mse` and `sdr_db`
- [ ] `swaps_report.wfr.json` has `mdl.i2.delta_l_struct <= 0`
- [ ] `build/reports/manifest.txt` deterministic
- [ ] Release notes updated (`docs/RELEASE_NOTES_F5.md`)
