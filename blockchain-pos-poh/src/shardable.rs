use std::path::Path;

pub trait ShardDB {
    fn open<P: AsRef<Path>>(path: P) -> Self;
    fn write(&mut self, key: &[u8], value: &[u8]);
    fn read(&self, key: &[u8]) -> Option<Vec<u8>>;
}

impl ShardDB for rocksdb::DB {
    fn open<P: AsRef<Path>>(path: P) -> Self {
        let opts = rocksdb::Options::default();
        rocksdb::DB::open(&opts, path).unwrap()
    }

    fn write(&mut self, key: &[u8], value: &[u8]) {
        let mut batch = rocksdb::WriteBatch::default();
        batch.put(key, value).unwrap();
        self.write(batch).unwrap();
    }

    fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get(key).unwrap().map(|value| value.to_vec())
    }
}

impl ShardDB for sled::Db {
    fn open<P: AsRef<Path>>(path: P) -> Self {
        sled::open(path).unwrap()
    }

    fn write(&mut self, key: &[u8], value: &[u8]) {
        self.insert(key.to_vec(), value.to_vec()).unwrap();
    }

    fn read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get(key).unwrap().map(|value| value.to_vec())
    }
}

pub fn merge_accounts_into_shards<D: ShardDB>(accounts: Vec<(Vec<u8>, Vec<u8>)>, db: &mut D) {
    for (account_id, account_data) in accounts {
        // Write the account data to the current shard
        db.write(&account_id, &account_data);
    }
}