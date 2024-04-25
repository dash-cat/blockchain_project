mod block;
mod blockchain;

// use crate::block::Block;
use crate::blockchain::Blockchain;

fn main() { 
    println!("1");
    let mut blockchain = Blockchain::new();
    println!("2");
    blockchain.add_block(vec!["Transaction 1 from Alice to Bob".to_string()]);
    println!("3");
    blockchain.add_block(vec!["Transaction 2 from Bob to Charlie".to_string()]);
    println!("4");
}
