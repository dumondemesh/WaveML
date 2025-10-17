use anyhow::{Result, Context, bail};
use std::path::Path;
use serde_json::Value;

pub fn run(wfr_path: &Path, _schema: &Path, require_pass: bool) -> Result<()> {
    let data: Value = serde_json::from_reader(std::fs::File::open(wfr_path)
        .with_context(|| format!("open {:?}", wfr_path))?)?;
    for key in ["schema_version","created_at","run_id","cert","w_params","w_perf","metrics"] {
        if data.get(key).is_none() {
            bail!("missing required field: {}", key);
        }
    }
    if require_pass {
        let cert = data.get("cert").and_then(|c| c.as_object()).context("cert object")?;
        for key in ["i1_unique_nf","i2_delta_l_le_0","i3_conservative_functors"] {
            if cert.get(key).and_then(|v| v.as_bool()) != Some(true) {
                bail!("certificate failed: {}", key);
            }
        }
    }
    eprintln!("[validate-wfr] OK: {:?}", wfr_path);
    Ok(())
}
