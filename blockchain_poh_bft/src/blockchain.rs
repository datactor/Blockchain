use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub height: u64,
    pub rewards: HashMap<Pubkey, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            height: 0,
            rewards: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
        self.height += 1;
        self.update_rewards();
    }

    fn update_rewards(&mut self) {
        self.rewards.clear();
        for block in &self.blocks {
            let rewards = block.rewards.clone();
            for (pubkey, reward) in rewards {
                let entry = self.rewards.entry(pubkey).or_insert(0);
                *entry += reward;
            }
        }
    }

    pub fn get_balance(&self, pubkey: Pubkey) -> u64 {
        *self.rewards.get(&pubkey).unwrap_or(&0)
    }
}