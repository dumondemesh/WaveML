use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn read_list_file(path: &Path) -> Result<Vec<PathBuf>> {
    let s = fs::read_to_string(path)
        .with_context(|| format!("failed to read list file {:?}", path))?;
    let mut out = Vec::new();
    for (i, line) in s.lines().enumerate() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let p = PathBuf::from(t);
        if !p.exists() {
            eprintln!("[nf-batch] warning: line {} points to missing file: {}", i + 1, t);
        }
        out.push(p);
    }
    Ok(out)
}

fn parse_nf_id_from_stdout(s: &str) -> Option<String> {
    // ищем последнюю 64-символьную hex-строку или формату NF-ID=xxxx
    let mut cand: Option<String> = None;
    for tok in s.split_whitespace() {
        if let Some(rest) = tok.strip_prefix("NF-ID=") {
            if rest.len() == 64 && rest.chars().all(|c| c.is_ascii_hexdigit()) {
                cand = Some(rest.to_string());
            }
        }
        if tok.len() == 64 && tok.chars().all(|c| c.is_ascii_hexdigit()) {
            cand = Some(tok.to_string());
        }
    }
    cand
}

fn nf_id_for_input_by_forge(path: &Path) -> Result<String> {
    let exe = std::env::current_exe().context("locate current exe")?;
    let out = Command::new(exe)
        .arg("forge")
        .arg("--input")
        .arg(path)
        .arg("--print-id")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| format!("spawn forge for {:?}", path))?;

    if !out.status.success() {
        bail!("forge failed for {:?}", path);
    }
    let s = String::from_utf8_lossy(&out.stdout);
    parse_nf_id_from_stdout(&s)
        .ok_or_else(|| anyhow::anyhow!("cannot parse NF-ID from forge output for {:?}", path))
}

pub fn run(
    inputs: &[PathBuf],
    list: Option<&PathBuf>,
    json_out: bool,
    csv_out: bool,
    out_path: Option<&Path>,
) -> Result<()> {
    // collect inputs
    let mut all: Vec<PathBuf> = inputs.to_vec();
    if let Some(list_file) = list {
        let mut from_list = read_list_file(list_file)?;
        all.append(&mut from_list);
    }
    if all.is_empty() {
        bail!("no inputs provided (use --input and/or --list)");
    }

    let mut rows: Vec<(String, String)> = Vec::with_capacity(all.len());
    for p in &all {
        // Быстрая проверка, что это JSON
        let _v: Value = serde_json::from_slice(
            &fs::read(p).with_context(|| format!("read {:?}", p))?,
        )
            .with_context(|| format!("parse json {:?}", p))?;
        let id = nf_id_for_input_by_forge(p)?;
        rows.push((p.to_string_lossy().into_owned(), id));
    }

    if json_out {
        let items: Vec<Value> = rows
            .iter()
            .map(|(inp, id)| json!({ "input": inp, "nf_id": id }))
            .collect();
        let obj = json!({ "items": items });
        println!("{}", serde_json::to_string_pretty(&obj)?);
        return Ok(());
    }

    if csv_out {
        let mut w: Box<dyn Write> = if let Some(path) = out_path {
            Box::new(fs::File::create(path).with_context(|| format!("open {:?}", path))?)
        } else {
            Box::new(std::io::stdout())
        };
        writeln!(w, "input,nf_id")?;
        for (inp, id) in rows {
            let inp_q = format!("\"{}\"", inp.replace('\"', "\"\""));
            let id_q = format!("\"{}\"", id.replace('\"', "\"\""));
            writeln!(w, "{},{}", inp_q, id_q)?;
        }
        return Ok(());
    }

    // human readable (по умолчанию)
    for (inp, id) in rows {
        println!("{}  {}", id, inp);
    }
    Ok(())
}
