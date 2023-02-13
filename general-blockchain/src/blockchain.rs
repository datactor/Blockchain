use super::*;
use std::collections::HashSet;

// custom Error type
#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    // integrity test
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        // 1. index check
        if block.index as usize != i {
            println!("Index mismatch {} != {}", &block.index, &i);
            return Err(BlockValidationErr::MismatchedIndex) // 2. Whether Block's hash fits stored difficulty value(+payload check)
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash)
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
                return Err(BlockValidationErr::AchronologicalTimestamp) // 4. Check that [block.prev_block_hash] and [previous block.hash] match
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash)
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat)
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            // 비트코인의 경우 transaction field(Vec<transaction>)에 항상 coinbase transaction이 포함되어 있다.
            // 왜? coinbase transaction은 블록을 블록체인에 추가하는 채굴자에게 "인센티브"를 주는 역할을 하기 때문에 항상 포함되어 있음.
            // 현재 기준으로 block 보상인 6.25 btc는 fixed specified 되어 있고, 선택적으로 다른 TX에서 발생하는
            // TX fee도 포함되어 있을 수 있다.
            if !coinbase.is_coinbase() {
                // if block.index == 0 {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
                // }
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 1;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationErr::InvalidInput)
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                 return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            // unspent_output인 것만 남기기
            self.unspent_outputs.retain(|output| !block_spent.contains(output));

            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}