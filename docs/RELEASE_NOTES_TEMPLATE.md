# WaveML vX.Y.Z — Release Notes (template)

## Highlights
- WFR schema: migrated to **1.0.0**.
- Metrics: filled (`cola_rel_dev`) with **periodic Hann**, amp mode, tol=1e-12.
- I2/I3 scaffold acceptance: **PASS** (simulate_swaps + verify_i23).
- Forge NF-ID stability: **PASS** (STRICT-NF).
- Overview published and validated: **OK**.

## Changes
- `waveeval`: COLA API cleanup; `ColaMode` parsing simplified; `cola_ok` takes `&str` mode; unitless relative deviations.
- `wavectl`: improved CLI, schema/overview/validate helpers.
- Tooling: migration + metrics fillers, acceptance runners, schema/overview gates.

## Breaking / Notes
- Some acceptance WFRs intentionally keep `w_params` empty; metrics section now ensures presence of `w_perf.cola_pass` (false by default when params are absent).
- Consumers should not rely on missing keys; expect explicit `false`/values.

## How to Verify Locally
```bash
bash ci/run_ci_local.sh
bash ci/run_all_gates.sh
./target/debug/wavectl cola --n-fft 1024 --hop 512 --window Hann --mode amp --tol 1e-12
./target/debug/wavectl validate-wfr --wfr build_migrated/reports/auto_amp.wfr.json --require-pass
```

## Artifacts
- `wfr_bundle.tgz` — migrated WFR set.
- `overview.html`, `overview.md` — acceptance overview.

## Checksums (fill after build)
```text
SHA256(wfr_bundle.tgz)= ...
SHA256(overview.md)= ...
SHA256(overview.html)= ...
```

## Thanks
Shout-out to the acceptance/CI scripts for catching clippy + schema regressions early.
