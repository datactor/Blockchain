use std::str::FromStr;
use chrono::prelude::*;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};
use serde::{Serialize, Deserialize};
use serde_json::{Serializer, Deserializer};

pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod transaction;
pub mod app;
pub mod account;
pub mod programs;
pub mod configmap;
pub mod repl;
pub mod repl2;
pub mod database;
pub mod nodes;
pub mod shardpath;
pub mod app2;
// pub mod shardable;
pub mod rate_limiter;
pub mod entrypoint;

pub use crate::{
    block::Block,
    blockchain::Blockchain,
    transaction::Transaction,
    hashable::{Hashable, Hash, Pubkey, Privatekey},
    account::{Account, AccountSet},
    configmap::{cli, db},
    programs::{
        sys::{
            Sys,
            create_essential_id,
            ProgramAccount,
        },
        token::{self, Token},
        mint::{self, Mint}
    },
    repl::login_menu_main,
    transaction::Message,
    database::{Database, DBPool, DBHandler},
    shardpath::ShardPath,
    rate_limiter::RateLimiter,
    // shardable::ShardDB,
    entrypoint::ProgramResult,
};

// type Signature = [u8; 64];

#[derive(Debug)]
struct SignatureError;

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid signature")
    }
}

impl std::error::Error for SignatureError {}

#[derive(Clone, Debug)]
pub struct Signature(pub [u8; 64]);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct EncodedPubkey(pub String);

impl From<Pubkey> for EncodedPubkey {
    fn from(pubkey: Pubkey) -> EncodedPubkey {
        EncodedPubkey(bs58::encode(pubkey.0).into_string())
    }
}

impl EncodedPubkey {
    pub fn to_pubkey(&self) -> Result<Pubkey, String> {
        let decoded = bs58::decode(&self.0)
            .into_vec()
            .map_err(|e| format!("Error decoding EncodedPubkey: {}", e))?;
        if decoded.len() != 32 {
            return Err("Invalid length for decoded EncodedPubkey".to_string());
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&decoded);
        Ok(Pubkey(bytes))
    }
}

impl std::fmt::Display for EncodedPubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}


impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let hex_str = hex::encode(self.0);
        serializer.serialize_str(&hex_str)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let hex_str = String::deserialize(deserializer)?;
        let bytes = hex::decode(hex_str)
            .map_err(|_e| serde::de::Error::custom(SignatureError))?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom(SignatureError));
        }
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&bytes[..]);
        Ok(Signature(arr))
    }
}

impl Hashable for Signature {
    fn update(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.0.as_ref());
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