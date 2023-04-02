use std::sync::Arc;
use std::time::Duration;
use crate::{Account, DBHandler, DBPool, Pubkey, ShardPath, RateLimiter};

struct Validator {
    handler: Arc<DBHandler>,
    rate_limiter: RateLimiter,
}

impl Validator {
    fn new(max_dbs: usize, max_requests: u32, time_frame: Duration) -> Validator {
        // Initialize validator
        Validator {
            handler: Arc::new(DBHandler {
                db_pool: Arc::new(DBPool::new(max_dbs))
            }),
            rate_limiter: RateLimiter::new(max_requests, time_frame),
        }
    }

    fn login(&mut self, shard_path: String, account_id: &[u8]) -> Result<Option<Vec<u8>>, String> {
        let db_pool = Arc::get_mut(&mut self.handler).unwrap();
        db_pool.handle_request_get(shard_path, account_id)
    }
}