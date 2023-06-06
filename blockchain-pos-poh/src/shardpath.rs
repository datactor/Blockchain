use std::{
    collections::{HashMap, HashSet, hash_map::DefaultHasher, BTreeSet},
    fs,
    time::{Duration, Instant},
    path::Path,
    io::BufReader,
    hash::{Hash, Hasher},
    path::PathBuf,
};
use ring::digest;

use rocksdb::{DB, Options, WriteBatch, WriteOptions, IteratorMode};
use serde::Deserialize;
use serde_json::Deserializer;

const MAX_NUM_SHARDS: usize = 128;
const SHARD_SIZE: usize = 25_000_000;
const SHARDS_PER_PATH: usize = 32;
const REINDEX_INTERVAL: u64 = 3600; // Reindex every 1 hour
const DEFAULT_REPLICAS: usize = 100;

#[derive(Deserialize)]
struct ShardConfig {
    shard_prefix: String,
    num_shards_per_path: u16,
    shard_size: u32,
}

#[derive(Clone)]
pub struct ShardPath {
    inner: HashMap<(u16, u16), String>,
    last_reindexed: Instant,
    ring: ConsistentHashRing,
}

impl ShardPath {
    pub fn new(paths: &[&str]) -> Self {
        let mut index = ShardPath {
            inner: HashMap::new(),
            last_reindexed: Instant::now(),
            ring: ConsistentHashRing::new(),
        };

        index.index_shards(Some(paths));

        index
    }

    pub fn index_shards(&mut self, paths: Option<&[&str]>) {
        if let Some(paths) = paths {
            // Load the shard configuration from file
            let config_file = fs::File::open(Path::new("src/configmap/shard_config.json")).unwrap();
            let config_reader = BufReader::new(config_file);
            let config: ShardConfig = serde_json::from_reader(config_reader).unwrap();

            // Create a consistent hash ring of shard paths
            let mut ring = ConsistentHashRing::new();
            for root_path in paths {
                for i in 0..config.num_shards_per_path {
                    let shard_path = format!("{}/shards/{}{:03}", root_path, config.shard_prefix, i);
                    ring.add(&shard_path, DEFAULT_REPLICAS);
                }
            }

            // Map each account ID to a shard using the hash ring
            let mut new_inner = HashMap::new();
            // for문을 lazy하게 move하는 기술. std::mem::take(&mut self.inner)을 통해 복제나 할당없이, 원소 하나씩 take하여 이동시킴
            for (account_id, path) in std::mem::take(&mut self.inner) {
                if let Some(shard_path) = ring.get_node(&path) {
                    new_inner.insert(account_id, shard_path);
                }
            }

            self.inner = new_inner;
            self.ring = ring;
            self.last_reindexed = Instant::now();
        }
    }

    pub fn get_shard(&mut self, account_id: &[u8]) -> Option<String> {
        let (high_level_shard_index, low_level_shard_index) = (account_id[0] as u16, account_id[1] as u16);

        // 샤드를 이미 할당하여 인덱싱 해뒀으면 shard 리턴
        if let Some(shard) = self.inner.get(&(high_level_shard_index, low_level_shard_index)).map(|s| s.to_owned()) {
            // Reindex periodically if necessary
            // Consistent Hash ring의 구조에서 현재의 샤드와 path의 결정론적을 유지하면서 re-indexing하는 것은 의미가 없다.
            // 그러니 추후에 정해진 시간이 되면 load가 큰 path의 shard와 load가 적은 path의 shard를 교환하는 로직을 짜보자.
            if self.last_reindexed.elapsed() > Duration::from_secs(REINDEX_INTERVAL) {
                self.rebuild_path(None);
            }
            return Some(shard)
        }

        // Check if shard is already assigned
        None
    }

    // 기존의 path에 새로운 path를 추가했을 때 사용.
    pub fn rebuild_path(&mut self, paths: Option<Vec<String>>) -> Self {
        let val = self.inner.values();
        let mut prev_paths = val.into_iter()
            .map(|mut path| path.split("/shards/").next().unwrap().to_string())
            .collect::<HashSet<_>>();

        // If None is received, only indexing is performed.
        let mut new_paths = paths.unwrap_or_default();

        let unioned: HashSet<String> = prev_paths.union(&new_paths.iter().cloned().collect()).cloned().collect();
        let all_paths: Vec<&str> = unioned.iter().map(|s| s.as_str()).collect();

        let mut new_shardpath = ShardPath {
            inner: HashMap::new(),
            last_reindexed: Instant::now(),
            ring: ConsistentHashRing::new(),
        };

        new_shardpath.index_shards(Some(&all_paths[..]));

        new_shardpath
    }
}

// Consistent Hash Ring implementation
#[derive(Clone)]
pub struct ConsistentHashRing {
    nodes: HashMap<u64, String>,
    // sorted_keys: Vec<u64>,
    sorted_keys: BTreeSet<u64>,
}

impl ConsistentHashRing {
    pub fn new() -> Self {
        ConsistentHashRing {
            nodes: HashMap::new(),
            // sorted_keys: Vec::new(),
            sorted_keys: BTreeSet::new(),
        }
    }

    // pub fn add(&mut self, node_id: &str, replicas: usize) {
    //     for i in 0..replicas {
    //         let key = Self::hash(&(node_id.to_owned() + &i.to_string()));
    //         self.nodes.insert(key, node_id.to_owned());
    //         self.sorted_keys.push(key);
    //     }
    //     self.sorted_keys.sort();
    // }

    // pub fn remove(&mut self, node_id: &str) {
    //     let mut remove_indices = Vec::new();
    //     for (i, key) in self.sorted_keys.iter().enumerate() {
    //         let node = self.nodes.get(&key).unwrap();
    //         if node == node_id {
    //             remove_indices.push(i);
    //         }
    //     }
    //     for i in remove_indices.iter().rev() {
    //         self.sorted_keys.remove(*i);
    //     }
    //     self.nodes.retain(|_, v| v != node_id);
    // }

    pub fn add(&mut self, node_id: &str, replicas: usize) {
        for i in 0..replicas {
            let key = Self::hash(&(node_id.to_owned() + &i.to_string()));
            self.nodes.insert(key, node_id.to_owned());
            self.sorted_keys.insert(key);
        }
    }

    pub fn remove(&mut self, node_id: &str) {
        let keys_to_remove: Vec<_> = self.nodes
            .iter()
            .filter(|(_, v)| *v == node_id)
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_remove {
            self.nodes.remove(&key);
            self.sorted_keys.remove(&key);
        }
    }

    pub fn get_node(&self, key: &str) -> Option<String> {
        if self.nodes.is_empty() {
            return None;
        }
        let hash = Self::hash(key);
        let &node_key = self.sorted_keys.range(hash..).next()
            .unwrap_or_else(|| self.sorted_keys.iter().next().unwrap());
        self.nodes.get(&node_key).map(|node| node.clone())
    }

    // pub fn get_node(&self, key: &str) -> Option<String> {
    //     if self.nodes.is_empty() {
    //         return None;
    //     }
    //     let hash = Self::hash(key);
    //     let pos = match self.sorted_keys.binary_search(&hash) {
    //         Ok(pos) => pos,
    //         Err(pos) => {
    //             if pos == self.sorted_keys.len() {
    //                 0
    //             } else {
    //                 pos
    //             }
    //         }
    //     };
    //
    //     self.nodes.get(&self.sorted_keys[pos]).map(|node| node.clone())
    // }

    fn hash(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

// fn tmp() {
//     pub fn move_shard(&mut self) {
//         unimplemented!();
//         // let mut max_load_path: Option<String> = None;
//         // let mut min_load_path: Option<String> = None;
//         // let mut max_load: u32 = 0;
//         // let mut min_load: u32 = u32::MAX;
//         //
//         // // Find the path with the highest load and the path with the lowest load
//         // for (path, load) in self.get_load_by_path() {
//         //     if load > max_load {
//         //         max_load_path = Some(path.to_owned());
//         //         max_load = load;
//         //     }
//         //     if load < min_load {
//         //         min_load_path = Some(path.to_owned());
//         //         min_load = load;
//         //     }
//         // }
//         //
//         // // Find the shard with the highest load in the path with the highest load
//         // let mut max_load_shard: Option<String> = None;
//         // let mut max_load_shard_count: u32 = 0;
//         // if let Some(path) = max_load_path {
//         //     for (shard, count) in self.get_shard_counts_by_path(&path) {
//         //         if count > max_load_shard_count {
//         //             max_load_shard = Some(shard.to_owned());
//         //             max_load_shard_count = count;
//         //         }
//         //     }
//         // }
//         //
//         // // Move the shard to the path with the lowest load
//         // if let Some(shard) = max_load_shard {
//         //     let shard_index = shard.rfind('/').unwrap();
//         //     let shard_root_path = &shard[..shard_index];
//         //     let new_shard_path = format!("{}/shards/{}", min_load_path.unwrap(), &shard[shard_index + 1..]);
//         //
//         //     // Rename the shard directory
//         //     let _ = fs::rename(&shard, &new_shard_path);
//         //
//         //     // Remove the old shard from the index
//         //     let _ = self.remove_shard_from_index(&shard_root_path, &shard);
//         //
//         //     // Add the new shard to the index
//         //     self.add_shard_to_index(&min_load_path.unwrap(), &new_shard_path);
//         // }
//     }
//
//     fn get_load_by_path(&self) -> HashMap<&str, u32> {
//         unimplemented!();
//         // let mut load_by_path: HashMap<&str, u32> = HashMap::new();
//         // for shard_path in self.inner.values() {
//         //     let path = shard_path.split('/').next().unwrap();
//         //     *load_by_path.entry(path).or_insert(0) += 1;
//         // }
//         // load_by_path
//     }
//
//     fn get_shard_counts_by_path(&self, path: &str) -> HashMap<&str, u32> {
//         unimplemented!();
//         // let mut shard_counts_by_path: HashMap<&str, u32> = HashMap::new();
//         // for shard_path in self.inner.values() {
//         //     if shard_path.starts_with(path) {
//         //         let shard = shard_path.rsplitn(2, '/').next().unwrap();
//         //         *shard_counts_by_path.entry(shard).or_insert(0) += 1;
//         //     }
//         // }
//         // shard_counts_by_path
//     }
//
//     fn remove_shard_from_index(&mut self, root_path: &str, shard_path: &str) -> Option<String> {
//         unimplemented!();
//         // let key = self.inner.iter().find(|(_, v)| **v == shard_path)?.0;
//         // let old_shard = self.inner.remove(key)?;
//         // let new_shard = old_shard.replace(root_path, "");
//         // self.inner.insert(key.to_owned(), new_shard);
//         // Some(old_shard)
//     }
// }