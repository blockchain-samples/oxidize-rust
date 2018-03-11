use chain::entity::Block;
use chain::entity::genesis_block;
use chain::entity::Transaction;

pub mod entity;
pub mod consensus;

#[derive(Debug)]
pub struct Blockchain {
    store: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { store: vec![genesis_block()] }
    }
}
