use time::Timespec;
use time::get_time;
use hash::Hash;
use hash::calculate_hash;

struct Block {
    data : String,
    timestamp : Timespec,
    previous_hash: Hash,
    hash : Hash
}

fn make_block(previous_hash: Hash, time: Timespec) -> Block {
    let data = String::from("");
    let hash_input = data + &previous_hash + &time.sec.to_string();

    return Block{
        data: String::from(""),
        timestamp: (time),
        previous_hash: previous_hash,
        hash: calculate_hash(&hash_input),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash() {
        let timestamp = super::get_time();
        let previous_hash = String::from("");
        let block = super::make_block(previous_hash, timestamp);

        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.timestamp, timestamp);
        assert_eq!(block.hash, String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
    }
}