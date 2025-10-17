// crates/waveforge/src/strict_nf.rs
use anyhow::{anyhow, bail, Context, Result};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Построить канонический NF JSON.
pub fn strict_nf(input: &Value) -> Result<Value> {
    let graph = input
        .get("graph")
        .ok_or_else(|| anyhow!("missing `graph`"))?;

    let nodes = graph
        .get("nodes")
        .ok_or_else(|| anyhow!("missing `graph.nodes`"))?
        .as_array()
        .ok_or_else(|| anyhow!("`graph.nodes` must be an array"))?;

    let mut out_nodes: Vec<Value> = Vec::with_capacity(nodes.len());
    for (idx, n) in nodes.iter().enumerate() {
        out_nodes.push(normalize_node(n).with_context(|| format!("node[{idx}]"))?);
    }

    // Стабильная детерминированная сортировка
    out_nodes.sort_by_key(node_sort_key);

    let mut g = Map::new();
    g.insert("nodes".to_string(), Value::Array(out_nodes));

    let mut root = Map::new();
    root.insert("graph".to_string(), Value::Object(g));
    Ok(Value::Object(root))
}

/// Вернуть hex SHA-256 канонического NF.
pub fn strict_nf_hex(input: &Value) -> Result<String> {
    let canon = strict_nf(input)?;
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_vec(&canon)?);
    Ok(hex::encode(hasher.finalize()))
}

// -------------------- internals --------------------

fn normalize_node(node: &Value) -> Result<Value> {
    let obj = node
        .as_object()
        .ok_or_else(|| anyhow!("node must be an object"))?;

    // op (много синонимов → "W"; legacy допускает отсутствие)
    let op_in = obj.get("op").and_then(Value::as_str).unwrap_or("w");
    let op = normalize_op(op_in)?;

    // n_fft
    let n_fft = must_u32_from_keys(
        obj,
        &["n_fft", "nfft", "fft", "N", "n", "win_len", "window_size", "frame_len"],
        "n_fft",
    )?;

    // hop: сначала явные ключи, затем ratio/overlap, иначе дефолт n_fft/2
    let hop = get_hop(obj, n_fft)?;

    // window
    let win_raw = must_str_from_keys(obj, &["window", "win", "w", "window_fn"], "window")?;
    let window = normalize_window(win_raw)?;

    // center
    let center = get_center(obj)?.unwrap_or(false);

    // pad_mode (по умолчанию от center)
    let pad_in = find_first_str(obj, &["pad_mode", "pad", "padding", "padmode"])
        .unwrap_or(if center { "reflect" } else { "toeplitz" });
    let pad_norm = normalize_pad(pad_in)?;

    // Схлопывание (center, pad_mode)
    let (center_canon, pad_canon) = canonicalize_center_pad(center, &pad_norm);

    // Итоговый узел с отсортированными ключами
    let mut m = Map::new();
    m.insert("op".into(), Value::String(op));
    m.insert("n_fft".into(), Value::Number(n_fft.into()));
    m.insert("hop".into(), Value::Number(hop.into()));
    m.insert("window".into(), Value::String(window));
    m.insert("center".into(), Value::Bool(center_canon));
    m.insert("pad_mode".into(), Value::String(pad_canon));
    Ok(Value::Object(m))
}

fn node_sort_key(v: &Value) -> String {
    let o = v.get("op").and_then(Value::as_str).unwrap_or("");
    let n = v.get("n_fft").and_then(Value::as_u64).unwrap_or(0);
    let h = v.get("hop").and_then(Value::as_u64).unwrap_or(0);
    let w = v.get("window").and_then(Value::as_str).unwrap_or("");
    let c = v.get("center").and_then(Value::as_bool).unwrap_or(false);
    let p = v.get("pad_mode").and_then(Value::as_str).unwrap_or("");
    format!("{o}:{n}:{h}:{w}:{c}:{p}")
}

// ---------- helpers: извлечение и парсинг ----------

fn must_u32_from_keys(obj: &Map<String, Value>, keys: &[&str], field_name: &str) -> Result<u32> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            if let Some(n) = parse_u32(v) {
                return Ok(n);
            }
        }
    }
    bail!("missing field `{field_name}`")
}

fn must_str_from_keys<'a>(
    obj: &'a Map<String, Value>,
    keys: &[&str],
    field_name: &str,
) -> Result<&'a str> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            if let Some(s) = v.as_str() {
                return Ok(s);
            }
        }
    }
    bail!("missing field `{field_name}`")
}

fn find_first_str<'a>(obj: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            if let Some(s) = v.as_str() {
                return Some(s);
            }
        }
    }
    None
}

fn parse_u32(v: &Value) -> Option<u32> {
    match v {
        Value::Number(n) => n.as_u64().and_then(|x| u32::try_from(x).ok()),
        Value::String(s) => s.trim().parse::<u64>().ok().and_then(|x| u32::try_from(x).ok()),
        _ => None,
    }
}

fn parse_ratio(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => {
            let s = s.trim();
            if let Some(body) = s.strip_suffix('%') {
                body.trim().parse::<f64>().ok().map(|p| p / 100.0)
            } else {
                s.parse::<f64>().ok()
            }
        }
        _ => None,
    }
}

fn find_first_number_like(obj: &Map<String, Value>, keys: &[&str]) -> Option<u32> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            if let Some(n) = parse_u32(v) {
                return Some(n);
            }
        }
    }
    None
}

fn find_first_ratio(obj: &Map<String, Value>, keys: &[&str]) -> Option<f64> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            if let Some(r) = parse_ratio(v) {
                return Some(r);
            }
        }
    }
    None
}

fn get_hop(obj: &Map<String, Value>, n_fft: u32) -> Result<u32> {
    // 1) Явные числовые ключи
    if let Some(n) = find_first_number_like(
        obj,
        &["hop", "hop_length", "stride", "step", "H", "h", "win_shift", "frame_shift"],
    ) {
        return Ok(n);
    }

    // 2) Доля окна (hop_ratio/stride_ratio)
    if let Some(r) = find_first_ratio(obj, &["hop_ratio", "stride_ratio", "r"]) {
        let x = (r * n_fft as f64).round().clamp(1.0, n_fft as f64);
        return Ok(x as u32);
    }

    // 3) Перекрытие (overlap → hop = (1 - overlap) * n_fft)
    if let Some(ov) = find_first_ratio(
        obj,
        &["overlap", "overlap_ratio", "overlap_pct", "overlap_percent", "ovlp"],
    ) {
        let r = ov.clamp(0.0, 1.0);
        let x = ((1.0 - r) * n_fft as f64).round().clamp(1.0, n_fft as f64);
        return Ok(x as u32);
    }

    // 4) Дефолт: половина окна
    Ok(n_fft / 2)
}

// ---------- нормализация значений ----------

fn get_center(obj: &Map<String, Value>) -> Result<Option<bool>> {
    for k in ["center", "centred", "centered"] {
        if let Some(v) = obj.get(k) {
            return Ok(Some(match v {
                Value::Bool(b) => *b,
                Value::Number(n) => n.as_u64().map(|u| u != 0).unwrap_or(false),
                Value::String(s) => parse_bool_like(s)?,
                _ => bail!("`{k}` must be bool/number/string"),
            }));
        }
    }
    Ok(None)
}

fn parse_bool_like(s: &str) -> Result<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Ok(true),
        "false" | "0" | "no" | "n" | "off" => Ok(false),
        other => bail!("invalid boolean string `{other}`"),
    }
}

fn normalize_op(s: &str) -> Result<String> {
    let o = s.trim().to_ascii_lowercase();
    let is_w = matches!(
        o.as_str(),
        "w" | "w-op" | "w_op" | "wml_w" | "wml" | "wtransform" | "w-transform" | "wave-op"
            | "wop" | "" | "w-graph"
    );
    if is_w {
        Ok("W".to_string())
    } else {
        bail!("unsupported op `{s}`")
    }
}

fn normalize_window(s: &str) -> Result<String> {
    let w = s.trim().to_ascii_lowercase();
    if matches!(w.as_str(), "hann" | "hanning") {
        Ok("Hann".to_string())
    } else if w == "hamming" {
        Ok("Hamming".to_string())
    } else if matches!(
        w.as_str(),
        "blackman" | "blackman62" | "blackman-harris" | "blackmanharris"
    ) {
        Ok("Blackman".to_string())
    } else {
        bail!("unsupported window `{s}`")
    }
}

fn normalize_pad(s: &str) -> Result<String> {
    let p = s.trim().to_ascii_lowercase();
    // reflect-like
    if matches!(p.as_str(), "reflect" | "symmetric" | "mirror" | "sym" | "mirrored") {
        Ok("reflect".to_string())
    // toeplitz-like
    } else if matches!(p.as_str(), "toeplitz" | "conv" | "convolution" | "valid-conv") {
        Ok("toeplitz".to_string())
    } else {
        bail!("unsupported pad_mode `{s}`")
    }
}

/// Схлопываем только одну эквивалентность: (center=true, reflect) → (center=false, toeplitz).
fn canonicalize_center_pad(center: bool, pad_mode: &str) -> (bool, String) {
    if center && pad_mode == "reflect" {
        (false, "toeplitz".to_string())
    } else {
        (center, pad_mode.to_string())
    }
}
