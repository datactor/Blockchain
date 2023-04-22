use super::*;
use std::collections::HashMap;
use bs58::{decode, encode};
use crate::Token;

pub fn run() {
    // let (sys, sys_account, chain) = Sys::genesis();

    // let genesis = spawn_genesis();
    // let blockchain = Blockchain::genesis(genesis.0);

    let sys = Sys::create_sys_account();
    let blockchain = Sys::genesis();
    let owner = Pubkey::new_rand();
    let (token_id, mint_id) = create_essential_id(&mut sys.unwrap(), owner);

    let mint = Mint::genesis(1_000_000_000_000, owner, 2);
    let token = Token::genesis(mint.total_supply, owner, 2);

    let mut accountset = AccountSet::new();


    ///////////
    let pubkey = Pubkey([0u8; 32]);

    let encoded = EncodedPubkey::from(pubkey.clone());
    println!("encoded: {}", encoded);

    let pubkey2 = encoded.to_pubkey().unwrap();
    println!("decoded: {:?}", pubkey2.0);

    let encoded2 = EncodedPubkey("4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi".to_string());
    println!("decoded: {:?}", encoded2.to_pubkey().unwrap().0);

    let encoded3 = EncodedPubkey("8qbHbw2BbbTHBW1sbeqakYXVKRQM8Ne7pLK7m6CVfeR".to_string());
    println!("decoded: {:?}", encoded3.to_pubkey().unwrap().0);

    let deco = Pubkey::from(encoded3);
    println!("{:?}", deco.0);


    ///////////

    login_menu_main(&mut accountset);

    // 1. The validator requests the network to create a block.
    // 2. The network selects a leader node to create a block.
    // 3. The leader node, which has the sys program installed ,create a block
    //    (Enter pubkey in msg. (Open the account db and find the pubkey as the recipient address (account)).
    // 4. The leader node broadcasts the created block to the network.
    // 5. The network sends the created block to the validator for verification.
    // 6. Validators receive the block from the network and perform PoH validation to verify that
    //    the block was created at a specific time. Multiple validators can verify the same block simultaneously.
    //    multiple validators can verify the same block simultaneously, which helps ensure that
    //    the validation process is efficient and decentralized. Each validator runs the PoH validation
    //    and block validation steps independently, and once a block is verified,
    //    the validator can sign it and send it back to the network.
    //    The validators do not need to wait for each other to complete the validation process,
    //    although they do need to agree on the order in which blocks are added to the blockchain to maintain consensus.
    // 7. When a validator completes the PoH verification, it further verifies the correctness of the block.
    //    Verification of correctness includes verifying the signature of each transaction,
    //    verifying that the transaction is valid and approved (checking that PoH verification
    //    has been completed and that the block conforms to the protocol rules),
    //    and checking that the block's hash satisfies the consensus algorithm requirements.
    //    If all of these are confirmed to be valid, the validator sets the is_confirmed field to true,
    //    signs the block, and sends it back to the network.
    // 8. When the network receives a verified block from the validator, it sends the block to the leader node.
    //    The leader node adds the block to the blockchain and sends the updated blockchain to the network.
    //    Other nodes in the network receive the updated blockchain and verify that
    //    it is a valid extension of the existing blockchain. When a blockchain is confirmed,
    //    it is considered a new valid state of the network.

    // let mut tx = Transaction::create(&private_key, &recipient_pubkey, amount, recent_blockhash);
    // tx.sign(&private_key);



    let pubkey = [0u8; 32];
    let encoded_pubkey = encode(&pubkey).into_string();

    println!("{:?}", pubkey);

    println!("{}", encoded_pubkey);

    let decoded_pubkey: [u8; 32] = decode(&encoded_pubkey).into_vec().unwrap().try_into().unwrap();
    println!("{:?}", decoded_pubkey);

}

// fn spawn_genesis() -> (Block, AccountSet, Mint) {
//     // 솔라나는 pos이기 때문에 utxo 대신 account를 기반으로 운영된다.
//     // 제네시스 블록이 생성되었을때, 초기 accounts set과 잔액은 제네시스블록에 포함되어 있었지만, 블록에는 tx가 없었다.
//     // 제네시스 블록이 생성된 후 account가 생성되고, 트랜잭션이 블록체인에 추가되었음.
//     // 즉 account와 balance없이 블록생성과 생성한 블록이 체인에 추가될 수 있어야 함.
//     let genesis =
//         Block::new(
//             [0u8; 64],
//             0,
//             0,
//             Hash([0; 32]),
//             HashMap::new(),
//             vec![],
//             0,
//             0,
//             0
//         );
//
//     let mut accountset = AccountSet::new();
//
//     // Privatekey는 서버에 저장하지 않음. 유저에게 1회 출력해주고 버린다. Privatekey 내부에 있는 Ed25519KeyPair를 사용해
//     // scalar와 prefix를 사용해 signature를 확인할 수 있어서 서버측에는 저장할 필요 없게 만듬.
//     // but pubkey만으로는 signature를 추출할 수 없기 때문에, signature는 서버에 저장해둔다.
//
//     // note_ 다만 제네시스의 경우에는 tx가 없었고 이 때 생성된 account들은 Privatekey가 없다.(tx가없었기 때문에 필요도 없다.)
//     // 그렇지만 account는 생성되었으므로, Pubkey는 생성되어 서버에 저장되어 있다.
//
//     // let sys_key = Privatekey::new();
//     // 시스템 프로그램에서 솔라나 블록체인의 state 관리
//     let sys_pubkey = Pubkey::new_rand();
//
//     let sys_account = Account::new(1, sys_pubkey.clone(), 1, vec![], false, None);
//     accountset.insert_account(sys_pubkey, sys_account);
//
//     // 필라델피아 민트 동전생산 공장에서 유래. 새로운 SOL 토큰 생성하는데 사용됨. Mint 프로그램에서 관리함.
//     // 이 계정의 잔액은 유통되는 총 SOL 토큰 수를 나타냄.
//
//     // 민트 계정은 별개로 token계정을 통해서 생성되지 않고, 토큰계정보다 먼저 생성시킨 후 민트의 값들을 토큰 계정에 인자로
//     // 넣어 토큰계정을 생성해 연동시킨다.
//     let mint_privatekey = Privatekey::new();
//     let mint_pubkey = mint_privatekey.pubkey();
//     let mint_program = Mint::genesis(1_000_000_000_000, mint_pubkey, 2); // Assuming you want to use 2 decimals
//     let mint_program_account = Account::new(mint_program.total_supply, mint_program.mint_authority, 0, vec![], false, None);
//     accountset.insert_account(mint_pubkey, mint_program_account);
//
//
//     // 토큰 계정 및 토큰 전송을 관리하는데 사용되는 토큰 프로그램용 smart contract 코드가 포함되어 있음.
//     let token_program_privatekey = Privatekey::new();
//     let token_program_pubkey = token_program_privatekey.pubkey();
//     let token_program = Token::genesis(mint_program.total_supply, mint_pubkey, 2);
//     let token_program_account = Account::new(0, token_program_pubkey, 0, vec![], true, None);
//     // 새로 생성할 때도 account에 program이 포함되어 있다면 executable: true. smart contract는 프로그램으로 구현되어 있어,
//     // account에 load되고 blockchain에서 실행될 수 있음을 나타낸다. smart contract 외에도
//     // 블록체인에서 account에 load되고 실행되는 다른 type의 프로그램이 있을 수 있다. 이 경우도 executable field가 true로 생성됨.
//     accountset.insert_account(token_program_pubkey, token_program_account);
//
//     // 솔라나 체인의 rent-exempt reserve 관리 Rent 프로그램용 account
//     let rent_sysvar_pubkey = Pubkey::new_rand();
//     let rent_sysvar_account = Account::new(0, rent_sysvar_pubkey.clone(), 0, vec![], false, None);
//     accountset.insert_account(rent_sysvar_pubkey, rent_sysvar_account);
//
//     // stake 프로그램에서 stake 계정 및 staking 활동 기록을 유지하는 account
//     let stake_history_pubkey = Pubkey::new_rand();
//     let stake_history_account = Account::new(0, stake_history_pubkey.clone(), 0, vec![], false, None);
//     accountset.insert_account(stake_history_pubkey, stake_history_account);
//
//     // stake 프로그램에서 staking하기 위한 config parameter를 저장하는데 사용되는 account
//     let stake_config_sysvar_pubkey = Pubkey::new_rand();
//     let stake_config_sysvar_account = Account::new(0, stake_config_sysvar_pubkey.clone(), 0, vec![], false, None);
//     accountset.insert_account(stake_config_sysvar_pubkey, stake_config_sysvar_account);
//
//     (genesis, accountset, mint_program)
// }


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