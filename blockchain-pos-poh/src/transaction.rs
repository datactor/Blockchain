use std::collections::HashMap;
// use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

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

// Tx가 일반적으로 사용되는 방법
// 1. user가 tx의 인수에 실행하고자 하는 instruction을 추가하여 msg struct를 생성한다.
// 2. 그런 다음 msg를 포함하는 tx struct를 생성하고 개인 키로 서명한다.
// 3. 서명된 tx는 그런 다음 solana 네트워크에 브로드 캐스팅된다.
// 4. 네트워크의 validator는 tx의 서명을 확인하고 tx에 포함된 instruction을 실행한다.
//    msg는 account의 ID, tx가 엑세스 할 수 있는 program_ID 목록,
//    tx에 대한 메타데이터를 포함하는 헤더 역할을 한다.
// 5. 명령이 성공적으로 실행되면 tx가 ledger에 commit되고 user의 account balance가 업데이트 된다.


#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction { // 서명된 집합. 네트워크에 브로드캐스트되는 명령의 수
    pub signatures: Vec<Signature>, // Tx를 인증하는데 사용됨
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub message: Message, // Msg는 실행 중인 명령을 지정함
    pub fee: u64,
    pub fee_payer: Pubkey,
    pub recent_blockhash: Hash, // prevent replay attack
    pub instructions: Vec<CompiledInstruction>,
}

impl Transaction {
    pub fn new(
        signatures: Vec<Signature>,
        sender: Pubkey,
        recipient: Pubkey,
        amount: u64,
        message: Message,
        fee: u64,
        fee_payer: Pubkey,
        recent_blockhash: Hash,
        instruction: Vec<CompiledInstruction>,
    ) -> Self {
        Self {
            signatures,
            sender,
            recipient,
            amount,
            message,
            fee,
            fee_payer,
            recent_blockhash,
            instructions: instruction,
        }
    }

    // // Verify the transaction's signatures
    // pub fn verify_signatures(&self) -> bool {
    //     // Step 1: Get the public keys of all the signatures
    //     let public_keys: Vec<Pubkey> = self
    //         .signatures
    //         .iter()
    //         // .map(|signature| signature.verify(&self.message, &self.sender))
    //         .map(|signature| verify(&self.sender.0, serialize(&self.message).unwrap().try_into().unwrap(), signature))
    //         .filter_map(|result| result)
    //         .collect();
    //     // Step 2: Check that the number of signatures matches the number of public keys
    //     if self.signatures.len() != public_keys.len() {
    //         return false;
    //     }
    //
    //     // Step 3: Check that all the signatures are valid
    //     for (i, signature) in self.signatures.iter().enumerate() {
    //         if !signature.verify(&self.message, &public_keys[i]).is_ok() {
    //             return false;
    //         }
    //     }
    //
    //     true
    // }
    //
    // // Verify the transaction's fee
    // pub fn verify_fee(&self) -> bool {
    //     self.fee_payer == self.sender && self.fee <= self.amount
    // }
    //
    // // Verify the transaction's recent blockhash
    // pub fn verify_recent_blockhash(&self, blockhash: Hash) -> bool {
    //     self.recent_blockhash == blockhash
    // }
    //
    // // Execute the transaction's instructions
    // pub fn execute_instructions(&self, accounts: &mut HashMap<Pubkey, Account>) {
    //     for instruction in &self.instructions {
    //         // Step 1: Get the program account associated with the instruction
    //         let program_account = accounts.get_mut(&instruction.program_id).unwrap();
    //
    //         // Step 2: Execute the instruction on the program account
    //         let result = program_account.execute(instruction.data.clone());
    //
    //         // Step 3: Check that the execution was successful
    //         if !result {
    //             panic!("Instruction execution failed");
    //         }
    //     }
    // }
}


impl Hashable for Transaction {
    fn update(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for signature in &self.signatures {
            bytes.extend(signature.0);
        }
        bytes.extend(&self.sender.0);
        bytes.extend(&self.recipient.0);
        bytes.extend(U64Bytes::from(&self.amount).data);
        // bytes.extend(&self.message.header);
        bytes.extend(U64Bytes::from(&self.fee).data);
        bytes.extend(&self.recent_blockhash.0);
        bytes
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader, // 필수 account address와 메타데이터 저장
    pub account_keys: Vec<Pubkey>, // msg가 의존하는 account address의 배열
    pub recent_blockhash: Hash, // prevent reply attack
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    // 여기의 accounts filed와 data field는 고유한 식별자나 특정 데이터구조에 대한 참조를 나타내지 않음.
    // 즉 고유하게 식별할 필요가 없음. 또한 CompiledInstruction 구조체는 임의의 데이터를 포함할 수 있는
    // 컴파일된 프로그램 명령을 나타내는 데 사용됨. 따라서 보다 일반적인 Vec<u8> type을 사용해
    // 프로그램에 포함해야 하는 모든 종류의 데이터를 수용할 수 있게 함.
}