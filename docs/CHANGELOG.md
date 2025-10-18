## 2025-10-17 — v0.2.0-rc1 (Phase P1/I1)

- waveforge: add stable API `canonicalize_graph` and `nf_id_hex` (deprecate internal names).
- wavectl forge: strict stdout contract for `--print-id` (hex only); add `--with-banner` for human output.
- wavectl forge: add `--input -` (stdin) and glob input support.
- schemas: add `graph.schema.json` (schema_semver=1.0.0).
- tests: add property-based tests for STRICT-NF (proptest).
- CI: add `schema_gate.sh`, `property_gate.sh`, update `run_all_gates.sh`.
