use hash::Hash;
use rand::random;

type Address = Hash;

fn random_address() -> Address {
    let bytes = (0..32).map(|_| random::<u8>()).collect::<Vec<u8>>();

    let mut address = [0u8; 32];
    for i in 0..address.len() {
        address[i] = bytes[i];
    }
    address
}

#[derive(Debug, Eq, PartialEq)]
struct Account {
    address: Address,
    balance: u64,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Transaction {
    amount: u64,
    receiver: Address,
    sender: Address,
}

fn build_account(address: Address, transactions: Vec<Transaction>) -> Account {
    let txs: Vec<Transaction> = transactions.iter()
        .filter(|tx| tx.receiver == address || tx.sender == address)
        .map(|tx| tx.clone())
        .collect();

    Account {
        address,
        balance: txs.iter()
            .map(|tx| if tx.sender == address { -(tx.amount as i64) } else { tx.amount as i64 })
            .fold(0i64, |sum, v| sum + v) as u64,
        transactions: txs,
    }
}

#[cfg(test)]
mod tests {
    struct Test {
        input_txs: Vec<super::Transaction>,
        input_address: super::Address,
        output: super::Account,
    }

    #[test]
    fn test_build_account() {
        let address1 = super::random_address();
        let address2 = super::random_address();
        let address3 = super::random_address();

        let transaction1 = super::Transaction {
            amount: 1,
            receiver: address1,
            sender: address2,
        };
        let transaction2 = super::Transaction {
            amount: 2,
            receiver: address1,
            sender: address2,
        };
        let transaction3 = super::Transaction {
            amount: 1,
            receiver: address2,
            sender: address1,
        };
        let transaction4 = super::Transaction {
            amount: 3,
            receiver: address2,
            sender: address3,
        };

        let tests = [
            Test {
                input_txs: vec![transaction1, transaction2, transaction3, transaction4],
                input_address: address1,
                output: super::Account {
                    address: address1,
                    balance: 2,
                    transactions: vec![transaction1, transaction2, transaction3],
                },
            },
            Test {
                input_txs: vec![transaction1, transaction2, transaction3, transaction4],
                input_address: address2,
                output: super::Account {
                    address: address2,
                    balance: 1,
                    transactions: vec![transaction1, transaction2, transaction3, transaction4],
                },
            },
        ];

        tests.iter().for_each(|test| {
            let account = super::build_account(test.input_address, test.input_txs.clone());
            assert_eq!(account, test.output)
        });
    }
}