# WaveML Patch — 2025-10-17

## Что это
Набор готовых файлов для:
- Унификации отчётов **WFR v1.x**
- Введения единого логирования (`tracing`) — crate `wave_logging`
- Обновления канона **STRICT-NF** (добавлены `center`, `pad_mode`)
- Порогов метрик `acceptance/thresholds.yaml`
- CI-воронки (`scripts/ci/run_all_gates.sh`, `.github/workflows/ci.yml`)

## Как применить
1. Распакуйте архив в корень репозитория (с заменой файлов).
2. Добавьте `wave_logging` в workspace (если используете workspace):

   **workspace Cargo.toml**:
   ```toml
   [workspace]
   members = [
     "crates/wave_logging",
     "crates/wavereport",
     # ... ваши остальные крейты
   ]
   ```

3. В бинарях (`wavectl` и т.п.) инициализируйте логирование в самом начале `main()`:
   ```rust
   wave_logging::init_from_env();
   // или на флагах CLI: wave_logging::init("info","compact");
   ```

4. Переход на WFR v1.x:
   - Используйте API из `wavereport::WfrV1` для записи отчётов.
   - Валидируйте через `docs/schemas/wfr.v1.schema.json`.

5. Запустите CI локально:
   ```bash
   chmod +x scripts/ci/run_all_gates.sh
   scripts/ci/run_all_gates.sh
   ```

## Примечания
- Отредактируйте `acceptance/thresholds.yaml` под ваши реальные пороги.
- Обновите `docs/versions.md` по мере появления новых полей.
- Для NF-ID включите `center`/`pad_mode` в коде `waveforge` (ключ сортировки).

Создан: 2025-10-17T14:15:46.599810Z
