use chrono::prelude::*;

pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod transaction;
pub mod app;
pub mod account;
pub mod mint;
pub mod sys;
pub mod configmap;
pub mod repl;

pub use crate::{
    block::Block,
    blockchain::Blockchain,
    transaction::Transaction,
    hashable::{Hashable, Hash, Pubkey, Privatekey},
    account::{Account, AccountSet},
    configmap::{cli, db},
    repl::login_menu_main,
};

type Signature = [u8; 64];

pub fn now() -> u128 {
    Utc::now().timestamp_millis() as u128
}

struct U32Bytes {
    data: [u8; 4],
}

struct U64Bytes {
    data: [u8; 8],
}

struct U128Bytes {
    data: [u8; 16],
}

impl From<&u32> for U32Bytes {
    fn from(u: &u32) -> Self {
        U32Bytes { data: u.to_le_bytes() }
    }
}

impl From<&u64> for U64Bytes {
    fn from(u: &u64) -> Self {
        U64Bytes { data: u.to_le_bytes() }
    }
}

impl From<&u128> for U128Bytes {
    fn from(u: &u128) -> Self {
        U128Bytes { data: u.to_le_bytes() }
    }
}