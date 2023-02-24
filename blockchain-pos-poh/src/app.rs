use super::*;
use std::collections::HashMap;

pub fn run() {
    let genesis = spawn_genesis();
    let genesis_chain = Blockchain::new(genesis.0);




    // let mut tx = Transaction::create(&private_key, &recipient_pubkey, amount, recent_blockhash);
    // tx.sign(&private_key);

}

fn spawn_genesis() -> (Block, AccountSet) {
    // 솔라나는 pos이기 때문에 utxo 대신 account를 기반으로 운영된다.
    // 제네시스 블록이 생성되었을때, 초기 accounts set과 잔액은 제네시스블록에 포함되어 있었지만, 블록에는 tx가 없었다.
    // 제네시스 블록이 생성된 후 account가 생성되고, 트랜잭션이 블록체인에 추가되었음.
    // 즉 account와 balance없이 블록생성과 생성한 블록이 체인에 추가될 수 있어야 함.
    let genesis =
        Block::new(
            [0u8; 64],
            0,
            0,
            Hash([0; 32]),
            HashMap::new(),
            vec![],
            0,
            0,
            0
        );

    let mut accountset = AccountSet::new();

    // Privatekey는 서버에 저장하지 않음. 유저에게 1회 출력해주고 버린다. Privatekey 내부에 있는 Ed25519KeyPair를 사용해
    // scalar와 prefix를 사용해 signature를 확인할 수 있어서 서버측에는 저장할 필요 없게 만듬.
    // but pubkey만으로는 signature를 추출할 수 없기 때문에, signature는 서버에 저장해둔다.

    // note_ 다만 제네시스의 경우에는 tx가 없었고 이 때 생성된 account들은 Privatekey가 없다.(tx가없었기 때문에 필요도 없다.)
    // 그렇지만 account는 생성되었으므로, Pubkey는 생성되어 서버에 저장되어 있다.

    // let sys_key = Privatekey::new();
    // 시스템 프로그램에서 솔라나 블록체인의 state 관리
    let sys_pubkey = Pubkey::new_rand();
    let sys_account = Account::new(1, sys_pubkey.clone(), 1, vec![], false, None);
    accountset.insert_account(sys_pubkey.clone(), sys_account);

    // 필라델피아 민트 동전생산 공장에서 유래. 새로운 SOL 토큰 생성하는데 사용됨. Mint 프로그램에서 관리함.
    // 이 계정의 잔액은 유통되는 총 SOL 토큰 수를 나타냄.
    let mint_pubkey = Pubkey::new_rand();
    let mint_account = Account::new(1_000_000_000_000, mint_pubkey.clone(), 0, vec![], false, None);
    accountset.insert_account(mint_pubkey.clone(), mint_account);

    // 토큰 계정 및 토큰 전송을 관리하는데 사용되는 토큰 프로그램용 smart contract 코드가 포함되어 있음.
    let token_program_pubkey = Pubkey::new_rand();
    let token_program_account = Account::new(0, token_program_pubkey.clone(), 0, vec![], true, None);
    // 새로 생성할 때도 account에 program이 포함되어 있다면 executable: true. smart contract는 프로그램으로 구현되어 있어,
    // account에 load되고 blockchain에서 실행될 수 있음을 나타낸다. smart contract 외에도
    // 블록체인에서 account에 load되고 실행되는 다른 type의 프로그램이 있을 수 있다. 이 경우도 executable field가 true로 생성됨.
    accountset.insert_account(token_program_pubkey.clone(), token_program_account);

    // 솔라나 체인의 rent-exempt reserve 관리 Rent 프로그램용 account
    let rent_sysvar_pubkey = Pubkey::new_rand();
    let rent_sysvar_account = Account::new(0, rent_sysvar_pubkey.clone(), 0, vec![], false, None);
    accountset.insert_account(rent_sysvar_pubkey.clone(), rent_sysvar_account);

    // stake 프로그램에서 stake 계정 및 staking 활동 기록을 유지하는 account
    let stake_history_pubkey = Pubkey::new_rand();
    let stake_history_account = Account::new(0, stake_history_pubkey.clone(), 0, vec![], false, None);
    accountset.insert_account(stake_history_pubkey.clone(), stake_history_account);

    // stake 프로그램에서 staking하기 위한 config parameter를 저장하는데 사용되는 account
    let stake_config_sysvar_pubkey = Pubkey::new_rand();
    let stake_config_sysvar_account = Account::new(0, stake_config_sysvar_pubkey.clone(), 0, vec![], false, None);
    accountset.insert_account(stake_config_sysvar_pubkey.clone(), stake_config_sysvar_account);

    (genesis, accountset)
}


pub fn add_tx() {
    todo!();
}
pub fn validate_tx() {
    todo!();
}

pub fn broadcast() {
    // transaction broadcast


    // block broadcast


    // blockchain broadcast
}

pub fn spawn_block() {
    todo!();
}

pub fn validate_block() {
    todo!();
}

pub fn update_blockcahin() {
    todo!();
}

