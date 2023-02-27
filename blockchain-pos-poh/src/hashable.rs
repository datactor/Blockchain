use std::ops::Deref;
use super::*;
use rand::{rngs::OsRng, RngCore, Rng};
// Edwards-curve(EdDSA(Elliptic Curve Digital Signature Algorithm))도 determinism을 엄격하게 준수한다.
use ring::signature::{Ed25519KeyPair, KeyPair};
use bs58::{decode, encode};

// SHA256의 digest는 언제나 256bit [u8; 32]를 반환한다.
// 만약 [u8; 128]을 입력으로 둔다면 중복되는 값이 존재하여 충돌하지 않을까?

// The reason that a hash function with an output of 32 bytes can uniquely represent
// inputs of arbitrary size is based on a few different principles:
//
// 1. Collisions: Any hash function may produce the same hash value for two distinct input values,
//    which is called a "collision". A good hash function aims to minimize the likelihood of collisions,
//    but they are still possible.
//
// 2. Avalanche effect: A good hash function is designed such that a small change in the input results
//    in a significant change in the output. This property is known as the "avalanche effect".
//
// 3. Output size: The size of the output of the hash function is fixed (in this case, 32 bytes).
//
// Given these principles, the idea is that even if the input size is much larger than the output size,
// the hash function will generate a hash that is unique to that input.
// While it is theoretically possible to have collisions with a 32-byte hash value,
// the likelihood of such collisions is extremely small,
// especially with high-quality hash functions like SHA256.
//
// So, the hashed result will not be the same number for different inputs,
// even if the inputs are longer than 32 bytes.


pub trait Hashable {
    fn update(&self) -> Vec<u8>; // extend bytes array.

    fn finalize(&self) -> Hash {
        let hash_to_arr = crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &self.update()
        )
            .try_into()
            .expect("Invalid bytes to hash");
        Hash(hash_to_arr)
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub fn new_rand() -> Self {
        let mut rng = OsRng;
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Self(bytes)
    }
}




pub struct Privatekey(Ed25519KeyPair);

impl Privatekey {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let keypair = Ed25519KeyPair::from_seed_unchecked(&seed).unwrap();
        // 솔라나는 keypair로 입력함. seed를 hashing한 친화적이진 않지만 익숙한 String type으로도 키를 배포하지 않음.
        // 왜? 키패어를 사용하면 유저가 키패어를 분실했을 때, 니모닉으로 찾을 수 있는 옵션을 넣을 여지가 존재함.
        // 그렇지만 seed를 해싱한 값만 가지고 지갑 복구에 단일 실패지점을 두는 것은 영구적으로 지갑을 잃을 수 있음.
        let mut privatekey_once = seed.to_vec();
        let public_keys = keypair.public_key().as_ref().to_owned();
        for i in public_keys {
            privatekey_once.push(i);
        }
        let once: [u8; 64] = privatekey_once.try_into().unwrap();

        println!("\n\
                  Do not disclose this key to anyone and store it in a safe place. It is not stored anywhere.\n\
                  \n\
                  keypair: \n\
                  {:?}\n\
                  ", once);
        once.to_vec().clear();

        Privatekey(keypair)
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.0.sign(message).as_ref().to_owned().try_into().unwrap()
    }

    pub fn pubkey(&self) -> Pubkey {
        Pubkey(self.0.public_key().as_ref().to_owned().try_into().unwrap())
    }
}

impl std::fmt::Debug for Privatekey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Privatekey")
            .field(&"Invalid approach")
            .finish()
    }
}


#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Pubkey(pub(crate) [u8; 32]);

impl Pubkey {
    pub fn new(pubkey_bytes: [u8; 32]) -> Self {
        let mut arr = [0u8; 32];
        arr[..pubkey_bytes.len()].copy_from_slice(&pubkey_bytes);
        Pubkey(arr)
    }

    pub fn new_rand() -> Self {
        let mut rng = OsRng;
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Self(bytes)
    }
}

impl std::fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&hex::encode(&self.0))
            .finish()
    }
}