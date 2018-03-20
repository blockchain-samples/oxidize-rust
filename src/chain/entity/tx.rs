use hash::Hash;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Hash,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub secret: [u8; 32],
}

impl Transaction {
    pub fn new(
        id: Hash,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
        secret: [u8; 32],
    ) -> Transaction {
        Transaction {
            id,
            inputs,
            outputs,
            secret,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub output_ref: OutputReference,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub index: u32,
    pub value: u64,
    pub public_key_hash: [u8; 20],
}

#[derive(Debug, Clone)]
pub struct OutputReference {
    pub id: Hash,
    pub output: Output,
}