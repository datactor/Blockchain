use std::{cmp, collections::HashMap, fs, sync::{Arc, Mutex}, time::{Duration, Instant}};

use crate::{Account, Hash, Pubkey};
use bs58::{encode, decode};
use rocksdb::{DB, Options, ReadOptions, WriteBatch, WriteOptions, CompactOptions, IteratorMode, DBWithThreadMode, SingleThreaded, Error};

use crate::ShardPath;

const NUM_SHARDS: usize = 128;
const SHARDS_PER_PATH: usize = 32;

pub struct Database {
    db: DB,
    read_opts: ReadOptions,
    write_opts: WriteOptions,
}

impl Database {
    pub fn new(shard_path: &str) -> Self {
        let db_opts = Options::default();
        let db = DB::open(&db_opts, shard_path).unwrap();
        let read_opts = ReadOptions::default();
        // 기본적으로 readOptions를 사용하지 않아도 RocksDB에서 데이터를 읽을 때, 데이터가 메모리에 캐시되어 후속 읽기 성능이 향상된다.
        // 그러나 이렇게 하면 데이터를 읽거나 자주 변경되는 데이터를 읽는 경우 memory bloat이 일어날 수 있다.
        // 이 경우 메모리 사용량을 줄이기 위해 캐싱을 비활성화 할 수 있다(set_fill_cache(), 기본값은 true).

        let write_opts = WriteOptions::default();
        // set_sync(true) -> 각 개별 write 작업이 부분적으로 commit되지 않고, disk에 완전히 commit되거나 전혀 commit되지 않도록 atomicity를 보장하게 한다.
        // activate Write-Ahead Logging -> git 리포지토리의 commit 로그를 남기는 것처럼 변경사항을 기록하고 이전 상태로 복원할 수 있도록 함.
        Self {
            db,
            read_opts,
            write_opts
        }
    }

    // unwrap 제거, 실패시 존재하지 않는 아이디 라는 메시지 반환하기
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        // self.db.get_opt(key, &self.read_opts).unwrap()
        match self.db.get_opt(key, &self.read_opts) {
            Ok(Some(val)) => Ok(Some(val)),
            Ok(None) => Err(format!("Error: The [{:?}] account ID does not exist", key)),
            Err(e) => Err(format!("Error: Failed to read from database: {:?}", e)),
        }
    }

    pub fn create_account(&mut self, key: &[u8], value: &[u8]) -> Result<(), String> {
        // Write operations are atomic by default
        // 'put' operation ensure that each write is fully committed or not committed at all
        let mut batch = WriteBatch::default(); // 개별 쓰기가 아닌 일괄 batch 처리

        // if let Ok(_) = self.get(key) {
        //     return Err(format!("Error: The [{:?}] account ID already exist", key))
        // }
        //
        // batch.put(key, value);
        // self.db.write_opt(batch,&self.write_opts).unwrap();
        // Ok(())
        match self.get(key) {
            Ok(Some(_)) => Err(format!("Error: The [{:?}] account ID already exists", key)),
            _ => {
                batch.put(key, value);
                self.db.write_opt(batch, &self.write_opts)
                    .map_err(|e| format!("Error: Failed to write to database: {:?}", e))
            }
        }
    }

    pub fn update(&mut self, key: &[u8], value: &[u8]) -> Result<(), String> {
        let mut batch = WriteBatch::default();
        if let Err(_) = self.get(key) {
            return Err(format!("Error: The [{:?}] account ID is not exist", key))
        }

        batch.put(key, value);
        self.db.write_opt(batch, &self.write_opts)
            .map_err(|e| format!("Error: Failed to write to database: {:?}", e))
    }
}

// validator는 캐시하는 것 처럼 DB들을 집합으로 DBPool로 만들어서 저장하고 있다.
pub struct DBPool {
    pool: Arc<HashMap<String, Arc<Mutex<Database>>>>,
    last_accessed: HashMap<String, Instant>,
}

// get shard index 메소드는 필요없다 validator에게 요청이 가기 전에 네트워크에서 먼저 sys 프로그램에 해당 요청의
// 메시지를 보낸 후에 해당 accountID로 consiste hash ring을 검색해서 ShardPath를 색인해서 꺼내온 다음에
// PoS에 의해 선택된 validator에게 accountID와 ShardPath, ShardIndex를 보낼 것이기 때문
// 그렇게 하면 validator는 해당 ShardPath의 chunk에 lock을 걸고 accountID를 검색해서 수정하거나 정보를 가져온 다음에
// DBPool에 넣고 lock을 해제할 것
impl DBPool {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(HashMap::new()),
            last_accessed: HashMap::new(),
        }
    }

    pub fn get_information(&mut self, account_id: &[u8], shard_path: String) -> Result<Option<Vec<u8>>, String> {
        let now = Instant::now();
        // if self contains the database
        let databases = Arc::get_mut(&mut self.pool).unwrap();
        if let Some(database) = databases.get(&shard_path) {
            let db = database.lock().unwrap();
            let result = db.get(account_id).expect("Account retrieval failure");
            self.last_accessed.insert(shard_path, now);
            return Ok(result)
        }
        // if self does not contains the database
        let metadata = fs::metadata(&shard_path); // 경로가 잘못되어도 무분별한 생성을 막기 위한 초석.
        if metadata.is_ok() && metadata.unwrap().is_dir() {
            let mut result = Some(Vec::new());
            let database = Arc::new(Mutex::new(Database::new(&shard_path)));
            {
                let db = database.lock().unwrap();
                result = db.get(account_id).expect("Account retrieval failure");
            }
            databases.insert(shard_path.clone(), database);
            self.last_accessed.insert(shard_path.clone(), now);
            return Ok(result)
        }

        // 경로에 메타데이터가 없을 경우, invalid 경로 error 반환
        Err(format!("Error: invalid shard_path {}", shard_path))
    }

    // sign-in 했을 경우에만 수행하는 함수
    pub fn put_account(&mut self, shard_path: String, key: &[u8], val: &[u8]) -> Result<(), String> {
        let now = Instant::now();
        let databases = Arc::get_mut(&mut self.pool).unwrap();

        // if self contains the database
        if let Some(database) = databases.get_mut(&shard_path) {
            let mut db = database.lock().unwrap();
            db.create_account(key, val).expect("account creation failure");
            self.last_accessed.insert(shard_path, now);
            return Ok(())
        }
        // if self does not contains the database
        let metadata = fs::metadata(&shard_path); // 경로가 잘못되어도 무분별한 생성을 막기 위한 초석.
        if metadata.is_ok() && metadata.unwrap().is_dir() {
            let database = Arc::new(Mutex::new(Database::new(&shard_path)));
            let mut databases = Arc::make_mut(&mut self.pool);
            {
                let mut db = database.lock().unwrap();
                db.create_account(key, val).expect("account creation failure");
            }
            databases.insert(shard_path.clone(), database);
            self.last_accessed.insert(shard_path.clone(), now);
            return Ok(())
        }

        // 경로에 메타데이터가 없을 경우, invalid 경로 error 반환
        Err(format!("Error: invalid shard_path {}", shard_path))
    }

    pub fn remove_inactive_database(&mut self) {
        let mut databases = Arc::make_mut(&mut self.pool);
        let mut database_usage = HashMap::new();

        // Calculate usage patterns for each database
        for (shard_path, database) in databases.iter() {
            let db = database.lock().unwrap();
            let stats = db.db.property_int_value("rocksdb.estimate-live-data-size").unwrap_or(Some(0));
            database_usage.insert(shard_path.clone(), stats.unwrap_or(0));
        }
        // Sort databases by usage and prune the least used ones
        let mut database_keys = database_usage.keys().cloned().collect::<Vec<_>>();
        database_keys.sort_by_key(|k| database_usage[k]);
        let num_to_remove = cmp::max(database_keys.len() / 4, 1);
        for shard_path in database_keys.into_iter().take(num_to_remove) {
            databases.remove(&shard_path);
            database_usage.remove(&shard_path);
            self.last_accessed.remove(&shard_path);
        }

        // Prune the rest of the databases based on time since last accessed
        self.last_accessed.retain(|k, v| {
            let elapsed_time = v.elapsed().as_secs();
            if elapsed_time >= 14400 && !database_usage.contains_key(k) {
                databases.remove(k);
                false
            } else {
                true
            }
        });
    }
}
// pub struct DBPool {
//     inner: Arc<Mutex<Database>>,
// }
//
// impl DBPool {
//     pub fn new(path: &str) -> Self {
//         Self {
//             inner: Arc::new(Mutex::new(Database::new(path))),
//         }
//     }
//
//     fn get_shard_index(&self, key: &[u8]) {}
//
//     pub fn get(&self, account_id: &[u8]) -> Option<Vec<u8>> {
//         let shard_index = self.get_shard_index(account_id);
//         let db = self.inner.lock().unwrap();
//         db.get(account_id)
//     }
//
//     pub fn put(&self, account_id: &[u8], value: &[u8]) {
//         let shard_index = self.get_shard_index(account_id);
//         let mut db = self.inner.lock().unwrap();
//         db.put(account_id, value);
//     }
// }
    // pub fn remove_path(&mut self, path: &str) {
    //     self.shard_path.remove_path(path);
    //     let shard_indexes = self.shard_path.get_shard_indexes_for_path(path);
    //     if shard_indexes.contains(&self.get_shard_index(&[])) {
    //         return;
    //     }
    //     let db_path = self.shard_path.get_path_for_shard_index(shard_indexes[0]);
    //     let db = Arc::new(Mutex::new(Database::new(&db_path)));
    //     self.inner = db;
    // }
    //
    // pub fn move_shard(&mut self, _shard_index: usize, from_path: &str, to_path: &str) {
    //     if from_path == to_path {
    //         return;
    //     }
    //     let from_shard_index = self.shard_path.get_shard_indexes_for_path(from_path)[0];
    //     let to_shard_index = self.shard_path.get_shard_indexes_for_path(to_path)[0];
    //     if from_shard_index == to_shard_index {
    //         return;
    //     }
    //     let db_path = self.shard_path.get_path_for_shard_index(to_shard_index);
    //     let db = Arc::new(Mutex::new(Database::new(&db_path)));
    //     self.inner = db;
    // }


// Todo!(); // compatcion
// 데이터 베이스를 더 효율적인 구조를 갖게 여러 개의 작은 파일을 병합하거나 중복 데이터를 삭제함. 이것은 atomic하게 처리됨.
// 패널티? 컴팩션 도중의 추가 cpu및 i/o resource. 당연하겠지만 compatcion 중에 lock이 걸리고 그동안 db 업데이트 불가
// 예를 들어 압축 전에 많은 작은 SSTable이 있었다면 압축 후에 더 큰 SSTable로 병합하여 여러 개의 작은 SSTable을
// 유지 관리하고 쿼리하는 오버헤드를 줄일 수 있다. 또한 compation은 fragmentation 줄이고, 읽기 성능을 향상시키는데
// 도움 될 수 있음. 즉 쿼리 중에 읽어야 하는 파일 수를 줄이기 위해 더 작은 SSTable을 더 큰 SSTable로 병합하여
// 디스크 검색을 줄이고 캐시 지역성을 개선한다.

// pub struct AccountDB {
//     db: DB,
//     read_opts: ReadOptions,
//     write_opts: WriteOptions,
// }
//
// impl AccountDB {
//     pub fn new(path: &str) -> Self {
//         let db_opts = Options::default();
//         let db = DB::open(&db_opts, path).unwrap();
//         let read_opts = ReadOptions::default();
//
//         let write_opts = WriteOptions::default();
//         Self {
//             db,
//             read_opts,
//             write_opts
//         }
//     }
//
//     pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
//         self.db.get_opt(key, &self.read_opts).unwrap()
//     }
//
//     pub fn put(&mut self, key: &[u8], value: &[u8]) {
//         let mut batch = WriteBatch::default();
//         batch.put(key, value);
//         self.db.write_opt(batch,&self.write_opts).unwrap();
//     }
// }
//
//
// pub struct SysDB {
//     db: DB,
//     read_opts: ReadOptions,
//     write_opts: WriteOptions,
// }
//
// impl SysDB {
//     pub fn new(path: &str) -> Self {
//         let db_opts = Options::default();
//         let db = DB::open(&db_opts, path).unwrap();
//         let read_opts = ReadOptions::default();
//
//         let write_opts = WriteOptions::default();
//         Self {
//             db,
//             read_opts,
//             write_opts
//         }
//     }
//
//     pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
//         self.db.get_opt(key, &self.read_opts).unwrap()
//     }
//
//     pub fn put(&mut self, key: &[u8], value: &[u8]) {
//         let mut batch = WriteBatch::default();
//         batch.put(key, value);
//         self.db.write_opt(batch,&self.write_opts).unwrap();
//     }
// }
//
// pub struct TokenDB {
//     db: DB,
//     read_opts: ReadOptions,
//     write_opts: WriteOptions,
// }
//
// impl TokenDB {
//     pub fn new(path: &str) -> Self {
//         let db_opts = Options::default();
//         let db = DB::open(&db_opts, path).unwrap();
//         let read_opts = ReadOptions::default();
//
//         let write_opts = WriteOptions::default();
//         Self {
//             db,
//             read_opts,
//             write_opts
//         }
//     }
//
//     pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
//         self.db.get_opt(key, &self.read_opts).unwrap()
//     }
//
//     pub fn put(&mut self, key: &[u8], value: &[u8]) {
//         let mut batch = WriteBatch::default();
//         batch.put(key, value);
//         self.db.write_opt(batch,&self.write_opts).unwrap();
//     }
// }
//
// pub fn merge_db(input_dbs: &[&str], shard_size: usize, shard_paths: &[&str]) {
//     let mut shard_dbs = Vec::new();
//     let mut shard_opts = Options::default();
//     shard_opts.create_if_missing(true);
//
//     let mut last_shard_index = 0;
//     let mut last_shard_capacity = 0;
//     let mut shard_count = 0;
//
//     // A total of 128 shards of 26 megabytes each will be allocated for a total capacity of 10 gigabytes.
//     // Considering emergency redundancy (2 additional shards),
//     // method 1 (filling all available shards in the path and then moving on to the next path)
//     // with half load will be used initially, It will fill the paths sequentially until it has allocated a total of 64 (128 halves) shards.
//     let max_shard_cap = 25_000_000; // Maximum capacity per shard. (margin = 1mb)
//     let mut last_shard_name = String::new();
//
//     // find insufficient shard
//     for shard_path in shard_paths {
//         // After getting the list of folders in "shard_path" with the OS command,
//         // set the last index file name as last_shard_name
//         // Get list of folders in shard_path
//         if let Ok(entries) = fs::read_dir(shard_path) {
//             let mut shard_indices: Vec<usize> = entries
//                 .filter_map(|e| e.ok())
//                 .filter_map(|e| {
//                     let file_name = e.file_name().into_string().ok()?;
//                     let mut suffixs = file_name.rsplitn(2, "_shard_");
//                     let suffix = suffixs.next()?;
//                     let prefix = suffixs.next()?;
//                     let index = suffix.parse::<usize>().ok()?;
//                     Some(index)
//                 })
//                 .collect();
//             shard_indices.sort();
//             shard_indices.dedup();
//             shard_count += shard_indices.len();
//
//             if let Some(last_index) = shard_indices.last() {
//                 let last_shard_opts = Options::default();
//                 let tmp_shard_name = format!("{}_shard_{}", shard_path, last_index);
//                 let mut last_shard = DB::open(&last_shard_opts, &tmp_shard_name).unwrap();
//                 let mut tmp_capacity = 0;
//                 let mut last = last_shard.iterator(IteratorMode::Start);
//                 while let Some(Ok((key, value))) = last.next() {
//                     tmp_capacity += value.len();
//                 }
//                 if tmp_capacity < max_shard_cap {
//                     last_shard_capacity = tmp_capacity;
//                     last_shard_name = tmp_shard_name;
//                 }
//             }
//         }
//     }
//
//     // If no shards exist or no insufficient shard exist, create the first one
//     if last_shard_name.is_empty() {
//         last_shard_name = format!("{}_shard_{}", shard_path, shard_count);
//         shard_count += 1;
//     }
//
//     let db = DB::open(&shard_opts, last_shard_name).unwrap();
//     shard_dbs.push(db);
//
//
//     // Merge all input databases into shard databases
//     let mut shards_per_path = vec![shard_count / shard_paths.len(); shard_paths.len()];
//     let mut current_path_index = 0;
//     for input_db_path in input_dbs {
//         let input_opts = Options::default();
//         let input_db = DB::open(&input_opts, input_db_path).unwrap();
//         let mut iter = input_db.iterator(IteratorMode::Start);
//
//         while let Some(Ok((key, value))) = iter.next() {
//             let shard_path = shard_paths[current_path_index];
//
//             let shard_index = calculate_shard_index(&key.to_vec().to_vec()[..], shards_per_path[current_path_index], shard_size, last_shard_index, last_shard_capacity);
//             let mut shard_db = shard_dbs.get_mut(shard_index).unwrap();
//
//
//             let mut batch = WriteBatch::default();
//             batch.put(&key.to_vec()[..], &value.to_vec()[..]);
//
//             let write_opts = WriteOptions::default();
//             shard_db.write_opt(batch, &write_opts).unwrap();
//
//             // Update last shard index and capacity
//             last_shard_index = shard_index;
//             last_shard_capacity += value.len();
//
//             // If last shard is full, create a new one
//             if last_shard_capacity >= shard_size {
//                 last_shard_capacity = 0;
//                 shard_count += 1;
//                 if shard_count % SHARDS_PER_PATH == 0 {
//                     current_path_index = (current_path_index + 1) % shard_paths.len();
//                     shards_per_path[current_path_index] += 1;
//                 }
//                 let db = DB::open(&shard_opts, format!("{}_shard_{}", shard_path, shard_count)).unwrap();
//                 shard_dbs.push(db);
//             }
//         }
//     }
//
//     // Compact all shard databases to optimize storage
//     for shard_db in &shard_dbs {
//         shard_db.compact_range(None::<&[u8]>, None::<&[u8]>); // 'None' compacts the entire database by default
//     }
// }
//
// // Helper function to calculate shard index based on column family name
// // 해싱하지 않으면 u256값을 사용해야함.
// fn calculate_shard_index(key: &[u8], num_shards: usize, shard_size: usize, last_shard_idx: usize, last_shard_capacity: usize) -> usize {
//     let mut shard_idx = (farmhash::hash64(key) as usize) % num_shards;
//
//     // Calculate the total capacity of the current shard
//     let mut total_shard_capacity = (shard_idx + 1) * shard_size;
//     if shard_idx == last_shard_idx {
//         total_shard_capacity -= last_shard_capacity;
//     }
//
//     // If the current shard is full, find the next available shard
//     while total_shard_capacity < key.len() {
//         shard_idx = (shard_idx + 1) % num_shards;
//         total_shard_capacity = (shard_idx + 1) * shard_size;
//         if shard_idx == last_shard_idx {
//             total_shard_capacity -= last_shard_capacity;
//         }
//     }
//     shard_idx
// }

// todo!
// 1. shard_dbs에 대해 일반 Vec 대신 Arc<Mutex>를 사용하면 여러 스레드가 동일한 벡터에 액세스할 때 스레드
// 안전성을 꾀하기
//
// 2. calculate_shard_index 함수 내에서 샤드의 순차 인덱스와 정확한 용량을 정확하게 계산하기 위해 각 샤드의
// 현재 용량을 추적하는 B-트리와 같은 데이터 구조를 사용하는 것을 고려. 이렇게 하려면 키-값 쌍이 샤드에 기록될
// 때마다 용량을 업데이트해야 한다.
//
// 3. 고정 크기 샤드를 생성하는 대신 데이터의 실제 크기를 기반으로 샤드 생성을 고려.
// 예를 들어, 단일 샤드를 생성하여 시작할 수 있으며, 채워지면 데이터를 수용하기 위해 동적으로 새 샤드를 생성.
// 이렇게 하면 저장 공간을 보다 효율적으로 사용 가능
//
// 4. error handling, logging framework 사용