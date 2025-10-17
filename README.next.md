# WaveML — Quick Start (CLI current)

## Build
```bash
cargo build
```

## Lint (R7/R8/R9)
```bash
./target/debug/wavectl lint --input examples/graph/demo_w.json
```

## COLA and report
```bash
./target/debug/wavectl cola --n-fft 1024 --hop 512 --window Hann --mode amp --tol 1e-12
./target/debug/wavectl report-from-graph --input examples/graph/demo_w.json \
  --out build/reports/auto_amp.wfr.json --mode amp --tol 1e-12
./target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass
```

## STRICT-NF
```bash
./target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-id
```

## I2/I3 — simulate-swaps
```bash
./target/debug/wavectl simulate-swaps --input examples/graph/i23_pass.json \
  --out build/i23_pass.wfr.json
```

## Acceptance — all
```bash
make all
```
