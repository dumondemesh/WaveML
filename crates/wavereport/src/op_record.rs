//! Recording operator parameters into WaveReport (.wfr.json).
use serde_json::json;
use crate::OpPerf;

// Если доступен общий тип WParams из waverunner:
#[allow(unused_imports)]
use waverunner::ops::w::{WParams, WindowKind, PadMode};

/// Универсальный адаптер: можно кормить либо готовый WParams,
/// либо «сырые» serde_json::Value (как в IR), чтобы не тянуть типы.
pub fn w_params_from_struct(p: &WParams) -> serde_json::Value {
    let window = match p.window { WindowKind::Hann=>"hann", WindowKind::Hamming=>"hamming", WindowKind::Blackman=>"blackman" };
    let pad = match p.pad_mode { PadMode::Reflect => "reflect" };
    json!({
        "bank": p.bank, "n_fft": p.n_fft, "hop": p.hop,
        "window": window, "center": p.center, "pad_mode": pad
    })
}

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

pub fn make_perf(
    backend: &str,
    wall_ms: f64,
    frames: u64,
    n_fft: u64,
    hop: u64,
    threads: Option<u32>,
    rustfft_ver: Option<String>
) -> OpPerf {
    OpPerf {
        backend: backend.to_string(),
        wall_ms, frames, n_fft, hop, threads, rustfft_ver
    }
}
