# WaveML — Test Plan (Ф5 / v0.2.0-rc1)

## Scope
- I1: Forge/NF-ID determinism, Schema compliance, Property (A==B==SYN)*
- I2: Swap orbits (ΔL_struct ≤ 0), structural MDL guard
- I3: WT equivalence — MSE mandatory, SDR/CoLA optional (thresholds.yaml)

## Environments
- macOS 12+ (BSD sed, /usr/bin/time), Rust stable
- Linux (GNU sed), Rust stable

## Data/Artifacts
- `acceptance/tests.yaml` — список кейсов (исходный в репо)
- `acceptance/thresholds.yaml` — пороги метрик WT
- `build/acceptance/*.wfr.json` — результаты ранa
- `forge/*.nf.json` — нормальные формы

## Steps
1. **Build & Lint**
   ```bash
   cargo build
   cargo clippy --workspace --all-targets -- -D warnings
   ```
2. **Run CI Gates**
   ```bash
   bash scripts/ci/run_all_gates.sh
   ```
3. **Inspect Artifacts**
   - `build/reports/*.wfr.json` → без `null` в ключах `mdl`, `w_params`, `w_perf`
   - `scripts/ci/wt_equiv_gate.sh` → отчет по MSE/SDR/COLA (с порогами)
4. **Deterministic Batch Check**
   - Повторить CI 3 раза, сравнить sha256 артефактов (ожидается совпадение).
5. **Property (SYN)** — (*будет активировано, когда фикстуры появятся*)
6. **WT Strict Mode**
   - Включить `require_sdr: true`, `require_cola: true` в thresholds.yaml и убедиться, что гейт падает/проходит корректно.

## Exit / DoD
- Все гейты PASS с `require_sdr/cola: false`
- Повторные прогоны дают идентичные хэши артефактов.
- Налицо валидные `.wfr` (без san) и стабильные отчеты.
