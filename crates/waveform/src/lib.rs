use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: String,          // "0.3.0"
    pub schema_semver: String,    // "0.3.0"
    pub domain: String,           // "audio" | ...
    #[serde(default = "default_endianness")]
    pub endianness: String,       // "little" | "big"
    pub timebase: Timebase,
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub wave_unit: Option<WaveUnit>,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub generator: Option<String>,
}

fn default_endianness() -> String { "little".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timebase {
    pub unit: String, // "s"
    pub tick: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveUnit {
    pub freq_base: Option<f64>,
    pub phase_zero: Option<f64>,
    pub amplitude_ref: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waveform {
    pub header: Header,
    #[serde(default)]
    pub axes: Vec<serde_json::Value>,
    #[serde(default)]
    pub components: serde_json::Value,
    #[serde(default)]
    pub multiscale: Option<serde_json::Value>,
    #[serde(default)]
    pub register_map: Option<serde_json::Value>,
    #[serde(default)]
    pub integrity: Option<serde_json::Value>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}

impl Waveform {
    /// Minimal v0.3 sanity checks (not a full JSON-Schema validator)
    pub fn validate_basic(&self) -> Result<()> {
        if !self.header.version.starts_with("0.3.") {
            return Err(anyhow!("header.version must be 0.3.x"));
        }
        if !self.header.schema_semver.starts_with("0.3.") {
            return Err(anyhow!("schema_semver must be 0.3.x"));
        }
        let allowed = ["audio","image","video","timeseries","graph","text","dna","field"];
        if !allowed.contains(&self.header.domain.as_str()) {
            return Err(anyhow!("domain {} is not allowed", self.header.domain));
        }
        if self.header.timebase.tick <= 0.0 {
            return Err(anyhow!("timebase.tick must be > 0"));
        }
        Ok(())
    }
}
