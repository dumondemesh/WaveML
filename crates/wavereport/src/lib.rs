use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use wmlb::Graph;

/// Acceptance certificate (legacy+new flags)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    // legacy I1..I5 kept for compatibility
    pub i1: bool,
    pub i2: bool,
    pub i3: bool,
    pub i4: bool,
    pub i5: bool,
    // new (extensible)
    #[serde(default)]
    pub r7_ok: bool,
    #[serde(default)]
    pub r8_ok: bool,
    #[serde(default)]
    pub phi_ok: bool,
    #[serde(default)]
    pub mdl_ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceInfo {
    pub graph_nodes: usize,
    pub edges_seen: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_sha: Option<String>,
}

/// Performance section for W
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OpPerf {
    pub backend: String,        // "rustfft"
    pub wall_ms: f64,           // wall-clock (optional; 0.0 if unknown)
    pub frames: u64,
    pub n_fft: u64,
    pub hop: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rustfft_ver: Option<String>,
}

/// W section of the report: params + perf + metrics
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WSection {
    pub params: Value,          // {bank,n_fft,hop,window,center,pad_mode}
    pub perf: OpPerf,
    pub metrics: Value,         // {mse,snr_db,cola_max_dev}
}

/// WaveReport root
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub certificate: Certificate,
    pub ops: Value,
    pub source: SourceInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub W: Option<WSection>,
}

pub fn save_report_json(rep: &Report, path: &Path) -> Result<()> {
    let s = serde_json::to_string_pretty(rep)?;
    if let Some(pdir) = path.parent() {
        fs::create_dir_all(pdir)?;
    }
    fs::write(path, s)?;
    Ok(())
}

/// Build minimal ops/source summary and (optionally) W.params from IR.
/// `wall_ms` unknown here — fill later in CLI or keep 0.0.
pub fn make_basic_report(graph: &Graph, cert: Certificate) -> Report {
    // ops summary: count ops and edges seen on W (reflect only allowed)
    let mut counts = serde_json::Map::new();
    let mut edges_seen: Vec<String> = Vec::new();

    for n in &graph.nodes {
        let op = n.op.clone();
        *counts.entry(op.clone()).or_insert(json!(0)) =
            json!(counts.get(&op).and_then(|v| v.as_u64()).unwrap_or(0) + 1);

        if n.op == "W" {
            if let Some(e) = n.params.get("pad_mode").or_else(|| n.params.get("edge")) {
                if let Some(es) = e.as_str() {
                    edges_seen.push(es.to_string());
                }
            }
        }
    }

    let ops_json = json!({
        "counts": counts,
        "edges_seen": edges_seen,
    });

    // Try to extract W.params from the first W node
    let w_params = graph.nodes.iter().find(|n| n.op == "W")
        .map(|w| w_params_from_ir(&w.params));

    let w_section = w_params.map(|params| {
        // Estimate frames (structural)
        let n_fft = params.get("n_fft").and_then(|v| v.as_u64()).unwrap_or(1024);
        let hop   = params.get("hop").and_then(|v| v.as_u64()).unwrap_or(n_fft/2);
        let perf = OpPerf {
            backend: "rustfft".to_string(),
            wall_ms: 0.0, // unknown here; CLI may fill later
            frames: 0,
            n_fft,
            hop,
            threads: None,
            rustfft_ver: Some(env!("CARGO_PKG_VERSION").to_string()),
        };
        let metrics = json!({});
        WSection { params, perf, metrics }
    });

    let source = SourceInfo {
        graph_nodes: graph.nodes.len(),
        edges_seen,
        git_sha: std::env::var("GITHUB_SHA").ok(),
    };

    Report { certificate: cert, ops: ops_json, source, W: w_section }
}

/// Adapter: W params from IR JSON ({bank,n_fft,hop,window,center,pad_mode})
pub fn w_params_from_ir(ir_params: &serde_json::Value) -> serde_json::Value {
    let bank   = ir_params.get("bank").and_then(|v| v.as_str()).unwrap_or("hann-default");
    let n_fft  = ir_params.get("n_fft").and_then(|v| v.as_u64()).unwrap_or(1024) as u64;
    let hop    = ir_params.get("hop").and_then(|v| v.as_u64()).unwrap_or(n_fft/2) as u64;
    let window = ir_params.get("window").and_then(|v| v.as_str()).unwrap_or("hann");
    let center = ir_params.get("center").and_then(|v| v.as_bool()).unwrap_or(true);
    let edge   = ir_params.get("pad_mode")
        .or_else(|| ir_params.get("edge"))
        .and_then(|v| v.as_str())
        .unwrap_or("reflect");
    json!({
        "bank": bank, "n_fft": n_fft, "hop": hop,
        "window": window, "center": center, "pad_mode": edge
    })
}
