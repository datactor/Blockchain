use super::*;
use std::collections::{HashMap, HashSet};

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
    InvalidMerkleRoot,
    UtxoSpentFailure,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub tip: Hash, // self.chain.last().unwrap()과 같음. 그럼에도 넣은 이유는? 최신 유효 블록에 빠르게 엑세스하기 위함.
}                  // chain.last()를 불러오기 위해 전체 chain을 메모리에 올리는 과정 생략.

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![],
            tip: vec![],
        }
    }

    pub fn spawn_block(&self, difficulty: u128, sender: String, recipient: String, mut amount: u64, utxo_set: &UtxoSet) -> Block {
        let fee = 0;

        let block_reward = 7; // 블록보상 6.25 + 추가적인 transaction fee
        // println!("{:?}", val);

        let mut block = Block::new(
            self.chain.last().unwrap().index + 1,
            now(),
            self.chain.last().unwrap().hash.clone(),
            vec![],
            difficulty
        );

        // coinbase transaction
        // 블록을 생성한 광부. 마이닝 해서 블록체인에 붙이려고 시도한다.
        // 이 coinbase tx의 sender도 광부, recipient도 광부. coinbase address라고 불린다.
        let coinbase_tx = Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "coinbase_miner".to_owned(),
                    value: block_reward,
                },
            ],
        };

        block.add_transaction(coinbase_tx);

        let inputs = utxo_set.get_optimal_inputs(amount).expect("Insufficient UTXO");

        let mut sub_amount = amount;
        for (txid, idx, input_amount, script_pubkey) in inputs {
            let txid_idx = format!("{}:{}", txid, idx);
            let input_to_addr = script_pubkey.split(":").nth(1).unwrap();
            if input_amount < amount {
                sub_amount = input_amount + fee;
                amount -= sub_amount;
            }

            let mut outputs = vec![
                transaction::Output {
                    to_addr: recipient.clone(),
                    value: sub_amount,
                }
            ];

            if input_amount > sub_amount {
                // change.
                // btc network에서 요구하는 대로 Input의 총 가치가 출력의 총 가치와 동일하도록 하기 위해
                // 본인에게 반환되는 Output 추가.
                outputs.push(
                    transaction::Output {
                        to_addr: sender.clone(),
                        value: input_amount - sub_amount,
                    },
                )
            };

            let mut inputs = Vec::new();
            inputs.push((
                transaction::Output {
                    to_addr: input_to_addr.to_owned(),
                    value: input_amount,
                }, txid_idx
            ));

            let transaction = Transaction {
                inputs,
                outputs,
            };

            block.add_transaction(transaction);
        }

        block.clone()
    }

    // integrity test
    pub fn update_with_block(&mut self, block: Block, utxo_set: &mut UtxoSet, sender: &String) -> Result<(), BlockValidationErr> {
        let i = self.chain.len();

        // 1. index check
        if block.index as usize != i {
            println!("Index mismatch {} != {}", &block.index, &i);
            return Err(BlockValidationErr::MismatchedIndex) // 2. Whether Block's hash fits stored difficulty value(+payload check)
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash)
        } else if i != 0 {
            // Not genesis block
            // 3. time elapsed or not
            let prev_block = &self.chain[i-1];
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
            self.tip = block.prev_block_hash.clone();
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
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            }
            // let mut block_spent: HashSet<Hash> = HashSet::new();
            // let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            // get coinbase txid
            // println!("Coinbase TxId: {}", coinbase_txid);

            let hashed_coinbase_tx = coinbase.hash();
            let coinbase_txid = &hex::encode(hashed_coinbase_tx);
            for (output_index, output) in coinbase.outputs.iter().enumerate() {
                let script_pubkey = format!("{}:{}", &output.to_addr, &output.to_addr); // coinbase_tx의 sender와 recipient는 모두 같은 광부임
                utxo_set.add_utxo(coinbase_txid.clone(), output_index, output.value, script_pubkey.to_owned());
            }

            for transaction in transactions {
                // utxo set에 추가.
                let txid = &hex::encode(transaction.hash());
                for (output_index, output) in transaction.outputs.iter().enumerate() {
                    let script_pubkey = format!("{}:{}", sender, &output.to_addr);
                    utxo_set.add_utxo(txid.clone(), output_index, output.value, script_pubkey.to_owned());
                }

                // let input_hashes = transaction.input_hashes();
                //
                // if !(&input_hashes - &self.unspent_outputs).is_empty() ||
                //     !(&input_hashes & &block_spent).is_empty() {
                //     return Err(BlockValidationErr::InvalidInput)
                // }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                // block_spent.extend(input_hashes);
                // block_created.extend(transaction.output_hashes());
                //
                // // get txid
                // let hashed_tx = transaction.hash();
                // // let txid = &hex::encode(hashed_tx);
                // // // println!("TxId: {}", txid);

                // remove used UTXOs.
                for (_, txid_index) in transaction.inputs.iter() {
                    let mut parts = txid_index.split(":");
                    utxo_set.spend(parts.next().unwrap().to_owned(), parts.next().unwrap().parse::<usize>().unwrap()).expect("Utxo does not exist");
                }
            }

            // if coinbase.output_value() < total_fee {
            //      return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            // } else {
            //     block_created.extend(coinbase.output_hashes());
            // }

            // // unspent_output인 것만 남기기
            // self.unspent_outputs.retain(|output| !block_spent.contains(output));
            //
            // self.unspent_outputs.extend(block_created);

            for utxo in &utxo_set.utxos {
                println!("{:?}", utxo);
            }
        }

        self.chain.push(block);

        Ok(())
    }
}