# Usage

## Forge (STRICT-NF)
Канонизация графа и контроль NF-ID.

```bash
# Печать NF-ID
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-id

# Вывести канонический граф и сохранить
target/debug/wavectl forge --input examples/graph/forge_eq_A.json --print-nf --out build/forge/forge_eq_A.nf.json
```

## COLA → WFR
```bash
target/debug/wavectl cola --n-fft 1024 --hop 512 --window hann --mode amp --out build/reports/auto_amp.wfr.json
target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass
```
