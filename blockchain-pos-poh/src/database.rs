use std::sync::{Arc, Mutex};
use rocksdb::{DB, Options, ReadOptions, WriteBatch, WriteOptions, IteratorMode, DBWithThreadMode, SingleThreaded};

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
        let mut batch = WriteBatch::default();
        batch.put(key, value);
        self.db.write_opt(batch,&self.write_opts).unwrap();
    }
}

pub struct ArcDatabase {
    inner: Arc<Mutex<Database>>,
}

impl ArcDatabase {
    pub fn new(path: &str) -> Self {
        let inner = Arc::new(Mutex::new(Database::new(path)));
        Self { inner }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let db = self.inner.lock().unwrap();
        db.get(key)
    }

    pub fn put(&self, key: &[u8], value: &[u8]) {
        let mut db = self.inner.lock().unwrap(); // MutexGuard는 패닉이 와도 락을 free.
        db.put(key, value);
    }
}