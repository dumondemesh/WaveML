# Overview / Schema gate quick notes

- `tools/overview_v3.py` is **null-safe**. Missing or `null` keys (`cert`, `w_params`, `w_perf`, `metrics`) no longer cause crashes.
- `ci/publish_overview.sh` wraps the tool and writes Markdown to `<base>/overview.md`.

## Typical run

```bash
bash ci/publish_overview.sh build_migrated
python3 ci/wfr_schema_gate.py build_migrated
bash ci/overview_gate.sh build_migrated
```
