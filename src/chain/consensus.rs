use chain::entity::Header;
use hash;
use std::error;
use std::fmt;

#[derive(Copy, Clone)]
pub enum ConsensusError {
    InvalidHashDifficulty
}

impl fmt::Debug for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConsensusError::InvalidHashDifficulty => write!(f, "Invalid hash difficulty"),
        }
    }
}

impl error::Error for ConsensusError {
    fn description(&self) -> &str {
        match *self {
            ConsensusError::InvalidHashDifficulty => "Invalid hash difficulty",
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

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_header() {
        use chain::entity;

        let genesis_block = entity::genesis_block();
        let result = super::verify_header(genesis_block.header);
        assert!(result.is_ok())
    }
}