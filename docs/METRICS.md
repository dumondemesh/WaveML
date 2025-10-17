# METRICS (P0 implementation)

- COLA (amp, periodic Hann) → metrics.cola_{max,rel}_dev, w_perf.cola_{rel_dev,pass} (tol=1e-12).
- rel_mse = 0.0 (заглушка до появления рантайма).

Валидация: `./target/debug/wavectl validate-wfr --wfr <file> --require-pass`
