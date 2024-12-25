use jiff::Zoned;
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn complies_with_difficulty(&self, mut difficulty: usize) -> bool {
        self.0.iter()
            .take_while(|&&byte| byte == 0)
            .count() >= difficulty
    }

    pub fn empty() -> Self {
        Hash([0; 32])
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}


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
        hasher.update(self.prior_hash.0);

        let hash256 = hasher.finalize();
        Hash(hash256[..32].try_into().unwrap())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.index, self.timestamp, self.nonce, self.prior_hash)
    }
}