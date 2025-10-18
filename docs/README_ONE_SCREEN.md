# WaveML — Ф5 (Release Candidate, v0.2.0-rc1) — One Screen

**Цель:** подготовка RC: зелёные I1–I3, чистый CI, детерминизированный батч, валидные `.wfr` без санитайзера, опциональные SDR/CoLA.

**Что добавлено (этот пакет):**
- Новые CI-гейты (`scripts/ci/*`), макОS‑френдли, POSIX.
- Детеминизация батча (`perf_determinize.sh`) — сортировка по `nf_id_hex`/пути.
- Гейт WT‑эквивалентности с опциями SDR/CoLA (`wt_equiv_gate.sh`) и чтением порогов из `acceptance/thresholds.yaml`.
- Утилиты нормализации WFR без "грязного" санитайзера (`tools/wfr-normalize.sh`, `tools/wfr-validate.sh`).
- Релизный скрипт (`scripts/release/make_release.sh`). 
- Пороговые значения: `acceptance/thresholds.yaml`.
- Планы: `docs/TEST_PLAN_F5.md`, ADR: `docs/adr/ADR-0003-WFR-Valid-Fields.md`.

**Команды (ритуал CI):**
```bash
cargo build
cargo clippy --workspace --all-targets -- -D warnings
bash scripts/ci/run_all_gates.sh
```
**Релиз RC:**
```bash
bash scripts/release/make_release.sh v0.2.0-rc1
```

**Статус:** готово для интеграции поверх корня репозитория. Скрипты не требуют изменения Rust‑кода.
