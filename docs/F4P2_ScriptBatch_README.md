# F4/P2 Script Batch
- Batch NF-ID implemented as script (no Rust changes): `scripts/nf-batch`
- Parallel jobs, deterministic ordering, JSON/CSV output.
- `perf_gate.sh` uses the script to measure serial vs parallel and checks ordering.

Usage:
  scripts/nf-batch --list examples/batch/list.txt --format json --out build/nf.json --jobs 8
