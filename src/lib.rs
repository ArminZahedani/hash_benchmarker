use blake3;
use sha2::{Digest, Sha256};
use sha3::Sha3_256;

pub fn sha3_hash(data: &str) -> Vec<u8> {
    let mut hasher = Sha3_256::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.to_vec()
}

pub fn sha2_hash(data: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.to_vec()
}

pub fn blake3_hash(data: &str) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();

    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    result.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha2_test() {
        let result = sha2_hash("abc");
        assert_eq!(
            result,
            hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
                .unwrap()
        );
    }
    #[test]
    fn sha3_test() {
        let result = sha3_hash("abc");
        assert_eq!(
            result,
            hex::decode("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532")
                .unwrap()
        );
    }

    #[test]
    fn blake3_test() {
        let result = blake3_hash("abc");
        assert_eq!(
            result,
            hex::decode("6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85")
                .unwrap()
        );
    }
}
