use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct LintReport {
    pub issues: Vec<LintIssue>,
}

impl LintReport {
    pub fn push(&mut self, code: &'static str, msg: impl Into<String>) {
        self.issues.push(LintIssue { code, message: msg.into() });
    }
    pub fn ok(&self) -> bool { self.issues.is_empty() }
}

/// Минимальная модель графа в manifest["graph"]
#[derive(Debug, Deserialize)]
struct Node {
    id: String,
    op: String,                // "W","T","D","A","C","Align","Phi"
    #[serde(default)]
    params: Value,
}
#[derive(Debug, Deserialize)]
struct Edge { from: String, to: String }

#[derive(Debug, Deserialize)]
struct Graph {
    nodes: Vec<Node>,
    #[serde(default)]
    edges: Vec<Edge>,
    #[serde(default)]
    passports: Value,
}

fn get_graph(manifest: &Value) -> Option<Graph> {
    manifest.get("graph").and_then(|g| serde_json::from_value(g.clone()).ok())
}

/// R7: W∘T — только при Sta или edge ∈ {reflect, Toeplitz}; запрет zero-pad
pub fn lint_r7(manifest: &Value) -> LintReport {
    let mut rep = LintReport::default();
    if let Some(graph) = get_graph(manifest) {
        let sta = graph.passports.get("Sta").and_then(|v| v.as_bool()).unwrap_or(false);
        for n in graph.nodes {
            if n.op == "W" {
                let edge = n.params.get("edge").and_then(|v| v.as_str()).unwrap_or("reflect");
                if matches!(edge, "zero" | "zero-pad" | "zeros") {
                    rep.push("R7/edge", format!("W node '{}' uses forbidden edge padding '{}'", n.id, edge));
                }
                if !(sta || edge == "reflect" || edge.eq_ignore_ascii_case("toeplitz")) {
                    rep.push("R7/guard", format!("W node '{}' requires Sta or edge reflect/Toeplitz; got edge='{}'", n.id, edge));
                }
            }
        }
    } else {
        rep.push("R7/no-graph", "manifest.graph missing or malformed");
    }
    rep
}

/// R8: D∘W — только с aa=true и паспортами GB-R8 (PR_ε, MM_d)
pub fn lint_r8(manifest: &Value) -> LintReport {
    let mut rep = LintReport::default();
    if let Some(graph) = get_graph(manifest) {
        let pr_eps = graph.passports.get("PR_epsilon").is_some() || graph.passports.get("PR_ε").is_some();
        let mm_d = graph.passports.get("MM_d").is_some() || graph.passports.get("MM").is_some();
        for n in graph.nodes {
            if n.op == "D" {
                let aa = n.params.get("aa").and_then(|v| v.as_bool()).unwrap_or(false);
                if !aa {
                    rep.push("R8/aa", format!("D node '{}' must set aa=true before downsample", n.id));
                }
                if !(pr_eps && mm_d) {
                    rep.push("R8/passport", "GB-R8 passports required: PR_ε and MM(d)");
                }
            }
        }
    } else {
        rep.push("R8/no-graph", "manifest.graph missing or malformed");
    }
    rep
}

/// R9: запрет паттерна A∘Align вдоль ребра графа
pub fn lint_r9(manifest: &Value) -> LintReport {
    let mut rep = LintReport::default();
    if let Some(graph) = get_graph(manifest) {
        use std::collections::HashMap;
        let map: HashMap<_,_> = graph.nodes.iter().map(|n| (n.id.as_str(), n.op.as_str())).collect();
        for e in graph.edges {
            if let (Some(&"Align"), Some(&"A")) = (map.get(e.from.as_str()), map.get(e.to.as_str())) {
                rep.push("R9/A∘Align", format!("Forbidden pattern A∘Align via edge {} -> {}", e.from, e.to));
            }
        }
    } else {
        rep.push("R9/no-graph", "manifest.graph missing or malformed");
    }
    rep
}
