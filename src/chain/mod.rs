use chain::entity::Block;
use chain::entity::genesis_block;

pub mod consensus;
pub mod entity;

#[derive(Debug)]
pub enum ChainError {
}

#[derive(Debug)]
pub struct Blockchain {
    store: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { store: vec![genesis_block()] }
    }

    pub fn best_block(&self) -> Result<Block, ChainError> {
        let best_block = self.store.last().expect("chain should never be empty");
        Ok(best_block.clone())
    }
}
