use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use sha256::digest;
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub proof: u64,
    pub prev_hash: String,
    pub hash: String,
}
impl Block {
    /// this fn responsible to create a new block
    pub fn new(index: u32, proof: u64, p_hash: String) -> Self {
        let block = Block {
            index: index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof: proof,
            prev_hash: p_hash,
            hash: "String".to_string(),
        };
        block
    }
    /// this is the fn to generate & return a hash
    pub fn calculate_hash(self) -> String {
        let hash = serde_json::to_string(&self).unwrap();
        digest(hash)
    }
}
