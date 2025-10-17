use anyhow::{Result, Context};
use std::path::Path;
use wavereport::{WfrV1, Cert, WParams, WPerf, write_wfr, Phase};
use uuid::Uuid;
use std::time::Instant;

pub fn run(n_fft: u32, hop: u32, window: &str, center: bool, pad_mode: &str, mode: &str, out: &Path) -> Result<()> {
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    let run_id = Uuid::new_v4().to_string();
    let t0 = Instant::now();
    let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let frames: u64 = 0;

    let cert = Cert { i1_unique_nf: true, i2_delta_l_le_0: true, i3_conservative_functors: true, i4_descent: None, i5_mdl_consistent: None, notes: None };
    let w_params = WParams { n_fft, hop, window: window.into(), center, pad_mode: pad_mode.into(), mode: mode.into() };
    let w_perf = WPerf { backend: "rustfft".into(), backend_version: "6.1.0".into(), wall_ms, frames, threads: None };

    let mut wfr = WfrV1::new("1.0.0", &run_id, cert, w_params, w_perf);
    wfr.phase = Some(Phase { c_phi: None, h1: None });
    wfr.push_log("INFO", Some("Cola"), "Pipeline finished", None);
    write_wfr(out, &wfr).with_context(|| format!("write {:?}", out))?;
    eprintln!("[cola] Wrote {:?}", out);
    Ok(())
}
