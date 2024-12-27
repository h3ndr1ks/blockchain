use crate::hash::Hash;
use jiff::Zoned;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u128,
    pub timestamp: Zoned,
    pub data: Vec<u8>,
    pub nonce: u128,
    pub prior_hash: Hash,
}

impl Block {
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_be_bytes());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.data);
        hasher.update(self.nonce.to_be_bytes());
        hasher.update(&self.prior_hash);

        let hash256 = hasher.finalize();
        Hash::new(hash256[..32].try_into().unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::block::Block;
    use crate::hash::Hash;
    use jiff::Zoned;
    use std::str::FromStr;

    #[test]
    fn block_hash() {
        let block = Block {
            index: 0,
            timestamp: Zoned::from_str("2024-12-27T18:00:00+01:00[Europe/Berlin]").unwrap(),
            data: vec![0, 1, 2],
            nonce: 0,
            prior_hash: Hash::empty(),
        };
        assert_eq!(
            block.hash().to_string(),
            "27ed0af1e00333a70be7a0111d4a3cda6eb9d87a407a62f336935bfb91bea10e"
        );
    }
}
