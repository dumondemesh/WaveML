use anyhow::{bail, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

// Берём реальные API из waveforge
use waveforge::strict_nf::{strict_nf, strict_nf_hex};

fn read_json(path: &Path) -> Result<Value> {
    let s = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&s)?)
}

pub fn run(
    input: &Path,
    print_id: bool,
    print_nf: bool,
    out: Option<&Path>,
    check: bool,
) -> Result<()> {
    let src = read_json(input)?;
    let nf = strict_nf(&src)?;

    if print_id {
        let id = strict_nf_hex(&src)?;
        // печатаем «сырой» ID (для пайпов) и удобную строку, как в примерах
        println!("{}", id);
        println!("[forge] NF-ID={}", id);
    }

    if check {
        // true, если файл уже в канонической форме
        if src == nf {
            println!("[forge] Input is already canonical.");
        } else {
            bail!("input is not canonical (differs from canonical NF)");
        }
    }

    if print_nf {
        println!("{}", serde_json::to_string_pretty(&nf)?);
    }

    if let Some(path) = out {
        fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))?;
        fs::write(path, serde_json::to_string_pretty(&nf)?)?;
        println!("[forge] Wrote {:?}", path);
    }

    Ok(())
}
