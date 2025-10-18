use std::path::Path;

use anyhow::Result;
use wavereport::{WaveReport, WParams, WPerf};

pub fn run(
    n_fft: u32,
    hop: u32,
    window: &str,
    center: bool,
    pad_mode: &str,
    mode: &str,
    out: &Path,
) -> Result<()> {
    // clippy: минимизируем булевы выражения
    let w = window.to_ascii_lowercase();
    let _pass = (w == "hann" || w == "hamming" || w == "blackman") && hop * 2 == n_fft;

    // clippy: не делаем field-reassign после Default::default()
    let rep = WaveReport {
        w_params: Some(WParams {
            n_fft,
            hop,
            window: window.to_string(),
            mode: mode.to_string(),
            center,
            pad_mode: pad_mode.to_string(),
        }),
        w_perf: Some(WPerf {
            backend: "cola".into(),
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
    println!("[cola] Wrote {:?}", out);
    Ok(())
}
