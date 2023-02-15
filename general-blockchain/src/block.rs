use std::fmt::{ self, Debug, Formatter };
use super::*;

#[derive(Clone)]
pub struct Block {
    pub index: u32, // Bitcoin doesn't have an index field, so instead it contains a field representing the version of the block: 'version: [u8, 32],'
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub merkle_root: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
               &self.index,
               &hex::encode(&self.hash),
               &self.timestamp,
               &self.transactions.len(),
               &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            merkle_root: vec![0; 32],
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    // O(N) N = 2.pow(64)
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::MAX) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        let new_tx_hashes = self.transactions.iter().map(|tx| tx.hash()).collect::<Vec<_>>();
        self.merkle_root = merkle_root(&new_tx_hashes);
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&u32_to_bytes(&self.index));
        bytes.extend(&u128_to_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&self.merkle_root);
        bytes.extend(&u64_to_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>());
        bytes.extend(&u128_to_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

fn merkle_root(hashes: &[Hash]) -> Hash {
    let mut hashes = hashes.to_owned();
    while hashes.len() > 1 {
        // 홀수일 경우 마지막 해시를 벡터에 추가
        if hashes.len() % 2 == 1 {
            hashes.push(hashes.last().unwrap().to_owned());
        }
        let mut new_hashes = vec![];
        for i in (0..hashes.len()).step_by(2) {
            // 쌍을 이뤄주고, extending해서 하나 부모 노드로 만듬
            let mut new_hash = Vec::new();
            new_hash.extend(hashes[i].clone());
            new_hash.extend(hashes[i+1].clone());

            // Merkle 트리의 각 부모 노드는 두 자식 노드의 연결된 hash를 hashing하여 구성된다.
            // extending된 쌍의 hash를 한번 더 hashing하여 부모 노드로 만들어준다.
            new_hash = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &new_hash);

            new_hashes.push(new_hash);
        }
        hashes = new_hashes;
    }
    hashes[0].clone()
}