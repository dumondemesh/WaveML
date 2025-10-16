# WaveML — context capsule (Freeze v1.0)
Repo: https://github.com/dumondemesh/WaveML • Branch: master • Commit: 8bb508f
Baseline: Freeze v1.0 • ABI: 1.0

## Focus
- Параметры W (n_fft/hop) + отчёт/тесты
- Ускорение STFT/iSTFT через FFT
- Align/C/A операторы и тесты

## Status
- Done: D(λ,aa), WT roundtrip, R7/R8, acceptance зелёный
- Risks: производительность FFT, семантика компоновки W→D→T, Unicode-ключи
- Next: параметризация W, rustfft, Align

## Artifacts
- build/acceptance/index.md
- build/reports/*.wfr.json
- examples/*.wml, acceptance/*.wml