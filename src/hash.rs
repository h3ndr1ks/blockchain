use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: [u8; 32]) -> Self {
        Hash(data)
    }

    pub fn try_from_hash(hex: &str) -> Self {
        // TODO: remove this .unwrap()
        let result = hex::decode(hex).unwrap();
        Hash::new(result.try_into().unwrap())
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

#[cfg(test)]
mod test {
    use crate::hash::Hash;

    #[test]
    fn leading_zeros() {
        for nr_zeros in 0..33 {
            let mut array = [0; 32];
            for i in nr_zeros..32 {
                array[i] = 255;
            }
            let hash = Hash(array);
            assert_eq!(hash.leading_zeros(), nr_zeros);
        }
    }

    #[test]
    fn display() {
        let hash = Hash([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        assert_eq!(
            hash.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        let hash = Hash([
            255, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            240, 129, 128, 255,
        ]);
        assert_eq!(
            hash.to_string(),
            "ff00ff00000000000000000000000000000000000000000000000000f08180ff"
        );
    }

    #[test]
    fn from_hex() {
        let hex = "000000202081f1c5c1a9a6f228172c7000c09aa1740972be3b81b4b0b5087f9c";
        let hash = Hash::try_from_hash(hex);
        assert_eq!(hash.to_string(), hex);
    }
}
