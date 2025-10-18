# WaveML — Cleanup & Global Test Pack (Phase 4 Freeze) — v3

**Дата:** 2025-10-18 14:53:12

**Что нового (v3):**
- Гейты не зависят от конкретных сабкоманд `wavectl`: выполняют **авто‑детекцию** возможностей CLI.
- Детреминизм (I1) теперь через `nf-batch` (если доступно) — без `print-id`.
- Acceptance (property‑gate) проверяет наличие `acceptance` сабкоманды; если её нет, **не падает**, а логирует WARN.
- Swaps‑gate проверяет наличие `simulate-swaps`; если нет, WARN (не фейлит).
- WT‑equiv поддерживает и `wt-equiv` бин, и `wavectl wt-equivalence`.
- Перф‑гейт жёстко завязан на `nf-batch` (если нет — WARN/skip).
- В пакет включены: `docs/graph.schema.json`, `acceptance/data/sample1.wml`.
  
Запуск:
```
zsh scripts/cleanup/clean_repo.zsh --dry-run
zsh scripts/cleanup/clean_repo.zsh
zsh scripts/cleanup/collect_reports.zsh
bash scripts/test/run_global_test.sh
```
