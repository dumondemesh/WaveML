# WaveML — Phase P1 / I1 Finalized (2025-10-17)

**Scope**: STRICT-NF canonicalization + deterministic NF-ID; CLI checks; CI gates green.

## What is verified
- Canonicalization of W-nodes (n_fft/hop/window/center/pad_mode/op).
- Deterministic NF-ID (SHA-256 over canonical form).
- CLI: `forge` prints stable machine-parsable output with `--print-id`.
- CI:
  - CLIPPY-GATE: OK
  - FORGE Gate: PASS
  - NF-DIFF Gate: PASS
  - NF-BATCH Gate (+WFR validate): PASS

### Reference NF-IDs seen in CI
- eq_A / eq_B / SYN → `80a5f28eb6480102634b0cc9b6c067c0084b1a9e119c735260a880b80e2c8316`
- center=false → `4fe8feb871acf4ee11bbd7c25e3e2c3370c1bad25647c9a360e5a230611a2395`
- center=true  → `2bcd7052eb562f9d3a885a9a34d93f531cf8801995ac0e1b4da9a6b157b34b64`
- pad=toeplitz → `def3203562078c87d8b96143f180c7cab62377e5f372eb92c1e4c39a25093ab7`

## Reproduce locally
```bash
cargo build
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p waveforge --tests -- --nocapture

# Forge / NF-ID examples
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-id

# Full CI suite
bash scripts/ci/run_all_gates.sh
```

## Artefacts to keep
- `build/ci_P1_I1.log` — CI log (optional).
- `build/NF-ID_eq_A.txt` — NF-ID snapshot for eq_A (optional).

## Next phase (I2)
- Introduce `L_struct`, admissible swap orbits, and `swaps_gate.sh`.
- Extend schema and tests for additional ops.
