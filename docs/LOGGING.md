# LOGGING (сильное логирование, уровни, покрытие всех этапов)

Цель: сделать повторяемую диагностику/аудит, быстро локализовать проблемы в acceptance и рантайме.

## 1) Политика уровней
- `error` — нарушение инварианта/валидации, падение шага.
- `warn`  — восстановимые аномалии, значения близкие к порогам.
- `info`  — вехи пайплайна: что делаем, над чем, краткие итоги.
- `debug` — параметры вычислений, размеры, флаги, каноны.
- `trace` — покадровая/поштучная телеметрия, дорого (включать только точечно).

## 2) Формат и вывод
- По умолчанию — человекочитаемо на `stderr`.
- Флаг `--log-json` → JSON Lines (по строке на событие).
- Флаг `--log-dir <path>` → дублирование в файл `logs/<ts>/<tool>.log` (`.jsonl` при `--log-json`).

## 3) Общие поля события
- `ts` (ISO8601), `level`, `tool` (wavectl/tools/ci), `stage` (migrate/fill/validate/...)
- `file`/`dir` (артефакты), `n_fft`, `hop`, `window`, `center`, `pad_mode`
- `backend` (`rustfft`), `rustfft_ver`, `threads`
- `metrics.*` (`mse`, `rel_mse`, `snr_db`, `cola_max_dev`, `cola_rel_dev`, `cola_pass`)
- `wfr.schema`, `wfr.id` (STRICT-NF), `duration_ms`

## 4) Rust (workspace)
Используем фасад `log` + `env_logger` (уже в репо). Для структурного JSON — фича `tracing` (опционально).

### 4.1 Инициализация (общая функция)
```rust
// crates/wavectl/src/logging.rs  (или общий модуль wave_logging)
use std::env;
use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn init_logging(default_level: &str, json: bool) {
    let level = env::var("WAVE_LOG")
        .or_else(|_| env::var("RUST_LOG"))
        .unwrap_or_else(|_| default_level.to_string());

    let mut b = Builder::new();
    b.parse_filters(&level);
    if json {
        // Компактный JSON-писатель (можно заменить на tracing-сабскрайбер при необходимости)
        b.format(|buf, record| {
            use std::io::Write;
            let ts = chrono::Utc::now().to_rfc3339();
            let line = format!(
                "{{"ts":"{}","level":"{}","target":"{}","msg":{}}}\n",
                ts, record.level(), record.target(), serde_json::to_string(&record.args()).unwrap()
            );
            buf.write_all(line.as_bytes())
        });
    } else {
        b.format_timestamp_millis();
        b.target(Target::Stderr);
    }
    b.init();
}
```

### 4.2 Использование в `main()`
```rust
fn main() -> anyhow::Result<()> {
    // парсим CLI: --log-level, --log-json, --log-dir
    wave_logging::init_logging(cli.log_level.as_deref().unwrap_or("info"), cli.log_json);
    log::info!("start";);
    // ... работа
    Ok(())
}
```

### 4.3 Полезные best‑practices
- Использовать `anyhow::Context` в ошибках: `?` + подробности в логи.
- В местах горячих циклов — понижение уровня до `trace` и выключено по умолчанию.
- Для длительных операций — логировать `start`,`end`,`duration_ms`.

## 5) Python (tools/*, ci/*)
Единый хелпер:
```python
# tools/logging_setup.py
import logging, json, time, sys

class JsonFormatter(logging.Formatter):
    def format(self, record):
        obj = {
            "ts": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(record.created)),
            "level": record.levelname.lower(),
            "tool": record.name,
            "msg": record.getMessage(),
        }
        if record.__dict__.get("extra"):
            obj.update(record.__dict__["extra"])
        return json.dumps(obj, ensure_ascii=False)

def setup_logging(level="INFO", json_mode=False, to_file=None):
    logger = logging.getLogger()
    logger.setLevel(level.upper())
    fmt = JsonFormatter() if json_mode else logging.Formatter("[%(levelname)s] %(message)s")
    h = logging.FileHandler(to_file, encoding="utf-8") if to_file else logging.StreamHandler(sys.stderr)
    h.setFormatter(fmt); logger.handlers.clear(); logger.addHandler(h)
```

Использование:
```python
from logging_setup import setup_logging
setup_logging(level=args.log_level, json_mode=args.log_json, to_file=args.log_file)
logging.info("migrating", extra={"extra": {"stage":"migrate","src":args.src,"dst":args.dst}})
```

Добавить в основные скрипты аргументы:
```
--log-level {error,warn,info,debug,trace}
--log-json
--log-file <path>
```

## 6) Покрытие этапов (минимум полей)
- **migrate_wfr.py**: `stage=migrate`, `in`, `out`, количество файлов, время.
- **fill_metrics.py**: `stage=metrics`, `file`, `n_fft`, `hop`, `window`, `cola_*`, `pass`.
- **overview_v3.py**: `stage=overview`, `total`, `empty`, `ok`.
- **wfr_schema_gate.py / overview_gate.sh / forge_gate.sh**: `stage=gate`, `gate_name`, `result`, `fail_items`.
- **simulate_swaps.py / verify_i23.py / acceptance_runner.py**: `stage=acceptance`, `case`, `expect`, `got`.
- **wavectl** (все подкоманды): `cmd`, `args`, `duration_ms`, для `cola` — метрики окна; для `report-from-graph` — идентификаторы графа; для `validate-wfr` — `cola_rel_dev`, `cola_pass`.
- **Рантайм STFT/iSTFT**: `backend`, `rustfft_ver`, `frames`, `n_fft`, `hop`, `threads`, `wall_ms`.

## 7) Артефакты логов
- По умолчанию — только `stderr`.
- При `--log-dir build/logs` — писать файлы:
  - `build/logs/<ts>/migrate.log[.jsonl]`
  - `build/logs/<ts>/metrics.log[.jsonl]`
  - `build/logs/<ts>/acceptance.log[.jsonl]`
  - `build/logs/<ts>/wavectl_<cmd>.log[.jsonl]`
- В `.wfr` добавлять краткую выжимку: `w_perf.log_digest = { level, started_ts, duration_ms }`.

## 8) Включение по умолчанию
- CLI флаги добавлены в чек-лист (см. §11 RELEASE_CHECKLIST).
- В CI — `WAVE_LOG=info` для человекочитаемого, `--log-json` для артефактов.
