use chain::entity::{Hashable, Header};
use hash;
use std::error;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum ConsensusError {
    InvalidHashDifficulty,
    InvalidProofOfWork,
}

impl fmt::Debug for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConsensusError::InvalidHashDifficulty => write!(f, "Invalid hash difficulty"),
            ConsensusError::InvalidProofOfWork => write!(f, "Invalid proof of work"),
        }
    }
}

impl error::Error for ConsensusError {
    fn description(&self) -> &str {
        match *self {
            ConsensusError::InvalidHashDifficulty => "Invalid hash difficulty",
            ConsensusError::InvalidProofOfWork => "Invalid proof of work",
        }
    }
}

impl fmt::Display for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

fn verify_header(header: Header) -> Result<(), ConsensusError> {
    if !hash::has_difficulty(header.hash, header.difficulty) {
        return Err(ConsensusError::InvalidHashDifficulty);
    }

    if header.to_hash() != header.hash {
        return Err(ConsensusError::InvalidProofOfWork);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_header() {
        use chain::entity;
        use chain::consensus;
        use hash;

        struct Test {
            name: &'static str,
            input: entity::Header,
            output: Option<consensus::ConsensusError>,
        }

        let tests = [
            Test {
                name: "genesis block",
                input: entity::genesis_block().header,
                output: None,
            },
            Test {
                name: "invalid hash difficulty",
                input: entity::Header {
                    index: 0,
                    previous_hash: hash::EMPTY_HASH,
                    timestamp: 1516146240,
                    transactions_hash: hash::hash_from_string("85fca3e06fe7196148c3f6beae1aeb7dc8b9308ce26bbd0f32bda91a60d63bbc").unwrap(),
                    nonce: 84167,
                    hash: hash::hash_from_string("00007bd9a44c3fea74388c483c3fc2fc8ac593c3da5566fbc1427cbf023e4ed9").unwrap(),
                    difficulty: 6,
                },
                output: Some(consensus::ConsensusError::InvalidHashDifficulty),
            },
            Test {
                name: "invalid hash",
                input: entity::Header {
                    index: 0,
                    previous_hash: hash::EMPTY_HASH,
                    timestamp: 1516146240,
                    transactions_hash: hash::hash_from_string("85fca3e06fe7196148c3f6beae1aeb7dc8b9308ce26bbd0f32bda91a60d63bbc").unwrap(),
                    nonce: 84167,
                    hash: hash::hash_from_string("00008bd9a44c3fea74388c483c3fc2fc8ac593c3da5566fbc1427cbf023e4ed9").unwrap(),
                    difficulty: 4,
                },
                output: Some(consensus::ConsensusError::InvalidProofOfWork),
            },
        ];

        tests.iter().for_each(|test| {
            let result = super::verify_header(test.input.clone());

            match test.output {
                None => {
                    assert!(result.is_ok(), "failed test '{}'", test.name)
                }
                Some(e) => {
                    assert!(result.is_err(), "expected error in test '{}'", test.name);
                    assert_eq!(result.unwrap_err(), e, "unexpected error in test '{}'", test.name)
                }
            }
        });
    }
}