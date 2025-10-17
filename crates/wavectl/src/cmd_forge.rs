
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Forge STRICT-NF and NF-ID.
/// Keeps CLI behavior:
///   --input <FILE>
///   --print-id
///   --print-nf
///   --out <FILE> (optional)
pub fn run(input: &Path, print_id: bool, print_nf: bool, out: Option<&Path>) -> Result<()> {
    let file = File::open(input).with_context(|| format!("open {:?}", input))?;
    let manifest: Value = serde_json::from_reader(file).context("parse manifest json")?;

    // Delegate to waveforge canonicalizer that includes center/pad_mode
    let nf = waveforge::strict_nf(&manifest).context("strict_nf")?;
    let nf_hex = waveforge::strict_nf_hex(&manifest).context("strict_nf_hex")?;

    if print_id {
        println!("{}", nf_hex);
        println!("[forge] NF-ID={}", nf_hex);
    }

    if print_nf {
        println!("{}", serde_json::to_string_pretty(&nf).unwrap_or_else(|_| "{}".to_string()));
    }

    if let Some(out_path) = out {
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let mut f = File::create(out_path).with_context(|| format!("create {:?}", out_path))?;
        write!(f, "{}", serde_json::to_string_pretty(&nf).unwrap_or_else(|_| "{}".to_string()))?;
        println!("[forge] Wrote {:?}", out_path);
    }

    Ok(())
}
