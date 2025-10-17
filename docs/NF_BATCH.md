# `wavectl nf-batch`

Пакетный расчёт NF-ID для набора входов.

## Примеры

```bash
# JSON массив
wavectl nf-batch \
  --input examples/graph/forge_eq_A.json \
  --input examples/graph/forge_eq_B.json \
  --json

# CSV (с заголовком)
wavectl nf-batch \
  --input examples/graph/forge_eq_A.json \
  --input examples/graph/forge_eq_B.json \
  --csv

# Из списка путей (по строкам)
wavectl nf-batch --list paths.txt --json > out.json
```

Опции:
- `--input <PATH>` — можно указывать несколько раз;
- `--list <FILE>` — файл с путями, один на строку (пустые и строки с `#` игнорируются);
- `--json` — печать JSON-массива вида `[{ "path": "...", "nf_id": "..." }, ...]`;
- `--csv` — печать CSV с заголовком `path,nf_id`;
- `--out <FILE>` — вывести результат в файл (по умолчанию — stdout).
