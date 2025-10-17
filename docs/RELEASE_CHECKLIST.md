# WaveML — Release Checklist (local)

> Быстрый чеклист, чтобы не нарваться на красные гейты перед релизом.

## 0) Предусловия
- Рабочее дерево чистое: `git status` → clean.
- Установлены зависимости (rustup, cargo, python3).

## 1) Локальные проверки
```bash
bash ci/run_ci_local.sh
bash ci/run_all_gates.sh
```
Ожидаем:
- **CLIPPY-GATE: OK**
- **UNIT-TEST-GATE: OK**
- **FORGE GATE: PASS**
- **SCHEMA-GATE: OK**
- **OVERVIEW-GATE: OK**

## 2) Сборка артефактов релиза
```bash
# мигрированный набор и отчёт
bash ci/run_acceptance_all.sh build build_migrated build_i23

# упаковать артефакты (WFR + overview)
bash ci/release_artifacts.sh build_migrated out
ls -l out/
```

Проверяем, что появились файлы:
- `out/wfr_bundle.tgz`
- `out/overview.md`
- `out/overview.html`

(Опционально) Чексуммы:
```bash
shasum -a 256 out/wfr_bundle.tgz out/overview.md out/overview.html
```

## 3) Финальная валидация выборочно
```bash
./target/debug/wavectl validate-wfr --wfr build_migrated/reports/auto_amp.wfr.json --require-pass
```

## 4) Тег + пуш
```bash
VERSION="vX.Y.Z"
git add -A
git commit -m "release: $VERSION — WFR v1.0.0 migration, metrics filled, acceptance gates green"
git tag -a "$VERSION" -m "WaveML $VERSION"
git push origin main --tags
```

## 5) GitHub Release
- Название: `WaveML {VERSION}`
- Прикрепить: `out/wfr_bundle.tgz`, `out/overview.html`, `out/overview.md`
- В описание вставить Release Notes (см. шаблон ниже).

## 6) Пост-релизная проверка
- Ссылки на артефакты открываются.
- Из архива `wfr_bundle.tgz` читаются WFR, совпадают NF-ID (Forge gate).
