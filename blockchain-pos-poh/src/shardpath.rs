use std::{
    collections::HashMap,
    fs,
    time::{Duration, Instant},
};

use rocksdb::{DB, Options, WriteBatch, WriteOptions, IteratorMode};

const MAX_NUM_SHARDS: usize = 128;
const SHARD_SIZE: usize = 25_000_000;
const SHARDS_PER_PATH: usize = 32;
const REINDEX_INTERVAL: u64 = 3600; // Reindex every 1 hour

pub struct ShardPath {
    pub high_level_shard_index: u16,
    pub low_level_shard_index: u16,
    pub path: String,
}

pub struct ShardIndex {
    inner: HashMap<(u16, u16), String>,
    last_reindexed: Instant,
}

impl ShardIndex {
    pub fn new(paths: &[&str]) -> Self {
        let mut index = ShardIndex {
            inner: HashMap::new(),
            last_reindexed: Instant::now(),
        };

        index.add_shard_path(Some(paths));
        index
    }

    fn index_shard_paths(&mut self) {
        // Sort shard paths alphabetically
        let mut sorted_paths = self.inner.values().collect::<Vec<&String>>();
        sorted_paths.sort();

        // Compute the number of shards and the size of each shard range
        let num_shards = sorted_paths.len() as u16;
        let range_size = 256 / num_shards;

        // Update the index for each shard path
        for (i, path) in sorted_paths.iter().enumerate() {
            // Compute the shard index based on the number of shards and the position of the path in the sorted list
            let high_level_shard_index = (i as u16 * range_size) / 256;
            let low_level_shard_index = (i as u16 * range_size) % 256;

            // Update the shard index of the inner HashMap
            self.inner.insert((high_level_shard_index, low_level_shard_index), path.to_string());
        }

        self.last_reindexed = Instant::now();
    }

    pub fn add_shard_path(&mut self, option_paths: Option<&[&str]>) {
        // Compute the shard index based on the number of shards and the position of the path in the sorted list
        // let num_shards = self.inner.len() as u16;
        // let high_level_shard_index = (num_shards * u8::MAX+1 as u16) / MAX_NUM_SHARDS as u16;
        // let low_level_shard_index = (num_shards * u8::MAX+1 as u16) % MAX_NUM_SHARDS as u16;

        // Calculate the shard index range for the new paths
        let range_size = if paths.len() > 0 {
            MAX_NUM_SHARDS as u16 / paths.len() as u16
        } else {
            0
        };

        let mut all_paths: Vec<String> = self.inner.values().map(|&s| s.clone()).collect();
        // Create a vector of all the paths, both old and new
        if let Some(paths) = option_paths {
            all_paths.extend(paths.iter().map(|&s| s.to_string()));
        }

        // Sort the paths in lexicographic order
        all_paths.sort();

        // Iterate through the sorted all paths and insert them into the index HashMap
        for (i, path) in all_paths.iter().enumerate() {
            // Calculate the high and low level shard indices for the new path
            let high_level_shard_index = (i as u16) / SHARDS_PER_PATH as u16;
            let low_level_shard_index = if range_size > 0 {
                ((i as u16) % SHARDS_PER_PATH as u16) * range_size
            } else {
                0
            };

            // Find the correct index to insert the new shard path
            let insert_index = match self.inner.get(&(high_level_shard_index, low_level_shard_index)) {
                Some(_) => {
                    println!("Duplicate shard path indices detected");
                    return
                },
                None => {
                    let mut index = 0;
                    for (i, path) in self.inner.values().enumerate() {
                        let (_, current_low_shard_index) = *self.inner.keys().nth(i).unwrap();
                        if current_low_shard_index > low_level_shard_index {
                            break;
                        }
                        index = i + 1;
                    }
                    index
                }
            };

            // Insert the shard path at the correct index
            let index_key = (high_level_shard_index, low_level_shard_index);
            self.inner.insert(index_key, path.to_string());
            self.inner.remove(&self.inner.keys().nth(insert_index).unwrap());

            // Reindex to ensure deterministic shard indexing
            self.index_shard_paths();
        }
    }

    pub fn get_shard_path(&mut self, account_id: &[u8]) -> Option<&String> {
        // Compute the shard index based on the account ID
        let num_shards = self.inner.len() as u16;
        let high_level_shard_index = (u16::from(account_id[0]) * num_shards as u16) / u8::MAX+1;
        let low_level_shard_index = (u16::from(account_id[1]) * num_shards as u16) / u8::MAX+1;

        // Check if the shard index is greater than or equal to the maximum number of shards
        if high_level_shard_index >= MAX_NUM_SHARDS as u16 || low_level_shard_index >= MAX_NUM_SHARDS as u16 {
            return None; // Return None if the shard index exceeds the maximum number of shards
        }

        // If the shard index is greater than or equal to the current number of shards, add a new shard to the index
        let shard_index = (high_level_shard_index, low_level_shard_index);
        if !self.inner.contains_key(&shard_index) {
            self.add_shard_path(None);
        }

        // Reindex periodically if necessary
        if self.last_reindexed.elapsed() > Duration::from_secs(REINDEX_INTERVAL) {
            self.index_shard_paths();
            self.last_reindexed = Instant::now();
        }

        // Get the shard path for the corresponding shard index
        self.inner.get(&(high_level_shard_index, low_level_shard_index)).map(|s| s.as_str())
    }
}

pub fn create_shard_paths(root_path: &str) -> Vec<ShardPath> {
    let mut shard_paths = Vec::new();
    for i in 0..SHARDS_PER_PATH {
        let path = format!("{}/shards/shard_{:03}", root_path, i);
        fs::create_dir_all(&path).expect("Failed to create shard directory");
        shard_paths.push(ShardPath {
            high_level_shard_index: i as u16 / 256,
            low_level_shard_index: i as u16 % 256,
            path: path,
        });
    }
    // Calculate the total size of each shard path
    let mut shard_sizes: Vec<(usize, &ShardPath)> = Vec::with_capacity(shard_paths.len());
    for shard_path in &shard_paths {
        let shard_size = get_shard_size(&shard_path.path);
        shard_sizes.push((shard_size, shard_path));
    }

    // Sort shard paths by size
    shard_sizes.sort_by_key(|&(size, _)| size);

    shard_paths
}

fn get_shard_size(path: &str) -> usize {
    let mut size = 0;
    let db = DB::open_default(path).unwrap();
    for (key, value) in db.iterator(IteratorMode::Start) {
        size += key.len() + value.len();
    }
    size
}

pub fn merge_accounts_into_shards(accounts: Vec<(Vec<u8>, Vec<u8>)>, shard_paths: &[ShardPath]) {
    let mut shard_opts = Options::default();
    shard_opts.create_if_missing(true);

    let mut shard_indices = vec![0; shard_paths.len()];
    let mut shard_dbs = vec![DB::open(&shard_opts, &shard_paths[0].path).unwrap()];
    let mut current_shard_index = 0;

    for (account_id, account_data) in accounts {
        // Get the path of the shard that the account should belong to
        let shard_path = shard_paths[current_shard_index].path.clone();
        let shard_db = &mut shard_dbs[current_shard_index];

        // Write the account data to the current shard
        let mut batch = WriteBatch::default();
        batch.put(&account_id, &account_data);
        let write_opts = WriteOptions::default();
        shard_db.write_opt(batch, &write_opts).unwrap();

        // Update shard index and capacity
        shard_indices[current_shard_index] += 1;
        let shard_capacity = shard_indices[current_shard_index] * account_data.len();
        if shard_capacity >= SHARD_SIZE {
            // If current shard is full, create a new shard and switch to it
            shard_indices[current_shard_index] = 0;
            current_shard_index += 1;
            if current_shard_index >= shard_paths.len() {
                panic!("Exceeded maximum number of shards");
            }
            let shard_path = &shard_paths[current_shard_index].path;
            let shard_db = DB::open(&shard_opts, shard_path).unwrap();
            shard_dbs.push(shard_db);
        }
    }
}