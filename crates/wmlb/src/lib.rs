use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

pub const ABI_VERSION: &str = "1.0";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph{ pub version:String, pub created_at:String, pub nodes:Vec<Node> }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node{ pub id:String, pub op:String, pub params:serde_json::Value, pub inputs:Vec<String>, pub outputs:Vec<String> }

impl Graph{
    pub fn new()->Self{ Self{ version:ABI_VERSION.into(), created_at: OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default(), nodes:vec![] } }
}
