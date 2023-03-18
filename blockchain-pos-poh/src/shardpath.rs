use std::{
    collections::HashMap,
    fs,
    time::{Duration, Instant},
    path::Path,
    io::BufReader,
};
use std::collections::HashSet;
use std::path::PathBuf;

use rocksdb::{DB, Options, WriteBatch, WriteOptions, IteratorMode};
use serde::Deserialize;

const MAX_NUM_SHARDS: usize = 128;
const SHARD_SIZE: usize = 25_000_000;
const SHARDS_PER_PATH: usize = 32;
const REINDEX_INTERVAL: u64 = 3600; // Reindex every 1 hour

const SHARD_INDICES: [(u16, u16); 65536] = {
    let mut shard_indices = [(0, 0); 65536];
    for i in 0..256 {
        for j in 0..256 {
            let index = i * 256 + j;
            shard_indices[index] = (i as u16, j as u16);
        }
    }
    shard_indices
};

#[derive(Deserialize)]
struct ShardConfig {
    shard_prefix: String,
    num_shards_per_path: u16,
    shard_size: u32,
}

pub struct ShardPath {
    inner: HashMap<(u16, u16), String>,
    last_reindexed: Instant,
}

impl ShardPath {
    pub fn new(paths: &[&str]) -> Self {
        let mut index = ShardPath {
            inner: HashMap::new(),
            last_reindexed: Instant::now(),
        };

        index.index_shards(paths);
        index
    }

    pub fn get_shard(&mut self, account_id: &[u8]) -> Option<&String> {
        let (high_level_shard_index, low_level_shard_index) = (account_id[0] as u16, account_id[1] as u16);

        // 샤드를 이미 할당하여 인덱싱 해뒀으면 shard 리턴
        if let Some(shard) = self.inner.get(&(high_level_shard_index, low_level_shard_index)) {
            // Reindex periodically if necessary
            if self.last_reindexed.elapsed() > Duration::from_secs(REINDEX_INTERVAL) {
                let val = self.inner.values().to_owned();
                let values = val.into_iter().map(|(k, v)| v.split("/shards/").next().unwrap()).collect::<HashSet<&str>>();
                self.index_shards(&Vec::from(values));
                self.last_reindexed = Instant::now();
            };
            return Some(shard)
        }

        None
    }

    pub fn index_shards(&mut self, paths: &[&str]) {
        // Clear the current index
        self.inner.clear();

        // Load the shard configuration from file
        let config_file = fs::File::open(Path::new("src/configmap/shard_config.json")).unwrap();
        let config_reader = BufReader::new(config_file);
        let config: ShardConfig = serde_json::from_reader(config_reader).unwrap();

        // Compute the size of each shard range
        let range_size = 65536 / config.num_shards_per_path as u16;

        // Compute the shard ID for each shard range
        let mut range_to_shard = vec![0; 65536];
        for i in 0..65536 {
            range_to_shard[i] = (i / range_size) as u16;
        } // [0; 2048, 1; 2048,..., 30; 2048, 31; 2048]

        // Assign each range to a shard
        let mut shard_ranges = vec![vec![]; config.num_shards_per_path as usize];
        for i in 0..65536 {
            shard_ranges[range_to_shard[i] as usize].push(i as u16);
        } // shard_ranges = [[0..2048], [2048..4096],..., [61440..63488], [63488..65536]]

        // Assign a path to each shard
        let mut shard_paths = vec![];
        for root_path in paths {
            for i in 0..config.num_shards_per_path {
                shard_paths.push(format!("{}/shards/{}{:03}", root_path, config.shard_prefix, i));
            }
        } // shard_paths = [somepath[0..32],..., otherpath[0..32]]

        // Distribute the ranges to the shards, ensuring roughly equal number of ranges per shard
        let mut shard_index = 0;
        let mut shard_start_range = 0;
        let mut shard_end_range = 0;
        for i in 0..65536 {
            let shard_id = range_to_shard[i];
            if shard_id != shard_index {
                self.inner.insert((shard_start_range / 256, shard_start_range % 256), shard_paths[shard_index as usize].clone());
                shard_index += 1;
                shard_start_range = i as u16;
            }
            shard_end_range = i as u16;
        }
        self.inner.insert((shard_start_range / 256, shard_start_range % 256), shard_paths[shard_index as usize].clone());
        self.last_reindexed = Instant::now();
    }
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