use std::{
    fmt::{self, Debug, Formatter, Result},
    collections::HashMap,
};

use super::*;

#[derive(Clone)]
pub struct Block {
    // version: u64,
    signature: Signature,
    // fees: u64,
    pub(crate) slot: u64, // index
    // skipped_slots: u64,
    pub(crate) timestamp: u64,
    parent_slot: u64,

    // The hash of the root of the transaction merkle tree
    transaction_root: Hash,

    // // The hash of the root of the bank state tree
    // results_root: Hash,

    // // The hash of the root of the vote accounts tree
    // votes: Hash,

    prev_block_hash: Hash,
    pub rewards: HashMap<Pubkey, u64>,
    is_confirmed: bool,
    pub(crate) hash: Hash,
    transactions: Vec<Transaction>,
    // transaction_count: u64,
    working_stake: u64,
    total_stake: u64,
    block_height: u64,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Block[{}]: {} at: {} with: {}, total: {}",
            &self.slot,
            &hex::encode(&self.hash.0),
            &self.timestamp,
            &self.working_stake,
            &self.total_stake,
        )
    }
}

impl Block {
    // "constructor arguments should define the object's required state"
    pub fn new(
        signature: Signature,
        slot: u64, // index
        timestamp: u64,
        prev_block_hash: Hash,
        rewards: HashMap<Pubkey, u64>,
        transactions: Vec<Transaction>,
        // transaction_count: u64,
        working_stake: u64,
        total_stake: u64,
        block_height: u64,
    ) -> Self {
        Block {
            signature,
            slot,
            parent_slot: 0,
            timestamp,
            transaction_root: Hash([0; 32]),
            is_confirmed: false,
            prev_block_hash,
            rewards,
            hash: Hash([0; 32]),
            transactions,
            // transaction_count: transactions.len() as u64,
            working_stake,
            total_stake,
            block_height,
        }
    }

    pub fn create(&mut self, parent_block_hash: Hash, block_height: u64) {
        self.parent_slot = self.slot;
        self.slot += 1;
        self.block_height = block_height;
        self.prev_block_hash = parent_block_hash;
        self.hash = self.finalize();
    }

    pub fn is_valid(&self, difficulty: u64) -> bool {
        self.update();
        let hash = self.finalize();

        let hash_bits = hash.0.iter().fold(0, |acc, &b| acc + b.count_ones());
        u64::from(hash_bits) >= difficulty
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            signature: [0u8; 64],
            slot: 0,
            timestamp: 0,
            parent_slot: 0,
            transaction_root: Hash([0u8; 32]),
            prev_block_hash: Hash([0u8; 32]),
            rewards: HashMap::new(),
            is_confirmed: false,
            hash: Hash([0u8; 32]),
            transactions: Vec::new(),
            working_stake: 0,
            total_stake: 0,
            block_height: 0,
        }
    }
}

impl Hashable for Block {
    fn update(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        //             transactions,
        //             transaction_count: transactions.len() as u64,

        bytes.extend(&self.signature);
        bytes.extend(U64Bytes::from(&self.slot).data);
        bytes.extend(U64Bytes::from(&self.parent_slot).data);
        bytes.extend(U64Bytes::from(&self.timestamp).data);
        bytes.extend(&self.transaction_root.0);
        if self.is_confirmed {
            bytes.push(0x01);
        } else {
            bytes.push(0x00);
        }
        bytes.extend(&self.prev_block_hash.0);

        let mut rewards_map_keys = self.rewards.keys()
            .map(|pubkey| pubkey.0)
            .collect::<Vec<[u8; 32]>>();
        rewards_map_keys.sort();

        // Append the serialized key-value pairs to the byte vector
        for key in rewards_map_keys {
            let value = self.rewards.get(&Pubkey(key)).unwrap();
            bytes.extend(&key);
            bytes.extend(&U64Bytes::from(value).data);
        }
        // bytes.extend(
        //     self.transactions
        //         .iter()
        //         .flat_map(|transaction| transaction.update())
        //         .collect::<Vec<u8>>());
        bytes.extend(U64Bytes::from(&self.working_stake).data);
        bytes.extend(U64Bytes::from(&self.total_stake).data);
        bytes.extend(U64Bytes::from(&self.block_height).data);

        bytes
    }
}