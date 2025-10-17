# WaveML — Next Acceptance & Gates

This bundle adds extra CI gates to keep the repo healthy:
- **Strict Clippy Gate**: `cargo clippy -D warnings`
- **Unit Tests Gate**: `cargo test --all`
- **Deps Gate**: forbids `serde_yaml` with `+deprecated` feature pin
- **Docs Gate**: checks presence of `docs/STRICT-NF.md`
- Works alongside existing: migrate→metrics→validate, schema-gate, overview-gate, forge gate, I2/I3 scaffold.

## Local run
```bash
bash ci/run_all_gates.sh
```

## Release artifacts
```bash
bash ci/release_artifacts.sh build_migrated out
ls -l out/
```

Integrate into GH Actions (see workflows in `.github/workflows`).