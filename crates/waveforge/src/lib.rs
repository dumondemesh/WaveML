use anyhow::Result;
use serde_json::Value;

pub mod strict_nf {
    use super::*;
    use sha2::{Sha256, Digest};
    use hex::ToHex;

    /// Canonicalize graph to STRICT-NF.
    /// Stub: returns input as-is. Replace with real implementation in your codebase.
    pub fn strict_nf(input: &Value) -> Result<Value> {
        Ok(input.clone())
    }

    /// Compute NF-ID (hex string) from canonical form.
    /// Stub: SHA-256 over canonical JSON string.
    pub fn strict_nf_hex(input: &Value) -> Result<String> {
        let canon = strict_nf(input)?;
        let s = serde_json::to_string(&canon)?;
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let bytes = hasher.finalize();
        Ok(bytes.encode_hex::<String>())
    }
}
