use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    time::{SystemTime, UNIX_EPOCH},
    collections::{HashMap, HashSet},
};
use rocksdb::{DB, Options, ReadOptions, WriteBatch, WriteOptions, CompactOptions, IteratorMode, DBWithThreadMode, SingleThreaded, Error};
use serde::{Deserialize, Serialize};
use bs58::{encode, decode};

use crate::block::Block;
use crate::{Blockchain, Hash, Pubkey, Token, Account, Database, DBHandler, Mint, Signature, ProgramResult, EncodedPubkey};

pub const SYS_ID: Pubkey = Pubkey::const_new([0u8; 32]);
pub const TOKEN_ID: Pubkey = Pubkey::const_new([1u8; 32]);
pub const MINT_ID: Pubkey = Pubkey::const_new([2u8; 32]);

pub const PATH: &str = "src/configmap/sys.json";

pub fn start() -> ProgramResult {
    // let sys = Sys::create_sys_account();
    let sys = if let Ok(sys) = Sys::from_file(PATH) {
        sys
    } else {
        let mut sys = Sys::create_sys_account().unwrap();
        let owner = Pubkey::new_rand();
        sys.create_account(owner, vec![], 0, false, TOKEN_ID);
        sys.create_account(owner, vec![], 0, false, MINT_ID);
        let mint = Mint::genesis(1_000_000_000_000, owner, 2);
        let token = Token::genesis(mint.total_supply, owner, 2);
        sys.to_file(PATH).expect("File creating failure");
        sys
    };

    Ok(())
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProgramAccount {
    pub lamports: u64,
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub executable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    pub current_block: Block,
    pub program_accounts: HashMap<EncodedPubkey, ProgramAccount>
}


impl Sys {
    pub fn to_file(&self, filepath: &str) -> io::Result<()> {
        let mut file = File::create(filepath)?;
        let copied_program_accounts = &self.program_accounts;
        // println!("{:?}", copied_program_accounts);
        // let mut vec: Vec<(Pubkey, ProgramAccount)> = copied_program_accounts
        //     .iter()
        //     .map(|(pubkey, program_account)| (*pubkey, program_account.clone()))
        //     .collect();
        //
        // vec.sort();
        let serialized = serde_json::to_string(&copied_program_accounts)?;

        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn from_file(filepath: &str) -> io::Result<Sys> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let sys: Sys = serde_json::from_reader(reader)?;
        Ok(sys)
    }

    pub fn create_sys_account() -> Option<Sys> {
        let sys_program_id = SYS_ID;
        let sys_owner = Pubkey::new_rand();
        let sys_account = ProgramAccount {
            lamports: 0,
            owner: sys_owner,
            data: vec![],
            executable: false,
        };

        let mut program_accounts = HashMap::new();
        program_accounts.insert(EncodedPubkey::from(sys_program_id), sys_account);

        let sys = Self {
            current_block: Block::default(),
            // block_hash: HashSet::new(),
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
        program_id: Pubkey,
    ) -> Pubkey {
        let program_account = ProgramAccount {
            lamports,
            owner,
            data,
            executable,
        };

        self.program_accounts.insert(EncodedPubkey::from(program_id), program_account);

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
            Signature([0u8; 64]),
            self.current_block.slot,
            self.current_block.timestamp,
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
        while !block.verify_tiny_pow(0) {
            block.slot += 1;
        }

        block
    }

    pub fn update_chain(&mut self, block: Block, blockchain: &mut Blockchain) {
        // self.block_hash.insert(block.hash.clone());
        self.current_block = block;

        blockchain.add_block(&mut self.current_block).expect("chain update failure");
    }

    pub fn from_db(dbpath: String) -> Result<Option<Vec<u8>>, String> {
        let mut dbhandler = DBHandler::new(0); // 여기의 0은 노드의 개수, 잠재적으로 진입할 최대 db의 개수
        dbhandler.handle_request_get(dbpath, &SYS_ID.0)
    }
}

pub fn create_essential_id(sys: &mut Sys, owner: Pubkey) -> (Pubkey, Pubkey) {
    let token_id = sys.create_account(owner, vec![], 0, false, TOKEN_ID);
    let mint_id = sys.create_account(owner, vec![], 0, false, MINT_ID);

    (token_id, mint_id)
}