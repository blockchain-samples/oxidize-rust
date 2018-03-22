use hash::Hash;
pub use self::block::Block;
pub use self::block::genesis_block;
pub use self::block::genesis_hash;
pub use self::header::Header;
pub use self::tx::Input;
pub use self::tx::Output;
pub use self::tx::OutputReference;
pub use self::tx::Transaction;

mod block;
mod header;
mod tx;

pub trait Hashable {
    fn to_hash(&self) -> Hash;
}
