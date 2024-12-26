use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: [u8; 32]) -> Self {
        Hash(data)
    }

    pub fn empty() -> Self {
        Hash([0; 32])
    }

    pub fn leading_zeros(&self) -> usize {
        self.0.iter().take_while(|&&byte| byte == 0).count()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

// This is required for the sha256 hasher to work.
// Otherwise, this (Hash) could not be passed to the hasher.update(...) method.
impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
