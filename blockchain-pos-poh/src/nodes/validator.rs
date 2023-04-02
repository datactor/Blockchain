use std::sync::Arc;
use crate::{Account, DBHandler, DBPool, Pubkey, ShardPath};

struct Validator {
    // shard_path: ShardPath,
    handler: DBHandler,
}

impl Validator {
    fn new(account: &[u8], max_dbs: usize) -> Validator {
        // Initialize validator
        Validator {
            handler: DBHandler {
                db_pool: Arc::new(DBPool::new(max_dbs))
            },
        }
    }

    // fn login(&mut self, account_id: u64, pubkey: Pubkey) -> Result<Account, String> {
    //     // Find the shard and lock it with ShardPath
    //     let shard = self.shard_path.get_shard(account_id);
    //     let shard_guard = shard.lock().unwrap();
    //
    //     // Get the chunk index for the account and lock it with AccountPath
    //     let chunk_idx = self.account_path.get_chunk_idx(account_id);
    //     let chunk_guard = shard_guard.accounts[chunk_idx].lock().unwrap();
    //
    //     // Access account data from chunk
    //     let account = chunk_guard.get_account(account_id);
    //
    //     // Verify account pubkey matches input pubkey
    //     if account.pubkey != pubkey {
    //         return Err(String::from("Public key does not match account"));
    //     }
    //
    //     // Return the account
    //     Ok(account)
    // }
}