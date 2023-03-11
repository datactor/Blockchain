use core::num::fmt::Part::Num;
use std::collections::HashMap;
use crate::{Account, Hash, Pubkey};
use std::sync::{Arc, Mutex};
use bs58::{encode, decode};
use rocksdb::{DB, Options, ReadOptions, WriteBatch, WriteOptions, IteratorMode, DBWithThreadMode, SingleThreaded};

const NUM_SHARDS: usize = 65536; // number of shards to use
const SHARD_PER_SHARDS: usize = 256;

pub struct Database {
    db: DB,
    read_opts: ReadOptions,
    write_opts: WriteOptions,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let db_opts = Options::default();
        let db = DB::open(&db_opts, path).unwrap();
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

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get_opt(key, &self.read_opts).unwrap()
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        // Write operations are atomic by default
        // 'put' operation ensure that each write is fully committed or not committed at all
        let mut batch = WriteBatch::default(); // 개별 쓰기가 아닌 일괄 batch 처리
        batch.put(key, value);
        self.db.write_opt(batch,&self.write_opts).unwrap();
    }
}


pub struct ArcDatabase {
    inner: Vec<Vec<Arc<Mutex<Database>>>>,
    paths: Vec<Vec<u8>>,
}

impl ArcDatabase {
    pub fn new(paths: &[&str]) -> Self {
        let mut shards = vec![];
        let mut shard = vec![];
        for i in 0..NUM_SHARDS {
            if i > 0 && i % SHARD_PER_SHARDS == 0 {
                shards.push(shard);
                shard = vec![];
            }
            shard.push(Arc::new(Mutex::new(Database::new(paths[i % paths.len()]))));
        }

        shards.push(shard);

        Self {
            inner: shards,
            paths: paths.iter().map(|p| decode(p).into_vec().unwrap()).collect(),
        }
    }

    fn get_index(&self, key: &[u8]) -> Option<(usize, usize)> {
        let shard_index = key[0] as usize % NUM_SHARDS;
        let mut path_index = None;
        for i in 0..self.paths.len() {
            if self.inner[NUM_SHARDS + i][shard_index].len() > 0 {
                path_index = Some(i);
                break;
            }
        }
        if let Some(i) = path_index {
            let db_index = (key[1] as usize) % self.inner[NUM_SHARDS + i][shard_index].len();
            Some((NUM_SHARDS + i, db_index))
        } else {
            None
        }
    }

    pub fn get(&self, account_id: &[u8]) -> Option<Vec<u8>> {
        if let Some((path_index, db_index)) = self.get_index(account_id) {
            let db = self.inner[path_index][db_index].lock().unwrap();
            db.get(account_id)
        } else {
            None
        }
    }

    pub fn put(&self, account_id: &[u8], value: &[u8]) {
        if let Some((path_index, db_index)) = self.get_index(account_id) {
            let mut db = self.inner[path_index][db_index].lock().unwrap();
            db.put(account_id, value);
        }
    }

    pub fn add_path(&mut self, path: &str) {
        self.inner.push(vec![Arc::new(Mutex::new(Database::new(path))); NUM_SHARDS]);
        self.paths.push(decode(path).into_vec().unwrap());
    }

    pub fn remove_path(&mut self, path: &str) {
        if let Some(i) = self.paths.iter().position(|p| p == path) {
            for shard in &self.inner[NUM_SHARDS + i] {
                for db in shard {
                    db.lock().unwrap().clear();
                }
            }
            self.inner.drain(NUM_SHARDS + i..NUM_SHARDS + i + 1);
            self.paths.remove(i);
        }
    }

    pub fn move_shard(&mut self, shard_index: usize, from_path: &str, to_path: &str) {
        if let Some(from_i) = self.paths.iter().position(|p| p == from_path) {
            if let Some(to_i) = self.paths.iter().position(|p| p == to_path) {
                let db = self.inner[NUM_SHARDS + from_i][shard_index].remove(0);
                self.inner[NUM_SHARDS + to_i][shard_index].push(db);
            }
        }
    }
}

// Todo!(); // compatcion
// 데이터 베이스를 더 효율적인 구조를 갖게 여러 개의 작은 파일을 병합하거나 중복 데이터를 삭제함. 이것은 atomic하게 처리됨.
// 패널티? 컴팩션 도중의 추가 cpu및 i/o resource. 당연하겠지만 compatcion 중에 lock이 걸리고 그동안 db 업데이트 불가
// 예를 들어 압축 전에 많은 작은 SSTable이 있었다면 압축 후에 더 큰 SSTable로 병합하여 여러 개의 작은 SSTable을
// 유지 관리하고 쿼리하는 오버헤드를 줄일 수 있다. 또한 compation은 fragmentation 줄이고, 읽기 성능을 향상시키는데
// 도움 될 수 있음. 즉 쿼리 중에 읽어야 하는 파일 수를 줄이기 위해 더 작은 SSTable을 더 큰 SSTable로 병합하여
// 디스크 검색을 줄이고 캐시 지역성을 개선한다.


pub struct SysDatabase {
    db: DB,
}

impl SysDatabase {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).unwrap();
        Self { db }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).unwrap().map(|v| v.to_vec())
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        self.db.put(key, value).unwrap();
    }

    pub fn delete(&self, key: &[u8]) {
        self.db.delete(key).unwrap();
    }
}