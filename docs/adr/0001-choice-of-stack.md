# ADR-0001: Стек и форматы (Rust/ABI/WaveForm)
## Context
Нужны строгие линтеры и быстрый рантайм → язык Rust; формат IR — JSON (wmlb); сигнал — WaveForm (JSON).

## Decision
- Язык: Rust (crates: waveforge, wavelint, waverunner, wavereport, wavectl)
- Форматы: WML (текст), IR (wmlb.json), отчёты (.wfr.json)
- Базовые операторы: W/T/D; линтеры R7/R8 обязательны

## Consequences
- Простая интеграция в CI; контроль через acceptance; переносимость
