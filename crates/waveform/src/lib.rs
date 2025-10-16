use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header{ pub domain:String, pub rate:Option<u32>, pub ver:String }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaveForm{ pub header:Header, pub tracks:serde_json::Value, pub passports:serde_json::Value }

impl WaveForm{
    pub fn load_json(p:&std::path::Path)->anyhow::Result<Self>{ Ok(serde_json::from_str(&std::fs::read_to_string(p)?)?) }
    pub fn save_json(&self, p:&std::path::Path)->anyhow::Result<()>{
        std::fs::create_dir_all(p.parent().unwrap_or(std::path::Path::new(".")))?;
        std::fs::write(p, serde_json::to_string_pretty(self)?)?; Ok(()) }
}
