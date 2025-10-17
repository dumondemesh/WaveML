# WaveML — Acceptance & CI quick start

## Local workflow

```bash
cargo build
# Generate a sample WFR to seed the pipeline
./target/debug/wavectl report-from-graph \  --input examples/graph/demo_w.json \  --out build/reports/auto_amp.wfr.json \  --mode amp --tol 1e-12

# End-to-end run (migrate → metrics → validate → I2/I3 → overview → gates)
bash ci/run_acceptance_all.sh build build_migrated build_i23

# Strict schema gate
python3 ci/wfr_schema_gate.py build_migrated

# Overview publishing
bash ci/publish_overview.sh build_migrated
bash ci/overview_gate.sh build_migrated
```

## GitHub Actions

The workflow `.github/workflows/waveml-ci.yml` runs on every push/PR:
- **Build** (Rust stable), clippy via your local settings.
- **Acceptance** with `ci/run_acceptance_all.sh`.
- **Schema gate** (`ci/wfr_schema_gate.py`).
- **Overview** generation + artifact upload (`overview.{md,html}`, WFRs).

## Notes

- `ci/fix_serde_yaml_dep.sh` safely replaces `0.9.34+deprecated` → `0.9.34` in `crates/wavectl/Cargo.toml` if present.
- All gates are idempotent; re-runs are safe.
