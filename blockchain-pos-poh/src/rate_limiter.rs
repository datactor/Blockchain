use std::collections::HashMap;
use std::time::{Duration, Instant};

struct RateLimiter {
    limits: HashMap<String, Instant>,
    max_requests: u32,
    time_frame: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, time_frame: Duration) -> Self {
        Self {
            limits: HashMap::new(),
            max_requests,
            time_frame,
        }
    }

    pub fn check_rate(&mut self, id: String) -> bool {
        let now = Instant::now();

        self.limits.retain(|_, time| now - *time < self.time_frame);
        if self.limits.len() as u32 >= self.max_requests {
            return false;
        }

        self.limits.insert(id, now);
        true
    }
}