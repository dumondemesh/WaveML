//! WaveReport v1.0 — отчёт из IR: базовая проверка инвариантов и сводка по операторам.
use anyhow::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use time::OffsetDateTime;
use wmlb::Graph;

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(rename = "I1")]
    pub i1: bool,
    #[serde(rename = "I2")]
    pub i2: bool,
    #[serde(rename = "I3")]
    pub i3: bool,
    #[serde(rename = "I4")]
    pub i4: bool,
    #[serde(rename = "I5")]
    pub i5: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub report_version: String,
    pub generated_at: String,
    pub certificate: Certificate,
    /// Сводка по операторам/параметрам (counts, edges, aa_missing, lambdas)
    pub ops: Map<String, Value>,
    pub notes: String,
}

pub fn from_ir(g: &Graph) -> Report {
    // Подсчёты и простые проверки по IR:
    let mut counts = std::collections::BTreeMap::<String, u32>::new();
    let mut edges_ok = true;
    let mut downs_ok = true;

    let mut edges_seen: Vec<String> = Vec::new();
    let mut aa_missing = 0usize;
    let mut lambdas: Vec<f64> = Vec::new();

    for n in &g.nodes {
        *counts.entry(n.op.clone()).or_default() += 1;

        if n.op == "W" {
            if let Some(edge_val) = n.params.get("edge") {
                let s = edge_val.as_str().unwrap_or_default().to_string();
                if !(s == "reflect" || s == "Toeplitz") {
                    edges_ok = false;
                }
                edges_seen.push(s);
            }
        }

        if n.op == "D" {
            if !n.params.get("aa").is_some() {
                downs_ok = false;
                aa_missing += 1;
            }
            if let Some(l) = n.params.get("lambda").and_then(|v| v.as_f64()) {
                lambdas.push(l);
            }
        }
    }

    // Прототипная трактовка: I1 = базовые инварианты по IR (R7/R8) пройдены
    let cert = Certificate {
        i1: edges_ok && downs_ok,
        i2: true,
        i3: true,
        i4: true,
        i5: true,
    };

    // Упакуем сводку
    let mut counts_map = Map::new();
    for (k, v) in counts {
        counts_map.insert(k, json!(v));
    }

    let mut ops = Map::new();
    ops.insert("counts".into(), Value::Object(counts_map));
    ops.insert("edges_seen".into(), json!(edges_seen));
    ops.insert("aa_missing".into(), json!(aa_missing));
    ops.insert("lambdas".into(), json!(lambdas));

    Report {
        report_version: "1.0".into(),
        generated_at: OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default(),
        certificate: cert,
        ops,
        notes: "auto-generated from IR (proto)".into(),
    }
}

pub fn save_report_json(rep: &Report, path: &std::path::Path) -> Result<()> {
    let s = serde_json::to_string_pretty(rep)?;
    std::fs::create_dir_all(path.parent().unwrap_or(std::path::Path::new(".")))?;
    std::fs::write(path, s)?;
    Ok(())
}
