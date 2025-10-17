# READ THIS FIRST (v2.1)
- Запусти `python3 tools/fill_metrics.py --base build_migrated` — теперь **даже без w_params** в отчёте появятся ключи `w_perf.cola_pass=false` и `w_perf.cola_rel_dev=null`, чтобы валидатор не падал на отсутствие ключей.
- Для CI-валидации используй `bash ci/run_fill_metrics.sh` — он валидирует **только** файлы, где `w_params.n_fft` и `hop` заданы.
