use bytes::u64_to_vec;
use chain::entity::Hashable;
use hash::Hash;
use hash::sha256;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    pub index: u64,
    pub previous_hash: Hash,
    pub timestamp: u64,
    pub transactions_hash: Hash,
    pub nonce: u64,
    pub hash: Hash,
    pub difficulty: u64,
}

impl Header {
    pub fn new(
        index: u64,
        previous_hash: Hash,
        timestamp: u64,
        transactions_hash: Hash,
        nonce: u64,
        hash: Hash,
        difficulty: u64,
    ) -> Header {
        Header {
            index,
            previous_hash,
            timestamp,
            transactions_hash,
            nonce,
            hash,
            difficulty,
        }
    }
}

impl Hashable for Header {
    fn to_hash(&self) -> Hash {
        let hash_input = [
            self.previous_hash.to_vec(),
            self.transactions_hash.to_vec(),
            u64_to_vec(self.timestamp),
            u64_to_vec(self.nonce),
            u64_to_vec(self.difficulty),
        ].concat();

        return sha256(&hash_input[..]);
    }
}
