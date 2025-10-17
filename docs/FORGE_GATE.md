# Forge Gate & Overview

- `ci/forge_gate.sh` — фейлит CI, если эквивалентные графы дают разные STRICT-NF ID.
- `ci/publish_overview.sh` — рендерит Markdown-обзор из `tools/overview_v3.py` в `build_migrated/overview.md`.
- `ci/run_acceptance_all.sh` — объединённый раннер всех приёмок.

Включено в `.github/workflows/waveml-ci.yml`.
