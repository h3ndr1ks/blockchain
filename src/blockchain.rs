use std::fmt::{Display, Formatter};
use jiff::{Zoned};
use crate::block::{Block, Hash};

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![
                Block {
                    index: 0,
                    timestamp: Zoned::now(),
                    data: vec![],
                    nonce: 0,
                    prior_hash: Hash::empty(),
                }
            ]
        }
    }

    pub fn check_validity(&self) -> bool {
        for window in self.chain.windows(2) {
            let current_block = &window[0];
            let next_block = &window[1];

            if current_block.hash() != next_block.prior_hash {
                return false;
            }
        }

        true
    }

    pub fn mine_block(&mut self, data: Vec<u8>, difficulty: usize) {
        let last_block = self.chain.last().unwrap();

        let mut nonce = 0u128;

        let mut new_block = Block {
            index: last_block.index + 1,
            timestamp: Zoned::now(),
            data,
            nonce: 0,
            prior_hash: last_block.hash(),
        };

        while !new_block.hash().complies_with_difficulty(difficulty) {
            nonce += 1;
            new_block.nonce = nonce;
        }

        self.chain.push(new_block);
    }
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nr_blocks = self.chain.len();
        for (i, block) in self.chain.iter().enumerate() {
            write!(f, "{block}")?;
            if i != nr_blocks - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}