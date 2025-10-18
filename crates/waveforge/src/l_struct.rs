use serde_json::Value;

/// Публичный API: детерминированная структурная метрика канона
pub fn l_struct(manifest: &Value) -> f64 {
    let g = match manifest.get("graph") {
        Some(v) => v,
        None => return 0.0,
    };

    // Избегаем заимствования временных значений: работаем через итераторы
    let mut acc = 0.0_f64;

    if let Some(nodes) = g.get("nodes").and_then(|n| n.as_array()) {
        for n in nodes.iter() {
            let op = n.get("op").and_then(|v| v.as_str()).unwrap_or("");
            let p  = n.get("params").and_then(|v| v.as_object());
            let cost = match op {
                "W" => cost_w(p),
                "T" => 0.5,
                "D" => 0.75, // уточним в F3
                "A" => 0.25,
                "C" => 0.5,
                "Align" => 0.3,
                "Φ" | "Phi" => 0.6,
                _ => 0.4,
            };
            acc += cost;
        }
    }

    let edges_len = g.get("edges")
        .and_then(|n| n.as_array())
        .map(|v| v.len())
        .unwrap_or(0);

    let lambda_topo = 0.01_f64;
    acc + lambda_topo * (edges_len as f64)
}

fn cost_w(p: Option<&serde_json::Map<String, Value>>) -> f64 {
    let w_op = 1.0_f64;
    let mut w_fft = 1.0_f64;
    let mut w_hop = 0.0_f64;
    let mut w_win = 0.0_f64;
    let mut w_center = 0.0_f64;
    let mut w_pad = 0.0_f64;

    if let Some(pm) = p {
        let n_fft = pm.get("n_fft").and_then(|v| v.as_u64()).unwrap_or(1024) as f64;
        let hop  = pm.get("hop").and_then(|v| v.as_u64()).unwrap_or((n_fft as u64 / 2) as u64) as f64;
        w_fft = (n_fft.log2()).max(1.0);
        w_hop = ((n_fft / hop).ln()).max(0.0) * 0.25;
        let window = pm.get("window").and_then(|v| v.as_str()).unwrap_or("Hann");
        w_win = match window {
            "Hann" => 0.0,
            "Hamming" => 0.05,
            "Blackman" => 0.075,
            "Rect" => 0.125,
            _ => 0.1,
        };
        let center = pm.get("center").and_then(|v| v.as_bool()).unwrap_or(false);
        if center { w_center = 0.02; }
        let pad = pm.get("pad_mode").and_then(|v| v.as_str()).unwrap_or("reflect");
        w_pad = match pad {
            "reflect" => 0.0,
            "toeplitz" => 0.01,
            _ => 0.05,
        };
    }
    w_op + w_fft + w_hop + w_win + w_center + w_pad
}
