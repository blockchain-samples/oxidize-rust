use hash::Hash;

#[derive(Debug, Clone)]
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
