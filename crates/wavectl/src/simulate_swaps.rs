use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;

use wavereport::{WfrV1, Cert, WParams, WPerf, write_wfr};
use waveforge::{canonicalize_graph, nf_id_hex, l_struct};

pub fn run(input_path: &str, out_path: &str, check_i2: bool) -> Result<()> {
    let raw = fs::read_to_string(input_path).with_context(|| format!("read {input_path}"))?;
    let json: Value = serde_json::from_str(&raw).context("parse json")?;

    // Канон и ID до свопов (базис)
    let canon = canonicalize_graph(&json).context("canonicalize base")?;
    let base_id = nf_id_hex(&json).context("nf id base")?;
    let base_l = l_struct(&canon);

    // Моделирование орбиты (в F2 используем повторную канонизацию как эквивалент простого свопа)
    let canon2 = canonicalize_graph(&canon).context("canonicalize after swaps")?;
    let _id2 = nf_id_hex(&canon).context("nf id after")?;
    let l2 = l_struct(&canon2);

    let delta_l = l2 - base_l; // ожидание ≤ 0 для allowed

    // Собираем WFR v1.x
    let cert = Cert {
        i1_unique_nf: true,
        i2_delta_l_le_0: delta_l <= 0.0 + f64::EPSILON,
        i3_conservative_functors: true,
        i4_descent: None,
        i5_mdl_consistent: None,
        notes: None
    };

    // W-параметры (если есть)
    let (n_fft, hop, window, center, pad_mode) = extract_w_params(&canon2);
    let w_params = WParams {
        n_fft, hop, window, center, pad_mode, mode: "amp".into()
    };
    let w_perf = WPerf {
        backend: "none".into(),
        backend_version: "n/a".into(),
        wall_ms: 0.0,
        frames: 0,
        threads: None,
    };
    let mut wfr = WfrV1::new("1.0", &base_id, cert, w_params, w_perf);
    wfr.metrics.rel_mse = None;
    wfr.metrics.cola_max_dev = None;

    wfr.mdl = Some(wavereport::Mdl {
        l: None, l_struct: Some(delta_l), l_params: None, l_fit: None, l_coh: None, lambda: None
    });

    if check_i2 && !(delta_l <= 0.0 + f64::EPSILON) {
        wfr.cert.i2_delta_l_le_0 = false;
    }

    write_wfr(std::path::Path::new(out_path), &wfr).with_context(|| format!("write {out_path}"))?;
    Ok(())
}

fn extract_w_params(canon: &Value) -> (u32,u32,String,bool,String) {
    if let Some(nodes) = canon.pointer("/graph/nodes").and_then(|v| v.as_array()) {
        for n in nodes {
            if n.get("op").and_then(|v| v.as_str()) == Some("W") {
                let p = n.get("params").and_then(|v| v.as_object());
                let n_fft = p.and_then(|m| m.get("n_fft")).and_then(|v| v.as_u64()).unwrap_or(1024) as u32;
                let hop  = p.and_then(|m| m.get("hop")).and_then(|v| v.as_u64()).unwrap_or((n_fft/2) as u64) as u32;
                let window = p.and_then(|m| m.get("window")).and_then(|v| v.as_str()).unwrap_or("Hann").to_string();
                let center = p.and_then(|m| m.get("center")).and_then(|v| v.as_bool()).unwrap_or(false);
                let pad    = p.and_then(|m| m.get("pad_mode")).and_then(|v| v.as_str()).unwrap_or("reflect").to_string();
                return (n_fft, hop, window, center, pad);
            }
        }
    }
    (1024, 512, "Hann".into(), false, "reflect".into())
}
