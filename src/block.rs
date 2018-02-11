use byteorder::{BigEndian, ByteOrder};
use hash::EMPTY_HASH;
use hash::Hash;
use hash::sha256;
use time::get_time;

struct Block<'a> {
    timestamp: u64,
    previous_hash: Hash,
    hash: Hash,
    data: &'a str,
}

fn make_block(previous_hash: Hash, data: &str) -> Block {
    let timestamp = get_time().sec as u64;

    let hash_input = [
        previous_hash.to_vec(),
        u64_to_vec(timestamp),
        data.as_bytes().to_vec(),
    ].concat();
    let hash = sha256(&hash_input[..]);

    return Block {
        data: data,
        timestamp: timestamp,
        previous_hash: previous_hash,
        hash: hash,
    };
}

fn u64_to_vec(v: u64) -> Vec<u8> {
    let mut buf = [0u8; 8];
    BigEndian::write_u64(&mut buf, v);
    buf.to_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_make_block() {
        let previous_hash = super::EMPTY_HASH;
        let data = "data yo";
        let block = super::make_block(previous_hash, &data);

        let hash_input = [
            previous_hash.to_vec(),
            super::u64_to_vec(block.timestamp),
            data.as_bytes().to_vec(),
        ].concat();
        let expected_hash = super::sha256(&hash_input[..]);

        assert_eq!(block.hash, expected_hash);
    }
}