
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MDL {
    pub l: Option<f64>,
    pub l_struct: Option<f64>,
    pub l_params: Option<f64>,
    pub l_fit: Option<f64>,
    pub l_coh: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WParams {
    pub n_fft: u32,
    pub hop: u32,
    pub window: String,
    pub mode: String,
    pub center: bool,
    pub pad_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WPerf {
    pub backend: String,
    pub backend_version: String,
    pub wall_ms: f64,
    pub frames: u32,
    pub threads: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metrics {
    pub cola_rel_dev: f64,
    pub cola_mean: f64,
    pub cola_tol: f64,
    pub cola_pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WaveReport {
    pub module: String,
    pub mdl: MDL,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_params: Option<WParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_perf: Option<WPerf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
}

impl WaveReport {
    pub fn new<S: AsRef<str>>(module: S) -> Self {
        Self {
            module: module.as_ref().to_string(),
            mdl: MDL::default(),
            w_params: None,
            w_perf: None,
            metrics: None,
        }
    }
}
