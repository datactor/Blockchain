use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};

use crate::block::Block;
use crate::{Blockchain, Hash, Pubkey, Token};

#[derive(Clone)]
pub struct ProgramAccount {
    pub lamports: u64,
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub executable: bool,
}

pub struct Sys {
    pub current_block: Block,
    pub block_hash: HashSet<Hash>,
    pub program_accounts: HashMap<Pubkey, ProgramAccount>
}

impl Sys {
    pub fn create_sys_account() -> Option<Sys> {
        let sys_program_id = Pubkey::new_rand();
        let sys_owner = Pubkey::new_rand();
        let sys_account = ProgramAccount {
            lamports: 0,
            owner: sys_owner,
            data: vec![],
            executable: false,
        };

        let mut program_accounts = HashMap::new();
        program_accounts.insert(sys_program_id, sys_account);

        let sys = Self {
            current_block: Block::default(),
            block_hash: HashSet::new(),
            program_accounts,
        };

        Some(sys)
    }

    pub fn create_account(
        &mut self,
        owner: Pubkey,
        data: Vec<u8>,
        lamports: u64,
        executable: bool,
    ) -> Pubkey {
        let program_id = Pubkey::new_rand();

        let program_account = ProgramAccount {
            lamports,
            owner,
            data,
            executable,
        };

        self.program_accounts.insert(program_id, program_account);

        program_id
    }

    // leader node's work
    pub fn genesis() -> Blockchain {
        Blockchain::genesis()
    }


    // leader node's work. 동시에 여러 노드가 진행할 수 있음.
    pub fn create_block(&mut self) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block::new(
            [0u8; 64],
            self.current_block.slot,
            timestamp,
            self.current_block.hash.clone(),
            HashMap::new(),
            vec![],
            0,
            0,
            0
        );

        // a tiny of PoW. Acts as a spam filter.
        // 단일 노드에서 너무 많은 블록이 생성되는 것을 방지하기 위한 스팸필터.
        // 블록을 생성하는 노드(leader node)가 이를 위해 일정 계산 리소스를 사용했는지 확인함.
        while !block.is_valid(0) {
            block.slot += 1;
        }

        block
    }

    pub fn update_chain(&mut self, block: Block, blockchain: &mut Blockchain) {
        self.block_hash.insert(block.hash.clone());
        self.current_block = block;

        blockchain.add_block(&mut self.current_block).expect("chain update failure");
    }
}

pub fn create_essential_id(sys: &mut Sys, owner: Pubkey) -> (Pubkey, Pubkey) {
    let token_id = sys.create_account(owner, vec![], 0, false);
    let mint_id = sys.create_account(owner, vec![], 0, false);

    (token_id, mint_id)
}