use chain::entity::header::Header;
use chain::entity::tx::Transaction;

#[derive(Debug)]
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
