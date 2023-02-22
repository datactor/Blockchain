use super::*;

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
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.update())
    }
}