
use serde::{Serialize, Deserialize};
use crate::block::Block; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        self.chain.push(new_block);
    }
}
