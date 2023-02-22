use super::*;
use std::collections::HashMap;

pub fn run() {
    let genesis =
    Block::new(
        [0u8; 64],
        0,
        0,
        vec![],
        HashMap::new(),
        vec![],
        0,
        0,
        0);

}

pub fn add_tx() {
    todo!(트랜잭션 생성);
}
pub fn validate_tx() {
    todo!(트랜잭션 검증);
}

pub fn broadcast() {
    // transaction broadcast


    // block broadcast


    // blockchain broadcast
}

pub fn spawn_block() {
    todo!(블록 생성)
}

pub fn validate_block() {
    todo!(블록 유효성 검사)
}

pub fn update_blockcahin() {
    todo!(blockchain에 추가)
}