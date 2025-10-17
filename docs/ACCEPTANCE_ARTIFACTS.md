# Acceptance Artifacts

CI теперь публикует:
- `build_migrated/overview.md` — Markdown-обзор
- `build_migrated/overview.html` — HTML-обзор
- все `*.wfr.json` из `build_migrated` и `build_i23`

Гейты:
- `ci/wfr_schema_gate.py` — проверка наличия и типов ключей WFR (schema_version, cert.*, w_perf.*).
- `ci/forge_gate.sh` — стабильность STRICT-NF ID.
