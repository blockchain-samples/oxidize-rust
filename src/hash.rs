use crypto::digest::Digest;
use crypto::sha2;

pub type Hash = [u8; 32];

pub const EMPTY_HASH: Hash = [0u8; 32];

#[inline]
pub fn sha256(data: &[u8]) -> Hash {
    let mut out = [0u8; 32];
    let mut hasher = sha2::Sha256::new();
    hasher.input(data);
    hasher.result(&mut out);
    return out;
}

#[cfg(test)]
mod tests {
    struct Test {
        input: &'static str,
        output: &'static str,
    }

    #[test]
    fn test_sha256() {
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
            assert_eq!(hash, test.output)
        });
    }
}
