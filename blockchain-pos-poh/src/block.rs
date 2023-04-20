use std::{
    fmt::{self, Debug, Formatter, Result},
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    // version: u64,
    signature: Signature,
    // fees: u64,
    pub(crate) slot: u64, // index
    // skipped_slots: u64,
    pub(crate) timestamp: u64,
    parent_timestamp: u64,

    // The hash of the root of the transaction merkle tree
    transaction_root: Hash,

    // // The hash of the root of the bank state tree
    // results_root: Hash,

    // // The hash of the root of the vote accounts tree
    // votes: Hash,

    pub prev_block_hash: Hash,
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
        parent_timestamp: u64,
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
            parent_timestamp,
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

    pub fn verify_tiny_pow(&self, difficulty: u64) -> bool {
        self.update();
        let hash = self.finalize();

        let hash_bits = hash.0.iter().fold(0, |acc, &b| acc + b.count_ones());
        u64::from(hash_bits) >= difficulty
    }

    // Verify the PoS
    fn verify_pos(&self, previous_block: &Block) -> bool {
        // Calculate the total stake and working stake
        let mut total_stake = 0;
        let mut working_stake = 0;

        for (pubkey, stake) in &self.rewards {
            total_stake += stake;
            if previous_block.rewards.get(pubkey).unwrap_or(&0) > stake {
                working_stake += stake;
            } else {
                working_stake += previous_block.rewards.get(pubkey).unwrap_or(&0);
            }
        }

        // Check if the working stake is greater than the threshold
        working_stake > total_stake / 2
    }

    // Verify the PoH (Proof of History)
    fn verify_poh(&self) -> bool {
        // Verify that the block's slot is greater than the slot of the previous block
        if self.timestamp <= self.parent_timestamp {
            return false
        }

        // Verify that the block's hash is correct by recomputing it
        let mut hash = self.finalize();
        if hash != self.hash {
            return false;
        }

        true
    }

    // Verify the transactions
    // fn verify_transactions(&self) -> bool {
    //     // Verify that each transaction in the block is valid
    //     for transaction in &self.transactions {
    //         if !transaction.verify() {
    //             return false;
    //         }
    //     }
    //
    //     true
    // }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            signature: Signature([0u8; 64]),
            slot: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            parent_timestamp: 0,
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

        bytes.extend(&self.signature.0);
        bytes.extend(U64Bytes::from(&self.slot).data);
        bytes.extend(U64Bytes::from(&self.parent_timestamp).data);
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