# COLA reports via wavectl

Examples:
```bash
# Generate .wfr with COLA metrics (amp mode, Hann, 50% overlap)
./target/debug/wavectl report-cola \
  --module demo \
  --out examples/demo_cola_amp.wfr.json \
  --n-fft 1024 --hop 512 --window Hann --mode amp --tol 1e-12

# Generate .wfr with COLA metrics (power mode, Hann, N/4 hop)
./target/debug/wavectl report-cola \
  --module demo \
  --out examples/demo_cola_power.wfr.json \
  --n-fft 1024 --hop 256 --window Hann --mode power --tol 1e-12
```
Outputs include `w_params` and `w_perf` in the `.wfr`.
