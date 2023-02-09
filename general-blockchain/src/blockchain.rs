use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    // integrity test
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            // 1. index check
            if block.index as usize != i {
                println!("Index mismatch {} != {}",
                    &block.index,
                    &i,
                );
                return false
            // 2. Whether Block's hash fits stored difficulty value(+payload check)
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail");
                return false
            } else if i != 0 {
                // Not genesis block
                // 3. time elapsed or not
                let prev_block = &self.blocks[i-1];
                // It is unlikely for a block to be mined within 1 millisecond.
                // The timestamp is the same as the previous value,
                // but most coins will pass the integrity check only
                // if the block timestamp is greater than the previous block timestamp.
                // 여기서는 빠르게 확인해 보는 것이 목적이기 때문에
                // 난이도를 낮게 설정하면 실패할 수 있음.
                if block.timestamp < prev_block.timestamp {
                    println!("Time did not incerese");
                    return false
                // 4. Check that [block.prev_block_hash] and [previous block.hash] match
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                    return false
                }
            } else {
                // Genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid");
                    return false
                }
            }

        } true
    }
}