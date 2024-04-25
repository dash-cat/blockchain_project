mod block;
mod blockchain;

// use crate::block::Block;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(vec!["Transaction 1 from Alice to Bob".to_string()]);
    blockchain.add_block(vec!["Transaction 2 from Bob to Charlie".to_string()]);

    println!("{:?}", blockchain);
}
