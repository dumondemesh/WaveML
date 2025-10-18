use serde::{Serialize, Deserialize};
use anyhow::Context;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WParams {
    pub n_fft: u32,
    pub hop: u32,
    pub window: String,
    pub center: bool,
    pub pad_mode: String,
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WPerf {
    pub backend: String,
    pub backend_version: String,
    pub wall_ms: f64,
    pub frames: u64,
    pub threads: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Cert {
    pub i1_unique_nf: bool,
    pub i2_delta_l_le_0: bool,
    pub i3_conservative_functors: bool,
    pub i4_descent: Option<bool>,
    pub i5_mdl_consistent: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Metrics {
    pub mse: Option<f64>,
    pub rel_mse: Option<f64>,
    pub snr_db: Option<f64>,
    pub cola_max_dev: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Mdl {
    #[serde(rename="L")] pub l: Option<f64>,
    #[serde(rename="L_struct")] pub l_struct: Option<f64>,
    #[serde(rename="L_params")] pub l_params: Option<f64>,
    #[serde(rename="L_fit")] pub l_fit: Option<f64>,
    #[serde(rename="L_coh")] pub l_coh: Option<f64>,
    pub lambda: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Phase {
    pub c_phi: Option<f64>,
    pub h1: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Swap {
    pub epsilon_budget: Option<f64>,
    pub swaps: u32,
    pub accepted: u32,
    pub rejected: u32,
}

/// Back-compat тип, который импортирует ваш wavectl: `wavereport::WaveReport`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaveReport {
    pub schema_version: String,
    pub created_at: String,
    pub run_id: String,
    pub cert: Cert,
    pub mdl: Option<Mdl>,
    pub phase: Option<Phase>,
    pub swap: Option<Swap>,
    pub w_params: Option<WParams>,
    pub w_perf: Option<WPerf>,
    pub metrics: Metrics,
}

impl Default for WaveReport {
    fn default() -> Self {
        Self {
            schema_version: "1.0".into(),
            created_at: "".into(),
            run_id: "".into(),
            cert: Cert::default(),
            mdl: None,
            phase: None,
            swap: None,
            w_params: None,
            w_perf: None,
            metrics: Metrics::default(),
        }
    }
}

impl WaveReport {
    pub fn new(schema_version: &str, run_id: &str, cert: Cert, w_params: WParams, w_perf: WPerf) -> Self {
        let created_at = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        Self {
            schema_version: schema_version.to_owned(),
            created_at,
            run_id: run_id.to_owned(),
            cert,
            mdl: None,
            phase: None,
            swap: None,
            w_params: Some(w_params),
            w_perf: Some(w_perf),
            metrics: Metrics::default(),
        }
    }
}

pub fn write_wfr(path: &Path, wfr: &WaveReport) -> anyhow::Result<()> {
    let mut f = File::create(path).with_context(|| format!("create {:?}", path))?;
    let buf = serde_json::to_string_pretty(wfr).context("serialize WaveReport")?;
    f.write_all(buf.as_bytes()).context("write WaveReport")?;
    Ok(())
}
