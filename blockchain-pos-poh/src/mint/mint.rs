// use super::super::*; // not idiomatic

use crate::Pubkey;

pub struct Mint {
    pub total_supply: u64,
    pub mint_authority: Pubkey,
    pub decimals: u8,
    // pub is_initialized: bool, // 버전 변경이나, 프로그램의 새 instance를 생성해야 하는 중요한 사항에서만
    // // true로 바꿔 mint program이 토큰을 만들고 발행할 준비가 되었음을 나타낸다. 솔라나는 genesis 이후로 현재까지 false이다.
}

impl Mint {
    pub fn genesis(total_supply: u64, mint_authority: Pubkey, decimals: u8) -> Self {
        Self {
            total_supply,
            mint_authority,
            decimals,
        }
    }

    pub fn mint(&mut self, recipient: Pubkey, amount: u64) {
        // Mint new tokens to mint's account by increasing total supply(mint balance).
        // The added supply will be managed by the token.
        self.total_supply += amount;
    }

    pub fn burn(&mut self, amount: u64) {
        // Burn tokens by removing them from the total supply
        self.total_supply -= amount;
    }
}