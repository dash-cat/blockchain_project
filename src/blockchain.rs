
use serde::{Serialize, Deserialize};
use crate::block::Block; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize, 
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 4, // Пример сложности
        };
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string());
        blockchain.chain.push(genesis_block);
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        println!("bl1");
        let previous_hash = self.chain.last().unwrap().hash.clone();
        println!("bl2");
        let mut new_block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        println!("bl3");
        new_block.mine(self.difficulty);
        println!("bl4");
        self.chain.push(new_block);
    }
}
