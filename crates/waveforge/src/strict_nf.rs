use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Window { Hann, Hamming, Blackman, Other(String) }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeKey {
    pub op: String,
    pub n_fft: u32,
    pub hop: u32,
    pub window: Window,
    pub center: bool,
    pub pad_mode: String,
}

impl NodeKey {
    pub fn tuple(&self) -> (&str, u32, u32, &Window, bool, &str) {
        (&self.op, self.n_fft, self.hop, &self.window, self.center, &self.pad_mode)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub key: NodeKey,
    pub params: serde_json::Value,
}

pub fn sort_nodes(mut nodes: Vec<Node>) -> Vec<Node> {
    nodes.sort_by(|a,b| a.key.tuple().cmp(&b.key.tuple()));
    nodes
}

pub fn nf_id(nodes: &[Node]) -> String {
    let mut hasher = Sha256::new();
    let ser = serde_json::to_vec(nodes).expect("serialize nodes");
    hasher.update(&ser);
    let out = hasher.finalize();
    format!("nfid:sha256:{:x}", out)
}
