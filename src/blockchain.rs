use crate::block::Block;
use crate::hash::Hash;
use jiff::Zoned;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: vec![] }
    }

    pub fn check_validity(&self, difficulty: usize) -> bool {
        for window in self.chain.windows(2) {
            let current_block = &window[0];
            let next_block = &window[1];

            if current_block.index + 1 != next_block.index {
                return false;
            }

            if current_block.hash().leading_zeros() < difficulty {
                return false;
            }

            if current_block.hash() != next_block.prior_hash {
                return false;
            }
        }

        if self.chain.len() > 1 && self.chain.last().unwrap().hash().leading_zeros() < difficulty {
            return false;
        }

        if self.chain.len() == 1 {
            if self.chain[0].hash().leading_zeros() < difficulty {
                return false;
            }
        }

        true
    }

    pub fn mine_block(&mut self, data: Vec<u8>, difficulty: usize) {
        let new_index = if self.chain.len() == 0 {
            0
        } else {
            self.chain.last().unwrap().index + 1
        };
        let last_hash = if self.chain.len() == 0 {
            Hash::empty()
        } else {
            self.chain.last().unwrap().hash()
        };

        let mut nonce: u128 = 0;

        let mut new_block = Block {
            index: new_index,
            timestamp: Zoned::now(),
            data,
            nonce: 0,
            prior_hash: last_hash,
        };

        while new_block.hash().leading_zeros() < difficulty {
            nonce += 1;
            new_block.nonce = nonce;
        }

        self.chain.push(new_block);
    }
}

#[cfg(test)]
mod test {
    use crate::block::Block;
    use crate::blockchain::Blockchain;
    use crate::hash::Hash;
    use jiff::Zoned;
    use std::str::FromStr;

    const DIFFICULTY: usize = 3;

    fn example_blockchain1() -> Blockchain {
        Blockchain {
            chain: vec![Block {
                index: 0,
                timestamp: Zoned::from_str("2024-12-27T22:45:16.63456+01:00[Europe/Berlin]")
                    .unwrap(),
                data: vec![],
                nonce: 67723593,
                prior_hash: Hash::empty(),
            }],
        }
    }

    fn example_blockchain3() -> Blockchain {
        let mut blockchain = example_blockchain1();
        blockchain.chain.push(Block {
            index: 1,
            timestamp: Zoned::from_str("2024-12-27T22:46:02.827648+01:00[Europe/Berlin]").unwrap(),
            data: vec![0, 1, 255],
            nonce: 8486214,
            prior_hash: Hash::try_from_hex(
                "000000b6bbc35e2d25e0694c45670a0321fa3c33fcc67a0c6abf971d2f8a718a",
            )
            .unwrap(),
        });
        blockchain.chain.push(Block {
            index: 2,
            timestamp: Zoned::from_str("2024-12-27T22:46:08.524718+01:00[Europe/Berlin]").unwrap(),
            data: vec![],
            nonce: 15393667,
            prior_hash: Hash::try_from_hex(
                "000000202081f1c5c1a9a6f228172c7000c09aa1740972be3b81b4b0b5087f9c",
            )
            .unwrap(),
        });
        blockchain
    }

    #[test]
    fn check_validity() {
        // check empty blockchain
        assert_eq!(Blockchain::new().check_validity(DIFFICULTY), true);

        // change block such that block is invalid
        let mut blockchain = example_blockchain1();
        let block = blockchain.chain.get_mut(0).unwrap();
        block.nonce = 0;
        assert_eq!(blockchain.check_validity(DIFFICULTY), false);

        // check blockchains that are valid
        assert_eq!(example_blockchain1().check_validity(DIFFICULTY), true);
        assert_eq!(example_blockchain3().check_validity(DIFFICULTY), true);
    }
}
