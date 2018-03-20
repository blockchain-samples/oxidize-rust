use chain::entity;

pub mod blockchain_entities;

pub type Block = blockchain_entities::Block;
pub type Header = blockchain_entities::BlockHeader;
pub type Transaction = blockchain_entities::Transaction;
pub type Input = blockchain_entities::SignedInput;
pub type Output = blockchain_entities::Output;

pub fn to_wire_block(block: entity::Block) -> Block {
    let mut out = Block::new();

    out.set_header(to_wire_header(block.header));
    out.set_transactions(
        ::protobuf::RepeatedField::from_vec(
            block.transactions.iter()
                .map(|tx| to_wire_tx(tx.clone()))
                .collect()
        )
    );

    return out;
}

pub fn from_wire_block(block: Block) -> entity::Block {
    return entity::Block {
        header: from_wire_header(block.get_header().clone()),
        transactions: block.get_transactions().iter().map(|tx| from_wire_tx(tx.clone())).collect(),
    };
}

pub fn to_wire_header(header: entity::Header) -> Header {
    let mut out = Header::new();

    out.set_index(header.index);
    out.set_previousHash(header.previous_hash.to_vec());
    out.set_timestamp(header.timestamp);
    out.set_transactionsHash(header.transactions_hash.to_vec());
    out.set_nonce(header.nonce);
    out.set_hash(header.hash.to_vec());
    out.set_difficulty(header.difficulty);

    out
}

pub fn from_wire_header(header: Header) -> entity::Header {
    return entity::Header {
        index: header.get_index(),
        previous_hash: ::hash::hash_from_bytes(header.get_previousHash()),
        timestamp: header.get_timestamp(),
        transactions_hash: ::hash::hash_from_bytes(header.get_transactionsHash()),
        nonce: header.get_nonce(),
        hash: ::hash::hash_from_bytes(header.get_hash()),
        difficulty: header.get_difficulty(),
    };
}

pub fn to_wire_tx(transaction: entity::Transaction) -> Transaction {
    let mut out = Transaction::new();
    out.set_id(transaction.id.to_vec());
    out.set_inputs(
        ::protobuf::RepeatedField::from_vec(
            transaction.inputs.iter()
                .map(|input| to_wire_input(input.clone()))
                .collect()
        )
    );
    out.set_outputs(
        ::protobuf::RepeatedField::from_vec(
            transaction.outputs.iter()
                .map(|output| to_wire_output(output.clone()))
                .collect()
        )
    );
    out.set_secret(transaction.secret.to_vec());

    out
}

pub fn from_wire_tx(transaction: Transaction) -> entity::Transaction {
    return entity::Transaction {
        id: ::hash::hash_from_bytes(transaction.get_id()),
        inputs: transaction.get_inputs()
            .iter()
            .map(|input| from_wire_input(input.clone()))
            .collect(),
        outputs: transaction.get_outputs()
            .iter()
            .map(|output| from_wire_output(output.clone()))
            .collect(),
        secret: ::hash::hash_from_bytes(transaction.get_secret()),
    };
}

pub fn to_wire_input(input: entity::Input) -> Input {
    let mut out = Input::new();

    out.set_reference({
        let mut reference = blockchain_entities::OutputReference::new();
        reference.set_id(input.output_ref.id.to_vec());
        reference.set_output(to_wire_output(input.output_ref.output));
        reference
    });
    out.set_publicKey(input.public_key);
    out.set_signature(input.signature);

    out
}

pub fn from_wire_input(input: Input) -> entity::Input {
    return entity::Input {
        output_ref: entity::OutputReference {
            id: ::hash::hash_from_bytes(input.get_reference().get_id()),
            output: from_wire_output(input.get_reference().get_output().clone()),
        },
        public_key: input.get_publicKey().to_vec(),
        signature: input.get_signature().to_vec(),
    };
}

pub fn to_wire_output(output: entity::Output) -> Output {
    let mut out = Output::new();

    out.set_index(output.index);
    out.set_value(output.value);
    out.set_publicKeyHash(output.public_key_hash.to_vec());

    out
}

pub fn from_wire_output(output: Output) -> entity::Output {
    return entity::Output {
        index: output.get_index(),
        value: output.get_value(),
        public_key_hash: public_key_hash_from_bytes(output.get_publicKeyHash()),
    };
}

fn public_key_hash_from_bytes(bytes: &[u8]) -> [u8; 20] {
    let mut hash = [0u8; 20];
    for i in 0..hash.len() {
        hash[i] = bytes[i];
    }
    hash
}
