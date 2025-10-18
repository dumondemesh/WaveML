# Versions — WaveML Freeze v1.0 → minor updates

- **WaveML**: 1.0 (Freeze)
- **WMLB**: **1.1 (MINOR)** — расширен формат `.wfr.json`:
    - `W.params` = `{ bank, n_fft, hop, window, center, pad_mode }`
    - `W.perf` = `{ backend, wall_ms, frames, n_fft, hop, threads?, rustfft_ver? }`
    - `W.metrics` = `{ mse, snr_db, cola_max_dev, rel_mse }`
      Обратная совместимость: старые поля `{ certificate, ops, source }` сохранены.
- **WaveForm/WaveStream/WaveReport/WaveBit/WMPKG**: 1.0 (без изменений)
