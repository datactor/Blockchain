use crate::{Account, Pubkey};
use crate::Mint;
use std::collections::HashMap;

pub struct Token {
    // Solana에서 각 계정은 고유한 주소를 가지며 accountDB는 본질적으로 계정 주소가 키이고 account 데이터가 값인
    // 키-값 데이터베이스이다. 각 account를 iter하지 않고 accountDB에서 총 계정 잔액 합계를 빠르게 얻기 위해
    // Solana는 "account index"라는 별도의 데이터 구조를 사용한다.
    //
    // account index는 유저의 Pubkey를 user가 소유한 account address set에 매핑하는 해시 테이블이다.
    // index를 사용하면 특정 유저가 소유한 모든 토큰 계정을 효율적으로 조회할 수 있다.
    //
    // 토큰의 total supply를 계산할 때 Solana는 먼저 account index에서 토큰에 대한 account address set을 검색한다.
    // 그런 다음 세트의 각 토큰 계정 주소를 iter하고 accountDB에서 각 토큰 account의 balance를 검색한다.
    // 마지막으로 balance을 합산하여 total supply를 얻는다.
    //
    // account index를 사용하여 특정 소유자가 소유한 토큰 계정을 효율적으로 조회함으로써 Solana는 accountDB의
    // 각 계정을 모두 iter할 필요 없이 모든 토큰 계정의 잔액을 신속하게 검색할 수 있다.
    // 그러므로 실제 솔라나에는 supply field가 필요 없다.
    pub supply: u64,
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
            supply: total_supply,
            mint_authority,
            decimals,
            accounts,
        }
    }

    pub fn mint_to(
        &mut self,
        mint: &mut Mint,
        authority: Pubkey, // 새롭게 가져온 mint's pubkey.
        amount: u64
    ) {
        assert_eq!(authority, mint.mint_authority);
        mint.total_supply += amount;
    }

    pub fn transfer(&mut self, mint: &mut Mint, sender: Pubkey, recipient: Pubkey, amount: u64) -> Result<(), String> {
        // Transfer tokens from sender to recipient
        if sender == mint.mint_authority {
            if self.supply < amount {
                return Err(String::from("Not enough supply to transfer"));
            } else {
                self.supply -= amount;
            }
        }
        let sender_balance = self.accounts.entry(sender).or_insert(0);
        if *sender_balance < amount {
            return Err(String::from("Not enough balance to transfer"));
        }
        *sender_balance -= amount;
        let recipient_balance = self.accounts.entry(recipient).or_insert(0);
        *recipient_balance += amount;

        Ok(())
    }

    pub fn get_balance(&self, account: Pubkey) -> u64 {
        // Return the balance of an account
        *self.accounts.get(&account).unwrap_or(&0)
    }

    pub fn burn(&mut self, amount: u64, mint: &mut Mint) {
        // Burn tokens by removing them from the total supply
        assert!(self.supply >= amount, "Not enough total supply to burn");
        self.supply -= amount;
        mint.total_supply -= amount;
    }
}