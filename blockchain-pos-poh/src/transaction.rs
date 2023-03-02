use super::*;

// leader node가 account db를 조회하지 않고 Pubkey와 balance를 먼저 넣을 방법이 있어야함
// 1. App client: app 로그인 정보에 pubkey가 나와있음.
// 2. Cache 사용. 이전에 이용한 적 있던 계정은 state를 저장해둔다(주의 필요), Redis, Memcached같은 분산 캐시 사용.
// 3. Bloom filter: 간소화된 light account db 추가. light account db를 추가해 자체적 부담을 덜어준다.

// 또는 잠재적 성능 향상 요소
// 1. pre-fetching: tx가 제출되기 기다리지 않고, leader node들은 tx를 예상해 account db를 가져와 메모리에 올려 놓는다.
// 2. parallel processing: multi-processing, 또는 distributed computing을 사용해 tx를 병렬처리하기
// 3. client측 caching: leader node에 의존하여 account db를 조회하는 대신 client는 accountset을 캐싱할 수 있음.
// 4. opt-accountDB: 더 효율적인 데이터 구조사용, 인덱스 수 감소, 확장성 향상을 위한 분산 데이터베이스 사용
// 5. Offloading to specialized hardware: FPGA 또는 ASIC 같은 특수 하드웨어로 account data 처리를 오프로딩.

#[derive(Clone)]
pub struct Transaction {
    pub signatures: Vec<Signature>,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub message: Message,
    pub fee: u64,
    pub recent_blockhash: Hash,
}

impl Transaction {
    pub fn new(signatures: Vec<Signature>,
               sender: Pubkey,
               recipient: Pubkey,
               amount: u64,
               message: Message,
               fee: u64,
               recent_blockhash: Hash)
        -> Self {
        Self {
            signatures,
            sender,
            recipient,
            amount,
            message,
            fee,
            recent_blockhash,
        }
    }

    // pub fn verify(&self) -> bool {
    //     // Verify the signature of the transaction's message
    //     if !self.message.verify_signature() {
    //         return false;
    //     }
    //
    //     // Verify the signature of each account's signature
    //     for account in &self.accounts {
    //         if !account.verify_signature() {
    //             return false;
    //         }
    //     }
    //
    //     // Verify the Merkle proof of the transaction
    //     if let Some(ref proof) = self.merkle_proof {
    //         if !proof.verify(self.finalize(), &self.message) {
    //             return false;
    //         }
    //     }
    //
    //     true
    // }

    // // Verify the signature of the transaction
    // pub fn verify_signature(&self) -> bool {
    //     for signature in &self.signatures {
    //         if !self.sender.verify(&self.hash(), &signature) {
    //             return false;
    //         }
    //     }
    //     true
    // }

    // pub fn create(private_key: &Privatekey,
    //               recipient_pubkey: &Pubkey,
    //               amount: u64,
    //               recent_blockhash: Hash) -> Self {
    //     let message = Message::new(
    //         &[private_key.pubkey(), recipient_pubkey],
    //         Some(&private_key.pubkey()),
    //         vec![Instruction::new_system_transfer(
    //             &private_key.pubkey(),
    //             recipient_pubkey,
    //             amount,
    //         )],
    //     );
    //     let signatures = vec![private_key.sign(&message.serialize())];
    //     let fee = 0;
    //     Self::new(signatures, message, fee, recent_blockhash)
    // }
    //
    // pub fn sign(&mut self, private_key: &Privatekey) {
    //     let message_bytes = self.message.serialize();
    //     self.signatures.push(private_key.sign(&message_bytes));
    // }
    //
    // pub fn verify(&self) -> bool {
    //     let message_bytes = self.message.serialize();
    //     for signature in &self.signatures {
    //         if !signature.verify(&message_bytes, &self.message.account_keys[0]) {
    //             return false;
    //         }
    //     }
    //     true
    // }
}


impl Hashable for Transaction {
    fn update(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for signature in &self.signatures {
            bytes.extend(signature);
        }
        bytes.extend(&self.sender.0);
        bytes.extend(&self.recipient.0);
        bytes.extend(U64Bytes::from(&self.amount).data);
        // bytes.extend(&self.message.header);
        bytes.extend(U64Bytes::from(&self.fee).data);
        // bytes.extend(&self.recent_blockhash);
        bytes
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