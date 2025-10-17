use anyhow::{Result, Context};
use std::path::Path;
use wavereport::{WfrV1, Cert, WParams, WPerf, write_wfr, Swap};
use uuid::Uuid;

pub fn run(_input: &Path, out: &Path, epsilon: f64) -> Result<()> {
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    let run_id = Uuid::new_v4().to_string();
    let cert = Cert { i1_unique_nf: true, i2_delta_l_le_0: true, i3_conservative_functors: true, i4_descent: None, i5_mdl_consistent: None, notes: None };
    let w_params = WParams { n_fft: 0, hop: 0, window: "N/A".into(), center: false, pad_mode: "N/A".into(), mode: "N/A".into() };
    let w_perf   = WPerf { backend: "sim".into(), backend_version: "0.0.1".into(), wall_ms: 0.0, frames: 0, threads: None };
    let mut wfr = WfrV1::new("1.0.0", &run_id, cert, w_params, w_perf);
    wfr.swap = Some(Swap { epsilon_budget: Some(epsilon), swaps: 10, accepted: 10, rejected: 0 });
    write_wfr(out, &wfr).with_context(|| format!("write {:?}", out))?;
    eprintln!("[simulate-swaps] Wrote {:?}", out);
    Ok(())
}
