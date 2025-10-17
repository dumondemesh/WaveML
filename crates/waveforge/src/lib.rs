// waveforge stable API for STRICT-NF (Phase P1/I1)
use anyhow::Result;
use serde_json::Value;

pub mod strict_nf;

/// Stable API: canonicalize input graph JSON into STRICT-NF.
pub fn canonicalize_graph(input: &Value) -> Result<Value> {
    strict_nf::strict_nf(input)
}

/// Stable API: compute deterministic NF-ID (hex) of STRICT-NF canonical form.
pub fn nf_id_hex(input: &Value) -> Result<String> {
    strict_nf::strict_nf_hex(input)
}

#[deprecated(note = "Use canonicalize_graph() / nf_id_hex() from waveforge stable API")]
pub use strict_nf::{strict_nf, strict_nf_hex};
