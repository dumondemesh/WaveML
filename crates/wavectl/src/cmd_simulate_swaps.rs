use std::path::Path;

use anyhow::Result;
use wavereport::{WaveReport, WParams, WPerf};

pub fn run(input: &Path, out: &Path) -> Result<()> {
    // input сейчас не используется — но оставляем сигнатуру для совместимости
    let _ = input;

    let rep = WaveReport {
        w_params: Some(WParams {
            n_fft: 1024,
            hop: 512,
            window: "Hann".into(),
            mode: "amp".into(),
            center: false,
            pad_mode: "reflect".into(),
        }),
        w_perf: Some(WPerf {
            backend: "sim".into(),
            backend_version: "0.0.1".into(),
            wall_ms: 0.0,
            frames: 0,
            threads: Some(0),
        }),
        ..Default::default()
    };

    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    let s = serde_json::to_string_pretty(&rep)?;
    std::fs::write(out, s)?;
    println!("[simulate-swaps] Wrote {:?}", out);
    Ok(())
}
