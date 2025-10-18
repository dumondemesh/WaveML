# Phase 5 — Test Plan

1) Determinism
```
for i in 1 2 3; do bash scripts/ci/run_all_gates_unified.sh; done
shasum build/reports/manifest.txt
git status --porcelain build/ | wc -l  # expect 0
```

2) WT Metrics presence
```
jq '.metrics' build/acceptance/wt_equiv.wfr.json
# expect mse and sdr_db
```

3) ΔL_struct validation
```
jq '.mdl.i2' build/acceptance/swaps_report.wfr.json
# expect delta_l_struct <= 0 and pass=true
```

4) Perf manifest stability
```
cat build/reports/manifest.txt
# order is deterministic across runs
```

5) Workspace guard
- Add a bogus member to `Cargo.toml`, rerun the unified runner;
- It should rewrite workspace with resolver=2 and existing crates only.
