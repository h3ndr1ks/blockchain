use crate::hash::Hash;
use jiff::Zoned;
use sha2::{Digest, Sha256};
use std::fmt::Display;

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

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block(idx={}, hash=\"{}\")", self.index, self.hash())
    }
}
