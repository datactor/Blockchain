use {
    std::{
        sync::Arc,
        time::Duration,
    },
    crate::{Account, DBHandler, DBPool, Pubkey, ShardPath, RateLimiter},
};

pub struct Validator {
    handler: Arc<DBHandler>,
    rate_limiter: RateLimiter,
}

impl Validator {
    pub fn new(max_dbs: usize, max_requests: u32, time_frame: Duration) -> Validator {
        // Initialize validator
        Validator {
            handler: Arc::new(DBHandler::new(max_dbs)),
            rate_limiter: RateLimiter::new(max_requests, time_frame),
        }
    }

    pub fn login(&mut self, shard_path: String, account_id: &[u8]) -> Result<Option<Vec<u8>>, String> {
        let db_pool = Arc::get_mut(&mut self.handler).unwrap();
        db_pool.handle_request_get(shard_path, account_id)
    }

    pub fn signup(&mut self, shard_path: String, account_id: &[u8], val: &[u8]) -> Result<(), String> {
        let db_pool = Arc::get_mut(&mut self.handler).unwrap();
        db_pool.handle_request_create(shard_path, account_id, val)
    }

    pub fn update(&mut self, shard_path: String, account_id: &[u8], val: &[u8]) -> Result<(), String> {
        let db_pool = Arc::get_mut(&mut self.handler).unwrap();
        db_pool.handle_request_update(shard_path, account_id, val)
    }
}