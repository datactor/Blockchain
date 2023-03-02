// use super::super::*; // not idiomatic

use crate::{Pubkey, Token};

pub struct Mint {
    pub total_supply: u64,
    pub mint_authority: Pubkey,
    pub decimals: u8,
    // pub is_initialized: bool, // 버전 변경이나, 프로그램의 새 instance를 생성해야 하는 중요한 사항에서만
    // true로 mint program이 토큰을 만들고 발행할 준비가 되었음을 나타낸다. 솔라나는 genesis 이후로 현재까지 false이다.
}

impl Mint {
    pub fn genesis(total_supply: u64, mint_authority: Pubkey, decimals: u8) -> Self {
        Self {
            total_supply,
            mint_authority,
            decimals,
        }
    }

    pub fn mint(&mut self, token: &mut Token, authority: Pubkey, amount: u64) {
        assert_eq!(authority, self.mint_authority);
        self.total_supply += amount;
        let mut mint_balance = token.accounts.get_mut(&self.mint_authority).expect("Mint account balance does not exist.");
        *mint_balance += amount;
    }

    // mint to recipients
    pub fn mint_to(&mut self, token: &mut Token, recipient: Pubkey, amount: u64) -> Result<(), String> {
        let mint_balance = token.accounts.get_mut(&self.mint_authority).expect("Mint account balance does not exist.");
        if *mint_balance < amount {
            return Err(String::from("Not enough supply to transfer"));
        }

        *mint_balance -= amount;

        let recipient_balance = token.accounts.entry(recipient).or_insert(0);
        *recipient_balance += amount;
        Ok(())
    }

    pub fn burn(&mut self, token: &mut Token, amount: u64) {
        // Burn tokens by removing them from the total supply
        let mint_balance = token.accounts.get_mut(&self.mint_authority).expect("Mint account balance does not exist.");
        self.total_supply -= amount;
        *mint_balance -= amount;
    }
}