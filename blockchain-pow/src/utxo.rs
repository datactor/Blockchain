use super::*;
use std::collections::HashMap;
use crate::blockchain::BlockValidationErr;

#[derive(Debug, Clone)]
pub struct Utxo {
    pub value: u64,
    script_pubkey: String,
}

#[derive(Debug)]
pub struct UtxoSet {
    pub utxos: HashMap<String, Utxo>,
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, txid: String, output_index: usize, value: u64, script_pubkey: String) {
        let utxo = Utxo {
            value,
            script_pubkey,
        };
        let key = format!("{}:{}", txid, output_index);
        self.utxos.insert(key, utxo);
    }

    pub fn spend(&mut self, txid: String, output_index: usize) -> Result<(), BlockValidationErr> {
        let key = format!("{}:{}", txid, output_index);
        match self.utxos.remove(&key) {
            None => return Err(BlockValidationErr::UtxoSpentFailure),
            _ => Ok(())
        }
    }

    pub fn get_balance(&self) -> u64 {
        let mut balance = 0;
        for (_, utxo) in &self.utxos {
            balance += utxo.value;
        }
        balance
    }

    pub fn get_optimal_inputs(&self, target_value: u64) -> Result<Vec<(String, usize, u64, String)>, BlockValidationErr> {
        // First, sort the UTXOs by value in descending order
        let mut utxos: Vec<(&String, &Utxo)> = self.utxos.iter().collect();
        utxos.sort_by(|a, b| b.1.value.cmp(&a.1.value));

        // Next, iterate over the UTXOs to find the optimal inputs
        let mut total_value = 0;
        let mut optimal_inputs = Vec::new();
        for (txo_id, utxo) in utxos {
            if total_value > target_value {
                // If we have already accumulated enough value, we can stop
                break;
            }
            let val = utxo.value;
            total_value += val;
            let mut txo_id_and_idx = txo_id.split(":");
            optimal_inputs.push((txo_id_and_idx.next().unwrap().to_owned(), txo_id_and_idx.next().unwrap().parse::<usize>().unwrap(), val, utxo.script_pubkey.clone()));
        }

        if total_value < target_value {
            // If we couldn't accumulate enough value, return Err
            return Err(BlockValidationErr::InsufficientInputValue);
        }

        Ok(optimal_inputs)
    }
}