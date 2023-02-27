use super::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Account {
    balance: u64,
    owner: Pubkey,
    lamports: u64, // 0.000000001 sol
    data: Vec<u8>,
    executable: bool, // account에 실행 가능한 프로그램(e.g. samrt contract)이 포함되어 있는지 여부
    signature: Option<Signature>,
}

impl Account {
    pub fn new(balance: u64, owner: Pubkey, lamports: u64, data: Vec<u8>, executable: bool, signature: Option<Signature>) -> Self {
        Account {
            balance,
            owner,
            lamports,
            data,
            executable,
            signature,
        }
    }
}


// AccountSet must be stored in its own db server(it called ledger).
// The ledger is maintained by the validators in the network, who store and update the ledger on their own servers.
#[derive(Clone)]
pub struct AccountSet {
    accounts: HashMap<Pubkey, Account>,
}

impl AccountSet {
    pub fn new() -> Self {
        AccountSet {
            accounts: HashMap::new(),
        }
    }

    pub fn get_account(&self, pubkey: &Pubkey) -> Option<&Account> {
        self.accounts.get(pubkey)
    }

    fn get_account_mut(&mut self, pubkey: &Pubkey) -> Option<&mut Account> {
        self.accounts.get_mut(pubkey)
    }

    pub fn insert_account(&mut self, pubkey: Pubkey, account: Account) {
        self.accounts.insert(pubkey, account);
    }
}