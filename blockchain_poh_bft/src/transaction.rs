use super::*;

#[derive(Clone)]
pub struct Transaction {
    pub signatures: Vec<Signature>,
    pub message: Message,
    pub fee: u64,
    pub recent_blockhash: Hash,
}

impl Hashable for Transaction {
    fn update(&self) -> Vec<u8> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: Hash,
    pub instructions: Vec<CompiledInstruction>,
}

#[derive(Clone)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Clone)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    // 여기의 accounts filed와 data field는 고유한 식별자나 특정 데이터구조에 대한 참조를 나타내지 않음.
    // 즉 고유하게 식별할 필요가 없음. 또한 CompiledInstruction 구조체는 임의의 데이터를 포함할 수 있는
    // 컴파일된 프로그램 명령을 나타내는 데 사용됨. 따라서 보다 일반적인 Vec<u8> type을 사용해
    // 프로그램에 포함해야 하는 모든 종류의 데이터를 수용할 수 있게 함.
}