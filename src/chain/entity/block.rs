use chain::entity::header::Header;
use chain::entity::tx::Output;
use chain::entity::tx::Transaction;
use hash::EMPTY_HASH;
use hash::hash_from_string;

#[derive(Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        header: Header,
        transactions: Vec<Transaction>,
    ) -> Block {
        Block {
            header,
            transactions,
        }
    }
}

pub fn genesis_block() -> Block {
    Block {
        header: Header {
            index: 0,
            previous_hash: EMPTY_HASH,
            timestamp: 1516146240,
            transactions_hash: hash_from_string("85fca3e06fe7196148c3f6beae1aeb7dc8b9308ce26bbd0f32bda91a60d63bbc").unwrap(),
            nonce: 84167,
            hash: hash_from_string("00007bd9a44c3fea74388c483c3fc2fc8ac593c3da5566fbc1427cbf023e4ed9").unwrap(),
            difficulty: 4,
        },
        transactions: vec![
            Transaction {
                id: hash_from_string("d81f935f0c45cd0df0ccf073ae0e33432dd14cd925262a51a5bb43a77f433862").unwrap(),
                inputs: vec![],
                outputs: vec![Output { index: 0, value: 10, public_key_hash: [0u8; 20] }],
                secret: hash_from_string("39f39efae3884e28f5a5c4a62dd994e2943ac9cc7f9684070dfd4add7353722f").unwrap(),
            }
        ],
    }
}