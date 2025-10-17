# WaveML — STRICT-NF Canon (Phase P1/I1)

This drop introduces a **stable API** in `waveforge`, a **strict stdout contract** in `wavectl forge`,
support for **stdin/glob inputs**, a **Graph JSON Schema**, and **property-based tests**.

## Quickstart
```bash
# Build
cargo build

# Canonical NF-ID (hex only on stdout)
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-id

# Canonical NF JSON
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-nf

# Check if already canonical (exit 0 if yes)
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --check

# Stdin / glob
cat examples/graph/forge_eq_A.json | target/debug/wavectl forge -i - --print-id
target/debug/wavectl forge -i "examples/graph/*.json" --print-id

# Run CI gates for Phase P1/I1
bash scripts/ci/run_all_gates.sh
```

## Stable API
```rust
use waveforge::{canonicalize_graph, nf_id_hex};
```

## Schema
- `schemas/graph.schema.json` (schema_semver: 1.0.0)
