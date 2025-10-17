# wavectl (hotfix E0283)

- Убран `.into()` в `create_dir_all` и добавлена проверка `parent()`.
- Совместимо с clippy `-D warnings`.

Sentinel: `WAVECTL_SENTINEL_v035_E0283_PARENT_GUARD`
