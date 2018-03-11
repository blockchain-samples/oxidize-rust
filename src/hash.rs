use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;
use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::FromHexError;
use std::ops::Shl;

pub type Hash = [u8; HASH_SIZE];

pub const HASH_SIZE: usize = 32;
pub const EMPTY_HASH: Hash = [0u8; HASH_SIZE];

pub fn hash_from_string(hash_str: &str) -> Result<Hash, FromHexError> {
    let mut hash_str = hash_str.to_owned();
    while hash_str.len() < HASH_SIZE * 2 {
        hash_str = format!("{}{}", "0", hash_str)
    }

    let bytes: Vec<u8> = hash_str.from_hex()?;
    let mut hash = [0u8; HASH_SIZE];
    for i in 0..hash.len() {
        hash[i] = bytes[i];
    }
    Ok(hash)
}

pub fn has_difficulty(hash: Hash, difficulty: u64) -> bool {
    let hash_int = BigUint::from_bytes_be(&hash[..]);
    let target = BigUint::one().shl((256 - (4 * difficulty)) as usize);
    return hash_int < target;
}

#[inline]
pub fn sha256(data: &[u8]) -> Hash {
    let mut out = [0u8; HASH_SIZE];
    let mut hasher = Sha256::new();
    hasher.input(data);
    hasher.result(&mut out);
    return out;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sha256() {
        struct Test {
            input: &'static str,
            output: &'static str,
        }

        let tests = [
            Test {
                input: "",
                output: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            },
            Test {
                input: "a",
                output: "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
            },
            Test {
                input: "ab",
                output: "fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603",
            },
            Test {
                input: "abc",
                output: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            },
            Test {
                input: "abcd",
                output: "88d4266fd4e6338d13b845fcf289579d209c897823b9217da3e161936f031589",
            },
            Test {
                input: "abcde",
                output: "36bbe50ed96841d10443bcb670d6554f0a34b761be67ec9c4a8ad2c0c44ca42c",
            },
            Test {
                input: "abcdef",
                output: "bef57ec7f53a6d40beb640a780a639c83bc29ac8a9816f1fc6c5c6dcd93c4721",
            },
            Test {
                input: "abcdefg",
                output: "7d1a54127b222502f5b79b5fb0803061152a44f92b37e23c6527baf665d4da9a",
            },
            Test {
                input: "abcdefgh",
                output: "9c56cc51b374c3ba189210d5b6d4bf57790d351c96c47c02190ecf1e430635ab",
            },
            Test {
                input: "abcdefghi",
                output: "19cc02f26df43cc571bc9ed7b0c4d29224a3ec229529221725ef76d021c8326f",
            },
            Test {
                input: "abcdefghij",
                output: "72399361da6a7754fec986dca5b7cbaf1c810a28ded4abaf56b2106d06cb78b0",
            },
        ];

        tests.iter().for_each(|test| {
            use rustc_serialize::hex::ToHex;

            let hash = super::sha256(test.input.as_bytes()).to_hex();
            assert_eq!(hash, test.output, "expected hashes to be equal")
        });
    }

    #[test]
    fn test_hash() {
        struct Test {
            input: &'static str,
            output: &'static str,
            want_err: bool,
        }

        use rustc_serialize::hex::ToHex;

        let tests = [
            Test {
                input: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
                output: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
                want_err: false,
            },
            Test {
                input: "19d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
                output: "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
                want_err: false,
            },
            Test {
                input: "",
                output: "0000000000000000000000000000000000000000000000000000000000000000",
                want_err: false,
            },
            Test {
                input: "1",
                output: "0000000000000000000000000000000000000000000000000000000000000001",
                want_err: false,
            },
            Test {
                input: "3264bc2ac36a60840790ba1d475d01367e7c723da941069e9dc",
                output: "00000000000003264bc2ac36a60840790ba1d475d01367e7c723da941069e9dc",
                want_err: false,
            },
            Test {
                input: "012345678901234567890123456789012345678901234567890123456789012345",
                output: "0123456789012345678901234567890123456789012345678901234567890123",
                want_err: false,
            },
            Test {
                input: "abcdefg",
                output: "",
                want_err: true,
            },
        ];

        tests.iter().for_each(|test| {
            let hash_res = super::hash_from_string(test.input);

            if test.want_err {
                hash_res.unwrap_err();
            } else {
                assert_eq!(hash_res.unwrap().to_hex(), test.output, "expected hashes to be equal")
            }
        });
    }

    #[test]
    fn test_has_difficulty() {
        struct Test {
            name: &'static str,
            input_hash: super::Hash,
            input_difficulty: u64,
            output: bool,
        }

        let tests = [
            Test {
                name: "has difficulty 0",
                input_hash: super::hash_from_string("69c6ba688ee9eb482aad1bf310681da13cd0690bd631beefc959de8ac8440bc8").unwrap(),
                input_difficulty: 0,
                output: true,
            },
            Test {
                name: "has difficulty 1",
                input_hash: super::hash_from_string("09c6ba688ee9eb482aad1bf310681da13cd0690bd631beefc959de8ac8440bc8").unwrap(),
                input_difficulty: 1,
                output: true,
            },
            Test {
                name: "has difficulty 4",
                input_hash: super::hash_from_string("0000ba688ee9eb482aad1bf310681da13cd0690bd631beefc959de8ac8440bc8").unwrap(),
                input_difficulty: 4,
                output: true,
            },
            Test {
                name: "has difficulty 8",
                input_hash: super::hash_from_string("000000008ee9eb482aad1bf310681da13cd0690bd631beefc959de8ac8440bc8").unwrap(),
                input_difficulty: 8,
                output: true,
            },
            Test {
                name: "has difficulty 4, not 6",
                input_hash: super::hash_from_string("00009f7a8ee9eb482aad1bf310681da13cd0690bd631beefc959de8ac8440bc8").unwrap(),
                input_difficulty: 6,
                output: false,
            },
        ];

        tests.iter().for_each(|test| {
            let has_difficulty = super::has_difficulty(test.input_hash, test.input_difficulty);

            assert_eq!(has_difficulty, test.output, "failed for test '{}'", test.name)
        })
    }
}
