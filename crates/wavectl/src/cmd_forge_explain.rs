use anyhow::Result;
use serde_json::{to_string_pretty, Value};
use std::{fs, path::Path};
use waveforge::strict_nf::{strict_nf, strict_nf_hex};

fn get_u(n: &Value, key: &str) -> Option<u64> {
    n.get(key)?.as_u64()
}
fn get_b(n: &Value, key: &str) -> Option<bool> {
    n.get(key)?.as_bool()
}
fn get_s<'a>(n: &'a Value, key: &str) -> Option<&'a str> {
    n.get(key)?.as_str()
}


fn print_num(name: &str, a: Option<u64>, b: Option<u64>) {
    match (a, b) {
        (Some(x), Some(y)) if x == y => println!("    - {name}: {x} (kept)"),
        (Some(x), Some(y)) => println!("    - {name}: {x} -> {y}"),
        (None, Some(y)) => println!("    - {name}: <inferred> -> {y}"),
        (Some(x), None) => println!("    - {name}: {x} -> <removed>"),
        (None, None) => {}
    }
}
fn print_bool(name: &str, a: Option<bool>, b: Option<bool>) {
    match (a, b) {
        (Some(x), Some(y)) if x == y => println!("    - {name}: {x} (kept)"),
        (Some(x), Some(y)) => println!("    - {name}: {x} -> {y}"),
        (None, Some(y)) => println!("    - {name}: <inferred> -> {y}"),
        (Some(x), None) => println!("    - {name}: {x} -> <removed>"),
        (None, None) => {}
    }
}
fn print_str(name: &str, a: Option<&str>, b: Option<&str>) {
    match (a, b) {
        (Some(x), Some(y)) if x == y => println!("    - {name}: {x} (kept)"),
        (Some(x), Some(y)) => println!("    - {name}: {x} -> {y}"),
        (None, Some(y)) => println!("    - {name}: <inferred> -> {y}"),
        (Some(x), None) => println!("    - {name}: {x} -> <removed>"),
        (None, None) => {}
    }
}

pub fn run(input: &Path) -> Result<()> {
    // 1) читаем исходный граф
    let raw = fs::read_to_string(input)?;
    let src: Value = serde_json::from_str(&raw)?;

    // 2) считаем каноническую NF и NF-ID
    let nf = strict_nf(&src)?;
    let nf_id = strict_nf_hex(&src)?;

    println!("[forge-explain] Input:   {:?}", input);
    println!("[forge-explain] NF-ID:   {nf_id}");

    // 3) сравниваем узлы src vs nf (только по полям, которые нас интересуют)
    let src_nodes: Vec<Value> = src
        .pointer("/graph/nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let nf_nodes: Vec<Value> = nf
        .pointer("/graph/nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    println!(
        "[forge-explain] Nodes: src={} -> nf={}",
        src_nodes.len(),
        nf_nodes.len()
    );

    let count = src_nodes.len().min(nf_nodes.len());
    for i in 0..count {
        let a = &src_nodes[i];
        let b = &nf_nodes[i];

        let op = get_s(b, "op").unwrap_or("?");
        println!("  [NF node #{i}] op={op}");
        print_num("n_fft", get_u(a, "n_fft"), get_u(b, "n_fft"));
        print_num("hop", get_u(a, "hop"), get_u(b, "hop"));
        print_str("window", get_s(a, "window"), get_s(b, "window"));
        print_bool("center", get_b(a, "center"), get_b(b, "center"));
        print_str("pad_mode", get_s(a, "pad_mode"), get_s(b, "pad_mode"));
    }

    // 4) печатаем каноническую NF
    println!("\n[forge-explain] Canonical NF:");
    println!("{}", to_string_pretty(&nf)?);

    Ok(())
}
