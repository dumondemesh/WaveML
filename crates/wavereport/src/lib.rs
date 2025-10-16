use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use wmlb::Graph;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certificate {
    // прежние индикаторы (для обратной совместимости)
    pub i1: bool,
    pub i2: bool,
    pub i3: bool,
    pub i4: bool,
    pub i5: bool,
    // новые:
    pub r7_ok: bool,
    pub r8_ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceInfo {
    pub git_sha: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub certificate: Certificate,
    pub ops: Value,
    pub source: SourceInfo,
}

pub fn from_ir(g: &Graph) -> Report {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut edges_seen: Vec<String> = Vec::new();
    let mut lambdas: Vec<f64> = Vec::new();
    let mut aa_missing = 0usize;

    for n in &g.nodes {
        *counts.entry(n.op.clone()).or_insert(0) += 1;

        if n.op == "W" {
            if let Some(e) = n.params.get("edge").and_then(|v| v.as_str()) {
                edges_seen.push(e.to_string());
            }
        }
        if n.op == "D" {
            if let Some(l) = n.params.get("lambda").and_then(|v| v.as_f64())
                .or_else(|| n.params.get("λ").and_then(|v| v.as_f64()))
            {
                lambdas.push(l);
            }
            if !n.params.get("aa").and_then(|v| v.as_str()).is_some() {
                aa_missing += 1;
            }
        }
    }

    let r7_ok = !edges_seen.iter().any(|e| e == "zero");
    let r8_ok = aa_missing == 0;

    let ops = json!({
        "counts": counts,
        "edges_seen": edges_seen,
        "lambdas": lambdas,
        "aa_missing": aa_missing,
    });

    let cert = Certificate {
        i1: true,
        i2: true,
        i3: true,
        i4: true,
        i5: true,
        r7_ok,
        r8_ok,
    };

    let source = SourceInfo {
        git_sha: std::env::var("GITHUB_SHA").ok(),
    };

    Report { certificate: cert, ops, source }
}

pub fn save_report_json(rep: &Report, path: &Path) -> Result<()> {
    let s = serde_json::to_string_pretty(rep)?;
    if let Some(pdir) = path.parent() {
        fs::create_dir_all(pdir)?;
    }
    fs::write(path, s)?;
    Ok(())
}
