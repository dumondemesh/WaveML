use anyhow::{bail, Result};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

// Канонизация и NF-ID из waveforge
use waveforge::strict_nf::{strict_nf, strict_nf_hex};

fn read_json(path: &Path) -> Result<Value> {
    let s = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&s)?)
}

/// Рекурсивный diff двух JSON-деревьев.
/// Копим отличия как (json_pointer, left_str, right_str) с устойчивым порядком ключей.
fn collect_diffs(a: &Value, b: &Value, path: &str, out: &mut Vec<(String, String, String)>) {
    match (a, b) {
        (Value::Object(ao), Value::Object(bo)) => {
            let mut keys: Vec<&str> = ao.keys().chain(bo.keys()).map(|s| s.as_str()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let na = ao.get(k);
                let nb = bo.get(k);
                let p = format!("{}/{}", path, k);
                match (na, nb) {
                    (Some(va), Some(vb)) => collect_diffs(va, vb, &p, out),
                    (Some(va), None) => out.push((p, serde_json::to_string(va).unwrap(), "<absent>".into())),
                    (None, Some(vb)) => out.push((p, "<absent>".into(), serde_json::to_string(vb).unwrap())),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(aa), Value::Array(bb)) => {
            let max_len = aa.len().max(bb.len());
            for i in 0..max_len {
                let pa = aa.get(i);
                let pb = bb.get(i);
                let p = format!("{}/{}", path, i);
                match (pa, pb) {
                    (Some(va), Some(vb)) => collect_diffs(va, vb, &p, out),
                    (Some(va), None) => out.push((p, serde_json::to_string(va).unwrap(), "<absent>".into())),
                    (None, Some(vb)) => out.push((p, "<absent>".into(), serde_json::to_string(vb).unwrap())),
                    (None, None) => {}
                }
            }
        }
        _ => {
            if a != b {
                out.push((
                    path.to_string(),
                    serde_json::to_string(a).unwrap(),
                    serde_json::to_string(b).unwrap(),
                ));
            }
        }
    }
}

fn print_plain_diff(title: &str, diffs: &[(String, String, String)]) {
    if diffs.is_empty() {
        println!("[nf-diff] No differences in {}.", title);
        return;
    }
    println!("\n[nf-diff] Differences in {} ({}):", title, diffs.len());
    for (p, l, r) in diffs {
        println!("  - {}: {} -> {}", p, l, r);
    }
}

pub fn run(
    left: &Path,
    right: &Path,
    fail_on_diff: bool,
    json: bool,
    id_only: bool,
    show_source_diff: bool,
) -> Result<()> {
    let left_src = read_json(left)?;
    let right_src = read_json(right)?;

    let left_id = strict_nf_hex(&left_src)?;
    let right_id = strict_nf_hex(&right_src)?;

    if id_only {
        println!("{}", left_id);
        println!("{}", right_id);
        if fail_on_diff && left_id != right_id {
            bail!("NF-ID differ");
        }
        return Ok(());
    }

    println!("[nf-diff] left:  {:?}", left);
    println!("[nf-diff] right: {:?}", right);
    println!("[nf-diff] NF-ID(left)  = {}", left_id);
    println!("[nf-diff] NF-ID(right) = {}", right_id);

    if left_id == right_id {
        println!("[nf-diff] Canonical NF are identical.");
        return Ok(());
    }

    let left_nf = strict_nf(&left_src)?;
    let right_nf = strict_nf(&right_src)?;

    // Diff канонических NF
    let mut nf_diffs = Vec::<(String, String, String)>::new();
    collect_diffs(&left_nf, &right_nf, "/graph/nodes", &mut nf_diffs);

    if json {
        let arr: Vec<Value> = nf_diffs
            .iter()
            .map(|(p, l, r)| {
                let mut m = Map::new();
                m.insert("path".into(), Value::String(p.clone()));
                m.insert("left".into(), serde_json::from_str(l).unwrap_or(Value::String(l.clone())));
                m.insert("right".into(), serde_json::from_str(r).unwrap_or(Value::String(r.clone())));
                Value::Object(m)
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&Value::Array(arr))?);
    } else {
        print_plain_diff("canonical NF", &nf_diffs);
    }

    if show_source_diff {
        let mut src_diffs = Vec::<(String, String, String)>::new();
        collect_diffs(&left_src, &right_src, "", &mut src_diffs);
        if json {
            let arr: Vec<Value> = src_diffs
                .iter()
                .map(|(p, l, r)| {
                    let mut m = Map::new();
                    m.insert("path".into(), Value::String(p.clone()));
                    m.insert("left".into(), serde_json::from_str(l).unwrap_or(Value::String(l.clone())));
                    m.insert("right".into(), serde_json::from_str(r).unwrap_or(Value::String(r.clone())));
                    Value::Object(m)
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&Value::Array(arr))?);
        } else {
            print_plain_diff("source (pre-canonical)", &src_diffs);
        }
    }

    if fail_on_diff {
        bail!("NF-ID differ");
    }

    Ok(())
}
