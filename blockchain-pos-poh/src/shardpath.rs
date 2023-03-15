use std::collections::HashMap;
use std::time::{Duration, Instant};

const MAX_NUM_SHARDS: u16 = 4096;
const REINDEX_INTERVAL: u64 = 3600; // Reindex every 1 hour

struct ShardPath {
    inner: HashMap<(u16, u16), String>,
    last_reindexed: Instant,
}

impl ShardPath {
    pub fn get_shard_path(&mut self, account_id: &[u8; 32]) -> Option<&str> {
        let num_shards = self.inner.len() as u16;
        let high_level_shard_index = (u16::from(account_id[0]) * num_shards) / 256;
        let low_level_shard_index = (u16::from(account_id[1]) * num_shards) / 256;

        // Check if the shard index is greater than or equal to the maximum number of shards
        if high_level_shard_index >= MAX_NUM_SHARDS || low_level_shard_index >= MAX_NUM_SHARDS {
            return None; // Return None if the shard index exceeds the maximum number of shards
        }

        // If the shard index is greater than or equal to the current number of shards, add a new shard to the index
        if high_level_shard_index >= num_shards || low_level_shard_index >= num_shards {
            self.index_shard_paths();
        }

        // Reindex periodically if necessary
        if self.last_reindexed.elapsed() > Duration::from_secs(REINDEX_INTERVAL) {
            self.index_shard_paths();
            self.last_reindexed = Instant::now();
        }

        // Get the shard path for the corresponding shard index
        self.inner.get(&(high_level_shard_index, low_level_shard_index)).map(|s| s.as_str())
    }

    fn index_shard_paths(&mut self) {
        // Sort shard paths alphabetically
        let mut sorted_paths: Vec<String> = self.inner.values().cloned().collect();
        sorted_paths.sort();

        // Compute the number of shards and the size of each shard range
        let num_shards = sorted_paths.len() as u16;
        let range_size = 256 / num_shards;

        // Generate index for each shard path
        let mut index = HashMap::new();
        for (i, path) in sorted_paths.iter().enumerate() {
            // Compute the shard index based on the number of shards and the position of the path in the sorted list
            let high_level_shard_index = (i as u16 * range_size) / 256;
            let low_level_shard_index = (i as u16 * range_size) % 256;

            // Store the shard path in the corresponding shard index of the index HashMap
            index.insert((high_level_shard_index, low_level_shard_index), path.clone());
        }

        // Update the inner HashMap with the new index
        self.inner = index;
        self.last_reindexed = Instant::now();
    }
}