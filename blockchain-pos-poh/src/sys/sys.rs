use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};

use crate::block::Block;
use crate::Hash;

pub struct Sys {
    pub current_block: Block,
    pub block_hash: HashSet<Hash>,
}

impl Sys {
    // leader node's work
    pub fn genesis() -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block::new(
            [0u8; 64],
            0,
            timestamp,
            Hash([0; 32]),
            HashMap::new(),
            vec![],
            0,
            0,
            0
        );
        block.timestamp = timestamp;

        let mut block_hash = HashSet::new();
        block_hash.insert(block.hash.clone());

        Self {
            current_block: block,
            block_hash,
        }
    }

    // leader node's work. 동시에 여러 노드가 진행할 수 있음.
    pub fn create_block(&mut self) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block::new(
            [0u8; 64],
            self.current_block.slot,
            timestamp,
            self.current_block.hash.clone(),
            HashMap::new(),
            vec![],
            0,
            0,
            0
        );

        // a tiny of PoW. Acts as a spam filter.
        // 단일 노드에서 너무 많은 블록이 생성되는 것을 방지하기 위한 스팸필터.
        // 블록을 생성하는 노드(leader node)가 이를 위해 일정 계산 리소스를 사용했는지 확인함.
        while !block.is_valid(0) {
            block.slot += 1;
        }

        block
    }

    // pub fn update_chain(&mut self) {
    //     self.current_block = block.clone();
    //     self.block_hash.insert(block.hash.clone());
    //
    //     // blockchain.push(self.current_block)
    // }
}