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
    pub nonce: u64,
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
            nonce: 0,
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
    
    pub fn mine(&mut self, difficulty: usize) {
        let prefix = std::iter::repeat("0").take(difficulty).collect::<String>();
        println!("Pref: {}", prefix);
        while !self.hash.starts_with(&prefix) {
            println!("Nonce: {}", self.nonce);
            self.nonce += 1;
            println!("Hash: {}", self.hash);
            self.hash = self.calculate_hash();
        }
    }
    
}
