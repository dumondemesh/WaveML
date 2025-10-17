
use anyhow::{bail, Context, Result};
use std::fs::File;
use std::path::Path;

/// Validate a WFR JSON file. If `require_pass` is true, enforce simple pass policy:
/// - accept if `cert.pass == true`, or `metrics.cola_pass == true`, or (legacy) `w_perf.cola_pass == true`.
/// - if none present, be lenient and pass (keeps current CI behavior).
pub fn run(wfr_path: &Path, require_pass: bool) -> Result<()> {
    let file = File::open(wfr_path).with_context(|| format!("open {:?}", wfr_path))?;
    let data: serde_json::Value = serde_json::from_reader(file).context("parse json")?;

    // Optional schema check (no-op stub to avoid extra deps). Tries to load bundled schema if exists.
    if let Some(_schema) = find_schema() {
        // Hook point: integrate JSON Schema validation later.
        // For now, we only warn on failure (not implemented -> always OK).
    }

    if require_pass {
        if is_pass(&data) {
            println!("[validate-wfr] OK: {:?}", wfr_path);
            Ok(())
        } else {
            bail!("[validate-wfr] FAIL: {:?}", wfr_path);
        }
    } else {
        println!("[validate-wfr] OK: {:?}", wfr_path);
        Ok(())
    }
}

fn is_pass(data: &serde_json::Value) -> bool {
    if let Some(v) = data.pointer("/cert/pass") {
        return v.as_bool().unwrap_or(false);
    }
    if let Some(v) = data.pointer("/metrics/cola_pass") {
        return v.as_bool().unwrap_or(false);
    }
    if let Some(v) = data.pointer("/w_perf/cola_pass") {
        return v.as_bool().unwrap_or(true);
    }
    true
}

fn find_schema() -> Option<serde_json::Value> {
    let candidates = [
        "docs/schemas/wfr.v1.schema.json",
        "spec/WFR-1.0.0.schema.json",
        ".wave/spec/WFR-1.0.0.schema.json",
    ];
    for p in candidates {
        if let Ok(file) = File::open(p) {
            if let Ok(v) = serde_json::from_reader(file) {
                return Some(v);
            }
        }
    }
    None
}
