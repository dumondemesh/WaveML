# Repo Structure (post-clean)

```
.
├─ crates/                    # Rust workspace (waverunner, wavectl, wavelint, ...)
├─ acceptance/                # Acceptance specs and plans
│  ├─ tests.yaml              # План приёмки (reference/strict)
│  └─ data/                   # Мини-набор входных файлов (коммитим)
├─ docs/
│  ├─ README.md               # 1-экран для старта (контекст-капсула)
│  ├─ CHECKLIST_RC.md         # DoR/DoD, PR, матрица инвариантов
│  ├─ STRUCTURE.md            # Эта страница
│  ├─ CHANGELOG.md            # Коротко по релизам
│  └─ adr/
│     ├─ ADR-0002-WT-Norm-and-COLA.md
│     └─ ADR-0003-stdout-and-schema.md
├─ scripts/
│  ├─ cleanup/
│  │  ├─ clean_repo.zsh       # Удаление мусора по паттернам (+ --dry-run)
│  │  └─ collect_reports.zsh  # Сбор *.wfr.json → build/reports/
│  ├─ ci/
│  │  ├─ run_all_gates.sh
│  │  ├─ forge_gate.sh
│  │  ├─ schema_gate.sh
│  │  ├─ property_gate.sh
│  │  ├─ swaps_gate.sh
│  │  ├─ wt_equiv_gate.sh
│  │  └─ perf_gate.sh
│  └─ test/
│     └─ run_global_test.sh   # Агрегатор полного прогона
├─ Makefile                   # make all | fast | acceptance | clean | release
└─ .github/workflows/ci.yml   # GitHub Actions (build + gates)
```
