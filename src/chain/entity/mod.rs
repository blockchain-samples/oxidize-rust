use hash::Hash;

mod block;
mod header;
mod tx;

pub type Block = block::Block;
pub type Header = header::Header;
pub type Transaction = tx::Transaction;

#[inline]
pub fn genesis_block() -> Block {
    block::genesis_block()
}

pub trait Hashable {
    fn to_hash(&self) -> Hash;
}
