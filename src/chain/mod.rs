use chain::entity::Block;
use chain::entity::genesis_block;
use chain::entity::Header;
use hash::Hash;
use std::fmt;

pub mod consensus;
pub mod entity;
pub mod rpc;
pub mod wire;

#[derive(Debug)]
pub enum ChainError {}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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
        let best_block = self.store.last()
            .map(|block| block.clone())
            .expect("chain should have at least one block");
        Ok(best_block)
    }

    pub fn best_header(&self) -> Result<Header, ChainError> {
        let best_header = self.store.last()
            .map(|block| block.header.clone())
            .expect("chain should have at least one block");
        Ok(best_header)
    }

    pub fn get_headers(&self) -> Result<Vec<Header>, ChainError> {
        let headers = self.store.iter()
            .map(|b| b.header.clone())
            .collect();
        Ok(headers)
    }

    pub fn get_block_by_hash(&self, hash: Hash) -> Result<Option<Block>, ChainError> {
        let block = self.store.iter()
            .find(|block| block.header.hash == hash)
            .map(|block| block.clone());
        Ok(block)
    }
}
