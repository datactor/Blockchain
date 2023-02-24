use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};

use crate::block::Block;
use crate::Hash;

pub struct Sys {
    pub current_block: Block,
    pub block_hash: HashSet<Hash>,
}

impl Sys {
    pub fn genesis() -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block::new([0u8; 64], 0, 0, Hash([0; 32]), HashMap::new(), vec![], 0, 0, 0);
        block.timestamp = timestamp;

        let mut block_hash = HashSet::new();
        block_hash.insert(block.hash.clone());

        Self {
            current_block: block,
            block_hash,
        }
    }

    pub fn create_block(&mut self) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block::new([0u8; 64], timestamp, 0, self.current_block.hash.clone(), HashMap::new(), vec![], 0, 0, 0);

        // a tiny of PoW. Acts as a spam filter
        while !block.is_valid(0) {
            block.slot += 1;
        }

        self.current_block = block.clone();
        self.block_hash.insert(block.hash.clone());

        block
    }
}