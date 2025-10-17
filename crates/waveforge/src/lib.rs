use anyhow::{anyhow, Result};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

/// STRICT-NF v0:
/// - drop non-semantic node `id` from NF
/// - op synonyms (op/type/kind) -> "W" (или UPPERCASE для прочих)
/// - nfft|n -> n_fft; hop|hop_size|h -> hop; win|w -> window
/// - window names normalized: "hann"->"Hann", "rect"->"Rect"
/// - nodes sorted by (op, n_fft, hop, window)
/// - canonical JSON key ordering
pub fn strict_nf(manifest: &Value) -> Result<Value> {
    let g = manifest.get("graph")
        .ok_or_else(|| anyhow!("manifest has no 'graph'"))?;
    let nodes = g.get("nodes").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let mut nf_nodes: Vec<Value> = nodes.into_iter()
        .map(normalize_node)
        .collect::<Result<_>>()?;
    nf_nodes.sort_by(|a, b| {
        let ka = node_sort_key(a);
        let kb = node_sort_key(b);
        ka.cmp(&kb)
    });
    let mut gmap = Map::new();
    gmap.insert("nodes".to_string(), Value::Array(nf_nodes));
    let graph_nf = canonicalize(&Value::Object(gmap));
    Ok(json!({ "graph": graph_nf }))
}

fn node_sort_key(v: &Value) -> (String, u64, u64, String) {
    let op = v.get("op").and_then(|s| s.as_str()).unwrap_or("").to_string();
    let n_fft = v.get("n_fft").and_then(|u| u.as_u64()).unwrap_or(0);
    let hop = v.get("hop").and_then(|u| u.as_u64()).unwrap_or(0);
    let window = v.get("window").and_then(|s| s.as_str()).unwrap_or("").to_string();
    (op, n_fft, hop, window)
}

fn normalize_node(v: Value) -> Result<Value> {
    if !v.is_object() {
        return Err(anyhow!("graph node must be object"));
    }
    let mut op = None;
    let mut n_fft: Option<u64> = None;
    let mut hop: Option<u64> = None;
    let mut window: Option<String> = None;

    if let Some(m) = v.as_object() {
        for (k, val) in m {
            let k_l = k.to_lowercase();
            match k_l.as_str() {
                // `id` намеренно игнорируем в NF
                "op" | "type" | "kind" => {
                    if let Some(s) = val.as_str() { op = Some(map_op_name(s)); }
                }
                "n_fft" | "nfft" | "n" => {
                    if let Some(u) = as_u64(val) { n_fft = Some(u); }
                    else if let Some(s) = val.as_str() { if let Ok(u) = s.parse::<u64>() { n_fft = Some(u); } }
                }
                "hop" | "hop_size" | "h" => {
                    if let Some(u) = as_u64(val) { hop = Some(u); }
                    else if let Some(s) = val.as_str() { if let Ok(u) = s.parse::<u64>() { hop = Some(u); } }
                }
                "window" | "win" | "w" => {
                    if let Some(s) = val.as_str() { window = Some(map_window_name(s)); }
                }
                _ => { /* drop unknowns for v0 */ }
            }
        }
    }
    let op_final = op.unwrap_or_else(|| "W".to_string());
    let mut out = Map::new();
    out.insert("op".to_string(), Value::String(op_final));
    if let Some(n) = n_fft { out.insert("n_fft".to_string(), Value::Number(n.into())); }
    if let Some(h) = hop { out.insert("hop".to_string(), Value::Number(h.into())); }
    if let Some(w) = window { out.insert("window".to_string(), Value::String(w)); }
    Ok(Value::Object(out))
}

fn as_u64(v: &Value) -> Option<u64> {
    if let Some(u) = v.as_u64() { return Some(u); }
    if let Some(i) = v.as_i64() { if i >= 0 { return Some(i as u64); } }
    None
}

fn map_op_name(s: &str) -> String {
    match s.trim().to_lowercase().as_str() {
        "w" | "window" | "wave" | "waveop" => "W".to_string(),
        "t" => "T".to_string(),
        "d" => "D".to_string(),
        "a" => "A".to_string(),
        "c" => "C".to_string(),
        "phi" | "φ" | "phiop" => "Phi".to_string(),
        other => other.to_string().to_uppercase(),
    }
}

fn map_window_name(s: &str) -> String {
    match s.trim().to_lowercase().as_str() {
        "hann" | "hanning" => "Hann".to_string(),
        "rect" | "boxcar" | "rectangular" => "Rect".to_string(),
        other => {
            let mut chrs = other.chars();
            match chrs.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chrs.as_str(),
                None => String::new(),
            }
        }
    }
}

/// Canonicalize JSON:
fn canonicalize(v: &Value) -> Value {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => v.clone(),
        Value::Array(arr) => Value::Array(arr.iter().map(canonicalize).collect()),
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys { out.insert(k.clone(), canonicalize(&map[k])); }
            Value::Object(out)
        }
    }
}

/// Stable SHA-256 hex от канонического `graph`
pub fn strict_nf_hex(manifest: &Value) -> Result<String> {
    let nf = strict_nf(manifest)?;
    let graph = nf.get("graph").unwrap(); // safe
    let bytes = serde_json::to_vec(graph).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}
