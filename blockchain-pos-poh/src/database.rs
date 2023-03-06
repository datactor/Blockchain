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
        // Write operations are atomic by default
        // 'put' operation ensure that each write is fully committed or not committed at all
        let mut batch = WriteBatch::default(); // 개별 쓰기가 아닌 일괄 batch 처리
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

// Todo!(); // compatcion
// 데이터 베이스를 더 효율적인 구조를 갖게 여러 개의 작은 파일을 병합하거나 중복 데이터를 삭제함. 이것은 atomic하게 처리됨.
// 패널티? 컴팩션 도중의 추가 cpu및 i/o resource. 당연하겠지만 compatcion 중에 lock이 걸리고 그동안 db 업데이트 불가
// 예를 들어 압축 전에 많은 작은 SSTable이 있었다면 압축 후에 더 큰 SSTable로 병합하여 여러 개의 작은 SSTable을
// 유지 관리하고 쿼리하는 오버헤드를 줄일 수 있다. 또한 compation은 fragmentation 줄이고, 읽기 성능을 향상시키는데
// 도움 될 수 있음. 즉 쿼리 중에 읽어야 하는 파일 수를 줄이기 위해 더 작은 SSTable을 더 큰 SSTable로 병합하여
// 디스크 검색을 줄이고 캐시 지역성을 개선한다.