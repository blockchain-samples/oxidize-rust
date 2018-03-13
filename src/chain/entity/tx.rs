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
}

#[derive(Debug, Clone)]
pub struct Output {
    pub index: u64,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct OutputReference {
    pub id: Hash,
    pub output: Output,
}