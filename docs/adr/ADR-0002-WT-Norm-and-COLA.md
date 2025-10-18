# ADR-0002 — WT-Norm и COLA (rustfft)

**Решение.** Переходим на `rustfft` для W/T операторов; нормировка по COLA (Hann/Hamming, hop=N/2).

**Порог эквивалентности.** WT-MSE ≤ 1e-9 (синтетика: импульс/синус/шум; центрирование и pad_mode фиксированы).

**Последствия.** Гейт `wt_equiv_gate.sh`, отчёт `.wfr.json: W.params (n_fft, hop, window, center, pad_mode), perf`.
