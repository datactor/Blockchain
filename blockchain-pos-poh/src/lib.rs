extern crate core;

use chrono::prelude::*;
// use ed25519_dalek::ed25519::Error;
// use ed25519_dalek::Verifier;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};

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
pub mod token;
pub mod database;

pub use crate::{
    block::Block,
    blockchain::Blockchain,
    transaction::Transaction,
    hashable::{Hashable, Hash, Pubkey, Privatekey},
    account::{Account, AccountSet},
    configmap::{cli, db},
    mint::mint::Mint,
    token::token::Token,
    sys::sys::*,
    repl::login_menu_main,
    transaction::Message,
};

type Signature = [u8; 64];

impl Hashable for Signature {
    fn update(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.as_ref());
        bytes
    }
}

// impl Signature {
//     pub fn verify(&self, message: &Message, pubkey: &Pubkey) -> bool {
//         // 1. Get the bytes of the message and convert them to a digest
//         let message_bytes = bincode::serialize(message).unwrap();
//         let message_digest = hash(&message_bytes);
//
//         // 2. Verify the signature using the public key and digest
//         let signature_bytes = self.as_ref();
//         let pubkey_bytes = pubkey.as_ref();
//         ed25519_dalek::verify(&message_digest, pubkey_bytes, signature_bytes).is_ok()
//     }
// }

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