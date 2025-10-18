use anyhow::Result;
use serde_json::Value;

/// Delegates to crate-level stable API, so we don't change canonicalization logic.
pub fn strict_nf(input: &Value) -> Result<Value> {
    crate::canonicalize_graph(input)
}

pub fn strict_nf_hex(input: &Value) -> Result<String> {
    crate::nf_id_hex(input)
}
