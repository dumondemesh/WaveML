//! WaveML unified logging based on `tracing`.
//! Use:
//!   wave_logging::init_from_env();    // honors WAVE_LOG and WAVE_LOG_FORMAT
//!   wave_logging::init("info","compact");
//!
//! Env vars:
//!   WAVE_LOG=trace|debug|info|warn|error (default: info)
//!   WAVE_LOG_FORMAT=compact|full|json   (default: compact)

use tracing_subscriber::{EnvFilter};

pub fn init(level: &str, format: &str) {
    let filter = EnvFilter::try_new(level).expect("invalid log level");
    match format {
        "json" => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .json()
                .with_writer(std::io::stderr)
                .init();
        },
        "full" => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_writer(std::io::stderr)
                .init();
        },
        _ => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .compact()
                .with_writer(std::io::stderr)
                .init();
        }
    }
}

pub fn init_from_env() {
    let level  = std::env::var("WAVE_LOG").unwrap_or_else(|_| "info".into());
    let format = std::env::var("WAVE_LOG_FORMAT").unwrap_or_else(|_| "compact".into());
    init(&level, &format);
}
