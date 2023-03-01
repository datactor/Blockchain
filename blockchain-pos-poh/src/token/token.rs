use crate::{Account, Pubkey};
use crate::Mint;
use std::collections::HashMap;

pub struct Token {
    pub total_supply: u64,
    pub mint_authority: Pubkey,
    pub decimals: u8,
    pub accounts: HashMap<Pubkey, u64>,
}

impl Token {
    pub fn genesis(total_supply: u64, mint_authority: Pubkey, decimals: u8) -> Self {
        let mut accounts = HashMap::new();

        // mint_authority는 최초의 genesis 실행때만 필요하다(제네시스로 token account 생성시에만 필요).
        accounts.insert(mint_authority.clone(), total_supply);

        Self {
            total_supply,
            mint_authority,
            decimals,
            accounts,
        }
    }

    pub fn mint_to(
        &mut self,
        mint: &mut Mint,
        recipient: Pubkey,
        authority: Pubkey, // 새롭게 가져온 mint's pubkey.
        amount: u64
    ) {
        assert_eq!(authority, mint.mint_authority);
        // Mint new tokens to the recipient
        self.total_supply += amount;
        let recipient_balance = self.accounts.entry(recipient).or_insert(0);
        *recipient_balance += amount;
    }

    // pub fn transfer(&mut self, sender: Pubkey, recipient: Pubkey, amount: u64) {
    //     // Transfer tokens from sender to recipient
    //     let sender_balance = self.accounts.entry(sender).or_insert(0);
    //     let recipient_balance = self.accounts.entry(recipient).or_insert(0);
    //     assert!(*sender_balance >= amount, "Not enough balance to transfer");
    //     *sender_balance -= amount;
    //     *recipient_balance += amount;
    // }

    pub fn get_balance(&self, account: Pubkey) -> u64 {
        // Return the balance of an account
        *self.accounts.get(&account).unwrap_or(&0)
    }

    pub fn burn(&mut self, amount: u64) {
        // Burn tokens by removing them from the total supply
        assert!(self.total_supply >= amount, "Not enough total supply to burn");
        self.total_supply -= amount;
    }
}