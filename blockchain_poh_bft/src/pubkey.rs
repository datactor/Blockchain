
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new(pubkey_bytes: [u8; 32]) -> Self {
        let mut arr = [0u8; 32];
        arr[..pubkey_bytes.len()].copy_from_slice(&pubkey_bytes);
        Pubkey(arr)
    }
}

impl std::fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&hex::encode(&self.0))
            .finish()
    }
}