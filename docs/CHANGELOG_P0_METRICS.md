# P0 Metrics Integration (Rust)
- waveeval: periodic Hann + COLA rel/max dev (Amp/Power).
- wavereport: WFR v1.0.0 writer now fills `metrics.cola_{max,rel}_dev`, `metrics.rel_mse=0` (placeholder),
  and `w_perf.cola_{rel_dev,pass}` with default tolerance 1e-12.
- schema placeholder added to `spec/WFR-1.0.0.schema.json` (minimal P0).
