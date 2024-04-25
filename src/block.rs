use sha2::{Sha256, Digest};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<String>, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(self.transactions.join(""));
        hasher.update(&self.previous_hash);

        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
    
    // fn mine(&mut self, difficulty: usize) {
    //     let prefix: String = std::iter::repeat("0").take(difficulty).collect();
    //     while !self.hash.starts_with(&prefix) {
    //         self.nonce += 1;
    //         self.hash = self.calculate_hash();
    //     }
    // }
}
