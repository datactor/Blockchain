use super::*;
use std::collections::HashMap;

pub fn run() {
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
        0);

    let private_key = Privatekey::new();
    println!("{:?}", private_key);
    let recipient_pubkey = Pubkey::new_rand();
    let amount = 100;
    let recent_blockhash = Hash::new_rand();

    // let mut tx = Transaction::create(&private_key, &recipient_pubkey, amount, recent_blockhash);
    // tx.sign(&private_key);

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