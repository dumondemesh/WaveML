[![WaveML Acceptance](https://github.com/dumondemesh/WaveML/actions/workflows/acceptance.yml/badge.svg)](https://github.com/dumondemesh/WaveML/actions/workflows/acceptance.yml)
# WaveML (proto)

WaveML — минимальный стек для декларативного аудио-ML графа: компилятор (WML→IR),
линтеры (R7/R8), рантайм (W/T/D), отчёты `.wfr`, acceptance-раннер.

## Быстрый старт
```bash
cargo build
cargo run -p wavectl -- compile examples/hello_down.wml -o build/hello_down.wmlb.json --strict
cargo run -p wavectl -- run build/hello_down.wmlb.json --in examples/dummy.wfm.json --out build/out.wfm.json
cargo run -p wavectl -- acceptance --plan acceptance/tests.yaml --outdir build/acceptance --strict
